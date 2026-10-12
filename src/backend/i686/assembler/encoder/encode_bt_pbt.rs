// Oracle: differential — llvm-mc i686 assembler (BT/BTS/BTR/BTC)
// Evidence: gp_integer.rs:1012 encode_bt; mod.rs:274
//   "btl"|"btsl"|"btrl"|"btcl"|"bt"|"bts"|"btr"|"btc" => encode_bt;
//   Intel SDM Vol.2 BT/BTS/BTR/BTC — 0F A3/AB/B3/BB r/m,r and 0F BA /4..7 ib;
//   AT&T order: bit-offset, bit-base; memory base is architecturally valid;
//   segment override must precede opcode (core.rs:31-42 emit_segment_prefix);
//   x86-64 sibling x87_misc.rs encode_bt calls emit_segment_prefix on mem arms;
//   registers.rs reg_num aliases r16/r8 to the same 3-bit numbers.
// Stronger considered:
//   - State machine: rejected — pure encoding, no lifecycle
//   - Algebraic round-trip: rejected — no in-tree i686 decoder for bt family
// Differential: candidate=encode_bt (via InstructionEncoder::encode),
//   reference=llvm-mc -triple=i686 -show-encoding (LLVM 15),
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=AT&T operand bytes <-> llvm-mc encoding bytes.
// Also: algebraic.invariant (0F opc /r), algebraic.metamorphic (bare ≡ *l; seg‖bare),
//   negative_error (arity / wrong width / OOR imm / unsupported ops).

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
const R16: &[&str] = &["ax", "cx", "dx", "bx", "sp", "bp", "si", "di"];
const R8: &[&str] = &["al", "cl", "dl", "bl", "ah", "ch", "dh", "bh"];
const SREG: &[&str] = &["es", "cs", "ss", "ds", "fs", "gs"];
const SCALES: &[u8] = &[1, 2, 4, 8];
const MNEMS: &[&str] = &["btl", "btsl", "btrl", "btcl", "bt", "bts", "btr", "btc"];
const MNEMS_L: &[&str] = &["btl", "btsl", "btrl", "btcl"];
const BASES: &[&str] = &["bt", "bts", "btr", "btc"];

fn reg_num(name: &str) -> u8 {
    match name {
        "eax" | "ax" | "al" => 0,
        "ecx" | "cx" | "cl" => 1,
        "edx" | "dx" | "dl" => 2,
        "ebx" | "bx" | "bl" => 3,
        "esp" | "sp" | "ah" => 4,
        "ebp" | "bp" | "ch" => 5,
        "esi" | "si" | "dh" => 6,
        "edi" | "di" | "bh" => 7,
        _ => panic!("bad reg {name}"),
    }
}

fn opc_rr(mnemonic: &str) -> u8 {
    match mnemonic {
        "btl" | "bt" => 0xA3,
        "btsl" | "bts" => 0xAB,
        "btrl" | "btr" => 0xB3,
        "btcl" | "btc" => 0xBB,
        _ => panic!("bad mnem {mnemonic}"),
    }
}

fn imm_ext(mnemonic: &str) -> u8 {
    match mnemonic {
        "btl" | "bt" => 4,
        "btsl" | "bts" => 5,
        "btrl" | "btr" => 6,
        "btcl" | "btc" => 7,
        _ => panic!("bad mnem {mnemonic}"),
    }
}

fn modrm(mod_: u8, reg: u8, rm: u8) -> u8 {
    (mod_ << 6) | ((reg & 7) << 3) | (rm & 7)
}

