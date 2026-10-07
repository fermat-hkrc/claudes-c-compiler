// Oracle: differential — llvm-mc RISC-V assembler (SLTU expansion of SNEZ)
// Evidence: src/backend/riscv/assembler/README.md:325 `snez rd, rs` → `sltu rd, x0, rs`;
//   encoder/mod.rs:3 Encodes RISC-V instructions into 32-bit machine code words;
//   encoder/mod.rs:873 "snez" => encode_snez;
//   pseudo.rs:266 sltu rd, x0, rs2;
//   RISC-V Unprivileged ISA pseudoinstruction SNEZ rd, rs = SLTU rd, x0, rs.
// Stronger considered:
//   - State machine: rejected — encode_snez is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree SNEZ/SLTU decoder
//   - Differential vs encode_alu_reg(sltu): rejected as primary — shared encode_r/get_reg
//     (same-job expansion used as a weaker metamorphic instead)
// Weaker available: algebraic.metamorphic (SLTU x0 expansion, ABI/xN alias, Imm 0..31),
//   algebraic.invariant (R-type field layout), negative_error (arity / invalid / extra)
// Differential: candidate=encode_snez, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs)] <-> `snez rd, rs` and `sltu rd, x0, rs`

use super::{encode_alu_reg, encode_snez, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn abi_name(n: u32) -> &'static str {
    ABI[n as usize]
}

fn xn(n: u32) -> String {
    format!("x{}", n)
}

fn reg(name: &str) -> Operand {
    Operand::Reg(name.to_string())
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_snez(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn sltu_word(ops: &[Operand]) -> Result<u32, String> {
    // SLTU: funct3=011, funct7=0000000
    match encode_alu_reg(ops, 0b011, 0b0000000)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn parse_llvm_encoding(stdout: &str) -> Result<u32, String> {
    let marker = "encoding: [";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("no encoding in stdout: {stdout}"))?;
    let rest = &stdout[start + marker.len()..];
    let end = rest
        .find(']')
        .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
    let inner = &rest[..end];
    let mut bytes = [0u8; 4];
    let parts: Vec<&str> = inner.split(',').collect();
    if parts.len() != 4 {
        return Err(format!("expected 4 bytes, got {inner}"));
    }
    for (i, p) in parts.iter().enumerate() {
        let p = p.trim();
        let hex = p
            .strip_prefix("0x")
            .ok_or_else(|| format!("non-hex byte {p}"))?;
        bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
    }
    Ok(u32::from_le_bytes(bytes))
}

fn llvm_mc_word(asm: &str) -> Result<u32, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=riscv64", "-show-encoding"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn llvm-mc: {e}"))?;
    {
        let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
        stdin
            .write_all(asm.as_bytes())
            .map_err(|e| format!("write llvm-mc: {e}"))?;
        stdin
            .write_all(b"\n")
            .map_err(|e| format!("write llvm-mc: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait llvm-mc: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() || stderr.contains("error:") {
        return Err(format!("llvm-mc error: {stderr}"));
    }
    parse_llvm_encoding(&stdout)
}

fn llvm_mc_rejects(asm: &str) -> bool {
    llvm_mc_word(asm).is_err()
}

/// Pin 0, 8 (s0/fp), 31 plus uniform 0..=31.
fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(8u32), Just(31u32), 0u32..=31]
}

/// Canonical names llvm-mc accepts: ABI or xN (not x00, not uppercase).
fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        reg_num().prop_map(|n| abi_name(n).to_string()),
        reg_num().prop_map(xn),
        Just("fp".to_string()),
        Just("zero".to_string()),
        Just("x0".to_string()),
        Just("x31".to_string()),
        Just("t6".to_string()),
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        gpr_name().prop_map(Operand::Reg),
        (-8i64..=40).prop_map(Operand::Imm),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label(".L1".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
    ]
}

