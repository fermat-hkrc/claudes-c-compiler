// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:239 System table lists mrs;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:971 "mrs" => encode_mrs(operands);
//   system.rs:55 "MRS Xt, system_reg";
//   system.rs:175 "MRS encoding: 0xd520_0000 has L=1 (bit 21) for read.";
//   system.rs:59 "mrs needs system register name";
//   ARM ARM MRS: 1101 0101 00 1 op0 op1 CRn CRm op2 Rt;
//   Xt is a 64-bit GPR (x0–x30 / xzr); SP is not Xt.
// Stronger considered:
//   - State machine: rejected — encode_mrs is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree MRS decoder
//   - Differential vs encode_msr: rejected — same-job gate (MSR write / L=0 / reversed operands)
// Weaker available: algebraic.metamorphic (case-fold), algebraic.invariant (ARM layout),
//   negative_error (extra / wrong dest / unknown / arity / oob generic)
// Differential: candidate=encode_mrs, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[Reg(Xt), Symbol(sysreg)] <-> `mrs Xt, sysreg`

use super::encode_mrs;
use super::EncodeResult;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

/// ARM ARM MRS template: bits[31:21] = 1101_0101_001 (group + L=1).
const MRS_HI: u32 = 0b11010101001;

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
    "id_aa64mmfr0_el1",
    "id_aa64mmfr1_el1",
    "cpacr_el1",
    "par_el1",
    "osdlr_el1",
    "currentel",
    "elr_el1",
    "spsr_el1",
    "esr_el1",
    "far_el1",
    "vbar_el1",
    "mpidr_el1",
    "contextidr_el1",
    "mair_el1",
    "isr_el1",
    "oslsr_el1",
    "midr_el1",
    "revidr_el1",
    "id_aa64pfr0_el1",
    "id_aa64pfr1_el1",
    "id_aa64isar0_el1",
    "id_aa64isar1_el1",
    "id_aa64isar2_el1",
    "amair_el1",
    "hcr_el2",
    "cptr_el2",
    "hstr_el2",
    "hacr_el2",
    "vpidr_el2",
    "vmpidr_el2",
    "actlr_el2",
    "elr_el2",
    "esr_el2",
    "afsr0_el2",
    "afsr1_el2",
    "far_el2",
    "hpfar_el2",
    "spsr_el2",
    "sctlr_el2",
    "mdcr_el2",
    "tcr_el2",
    "ttbr0_el2",
    "vttbr_el2",
    "vtcr_el2",
    "vbar_el2",
    "mair_el2",
    "amair_el2",
    "sp_el1",
    "pmuserenr_el0",
    "cntfrq_el0",
    "cntpct_el0",
    "cntv_ctl_el0",
    "cntp_ctl_el0",
    "cntv_cval_el0",
    "cntp_cval_el0",
    "ctr_el0",
    "ttbr1_el1",
    "cntkctl_el1",
    "id_aa64dfr0_el1",
    "oslar_el1",
    "cntvct_el0",
    "clidr_el1",
    "ccsidr_el1",
    "csselr_el1",
    "id_aa64mmfr2_el1",
    "id_aa64dfr1_el1",
    "actlr_el1",
    "afsr0_el1",
    "afsr1_el1",
    "id_pfr0_el1",
    "id_pfr1_el1",
    "cnthctl_el2",
    "cntvoff_el2",
    "sp_el2",
    "pmintenset_el1",
    "pmintenclr_el1",
    "pmcr_el0",
    "pmcntenset_el0",
    "pmcntenclr_el0",
    "pmovsclr_el0",
    "pmselr_el0",
    "pmceid0_el0",
    "pmceid1_el0",
    "pmccntr_el0",
    "pmxevtyper_el0",
    "pmxevcntr_el0",
    "pmccfiltr_el0",
    "dczid_el0",
    "daif",
    "fpcr",
    "fpsr",
    "nzcv",
    "spsel",
    "mdccint_el1",
    "fpexc32_el2",
    "dbgauthstatus_el1",
    "spsr_abt",
    "spsr_und",
    "spsr_irq",
    "spsr_fiq",
    "ifsr32_el2",
    "dacr32_el2",
];

const DBG_FAMS: &[&str] = &["dbgbcr", "dbgbvr", "dbgwcr", "dbgwvr"];
const PMU_FAMS: &[&str] = &["pmevcntr", "pmevtyper"];

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn is_named(s: &str) -> bool {
    let l = s.to_ascii_lowercase();
    NAMED.iter().any(|n| *n == l)
}