fn seg_prefix_byte(seg: &str) -> u8 {
    match seg {
        "es" => 0x26,
        "cs" => 0x2E,
        "ss" => 0x36,
        "ds" => 0x3E,
        "fs" => 0x64,
        "gs" => 0x65,
        _ => panic!("bad seg {seg}"),
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
        // llvm-mc uses 'A' placeholders for fixup bytes — treat as 0
        if t == "A" {
            bytes.push(0);
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

fn gen_bare_mem(
    kind: u8,
    bi: usize,
    ii: usize,
    sc_i: usize,
    disp: i64,
) -> MemoryOperand {
    match kind % 4 {
        0 => mem_base(GP32[bi % 8], Displacement::Integer(disp), None),
        1 => {
            // avoid esp as index
            let idx = GP32[ii % 8];
            let idx = if idx == "esp" { "eax" } else { idx };
            mem_base_index(
                Some(GP32[bi % 8]),
                idx,
                SCALES[sc_i % 4],
                Displacement::Integer(disp),
                None,
            )
        }
        2 => {
            let idx = GP32[ii % 8];
            let idx = if idx == "esp" { "ecx" } else { idx };
            mem_base_index(
                None,
                idx,
                SCALES[sc_i % 4],
                Displacement::Integer(disp),
                None,
            )
        }
        _ => mem_abs(if disp == 0 { 0x1000 } else { disp }, None),
    }
}

// --- KAT gate (reference/differential prerequisite) ---

#[test]
fn encode_bt_kat_llvm_mc_btl_rr() {
    let mc = llvm_mc_bytes("btl %eax, %ebx").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xa3, 0xc3], "llvm-mc KAT mapping broken");
    let sut = sut_encode(
        "btl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Register(Register::new("ebx")),
        ],
    )
    .expect("SUT KAT");
    assert_eq!(sut, mc, "SUT KAT btl %eax, %ebx");
}

