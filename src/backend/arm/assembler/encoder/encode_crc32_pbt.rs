// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:241 CRC32 table lists crc32b/h/w/x and crc32cb/ch/cw/cx;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:1047-1048 "crc32b"|...|"crc32cx" => encode_crc32(mnemonic, operands);
//   bitfield.rs:243 "CRC32: sf 0 0 11010110 Rm 010 C sz Rn Rd";
//   ARM ARM CRC32/CRC32C: sf 00 11010110 Rm 010 C sz Rn Rd;
//     C=1 for CRC32C polynomial; sz 00/01/10/11 = B/H/W/X; sf=1 only for X;
//     Rd/Rn always Wd/Wn (WZR not SP); Rm is Wm for B/H/W and Xm for X.
// Stronger considered:
//   - State machine: rejected — encode_crc32 is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree CRC32 decoder
//   - encode_clz / encode_cls as differential siblings: rejected — same-job gate
//     (Data-processing 2-source, not CRC)
// Weaker available: algebraic.metamorphic (C-bit / sz / Rd/Rn/Rm isolation),
//   algebraic.invariant (ARM field layout), negative_error (arity / extra / SP / FP / wrong width)
// Differential: candidate=encode_crc32, reference=llvm-mc -triple=aarch64 -mattr=+crc -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Wd), Reg(Wn), Reg(Wm|Xm)] <-> `crc32{b,h,w,x,cb,ch,cw,cx} Wd, Wn, Wm|Xm`

use super::encode_crc32;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

const MNEMS: [&str; 8] = [
    "crc32b", "crc32h", "crc32w", "crc32x", "crc32cb", "crc32ch", "crc32cw", "crc32cx",
];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn is_x_form(m: &str) -> bool {
    m.ends_with('x')
}

fn c_bit(m: &str) -> u32 {
    if m.contains("crc32c") {
        1
    } else {
        0
    }
}

fn sz_of(m: &str) -> u32 {
    match m.as_bytes().last() {
        Some(b'b') => 0b00,
        Some(b'h') => 0b01,
        Some(b'w') => 0b10,
        Some(b'x') => 0b11,
        _ => 0,
    }
}

fn sf_of(m: &str) -> u32 {
    if is_x_form(m) {
        1
    } else {
        0
    }
}

fn w_reg(n: u32) -> String {
    if n == 31 {
        "wzr".into()
    } else {
        format!("w{n}")
    }
}

fn x_reg(n: u32) -> String {
    if n == 31 {
        "xzr".into()
    } else {
        format!("x{n}")
    }
}

fn rm_reg(m: &str, n: u32) -> String {
    if is_x_form(m) {
        x_reg(n)
    } else {
        w_reg(n)
    }
}

fn ops3(m: &str, rd: u32, rn: u32, rm: u32) -> [Operand; 3] {
    [
        Operand::Reg(w_reg(rd)),
        Operand::Reg(w_reg(rn)),
        Operand::Reg(rm_reg(m, rm)),
    ]
}

fn sut_word(m: &str, ops: &[Operand]) -> Result<u32, String> {
    match encode_crc32(m, ops)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {:?}", other)),
    }
}

/// ARM ARM CRC32 word, independent of the SUT shift-or expression.
fn arm_crc32_word(sf: u32, c: u32, sz: u32, rd: u32, rn: u32, rm: u32) -> u32 {
    (sf << 31)
        | (0b0011010110 << 21)
        | (rm << 16)
        | (0b010 << 13)
        | (c << 12)
        | (sz << 10)
        | (rn << 5)
        | rd
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
        .args(["-triple=aarch64", "-mattr=+crc", "-show-encoding"])
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

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        Just(Operand::Imm(0)),
        Just(Operand::Imm(-1)),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "8b".into(),
        }),
    ]
}

fn gpr(is_64: bool, n: u32) -> String {
    if is_64 {
        x_reg(n)
    } else {
        w_reg(n)
    }
}

