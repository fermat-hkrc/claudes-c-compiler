// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   README.md:233 lists tbx under NEON permute;
//   encoder/mod.rs:736 "tbx" => encode_neon_tbx;
//   neon.rs:802 Encode NEON TBX: table vector lookup with insert (preserves out-of-range lanes);
//   ARM ARM Advanced SIMD table lookup TBX: 0 Q 00 1110 00 0 Rm 0 len op=1 00 Rn Rd.
// Stronger considered:
//   - State machine: rejected — encode_neon_tbx is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree TBX decoder
//   - Differential vs encode_neon_tbl: rejected — same-job gate (TBL / op=0)
// Weaker available: algebraic.metamorphic (Q bit, len field), algebraic.invariant (word layout),
//   negative_error (arity / Ta / table contract / extra / mismatched T / non-reg)
// Differential: candidate=encode_neon_tbx, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,Ta), RegList({Vn.16B..}), RegArrangement(Vm,Ta)]
//     <-> `tbx Vd.Ta, {Vn.16B, ...}, Vm.Ta`

use super::encode_neon_tbx;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn vreg(n: u32) -> String {
    format!("v{}", n)
}

fn neon_arr(reg: u32, arr: &str) -> Operand {
    Operand::RegArrangement {
        reg: vreg(reg),
        arrangement: arr.to_string(),
    }
}

fn table_list(start: u32, n: u32, arr: &str) -> Operand {
    let regs: Vec<Operand> = (0..n)
        .map(|i| neon_arr((start + i) % 32, arr))
        .collect();
    Operand::RegList(regs)
}

fn tbx_asm(rd: u32, ta: &str, rn: u32, n: u32, rm: u32) -> String {
    let list: Vec<String> = (0..n)
        .map(|i| format!("v{}.16b", (rn + i) % 32))
        .collect();
    format!(
        "tbx v{}.{}, {{{}}}, v{}.{}",
        rd,
        ta,
        list.join(", "),
        rm,
        ta
    )
}

