// Oracle: differential — llvm-mc RISC-V assembler (BGE expansion of BGEZ)
// Evidence: src/backend/riscv/assembler/README.md:329 `blez/bgez/...` → Corresponding `bge`/`blt` with x0;
//   encoder/mod.rs:3 Encodes RISC-V instructions into 32-bit machine code words;
//   encoder/mod.rs:893 "bgez" => encode_bgez;
//   pseudo.rs:309-315 encode_b(OP_BRANCH, 0b101, rs1, 0, 0) + RelocType::Branch // bge rs, x0;
//   RISC-V Unprivileged ISA pseudoinstruction BGEZ rs, offset = BGE rs, x0, offset.
// Stronger considered:
//   - State machine: rejected — encode_bgez is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree BGEZ/BGE decoder
//   - Differential vs encode_branch_instr(bge): rejected as primary — shared encode_b/get_reg
//     (same-job expansion used as a weaker metamorphic instead)
// Weaker available: algebraic.metamorphic (BGE rs,x0 expansion, ABI/xN alias, target forms),
//   algebraic.invariant (B-type field layout), negative_error (arity / invalid / extra)
// Differential: candidate=encode_bgez, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rs), Imm(0)|Symbol(lbl)] <-> `bgez rs, 0` / `bge rs, x0, 0`

use super::{encode_bgez, encode_branch_instr, EncodeResult, RelocType};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_BRANCH: u32 = 0b1100011;
const FUNCT3_BGE: u32 = 0b101;

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

/// Unpack B-type per RISC-V unprivileged ISA (not a copy of encode_b).
/// Layout: imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode
fn unpack_b(word: u32) -> (u32, u32, u32, u32, i32) {
    let opcode = word & 0x7f;
    let imm11 = (word >> 7) & 1;
    let bits4_1 = (word >> 8) & 0xf;
    let funct3 = (word >> 12) & 7;
    let rs1 = (word >> 15) & 0x1f;
    let rs2 = (word >> 20) & 0x1f;
    let bits10_5 = (word >> 25) & 0x3f;
    let bit12 = (word >> 31) & 1;
    let u13 = (bit12 << 12) | (imm11 << 11) | (bits10_5 << 5) | (bits4_1 << 1);
    let off = ((u13 as i32) << 19) >> 19;
    (opcode, funct3, rs1, rs2, off)
}

fn reloc_kind(t: &RelocType) -> &'static str {
    match t {
        RelocType::Branch => "Branch",
        other => panic!("unexpected reloc {other:?}"),
    }
}

fn sut_reloc(ops: &[Operand]) -> Result<(u32, &'static str, String, i64), String> {
    match encode_bgez(ops)? {
        EncodeResult::WordWithReloc { word, reloc } => {
            Ok((word, reloc_kind(&reloc.reloc_type), reloc.symbol, reloc.addend))
        }
        other => Err(format!("expected WordWithReloc, got {other:?}")),
    }
}

