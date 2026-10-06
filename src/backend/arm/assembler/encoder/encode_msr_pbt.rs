// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists msr;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:974 "msr" => encode_msr(operands);
//   system.rs:268 "MSR (immediate): msr <pstatefield>, #imm";
//   system.rs:294 "MSR (register): msr sysreg, Xt";
//   system.rs:383 "MSR encoding: 0xd500_0000 has L=0 (bit 21) for write.";
//   system.rs:265 "msr needs system register name";
//   ARM ARM MSR (register): 1101 0101 00 0 op0 op1 CRn CRm op2 Rt;
//   ARM ARM MSR (immediate): 1101 0101 0000 0 op1 0100 CRm op2 11111;
//   Xt is a 64-bit GPR (x0–x30 / xzr); SP is not Xt.
// Stronger considered:
//   - State machine: rejected — encode_msr is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree MSR decoder
//   - Differential vs encode_mrs: rejected — same-job gate (MRS read / L=1 / reversed operands)
// Weaker available: algebraic.metamorphic (case-fold), algebraic.invariant (ARM layout),
//   negative_error (extra / wrong Xt / unknown / arity / oob generic / oob imm)
// Differential: candidate=encode_msr, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Symbol(sysreg), Reg(Xt)] <-> `msr sysreg, Xt`;
//   [Symbol(pstate), Imm(n)] <-> `msr pstate, #n`

use super::encode_msr;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM MSR (register) template: bits[31:21] = 1101_0101_000 (group + L=0).
const MSR_HI: u32 = 0b11010101000;

const NAMED: &[&str] = &[
    "sp_el0",
    "tpidr_el0",
    "tpidr_el1",
    "tpidr_el2",
    "tpidrro_el0",
    "tcr_el1",
    "ttbr0_el1",
    "sctlr_el1",
    "mdscr_el1",
    "cpacr_el1",
    "par_el1",
    "osdlr_el1",
    "oslar_el1",
    "oslsr_el1",
    "elr_el1",
    "spsr_el1",
    "esr_el1",
    "far_el1",
    "vbar_el1",
    "contextidr_el1",
    "mair_el1",
    "amair_el1",
    "hcr_el2",
    "cptr_el2",
    "hstr_el2",
    "elr_el2",
    "esr_el2",
    "far_el2",
    "spsr_el2",
    "sctlr_el2",
    "mdcr_el2",
    "tcr_el2",
    "ttbr0_el2",
    "vttbr_el2",
    "vtcr_el2",
    "vbar_el2",
    "mair_el2",
    "sp_el1",
    "csselr_el1",
    "actlr_el1",
    "cnthctl_el2",
    "cntvoff_el2",
    "sp_el2",
    "vpidr_el2",
    "vmpidr_el2",
    "hacr_el2",
    "actlr_el2",
    "afsr0_el2",
    "afsr1_el2",
    "amair_el2",
    "hpfar_el2",
    "pmintenset_el1",
    "pmintenclr_el1",
    "pmcr_el0",
    "pmcntenset_el0",
    "pmcntenclr_el0",
    "pmovsclr_el0",
    "pmselr_el0",
    "pmccntr_el0",
    "pmxevtyper_el0",
    "pmxevcntr_el0",
    "pmuserenr_el0",
    "pmccfiltr_el0",
    "cntv_ctl_el0",
    "cntp_ctl_el0",
    "cntp_cval_el0",
    "cntv_cval_el0",
    "ttbr1_el1",
    "cntkctl_el1",
    "daif",
    "fpcr",
    "fpsr",
    "nzcv",
    "spsel",
    "mdccint_el1",
    "fpexc32_el2",
    "spsr_abt",
    "spsr_und",
    "spsr_irq",
    "spsr_fiq",
    "ifsr32_el2",
    "dacr32_el2",
];

const PSTATE: &[&str] = &["daifset", "daifclr", "spsel"];

const DBG_FAMS: &[&str] = &["dbgbcr", "dbgbvr", "dbgwcr", "dbgwvr"];
const PMU_FAMS: &[&str] = &["pmevcntr", "pmevtyper"];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn is_named(s: &str) -> bool {
    let l = s.to_ascii_lowercase();
    NAMED.iter().any(|n| *n == l)
}