#[test]
fn encode_bt_kat_llvm_mc_btsl_imm_reg() {
    let mc = llvm_mc_bytes("btsl $5, %eax").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xba, 0xe8, 0x05]);
    let sut = sut_encode(
        "btsl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(5)),
            Operand::Register(Register::new("eax")),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bt_kat_llvm_mc_btrl_reg_mem() {
    let mc = llvm_mc_bytes("btrl %edx, (%ebp)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xb3, 0x55, 0x00]);
    let sut = sut_encode(
        "btrl",
        vec![
            Operand::Register(Register::new("edx")),
            Operand::Memory(mem_base("ebp", Displacement::None, None)),
        ],
    )
    .expect("SUT mem");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bt_kat_llvm_mc_btcl_imm_mem() {
    let mc = llvm_mc_bytes("btcl $3, (%ebx)").expect("llvm-mc KAT");
    assert_eq!(mc, vec![0x0f, 0xba, 0x3b, 0x03]);
    let sut = sut_encode(
        "btcl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(3)),
            Operand::Memory(mem_base("ebx", Displacement::None, None)),
        ],
    )
    .expect("SUT");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bt_kat_llvm_mc_bare_bt() {
    let mc = llvm_mc_bytes("bt %esi, %edi").expect("llvm-mc bare");
    assert_eq!(mc, vec![0x0f, 0xa3, 0xf7]);
    let sut = sut_encode(
        "bt",
        vec![
            Operand::Register(Register::new("esi")),
            Operand::Register(Register::new("edi")),
        ],
    )
    .expect("SUT bare");
    assert_eq!(sut, mc);
}

#[test]
fn encode_bt_kat_llvm_mc_segment_es() {
    // Segment override must precede opcode: [26, 0f, a3, 03]
    let mc = llvm_mc_bytes("btl %eax, %es:(%ebx)").expect("llvm-mc KAT es");
    assert_eq!(mc, vec![0x26, 0x0f, 0xa3, 0x03]);
    let sut = sut_encode(
        "btl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Memory(mem_base("ebx", Displacement::None, Some("es"))),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT KAT btl %eax, %es:(%ebx) must include 0x26"
        ),
        Err(e) => panic!("SUT rejected btl %eax, %es:(%ebx): {e}"),
    }
}

#[test]
fn encode_bt_kat_llvm_mc_imm_segment_fs() {
    let mc = llvm_mc_bytes("btsl $7, %fs:(%eax)").expect("llvm-mc KAT fs");
    assert_eq!(mc, vec![0x64, 0x0f, 0xba, 0x28, 0x07]);
    let sut = sut_encode(
        "btsl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(7)),
            Operand::Memory(mem_base("eax", Displacement::None, Some("fs"))),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(
            bytes, mc,
            "SUT KAT btsl $7, %fs:(%eax) must include 0x64"
        ),
        Err(e) => panic!("SUT rejected btsl $7, %fs:(%eax): {e}"),
    }
}

proptest! {
    #![proptest_config(cfg())]

    // P1: differential — GP32 RR pairs match llvm-mc
    #[test]
    fn encode_bt_diff_rr(
        mi in 0usize..8,
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let mnem = MNEMS[mi % MNEMS.len()];
        let src = GP32[si % 8];
        let dst = GP32[di % 8];
        let asm = format!("{mnem} %{src}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff RR `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P2: differential — Imm→Reg (imm8 domain accepted by llvm-mc)
    #[test]
    fn encode_bt_diff_imm_reg(
        mi in 0usize..4,
        di in 0usize..8,
        edge in 0u8..10,
        raw in -128i64..=255,
    ) {
        let mnem = MNEMS_L[mi % 4];
        let dst = GP32[di % 8];
        let imm = match edge {
            0 => 0i64,
            1 => 1,
            2 => 31,
            3 => 32,
            4 => 127,
            5 => 128,
            6 => 255,
            7 => -1,
            8 => -128,
            _ => raw,
        };
        let asm = format!("{mnem} ${imm}, %{dst}");
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(
            mnem,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff imm→reg `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P3: differential — Reg→Mem bare (no segment)
    #[test]
    fn encode_bt_diff_reg_mem_bare(
        mi in 0usize..4,
        si in 0usize..8,
        kind in 0u8..4,
        bi in 0usize..8,
        ii in 0usize..8,
        sc_i in 0usize..4,
        disp in prop::sample::select(vec![0i64, 1, -1, 4, 127, 128, -128, 0x1000]),
    ) {
        let mnem = MNEMS_L[mi % 4];
        let src = GP32[si % 8];
        let mem = gen_bare_mem(kind, bi, ii, sc_i, disp);
        let asm = format!("{mnem} %{src}, {}", att_mem(&mem));
        let ops = vec![
            Operand::Register(Register::new(src)),
            Operand::Memory(mem),
        ];
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff reg→mem `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P4: differential — Imm→Mem bare
    #[test]
    fn encode_bt_diff_imm_mem_bare(
        mi in 0usize..4,
        imm in 0u8..=255,
        kind in 0u8..4,
        bi in 0usize..8,
        ii in 0usize..8,
        sc_i in 0usize..4,
        disp in prop::sample::select(vec![0i64, 1, -1, 4, 127, 128, -128, 0x1000]),
    ) {
        let mnem = MNEMS_L[mi % 4];
        let imm = imm as i64;
        let mem = gen_bare_mem(kind, bi, ii, sc_i, disp);
        let asm = format!("{mnem} ${imm}, {}", att_mem(&mem));
        let ops = vec![
            Operand::Immediate(ImmediateValue::Integer(imm)),
            Operand::Memory(mem),
        ];
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        prop_assert_eq!(&sut, &mc, "diff imm→mem `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P5: differential — segment overrides on Reg→Mem and Imm→Mem
    #[test]
    fn encode_bt_diff_segment(
        mi in 0usize..4,
        form in 0u8..2,
        si in 0usize..6,
        bi in 0usize..8,
        ri in 0usize..8,
        disp in prop::sample::select(vec![0i64, 4, -8, 0x100]),
        imm in 0u8..=31,
    ) {
        let mnem = MNEMS_L[mi % 4];
        let seg = SREG[si % 6];
        let base = GP32[bi % 8];
        let reg = GP32[ri % 8];
        let mem = mem_base(base, Displacement::Integer(disp), Some(seg));
        let (asm, ops) = if form == 0 {
            let asm = format!("{mnem} %{reg}, {}", att_mem(&mem));
            let ops = vec![
                Operand::Register(Register::new(reg)),
                Operand::Memory(mem.clone()),
            ];
            (asm, ops)
        } else {
            let imm = imm as i64;
            let asm = format!("{mnem} ${imm}, {}", att_mem(&mem));
            let ops = vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Memory(mem.clone()),
            ];
            (asm, ops)
        };
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops).map_err(|e| {
            TestCaseError::fail(format!(
                "SUT rejected valid segment BT `{asm}`: {e}; \
                 i686 BT mem must emit segment override (core.rs emit_segment_prefix; \
                 x86-64 sibling encode_bt calls it)"
            ))
        })?;
        prop_assert_eq!(
            &sut, &mc,
            "segment diff `{}`: sut={:02x?} mc={:02x?}; expected prefix 0x{:02x}",
            asm, &sut, &mc, seg_prefix_byte(seg)
        );
    }

    // P6: metamorphic — bare mnemonic ≡ *l for r32 RR
    #[test]
    fn encode_bt_meta_bare_equiv_l(
        bi in 0usize..4,
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let base = BASES[bi % 4];
        let mnem_l = format!("{base}l");
        let src = GP32[si % 8];
        let dst = GP32[di % 8];
        let ops = vec![
            Operand::Register(Register::new(src)),
            Operand::Register(Register::new(dst)),
        ];
        let bare = sut_encode(base, ops.clone())
            .map_err(|e| TestCaseError::fail(format!("bare {base}: {e}")))?;
        let with_l = sut_encode(&mnem_l, ops)
            .map_err(|e| TestCaseError::fail(format!("{mnem_l}: {e}")))?;
        prop_assert_eq!(
            &bare, &with_l,
            "metamorphic bare≡*l {} vs {} %{},%{}: {:02x?} vs {:02x?}",
            base, mnem_l, src, dst, &bare, &with_l
        );
    }

    // P7: algebraic invariant — RR layout [0F, opc, ModRM(3,src,dst)]
    #[test]
    fn encode_bt_invariant_rr_layout(
        mi in 0usize..4,
        si in 0usize..8,
        di in 0usize..8,
    ) {
        let mnem = MNEMS_L[mi % 4];
        let src = GP32[si % 8];
        let dst = GP32[di % 8];
        let bytes = sut_encode(
            mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(bytes.len(), 3, "exact length 3 for RR");
        prop_assert_eq!(bytes[0], 0x0F, "escape 0F");
        prop_assert_eq!(bytes[1], opc_rr(mnem), "opc for {}", mnem);
        let m = bytes[2];
        prop_assert_eq!(m >> 6, 0b11, "mod must be 11");
        prop_assert_eq!((m >> 3) & 7, reg_num(src), "ModRM.reg = bit-offset src");
        prop_assert_eq!(m & 7, reg_num(dst), "ModRM.rm = bit-base dst");
        prop_assert_eq!(m, modrm(3, reg_num(src), reg_num(dst)));
    }

    // P8a: negative — wrong arity
    #[test]
    fn encode_bt_neg_arity(
        mi in 0usize..4,
        arity in 0u8..4,
        ri in 0usize..8,
    ) {
        prop_assume!(arity != 2);
        let mnem = MNEMS_L[mi % 4];
        let r = GP32[ri % 8];
        let ops: Vec<Operand> = (0..arity)
            .map(|_| Operand::Register(Register::new(r)))
            .collect();
        match sut_encode(mnem, ops) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted arity-{arity} `{mnem}` → {bytes:02x?}; \
                     encode_bt requires exactly 2 operands"
                )));
            }
        }
    }

    // P8b: negative — wrong GP width (r16/r8) that llvm-mc rejects for *l forms
    #[test]
    fn encode_bt_neg_wrong_width(
        mi in 0usize..4,
        mode in 0u8..4,
        a_i in 0usize..8,
        b_i in 0usize..8,
    ) {
        let mnem = MNEMS_L[mi % 4];
        let (src, dst) = match mode {
            0 => (R16[a_i % 8], GP32[b_i % 8]),
            1 => (GP32[a_i % 8], R16[b_i % 8]),
            2 => (R8[a_i % 8], GP32[b_i % 8]),
            _ => (GP32[a_i % 8], R8[b_i % 8]),
        };
        let asm = format!("{mnem} %{src}, %{dst}");
        // Confirm llvm-mc rejects
        if llvm_mc_bytes(&asm).is_ok() {
            // rare acceptance — skip
            return Ok(());
        }
        match sut_encode(
            mnem,
            vec![
                Operand::Register(Register::new(src)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted wrong-width `{asm}` → {bytes:02x?}; \
                     llvm-mc rejects; Intel BT *l is r32 only (reg_num aliasing bug)"
                )));
            }
        }
    }

    // P8c: negative — imm outside imm8 domain that llvm-mc rejects
    #[test]
    fn encode_bt_neg_imm_oor(
        mi in 0usize..4,
        di in 0usize..8,
        edge in 0u8..6,
        raw in prop::num::i64::ANY,
    ) {
        let mnem = MNEMS_L[mi % 4];
        let dst = GP32[di % 8];
        let imm = match edge {
            0 => 256i64,
            1 => 257,
            2 => 0x10000,
            3 => -129,
            4 => i64::MAX,
            _ => {
                // force outside signed-imm8-or-u8 range llvm-mc accepts
                if (-128..=255).contains(&raw) {
                    256 + (raw.rem_euclid(1000))
                } else {
                    raw
                }
            }
        };
        prop_assume!(!(-128..=255).contains(&imm));
        let asm = format!("{mnem} ${imm}, %{dst}");
        if llvm_mc_bytes(&asm).is_ok() {
            return Ok(());
        }
        match sut_encode(
            mnem,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Register(Register::new(dst)),
            ],
        ) {
            Err(_) => {}
            Ok(bytes) => {
                return Err(TestCaseError::fail(format!(
                    "SUT accepted OOR imm `{asm}` → {bytes:02x?}; \
                     llvm-mc rejects; `*val as u8` truncates (imm8 contract broken)"
                )));
            }
        }
    }

    // P9: metamorphic — segmented mem = seg_prefix ‖ bare mem (reg→mem)
    #[test]
    fn encode_bt_meta_segment_prefix(
        mi in 0usize..4,
        si in 0usize..6,
        bi in 0usize..8,
        ri in 0usize..8,
        form in 0u8..2,
        imm in 0u8..=31,
    ) {
        let mnem = MNEMS_L[mi % 4];
        let seg = SREG[si % 6];
        let base = GP32[bi % 8];
        let reg = GP32[ri % 8];
        let bare = mem_base(base, Displacement::None, None);
        let segd = mem_base(base, Displacement::None, Some(seg));

        let (ops_bare, ops_seg) = if form == 0 {
            (
                vec![
                    Operand::Register(Register::new(reg)),
                    Operand::Memory(bare),
                ],
                vec![
                    Operand::Register(Register::new(reg)),
                    Operand::Memory(segd),
                ],
            )
        } else {
            let imm = imm as i64;
            (
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(bare),
                ],
                vec![
                    Operand::Immediate(ImmediateValue::Integer(imm)),
                    Operand::Memory(segd),
                ],
            )
        };

        let bare_bytes = sut_encode(mnem, ops_bare)
            .map_err(|e| TestCaseError::fail(format!("bare encode: {e}")))?;
        let seg_bytes = sut_encode(mnem, ops_seg).map_err(|e| {
            TestCaseError::fail(format!(
                "segmented encode must succeed (emit_segment_prefix contract): {e}"
            ))
        })?;
        let mut expect = vec![seg_prefix_byte(seg)];
        expect.extend_from_slice(&bare_bytes);
        prop_assert_eq!(
            &seg_bytes,
            &expect,
            "metamorphic seg||bare for {} %{}:(%{}): got {:02x?} expect {:02x?}",
            mnem,
            seg,
            base,
            &seg_bytes,
            &expect
        );
    }

    // P10: differential — Imm→Label and Reg→Label absolute forms
    #[test]
    fn encode_bt_diff_label(
        mi in 0usize..4,
        form in 0u8..2,
        ri in 0usize..8,
        imm in 0u8..=31,
        label in prop::sample::select(vec!["foo", "bar", "L1", "_sym"]),
    ) {
        let mnem = MNEMS_L[mi % 4];
        let (asm, ops) = if form == 0 {
            let imm = imm as i64;
            let asm = format!("{mnem} ${imm}, {label}");
            let ops = vec![
                Operand::Immediate(ImmediateValue::Integer(imm)),
                Operand::Label(label.to_string()),
            ];
            (asm, ops)
        } else {
            let reg = GP32[ri % 8];
            let asm = format!("{mnem} %{reg}, {label}");
            let ops = vec![
                Operand::Register(Register::new(reg)),
                Operand::Label(label.to_string()),
            ];
            (asm, ops)
        };
        let mc = llvm_mc_bytes(&asm)
            .map_err(|e| TestCaseError::fail(format!("llvm-mc failed on valid `{asm}`: {e}")))?;
        let sut = sut_encode(mnem, ops)
            .map_err(|e| TestCaseError::fail(format!("SUT Err on valid `{asm}`: {e}")))?;
        // Fixup bytes are zeros on both sides after parse_llvm_bytes maps A→0
        prop_assert_eq!(&sut, &mc, "diff label `{}`: sut={:02x?} mc={:02x?}", asm, &sut, &mc);
    }

    // P11: invariant — Imm→Reg layout [0F, BA, ModRM(3,ext,dst), imm8]
    #[test]
    fn encode_bt_invariant_imm_reg_layout(
        mi in 0usize..4,
        di in 0usize..8,
        imm in 0u8..=255,
    ) {
        let mnem = MNEMS_L[mi % 4];
        let dst = GP32[di % 8];
        let imm_i = imm as i64;
        let bytes = sut_encode(
            mnem,
            vec![
                Operand::Immediate(ImmediateValue::Integer(imm_i)),
                Operand::Register(Register::new(dst)),
            ],
        )
        .map_err(|e| TestCaseError::fail(e))?;
        prop_assert_eq!(bytes.len(), 4, "exact length 4 for imm→reg");
        prop_assert_eq!(bytes[0], 0x0F);
        prop_assert_eq!(bytes[1], 0xBA);
        let m = bytes[2];
        prop_assert_eq!(m >> 6, 0b11);
        prop_assert_eq!((m >> 3) & 7, imm_ext(mnem), "ModRM.reg = ext for {}", mnem);
        prop_assert_eq!(m & 7, reg_num(dst));
        prop_assert_eq!(bytes[3], imm);
    }
}

