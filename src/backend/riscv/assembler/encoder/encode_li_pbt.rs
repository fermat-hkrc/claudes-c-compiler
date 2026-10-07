// Oracle: differential — llvm-mc RISC-V assembler (LI pseudo expansion)
// Evidence: assembler/README.md:318 `li rd, imm` → `lui + addi(w)` or single `addi`;
//   encoder/mod.rs:3 Encodes RISC-V instructions into 32-bit machine code words;
//   encoder/mod.rs:854 "li" => encode_li;
//   pseudo.rs:22-27 GAS always uses addiw after lui for li on RV64; 12-bit addi;
//   pseudo.rs:47-51 Encode li for an arbitrary 64-bit immediate;
//   RISC-V Unprivileged ISA LI pseudo leaves imm in rd.
// Stronger considered:
//   - State machine: rejected — encode_li is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree LI decoder
//   - 64-bit encoding-equality vs llvm-mc: rejected as primary — independent
//     assemblers may choose different 64-bit expansions (semantic agreement used)
// Weaker available: algebraic.invariant (I-type 12-bit layout), algebraic.metamorphic
//   (ABI/xN alias, Imm 0..31 as rd), negative_error (arity / invalid / extra)
// Differential: candidate=encode_li, reference=llvm-mc -triple=riscv64 -show-encoding,
//   SUT-boundary=internal-helper of RISC-V assembler,
//   mapping=[Reg(rd), Imm(imm)] <-> `li rd, imm`
// Reference interpreter: RISC-V ISA LUI/ADDI/ADDIW/SLLI (not a copy of the decomposer)

use super::{encode_li, EncodeResult};
use crate::backend::riscv::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const MAX_WORDS: usize = 16;

const ABI: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
    "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
    "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
    "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6",
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

fn sut_words(ops: &[Operand]) -> Result<Vec<u32>, String> {
    match encode_li(ops)? {
        EncodeResult::Word(w) => Ok(vec![w]),
        EncodeResult::Words(ws) => {
            if ws.is_empty() {
                return Err("empty Words".into());
            }
            Ok(ws)
        }
        other => Err(format!("expected Word or Words, got {:?}", other)),
    }
}

fn parse_llvm_encodings(stdout: &str) -> Result<Vec<u32>, String> {
    let marker = "encoding: [";
    let mut words = Vec::new();
    let mut rest = stdout;
    while let Some(pos) = rest.find(marker) {
        rest = &rest[pos + marker.len()..];
        let end = rest
            .find(']')
            .ok_or_else(|| format!("no closing bracket: {stdout}"))?;
        let inner = &rest[..end];
        rest = &rest[end + 1..];
        let parts: Vec<&str> = inner.split(',').collect();
        if parts.len() != 4 {
            return Err(format!("expected 4 bytes, got {inner}"));
        }
        let mut bytes = [0u8; 4];
        for (i, p) in parts.iter().enumerate() {
            let p = p.trim();
            let hex = p
                .strip_prefix("0x")
                .ok_or_else(|| format!("non-hex byte {p}"))?;
            bytes[i] = u8::from_str_radix(hex, 16).map_err(|e| e.to_string())?;
        }
        words.push(u32::from_le_bytes(bytes));
    }
    if words.is_empty() {
        return Err(format!("no encoding in stdout: {stdout}"));
    }
    Ok(words)
}

fn llvm_mc_words(asm: &str) -> Result<Vec<u32>, String> {
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
    parse_llvm_encodings(&stdout)
}