fn xt_name(rt: u32) -> String {
    if rt == 31 {
        "xzr".to_string()
    } else {
        format!("x{rt}")
    }
}

fn sform(op0: u32, op1: u32, crn: u32, crm: u32, op2: u32) -> String {
    format!("s{op0}_{op1}_c{crn}_c{crm}_{op2}")
}

/// ARM ARM MSR (register) word from unmasked sysreg fields (independent of SUT masking).
fn arm_msr_word(op0: u32, op1: u32, crn: u32, crm: u32, op2: u32, rt: u32) -> u32 {
    let enc = (op0 << 14) | (op1 << 11) | (crn << 7) | (crm << 3) | op2;
    0xd5000000 | (enc << 5) | (rt & 0x1f)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_msr(ops)? {
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

fn named_sysreg() -> impl Strategy<Value = &'static str> {
    prop::sample::select(NAMED.to_vec())
}

fn pstate_field() -> impl Strategy<Value = &'static str> {
    prop::sample::select(PSTATE.to_vec())
}

fn xt_gpr() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=30).prop_map(|n| format!("x{n}")),
        Just("xzr".to_string()),
        Just("lr".to_string()),
        Just("x31".to_string()),
    ]
}

fn case_fold(name: String) -> impl Strategy<Value = String> {
    prop_oneof![
        Just(name.clone()),
        Just(name.to_ascii_uppercase()),
        Just({
            let mut s = name.clone();
            if let Some(c) = s.get_mut(0..1) {
                c.make_ascii_uppercase();
            }
            s
        }),
        Just({
            name.chars()
                .enumerate()
                .map(|(i, c)| {
                    if i % 2 == 0 {
                        c.to_ascii_uppercase()
                    } else {
                        c.to_ascii_lowercase()
                    }
                })
                .collect::<String>()
        }),
    ]
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        (0u32..=31).prop_map(|n| Operand::Reg(format!("x{n}"))),
        (0u32..=31).prop_map(|n| Operand::Reg(format!("w{n}"))),
        Just(Operand::Reg("sp".into())),
        Just(Operand::Reg("xzr".into())),
        (-4i64..=32).prop_map(Operand::Imm),
        named_sysreg().prop_map(|n| Operand::Symbol(n.to_string())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Barrier("sy".into())),
        Just(Operand::Label(".L0".into())),
    ]
}

fn extra_asm(extra: &Operand) -> String {
    match extra {
        Operand::Reg(r) => r.clone(),
        Operand::Imm(n) => format!("#{n}"),
        Operand::Symbol(s) => s.clone(),
        Operand::Cond(c) => c.clone(),
        Operand::Barrier(b) => b.clone(),
        Operand::Label(l) => l.clone(),
        _ => "x1".to_string(),
    }
}

fn wrong_xt() -> impl Strategy<Value = String> {
    prop_oneof![
        (0u32..=30).prop_map(|n| format!("w{n}")),
        Just("wzr".to_string()),
        Just("w31".to_string()),
        Just("sp".to_string()),
        Just("wsp".to_string()),
        (0u32..=31).prop_map(|n| format!("d{n}")),
        (0u32..=31).prop_map(|n| format!("s{n}")),
        (0u32..=31).prop_map(|n| format!("q{n}")),
        (0u32..=31).prop_map(|n| format!("v{n}")),
        (0u32..=31).prop_map(|n| format!("h{n}")),
        (0u32..=31).prop_map(|n| format!("b{n}")),
    ]
}

fn unknown_name() -> impl Strategy<Value = String> {
    prop_oneof![
        Just("foo".to_string()),
        Just("bar".to_string()),
        Just("nzcvv".to_string()),
        Just("sys".to_string()),
        Just("x".to_string()),
        Just("".to_string()),
        Just("s_0_c1_c0_1".to_string()),
        Just("s3_0_c1_c0".to_string()),
        Just("dbgbcr16_el1".to_string()),
        Just("pmevcntr31_el0".to_string()),
        Just("currentel".to_string()),
        Just("midr_el1".to_string()),
        prop::string::string_regex("[a-z]{1,10}")
            .unwrap()
            .prop_filter("not a named sysreg", |s| !is_named(s)
                && s != "daifset"
                && s != "daifclr"),
    ]
}

