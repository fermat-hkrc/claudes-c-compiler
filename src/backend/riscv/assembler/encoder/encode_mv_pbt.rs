// Oracle: differential — llvm-mc RISC-V assembler (ADD expansion of MV)
// Evidence: src/backend/riscv/assembler/README.md:319 `mv rd, rs` → `add rd, x0, rs`
//   (uses ADD form for RV64C eligibility);
//   encoder/mod.rs:3 Encodes RISC-V instructions into 32-bit machine code words;
//   encoder/mod.rs:857 "mv" | "move" => encode_mv;
//   pseudo.rs:228-229 Use add rd, x0, rs instead of addi rd, rs, 0 so the
//   instruction is eligible for RV64C compression to C.MV (ADD form);
//   RISC-V Unprivileged ISA R-type ADD; MV pseudo copies rs into rd.
// Stronger considered:
//   - State machine: rejected — encode_mv is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree MV/ADD decoder
//   - Encoding-equality vs llvm-mc `mv`: rejected as primary — llvm-mc expands
//     MV to ADDI (addi rd, rs, 0); this assembler documents ADD (README:319)
//   - Differential vs encode_alu_reg(add): rejected as primary — shared
//     encode_r/get_reg (same-job expansion used as a weaker metamorphic)
// Weaker available: algebraic.invariant (R-type field layout), algebraic.metamorphic
//   (ABI/xN alias, Imm 0..31, field isolation, ADD x0 expansion),
//   negative_error (arity / invalid / extra)
// Differential: candidate=encode_mv, reference=llvm-mc -triple=riscv64 -show-encoding
//   of `add rd, x0, rs`, SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Reg(rs)] <-> `add rd, x0, rs`
// Semantic differential: sim(SUT ADD)[rd] == sim(llvm-mc `mv`)[rd] for rd != x0

use super::{encode_alu_reg, encode_mv, EncodeResult};
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
    match encode_mv(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

fn add_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_alu_reg(ops, 0b000, 0b0000000)? {
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

/// Simulate one ADD (OP-OP funct3=000 funct7=0000000) or ADDI (OP-IMM funct3=000)
/// word per the RISC-V unprivileged ISA. Not a copy of encode_mv. x0 stays 0.
fn sim_copy(word: u32, init: [i64; 32]) -> Result<[i64; 32], String> {
    let mut regs = init;
    let opcode = word & 0x7f;
    let rd = ((word >> 7) & 0x1f) as usize;
    match opcode {
        0b0110011 => {
            // OP (R-type)
            let funct3 = (word >> 12) & 7;
            let rs1 = ((word >> 15) & 0x1f) as usize;
            let rs2 = ((word >> 20) & 0x1f) as usize;
            let funct7 = (word >> 25) & 0x7f;
            if funct3 != 0 || funct7 != 0 {
                return Err(format!("unexpected OP funct3/funct7 in {word:#010x}"));
            }
            regs[rd] = regs[rs1].wrapping_add(regs[rs2]);
        }
        0b0010011 => {
            // OP-IMM (I-type)
            let funct3 = (word >> 12) & 7;
            let rs1 = ((word >> 15) & 0x1f) as usize;
            let imm12 = ((word as i32) >> 20) as i64;
            if funct3 != 0 {
                return Err(format!("unexpected OP-IMM funct3 in {word:#010x}"));
            }
            regs[rd] = regs[rs1].wrapping_add(imm12);
        }
        _ => return Err(format!("unexpected opcode {opcode:#x} in {word:#010x}")),
    }
    regs[0] = 0;
    Ok(regs)
}

fn init_regs() -> [i64; 32] {
    let mut r = [0i64; 32];
    for i in 1..32 {
        r[i] = (i as i64) * 0x1111_1111_1111 + 7;
    }
    r
}

/// Pin 0, 8 (s0/fp), 31 plus uniform 0..=31.
fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(8u32), Just(10u32), Just(31u32), 0u32..=31]
}

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
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
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

