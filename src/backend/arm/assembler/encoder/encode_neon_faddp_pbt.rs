// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:226 NEON float vector lists faddp;
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:758 "faddp" => encode_neon_faddp;
//   neon.rs:1681 FADDP — float add pairwise;
//   neon.rs:1682 Vector form: FADDP Vd.T, Vn.T, Vm.T;
//   neon.rs:1683 Format: 0 Q 1 01110 0 sz 1 Rm 110101 Rn Rd;
//   neon.rs:1684 Scalar form: FADDP Sd, Vn.2S  or FADDP Dd, Vn.2D;
//   neon.rs:1685 Format: 01 1 11110 0 sz 11000 01101 10 Rn Rd;
//   ARM ARM Advanced SIMD three-same FADDP; T in {2S,4S,2D}.
// Stronger considered:
//   - State machine: rejected — encode_neon_faddp is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree FADDP decoder
//   - Differential vs encode_neon_float_three_same / encode_neon_scalar_addp: rejected — same-job gate
//     (FADD is not pairwise; integer SISD ADDP is a different opcode)
// Weaker available: algebraic.metamorphic (Rd/Rn/Rm/Q/sz fields), algebraic.invariant (word layout),
//   negative_error (arity / extra / invalid T / mismatched T / GPR dest / scalar dest-size)
// Differential: candidate=encode_neon_faddp, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegArrangement(Vd,T), RegArrangement(Vn,T), RegArrangement(Vm,T)] <-> `faddp Vd.T, Vn.T, Vm.T`
//     and [Reg(Sd|Dd), RegArrangement(Vn,2s|2d)] <-> `faddp Sd, Vn.2S` / `faddp Dd, Vn.2D`

use super::encode_neon_faddp;
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

fn arr(reg: u32, t: &str) -> Operand {
    Operand::RegArrangement {
        reg: format!("v{}", reg),
        arrangement: t.to_string(),
    }
}

fn gpr(prefix: &str, n: u32) -> Operand {
    Operand::Reg(format!("{}{}", prefix, n))
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_neon_faddp(ops)? {
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

fn valid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["2s", "4s", "2d"])
}

fn invalid_t() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec![
        "8b", "16b", "4h", "8h", "1d", "1s", "4d", "2h", "1q", "4b",
    ])
}

/// ARM Advanced SIMD three-same FADDP:
/// 0 Q 1 01110 0 sz 1 Rm 110101 Rn Rd
fn arm_faddp_vec(rd: u32, rn: u32, rm: u32, t: &str) -> u32 {
    let (q, sz) = match t {
        "2s" => (0u32, 0u32),
        "4s" => (1, 0),
        "2d" => (1, 1),
        _ => (0, 0),
    };
    (q << 30)
        | (1 << 29)
        | (0b01110 << 24)
        | (sz << 22)
        | (1 << 21)
        | (rm << 16)
        | (0b110101 << 10)
        | (rn << 5)
        | rd
}

/// ARM SISD FADDP: 01 1 11110 0 sz 11000 01101 10 Rn Rd
fn arm_faddp_sisd(rd: u32, rn: u32, sz: u32) -> u32 {
    (0b01 << 30)
        | (1 << 29)
        | (0b11110 << 24)
        | (sz << 22)
        | (0b11000 << 17)
        | (0b01101 << 12)
        | (0b10 << 10)
        | (rn << 5)
        | rd
}

