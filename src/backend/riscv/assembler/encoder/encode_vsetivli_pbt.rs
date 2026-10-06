// Oracle: differential — llvm-mc RISC-V assembler (RVV vsetivli)
// Evidence: vector.rs:57-58 "Encode vsetivli rd, uimm[4:0], vtypei" /
//   "Format: [11][vtypei[9:0]][uimm[4:0]][111][rd][1010111]";
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:944-945 "vsetivli" => encode_vsetivli(operands) (operands passed through);
//   assembler/README.md:14 V (vector) standard extension; assembler/README.md:109 vector.rs RVV;
//   RISC-V V 1.0: opcode=1010111, funct3=111, bits[31:30]=11, zimm[9:0]=vtypei,
//   uimm[4:0] AVL, vtypei = {vma, vta, vsew[2:0], vlmul[2:0]}.
// Stronger considered:
//   - State machine: rejected — encode_vsetivli is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no vsetivli decoder
//   - encode_vsetvli / encode_vsetvl as differential sibling: rejected —
//     same-job gate (vsetvli bit31=0 and rs1; vsetvl register vtype)
// Weaker available: algebraic.invariant (field unpack), algebraic.metamorphic
//   (ABI vs xN alias; field isolation), negative_error (arity / FP / extra / AVL)
// Differential: candidate=encode_vsetivli,
//   reference=llvm-mc -triple=riscv64 -mattr=+v -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Imm(uimm), Symbol(eSEW), Symbol(mLMUL), Symbol(ta|tu),
//            Symbol(ma|mu)] <-> `vsetivli rd, uimm, eSEW, mLMUL, ta|tu, ma|mu`;
//   [Reg(rd), Imm(uimm), Imm(vtypei)] <-> `vsetivli rd, uimm, vtypei`.

use super::{encode_vsetivli, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_V: u32 = 0b1010111;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3",
    "a4", "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s9", "s10", "s11",
    "t3", "t4", "t5", "t6",
];

const SEW: [&str; 4] = ["e8", "e16", "e32", "e64"];
const SEW_ENC: [u32; 4] = [0, 1, 2, 3];
const WIDE_SEW: [&str; 4] = ["e128", "e256", "e512", "e1024"];
const LMUL: [&str; 7] = ["m1", "m2", "m4", "m8", "mf2", "mf4", "mf8"];
const LMUL_ENC: [u32; 7] = [0b000, 0b001, 0b010, 0b011, 0b111, 0b110, 0b101];
const TA: [&str; 2] = ["tu", "ta"];
const MA: [&str; 2] = ["mu", "ma"];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xn(n: u32) -> String {
    format!("x{n}")
}

fn abi_name(n: u32) -> &'static str {
    ABI[n as usize]
}

fn reg(name: &str) -> Operand {
    Operand::Reg(name.to_string())
}

fn sym(name: &str) -> Operand {
    Operand::Symbol(name.to_string())
}

fn named_ops(rd: &str, uimm: i64, sew: &str, lmul: &str, ta: &str, ma: &str) -> Vec<Operand> {
    vec![
        reg(rd),
        Operand::Imm(uimm),
        sym(sew),
        sym(lmul),
        sym(ta),
        sym(ma),
    ]
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_vsetivli(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack vsetivli per RISC-V V 1.0 (not a copy of encode_vsetivli).
fn unpack_vsetivli(word: u32) -> (u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let uimm = (word >> 15) & 0x1f;
    let vtypei = (word >> 20) & 0x3ff;
    let bits31_30 = word >> 30;
    (opcode, rd, funct3, uimm, vtypei, bits31_30)
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
        .args(["-triple=riscv64", "-mattr=+v", "-show-encoding"])
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

fn gpr_name() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(xn),
        (0u32..=31).prop_map(|n| abi_name(n).to_string()),
        Just("fp".into()),
    ]
}

fn gpr_n() -> impl Strategy<Value = u32> {
    0u32..=31
}

fn uimm5() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(31u32), 0u32..=31]
}

