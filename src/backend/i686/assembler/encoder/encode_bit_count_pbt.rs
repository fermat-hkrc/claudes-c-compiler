// Oracle: differential — llvm-mc i686 assembler (LZCNT/TZCNT/POPCNT r32, r/m32)
// Evidence: gp_integer.rs:959 encode_bit_count; mod.rs:267
//   "lzcntl" | "tzcntl" | "popcntl" => encode_bit_count;
//   Intel SDM Vol.2 LZCNT/TZCNT/POPCNT — Opcode F3 0F BD/BC/B8 /r,
//   form r32, r/m32 (AT&T: src, dst); memory source is architecturally valid;
//   registers.rs:4 reg_num aliases r16/r8/xmm/mm/st to the same 3-bit numbers.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for bit-count
// Differential: candidate=encode_bit_count (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (F3 0F opc /r), algebraic.metamorphic (src/dst swap),
//   negative_error (arity / mem / wrong width / non-GP / imm-label).

use super::InstructionEncoder;
use crate::backend::x86::assembler::parser::{
    Displacement, ImmediateValue, Instruction, MemoryOperand, Operand, Register,
};
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

const GP32: &[&str] = &["eax", "ecx", "edx", "ebx", "esp", "ebp", "esi", "edi"];
const BAD_WIDTH: &[&str] = &[
    "ax", "cx", "dx", "bx", "sp", "bp", "si", "di", "al", "cl", "dl", "bl", "ah", "ch",
    "dh", "bh",
];
const MNEMS: &[&str] = &["lzcntl", "tzcntl", "popcntl"];
const NON_GP_ALIASED: &[&str] = &[
    "xmm0", "xmm1", "xmm7", "mm0", "mm3", "st", "st(0)", "st(1)", "ymm0", "ymm7",
];
const NON_GP_UNKNOWN: &[&str] = &[
    "es", "cs", "ss", "ds", "fs", "gs", "cr0", "cr2", "cr3", "cr4",
];

fn reg_num(name: &str) -> u8 {
    match name {
        "eax" | "ax" | "al" | "xmm0" | "mm0" | "st" | "st(0)" => 0,
        "ecx" | "cx" | "cl" | "xmm1" | "mm1" | "st(1)" => 1,
        "edx" | "dx" | "dl" | "xmm2" | "mm2" | "st(2)" => 2,
        "ebx" | "bx" | "bl" | "xmm3" | "mm3" | "st(3)" => 3,
        "esp" | "sp" | "ah" | "xmm4" | "mm4" | "st(4)" => 4,
        "ebp" | "bp" | "ch" | "xmm5" | "mm5" | "st(5)" => 5,
        "esi" | "si" | "dh" | "xmm6" | "mm6" | "st(6)" => 6,
        "edi" | "di" | "bh" | "xmm7" | "mm7" | "st(7)" => 7,
        _ => panic!("bad reg {name}"),
    }
}

fn opc(mnemonic: &str) -> u8 {
    match mnemonic {
        "lzcntl" => 0xBD,
        "tzcntl" => 0xBC,
        "popcntl" => 0xB8,
        _ => panic!("bad mnem {mnemonic}"),
    }
}

fn modrm(mod_: u8, reg: u8, rm: u8) -> u8 {
    (mod_ << 6) | ((reg & 7) << 3) | (rm & 7)
}

fn sut_encode(mnemonic: &str, ops: Vec<Operand>) -> Result<Vec<u8>, String> {
    let mut enc = InstructionEncoder::new();
    enc.encode(&Instruction {
        prefix: None,
        mnemonic: mnemonic.to_string(),
        operands: ops,
    })?;
    Ok(enc.bytes)
}

fn parse_llvm_bytes(stdout: &str) -> Result<Vec<u8>, String> {
    let marker = "encoding: [";
    let start = stdout
        .find(marker)
        .ok_or_else(|| format!("no encoding in llvm-mc output: {stdout}"))?
        + marker.len();
    let end = stdout[start..]
        .find(']')
        .ok_or_else(|| format!("no closing bracket in: {stdout}"))?
        + start;
    let body = &stdout[start..end];
    let mut bytes = Vec::new();
    for part in body.split(',') {
        let t = part.trim();
        if t.is_empty() {
            continue;
        }
        let v = u8::from_str_radix(t.trim_start_matches("0x"), 16)
            .map_err(|e| format!("bad byte {t}: {e}"))?;
        bytes.push(v);
    }
    if bytes.is_empty() {
        return Err(format!("empty encoding: {stdout}"));
    }
    Ok(bytes)
}