fn valid_ops(rd: u32, ta: &str, rn: u32, n: u32, rm: u32) -> Vec<Operand> {
    vec![
        neon_arr(rd, ta),
        table_list(rn, n, "16b"),
        neon_arr(rm, ta),
    ]
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_tbx(ops)? {
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
        .args(["-triple=aarch64", "-show-encoding"])
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

fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn nregs() -> impl Strategy<Value = u32> {
    prop_oneof![Just(1u32), Just(4u32), 1u32..=4]
}

fn ta_arr() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b"])
}

fn q_bit(t: &str) -> u32 {
    if t == "16b" {
        1
    } else {
        0
    }
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_tbx_kat_llvm_mc() {
    let want = 0x0e021020u32;
    let mc = llvm_mc_word("tbx v0.8b, {v1.16b}, v2.8b").expect("llvm-mc KAT tbx 8b 1-reg");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for tbx v0.8b, {{v1.16b}}, v2.8b");
    let ops = valid_ops(0, "8b", 1, 1, 2);
    assert_eq!(sut_word(&ops).expect("SUT KAT tbx 8b 1-reg"), want);

    let want_16b = 0x4e021020u32;
    let mc_16b = llvm_mc_word("tbx v0.16b, {v1.16b}, v2.16b").expect("llvm-mc KAT tbx 16b 1-reg");
    assert_eq!(mc_16b, want_16b, "llvm-mc KAT mapping broken for tbx 16b 1-reg");
    let ops_16b = valid_ops(0, "16b", 1, 1, 2);
    assert_eq!(sut_word(&ops_16b).expect("SUT KAT tbx 16b 1-reg"), want_16b);

    let want_2 = 0x0e033020u32;
    let mc_2 = llvm_mc_word("tbx v0.8b, {v1.16b, v2.16b}, v3.8b").expect("llvm-mc KAT tbx 2-reg");
    assert_eq!(mc_2, want_2, "llvm-mc KAT mapping broken for tbx 2-reg");
    let ops_2 = valid_ops(0, "8b", 1, 2, 3);
    assert_eq!(sut_word(&ops_2).expect("SUT KAT tbx 2-reg"), want_2);

    let want_3 = 0x4e045020u32;
    let mc_3 = llvm_mc_word("tbx v0.16b, {v1.16b, v2.16b, v3.16b}, v4.16b")
        .expect("llvm-mc KAT tbx 3-reg");
    assert_eq!(mc_3, want_3, "llvm-mc KAT mapping broken for tbx 3-reg");
    let ops_3 = valid_ops(0, "16b", 1, 3, 4);
    assert_eq!(sut_word(&ops_3).expect("SUT KAT tbx 3-reg"), want_3);

    let want_4 = 0x0e057020u32;
    let mc_4 = llvm_mc_word("tbx v0.8b, {v1.16b, v2.16b, v3.16b, v4.16b}, v5.8b")
        .expect("llvm-mc KAT tbx 4-reg");
    assert_eq!(mc_4, want_4, "llvm-mc KAT mapping broken for tbx 4-reg");
    let ops_4 = valid_ops(0, "8b", 1, 4, 5);
    assert_eq!(sut_word(&ops_4).expect("SUT KAT tbx 4-reg"), want_4);

    let want_wrap = 0x0e0233e0u32;
    let mc_wrap = llvm_mc_word("tbx v0.8b, {v31.16b, v0.16b}, v2.8b")
        .expect("llvm-mc KAT tbx wrap v31");
    assert_eq!(mc_wrap, want_wrap, "llvm-mc KAT mapping broken for wrap v31");
    let ops_wrap = valid_ops(0, "8b", 31, 2, 2);
    assert_eq!(sut_word(&ops_wrap).expect("SUT KAT tbx wrap v31"), want_wrap);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_tbx_diff_llvm_mc(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        ta in ta_arr(),
        n in nregs(),
    ) {
        let asm = tbx_asm(rd, ta, rn, n, rm);
        let ops = valid_ops(rd, ta, rn, n, rm);
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_tbx_metamorphic_q(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        n in nregs(),
    ) {
        let ops_lo = valid_ops(rd, "8b", rn, n, rm);
        let ops_hi = valid_ops(rd, "16b", rn, n, rm);
        let lo = sut_word(&ops_lo)
            .unwrap_or_else(|e| panic!("SUT Q=0 rejected: {}", e));
        let hi = sut_word(&ops_hi)
            .unwrap_or_else(|e| panic!("SUT Q=1 rejected: {}", e));
        prop_assert_eq!(lo ^ hi, 1u32 << 30, "Q from Ta must toggle only bit 30");
    }

    #[test]
    fn encode_neon_tbx_invariant_arm_fields(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        ta in ta_arr(),
        n in nregs(),
    ) {
        let ops = valid_ops(rd, ta, rn, n, rm);
        let w = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid layout: {}", e));
        prop_assert_eq!((w >> 31) & 1, 0u32, "bit 31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q_bit(ta), "Q bit");
        prop_assert_eq!((w >> 24) & 0b111111, 0b001110u32, "bits[29:24]=001110");
        prop_assert_eq!((w >> 21) & 0b111, 0u32, "bits[23:21]=000");
        prop_assert_eq!((w >> 16) & 0b11111, rm, "Rm");
        prop_assert_eq!((w >> 15) & 1, 0u32, "bit 15 must be 0");
        prop_assert_eq!((w >> 13) & 0b11, n - 1, "len = nregs-1");
        prop_assert_eq!((w >> 12) & 1, 1u32, "op=1 for TBX");
        prop_assert_eq!((w >> 10) & 0b11, 0u32, "bits[11:10]=00");
        prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
        prop_assert_eq!(w & 0b11111, rd, "Rd");
    }

    #[test]
    fn encode_neon_tbx_metamorphic_len(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        ta in ta_arr(),
        n1 in nregs(),
        n2 in nregs(),
    ) {
        let w1 = sut_word(&valid_ops(rd, ta, rn, n1, rm))
            .unwrap_or_else(|e| panic!("SUT n={} rejected: {}", n1, e));
        let w2 = sut_word(&valid_ops(rd, ta, rn, n2, rm))
            .unwrap_or_else(|e| panic!("SUT n={} rejected: {}", n2, e));
        prop_assert_eq!(
            (w1 ^ w2) & !(0b11u32 << 13),
            0u32,
            "changing only table length must differ only in len bits [14:13]"
        );
        prop_assert_eq!((w1 >> 13) & 0b11, n1 - 1, "len field for n1");
        prop_assert_eq!((w2 >> 13) & 0b11, n2 - 1, "len field for n2");
    }

    #[test]
    fn encode_neon_tbx_neg_extra_operand(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        ta in ta_arr(),
        n in nregs(),
    ) {
        let asm = format!("{}, v{}.{}", tbx_asm(rd, ta, rn, n, rm), extra, ta);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted 4-operand {}",
            asm
        );
        let mut ops = valid_ops(rd, ta, rn, n, rm);
        ops.push(neon_arr(extra, ta));
        prop_assert!(
            encode_neon_tbx(&ops).is_err(),
            "4 operands must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_tbx_neg_invalid_ta(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        ta in prop::sample::select(vec!["4h", "8h", "2s", "4s", "2d", "1d"]),
        n in nregs(),
    ) {
        let asm = tbx_asm(rd, ta, rn, n, rm);
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted invalid Ta {}",
            asm
        );
        let ops = valid_ops(rd, ta, rn, n, rm);
        prop_assert!(
            encode_neon_tbx(&ops).is_err(),
            "Ta={} is not 8b/16b and must Err (llvm-mc rejects {})",
            ta,
            asm
        );
    }

    #[test]
    fn encode_neon_tbx_neg_table_contract(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        n_hi in prop_oneof![Just(5u32), Just(8u32), 5u32..=8],
        gap in prop_oneof![Just(2u32), Just(3u32), 2u32..=16],
        bad_arr in prop::sample::select(vec!["8b", "4h", "8h", "2s", "4s", "2d"]),
        kind in 0u8..=3,
    ) {
        let ops = match kind {
            0 => {
                vec![
                    neon_arr(rd, "8b"),
                    Operand::RegList(vec![]),
                    neon_arr(rm, "8b"),
                ]
            }
            1 => {
                valid_ops(rd, "8b", rn, n_hi, rm)
            }
            2 => {
                let second = (rn + gap) % 32;
                prop_assume!(second != (rn + 1) % 32);
                vec![
                    neon_arr(rd, "8b"),
                    Operand::RegList(vec![neon_arr(rn, "16b"), neon_arr(second, "16b")]),
                    neon_arr(rm, "8b"),
                ]
            }
            _ => {
                vec![
                    neon_arr(rd, "8b"),
                    table_list(rn, 1, bad_arr),
                    neon_arr(rm, "8b"),
                ]
            }
        };
        let caught =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| encode_neon_tbx(&ops)));
        match caught {
            Ok(Err(_)) => {}
            Ok(Ok(w)) => {
                return Err(proptest::test_runner::TestCaseError::fail(format!(
                    "invalid table (kind={kind}) must Err, got Ok({w:?})"
                )));
            }
            Err(_) => {
                return Err(proptest::test_runner::TestCaseError::fail(format!(
                    "invalid table (kind={kind}) must Err, not panic"
                )));
            }
        }
    }

    #[test]
    fn encode_neon_tbx_neg_arity_kinds(
        n in 0usize..=2,
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b"]),
        dest_n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        bad in prop_oneof![
            Just("v32".to_string()),
            Just("v99".to_string()),
            Just("foo".to_string()),
            Just("".to_string()),
            Just("v".to_string()),
            Just("v-1".to_string()),
        ],
        ta in ta_arr(),
    ) {
        let all = valid_ops(rd, "8b", rn, 1, rm);
        let ops: Vec<Operand> = all.iter().take(n).cloned().collect();
        prop_assert!(
            encode_neon_tbx(&ops).is_err(),
            "len={} must Err (requires 3 operands)",
            n
        );

        let dest = format!("{}{}", prefix, dest_n);
        let asm_gpr = format!("tbx {}, {{v{}.16b}}, v{}.8b", dest, rn, rm);
        prop_assert!(
            llvm_mc_word(&asm_gpr).is_err(),
            "llvm-mc unexpectedly accepted GPR/FP dest {}",
            asm_gpr
        );
        let ops_gpr = vec![
            Operand::Reg(dest.clone()),
            table_list(rn, 1, "16b"),
            neon_arr(rm, "8b"),
        ];
        prop_assert!(
            encode_neon_tbx(&ops_gpr).is_err(),
            "GPR/FP dest {} is not a NEON Vd.Ta and must Err (llvm-mc rejects {})",
            dest,
            asm_gpr
        );

        let ops_bad = vec![
            Operand::RegArrangement {
                reg: bad.clone(),
                arrangement: "8b".to_string(),
            },
            table_list(rn, 1, "16b"),
            neon_arr(rm, "8b"),
        ];
        prop_assert!(
            encode_neon_tbx(&ops_bad).is_err(),
            "invalid dest name {} must Err",
            bad
        );

        let ops_nolist = vec![neon_arr(rd, "8b"), neon_arr(rn, "16b"), neon_arr(rm, "8b")];
        let asm_nolist = format!("tbx v{}.8b, v{}.16b, v{}.8b", rd, rn, rm);
        prop_assert!(
            llvm_mc_word(&asm_nolist).is_err(),
            "llvm-mc unexpectedly accepted missing list {}",
            asm_nolist
        );
        prop_assert!(
            encode_neon_tbx(&ops_nolist).is_err(),
            "second operand must be a register list (llvm-mc rejects {})",
            asm_nolist
        );

        prop_assume!(ta != "8b");
        let asm_mis = format!(
            "tbx v{}.8b, {{v{}.16b}}, v{}.{}",
            rd, rn, rm, ta
        );
        prop_assert!(
            llvm_mc_word(&asm_mis).is_err(),
            "llvm-mc unexpectedly accepted mismatched T {}",
            asm_mis
        );
        let ops_mis = vec![
            neon_arr(rd, "8b"),
            table_list(rn, 1, "16b"),
            neon_arr(rm, ta),
        ];
        prop_assert!(
            encode_neon_tbx(&ops_mis).is_err(),
            "mismatched Vd.8b vs Vm.{} must Err (llvm-mc rejects {})",
            ta,
            asm_mis
        );
    }

    /// Coverage sweep: list[0] must be Vn.16B (not bare Reg / Imm / bad name);
    /// Vm must be Vm.Ta (not bare GPR/FP / bad name). Uppercase V/X spellings
    /// match llvm-mc (README gas-compatible assembly).
    #[test]
    fn encode_neon_tbx_neg_list_and_vm_kinds(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        prefix in prop::sample::select(vec!["x", "w", "d", "s", "q", "h", "b", "v"]),
        n in prop_oneof![Just(0u32), Just(31u32), 0u32..=31],
        bad in prop_oneof![
            Just("v32".to_string()),
            Just("foo".to_string()),
            Just("".to_string()),
            Just("v".to_string()),
        ],
        kind in 0u8..=4,
    ) {
        let bare = format!("{}{}", prefix, n);
        let ops = match kind {
            0 => vec![
                neon_arr(rd, "8b"),
                Operand::RegList(vec![Operand::Reg(vreg(rn))]),
                neon_arr(rm, "8b"),
            ],
            1 => vec![
                neon_arr(rd, "8b"),
                Operand::RegList(vec![Operand::Imm(n as i64)]),
                neon_arr(rm, "8b"),
            ],
            2 => vec![
                neon_arr(rd, "8b"),
                Operand::RegList(vec![Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "16b".to_string(),
                }]),
                neon_arr(rm, "8b"),
            ],
            3 => vec![
                neon_arr(rd, "8b"),
                table_list(rn, 1, "16b"),
                Operand::Reg(bare.clone()),
            ],
            _ => vec![
                neon_arr(rd, "8b"),
                table_list(rn, 1, "16b"),
                Operand::RegArrangement {
                    reg: bad.clone(),
                    arrangement: "8b".to_string(),
                },
            ],
        };
        let caught =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| encode_neon_tbx(&ops)));
        match caught {
            Ok(Err(_)) => {}
            Ok(Ok(w)) => {
                return Err(proptest::test_runner::TestCaseError::fail(format!(
                    "list/Vm kind={kind} bare={bare} bad={bad} must Err, got Ok({w:?})"
                )));
            }
            Err(_) => {
                return Err(proptest::test_runner::TestCaseError::fail(format!(
                    "list/Vm kind={kind} must Err, not panic"
                )));
            }
        }
    }

    #[test]
    fn encode_neon_tbx_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        n in nregs(),
    ) {
        let list: Vec<String> = (0..n)
            .map(|i| format!("V{}.16B", (rn + i) % 32))
            .collect();
        let asm = format!(
            "tbx V{}.8B, {{{}}}, V{}.8B",
            rd,
            list.join(", "),
            rm
        );
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected uppercase {}: {}", asm, e));
        let ops = valid_ops(rd, "8b", rn, n, rm);
        let sut = sut_word(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "uppercase spelling mismatch for {}", asm);
    }
}