/// Known-answer gate for the llvm-mc ADD differential connection.
#[test]
fn encode_mv_kat_llvm_mc_add_a0_a1() {
    let want = 0x00b0_0533u32;
    let mc = llvm_mc_word("add a0, x0, a1").expect("llvm-mc KAT add a0, x0, a1");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for add a0, x0, a1");
    assert_eq!(
        sut_word(&[reg("a0"), reg("a1")]).expect("SUT KAT mv a0, a1"),
        want
    );
    let mc_mv = llvm_mc_word("mv a0, a1").expect("llvm-mc KAT mv a0, a1");
    assert_eq!(
        mc_mv, 0x0005_8513u32,
        "llvm-mc mv a0, a1 must be ADDI (mapping check)"
    );
}

#[test]
fn encode_mv_kat_llvm_mc_add_zero_zero() {
    let want = 0x0000_0033u32;
    let mc = llvm_mc_word("add zero, x0, zero").expect("llvm-mc KAT add zero, x0, zero");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for add zero, x0, zero");
    assert_eq!(
        sut_word(&[reg("zero"), reg("zero")]).expect("SUT KAT mv zero, zero"),
        want
    );
    assert_eq!(
        sut_word(&[reg("x0"), reg("x0")]).expect("SUT KAT mv x0, x0"),
        want
    );
}

#[test]
fn encode_mv_kat_llvm_mc_add_t6_ra() {
    let want = 0x0010_0fb3u32;
    let mc = llvm_mc_word("add t6, x0, ra").expect("llvm-mc KAT add t6, x0, ra");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for add t6, x0, ra");
    assert_eq!(
        sut_word(&[reg("t6"), reg("ra")]).expect("SUT KAT mv t6, ra"),
        want
    );
    assert_eq!(
        sut_word(&[reg("x31"), reg("x1")]).expect("SUT KAT mv x31, x1"),
        want
    );
}

#[test]
fn encode_mv_kat_llvm_mc_add_fp_s0() {
    let want = 0x0080_0433u32;
    let mc = llvm_mc_word("add fp, x0, s0").expect("llvm-mc KAT add fp, x0, s0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for add fp, x0, s0");
    assert_eq!(
        sut_word(&[reg("fp"), reg("s0")]).expect("SUT KAT mv fp, s0"),
        want
    );
    assert_eq!(
        sut_word(&[reg("x8"), reg("x8")]).expect("SUT KAT mv x8, x8"),
        want
    );
}

#[test]
fn encode_mv_kat_llvm_mc_rejects_arity() {
    assert!(
        llvm_mc_word("mv a0, a1, a2").is_err(),
        "llvm-mc must reject extra operand"
    );
    assert!(
        llvm_mc_word("mv a0").is_err(),
        "llvm-mc must reject missing operand"
    );
    assert!(
        llvm_mc_word("mv fa0, a1").is_err(),
        "llvm-mc must reject FP dest"
    );
}

