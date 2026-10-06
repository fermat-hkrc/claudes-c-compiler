// Oracle: differential — llvm-mc RISC-V assembler (I-type MISC-MEM FENCE)
// Evidence: src/backend/riscv/assembler/README.md:6-7 textual assembly from codegen;
//   README.md:311-312 System: ecall, ebreak, fence, fence.i, csrr/csrw/csrs/csrc
//   and their immediate variants (csrwi, csrsi, csrci);
//   README.md:179 Operand::FenceArg fence operand "iorw";
//   README.md:353 I-type: [imm[11:0] | rs1 | funct3 | rd | opcode];
//   encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words";
//   encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen (RV64GC + Zbb).";
//   encoder/mod.rs:303 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]";
//   encoder/mod.rs:357 OP_MISC_MEM = 0b0001111;
//   encoder/mod.rs:682 "fence" => encode_fence(operands);
//   system.rs:7 empty operands comment "fence iorw, iorw";
//   RISC-V Unprivileged ISA FENCE: opcode=0001111, rd=0, funct3=000, rs1=0,
//   imm[11:0] = fm[31:28]|pred[27:24]|succ[23:20] with fm=0000, I=8 O=4 R=2 W=1;
//   llvm-mc: operand must be letters selected in-order from 'iorw' or be 0.
// Stronger considered:
//   - State machine: rejected — encode_fence is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no FENCE decoder
//   - encode_i / fence.i / fence.tso as differential sibling: rejected —
//     same-job gate (private packer / different mnemonics)
// Weaker available: algebraic.invariant (I-type field unpack), algebraic.metamorphic
//   (empty == iorw,iorw), negative_error (extra / arity-1 / out-of-order / invalid kind)
// Differential: candidate=encode_fence, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[] <-> `fence`; [FenceArg(pred), FenceArg(succ)] <-> `fence pred, succ`;
//   [Imm(0), ...] <-> `fence 0, ...` (parser emits Imm for numeric 0).

use super::{encode_fence, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const OP_MISC_MEM: u32 = 0b0001111;

/// Non-empty subsequences of iorw selected in order, with ISA pred/succ masks.
/// I=8, O=4, R=2, W=1 per the RISC-V unprivileged ISA (not parse_fence_bits).
const IORW: [(&str, u32); 15] = [
    ("i", 8),
    ("o", 4),
    ("r", 2),
    ("w", 1),
    ("io", 12),
    ("ir", 10),
    ("iw", 9),
    ("or", 6),
    ("ow", 5),
    ("rw", 3),
    ("ior", 14),
    ("iow", 13),
    ("irw", 11),
    ("orw", 7),
    ("iorw", 15),
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn fence_arg(s: &str) -> Operand {
    Operand::FenceArg(s.to_string())
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_fence(ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
    }
}

/// Unpack FENCE I-type per RISC-V unprivileged ISA (not a copy of encode_i).
/// Layout: fm[31:28] | pred[27:24] | succ[23:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]
fn unpack_fence(word: u32) -> (u32, u32, u32, u32, u32, u32, u32) {
    let opcode = word & 0x7f;
    let rd = (word >> 7) & 0x1f;
    let funct3 = (word >> 12) & 0x7;
    let rs1 = (word >> 15) & 0x1f;
    let succ = (word >> 20) & 0xf;
    let pred = (word >> 24) & 0xf;
    let fm = (word >> 28) & 0xf;
    (opcode, rd, funct3, rs1, succ, pred, fm)
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

fn iorw_pair() -> impl Strategy<Value = ((&'static str, u32), (&'static str, u32))> {
    (
        prop::sample::select(IORW.to_vec()),
        prop::sample::select(IORW.to_vec()),
    )
}

fn fence_token() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("0"),
        prop::sample::select(IORW.iter().map(|(s, _)| *s).collect::<Vec<_>>()),
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Reg("x0".into())),
        Just(Operand::Reg("a0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::FenceArg("iorw".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 8,
        }),
    ]
}

fn one_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(fence_arg("iorw")),
        Just(fence_arg("rw")),
        Just(fence_arg("i")),
        Just(Operand::Imm(0)),
        Just(Operand::Reg("x0".into())),
        Just(Operand::Symbol("foo".into())),
    ]
}

fn invalid_letters() -> impl Strategy<Value = &'static str> {
    prop_oneof![
        Just("wroi"),
        Just("irow"),
        Just("ri"),
        Just("wi"),
        Just("oi"),
        Just("wr"),
        Just("ro"),
        Just("wo"),
        Just("ii"),
        Just("rr"),
        Just("ww"),
        Just("oo"),
        Just("IORW"),
        Just("I"),
        Just("Rw"),
        Just("RW"),
    ]
}