fn llvm_mc_bytes(asm: &str) -> Result<Vec<u8>, String> {
    let mut child = Command::new(LLVM_MC)
        .args(["-triple=i686", "-show-encoding"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("spawn llvm-mc: {e}"))?;
    {
        let mut stdin = child.stdin.take().ok_or("llvm-mc stdin")?;
        writeln!(stdin, "{asm}").map_err(|e| format!("write llvm-mc: {e}"))?;
    }
    let out = child
        .wait_with_output()
        .map_err(|e| format!("wait llvm-mc: {e}"))?;
    if !out.status.success() {
        return Err(format!(
            "llvm-mc error: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    parse_llvm_bytes(&String::from_utf8_lossy(&out.stdout))
}

fn mem_base(base: &str) -> MemoryOperand {
    MemoryOperand {
        segment: None,
        displacement: Displacement::None,
        base: Some(Register::new(base)),
        index: None,
        scale: None,
    }
}

// --- KAT gate (reference/differential prerequisite) ---

#[test]
fn encode_bit_count_kat_llvm_mc_lzcntl_eax_ebx() {
    let mc = llvm_mc_bytes("lzcntl %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xf3, 0x0f, 0xbd, 0xd8], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "lzcntl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT lzcntl %eax, %ebx");
}

#[test]
fn encode_bit_count_kat_llvm_mc_tzcntl_ecx_edx() {
    let mc = llvm_mc_bytes("tzcntl %ecx, %edx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xf3, 0x0f, 0xbc, 0xd1]);
    let sut = sut_encode(
        "tzcntl",
        vec![
            Operand::Register(Register::new("ecx")),
            Operand::Register(Register::new("edx")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bit_count_kat_llvm_mc_popcntl_esi_edi() {
    let mc = llvm_mc_bytes("popcntl %esi, %edi").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0xf3, 0x0f, 0xb8, 0xfe]);
    let sut = sut_encode(
        "popcntl",
        vec![
            Operand::Register(Register::new("esi")),
            Operand::Register(Register::new("edi")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bit_count_kat_llvm_mc_lzcntl_esp_ebp() {
    let mc = llvm_mc_bytes("lzcntl %esp, %ebp").expect("llvm-mc KAT");
    let sut = sut_encode(
        "lzcntl",
        vec![
            Operand::Register(Register::new("esp")),
            Operand::Register(Register::new("ebp")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — *l r32,r32 vs llvm-mc
    #[test]
    fn encode_bit_count_diff_rr_llvm_mc(
        m in prop::sample::select(MNEMS),
        s in prop::sample::select(GP32),
        d in prop::sample::select(GP32),
    ) {
        let asm = format!("{m} %{s}, %{d}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        let sut = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(s)),
                Operand::Register(Register::new(d)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected `{asm}`: {e}"));
        prop_assert_eq!(sut, mc, "diff vs llvm-mc for {}", asm);
    }

    // Oracle: algebraic.invariant — F3 0F opc ModRM(3,dst,src)
    #[test]
    fn encode_bit_count_invariant_opcode(
        m in prop::sample::select(MNEMS),
        s in prop::sample::select(GP32),
        d in prop::sample::select(GP32),
    ) {
        let expected = vec![
            0xF3u8,
            0x0F,
            opc(m),
            modrm(3, reg_num(d), reg_num(s)),
        ];
        let sut = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(s)),
                Operand::Register(Register::new(d)),
            ],
        )
        .unwrap_or_else(|e| panic!("SUT rejected {m} %{s}, %{d}: {e}"));
        prop_assert_eq!(sut, expected, "invariant F3 0F opc /r for {} %{}, %{}", m, s, d);
    }

    // Oracle: algebraic.metamorphic — src/dst swap changes encoding when s≠d
    #[test]
    fn encode_bit_count_meta_modrm_swap(
        m in prop::sample::select(MNEMS),
        s in prop::sample::select(GP32),
        d in prop::sample::select(GP32),
    ) {
        prop_assume!(s != d);
        let ab = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(s)),
                Operand::Register(Register::new(d)),
            ],
        )
        .unwrap_or_else(|e| panic!("{m} %{s}, %{d}: {e}"));
        let ba = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(d)),
                Operand::Register(Register::new(s)),
            ],
        )
        .unwrap_or_else(|e| panic!("{m} %{d}, %{s}: {e}"));
        prop_assert_ne!(ab.clone(), ba, "src/dst swap must change encoding for {}", m);
        // determinism
        let ab2 = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(s)),
                Operand::Register(Register::new(d)),
            ],
        )
        .unwrap();
        prop_assert_eq!(ab, ab2, "re-encode must be deterministic");
    }

    // Oracle: differential — memory source vs llvm-mc (Intel r32, r/m32)
    #[test]
    fn encode_bit_count_diff_mem_llvm_mc(
        m in prop::sample::select(MNEMS),
        base in prop::sample::select(GP32),
        d in prop::sample::select(GP32),
    ) {
        // esp as base needs SIB; still valid. ebp needs disp8=0 in some forms —
        // llvm-mc accepts bare (%ebp). Drive through both tools.
        let asm = format!("{m} (%{base}), %{d}");
        let mc = llvm_mc_bytes(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected `{asm}`: {e}"));
        match sut_encode(
            m,
            vec![
                Operand::Memory(mem_base(base)),
                Operand::Register(Register::new(d)),
            ],
        ) {
            Ok(sut) => {
                prop_assert_eq!(sut, mc, "diff mem vs llvm-mc for {}", asm);
            }
            Err(e) => {
                return Err(TestCaseError::fail(format!(
                    "SUT rejected valid mem-source `{asm}`: {e}; \
                     Intel SDM / llvm-mc encode r32, r/m32 (got mc={mc:02x?})"
                )));
            }
        }
    }

    // Oracle: negative_error — arity ≠ 2
    #[test]
    fn encode_bit_count_neg_arity(
        m in prop::sample::select(MNEMS),
        n in 0usize..5,
        r in prop::sample::select(GP32),
    ) {
        prop_assume!(n != 2);
        let ops: Vec<Operand> = (0..n)
            .map(|_| Operand::Register(Register::new(r)))
            .collect();
        let err = sut_encode(m, ops).expect_err("arity ≠ 2 must Err");
        prop_assert!(
            err.contains("requires 2 operands") || err.contains("operand"),
            "unexpected err text: {err}"
        );
    }

    // Oracle: negative_error — r16/r8 must be rejected for *l forms
    #[test]
    fn encode_bit_count_neg_wrong_width(
        m in prop::sample::select(MNEMS),
        s in prop::sample::select(BAD_WIDTH),
        d in prop::sample::select(GP32),
        flip in prop::bool::ANY,
    ) {
        let (src, dst) = if flip { (d, s) } else { (s, d) };
        // Ensure at least one operand is wrong-width (s is always BAD_WIDTH)
        let asm = format!("{m} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted invalid-width `{asm}` → {bytes:02x?}; \
                     {m} is r32-only (Intel SDM; llvm-mc rejects)"
                )));
            }
        }
    }

    // Oracle: negative_error — non-GP names aliased by reg_num must be rejected
    #[test]
    fn encode_bit_count_neg_non_gp(
        m in prop::sample::select(MNEMS),
        bad in prop::sample::select(NON_GP_ALIASED),
        d in prop::sample::select(GP32),
        flip in prop::bool::ANY,
    ) {
        let (src, dst) = if flip { (d, bad) } else { (bad, d) };
        let asm = format!("{m} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm);
        prop_assert!(mc.is_err(), "expected llvm-mc to reject `{asm}`");
        match sut_encode(
            m,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted non-GP `{asm}` → {bytes:02x?}; \
                     bit-count requires GP r32 (Intel SDM; reg_num must not alias xmm/mm/st/ymm)"
                )));
            }
        }
    }

    // Oracle: negative_error — Imm/Label and reg-as-only-dst-with-imm
    #[test]
    fn encode_bit_count_neg_imm_label(
        m in prop::sample::select(MNEMS),
        kind in 0u8..4,
        r in prop::sample::select(GP32),
    ) {
        let ops = match kind {
            0 => vec![
                Operand::Immediate(ImmediateValue::Integer(0)),
                Operand::Register(Register::new(r)),
            ],
            1 => vec![
                Operand::Register(Register::new(r)),
                Operand::Immediate(ImmediateValue::Integer(1)),
            ],
            2 => vec![
                Operand::Label("sym".into()),
                Operand::Register(Register::new(r)),
            ],
            _ => vec![
                Operand::Register(Register::new(r)),
                Operand::Memory(mem_base(r)),
            ],
        };
        let err = sut_encode(m, ops).expect_err("imm/label/mem-dst must Err");
        prop_assert!(
            err.contains("unsupported") || err.contains("operand") || err.contains(m),
            "unexpected err: {err}"
        );
    }

    // Strengthen: sreg/cr (unknown to reg_num) already Err
    #[test]
    fn encode_bit_count_neg_unknown_reg(
        m in prop::sample::select(MNEMS),
        r in prop::sample::select(NON_GP_UNKNOWN),
        d in prop::sample::select(GP32),
    ) {
        let err = sut_encode(
            m,
            vec![
                Operand::Register(Register::new(r)),
                Operand::Register(Register::new(d)),
            ],
        )
        .expect_err("sreg/cr must Err via bad register");
        prop_assert!(
            err.contains("bad register") || err.contains("register") || err.contains("unsupported"),
            "unexpected err for %{r}: {err}"
        );
    }

    // Strengthen round: distinct mnemonics → distinct opcodes (same regs)
    #[test]
    fn encode_bit_count_meta_mnemonic_opcodes_differ(
        s in prop::sample::select(GP32),
        d in prop::sample::select(GP32),
    ) {
        let lz = sut_encode(
            "lzcntl",
            vec![
                Operand::Register(Register::new(s)),
                Operand::Register(Register::new(d)),
            ],
        )
        .unwrap();
        let tz = sut_encode(
            "tzcntl",
            vec![
                Operand::Register(Register::new(s)),
                Operand::Register(Register::new(d)),
            ],
        )
        .unwrap();
        let pc = sut_encode(
            "popcntl",
            vec![
                Operand::Register(Register::new(s)),
                Operand::Register(Register::new(d)),
            ],
        )
        .unwrap();
        prop_assert_ne!(&lz[..], &tz[..]);
        prop_assert_ne!(&lz[..], &pc[..]);
        prop_assert_ne!(&tz[..], &pc[..]);
        prop_assert_eq!(lz[2], 0xBDu8);
        prop_assert_eq!(tz[2], 0xBCu8);
        prop_assert_eq!(pc[2], 0xB8u8);
    }
}