fn invalid_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        prop::sample::select(vec![
            "fa0", "ft0", "f0", "fs0", "fa1", "f31", "v0", "v31", "x32", "x", "foo", "", "spx",
            "x-1", "r0", "w0",
        ])
        .prop_map(|s| Operand::Reg(s.to_string())),
        prop_oneof![Just(-1i64), Just(32i64), Just(100i64), Just(i64::MIN), Just(i64::MAX)]
            .prop_map(Operand::Imm),
        Just(Operand::Symbol("sym".into())),
        Just(Operand::Label("lbl".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::SymbolOffset("sym".into(), 4)),
        Just(Operand::MemSymbol {
            base: "sp".into(),
            symbol: "foo".into(),
            modifier: "%lo".into(),
        }),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        gpr_name().prop_map(|n| vec![reg(&n)]),
        invalid_operand().prop_map(|o| vec![o]),
        (-4i64..=40).prop_map(|i| vec![Operand::Imm(i)]),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_snez_kat_llvm_mc() {
    // snez a0, a1 = sltu a0, x0, a1 = encoding [0x33,0x35,0xb0,0x00] = 0x00b03533
    let want = 0x00b0_3533u32;
    let mc = llvm_mc_word("snez a0, a1").expect("llvm-mc KAT snez a0, a1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for snez a0, a1");
    assert_eq!(
        sut_word(&[reg("a0"), reg("a1")]).expect("SUT KAT snez a0, a1"),
        want
    );

    let want_z = 0x0000_3033u32;
    let mc_z = llvm_mc_word("snez zero, zero").expect("llvm-mc KAT snez zero, zero");
    assert_eq!(mc_z, want_z, "llvm-mc KAT mapping broken for snez zero, zero");
    assert_eq!(
        sut_word(&[reg("zero"), reg("zero")]).expect("SUT KAT snez zero, zero"),
        want_z
    );
    assert_eq!(
        sut_word(&[reg("x0"), reg("x0")]).expect("SUT KAT snez x0, x0"),
        want_z
    );

    let want_t6 = 0x0010_3fb3u32;
    let mc_t6 = llvm_mc_word("snez t6, ra").expect("llvm-mc KAT snez t6, ra");
    assert_eq!(mc_t6, want_t6, "llvm-mc KAT mapping broken for snez t6, ra");
    assert_eq!(
        sut_word(&[reg("t6"), reg("ra")]).expect("SUT KAT snez t6, ra"),
        want_t6
    );
    assert_eq!(
        sut_word(&[reg("x31"), reg("x1")]).expect("SUT KAT snez x31, x1"),
        want_t6
    );

    // fp == s0 == x8; llvm-mc pretty-prints fp as s0
    let want_fp = 0x0080_3433u32;
    let mc_fp = llvm_mc_word("snez fp, s0").expect("llvm-mc KAT snez fp, s0");
    assert_eq!(mc_fp, want_fp, "llvm-mc KAT mapping broken for snez fp, s0");
    assert_eq!(
        sut_word(&[reg("fp"), reg("s0")]).expect("SUT KAT snez fp, s0"),
        want_fp
    );
    assert_eq!(
        sut_word(&[reg("x8"), reg("x8")]).expect("SUT KAT snez x8, x8"),
        want_fp
    );

    let mc_sltu = llvm_mc_word("sltu a0, x0, a1").expect("llvm-mc KAT sltu a0, x0, a1");
    assert_eq!(mc_sltu, want, "sltu a0, x0, a1 must encode as snez a0, a1");
    assert_eq!(
        sltu_word(&[reg("a0"), reg("x0"), reg("a1")]).expect("SLTU KAT"),
        want
    );

    assert!(
        llvm_mc_word("snez a0, a1, a2").is_err(),
        "llvm-mc must reject extra operand"
    );
    assert!(
        llvm_mc_word("snez a0").is_err(),
        "llvm-mc must reject missing operand"
    );
    assert!(
        llvm_mc_word("snez fa0, a1").is_err(),
        "llvm-mc must reject FP dest"
    );
}

/// Regression: extra operand must be rejected (README two-operand form; llvm-mc errors).
#[test]
fn test_encode_snez_regression_extra_operand() {
    let ops = [reg("zero"), reg("zero"), reg("zero")];
    assert!(
        encode_snez(&ops).is_err(),
        "snez zero, zero with a third operand must be rejected (llvm-mc rejects; README documents `snez rd, rs`); got {:?}",
        encode_snez(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_snez_diff_llvm_mc(rd in gpr_name(), rs in gpr_name()) {
        let asm = format!("snez {}, {}", rd, rs);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    /// Sweep: documented expansion vs independent assembler (not in-tree encode_alu_reg).
    #[test]
    fn encode_snez_diff_llvm_mc_sltu(rd in gpr_name(), rs in gpr_name()) {
        let asm_sltu = format!("sltu {}, x0, {}", rd, rs);
        let mc = llvm_mc_word(&asm_sltu)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm_sltu, e));
        let sut = sut_word(&[reg(&rd), reg(&rs)])
            .unwrap_or_else(|e| panic!("SUT rejected valid snez {}, {}: {}", rd, rs, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc sltu {:08x} for {}", sut, mc, asm_sltu);
    }

    #[test]
    fn encode_snez_eq_sltu_x0(rd in reg_num(), rs in reg_num()) {
        let ops_snez = [reg(&xn(rd)), reg(&xn(rs))];
        let ops_sltu = [reg(&xn(rd)), reg("x0"), reg(&xn(rs))];
        let s = sut_word(&ops_snez)
            .unwrap_or_else(|e| panic!("SUT rejected snez x{}, x{}: {}", rd, rs, e));
        let u = sltu_word(&ops_sltu)
            .unwrap_or_else(|e| panic!("SUT rejected sltu x{}, x0, x{}: {}", rd, rs, e));
        prop_assert_eq!(s, u);
    }

    #[test]
    fn encode_snez_isa_fields(rd in reg_num(), rs in reg_num()) {
        let w = sut_word(&[reg(&xn(rd)), reg(&xn(rs))])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, x{}: {}", rd, rs, e));
        prop_assert_eq!(w & 0x7F, 0b0110011u32, "opcode OP");
        prop_assert_eq!((w >> 7) & 0x1F, rd, "rd");
        prop_assert_eq!((w >> 12) & 7, 0b011u32, "funct3 SLTU");
        prop_assert_eq!((w >> 15) & 0x1F, 0u32, "rs1 must be x0");
        prop_assert_eq!((w >> 20) & 0x1F, rs, "rs2");
        prop_assert_eq!((w >> 25) & 0x7F, 0u32, "funct7 must be 0");
    }

    #[test]
    fn encode_snez_abi_xn_alias(n in reg_num(), m in reg_num()) {
        let via_x = sut_word(&[reg(&xn(n)), reg(&xn(m))])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_word(&[reg(abi_name(n)), reg(abi_name(m))])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&[reg("fp"), reg(&xn(m))])
                .unwrap_or_else(|e| panic!("fp rd rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if m == 8 {
            let via_fp = sut_word(&[reg(&xn(n)), reg("fp")])
                .unwrap_or_else(|e| panic!("fp rs rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
        if n == 0 {
            let via_zero = sut_word(&[reg("zero"), reg(&xn(m))])
                .unwrap_or_else(|e| panic!("zero rd rejected: {}", e));
            prop_assert_eq!(via_zero, via_x);
        }
        if m == 0 {
            let via_zero = sut_word(&[reg(&xn(n)), reg("zero")])
                .unwrap_or_else(|e| panic!("zero rs rejected: {}", e));
            prop_assert_eq!(via_zero, via_x);
        }
        let via_imm = sut_word(&[Operand::Imm(n as i64), Operand::Imm(m as i64)])
            .unwrap_or_else(|e| panic!("Imm({}, {}) rejected: {}", n, m, e));
        prop_assert_eq!(via_imm, via_x);
    }

    #[test]
    fn encode_snez_field_isolation(
        rd in reg_num(),
        rs_a in reg_num(),
        rs_b in reg_num(),
        rd_a in reg_num(),
        rd_b in reg_num(),
        rs in reg_num(),
    ) {
        let ha = sut_word(&[reg(&xn(rd)), reg(&xn(rs_a))])
            .unwrap_or_else(|e| panic!("rs_a rejected: {}", e));
        let hb = sut_word(&[reg(&xn(rd)), reg(&xn(rs_b))])
            .unwrap_or_else(|e| panic!("rs_b rejected: {}", e));
        prop_assert_eq!((ha >> 7) & 0x1F, (hb >> 7) & 0x1F, "rd field must be independent of rs2");
        let ha2 = sut_word(&[reg(&xn(rd_a)), reg(&xn(rs))])
            .unwrap_or_else(|e| panic!("rd_a rejected: {}", e));
        let hb2 = sut_word(&[reg(&xn(rd_b)), reg(&xn(rs))])
            .unwrap_or_else(|e| panic!("rd_b rejected: {}", e));
        let rd_mask = 0x0000_0F80u32;
        prop_assert_eq!(ha2 & !rd_mask, hb2 & !rd_mask, "opcode/funct3/rs1/rs2/funct7 bits must be independent of rd");
    }

    #[test]
    fn encode_snez_neg_arity(ops in short_ops()) {
        prop_assume!(ops.len() < 2);
        prop_assert!(encode_snez(&ops).is_err(), "arity {} must Err", ops.len());
    }

    #[test]
    fn encode_snez_neg_invalid(
        bad in invalid_operand(),
        good in gpr_name(),
        which in 0u8..=2u8,
    ) {
        let ops: Vec<Operand> = match which {
            0 => vec![bad.clone(), bad.clone()],
            1 => vec![bad, reg(&good)],
            _ => vec![reg(&good), bad],
        };
        prop_assert!(encode_snez(&ops).is_err(), "invalid operand must Err, got {:?}", encode_snez(&ops));
    }

    #[test]
    fn encode_snez_neg_extra(
        rd in gpr_name(),
        rs in gpr_name(),
        extra in extra_operand(),
    ) {
        let asm = format!("snez {}, {}", rd, rs);
        let ops = vec![reg(&rd), reg(&rs), extra];
        prop_assert!(
            encode_snez(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects: {})",
            asm,
            llvm_mc_rejects(&format!("{}, a2", asm))
        );
    }
}