fn invalid_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Reg("x0".into())),
        Just(Operand::Reg("a0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Csr("mstatus".into())),
        Just(Operand::RoundingMode("rne".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Imm(1)),
        Just(Operand::Imm(15)),
        Just(Operand::Imm(-1)),
        Just(Operand::Imm(16)),
        Just(Operand::Imm(255)),
        Just(Operand::Mem {
            base: "sp".into(),
            offset: 0,
        }),
    ]
}

fn op_fence_token(tok: &str) -> Operand {
    if tok == "0" {
        Operand::Imm(0)
    } else {
        fence_arg(tok)
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_fence_kat_llvm_mc_x0_x1() {
    let want_full = 0x0ff0_000fu32;
    let mc_bare = llvm_mc_word("fence").expect("llvm-mc KAT fence");
    assert_eq!(mc_bare, want_full, "llvm-mc KAT mapping broken for bare fence");
    let mc_iorw = llvm_mc_word("fence iorw, iorw").expect("llvm-mc KAT fence iorw, iorw");
    assert_eq!(mc_iorw, want_full);
    let sut_empty = sut_word(&[]).expect("SUT KAT empty fence");
    assert_eq!(sut_empty, want_full);
    let sut_iorw = sut_word(&[fence_arg("iorw"), fence_arg("iorw")]).expect("SUT KAT iorw,iorw");
    assert_eq!(sut_iorw, want_full);
}

#[test]
fn encode_fence_kat_llvm_mc_rw_variants() {
    let want_rw = 0x0330_000fu32;
    assert_eq!(llvm_mc_word("fence rw, rw").unwrap(), want_rw);
    assert_eq!(
        sut_word(&[fence_arg("rw"), fence_arg("rw")]).unwrap(),
        want_rw
    );

    let want_acq = 0x0230_000fu32;
    assert_eq!(llvm_mc_word("fence r, rw").unwrap(), want_acq);
    assert_eq!(
        sut_word(&[fence_arg("r"), fence_arg("rw")]).unwrap(),
        want_acq
    );

    let want_rel = 0x0310_000fu32;
    assert_eq!(llvm_mc_word("fence rw, w").unwrap(), want_rel);
    assert_eq!(
        sut_word(&[fence_arg("rw"), fence_arg("w")]).unwrap(),
        want_rel
    );

    let want_io = 0x0840_000fu32;
    assert_eq!(llvm_mc_word("fence i, o").unwrap(), want_io);
    assert_eq!(
        sut_word(&[fence_arg("i"), fence_arg("o")]).unwrap(),
        want_io
    );
}

#[test]
fn encode_fence_kat_llvm_mc_numeric_zero() {
    let want_00 = 0x0000_000fu32;
    assert_eq!(llvm_mc_word("fence 0, 0").unwrap(), want_00);
    assert_eq!(
        sut_word(&[Operand::Imm(0), Operand::Imm(0)]).unwrap(),
        want_00
    );

    let want_0rw = 0x0030_000fu32;
    assert_eq!(llvm_mc_word("fence 0, rw").unwrap(), want_0rw);
    assert_eq!(
        sut_word(&[Operand::Imm(0), fence_arg("rw")]).unwrap(),
        want_0rw
    );
}

/// Regression: extra operand must be rejected (llvm-mc errors).
#[test]
fn test_encode_fence_regression_extra_operand() {
    let ops = [
        fence_arg("iorw"),
        fence_arg("iorw"),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_fence(&ops).is_err(),
        "fence iorw, iorw, x0 must Err; got {:?}",
        encode_fence(&ops)
    );
}

/// Regression: a single operand must be rejected (llvm-mc too few operands).
#[test]
fn test_encode_fence_regression_arity_one() {
    let ops = [fence_arg("iorw")];
    assert!(
        encode_fence(&ops).is_err(),
        "fence iorw (one operand) must Err; got {:?}",
        encode_fence(&ops)
    );
}

/// Regression: out-of-order letters must be rejected (llvm-mc in-order iorw).
#[test]
fn test_encode_fence_regression_out_of_order() {
    let ops = [fence_arg("wroi"), fence_arg("iorw")];
    assert!(
        encode_fence(&ops).is_err(),
        "fence wroi, iorw must Err; got {:?}",
        encode_fence(&ops)
    );
}

/// Regression: Imm(0) must encode pred/succ = 0, not default to iorw.
#[test]
fn test_encode_fence_regression_imm0() {
    let got = sut_word(&[Operand::Imm(0), Operand::Imm(0)]);
    assert_eq!(
        got,
        Ok(0x0000_000fu32),
        "fence 0, 0 must encode 0x0000000f; got {got:?}"
    );
}

/// Regression: non-zero immediates / registers must be rejected.
#[test]
fn test_encode_fence_regression_invalid_operand() {
    let ops = [Operand::Imm(1), fence_arg("rw")];
    assert!(
        encode_fence(&ops).is_err(),
        "fence 1, rw must Err; got {:?}",
        encode_fence(&ops)
    );
    let ops_reg = [Operand::Reg("x0".into()), Operand::Reg("x0".into())];
    assert!(
        encode_fence(&ops_reg).is_err(),
        "fence x0, x0 must Err; got {:?}",
        encode_fence(&ops_reg)
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_fence_diff_llvm_mc(((pred, _), (succ, _)) in iorw_pair()) {
        let asm = format!("fence {}, {}", pred, succ);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[fence_arg(pred), fence_arg(succ)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_fence_empty_is_iorw(_unit in 0u8..=0u8) {
        let empty = sut_word(&[]).expect("empty fence");
        let iorw = sut_word(&[fence_arg("iorw"), fence_arg("iorw")]).expect("iorw,iorw");
        prop_assert_eq!(empty, iorw);
        prop_assert_eq!(empty, 0x0ff0_000fu32);
    }

    #[test]
    fn encode_fence_i_type_fields(((pred, pbits), (succ, sbits)) in iorw_pair()) {
        let w = sut_word(&[fence_arg(pred), fence_arg(succ)])
            .unwrap_or_else(|e| panic!("SUT rejected fence {}, {}: {}", pred, succ, e));
        let (opc, rd, f3, rs1, got_succ, got_pred, fm) = unpack_fence(w);
        prop_assert_eq!(opc, OP_MISC_MEM, "opcode");
        prop_assert_eq!(rd, 0u32, "rd");
        prop_assert_eq!(f3, 0u32, "funct3");
        prop_assert_eq!(rs1, 0u32, "rs1");
        prop_assert_eq!(fm, 0u32, "fm");
        prop_assert_eq!(got_pred, pbits, "pred bits for {}", pred);
        prop_assert_eq!(got_succ, sbits, "succ bits for {}", succ);
    }

    #[test]
    fn encode_fence_imm0_diff_llvm_mc(a in fence_token(), b in fence_token()) {
        let asm = format!("fence {}, {}", a, b);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&[op_fence_token(a), op_fence_token(b)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT {:08x} != llvm-mc {:08x} for {}", sut, mc, asm);
    }
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_fence_neg_extra(((pred, _), (succ, _)) in iorw_pair(), extra in extra_operand()) {
        let asm = format!("fence {}, {}", pred, succ);
        let ops = vec![fence_arg(pred), fence_arg(succ), extra];
        prop_assert!(
            encode_fence(&ops).is_err(),
            "extra operand must Err for {} (llvm-mc rejects extra operands); got {:?}",
            asm,
            encode_fence(&ops)
        );
    }

    #[test]
    fn encode_fence_neg_arity(op in one_operand()) {
        let ops = [op];
        prop_assert!(
            encode_fence(&ops).is_err(),
            "single operand must Err (llvm-mc too few operands); got {:?}",
            encode_fence(&ops)
        );
    }

    #[test]
    fn encode_fence_neg_out_of_order(pred in invalid_letters(), ((succ, _),) in (prop::sample::select(IORW.to_vec()),)) {
        let ops = [fence_arg(pred), fence_arg(succ)];
        prop_assert!(
            encode_fence(&ops).is_err(),
            "out-of-order/duplicate/uppercase {} must Err (llvm-mc in-order iorw); got {:?}",
            pred,
            encode_fence(&ops)
        );
    }

    #[test]
    fn encode_fence_neg_invalid_operand(bad in invalid_operand()) {
        let left = [bad.clone(), fence_arg("rw")];
        prop_assert!(
            encode_fence(&left).is_err(),
            "invalid pred {:?} must Err (llvm-mc fence operand rule); got {:?}",
            left[0],
            encode_fence(&left)
        );
        let right = [fence_arg("rw"), bad];
        prop_assert!(
            encode_fence(&right).is_err(),
            "invalid succ {:?} must Err (llvm-mc fence operand rule); got {:?}",
            right[1],
            encode_fence(&right)
        );
    }
}