/// Simulate integer LI expansions per RISC-V unprivileged ISA (LUI + OP-IMM +
/// OP-IMM-32). Not a copy of encode_li_immediate. x0 is hardwired zero after every write.
fn sim_li(words: &[u32]) -> Result<[i64; 32], String> {
    let mut regs = [0i64; 32];
    for &w in words {
        let opcode = w & 0x7f;
        let rd = (w >> 7) & 0x1f;
        match opcode {
            0b0110111 => {
                // LUI: rd = sext(imm[31:12] << 12)
                let imm = (w & 0xFFFFF000) as i32 as i64;
                regs[rd as usize] = imm;
            }
            0b0010011 => {
                // OP-IMM
                let funct3 = (w >> 12) & 7;
                let rs1 = (w >> 15) & 0x1f;
                let imm12 = ((w as i32) >> 20) as i64;
                let shamt = (w >> 20) & 0x3f;
                let rs1v = regs[rs1 as usize];
                regs[rd as usize] = match funct3 {
                    0b000 => rs1v.wrapping_add(imm12), // addi
                    0b001 => rs1v << shamt,            // slli
                    0b010 => (rs1v < imm12) as i64,    // slti
                    0b011 => ((rs1v as u64) < (imm12 as u64)) as i64, // sltiu
                    0b100 => rs1v ^ imm12,             // xori
                    0b101 => {
                        if (w >> 30) & 1 == 1 {
                            rs1v >> shamt // srai
                        } else {
                            ((rs1v as u64) >> shamt) as i64 // srli
                        }
                    }
                    0b110 => rs1v | imm12, // ori
                    0b111 => rs1v & imm12, // andi
                    _ => unreachable!(),
                };
            }
            0b0011011 => {
                // OP-IMM-32
                let funct3 = (w >> 12) & 7;
                let rs1 = (w >> 15) & 0x1f;
                let imm12 = ((w as i32) >> 20) as i64;
                let shamt = (w >> 20) & 0x1f;
                let rs1v = regs[rs1 as usize] as i32;
                let val32: i32 = match funct3 {
                    0b000 => rs1v.wrapping_add(imm12 as i32), // addiw
                    0b001 => rs1v << shamt,                   // slliw
                    0b101 => {
                        if (w >> 30) & 1 == 1 {
                            rs1v >> shamt // sraiw
                        } else {
                            ((rs1v as u32) >> shamt) as i32 // srliw
                        }
                    }
                    _ => {
                        return Err(format!("unexpected OP-IMM-32 funct3 {funct3} in {w:#010x}"));
                    }
                };
                regs[rd as usize] = val32 as i64;
            }
            _ => return Err(format!("unexpected opcode {opcode:#x} in {w:#010x}")),
        }
        regs[0] = 0;
    }
    Ok(regs)
}

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
        Just("a0".to_string()),
    ]
}

fn i32_imm() -> impl Strategy<Value = i32> {
    prop_oneof![
        Just(0i32),
        Just(1),
        Just(-1),
        Just(2047),
        Just(2048),
        Just(-2048),
        Just(-2049),
        Just(i32::MIN),
        Just(i32::MAX),
        Just(0x1000),
        Just(0x12345000u32 as i32),
        Just(0x7fffffff),
        -2048i32..=2047,
        proptest::num::i32::ANY,
    ]
}

fn i64_imm() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(0i64),
        Just(1),
        Just(-1),
        Just(2047),
        Just(2048),
        Just(-2048),
        Just(-2049),
        Just(i32::MIN as i64),
        Just(i32::MAX as i64),
        Just(1i64 << 31),
        Just(i64::MIN),
        Just(i64::MAX),
        Just(0x123456789abcdef0u64 as i64),
        Just(0x0123456789abcdef),
        Just(0x7fffffffffffffff),
        Just(-0x80000000i64),
        -2048i64..=2047,
        (i32::MIN as i64)..=(i32::MAX as i64),
        proptest::num::i64::ANY,
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