fn sew_name() -> impl Strategy<Value = String> {
    prop::sample::select(SEW.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

fn lmul_name() -> impl Strategy<Value = String> {
    prop::sample::select(LMUL.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

fn ta_name() -> impl Strategy<Value = String> {
    prop::sample::select(TA.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

fn ma_name() -> impl Strategy<Value = String> {
    prop::sample::select(MA.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

fn wide_sew_name() -> impl Strategy<Value = String> {
    prop::sample::select(WIDE_SEW.iter().map(|s| s.to_string()).collect::<Vec<_>>())
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(208)),
        (0u32..=31).prop_map(|n| Operand::Reg(xn(n))),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Symbol("e8".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::SymbolOffset("foo".into(), 4)),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
    ]
}

fn fp_or_vreg() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=31).prop_map(|n| format!("f{n}")),
        (0u32..=31).prop_map(|n| format!("v{n}")),
        Just("fa0".into()),
        Just("ft0".into()),
        Just("fs0".into()),
        Just("fa7".into()),
        Just("ft11".into()),
        Just("v0".into()),
        Just("v31".into()),
    ]
}

fn short_ops() -> impl Strategy<Value = Vec<Operand>> {
    prop_oneof![
        Just(vec![]),
        gpr_name().prop_map(|rd| vec![reg(&rd)]),
        (gpr_name(), uimm5()).prop_map(|(rd, u)| vec![reg(&rd), Operand::Imm(u as i64)]),
    ]
}

fn isa_vtypei(sew: &str, lmul: &str, ta: &str, ma: &str) -> u32 {
    let sew_i = SEW
        .iter()
        .position(|s| *s == sew)
        .map(|i| SEW_ENC[i])
        .unwrap_or(0);
    let lmul_i = LMUL
        .iter()
        .position(|s| *s == lmul)
        .map(|i| LMUL_ENC[i])
        .unwrap_or(0);
    let ta_i = if ta == "ta" { 1 } else { 0 };
    let ma_i = if ma == "ma" { 1 } else { 0 };
    (ma_i << 7) | (ta_i << 6) | (sew_i << 3) | lmul_i
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_vsetivli_kat_llvm_mc_a0_1_e32_m1_ta_ma() {
    let want = 0xcd00f557u32;
    let mc = llvm_mc_word("vsetivli a0, 1, e32, m1, ta, ma").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT drifted: {mc:#x} != {want:#x}");
    let sut = sut_word(&named_ops("a0", 1, "e32", "m1", "ta", "ma")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetivli_kat_llvm_mc_a0_0_e8_m8_tu_mu() {
    let want = 0xc0307557u32;
    let mc = llvm_mc_word("vsetivli a0, 0, e8, m8, tu, mu").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&named_ops("a0", 0, "e8", "m8", "tu", "mu")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetivli_kat_llvm_mc_imm0() {
    let want = 0xc000f557u32;
    let mc = llvm_mc_word("vsetivli a0, 1, 0").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&[reg("a0"), Operand::Imm(1), Operand::Imm(0)]).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetivli_kat_llvm_mc_a0_31_e64_mf2() {
    let want = 0xcdfff557u32;
    let mc = llvm_mc_word("vsetivli a0, 31, e64, mf2, ta, ma").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&named_ops("a0", 31, "e64", "mf2", "ta", "ma")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetivli_kat_llvm_mc_x10_5() {
    let want = 0xc8e2f557u32;
    let mc = llvm_mc_word("vsetivli x10, 5, e16, mf4, tu, ma").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&named_ops("x10", 5, "e16", "mf4", "tu", "ma")).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_vsetivli_kat_llvm_mc_zero_0() {
    let want = 0xc0007057u32;
    let mc = llvm_mc_word("vsetivli zero, 0, e8, m1, tu, mu").expect("llvm-mc KAT");
    assert_eq!(mc, want);
    let sut = sut_word(&named_ops("zero", 0, "e8", "m1", "tu", "mu")).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_vsetivli_diff_llvm_mc_named(
        rd in gpr_name(),
        uimm in uimm5(),
        sew in sew_name(),
        lmul in lmul_name(),
        ta in ta_name(),
        ma in ma_name(),
    ) {
        let asm = format!("vsetivli {rd}, {uimm}, {sew}, {lmul}, {ta}, {ma}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&named_ops(&rd, uimm as i64, &sew, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vsetivli_diff_llvm_mc_imm(
        rd in gpr_name(),
        uimm in uimm5(),
        v in prop_oneof![Just(0u32), Just(1u32), Just(208u32), Just(512u32), Just(1023u32), 0u32..=1023],
    ) {
        let asm = format!("vsetivli {rd}, {uimm}, {v}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&[reg(&rd), Operand::Imm(uimm as i64), Operand::Imm(v as i64)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vsetivli_format_fields(
        rd in gpr_n(),
        uimm in uimm5(),
        sew in sew_name(),
        lmul in lmul_name(),
        ta in ta_name(),
        ma in ma_name(),
    ) {
        let ops = named_ops(&xn(rd), uimm as i64, &sew, &lmul, &ta, &ma);
        let w = sut_word(&ops).unwrap_or_else(|e| panic!("SUT rejected: {e}"));
        let (opcode, got_rd, funct3, got_uimm, vtypei, bits31_30) = unpack_vsetivli(w);
        prop_assert_eq!(opcode, OP_V, "opcode");
        prop_assert_eq!(funct3, 0b111, "funct3");
        prop_assert_eq!(bits31_30, 0b11, "bits[31:30] must be 11 for vsetivli");
        prop_assert_eq!(got_rd, rd, "rd");
        prop_assert_eq!(got_uimm, uimm, "uimm");
        prop_assert_eq!(vtypei, isa_vtypei(&sew, &lmul, &ta, &ma), "vtypei packing");
    }

    #[test]
    fn encode_vsetivli_abi_xn_alias(
        n in gpr_n(),
        uimm in uimm5(),
        sew in sew_name(),
        lmul in lmul_name(),
        ta in ta_name(),
        ma in ma_name(),
    ) {
        let via_x = sut_word(&named_ops(&xn(n), uimm as i64, &sew, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("xN rejected: {e}"));
        let via_abi = sut_word(&named_ops(abi_name(n), uimm as i64, &sew, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("ABI rejected: {e}"));
        prop_assert_eq!(via_abi, via_x);
        if n == 8 {
            let via_fp = sut_word(&named_ops("fp", uimm as i64, &sew, &lmul, &ta, &ma))
                .unwrap_or_else(|e| panic!("fp rd rejected: {e}"));
            prop_assert_eq!(via_fp, via_x);
        }
    }

    #[test]
    fn encode_vsetivli_field_isolation(
        rd_a in gpr_n(),
        rd_b in gpr_n(),
        uimm_a in uimm5(),
        uimm_b in uimm5(),
        sew_a in sew_name(),
        sew_b in sew_name(),
        lmul in lmul_name(),
        ta in ta_name(),
        ma in ma_name(),
    ) {
        let wa = sut_word(&named_ops(&xn(rd_a), uimm_a as i64, &sew_a, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("a rejected: {e}"));
        let wb = sut_word(&named_ops(&xn(rd_b), uimm_a as i64, &sew_a, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("b rejected: {e}"));
        let rd_mask = 0x1fu32 << 7;
        prop_assert_eq!(wa & !rd_mask, wb & !rd_mask, "non-rd bits independent of rd");
        prop_assert_eq!((wa >> 7) & 0x1f, rd_a);
        prop_assert_eq!((wb >> 7) & 0x1f, rd_b);

        let wc = sut_word(&named_ops(&xn(rd_a), uimm_b as i64, &sew_a, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("c rejected: {e}"));
        let uimm_mask = 0x1fu32 << 15;
        prop_assert_eq!(wa & !uimm_mask, wc & !uimm_mask, "non-uimm bits independent of uimm");
        prop_assert_eq!((wc >> 15) & 0x1f, uimm_b);

        let wd = sut_word(&named_ops(&xn(rd_a), uimm_a as i64, &sew_b, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("d rejected: {e}"));
        let vtype_mask = 0x3ffu32 << 20;
        prop_assert_eq!(wa & !vtype_mask, wd & !vtype_mask, "non-vtypei bits independent of vtypei");
    }

    #[test]
    fn encode_vsetivli_neg_arity_fp(ops in short_ops(), fp in fp_or_vreg()) {
        prop_assert!(
            encode_vsetivli(&ops).is_err(),
            "arity {} must Err (llvm-mc too few operands); got {:?}",
            ops.len(),
            encode_vsetivli(&ops)
        );
        let fp_rd = named_ops(&fp, 1, "e32", "m1", "ta", "ma");
        prop_assert!(
            encode_vsetivli(&fp_rd).is_err(),
            "FP/vector rd {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_vsetivli(&fp_rd)
        );
    }

    #[test]
    fn encode_vsetivli_neg_fp(fp in fp_or_vreg()) {
        let fp_rd = named_ops(&fp, 1, "e32", "m1", "ta", "ma");
        prop_assert!(
            encode_vsetivli(&fp_rd).is_err(),
            "FP/vector rd {} must Err (llvm-mc invalid operand); got {:?}",
            fp,
            encode_vsetivli(&fp_rd)
        );
    }

    #[test]
    fn encode_vsetivli_neg_extra(
        rd in gpr_name(),
        uimm in uimm5(),
        sew in sew_name(),
        lmul in lmul_name(),
        ta in ta_name(),
        ma in ma_name(),
        extra in extra_operand(),
    ) {
        let mut ops = named_ops(&rd, uimm as i64, &sew, &lmul, &ta, &ma);
        ops.push(extra.clone());
        prop_assert!(
            encode_vsetivli(&ops).is_err(),
            "extra operand {:?} must Err for vsetivli (llvm-mc rejects extra); got {:?}",
            extra,
            encode_vsetivli(&ops)
        );
    }

    #[test]
    fn encode_vsetivli_diff_llvm_mc_wide_sew(
        rd in gpr_name(),
        uimm in uimm5(),
        sew in wide_sew_name(),
        lmul in lmul_name(),
        ta in ta_name(),
        ma in ma_name(),
    ) {
        let asm = format!("vsetivli {rd}, {uimm}, {sew}, {lmul}, {ta}, {ma}");
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        let sut = sut_word(&named_ops(&rd, uimm as i64, &sew, &lmul, &ta, &ma))
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT {:#010x} != llvm-mc {:#010x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_vsetivli_neg_uimm_oob(
        rd in gpr_name(),
        uimm in prop_oneof![
            Just(-1i64),
            Just(32i64),
            Just(33i64),
            Just(63i64),
            Just(64i64),
            Just(i64::MIN),
            Just(i64::MAX),
            (-128i64..=-1),
            (32i64..=128),
        ],
        sew in sew_name(),
        lmul in lmul_name(),
        ta in ta_name(),
        ma in ma_name(),
    ) {
        let ops = named_ops(&rd, uimm, &sew, &lmul, &ta, &ma);
        prop_assert!(
            encode_vsetivli(&ops).is_err(),
            "AVL {uimm} outside 0..=31 must Err (llvm-mc: immediate must be in [0, 31]); got {:?}",
            encode_vsetivli(&ops)
        );
    }

    #[test]
    fn encode_vsetivli_neg_vtypei_imm_oob(
        rd in gpr_name(),
        uimm in uimm5(),
        v in prop_oneof![
            Just(1024u32),
            Just(1025u32),
            Just(2047u32),
            Just(2048u32),
            1024u32..=2047,
        ],
    ) {
        let ops = [reg(&rd), Operand::Imm(uimm as i64), Operand::Imm(v as i64)];
        prop_assert!(
            encode_vsetivli(&ops).is_err(),
            "vtypei imm {v} outside 0..=1023 must Err (llvm-mc rejects 10-bit overflow); got {:?}",
            encode_vsetivli(&ops)
        );
    }
}

/// Regression: RVV 1.0 SEW e128 must encode (llvm-mc accepts; vsew=100).
#[test]
fn test_encode_vsetivli_regression_wide_sew_e128() {
    let want = 0xc2007057u32;
    let mc = llvm_mc_word("vsetivli x0, 0, e128, m1, tu, mu")
        .expect("llvm-mc accepts e128");
    assert_eq!(mc, want);
    let sut = sut_word(&named_ops("x0", 0, "e128", "m1", "tu", "mu"));
    assert_eq!(
        sut,
        Ok(want),
        "vsetivli x0, 0, e128, m1, tu, mu must match llvm-mc {want:#010x}; got {sut:?}"
    );
}

/// Regression: two-operand vsetivli (missing vtypei) must Err.
#[test]
fn test_encode_vsetivli_regression_arity_two() {
    let ops = [reg("x0"), Operand::Imm(0)];
    let got = encode_vsetivli(&ops);
    assert!(
        got.is_err(),
        "vsetivli x0, 0 (no vtypei) must Err; got {got:?}"
    );
}

/// Regression: extra vtype field after a complete named vtype must Err.
#[test]
fn test_encode_vsetivli_regression_extra_operand() {
    let mut ops = named_ops("x0", 0, "e8", "m1", "tu", "mu");
    ops.push(Operand::Imm(0));
    let got = encode_vsetivli(&ops);
    assert!(
        got.is_err(),
        "vsetivli x0, 0, e8, m1, tu, mu, 0 must Err; got {got:?}"
    );
}

/// Regression: AVL outside the 5-bit uimm field must Err.
#[test]
fn test_encode_vsetivli_regression_uimm_oob() {
    let ops = named_ops("x0", 32, "e8", "m1", "tu", "mu");
    let got = encode_vsetivli(&ops);
    assert!(
        got.is_err(),
        "vsetivli x0, 32, e8, m1, tu, mu must Err; got {got:?}"
    );
}

/// Regression: raw vtypei immediate above 10 bits must Err.
#[test]
fn test_encode_vsetivli_regression_vtypei_imm_oob() {
    let ops = [reg("x0"), Operand::Imm(0), Operand::Imm(1024)];
    let got = encode_vsetivli(&ops);
    assert!(
        got.is_err(),
        "vsetivli x0, 0, 1024 must Err; got {got:?}"
    );
}