fn mnem_strategy() -> impl Strategy<Value = &'static str> {
    prop::sample::select(MNEMS.to_vec())
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_crc32_kat_llvm_mc_crc32b_w0_w1_w2() {
    let want = 0x1ac24020u32;
    let mc = llvm_mc_word("crc32b w0, w1, w2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32b", &ops3("crc32b", 0, 1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_crc32_kat_llvm_mc_crc32h_w0_w1_w2() {
    let want = 0x1ac24420u32;
    let mc = llvm_mc_word("crc32h w0, w1, w2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32h", &ops3("crc32h", 0, 1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_crc32_kat_llvm_mc_crc32w_w0_w1_w2() {
    let want = 0x1ac24820u32;
    let mc = llvm_mc_word("crc32w w0, w1, w2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32w", &ops3("crc32w", 0, 1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_crc32_kat_llvm_mc_crc32x_w0_w1_x2() {
    let want = 0x9ac24c20u32;
    let mc = llvm_mc_word("crc32x w0, w1, x2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32x", &ops3("crc32x", 0, 1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_crc32_kat_llvm_mc_crc32cb_w0_w1_w2() {
    let want = 0x1ac25020u32;
    let mc = llvm_mc_word("crc32cb w0, w1, w2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32cb", &ops3("crc32cb", 0, 1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_crc32_kat_llvm_mc_crc32cx_w0_w1_x2() {
    let want = 0x9ac25c20u32;
    let mc = llvm_mc_word("crc32cx w0, w1, x2").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32cx", &ops3("crc32cx", 0, 1, 2)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_crc32_kat_llvm_mc_crc32b_wzr_wzr_wzr() {
    let want = 0x1adf43ffu32;
    let mc = llvm_mc_word("crc32b wzr, wzr, wzr").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32b", &ops3("crc32b", 31, 31, 31)).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_crc32_kat_llvm_mc_crc32x_wzr_w0_xzr() {
    let want = 0x9adf4c1fu32;
    let mc = llvm_mc_word("crc32x wzr, w0, xzr").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let sut = sut_word("crc32x", &ops3("crc32x", 31, 0, 31)).expect("SUT KAT");
    assert_eq!(sut, want);
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_crc32_diff_valid_gpr(
        m in mnem_strategy(),
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
    ) {
        let dest = w_reg(rd);
        let src_n = w_reg(rn);
        let src_m = rm_reg(m, rm);
        let asm = format!("{m} {dest}, {src_n}, {src_m}");
        let ops = ops3(m, rd, rn, rm);
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(m, &ops).expect("SUT");
        prop_assert_eq!(sut, mc, "CRC32 mismatch for {}", asm);
    }

    #[test]
    fn encode_crc32_arm_fields(
        m in mnem_strategy(),
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
    ) {
        let sf = sf_of(m);
        let c = c_bit(m);
        let sz = sz_of(m);
        let w = sut_word(m, &ops3(m, rd, rn, rm)).expect("SUT");
        let want = arm_crc32_word(sf, c, sz, rd, rn, rm);
        prop_assert_eq!(w, want, "ARM ARM CRC32 field layout for {}", m);
        prop_assert_eq!(w >> 31, sf, "sf");
        prop_assert_eq!((w >> 21) & 0x3ff, 0b0011010110, "bits[30:21]");
        prop_assert_eq!((w >> 16) & 0x1f, rm, "Rm");
        prop_assert_eq!((w >> 13) & 0b111, 0b010, "bits[15:13]");
        prop_assert_eq!((w >> 12) & 1, c, "C");
        prop_assert_eq!((w >> 10) & 0b11, sz, "sz");
        prop_assert_eq!((w >> 5) & 0x1f, rn, "Rn");
        prop_assert_eq!(w & 0x1f, rd, "Rd");
    }

    #[test]
    fn encode_crc32_meta_c_sz(
        sz in prop::sample::select(vec!["b", "h", "w", "x"]),
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
    ) {
        let plain = format!("crc32{sz}");
        let crc = format!("crc32c{sz}");
        let ops_p = ops3(&plain, rd, rn, rm);
        let ops_c = ops3(&crc, rd, rn, rm);
        let w_p = sut_word(&plain, &ops_p).expect("plain");
        let w_c = sut_word(&crc, &ops_c).expect("crc32c");
        prop_assert_eq!(w_p ^ w_c, 1 << 12, "CRC32C XOR CRC32 must be C-bit 12");

        if sz != "x" {
            let b = sut_word("crc32b", &ops3("crc32b", rd, rn, rm)).expect("b");
            let h = sut_word("crc32h", &ops3("crc32h", rd, rn, rm)).expect("h");
            let w = sut_word("crc32w", &ops3("crc32w", rd, rn, rm)).expect("w");
            prop_assert_eq!(b ^ h, 1 << 10, "B vs H is sz bit 10");
            prop_assert_eq!(h ^ w, 0b11 << 10, "H vs W is sz bits[11:10]");
        } else {
            let w32 = sut_word("crc32w", &ops3("crc32w", rd, rn, rm)).expect("w");
            let w64 = sut_word("crc32x", &ops3("crc32x", rd, rn, rm)).expect("x");
            prop_assert_eq!(w32 ^ w64, (1 << 31) | (1 << 10), "W vs X is sf plus sz lsb");
        }
    }

    #[test]
    fn encode_crc32_meta_rd_rn_rm(
        m in mnem_strategy(),
        rd in 0u32..=30,
        rn in 0u32..=30,
        rm in 0u32..=30,
    ) {
        let base = sut_word(m, &ops3(m, rd, rn, rm)).expect("base");
        let w_rd = sut_word(m, &ops3(m, rd + 1, rn, rm)).expect("rd+1");
        let w_rn = sut_word(m, &ops3(m, rd, rn + 1, rm)).expect("rn+1");
        let w_rm = sut_word(m, &ops3(m, rd, rn, rm + 1)).expect("rm+1");
        prop_assert_eq!(w_rd & 0x1f, rd + 1, "Rd+1 updates Rd field");
        prop_assert_eq!(w_rd & !0x1fu32, base & !0x1fu32, "Rd+1 leaves other fields unchanged");
        prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 updates Rn field");
        prop_assert_eq!(w_rn & !(0x1fu32 << 5), base & !(0x1fu32 << 5), "Rn+1 leaves other fields unchanged");
        prop_assert_eq!((w_rm >> 16) & 0x1f, rm + 1, "Rm+1 updates Rm field");
        prop_assert_eq!(w_rm & !(0x1fu32 << 16), base & !(0x1fu32 << 16), "Rm+1 leaves other fields unchanged");
    }

    #[test]
    fn encode_crc32_neg_arity(
        m in mnem_strategy(),
        n in 0usize..=2,
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
    ) {
        let mut ops = ops3(m, rd, rn, rm).to_vec();
        ops.truncate(n);
        prop_assert!(
            encode_crc32(m, &ops).is_err(),
            "CRC32 with {} operands must Err (llvm-mc: too few operands)",
            n
        );
    }

    #[test]
    fn encode_crc32_neg_extra_operand(
        m in mnem_strategy(),
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        extra in extra_operand(),
    ) {
        let mut ops = ops3(m, rd, rn, rm).to_vec();
        ops.push(extra);
        prop_assert!(
            encode_crc32(m, &ops).is_err(),
            "CRC32 has no 4th operand; extra operand must Err (llvm-mc rejects it)"
        );
    }

    #[test]
    fn encode_crc32_neg_sp(
        m in mnem_strategy(),
        slot in 0u32..=2,
        sp64 in any::<bool>(),
        other in 0u32..=30,
    ) {
        let sp = if sp64 { "sp" } else { "wsp" };
        let mut ops = ops3(m, other, other, other);
        ops[slot as usize] = Operand::Reg(sp.to_string());
        prop_assert!(
            encode_crc32(m, &ops).is_err(),
            "SP/WSP is not a valid CRC32 operand (slot={} sp={})",
            slot,
            sp
        );
    }

    #[test]
    fn encode_crc32_neg_wrong_width(
        m in mnem_strategy(),
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        rd64 in any::<bool>(),
        rn64 in any::<bool>(),
        rm64 in any::<bool>(),
    ) {
        let want_rm64 = is_x_form(m);
        prop_assume!(rd64 || rn64 || rm64 != want_rm64);
        let ops = [
            Operand::Reg(gpr(rd64, rd)),
            Operand::Reg(gpr(rn64, rn)),
            Operand::Reg(gpr(rm64, rm)),
        ];
        prop_assert!(
            encode_crc32(m, &ops).is_err(),
            "CRC32 {} wrong-width rd64={} rn64={} rm64={} must Err (llvm-mc rejects it)",
            m,
            rd64,
            rn64,
            rm64
        );
    }

    #[test]
    fn encode_crc32_neg_fp(
        m in mnem_strategy(),
        slot in 0u32..=2,
        prefix in prop::sample::select(vec!["d", "s", "q", "v", "h", "b"]),
        n in 0u32..=31,
    ) {
        let fp = format!("{prefix}{n}");
        let mut ops = ops3(m, 0, 1, 2);
        ops[slot as usize] = Operand::Reg(fp.clone());
        prop_assert!(
            encode_crc32(m, &ops).is_err(),
            "FP/SIMD register {} is not a valid CRC32 operand (slot={})",
            fp,
            slot
        );
    }

    #[test]
    fn encode_crc32_neg_invalid_name(
        m in mnem_strategy(),
        slot in 0u32..=2,
        name in invalid_name(),
    ) {
        let mut ops = ops3(m, 0, 1, 2);
        ops[slot as usize] = Operand::Reg(name.clone());
        prop_assert!(
            encode_crc32(m, &ops).is_err(),
            "invalid register name {:?} at slot {} must Err",
            name,
            slot
        );
    }

    #[test]
    fn encode_crc32_neg_nonreg(
        m in mnem_strategy(),
        slot in 0u32..=2,
        bad in non_reg_operand(),
    ) {
        let mut ops = ops3(m, 0, 1, 2).to_vec();
        ops[slot as usize] = bad;
        prop_assert!(
            encode_crc32(m, &ops).is_err(),
            "wrong operand kind at slot {} must Err",
            slot
        );
    }

    #[test]
    fn encode_crc32_diff_alt_spellings(
        m in mnem_strategy(),
        rd in 0u32..=31,
        rn in 0u32..=31,
        rm in 0u32..=31,
        dest_spell in 0u32..=3,
        src_n_spell in 0u32..=3,
        src_m_spell in 0u32..=4,
    ) {
        let dest = spell_w(rd, dest_spell);
        let src_n = spell_w(rn, src_n_spell);
        let src_m = if is_x_form(m) {
            spell_x(rm, src_m_spell)
        } else {
            spell_w(rm, src_m_spell.min(3))
        };
        let asm = format!("{m} {dest}, {src_n}, {src_m}");
        let ops = [
            Operand::Reg(dest),
            Operand::Reg(src_n),
            Operand::Reg(src_m),
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(m, &ops).expect("SUT");
        prop_assert_eq!(sut, mc, "CRC32 alt-spelling mismatch for {}", asm);
    }
}

fn spell_w(n: u32, kind: u32) -> String {
    match kind {
        0 if n == 31 => "w31".into(),
        1 if n == 31 => "WZR".into(),
        2 => w_reg(n).to_uppercase(),
        _ => w_reg(n),
    }
}

fn spell_x(n: u32, kind: u32) -> String {
    match kind {
        0 if n == 31 => "x31".into(),
        1 if n == 31 => "XZR".into(),
        2 if n == 30 => "lr".into(),
        3 => x_reg(n).to_uppercase(),
        _ => x_reg(n),
    }
}

fn invalid_name() -> impl Strategy<Value = String> {
    prop::sample::select(vec![
        "foo".into(),
        "x32".into(),
        "w32".into(),
        "x".into(),
        "r0".into(),
        "".into(),
        "x-1".into(),
        "x99".into(),
        "w".into(),
    ])
}

fn non_reg_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        any::<i64>().prop_map(Operand::Imm),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0,
        }),
        Just(Operand::Mem {
            base: "x0".into(),
            offset: 0,
        }),
        Just(Operand::Label("L0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "8b".into(),
        }),
    ]
}

#[test]
fn test_encode_crc32_regression_extra_operand() {
    let mut ops = ops3("crc32b", 0, 1, 2).to_vec();
    ops.push(Operand::Reg("w3".into()));
    assert!(
        encode_crc32("crc32b", &ops).is_err(),
        "crc32b w0, w1, w2, w3 must Err; extra operand is invalid (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_crc32_regression_sp() {
    let ops = [
        Operand::Reg("wsp".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
    ];
    assert!(
        encode_crc32("crc32b", &ops).is_err(),
        "crc32b wsp, w1, w2 must Err; register 31 is ZR not SP/WSP (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_crc32_regression_wrong_width() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
    ];
    assert!(
        encode_crc32("crc32b", &ops).is_err(),
        "crc32b x0, w1, w2 must Err; Rd must be W (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_crc32_regression_fp() {
    let ops = [
        Operand::Reg("d0".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
    ];
    assert!(
        encode_crc32("crc32b", &ops).is_err(),
        "crc32b d0, w1, w2 must Err; FP/SIMD registers are not CRC32 operands (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_crc32_regression_crc32x_w_rm() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Reg("w1".into()),
        Operand::Reg("w2".into()),
    ];
    assert!(
        encode_crc32("crc32x", &ops).is_err(),
        "crc32x w0, w1, w2 must Err; Rm must be X (llvm-mc rejects it)"
    );
}