// --- Deterministic regression witnesses (filled after PBT shrinks) ---

#[test]
fn encode_bt_regression_segment_es_reg_mem() {
    // Witness: btl %eax, %es:(%ebx) must emit 0x26 prefix
    let expect = vec![0x26, 0x0f, 0xa3, 0x03];
    let sut = sut_encode(
        "btl",
        vec![
            Operand::Register(Register::new("eax")),
            Operand::Memory(mem_base("ebx", Displacement::None, Some("es"))),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, expect, "segment ES prefix missing on BT reg→mem"),
        Err(e) => panic!("SUT rejected valid btl %eax, %es:(%ebx): {e}"),
    }
}

#[test]
fn encode_bt_regression_segment_fs_imm_mem() {
    let expect = vec![0x64, 0x0f, 0xba, 0x28, 0x07];
    let sut = sut_encode(
        "btsl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(7)),
            Operand::Memory(mem_base("eax", Displacement::None, Some("fs"))),
        ],
    );
    match sut {
        Ok(bytes) => assert_eq!(bytes, expect, "segment FS prefix missing on BTS imm→mem"),
        Err(e) => panic!("SUT rejected valid btsl $7, %fs:(%eax): {e}"),
    }
}

#[test]
fn encode_bt_regression_wrong_width_ax_ebx() {
    // btl %ax, %ebx — llvm-mc rejects; SUT must not alias to eax
    let r = sut_encode(
        "btl",
        vec![
            Operand::Register(Register::new("ax")),
            Operand::Register(Register::new("ebx")),
        ],
    );
    assert!(
        r.is_err(),
        "SUT accepted btl %ax, %ebx → {r:?}; r16 not valid for btl"
    );
}

#[test]
fn encode_bt_regression_imm_oor_256() {
    let r = sut_encode(
        "btl",
        vec![
            Operand::Immediate(ImmediateValue::Integer(256)),
            Operand::Register(Register::new("eax")),
        ],
    );
    assert!(
        r.is_err(),
        "SUT accepted btl $256, %eax → {r:?}; imm must fit imm8"
    );
}