/// Deterministic regression: exactly 3 operands (shrunk from neg_extra_operand).
#[test]
fn test_encode_neon_tbx_regression_extra_operand() {
    let mut ops = valid_ops(0, "8b", 0, 1, 0);
    ops.push(neon_arr(0, "8b"));
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.8b, {{v0.16b}}, v0.8b, v0.8b must Err (exactly 3 operands)"
    );
}

/// Deterministic regression: Ta not in {8b,16b} (shrunk from neg_invalid_ta).
#[test]
fn test_encode_neon_tbx_regression_invalid_ta() {
    let ops = valid_ops(0, "4h", 0, 1, 0);
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.4h, {{v0.16b}}, v0.4h must Err (Ta not 8b/16b)"
    );
}

/// Deterministic regression: empty register list must Err, not panic (shrunk from neg_table_contract).
#[test]
fn test_encode_neon_tbx_regression_empty_list() {
    let ops = [
        neon_arr(0, "8b"),
        Operand::RegList(vec![]),
        neon_arr(0, "8b"),
    ];
    let caught =
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| encode_neon_tbx(&ops)));
    match caught {
        Ok(Err(_)) => {}
        Ok(Ok(w)) => panic!("empty table list must Err, got Ok({w:?})"),
        Err(_) => panic!("empty table list must Err, not panic on regs[0]"),
    }
}

