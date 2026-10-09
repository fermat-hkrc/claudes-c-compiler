// Oracle: differential — llvm-mc i686 assembler (SHLD/SHRD Imm8|CL, r32, r/m32)
// Evidence: gp_integer.rs:920-942 encode_double_shift; mod.rs:250-251 shldl/shld/shrdl/shrd;
//   assembler/README.md:171 lists shld/shrd; Intel SDM Vol.2 SHLD/SHRD —
//   0F A4/A5 (SHLD) and 0F AC/AD (SHRD) with Imm8 or CL, r/m16|32, r16|32.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for shld/shrd
// Differential: candidate=encode_double_shift (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (0F A4/AC /r ib; 0F A5/AD /r),
//   algebraic.metamorphic (shld ≡ shldl; shrd ≡ shrdl),
//   negative_error (arity / non-GP / width / Imm8 domain / non-CL count reg).

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
const GP16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const GP8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const XMM: &[&str] = &["xmm0", "xmm1", "xmm2", "xmm3", "xmm4", "xmm5", "xmm6", "xmm7"];
const SREGS: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];
const MNEMS: &[&str] = &["shldl", "shld", "shrdl", "shrd"];
const MNEMS32: &[&str] = &["shldl", "shrdl"];

fn reg_num(name: &str) -> u8 {
    match name {
        "ax" | "eax" | "al" => 0,
        "cx" | "ecx" | "cl" => 1,
        "dx" | "edx" | "dl" => 2,
        "bx" | "ebx" | "bl" => 3,
        "sp" | "esp" | "ah" => 4,
        "bp" | "ebp" | "ch" => 5,
        "si" | "esi" | "dh" => 6,
        "di" | "edi" | "bh" => 7,
        _ => panic!("bad reg {name}"),
    }
}

fn opc_imm(mnemonic: &str) -> u8 {
    match mnemonic {
        "shldl" | "shld" => 0xA4,
        "shrdl" | "shrd" => 0xAC,
        _ => panic!("bad mnem {mnemonic}"),
    }
}

fn mem_base(base: &str, disp: Displacement, segment: Option<&str>) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: disp,
        base: Some(Register::new(base)),
        index: None,
        scale: None,
    }
}

fn mem_base_index(
    base: Option<&str>,
    index: &str,
    scale: u8,
    disp: Displacement,
    segment: Option<&str>,
) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: disp,
        base: base.map(Register::new),
        index: Some(Register::new(index)),
        scale: Some(scale),
    }
}

fn mem_abs(disp: i64, segment: Option<&str>) -> MemoryOperand {
    MemoryOperand {
        segment: segment.map(|s| s.to_string()),
        displacement: Displacement::Integer(disp),
        base: None,
        index: None,
        scale: None,
    }
}

fn att_disp(d: &Displacement) -> String {
    match d {
        Displacement::None => String::new(),
        Displacement::Integer(0) => String::new(),
        Displacement::Integer(v) => format!("{v}"),
        Displacement::Symbol(s) => s.clone(),
        Displacement::SymbolAddend(s, a) => {
            if *a >= 0 {
                format!("{s}+{a}")
            } else {
                format!("{s}{a}")
            }
        }
        Displacement::SymbolPlusOffset(s, a) => {
            if *a >= 0 {
                format!("{s}+{a}")
            } else {
                format!("{s}{a}")
            }
        }
        Displacement::SymbolMod(s, m) => format!("{s}@{m}"),
    }
}