fn bge_reloc(ops: &[Operand]) -> Result<(u32, &'static str, String, i64), String> {
    match encode_branch_instr(ops, FUNCT3_BGE)? {
        EncodeResult::WordWithReloc { word, reloc } => {
            Ok((word, reloc_kind(&reloc.reloc_type), reloc.symbol, reloc.addend))
        }
        other => Err(format!("expected WordWithReloc, got {other:?}")),
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
        if p.starts_with("0b") {
            return Err(format!("unresolved fixup encoding {inner}"));
        }
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

fn ident() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("foo".into()),
        Just("bar".into()),
        Just("L0".into()),
        Just(".LBB0_1".into()),
        Just("my_label".into()),
        Just("_start".into()),
        Just("loop_top".into()),
        Just(".L1".into()),
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

fn invalid_rs() -> impl Strategy<Value = Operand> {
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

/// Target forms get_branch_target rejects (not Symbol/Label/Imm/Reg).
fn invalid_target() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
        Just(Operand::Mem {
            base: "x1".into(),
            offset: 0,
        }),
        Just(Operand::MemSymbol {
            base: "sp".into(),
            symbol: "foo".into(),
            modifier: "%lo".into(),
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        gpr_name().prop_map(|n| vec![reg(&n)]),
        invalid_rs().prop_map(|o| vec![o]),
        (-4i64..=40).prop_map(|i| vec![Operand::Imm(i)]),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_bgez_kat_llvm_mc() {
    // bgez a0, 0 = bge a0, x0, 0 = encoding [0x63,0x50,0x05,0x00] = 0x00055063
    let want = 0x0005_5063u32;
    let mc = llvm_mc_word("bgez a0, 0").expect("llvm-mc KAT bgez a0, 0");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for bgez a0, 0");
    let (w, kind, sym, addend) =
        sut_reloc(&[reg("a0"), Operand::Imm(0)]).expect("SUT KAT bgez a0, Imm(0)");
    assert_eq!(w, want);
    assert_eq!(kind, "Branch");
    assert_eq!(sym, "0");
    assert_eq!(addend, 0);

    // bgez zero, 0 = bge x0, x0, 0 = 0x00005063 (llvm may print as blez zero, 0)
    let want_z = 0x0000_5063u32;
    let mc_z = llvm_mc_word("bgez zero, 0").expect("llvm-mc KAT bgez zero, 0");
    assert_eq!(mc_z, want_z, "llvm-mc KAT mapping broken for bgez zero, 0");
    let (w, ..) = sut_reloc(&[reg("zero"), Operand::Imm(0)]).expect("SUT KAT zero");
    assert_eq!(w, want_z);
    let (w, ..) = sut_reloc(&[reg("x0"), Operand::Imm(0)]).expect("SUT KAT x0");
    assert_eq!(w, want_z);

    // bgez t6, 0 = encoding [0x63,0xd0,0x0f,0x00] = 0x000fd063
    let want_t6 = 0x000f_d063u32;
    let mc_t6 = llvm_mc_word("bgez t6, 0").expect("llvm-mc KAT bgez t6, 0");
    assert_eq!(mc_t6, want_t6, "llvm-mc KAT mapping broken for bgez t6, 0");
    let (w, ..) = sut_reloc(&[reg("t6"), Operand::Imm(0)]).expect("SUT KAT t6");
    assert_eq!(w, want_t6);
    let (w, ..) = sut_reloc(&[reg("x31"), Operand::Imm(0)]).expect("SUT KAT x31");
    assert_eq!(w, want_t6);

    // fp == s0 == x8; encoding [0x63,0x50,0x04,0x00] = 0x00045063
    let want_fp = 0x0004_5063u32;
    let mc_fp = llvm_mc_word("bgez fp, 0").expect("llvm-mc KAT bgez fp, 0");
    assert_eq!(mc_fp, want_fp, "llvm-mc KAT mapping broken for bgez fp, 0");
    let (w, ..) = sut_reloc(&[reg("fp"), Operand::Imm(0)]).expect("SUT KAT fp");
    assert_eq!(w, want_fp);
    let (w, ..) = sut_reloc(&[reg("x8"), Operand::Imm(0)]).expect("SUT KAT x8");
    assert_eq!(w, want_fp);

    // bgez ra, 0 = encoding [0x63,0xd0,0x00,0x00] = 0x0000d063
    let want_ra = 0x0000_d063u32;
    let mc_ra = llvm_mc_word("bgez ra, 0").expect("llvm-mc KAT bgez ra, 0");
    assert_eq!(mc_ra, want_ra, "llvm-mc KAT mapping broken for bgez ra, 0");
    let (w, ..) = sut_reloc(&[reg("ra"), Operand::Imm(0)]).expect("SUT KAT ra");
    assert_eq!(w, want_ra);

    let mc_bge = llvm_mc_word("bge a0, x0, 0").expect("llvm-mc KAT bge a0, x0, 0");
    assert_eq!(mc_bge, want, "bge a0, x0, 0 must encode as bgez a0, 0");

    // Label form: word still zero-imm BGE, reloc carries the symbol.
    let (w, kind, sym, addend) =
        sut_reloc(&[reg("a0"), Operand::Symbol("foo".into())]).expect("SUT symbol KAT");
    assert_eq!(w, want);
    assert_eq!(kind, "Branch");
    assert_eq!(sym, "foo");
    assert_eq!(addend, 0);

    assert!(
        llvm_mc_word("bgez a0, 0, a1").is_err(),
        "llvm-mc must reject extra operand"
    );
    assert!(llvm_mc_word("bgez a0").is_err(), "llvm-mc must reject missing operand");
}

/// Regression: extra operand must be rejected (README two-operand form; llvm-mc errors).
#[test]
fn test_encode_bgez_regression_extra_operand() {
    let ops = [
        reg("a0"),
        Operand::Symbol("foo".into()),
        reg("a1"),
    ];
    assert!(
        encode_bgez(&ops).is_err(),
        "bgez a0, foo with a third operand must be rejected (llvm-mc rejects; README documents `bgez rs, label`); got {:?}",
        encode_bgez(&ops)
    );
}

proptest! {
    #![proptest_config(cfg())]

    /// Differential: SUT word for bgez rs, Imm(0) matches llvm-mc `bgez rs, 0`.
    #[test]
    fn encode_bgez_diff_llvm_mc(rs in gpr_name()) {
        let asm = format!("bgez {}, 0", rs);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let (sut, kind, sym, addend) = sut_reloc(&[reg(&rs), Operand::Imm(0)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
        prop_assert_eq!(kind, "Branch");
        prop_assert_eq!(sym, "0");
        prop_assert_eq!(addend, 0i64);
    }

    /// Sweep: documented expansion vs independent assembler (bge rs, x0, 0).
    #[test]
    fn encode_bgez_diff_llvm_mc_bge(rs in gpr_name()) {
        let asm_bge = format!("bge {}, x0, 0", rs);
        let mc = llvm_mc_word(&asm_bge)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm_bge, e));
        let (sut, ..) = sut_reloc(&[reg(&rs), Operand::Imm(0)])
            .unwrap_or_else(|e| panic!("SUT rejected valid bgez {}, 0: {}", rs, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc bge {:08x} for {}", sut, mc, asm_bge);
    }

    /// Metamorphic: bgez rs, label ≡ bge rs, x0, label (WordWithReloc).
    #[test]
    fn encode_bgez_eq_bge_rs_x0(rs in reg_num(), tgt in ident()) {
        let ops_bgez = [reg(&xn(rs)), Operand::Symbol(tgt.clone())];
        let ops_bge = [reg(&xn(rs)), reg("x0"), Operand::Symbol(tgt.clone())];
        let bgez = sut_reloc(&ops_bgez)
            .unwrap_or_else(|e| panic!("SUT rejected bgez x{}, {}: {}", rs, tgt, e));
        let bge = bge_reloc(&ops_bge)
            .unwrap_or_else(|e| panic!("SUT rejected bge x{}, x0, {}: {}", rs, tgt, e));
        prop_assert_eq!(bgez.clone(), bge);
        let ops_zero = [reg(&xn(rs)), reg("zero"), Operand::Symbol(tgt.clone())];
        let bge_z = bge_reloc(&ops_zero)
            .unwrap_or_else(|e| panic!("SUT rejected bge x{}, zero, {}: {}", rs, tgt, e));
        prop_assert_eq!(bgez, bge_z);
    }

    #[test]
    fn encode_bgez_isa_b_type(rs in reg_num(), tgt in ident()) {
        let (w, kind, sym, addend) = sut_reloc(&[reg(&xn(rs)), Operand::Symbol(tgt.clone())])
            .unwrap_or_else(|e| panic!("SUT rejected x{}, {}: {}", rs, tgt, e));
        let (opc, f3, got_rs1, got_rs2, off) = unpack_b(w);
        prop_assert_eq!(opc, OP_BRANCH, "opcode");
        prop_assert_eq!(f3, FUNCT3_BGE, "funct3 BGE");
        prop_assert_eq!(got_rs1, rs, "rs1");
        prop_assert_eq!(got_rs2, 0u32, "rs2 must be x0");
        prop_assert_eq!(off, 0i32, "imm deferred to reloc");
        prop_assert_eq!(kind, "Branch");
        prop_assert_eq!(sym, tgt);
        prop_assert_eq!(addend, 0i64);
    }

    #[test]
    fn encode_bgez_abi_xn_alias(n in reg_num(), tgt in ident()) {
        let via_x = sut_reloc(&[reg(&xn(n)), Operand::Symbol(tgt.clone())])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_reloc(&[reg(abi_name(n)), Operand::Symbol(tgt.clone())])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(via_abi, via_x.clone());
        if n == 8 {
            let via_fp = sut_reloc(&[reg("fp"), Operand::Symbol(tgt.clone())])
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    /// Symbol / Label / Reg-as-label targets with the same string are equivalent.
    #[test]
    fn encode_bgez_target_forms(rs in reg_num(), s in ident()) {
        let via_sym = sut_reloc(&[reg(&xn(rs)), Operand::Symbol(s.clone())])
            .unwrap_or_else(|e| panic!("Symbol rejected: {}", e));
        let via_lbl = sut_reloc(&[reg(&xn(rs)), Operand::Label(s.clone())])
            .unwrap_or_else(|e| panic!("Label rejected: {}", e));
        let via_reg = sut_reloc(&[reg(&xn(rs)), Operand::Reg(s.clone())])
            .unwrap_or_else(|e| panic!("Reg-as-label rejected: {}", e));
        prop_assert_eq!(via_sym.clone(), via_lbl);
        prop_assert_eq!(via_sym, via_reg);
    }

    /// Imm target is stringified into the reloc symbol; word stays zero-imm BGE.
    #[test]
    fn encode_bgez_imm_target(
        rs in reg_num(),
        imm in prop_oneof![Just(0i64), Just(4i64), Just(-4i64), Just(4094i64), Just(-4096i64), -64i64..=64]
    ) {
        let (w, kind, sym, addend) = sut_reloc(&[reg(&xn(rs)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("Imm target rejected: {}", e));
        let (opc, f3, got_rs1, got_rs2, off) = unpack_b(w);
        prop_assert_eq!(opc, OP_BRANCH);
        prop_assert_eq!(f3, FUNCT3_BGE);
        prop_assert_eq!(got_rs1, rs);
        prop_assert_eq!(got_rs2, 0u32);
        prop_assert_eq!(off, 0i32, "bgez always encodes imm=0; reloc carries target");
        prop_assert_eq!(kind, "Branch");
        prop_assert_eq!(sym, format!("{}", imm));
        prop_assert_eq!(addend, 0i64);
    }

    /// Sweep: bare Imm(0..31) as rs (get_reg GCC bare-number path) equals xN.
    #[test]
    fn encode_bgez_imm_as_rs(n in reg_num(), tgt in ident()) {
        let via_imm = sut_reloc(&[Operand::Imm(n as i64), Operand::Symbol(tgt.clone())])
            .unwrap_or_else(|e| panic!("Imm rs rejected: {}", e));
        let via_xn = sut_reloc(&[reg(&xn(n)), Operand::Symbol(tgt.clone())])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        prop_assert_eq!(via_imm, via_xn);
    }

    #[test]
    fn encode_bgez_neg_arity(ops in short_ops()) {
        prop_assume!(ops.len() < 2);
        prop_assert!(encode_bgez(&ops).is_err(), "arity {} must Err", ops.len());
    }

    #[test]
    fn encode_bgez_neg_invalid_rs(bad in invalid_rs(), tgt in ident()) {
        let ops = vec![bad, Operand::Symbol(tgt)];
        prop_assert!(
            encode_bgez(&ops).is_err(),
            "invalid rs must Err, got {:?}",
            encode_bgez(&ops)
        );
    }

    #[test]
    fn encode_bgez_neg_invalid_target(rs in gpr_name(), bad in invalid_target()) {
        let ops = vec![reg(&rs), bad];
        prop_assert!(
            encode_bgez(&ops).is_err(),
            "invalid target must Err, got {:?}",
            encode_bgez(&ops)
        );
    }

    #[test]
    fn encode_bgez_neg_extra(rs in gpr_name(), tgt in ident(), extra in extra_operand()) {
        let asm = format!("bgez {}, {}", rs, tgt);
        let ops = vec![reg(&rs), Operand::Symbol(tgt), extra];
        prop_assert!(
            encode_bgez(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects: {})",
            asm,
            llvm_mc_rejects(&format!("{}, a1", asm))
        );
    }
}