/// Deterministic regression: more than 4 table registers must Err (kind=1 of neg_table_contract).
#[test]
fn test_encode_neon_tbx_regression_five_regs() {
    let ops = valid_ops(0, "8b", 0, 5, 0);
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.8b, {{v0.16b..v4.16b}}, v0.8b must Err (invalid number of vectors)"
    );
}

/// Deterministic regression: non-sequential table registers must Err (kind=2 of neg_table_contract).
#[test]
fn test_encode_neon_tbx_regression_nonsequential() {
    let ops = [
        neon_arr(0, "8b"),
        Operand::RegList(vec![neon_arr(0, "16b"), neon_arr(2, "16b")]),
        neon_arr(0, "8b"),
    ];
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.8b, {{v0.16b, v2.16b}}, v0.8b must Err (registers must be sequential)"
    );
}

/// Deterministic regression: table arrangement must be .16b (kind=3 of neg_table_contract).
#[test]
fn test_encode_neon_tbx_regression_table_not_16b() {
    let ops = [
        neon_arr(0, "8b"),
        table_list(0, 1, "8b"),
        neon_arr(0, "8b"),
    ];
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.8b, {{v0.8b}}, v0.8b must Err (table must be .16b)"
    );
}

/// Deterministic regression: dest must be Vd.Ta, not a bare GPR (shrunk from neg_arity_kinds).
#[test]
fn test_encode_neon_tbx_regression_gpr_dest() {
    let ops = [
        Operand::Reg("x0".into()),
        table_list(0, 1, "16b"),
        neon_arr(0, "8b"),
    ];
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx x0, {{v0.16b}}, v0.8b must Err (GPR dest is not Vd.Ta)"
    );
}