fn oob_generic() -> impl Strategy<Value = String> {
    prop_oneof![
        (4u32..=9).prop_map(|op0| sform(op0, 0, 1, 0, 1)),
        (8u32..=16).prop_map(|op1| sform(3, op1, 1, 0, 1)),
        (16u32..=32).prop_map(|crn| sform(3, 0, crn, 0, 1)),
        (16u32..=32).prop_map(|crm| sform(3, 0, 1, crm, 1)),
        (8u32..=16).prop_map(|op2| sform(3, 0, 1, 0, op2)),
    ]
}

fn oob_numbered() -> impl Strategy<Value = String> {
    prop_oneof![
        (16u32..=32).prop_flat_map(|n| {
            prop::sample::select(DBG_FAMS.to_vec()).prop_map(move |f| format!("{f}{n}_el1"))
        }),
        (31u32..=40).prop_flat_map(|n| {
            prop::sample::select(PMU_FAMS.to_vec()).prop_map(move |f| format!("{f}{n}_el0"))
        }),
    ]
}

fn numbered_valid() -> impl Strategy<Value = String> {
    prop_oneof![
        (prop::sample::select(DBG_FAMS.to_vec()), 0u32..=15u32)
            .prop_map(|(f, n)| format!("{f}{n}_el1")),
        (prop::sample::select(PMU_FAMS.to_vec()), 0u32..=30u32)
            .prop_map(|(f, n)| format!("{f}{n}_el0")),
    ]
}

fn oob_imm() -> impl Strategy<Value = i64> {
    prop_oneof![(-32i64..=-1), (16i64..=32)]
}

fn agree_or_both_err(asm: &str, sut: Result<EncodeResult, String>) -> Result<(), String> {
    let mc = llvm_mc_word(asm);
    match (mc, sut) {
        (Ok(w), Ok(EncodeResult::Word(w2))) => {
            if w == w2 {
                Ok(())
            } else {
                Err(format!("SUT vs llvm-mc for {asm}: sut={w2:08x} mc={w:08x}"))
            }
        }
        (Err(_), Err(_)) => Ok(()),
        (Ok(w), Err(e)) => Err(format!("SUT rejected {asm}, llvm-mc encoded {w:08x}: {e}")),
        (Err(e), Ok(EncodeResult::Word(w))) => {
            Err(format!("SUT encoded {asm} as {w:08x}, llvm-mc rejected: {e}"))
        }
        (Ok(w), Ok(other)) => {
            Err(format!("SUT returned {other:?} for {asm}, llvm-mc encoded {w:08x}"))
        }
        (Err(e), Ok(other)) => Err(format!("SUT returned {other:?} for {asm}, llvm-mc rejected: {e}")),
    }
}