fn xt_rt(xt: &str) -> u32 {
    match xt {
        "xzr" | "x31" => 31,
        "lr" => 30,
        s if s.starts_with('x') => s[1..].parse().unwrap_or(0),
        _ => 0,
    }
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

/// ARM ARM MRS word from unmasked sysreg fields (independent of SUT masking).
fn arm_mrs_word(op0: u32, op1: u32, crn: u32, crm: u32, op2: u32, rt: u32) -> u32 {
    let enc = (op0 << 14) | (op1 << 11) | (crn << 7) | (crm << 3) | op2;
    0xd5200000 | (enc << 5) | (rt & 0x1f)
}

fn sut_word(ops: &[Operand]) -> Result<u32, String> {
    match encode_mrs(ops)? {
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

fn wrong_dest() -> impl Strategy<Value = String> {
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
        Just("daifset".to_string()),
        Just("daifclr".to_string()),
        Just("sys".to_string()),
        Just("x".to_string()),
        Just("".to_string()),
        Just("s_0_c1_c0_1".to_string()),
        Just("s3_0_c1_c0".to_string()),
        Just("dbgbcr16_el1".to_string()),
        Just("pmevcntr31_el0".to_string()),
        prop::string::string_regex("[a-z]{1,10}")
            .unwrap()
            .prop_filter("not a named sysreg", |s| !is_named(s)),
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

#[test]
fn encode_mrs_kat_llvm_mc_x0_tpidr_el0() {
    let mc = llvm_mc_word("mrs x0, tpidr_el0").expect("llvm-mc mrs x0, tpidr_el0");
    assert_eq!(mc, 0xd53bd040, "llvm-mc KAT mapping broken for mrs x0, tpidr_el0");
    let sut = sut_word(&[
        Operand::Reg("x0".into()),
        Operand::Symbol("tpidr_el0".into()),
    ])
    .expect("SUT KAT tpidr_el0");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mrs_kat_llvm_mc_x0_nzcv() {
    let mc = llvm_mc_word("mrs x0, nzcv").expect("llvm-mc mrs x0, nzcv");
    assert_eq!(mc, 0xd53b4200, "llvm-mc KAT mapping broken for mrs x0, nzcv");
    let sut = sut_word(&[Operand::Reg("x0".into()), Operand::Symbol("nzcv".into())])
        .expect("SUT KAT nzcv");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mrs_kat_llvm_mc_xzr_nzcv() {
    let mc = llvm_mc_word("mrs xzr, nzcv").expect("llvm-mc mrs xzr, nzcv");
    assert_eq!(mc, 0xd53b421f, "llvm-mc KAT mapping broken for mrs xzr, nzcv");
    let sut = sut_word(&[
        Operand::Reg("xzr".into()),
        Operand::Symbol("nzcv".into()),
    ])
    .expect("SUT KAT xzr nzcv");
    assert_eq!(sut, mc);
}

#[test]
fn encode_mrs_kat_llvm_mc_x0_cntv_cval_el0_mapping() {
    // Mapping gate only — pins llvm-mc's CNTV_CVAL_EL0 word (ARM S3_3_C14_C3_2).
    let mc = llvm_mc_word("mrs x0, cntv_cval_el0").expect("llvm-mc mrs x0, cntv_cval_el0");
    assert_eq!(mc, 0xd53be340, "llvm-mc KAT mapping broken for mrs x0, cntv_cval_el0");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_mrs_diff_named(name in named_sysreg(), xt in xt_gpr()) {
        let asm = format!("mrs {xt}, {name}");
        let mc = llvm_mc_word(&asm);
        let sut = encode_mrs(&[
            Operand::Reg(xt.clone()),
            Operand::Symbol(name.to_string()),
        ]);
        match (mc, sut) {
            (Ok(w), Ok(EncodeResult::Word(w2))) => {
                prop_assert_eq!(w, w2, "SUT vs llvm-mc for {}", asm);
            }
            (Err(_), Err(_)) => {}
            (Ok(w), Err(e)) => {
                prop_assert!(
                    false,
                    "SUT rejected {}, llvm-mc encoded {:08x}: {e}",
                    asm, w
                );
            }
            (Err(e), Ok(EncodeResult::Word(w))) => {
                prop_assert!(
                    false,
                    "SUT encoded {} as {:08x}, llvm-mc rejected: {e}",
                    asm, w
                );
            }
            (Ok(w), Ok(other)) => {
                prop_assert!(
                    false,
                    "SUT returned {:?} for {}, llvm-mc encoded {:08x}",
                    other, asm, w
                );
            }
            (Err(e), Ok(other)) => {
                prop_assert!(
                    false,
                    "SUT returned {:?} for {}, llvm-mc rejected: {e}",
                    other, asm
                );
            }
        }
    }

    #[test]
    fn encode_mrs_diff_named_word(name in named_sysreg(), xt in xt_gpr()) {
        // Implication: when llvm-mc accepts a named sysreg, the SUT word must match.
        // Write-only names (llvm-mc Err) do not discharge this; they fail encode_mrs_diff_named.
        let asm = format!("mrs {xt}, {name}");
        if let Ok(mc) = llvm_mc_word(&asm) {
            let sut = sut_word(&[
                Operand::Reg(xt.clone()),
                Operand::Symbol(name.to_string()),
            ])
            .expect(&asm);
            prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
        }
    }

    #[test]
    fn encode_mrs_diff_generic(
        op0 in 0u32..=3,
        op1 in 0u32..=7,
        crn in 0u32..=15,
        crm in 0u32..=15,
        op2 in 0u32..=7,
        rt in 0u32..=31,
    ) {
        let name = sform(op0, op1, crn, crm, op2);
        let xt = xt_name(rt);
        let asm = format!("mrs {xt}, {name}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let expect = arm_mrs_word(op0, op1, crn, crm, op2, rt);
        prop_assert_eq!(mc, expect, "llvm-mc vs ARM ARM for {}", asm);
        let sut = sut_word(&[
            Operand::Reg(xt),
            Operand::Symbol(name),
        ])
        .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_mrs_diff_numbered(name in numbered_valid(), rt in 0u32..=31) {
        let xt = xt_name(rt);
        let asm = format!("mrs {xt}, {name}");
        let mc = llvm_mc_word(&asm).expect(&asm);
        let sut = sut_word(&[
            Operand::Reg(xt),
            Operand::Symbol(name.clone()),
        ])
        .expect(&asm);
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc for {}", asm);
    }

    #[test]
    fn encode_mrs_inv_arm_layout(
        name in named_sysreg(),
        rt1 in 0u32..=31,
        rt2 in 0u32..=31,
    ) {
        let w1 = sut_word(&[
            Operand::Reg(xt_name(rt1)),
            Operand::Symbol(name.to_string()),
        ]);
        let w2 = sut_word(&[
            Operand::Reg(xt_name(rt2)),
            Operand::Symbol(name.to_string()),
        ]);
        let w1 = w1.expect("named sysreg must encode");
        let w2 = w2.expect("named sysreg must encode");
        prop_assert_eq!(w1 >> 21, MRS_HI, "bits[31:21] MRS group+L for {}", name);
        prop_assert_eq!(w1 & 0x1f, rt1, "Rt field for {}", name);
        prop_assert_eq!(w2 >> 21, MRS_HI, "bits[31:21] MRS group+L rt2");
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
    fn encode_mrs_meta_casefold(
        name in named_sysreg(),
        cased in named_sysreg().prop_flat_map(|n| case_fold(n.to_string())),
        rt in 0u32..=31,
    ) {
        let _ = name;
        prop_assume!(is_named(&cased) || NAMED.iter().any(|n| n.eq_ignore_ascii_case(&cased)));
        let xt = xt_name(rt);
        let lower = cased.to_ascii_lowercase();
        let a = sut_word(&[
            Operand::Reg(xt.clone()),
            Operand::Symbol(cased.clone()),
        ])
        .expect("cased");
        let b = sut_word(&[
            Operand::Reg(xt),
            Operand::Symbol(lower),
        ])
        .expect("lower");
        prop_assert_eq!(a, b, "case-fold of {} must be encoding-invariant", cased);
    }

    #[test]
    fn encode_mrs_neg_extra(
        name in named_sysreg(),
        xt in xt_gpr(),
        extra in extra_operand(),
    ) {
        let asm = format!("mrs {xt}, {name}, {}", extra_asm(&extra));
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted extra {}",
            asm
        );
        let ops = [
            Operand::Reg(xt),
            Operand::Symbol(name.to_string()),
            extra,
        ];
        prop_assert!(
            encode_mrs(&ops).is_err(),
            "extra operand must Err (llvm-mc rejects {})",
            asm
        );
    }

    #[test]
    fn encode_mrs_neg_wrong_dest(dest in wrong_dest(), name in named_sysreg()) {
        let asm = format!("mrs {dest}, {name}");
        prop_assert!(
            llvm_mc_word(&asm).is_err(),
            "llvm-mc unexpectedly accepted {}",
            asm
        );
        let ops = [Operand::Reg(dest.clone()), Operand::Symbol(name.to_string())];
        prop_assert!(
            encode_mrs(&ops).is_err(),
            "non-Xt dest must Err (llvm-mc rejects {}), got {:?}",
            asm,
            encode_mrs(&ops)
        );
    }

    #[test]
    fn encode_mrs_neg_unknown_arity_oob(
        kind in 0u8..=4,
        unknown in unknown_name(),
        oob_g in oob_generic(),
        oob_n in oob_numbered(),
        xt in xt_gpr(),
    ) {
        match kind {
            0 => {
                prop_assume!(!is_named(&unknown));
                let asm = format!("mrs {xt}, {unknown}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_mrs(&[
                        Operand::Reg(xt),
                        Operand::Symbol(unknown.clone())
                    ])
                    .is_err(),
                    "unknown sysreg must Err (llvm-mc rejects {})",
                    asm
                );
            }
            1 => {
                prop_assert!(
                    llvm_mc_word("mrs").is_err(),
                    "llvm-mc unexpectedly accepted omitted-operand mrs"
                );
                prop_assert!(
                    encode_mrs(&[]).is_err(),
                    "empty operands must Err (llvm-mc rejects omitted mrs)"
                );
            }
            2 => {
                let asm = format!("mrs {xt}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_mrs(&[Operand::Reg(xt)]).is_err(),
                    "missing sysreg must Err (llvm-mc rejects {})",
                    asm
                );
            }
            3 => {
                let asm = format!("mrs {xt}, {oob_g}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_mrs(&[
                        Operand::Reg(xt),
                        Operand::Symbol(oob_g.clone())
                    ])
                    .is_err(),
                    "oob generic sysreg must Err (llvm-mc rejects {})",
                    asm
                );
            }
            _ => {
                let asm = format!("mrs {xt}, {oob_n}");
                prop_assert!(
                    llvm_mc_word(&asm).is_err(),
                    "llvm-mc unexpectedly accepted {}",
                    asm
                );
                prop_assert!(
                    encode_mrs(&[
                        Operand::Reg(xt),
                        Operand::Symbol(oob_n.clone())
                    ])
                    .is_err(),
                    "oob numbered sysreg must Err (llvm-mc rejects {})",
                    asm
                );
            }
        }
    }
}

#[test]
fn test_encode_mrs_regression_oslar_el1() {
    assert!(
        llvm_mc_word("mrs x0, oslar_el1").is_err(),
        "llvm-mc must reject write-only OSLAR_EL1"
    );
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Symbol("oslar_el1".into()),
    ];
    assert!(
        encode_mrs(&ops).is_err(),
        "mrs x0, oslar_el1 must Err (OSLAR_EL1 is write-only)"
    );
}

#[test]
fn test_encode_mrs_regression_cntv_cval_el0() {
    let mc = llvm_mc_word("mrs x0, cntv_cval_el0").expect("llvm-mc mrs x0, cntv_cval_el0");
    assert_eq!(mc, 0xd53be340);
    let sut = sut_word(&[
        Operand::Reg("x0".into()),
        Operand::Symbol("cntv_cval_el0".into()),
    ])
    .expect("named cntv_cval_el0 must encode");
    assert_eq!(
        sut, mc,
        "cntv_cval_el0 must encode ARM S3_3_C14_C3_2 (0xdf1a), not 0xdf1c"
    );
}

#[test]
fn test_encode_mrs_regression_extra_x0() {
    assert!(
        llvm_mc_word("mrs x0, sp_el0, x0").is_err(),
        "llvm-mc must reject extra operand"
    );
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Symbol("sp_el0".into()),
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_mrs(&ops).is_err(),
        "mrs x0, sp_el0, x0 must Err (gas/llvm-mc reject extra operands)"
    );
}

#[test]
fn test_encode_mrs_regression_w0_dest() {
    assert!(
        llvm_mc_word("mrs w0, sp_el0").is_err(),
        "llvm-mc must reject Wt dest"
    );
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Symbol("sp_el0".into()),
    ];
    assert!(
        encode_mrs(&ops).is_err(),
        "mrs w0, sp_el0 must Err (MRS requires Xt)"
    );
}

#[test]
fn test_encode_mrs_regression_oob_generic_s4() {
    assert!(
        llvm_mc_word("mrs x0, s4_0_c1_c0_1").is_err(),
        "llvm-mc must reject op0=4"
    );
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Symbol("s4_0_c1_c0_1".into()),
    ];
    assert!(
        encode_mrs(&ops).is_err(),
        "mrs x0, s4_0_c1_c0_1 must Err (op0 outside 0..=3)"
    );
}