fn att_mem(mem: &MemoryOperand) -> String {
    if mem.base.is_none() && mem.index.is_none() {
        let mut abs = String::new();
        if let Some(ref seg) = mem.segment {
            abs.push('%');
            abs.push_str(seg);
            abs.push(':');
        }
        match &mem.displacement {
            Displacement::None => abs.push_str("0"),
            Displacement::Integer(v) => abs.push_str(&format!("{v}")),
            other => abs.push_str(&att_disp(other)),
        }
        return abs;
    }
    let mut s = String::new();
    if let Some(ref seg) = mem.segment {
        s.push('%');
        s.push_str(seg);
        s.push(':');
    }
    s.push_str(&att_disp(&mem.displacement));
    s.push('(');
    match (&mem.base, &mem.index, mem.scale) {
        (Some(b), None, _) => {
            s.push('%');
            s.push_str(&b.name);
        }
        (Some(b), Some(i), sc) => {
            s.push('%');
            s.push_str(&b.name);
            s.push(',');
            s.push('%');
            s.push_str(&i.name);
            s.push(',');
            s.push_str(&format!("{}", sc.unwrap_or(1)));
        }
        (None, Some(i), sc) => {
            s.push(',');
            s.push('%');
            s.push_str(&i.name);
            s.push(',');
            s.push_str(&format!("{}", sc.unwrap_or(1)));
        }
        (None, None, _) => {}
    }
    s.push(')');
    s
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
        if t.chars().all(|c| c == 'A') {
            for _ in 0..t.len() {
                bytes.push(0);
            }
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

fn imm_ops(count: i64, src: &str, dst: &str) -> Vec<Operand> {
    vec![
        Operand::Immediate(ImmediateValue::Integer(count)),
        Operand::Register(Register::new(src)),
        Operand::Register(Register::new(dst)),
    ]
}

fn cl_ops(src: &str, dst: &str) -> Vec<Operand> {
    vec![
        Operand::Register(Register::new("cl")),
        Operand::Register(Register::new(src)),
        Operand::Register(Register::new(dst)),
    ]
}

fn imm_mem_ops(count: i64, src: &str, mem: MemoryOperand) -> Vec<Operand> {
    vec![
        Operand::Immediate(ImmediateValue::Integer(count)),
        Operand::Register(Register::new(src)),
        Operand::Memory(mem),
    ]
}

fn cl_mem_ops(src: &str, mem: MemoryOperand) -> Vec<Operand> {
    vec![
        Operand::Register(Register::new("cl")),
        Operand::Register(Register::new(src)),
        Operand::Memory(mem),
    ]
}

// --- KAT gate (reference/differential prerequisite) ---

#[test]
fn encode_double_shift_kat_llvm_mc_shldl_imm() {
    let mc = llvm_mc_bytes("shldl $1, %eax, %edx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xa4, 0xc2, 0x01], "llvm-mc KAT mapping broken");
    let sut = sut_encode("shldl", imm_ops(1, "eax", "edx")).expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_double_shift_kat_llvm_mc_shldl_cl() {
    let mc = llvm_mc_bytes("shldl %cl, %eax, %edx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xa5, 0xc2]);
    let sut = sut_encode("shldl", cl_ops("eax", "edx")).expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_double_shift_kat_llvm_mc_shrdl_imm() {
    let mc = llvm_mc_bytes("shrdl $5, %ebx, %ecx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xac, 0xd9, 0x05]);
    let sut = sut_encode("shrdl", imm_ops(5, "ebx", "ecx")).expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_double_shift_kat_llvm_mc_shrdl_cl() {
    let mc = llvm_mc_bytes("shrdl %cl, %esi, %edi").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xad, 0xf7]);
    let sut = sut_encode("shrdl", cl_ops("esi", "edi")).expect("SUT KAT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_double_shift_kat_llvm_mc_mem_dst() {
    // Contract: Intel SDM SHLD r/m32 forms; README lists shld/shrd as supported.
    let mc = llvm_mc_bytes("shldl $1, %eax, (%ebx)").expect("llvm-mc KAT mem");
    assert_eq!(mc, vec![0x0f, 0xa4, 0x03, 0x01]);
    let sut = sut_encode(
        "shldl",
        imm_mem_ops(1, "eax", mem_base("ebx", Displacement::None, None)),
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, mc, "SUT must encode memory destination SHLD"),
        Err(e) => panic!("SUT erred on valid mem-dst shldl: {e}"),
    }
}

#[test]
fn encode_double_shift_kat_llvm_mc_mem_seg() {
    let mc = llvm_mc_bytes("shldl $1, %eax, %fs:(%ebx)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0xa4, 0x03, 0x01]);
    let sut = sut_encode(
        "shldl",
        imm_mem_ops(1, "eax", mem_base("ebx", Displacement::None, Some("fs"))),
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT must emit FS override 0x64 before 0F A4 mem form"
        ),
        Err(e) => panic!("SUT erred on valid FS shldl mem: {e}"),
    }
}

/// Deterministic regression: memory destination rejected.
/// Witness: shldl $1, %eax, (%ebx) → Err("unsupported double shift operands")
#[test]
fn encode_double_shift_regression_mem_dst_rejected() {
    let mc = llvm_mc_bytes("shldl $1, %eax, (%ebx)").expect("llvm-mc");
    assert_eq!(mc, vec![0x0f, 0xa4, 0x03, 0x01]);
    let sut = sut_encode(
        "shldl",
        imm_mem_ops(1, "eax", mem_base("ebx", Displacement::None, None)),
    )
    .expect("valid mem-dst shldl must encode");
    assert_eq!(
        sut, mc,
        "regression: encode_double_shift must encode memory destination"
    );
}

/// Deterministic regression: non-GP xmm accepted via reg_num alias.
/// Witness: shldl $1, %xmm0, %edx → SUT emits [0F A4 C2 01] (xmm0→0 like eax)
#[test]
fn encode_double_shift_regression_xmm0_accepted() {
    assert!(
        llvm_mc_bytes("shldl $1, %xmm0, %edx").is_err(),
        "llvm-mc must reject shldl with xmm0"
    );
    let r = sut_encode("shldl", imm_ops(1, "xmm0", "edx"));
    assert!(
        r.is_err(),
        "encode_double_shift must reject non-GP xmm0 src, got {r:?}"
    );
}

/// Deterministic regression: width-mismatched r16 accepted.
/// Witness: shldl $1, %ax, %edx → SUT emits same as eax
#[test]
fn encode_double_shift_regression_mismatched_width() {
    assert!(
        llvm_mc_bytes("shldl $1, %ax, %edx").is_err(),
        "llvm-mc must reject shldl $1, %ax, %edx"
    );
    let r = sut_encode("shldl", imm_ops(1, "ax", "edx"));
    assert!(
        r.is_err(),
        "encode_double_shift must reject width-mismatched %ax src, got {r:?}"
    );
}

/// Deterministic regression: Imm count outside u8 truncated.
/// Witness: shldl $256, %eax, %edx → llvm-mc Err; SUT Ok([0F A4 C2 00])
#[test]
fn encode_double_shift_regression_imm_truncate() {
    assert!(
        llvm_mc_bytes("shldl $256, %eax, %edx").is_err(),
        "llvm-mc must reject Imm count 256"
    );
    let r = sut_encode("shldl", imm_ops(256, "eax", "edx"));
    assert!(
        r.is_err(),
        "encode_double_shift must reject Imm count 256 (Imm8 domain), got {r:?}"
    );
}

// --- Properties ---

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — Imm8 + r32 + r32
    #[test]
    fn encode_double_shift_diff_imm_rr(
        mnem in prop::sample::select(MNEMS),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
        count in 0u8..=255u8,
    ) {
        let asm = format!("{mnem} ${count}, %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, imm_ops(count as i64, src, dst))
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "imm-rr diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — CL + r32 + r32
    #[test]
    fn encode_double_shift_diff_cl_rr(
        mnem in prop::sample::select(MNEMS),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
    ) {
        let asm = format!("{mnem} %cl, %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc rejected `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, cl_ops(src, dst))
            .map_err(|e| TestCaseError::fail(format!("SUT rejected `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "cl-rr diff `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — Imm/CL + r32 + memory destination (Intel r/m32)
    #[test]
    fn encode_double_shift_diff_mem_dst(
        kind in 0u8..10,
        mnem in prop::sample::select(MNEMS32),
        src in prop::sample::select(GP32),
        base in prop::sample::select(GP32),
        index in prop::sample::select(
            GP32.iter().copied().filter(|r| *r != "esp").collect::<Vec<_>>()
        ),
        scale in prop::sample::select(SCALES),
        disp in prop_oneof![
            Just(0i64),
            Just(-1i64),
            Just(1i64),
            Just(127i64),
            Just(-128i64),
            Just(128i64),
            Just(0x1234i64),
            (-512i64..=512i64),
        ],
        use_cl in prop::bool::ANY,
        count in 0u8..=255u8,
        seg in prop::option::of(prop::sample::select(SREGS)),
    ) {
        let mem = match kind {
            0 => mem_base(base, Displacement::None, seg),
            1 => mem_base(base, Displacement::Integer(disp), seg),
            2 => mem_base("esp", Displacement::None, seg),
            3 => mem_base("ebp", Displacement::Integer(if disp == 0 { 1 } else { disp }), seg),
            4 => mem_base_index(Some(base), index, scale, Displacement::None, seg),
            5 => mem_base_index(Some(base), index, scale, Displacement::Integer(disp), seg),
            6 => mem_base_index(Some("esp"), index, scale, Displacement::Integer(disp), seg),
            7 => mem_abs(disp, seg),
            8 => mem_abs(0x1234_5678, seg),
            _ => mem_base("ebx", Displacement::Integer(4), seg),
        };
        let (asm, ops) = if use_cl {
            (
                format!("{mnem} %cl, %{src}, {}", att_mem(&mem)),
                cl_mem_ops(src, mem),
            )
        } else {
            (
                format!("{mnem} ${count}, %{src}, {}", att_mem(&mem)),
                imm_mem_ops(count as i64, src, mem),
            )
        };
        let mc = match llvm_mc_bytes(&asm) {
            Ok(b) => b,
            Err(_) => return Ok(()), // skip unencodable edge (e.g. abs+seg combos)
        };
        let sut = sut_encode(mnem, ops).map_err(|e| {
            TestCaseError::fail(format!(
                "SUT rejected valid mem-dst form `{asm}`: {e}; \
                 Intel SDM SHLD/SHRD r/m32 and assembler README list shld/shrd as supported"
            ))
        })?;
        prop_assert_eq!(
            &sut, &mc,
            "mem-dst diff `{}`: sut={:02x?} mc={:02x?}",
            asm, &sut, &mc
        );
    }

    // P4: algebraic.invariant — Imm/CL register-register opcode layout
    #[test]
    fn encode_double_shift_invariant_opcodes(
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
        count in 0u8..=255u8,
    ) {
        let sn = reg_num(src);
        let dn = reg_num(dst);
        let modrm = (3u8 << 6) | (sn << 3) | dn;

        let shld_imm = sut_encode("shldl", imm_ops(count as i64, src, dst))
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(
            &shld_imm,
            &vec![0x0F, 0xA4, modrm, count],
            "shldl imm %{} %{} c={}",
            src, dst, count
        );

        let shrd_imm = sut_encode("shrdl", imm_ops(count as i64, src, dst))
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(
            &shrd_imm,
            &vec![0x0F, 0xAC, modrm, count],
            "shrdl imm"
        );

        let shld_cl = sut_encode("shldl", cl_ops(src, dst))
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(&shld_cl, &vec![0x0F, 0xA5, modrm], "shldl cl");

        let shrd_cl = sut_encode("shrdl", cl_ops(src, dst))
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(&shrd_cl, &vec![0x0F, 0xAD, modrm], "shrdl cl");
    }

    // P5: algebraic.metamorphic — shld≡shldl, shrd≡shrdl
    #[test]
    fn encode_double_shift_meta_alias_mnemonic(
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
        count in 0u8..=255u8,
        use_cl in prop::bool::ANY,
    ) {
        let (ops_a, ops_b) = if use_cl {
            (cl_ops(src, dst), cl_ops(src, dst))
        } else {
            (imm_ops(count as i64, src, dst), imm_ops(count as i64, src, dst))
        };
        let shld = sut_encode("shld", ops_a.clone())
            .map_err(|e| TestCaseError::fail(e))?;
        let shldl = sut_encode("shldl", ops_b.clone())
            .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(&shld, &shldl, "shld ≡ shldl");

        let shrd = sut_encode("shrd", ops_a).map_err(|e| TestCaseError::fail(e))?;
        let shrdl = sut_encode("shrdl", ops_b).map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(&shrd, &shrdl, "shrd ≡ shrdl");

        // Also pin primary opcode byte
        prop_assert_eq!(shld.get(1).copied(), Some(if use_cl { 0xA5 } else { opc_imm("shld") }));
        prop_assert_eq!(shrd.get(1).copied(), Some(if use_cl { 0xAD } else { opc_imm("shrd") }));
    }

    // P6: negative_error — wrong arity
    #[test]
    fn encode_double_shift_neg_arity(n in 0usize..6) {
        prop_assume!(n != 3);
        let ops: Vec<Operand> = (0..n)
            .map(|i| match i {
                0 => Operand::Immediate(ImmediateValue::Integer(1)),
                1 => Operand::Register(Register::new("eax")),
                _ => Operand::Register(Register::new("edx")),
            })
            .collect();
        let r = sut_encode("shldl", ops);
        prop_assert!(r.is_err(), "arity {n} must be Err, got {r:?}");
    }

    // P7: negative_error — non-GP xmm / mm must be rejected as src or dst
    #[test]
    fn encode_double_shift_neg_non_gp(
        x in prop::sample::select(XMM),
        mnem in prop::sample::select(MNEMS32),
        as_src in prop::bool::ANY,
        use_cl in prop::bool::ANY,
    ) {
        let (src, dst) = if as_src {
            (x, "edx")
        } else {
            ("eax", x)
        };
        let asm = if use_cl {
            format!("{mnem} %cl, %{src}, %{dst}")
        } else {
            format!("{mnem} $1, %{src}, %{dst}")
        };
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc must reject `{asm}`"
        );
        let ops = if use_cl {
            cl_ops(src, dst)
        } else {
            imm_ops(1, src, dst)
        };
        let r = sut_encode(mnem, ops);
        prop_assert!(
            r.is_err(),
            "encode_double_shift must reject non-GP in `{asm}`, got {r:?}"
        );
    }

    // P8: negative_error — width-mismatched GP (r16/r8) as src or dst
    #[test]
    fn encode_double_shift_neg_mismatched_width(
        bad in prop::sample::select(
            GP16.iter().chain(GP8.iter()).copied().collect::<Vec<_>>()
        ),
        mnem in prop::sample::select(MNEMS32),
        as_src in prop::bool::ANY,
    ) {
        // cl as count is the only legal 8-bit role; still illegal as src/dst of shldl
        let (src, dst) = if as_src {
            (bad, "edx")
        } else {
            ("eax", bad)
        };
        let asm = format!("{mnem} $1, %{src}, %{dst}");
        prop_assert!(
            llvm_mc_bytes(&asm).is_err(),
            "llvm-mc must reject `{asm}`"
        );
        let r = sut_encode(mnem, imm_ops(1, src, dst));
        prop_assert!(
            r.is_err(),
            "encode_double_shift must reject width-mismatched `{asm}`, got {r:?}"
        );
    }

    // P9: differential/negative — Imm outside Imm8 domain must agree with llvm-mc
    // (llvm rejects → SUT must Err; no silent `as u8` truncation)
    #[test]
    fn encode_double_shift_neg_imm_out_of_u8(
        count in prop_oneof![
            Just(256i64),
            Just(257i64),
            Just(512i64),
            Just(0x1000i64),
            Just(0x1_0000i64),
            Just(-129i64),
            Just(-256i64),
            Just(-1000i64),
            (256i64..=1024i64),
            (-2048i64..=-129i64),
        ],
        mnem in prop::sample::select(MNEMS32),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
    ) {
        let asm = format!("{mnem} ${count}, %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm);
        let sut = sut_encode(mnem, imm_ops(count, src, dst));
        match (mc, sut) {
            (Err(_), Err(_)) => {} // both reject — correct
            (Ok(m), Ok(s)) => prop_assert_eq!(
                &s, &m,
                "imm edge both Ok must match for `{}`",
                asm
            ),
            (Err(e), Ok(s)) => {
                return Err(TestCaseError::fail(format!(
                    "Imm outside Imm8: llvm-mc rejects `{asm}` ({e}) but SUT encoded {:02x?} \
                     (silent truncate via `*count as u8`)",
                    s
                )));
            }
            (Ok(m), Err(e)) => {
                return Err(TestCaseError::fail(format!(
                    "Imm form `{asm}`: llvm-mc Ok={:02x?} but SUT Err={e}",
                    m
                )));
            }
        }
    }

    // P10: negative_error — first operand must be Imm or %cl (not other regs)
    #[test]
    fn encode_double_shift_neg_bad_count_reg(
        bad_count in prop::sample::select(
            GP32.iter()
                .copied()
                .chain(GP16.iter().copied())
                .filter(|r| *r != "cl")
                .collect::<Vec<_>>()
        ),
        src in prop::sample::select(GP32),
        dst in prop::sample::select(GP32),
        mnem in prop::sample::select(MNEMS32),
    ) {
        let ops = vec![
            Operand::Register(Register::new(bad_count)),
            Operand::Register(Register::new(src)),
            Operand::Register(Register::new(dst)),
        ];
        let r = sut_encode(mnem, ops);
        prop_assert!(
            r.is_err(),
            "count reg %{bad_count} must be Err for {mnem}, got {r:?}"
        );
    }
}