#[test]
fn encode_msr_kat_llvm_mc_tpidr_el0_x0() {
    let mc = llvm_mc_word("msr tpidr_el0, x0").expect("llvm-mc msr tpidr_el0, x0");
    assert_eq!(mc, 0xd51bd040, "llvm-mc KAT mapping broken for msr tpidr_el0, x0");
    let sut = sut_word(&[
        Operand::Symbol("tpidr_el0".into()),
        Operand::Reg("x0".into()),
    ])
    .expect("SUT KAT tpidr_el0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_msr_kat_llvm_mc_nzcv_x0() {
    let mc = llvm_mc_word("msr nzcv, x0").expect("llvm-mc msr nzcv, x0");
    assert_eq!(mc, 0xd51b4200, "llvm-mc KAT mapping broken for msr nzcv, x0");
    let sut = sut_word(&[Operand::Symbol("nzcv".into()), Operand::Reg("x0".into())])
        .expect("SUT KAT nzcv");
    assert_eq!(sut, mc);
}

#[test]
fn encode_msr_kat_llvm_mc_nzcv_xzr() {
    let mc = llvm_mc_word("msr nzcv, xzr").expect("llvm-mc msr nzcv, xzr");
    assert_eq!(mc, 0xd51b421f, "llvm-mc KAT mapping broken for msr nzcv, xzr");
    let sut = sut_word(&[
        Operand::Symbol("nzcv".into()),
        Operand::Reg("xzr".into()),
    ])
    .expect("SUT KAT nzcv xzr");
    assert_eq!(sut, mc);
}

#[test]
fn encode_msr_kat_llvm_mc_daifset_imm2() {
    let mc = llvm_mc_word("msr daifset, #2").expect("llvm-mc msr daifset, #2");
    assert_eq!(mc, 0xd50342df, "llvm-mc KAT mapping broken for msr daifset, #2");
    let sut = sut_word(&[Operand::Symbol("daifset".into()), Operand::Imm(2)])
        .expect("SUT KAT daifset");
    assert_eq!(sut, mc);
}

#[test]
fn encode_msr_kat_llvm_mc_spsel_imm1() {
    let mc = llvm_mc_word("msr spsel, #1").expect("llvm-mc msr spsel, #1");
    assert_eq!(mc, 0xd50041bf, "llvm-mc KAT mapping broken for msr spsel, #1");
    let sut = sut_word(&[Operand::Symbol("spsel".into()), Operand::Imm(1)])
        .expect("SUT KAT spsel imm");
    assert_eq!(sut, mc);
}

#[test]
fn encode_msr_kat_llvm_mc_spsel_x0() {
    let mc = llvm_mc_word("msr spsel, x0").expect("llvm-mc msr spsel, x0");
    assert_eq!(mc, 0xd5184200, "llvm-mc KAT mapping broken for msr spsel, x0");
    let sut = sut_word(&[Operand::Symbol("spsel".into()), Operand::Reg("x0".into())])
        .expect("SUT KAT spsel reg");
    assert_eq!(sut, mc);
}

#[test]
fn encode_msr_kat_llvm_mc_cntv_cval_el0_mapping() {
    // Mapping gate only — pins llvm-mc's CNTV_CVAL_EL0 word (ARM S3_3_C14_C3_2).
    let mc = llvm_mc_word("msr cntv_cval_el0, x0").expect("llvm-mc msr cntv_cval_el0, x0");
    assert_eq!(mc, 0xd51be340, "llvm-mc KAT mapping broken for msr cntv_cval_el0, x0");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_msr_diff_named(name in named_sysreg(), xt in xt_gpr()) {
        let asm = format!("msr {name}, {xt}");
        let sut = encode_msr(&[
            Operand::Symbol(name.to_string()),
            Operand::Reg(xt),
        ]);
        if let Err(e) = agree_or_both_err(&asm, sut) {
            prop_assert!(false, "{e}");
        }
    }

    #[test]
    fn encode_msr_diff_generic(
        op0 in 0u32..=3,
        op1 in 0u32..=7,
        crn in 0u32..=15,
        crm in 0u32..=15,
        op2 in 0u32..=7,
        rt in 0u32..=31,
    ) {
        let name = sform(op0, op1, crn, crm, op2);
        let xt = xt_name(rt);
        let asm = format!("msr {name}, {xt}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let expect = arm_msr_word(op0, op1, crn, crm, op2, rt);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&[
            Operand::Symbol(name),
            Operand::Reg(xt),
        ])
        .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_msr_diff_numbered(name in numbered_valid(), rt in 0u32..=31) {
        let xt = xt_name(rt);
        let asm = format!("msr {name}, {xt}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[
            Operand::Symbol(name.clone()),
            Operand::Reg(xt),
        ])
        .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_msr_diff_imm_pstate(field in pstate_field(), imm in 0i64..=15) {
        let asm = format!("msr {field}, #{imm}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[
            Operand::Symbol(field.to_string()),
            Operand::Imm(imm),
        ])
        .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_msr_inv_arm_layout(
        name in named_sysreg(),
        rt1 in 0u32..=31,
        rt2 in 0u32..=31,
    ) {
        let w1 = sut_word(&[
            Operand::Symbol(name.to_string()),
            Operand::Reg(xt_name(rt1)),
        ]);
        let w2 = sut_word(&[
            Operand::Symbol(name.to_string()),
            Operand::Reg(xt_name(rt2)),
        ]);
        let w1 = w1.expect("named sysreg must encode");
        let w2 = w2.expect("named sysreg must encode");
        prop_assert_eq!(w1 >> 21, MSR_HI, "bits[31:21] MSR group+L for {}", name);
        prop_assert_eq!(w1 & 0x1f, rt1, "Rt field for {}", name);
        prop_assert_eq!(w2 >> 21, MSR_HI, "bits[31:21] MSR group+L rt2");
        prop_assert_eq!(w2 & 0x1f, rt2, "Rt field rt2");
        prop_assert_eq!(
            (w1 ^ w2) & !0x1fu32,
            0,
            "different Xt must differ only in bits[4:0] for {}",
            name
        );
        if rt1 != rt2 {
            prop_assert_ne!(w1, w2, "different Rt must change the word");
        }
    }

    #[test]
    fn encode_msr_meta_casefold(
        name in named_sysreg(),
        cased in named_sysreg().prop_flat_map(|n| case_fold(n.to_string())),
        rt in 0u32..=31,
    ) {
        let _ = name;
        prop_assume!(is_named(&cased) || NAMED.iter().any(|n| n.eq_ignore_ascii_case(&cased)));
        let xt = xt_name(rt);
        let lower = cased.to_ascii_lowercase();
        let a = sut_word(&[
            Operand::Symbol(cased.clone()),
            Operand::Reg(xt.clone()),
        ])
        .expect("cased");
        let b = sut_word(&[
            Operand::Symbol(lower),
            Operand::Reg(xt),
        ])
        .expect("lower");
        prop_assert_eq!(a, b, "case-fold of {} must be encoding-invariant", cased);
    }

    #[test]
    fn encode_msr_neg_extra(
        name in named_sysreg(),
        xt in xt_gpr(),
        extra in extra_operand(),
    ) {
        let asm = format!("msr {name}, {xt}, {}", extra_asm(&extra));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [
            Operand::Symbol(name.to_string()),
            Operand::Reg(xt),
            extra,
        ];
        prop_assert!(
            encode_msr(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_msr_neg_wrong_src_unknown_arity_oob(
        kind in 0u8..=6,
        dest in wrong_xt(),
        name in named_sysreg(),
        unknown in unknown_name(),
        oob_g in oob_generic(),
        oob_n in oob_numbered(),
        field in pstate_field(),
        imm in oob_imm(),
        xt in xt_gpr(),
    ) {
        match kind {
            0 => {
                let asm = format!("msr {name}, {dest}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                let ops = [Operand::Symbol(name.to_string()), Operand::Reg(dest)];
                prop_assert!(
                    encode_msr(&ops).is_err(),
                    "non-Xt source must Err (llvm-mc rejects {}), got {:?}",
                    asm,
                    encode_msr(&ops)
                );
            }
            1 => {
                prop_assume!(!is_named(&unknown));
                let asm = format!("msr {unknown}, {xt}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_msr(&[
                        Operand::Symbol(unknown.clone()),
                        Operand::Reg(xt)
                    ])
                    .is_err(),
                    "unknown sysreg must Err (llvm-mc rejects {})",
                    asm
                );
            }
            2 => {
                prop_assert!(
                    llvm_mc_word("msr").is_err(),
                    "llvm-mc unexpectedly accepted omitted-operand msr"
                );
                prop_assert!(
                    encode_msr(&[]).is_err(),
                    "empty operands must Err (llvm-mc rejects omitted msr)"
                );
            }
            3 => {
                let asm = format!("msr {name}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_msr(&[Operand::Symbol(name.to_string())]).is_err(),
                    "missing Xt must Err (llvm-mc rejects {})",
                    asm
                );
            }
            4 => {
                let asm = format!("msr {oob_g}, {xt}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_msr(&[
                        Operand::Symbol(oob_g.clone()),
                        Operand::Reg(xt)
                    ])
                    .is_err(),
                    "oob generic sysreg must Err (llvm-mc rejects {})",
                    asm
                );
            }
            5 => {
                let asm = format!("msr {oob_n}, {xt}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_msr(&[
                        Operand::Symbol(oob_n.clone()),
                        Operand::Reg(xt)
                    ])
                    .is_err(),
                    "oob numbered sysreg must Err (llvm-mc rejects {})",
                    asm
                );
            }
            _ => {
                let asm = format!("msr {field}, #{imm}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_msr(&[
                        Operand::Symbol(field.to_string()),
                        Operand::Imm(imm)
                    ])
                    .is_err(),
                    "oob PSTATE imm must Err (llvm-mc rejects {})",
                    asm
                );
            }
        }
    }

    #[test]
    fn encode_msr_neg_wrong_src(dest in wrong_xt(), name in named_sysreg()) {
        let asm = format!("msr {name}, {dest}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [Operand::Symbol(name.to_string()), Operand::Reg(dest)];
        prop_assert!(
            encode_msr(&ops).is_err(),
            "non-Xt source must Err (llvm-mc rejects {}), got {:?}",
            asm,
            encode_msr(&ops)
        );
    }

    #[test]
    fn encode_msr_neg_oob_imm(field in pstate_field(), imm in oob_imm()) {
        let asm = format!("msr {field}, #{imm}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        prop_assert!(
            encode_msr(&[Operand::Symbol(field.to_string()), Operand::Imm(imm)]).is_err(),
            "oob PSTATE imm must Err (llvm-mc rejects {})",
            asm
        );
    }
}