fn invalid_rd() -> impl Strategy<Value = Operand> {
    prop_oneof![
        prop::sample::select(vec![
            "fa0", "ft0", "f0", "fs0", "fa1", "f31", "v0", "v31", "x32", "x",
            "foo", "", "spx", "x-1", "r0", "w0",
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

fn invalid_imm() -> impl Strategy<Value = Operand> {
    prop_oneof![
        gpr_name().prop_map(Operand::Reg),
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
        invalid_rd().prop_map(|o| vec![o]),
        (-4i64..=40).prop_map(|i| vec![Operand::Imm(i)]),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_li_kat_llvm_mc() {
    let mc0 = llvm_mc_words("li a0, 0").expect("llvm-mc KAT li a0, 0");
    assert_eq!(mc0, vec![0x0000_0513u32], "llvm-mc KAT mapping broken for li a0, 0");
    assert_eq!(
        sut_words(&[reg("a0"), Operand::Imm(0)]).expect("SUT KAT li a0, 0"),
        mc0
    );

    let mc1 = llvm_mc_words("li a0, 1").expect("llvm-mc KAT li a0, 1");
    assert_eq!(mc1, vec![0x0010_0513u32]);
    assert_eq!(
        sut_words(&[reg("a0"), Operand::Imm(1)]).expect("SUT KAT li a0, 1"),
        mc1
    );

    let mc_hi = llvm_mc_words("li a0, 2047").expect("llvm-mc KAT li a0, 2047");
    assert_eq!(mc_hi, vec![0x7ff0_0513u32]);
    assert_eq!(
        sut_words(&[reg("a0"), Operand::Imm(2047)]).expect("SUT KAT 2047"),
        mc_hi
    );

    let mc_lo = llvm_mc_words("li a0, -2048").expect("llvm-mc KAT li a0, -2048");
    assert_eq!(mc_lo, vec![0x8000_0513u32]);
    assert_eq!(
        sut_words(&[reg("a0"), Operand::Imm(-2048)]).expect("SUT KAT -2048"),
        mc_lo
    );

    let mc_2048 = llvm_mc_words("li a0, 2048").expect("llvm-mc KAT li a0, 2048");
    assert_eq!(mc_2048, vec![0x0000_1537u32, 0x8005_051bu32]);
    assert_eq!(
        sut_words(&[reg("a0"), Operand::Imm(2048)]).expect("SUT KAT 2048"),
        mc_2048
    );

    let mc_max = llvm_mc_words("li a0, 2147483647").expect("llvm-mc KAT i32::MAX");
    assert_eq!(mc_max, vec![0x8000_0537u32, 0xfff5_051bu32]);
    assert_eq!(
        sut_words(&[reg("a0"), Operand::Imm(2147483647)]).expect("SUT KAT i32::MAX"),
        mc_max
    );

    let mc_nop = llvm_mc_words("li x0, 0").expect("llvm-mc KAT li x0, 0");
    assert_eq!(mc_nop, vec![0x0000_0013u32]);
    assert_eq!(
        sut_words(&[reg("x0"), Operand::Imm(0)]).expect("SUT KAT li x0, 0"),
        mc_nop
    );
    assert_eq!(
        sut_words(&[reg("zero"), Operand::Imm(0)]).expect("SUT KAT li zero, 0"),
        mc_nop
    );

    let extra = llvm_mc_words("li a0, 1, a1");
    assert!(extra.is_err(), "llvm-mc must reject extra operand");
    let missing = llvm_mc_words("li a0");
    assert!(missing.is_err(), "llvm-mc must reject missing imm");
}

/// Regression: extra operand must be rejected (README two-operand form; llvm-mc errors).
#[test]
fn test_encode_li_regression_extra_operand() {
    let ops = [reg("a0"), Operand::Imm(1), reg("a1")];
    assert!(
        encode_li(&ops).is_err(),
        "li a0, 1 with a third operand must be rejected (llvm-mc rejects; README documents `li rd, imm`)"
    );
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_li_diff_llvm_mc_i32(rd in gpr_name(), imm in i32_imm()) {
        let asm = format!("li {}, {}", rd, imm);
        let mc = llvm_mc_words(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_words(&[reg(&rd), Operand::Imm(imm as i64)])
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(&sut, &mc, "SUT {:08x?} != llvm-mc {:08x?} for {}", sut, mc, asm);
    }

    #[test]
    fn encode_li_sem_i64(rd in 1u32..=31, imm in i64_imm()) {
        let ops = [reg(&xn(rd)), Operand::Imm(imm)];
        let sut = sut_words(&ops)
            .unwrap_or_else(|e| panic!("SUT rejected li x{}, {}: {}", rd, imm, e));
        prop_assert!(
            sut.len() <= MAX_WORDS,
            "li x{}, {} produced {} insns (cap {})",
            rd, imm, sut.len(), MAX_WORDS
        );
        let got = sim_li(&sut)
            .unwrap_or_else(|e| panic!("sim SUT li x{}, {}: {}", rd, imm, e));
        prop_assert_eq!(
            got[rd as usize], imm,
            "SUT sim x{}={:#x} != {:#x} words={:08x?}",
            rd, got[rd as usize], imm, sut
        );

        let asm = format!("li x{}, {}", rd, imm);
        let mc = llvm_mc_words(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let mc_got = sim_li(&mc)
            .unwrap_or_else(|e| panic!("sim llvm-mc {}: {}", asm, e));
        prop_assert_eq!(
            mc_got[rd as usize], imm,
            "llvm-mc sim x{}={:#x} != {:#x} (mapping check)",
            rd, mc_got[rd as usize], imm
        );
        prop_assert_eq!(
            got[rd as usize], mc_got[rd as usize],
            "SUT sim {:#x} != llvm-mc sim {:#x} for {}",
            got[rd as usize], mc_got[rd as usize], asm
        );
    }

    #[test]
    fn encode_li_12bit_fields(rd in reg_num(), imm in -2048i64..=2047) {
        let w = match encode_li(&[reg(&xn(rd)), Operand::Imm(imm)]) {
            Ok(EncodeResult::Word(w)) => w,
            other => {
                return Err(TestCaseError::fail(format!(
                    "12-bit li x{}, {} expected Word, got {:?}",
                    rd, imm, other
                )));
            }
        };
        prop_assert_eq!(w & 0x7F, 0b0010011u32, "opcode addi");
        prop_assert_eq!((w >> 7) & 0x1F, rd, "rd");
        prop_assert_eq!((w >> 12) & 7, 0u32, "funct3");
        prop_assert_eq!((w >> 15) & 0x1F, 0u32, "rs1 must be x0");
        let imm12 = ((w as i32) >> 20) as i64;
        prop_assert_eq!(imm12, imm, "imm12");
    }

    #[test]
    fn encode_li_abi_xn_alias(n in reg_num(), imm in i64_imm()) {
        let via_x = sut_words(&[reg(&xn(n)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        let via_abi = sut_words(&[reg(abi_name(n)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("ABI rejected: {}", e));
        prop_assert_eq!(&via_abi, &via_x);
        if n == 8 {
            let via_fp = sut_words(&[reg("fp"), Operand::Imm(imm)])
                .unwrap_or_else(|e| panic!("fp rejected: {}", e));
            prop_assert_eq!(&via_fp, &via_x);
        }
        if n == 0 {
            let via_zero = sut_words(&[reg("zero"), Operand::Imm(imm)])
                .unwrap_or_else(|e| panic!("zero rejected: {}", e));
            prop_assert_eq!(&via_zero, &via_x);
        }
    }

    #[test]
    fn encode_li_neg_arity(ops in short_ops()) {
        prop_assume!(ops.len() < 2);
        prop_assert!(encode_li(&ops).is_err(), "arity {} must Err", ops.len());
    }

    #[test]
    fn encode_li_neg_invalid(
        bad_rd in invalid_rd(),
        bad_imm in invalid_imm(),
        good_rd in gpr_name(),
        good_imm in i64_imm(),
        which in 0u8..=2u8,
    ) {
        let ops: Vec<Operand> = match which {
            0 => vec![bad_rd, Operand::Imm(good_imm)],
            1 => vec![reg(&good_rd), bad_imm],
            _ => vec![bad_rd, bad_imm],
        };
        prop_assert!(
            encode_li(&ops).is_err(),
            "invalid operand must Err, got {:?}",
            encode_li(&ops)
        );
    }

    #[test]
    fn encode_li_neg_extra(
        rd in gpr_name(),
        imm in i64_imm(),
        extra in extra_operand(),
    ) {
        let ops = vec![reg(&rd), Operand::Imm(imm), extra];
        prop_assert!(
            encode_li(&ops).is_err(),
            "extra operand must Err for li {}, {} (README two-operand; llvm-mc rejects)",
            rd, imm
        );
    }

    #[test]
    fn encode_li_imm_regnum(n in 0i64..=31, imm in i64_imm()) {
        let via_imm = sut_words(&[Operand::Imm(n), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("Imm({}, {}) rejected: {}", n, imm, e));
        let via_x = sut_words(&[reg(&xn(n as u32)), Operand::Imm(imm)])
            .unwrap_or_else(|e| panic!("xN rejected: {}", e));
        prop_assert_eq!(via_imm, via_x);
    }
}