/// Deterministic regression: table list member must be Vn.16B, not a bare Reg (shrunk from neg_list_and_vm_kinds kind=0).
#[test]
fn test_encode_neon_tbx_regression_bare_list_reg() {
    let ops = [
        neon_arr(0, "8b"),
        Operand::RegList(vec![Operand::Reg("v0".into())]),
        neon_arr(0, "8b"),
    ];
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.8b, {{v0}}, v0.8b must Err (table member is not Vn.16B)"
    );
}

/// Deterministic regression: Vm must be Vm.Ta, not a bare GPR (coverage sweep kind=3).
#[test]
fn test_encode_neon_tbx_regression_bare_vm() {
    let ops = [
        neon_arr(0, "8b"),
        table_list(0, 1, "16b"),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.8b, {{v0.16b}}, x0 must Err (GPR Vm is not Vm.Ta)"
    );
}

/// Deterministic regression: Vm.Ta must match Vd.Ta (from neg_arity_kinds mismatched T).
#[test]
fn test_encode_neon_tbx_regression_mismatched_t() {
    let ops = [
        neon_arr(0, "8b"),
        table_list(0, 1, "16b"),
        neon_arr(0, "16b"),
    ];
    assert!(
        encode_neon_tbx(&ops).is_err(),
        "tbx v0.8b, {{v0.16b}}, v0.16b must Err (Vd.Ta must equal Vm.Ta)"
    );
}