/// Deterministic regression: memory source must encode (Intel r32, r/m32).
#[test]
fn test_encode_bit_count_regression_mem_src_eax_eax() {
    let result = sut_encode(
        "lzcntl",
        vec![
            Operand::Memory(mem_base("eax")),
            Operand::Register(Register::new("eax")),
        ],
    );
    let mc = llvm_mc_bytes("lzcntl (%eax), %eax").expect("llvm-mc");
    assert_eq!(
        result.expect("lzcntl (%eax), %eax must encode"),
        mc,
        "memory source must match llvm-mc"
    );
}

/// Deterministic regression: r16 source must be rejected for *l forms.
#[test]
fn test_encode_bit_count_regression_rejects_ax_eax() {
    let result = sut_encode(
        "lzcntl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("eax")),
        ],
    );
    assert!(
        result.is_err(),
        "lzcntl %ax, %eax must Err (r32 only); got Ok({result:?})"
    );
}

/// Deterministic regression: xmm must not alias to GP bit-count encoding.
#[test]
fn test_encode_bit_count_regression_rejects_xmm0_eax() {
    let result = sut_encode(
        "lzcntl",
        vec![
            Operand::Register(Register::new("xmm0")),
            Operand::Register(Register::new("eax")),
        ],
    );
    assert!(
        result.is_err(),
        "lzcntl %xmm0, %eax must Err (GP r32 only); got Ok({result:?})"
    );
}