/// Regression: extra operand must be rejected (README two-operand form; llvm-mc errors).
#[test]
fn test_encode_mv_regression_extra_operand() {
    let ops = [reg("a0"), reg("a1"), Operand::Imm(0)];
    assert!(
        encode_mv(&ops).is_err(),
        "mv a0, a1 with a third operand must be rejected (llvm-mc rejects; README documents `mv rd, rs`); got {:?}",
        encode_mv(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_mv_diff_llvm_mc_add(rd in gpr_name(), rs in gpr_name()) {
        let asm = format!("add {}, x0, {}", rd, rs);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[reg(&rd), reg(&rs)])
            .unwrap_or_else(|e| panic!("SUT rejected valid mv {}, {}: {}", rd, rs, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_mv_sem_vs_llvm_mc_mv(rd in 1u32..=31, rs in reg_num()) {
        let init = init_regs();
        let sut = sut_word(&[reg(&xn(rd)), reg(&xn(rs))])
            .unwrap_or_else(|e| panic!("SUT rejected mv x{}, x{}: {}", rd, rs, e));
        let got = sim_copy(sut, init)
            .unwrap_or_else(|e| panic!("sim SUT mv x{}, x{}: {}", rd, rs, e));
        prop_assert_eq!(
            got[rd as usize], init[rs as usize],
            "SUT sim x{}={:#x} != init[x{}]={:#x} word={:#010x}",
            rd, got[rd as usize], rs, init[rs as usize], sut
        );

        let asm = format!("mv x{}, x{}", rd, rs);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let mc_got = sim_copy(mc, init)
            .unwrap_or_else(|e| panic!("sim llvm-mc {}: {}", asm, e));
        prop_assert_eq!(
            mc_got[rd as usize], init[rs as usize],
            "llvm-mc sim x{}={:#x} != init[x{}]={:#x} (mapping check)",
            rd, mc_got[rd as usize], rs, init[rs as usize]
        );
        prop_assert_eq!(
            got[rd as usize], mc_got[rd as usize],
            "SUT sim {:#x} != llvm-mc sim {:#x} for {}",
            got[rd as usize], mc_got[rd as usize], asm
        );
    }

    #[test]
    fn encode_mv_isa_fields(rd in reg_num(), rs in reg_num()) {
        let w = sut_word(&[reg(&xn(rd)), reg(&xn(rs))])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, x{}: {}", rd, rs, e));
        prop_assert_eq!(w & 0x7F, 0b0110011u32, "opcode");
        prop_assert_eq!((w >> 7) & 0x1F, rd, "rd");
        prop_assert_eq!((w >> 12) & 7, 0u32, "funct3");
        prop_assert_eq!((w >> 15) & 0x1F, 0u32, "rs1 must be x0");
        prop_assert_eq!((w >> 20) & 0x1F, rs, "rs2");
        prop_assert_eq!((w >> 25) & 0x7F, 0u32, "funct7");
    }

    #[test]
    fn encode_mv_abi_xn_alias(n in reg_num(), m in reg_num()) {
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
    fn encode_mv_field_isolation(
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
        prop_assert_eq!((ha >> 7) & 0x1F, (hb >> 7) & 0x1F, "rd field must be independent of rs");
        let ha2 = sut_word(&[reg(&xn(rd_a)), reg(&xn(rs))])
            .unwrap_or_else(|e| panic!("rd_a rejected: {}", e));
        let hb2 = sut_word(&[reg(&xn(rd_b)), reg(&xn(rs))])
            .unwrap_or_else(|e| panic!("rd_b rejected: {}", e));
        let rs2_mask = !0x0000_0F80u32;
        prop_assert_eq!(ha2 & rs2_mask, hb2 & rs2_mask, "rs2/op/funct3/rs1/funct7 bits must be independent of rd");
    }

    #[test]
    fn encode_mv_eq_add_x0(rd in reg_num(), rs in reg_num()) {
        let ops_mv = [reg(&xn(rd)), reg(&xn(rs))];
        let ops_add = [reg(&xn(rd)), reg("x0"), reg(&xn(rs))];
        let mv = sut_word(&ops_mv)
            .unwrap_or_else(|e| panic!("SUT rejected mv x{}, x{}: {}", rd, rs, e));
        let add = add_word(&ops_add)
            .unwrap_or_else(|e| panic!("SUT rejected add x{}, x0, x{}: {}", rd, rs, e));
        prop_assert_eq!(mv, add);
        let ops_zero = [reg(&xn(rd)), reg("zero"), reg(&xn(rs))];
        let add_z = add_word(&ops_zero)
            .unwrap_or_else(|e| panic!("SUT rejected add x{}, zero, x{}: {}", rd, rs, e));
        prop_assert_eq!(mv, add_z);
    }

    #[test]
    fn encode_mv_neg_extra(rd in gpr_name(), rs in gpr_name(), extra in extra_operand()) {
        let asm = format!("mv {}, {}", rd, rs);
        let ops = vec![reg(&rd), reg(&rs), extra];
        prop_assert!(
            encode_mv(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects: {}); got {:?}",
            asm,
            llvm_mc_rejects(&format!("{}, a2", asm)),
            encode_mv(&ops)
        );
    }

    #[test]
    fn encode_mv_neg_arity_invalid(
        ops in short_ops(),
        bad in invalid_operand(),
        good in gpr_name(),
        which in 0u8..=2u8,
    ) {
        prop_assume!(ops.len() < 2);
        prop_assert!(
            encode_mv(&ops).is_err(),
            "arity {} must Err, got {:?}",
            ops.len(),
            encode_mv(&ops)
        );
        let mixed: Vec<Operand> = match which {
            0 => vec![bad.clone(), bad.clone()],
            1 => vec![bad, reg(&good)],
            _ => vec![reg(&good), bad],
        };
        prop_assert!(
            encode_mv(&mixed).is_err(),
            "invalid operand must Err, got {:?}",
            encode_mv(&mixed)
        );
    }
}