#[test]
fn test_encode_msr_regression_cntv_cval_el0() {
    let mc = llvm_mc_word("msr cntv_cval_el0, x0").expect("llvm-mc msr cntv_cval_el0, x0");
    assert_eq!(mc, 0xd51be340);
    let sut = sut_word(&[
        Operand::Symbol("cntv_cval_el0".into()),
        Operand::Reg("x0".into()),
    ])
    .expect("named cntv_cval_el0 must encode");
    assert_eq!(
        sut, mc,
        "cntv_cval_el0 must encode ARM S3_3_C14_C3_2 (0xdf1a), not 0xdf1c"
    );
}

#[test]
fn test_encode_msr_regression_oslsr_el1() {
    assert!(
        llvm_mc_word("msr oslsr_el1, x0").is_err(),
        "llvm-mc must reject read-only OSLSR_EL1"
    );
    let ops = [
        Operand::Symbol("oslsr_el1".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_msr(&ops).is_err(),
        "msr oslsr_el1, x0 must Err (OSLSR_EL1 is read-only)"
    );
}

#[test]
fn test_encode_msr_regression_extra_x0() {
    assert!(
        llvm_mc_word("msr sp_el0, x0, x0").is_err(),
        "llvm-mc must reject extra operand"
    );
    let ops = [
        Operand::Symbol("sp_el0".into()),
        Operand::Reg("x0".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_msr(&ops).is_err(),
        "msr sp_el0, x0, x0 must Err (gas/llvm-mc reject extra operands)"
    );
}

#[test]
fn test_encode_msr_regression_w0_src() {
    assert!(
        llvm_mc_word("msr tpidr_el0, w0").is_err(),
        "llvm-mc must reject Wt source"
    );
    let ops = [
        Operand::Symbol("tpidr_el0".into()),
        Operand::Reg("w0".into()),
    ];
    assert!(
        encode_msr(&ops).is_err(),
        "msr tpidr_el0, w0 must Err (MSR requires Xt)"
    );
}

#[test]
fn test_encode_msr_regression_oob_generic_s4() {
    assert!(
        llvm_mc_word("msr s4_0_c1_c0_1, x0").is_err(),
        "llvm-mc must reject op0=4"
    );
    let ops = [
        Operand::Symbol("s4_0_c1_c0_1".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_msr(&ops).is_err(),
        "msr s4_0_c1_c0_1, x0 must Err (op0 outside 0..=3)"
    );
}

#[test]
fn test_encode_msr_regression_daifset_imm16() {
    assert!(
        llvm_mc_word("msr daifset, #16").is_err(),
        "llvm-mc must reject CRm imm 16"
    );
    let ops = [Operand::Symbol("daifset".into()), Operand::Imm(16)];
    assert!(
        encode_msr(&ops).is_err(),
        "msr daifset, #16 must Err (CRm immediate outside 0..=15)"
    );
}