// Reference KAT gate (must pass before PBT).
#[test]
fn encode_neon_faddp_kat_llvm_mc_v0_2s_v1_v2() {
    let want = 0x2e22d420u32;
    let mc = llvm_mc_word("faddp v0.2s, v1.2s, v2.2s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "2s"), arr(1, "2s"), arr(2, "2s")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_faddp_kat_llvm_mc_v0_4s_v1_v2() {
    let want = 0x6e22d420u32;
    let mc = llvm_mc_word("faddp v0.4s, v1.4s, v2.4s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "4s"), arr(1, "4s"), arr(2, "4s")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_faddp_kat_llvm_mc_v0_2d_v1_v2() {
    let want = 0x6e62d420u32;
    let mc = llvm_mc_word("faddp v0.2d, v1.2d, v2.2d").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(0, "2d"), arr(1, "2d"), arr(2, "2d")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_faddp_kat_llvm_mc_s0_v1_2s() {
    let want = 0x7e30d820u32;
    let mc = llvm_mc_word("faddp s0, v1.2s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [gpr("s", 0), arr(1, "2s")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_faddp_kat_llvm_mc_d0_v1_2d() {
    let want = 0x7e70d820u32;
    let mc = llvm_mc_word("faddp d0, v1.2d").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [gpr("d", 0), arr(1, "2d")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_neon_faddp_kat_llvm_mc_v31_2s_v31_v31() {
    let want = 0x2e3fd7ffu32;
    let mc = llvm_mc_word("faddp v31.2s, v31.2s, v31.2s").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [arr(31, "2s"), arr(31, "2s"), arr(31, "2s")];
    let sut = sut_word(&ops).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_faddp_diff_llvm_mc_vector(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("faddp v{rd}.{t}, v{rn}.{t}, v{rm}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_faddp_diff_llvm_mc_scalar(
        rd in reg_num(),
        rn in reg_num(),
        is_d in proptest::bool::ANY,
    ) {
        let (dest, t) = if is_d { ("d", "2d") } else { ("s", "2s") };
        let asm = format!("faddp {dest}{rd}, v{rn}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[gpr(dest, rd), arr(rn, t)]).expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_neon_faddp_meta_rd_rn_rm(
        rd1 in reg_num(),
        rd2 in reg_num(),
        rn1 in reg_num(),
        rn2 in reg_num(),
        rm1 in reg_num(),
        rm2 in reg_num(),
        t in valid_t(),
    ) {
        let w111 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm1, t)]).expect("w111");
        let w211 = sut_word(&[arr(rd2, t), arr(rn1, t), arr(rm1, t)]).expect("w211");
        let w121 = sut_word(&[arr(rd1, t), arr(rn2, t), arr(rm1, t)]).expect("w121");
        let w112 = sut_word(&[arr(rd1, t), arr(rn1, t), arr(rm2, t)]).expect("w112");
        prop_assert_eq!(
            (w111 ^ w211) & !0x1Fu32,
            0,
            "changing only Rd must differ only in bits[4:0]"
        );
        prop_assert_eq!(w111 & 0x1F, rd1);
        prop_assert_eq!(w211 & 0x1F, rd2);
        prop_assert_eq!(
            (w111 ^ w121) & !(0x1Fu32 << 5),
            0,
            "changing only Rn must differ only in bits[9:5]"
        );
        prop_assert_eq!((w111 >> 5) & 0x1F, rn1);
        prop_assert_eq!((w121 >> 5) & 0x1F, rn2);
        prop_assert_eq!(
            (w111 ^ w112) & !(0x1Fu32 << 16),
            0,
            "changing only Rm must differ only in bits[20:16]"
        );
        prop_assert_eq!((w111 >> 16) & 0x1F, rm1);
        prop_assert_eq!((w112 >> 16) & 0x1F, rm2);
    }

    #[test]
    fn encode_neon_faddp_inv_layout(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
        is_d in proptest::bool::ANY,
    ) {
        let w = sut_word(&[arr(rd, t), arr(rn, t), arr(rm, t)]).expect("layout");
        prop_assert_eq!(w, arm_faddp_vec(rd, rn, rm, t), "ARM FADDP vector layout");
        prop_assert_eq!(w >> 31, 0);
        let (q, sz) = match t {
            "2s" => (0u32, 0u32),
            "4s" => (1, 0),
            "2d" => (1, 1),
            _ => (0, 0),
        };
        prop_assert_eq!((w >> 30) & 1, q);
        prop_assert_eq!((w >> 24) & 0x3F, 0b101110);
        prop_assert_eq!((w >> 23) & 1, 0);
        prop_assert_eq!((w >> 22) & 1, sz);
        prop_assert_eq!((w >> 21) & 1, 1);
        prop_assert_eq!((w >> 16) & 0x1F, rm);
        prop_assert_eq!((w >> 10) & 0x3F, 0b110101);
        prop_assert_eq!((w >> 5) & 0x1F, rn);
        prop_assert_eq!(w & 0x1F, rd);

        let (dest, st, ssz) = if is_d { ("d", "2d", 1u32) } else { ("s", "2s", 0u32) };
        let ws = sut_word(&[gpr(dest, rd), arr(rn, st)]).expect("scalar layout");
        prop_assert_eq!(ws, arm_faddp_sisd(rd, rn, ssz), "ARM FADDP scalar layout");
        prop_assert_eq!((ws >> 24) & 0xFF, 0b01111110);
        prop_assert_eq!((ws >> 22) & 1, ssz);
        prop_assert_eq!((ws >> 17) & 0x1F, 0b11000);
        prop_assert_eq!((ws >> 12) & 0x1F, 0b01101);
        prop_assert_eq!((ws >> 10) & 0b11, 0b10);
        prop_assert_eq!((ws >> 5) & 0x1F, rn);
        prop_assert_eq!(ws & 0x1F, rd);
    }

    #[test]
    fn encode_neon_faddp_neg_extra(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        extra in reg_num(),
        t in valid_t(),
        n in 0usize..=1,
    ) {
        let short: Vec<Operand> = match n {
            0 => vec![],
            _ => vec![arr(rd, t)],
        };
        prop_assert!(
            encode_neon_faddp(&short).is_err(),
            "arity {} must Err (faddp requires 2 or 3 operands)",
            n
        );
        let asm = format!("faddp v{rd}.{t}, v{rn}.{t}, v{rm}.{t}, v{extra}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t), arr(extra, t)];
        prop_assert!(
            encode_neon_faddp(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_faddp_neg_invalid_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in invalid_t(),
    ) {
        let asm = format!("faddp v{rd}.{t}, v{rn}.{t}, v{rm}.{t}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, t), arr(rn, t), arr(rm, t)];
        prop_assert!(
            encode_neon_faddp(&ops).is_err(),
            "invalid T must Err (only .2s/.4s/.2d; llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_faddp_neg_mismatch_t(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        td in valid_t(),
        tn in valid_t(),
        tm in valid_t(),
    ) {
        prop_assume!(!(td == tn && tn == tm));
        let asm = format!("faddp v{rd}.{td}, v{rn}.{tn}, v{rm}.{tm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [arr(rd, td), arr(rn, tn), arr(rm, tm)];
        prop_assert!(
            encode_neon_faddp(&ops).is_err(),
            "mismatched T must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_faddp_neg_gpr_bare_scalar_dest(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        kind in 0u8..=7,
    ) {
        let (ops, asm) = match kind {
            0 => {
                let asm = format!("faddp x{rd}, x{rn}, x{rm}");
                let ops = vec![gpr("x", rd), gpr("x", rn), gpr("x", rm)];
                (ops, asm)
            }
            1 => {
                let asm = format!("faddp w{rd}, v{rn}.2s, v{rm}.2s");
                let ops = vec![gpr("w", rd), arr(rn, "2s"), arr(rm, "2s")];
                (ops, asm)
            }
            2 => {
                let asm = "faddp sp, v0.2s, v1.2s".to_string();
                let ops = vec![Operand::Reg("sp".into()), arr(0, "2s"), arr(1, "2s")];
                (ops, asm)
            }
            3 => {
                let asm = format!("faddp v{rd}, v{rn}, v{rm}");
                let ops = vec![gpr("v", rd), gpr("v", rn), gpr("v", rm)];
                (ops, asm)
            }
            4 => {
                let asm = format!("faddp d{rd}, v{rn}.2s");
                let ops = vec![gpr("d", rd), arr(rn, "2s")];
                (ops, asm)
            }
            5 => {
                let asm = format!("faddp s{rd}, v{rn}.2d");
                let ops = vec![gpr("s", rd), arr(rn, "2d")];
                (ops, asm)
            }
            6 => {
                let asm = format!("faddp x{rd}, v{rn}.2s");
                let ops = vec![gpr("x", rd), arr(rn, "2s")];
                (ops, asm)
            }
            _ => {
                let asm = format!("faddp s{rd}, v{rn}.4s");
                let ops = vec![gpr("s", rd), arr(rn, "4s")];
                (ops, asm)
            }
        };
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_neon_faddp(&ops).is_err(),
            "non-arranged NEON / GPR / SP / dest-size mismatch must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_neon_faddp_diff_alt_spellings(
        rd in reg_num(),
        rn in reg_num(),
        rm in reg_num(),
        t in valid_t(),
    ) {
        let asm = format!("faddp V{rd}.{t}, V{rn}.{t}, V{rm}.{t}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let ops = [
            Operand::RegArrangement {
                reg: format!("V{rd}"),
                arrangement: t.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{rn}"),
                arrangement: t.to_string(),
            },
            Operand::RegArrangement {
                reg: format!("V{rm}"),
                arrangement: t.to_string(),
            },
        ];
        let sut = sut_word(&ops).expect(&asm);
        prop_assert_eq!(sut, mc, "uppercase V prefix must match llvm-mc {}", asm);
    }
}

#[test]
fn test_encode_neon_faddp_regression_extra_operand() {
    let ops = [arr(0, "2s"), arr(0, "2s"), arr(0, "2s"), arr(0, "2s")];
    assert!(
        encode_neon_faddp(&ops).is_err(),
        "faddp v0.2s, v0.2s, v0.2s, v0.2s must Err (llvm-mc/gas reject a fourth operand)"
    );
}

#[test]
fn test_encode_neon_faddp_regression_mismatch_t() {
    let ops = [arr(0, "2d"), arr(0, "2s"), arr(0, "2s")];
    assert!(
        encode_neon_faddp(&ops).is_err(),
        "faddp v0.2d, v0.2s, v0.2s must Err (llvm-mc/gas: operand mismatch)"
    );
}

#[test]
fn test_encode_neon_faddp_regression_scalar_dest_size() {
    let ops = [gpr("d", 0), arr(0, "2s")];
    assert!(
        encode_neon_faddp(&ops).is_err(),
        "faddp d0, v0.2s must Err (llvm-mc/gas require Sd+Vn.2S or Dd+Vn.2D)"
    );
}
