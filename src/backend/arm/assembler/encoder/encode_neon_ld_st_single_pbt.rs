
// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   README.md:235 ld1/st1 (1-4 regs), ld2/st2, ld3/st3, ld4/st4 (with post-index);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:733-741 ld1-4/st1-4 => encode_neon_ld_st_dispatch => encode_neon_ld_st_single on RegListIndexed;
//   ARM AdvSIMD load/store single structure: Q 0011010 L R Rm opcode S size Rn Rt;
//   no-offset Rm=00000; imm post-index Rm=11111 bit23=1; register post-index Rm=Xm bit23=1;
//   gas aarch64-linux-gnu-as agrees with llvm-mc on KAT vectors.
// Stronger considered:
//   - State machine: rejected — encode_neon_ld_st_single is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree LD/ST single-element decoder
//   - Differential vs encode_neon_ld_st_multi: rejected — same-job gate (multiple-structures encoding)
//   - Differential vs encode_neon_ld1r / encode_neon_ldnr: rejected — replicate class, different mnemonic
// Weaker available: algebraic.metamorphic (Rt/Rn/L/R), algebraic.invariant (ARM fields), negative_error
// Differential: candidate=encode_neon_ld_st_single, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegListIndexed({Vt.T..}[idx]), Mem{Xn|SP}|MemPostIndex] <-> `ldN/stN {Vt.T..}[idx], [Xn|SP{], #imm}]`

use super::encode_neon_ld_st_single;
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

fn xreg(n: u32) -> String {
    if n == 31 {
        "sp".to_string()
    } else {
        format!("x{}", n)
    }
}

fn mnem(load: bool, n: u32) -> String {
    format!("{}{}", if load { "ld" } else { "st" }, n)
}

fn sz_and_idx() -> impl Strategy<Value = (&'static str, u32)> {
    prop_oneof![
        (Just("b"), 0u32..=15u32),
        (Just("h"), 0u32..=7u32),
        (Just("s"), 0u32..=3u32),
        (Just("d"), 0u32..=1u32),
    ]
}

fn n_structs() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![1u32, 2, 3, 4])
}

fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn esize(sz: &str) -> i64 {
    match sz {
        "b" => 1,
        "h" => 2,
        "s" => 4,
        "d" => 8,
        _ => 1,
    }
}

fn opcode(sz: &str, n: u32) -> u32 {
    let base = match sz {
        "b" => 0b000u32,
        "h" => 0b010,
        "s" | "d" => 0b100,
        _ => 0,
    };
    if n <= 2 {
        base
    } else {
        base | 1
    }
}

fn r_bit(n: u32) -> u32 {
    match n {
        1 | 3 => 0u32,
        _ => 1u32,
    }
}

fn q_s_size(sz: &str, idx: u32) -> (u32, u32, u32) {
    match sz {
        "b" => ((idx >> 3) & 1, (idx >> 2) & 1, idx & 3),
        "h" => ((idx >> 2) & 1, (idx >> 1) & 1, (idx & 1) << 1),
        "s" => ((idx >> 1) & 1, idx & 1, 0),
        "d" => (idx & 1, 0, 0b01),
        _ => (0, 0, 0),
    }
}

fn neon_arr(reg: u32, sz: &str) -> Operand {
    Operand::RegArrangement {
        reg: vreg(reg),
        arrangement: sz.to_string(),
    }
}

fn indexed_list(rt: u32, n: u32, sz: &str, idx: u32) -> Operand {
    let regs: Vec<Operand> = (0..n).map(|i| neon_arr((rt + i) % 32, sz)).collect();
    Operand::RegListIndexed {
        regs,
        index: idx,
    }
}

fn list_asm(rt: u32, n: u32, sz: &str, idx: u32) -> String {
    let parts: Vec<String> = (0..n)
        .map(|i| format!("{}.{}", vreg((rt + i) % 32), sz))
        .collect();
    format!("{{{}}}[{}]", parts.join(", "), idx)
}

fn mem0(rn: u32) -> Operand {
    Operand::Mem {
        base: xreg(rn),
        offset: 0,
    }
}

fn mem_post(rn: u32, imm: i64) -> Operand {
    Operand::MemPostIndex {
        base: xreg(rn),
        offset: imm,
    }
}

fn sut_word(ops: &[Operand], load: bool, n: u32) -> Result<u32, String> {
    match encode_neon_ld_st_single(ops, load, n)? {
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

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_neon_ld_st_single_kat_llvm_mc() {
    let want_st1 = 0x0d008060u32;
    let mc = llvm_mc_word("st1 {v0.s}[0], [x3]").expect("llvm-mc KAT st1");
    assert_eq!(mc, want_st1, "llvm-mc KAT mapping broken for st1");
    let sut = sut_word(&[indexed_list(0, 1, "s", 0), mem0(3)], false, 1).expect("SUT KAT st1");
    assert_eq!(sut, want_st1);

    let want_ld1b = 0x0d400020u32;
    let mc_b = llvm_mc_word("ld1 {v0.b}[0], [x1]").expect("llvm-mc KAT ld1.b");
    assert_eq!(mc_b, want_ld1b);
    let sut_b = sut_word(&[indexed_list(0, 1, "b", 0), mem0(1)], true, 1).expect("SUT KAT ld1.b");
    assert_eq!(sut_b, want_ld1b);

    let want_h = 0x0d404020u32;
    assert_eq!(llvm_mc_word("ld1 {v0.h}[0], [x1]").unwrap(), want_h);
    let want_s = 0x0d408020u32;
    assert_eq!(llvm_mc_word("ld1 {v0.s}[0], [x1]").unwrap(), want_s);
    let want_d = 0x0d408420u32;
    assert_eq!(llvm_mc_word("ld1 {v0.d}[0], [x1]").unwrap(), want_d);

    let want_b15 = 0x4d401c20u32;
    assert_eq!(llvm_mc_word("ld1 {v0.b}[15], [x1]").unwrap(), want_b15);
    let sut_b15 = sut_word(&[indexed_list(0, 1, "b", 15), mem0(1)], true, 1).expect("SUT KAT b15");
    assert_eq!(sut_b15, want_b15);

    let want_ld2 = 0x0d608060u32;
    assert_eq!(llvm_mc_word("ld2 {v0.s, v1.s}[0], [x3]").unwrap(), want_ld2);
    let sut_ld2 = sut_word(&[indexed_list(0, 2, "s", 0), mem0(3)], true, 2).expect("SUT KAT ld2");
    assert_eq!(sut_ld2, want_ld2);

    let want_ld3 = 0x0d40a060u32;
    assert_eq!(llvm_mc_word("ld3 {v0.s, v1.s, v2.s}[0], [x3]").unwrap(), want_ld3);
    let sut_ld3 = sut_word(&[indexed_list(0, 3, "s", 0), mem0(3)], true, 3).expect("SUT KAT ld3");
    assert_eq!(sut_ld3, want_ld3);

    let want_ld4 = 0x0d60a060u32;
    assert_eq!(llvm_mc_word("ld4 {v0.s, v1.s, v2.s, v3.s}[0], [x3]").unwrap(), want_ld4);
    let sut_ld4 = sut_word(&[indexed_list(0, 4, "s", 0), mem0(3)], true, 4).expect("SUT KAT ld4");
    assert_eq!(sut_ld4, want_ld4);

    let want_st2 = 0x0d208060u32;
    assert_eq!(llvm_mc_word("st2 {v0.s, v1.s}[0], [x3]").unwrap(), want_st2);

    let want_post = 0x0ddf8020u32;
    let mc_post = llvm_mc_word("ld1 {v0.s}[0], [x1], #4").expect("llvm-mc KAT post");
    assert_eq!(mc_post, want_post);
    let sut_post = sut_word(&[indexed_list(0, 1, "s", 0), mem_post(1, 4)], true, 1)
        .expect("SUT KAT post");
    assert_eq!(sut_post, want_post);

    let want_sp = 0x0d4083e0u32;
    assert_eq!(llvm_mc_word("ld1 {v0.s}[0], [sp]").unwrap(), want_sp);
    let sut_sp = sut_word(&[indexed_list(0, 1, "s", 0), mem0(31)], true, 1).expect("SUT KAT sp");
    assert_eq!(sut_sp, want_sp);

    let want_wrap = 0x4d4087dfu32;
    assert_eq!(llvm_mc_word("ld1 {v31.d}[1], [x30]").unwrap(), want_wrap);
    let sut_wrap = sut_word(&[indexed_list(31, 1, "d", 1), mem0(30)], true, 1).expect("SUT KAT wrap");
    assert_eq!(sut_wrap, want_wrap);

    let want_ld2wrap = 0x0d60803fu32;
    assert_eq!(llvm_mc_word("ld2 {v31.s, v0.s}[0], [x1]").unwrap(), want_ld2wrap);
    let sut_ld2wrap = sut_word(&[indexed_list(31, 2, "s", 0), mem0(1)], true, 2).expect("SUT KAT ld2 wrap");
    assert_eq!(sut_ld2wrap, want_ld2wrap);

    let want_regpost = 0x0dc28020u32;
    let mc_rp = llvm_mc_word("ld1 {v0.s}[0], [x1], x2").expect("llvm-mc KAT reg post");
    assert_eq!(mc_rp, want_regpost, "llvm-mc KAT mapping broken for register post-index");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_ld_st_single_diff_no_offset_llvm_mc(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        rn in reg_num(),
        load in any::<bool>(),
    ) {
        let asm = format!(
            "{} {}, [{}]",
            mnem(load, n),
            list_asm(rt, n, sz, idx),
            xreg(rn)
        );
        let ops = [indexed_list(rt, n, sz, idx), mem0(rn)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld_st_single_diff_post_imm_llvm_mc(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        rn in reg_num(),
        load in any::<bool>(),
    ) {
        let imm = n as i64 * esize(sz);
        let asm = format!(
            "{} {}, [{}], #{}",
            mnem(load, n),
            list_asm(rt, n, sz, idx),
            xreg(rn),
            imm
        );
        let ops = [indexed_list(rt, n, sz, idx), mem_post(rn, imm)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld_st_single_arm_fields(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        rn in reg_num(),
        load in any::<bool>(),
        post in any::<bool>(),
    ) {
        let ops = if post {
            vec![indexed_list(rt, n, sz, idx), mem_post(rn, n as i64 * esize(sz))]
        } else {
            vec![indexed_list(rt, n, sz, idx), mem0(rn)]
        };
        let w = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid ld/st single: {}", e));
        let (q, s, size) = q_s_size(sz, idx);
        let l = if load { 1u32 } else { 0u32 };
        let rm = if post { 0b11111u32 } else { 0u32 };
        let bit23 = if post { 1u32 } else { 0u32 };
        prop_assert_eq!(w >> 31, 0, "bit31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q, "Q bit");
        prop_assert_eq!((w >> 24) & 0b111111, 0b001101, "bits[29:24]=001101");
        prop_assert_eq!((w >> 23) & 1, bit23, "post-index bit23");
        prop_assert_eq!((w >> 22) & 1, l, "L bit");
        prop_assert_eq!((w >> 21) & 1, r_bit(n), "R bit");
        prop_assert_eq!((w >> 16) & 0b11111, rm, "Rm");
        prop_assert_eq!((w >> 13) & 0b111, opcode(sz, n), "opcode");
        prop_assert_eq!((w >> 12) & 1, s, "S bit");
        prop_assert_eq!((w >> 10) & 0b11, size, "size");
        prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
        prop_assert_eq!(w & 0b11111, rt, "Rt");
    }

    #[test]
    fn encode_neon_ld_st_single_metamorphic_rt_rn_l_r(
        (sz, idx) in sz_and_idx(),
        rt in 0u32..=30,
        rn in 0u32..=30,
    ) {
        let w = sut_word(&[indexed_list(rt, 1, sz, idx), mem0(rn)], true, 1)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let w_rt = sut_word(&[indexed_list(rt + 1, 1, sz, idx), mem0(rn)], true, 1)
            .unwrap_or_else(|e| panic!("SUT rejected rt+1: {}", e));
        let w_rn = sut_word(&[indexed_list(rt, 1, sz, idx), mem0(rn + 1)], true, 1)
            .unwrap_or_else(|e| panic!("SUT rejected rn+1: {}", e));
        let w_st = sut_word(&[indexed_list(rt, 1, sz, idx), mem0(rn)], false, 1)
            .unwrap_or_else(|e| panic!("SUT rejected store: {}", e));
        let w_n2 = sut_word(&[indexed_list(rt, 2, sz, idx), mem0(rn)], true, 2)
            .unwrap_or_else(|e| panic!("SUT rejected n=2: {}", e));
        prop_assert_eq!(w_rt & 0x1f, rt + 1, "Rt+1 must increment bits[4:0]");
        prop_assert_eq!(w_rt & !0x1f, w & !0x1f, "Rt+1 must not change other fields");
        prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 must increment bits[9:5]");
        prop_assert_eq!(w_rn & !(0x1f << 5), w & !(0x1f << 5), "Rn+1 must not change other fields");
        prop_assert_eq!(w_st ^ w, 1u32 << 22, "load vs store must flip only L");
        prop_assert_eq!(w_n2 ^ w, 1u32 << 21, "n=1 vs n=2 must flip only R");
    }

    #[test]
    fn encode_neon_ld_st_single_neg_arity_kinds(
        kind in 0u32..=5,
        n in n_structs(),
        load in any::<bool>(),
        rt in reg_num(),
        rn in 0u32..=30,
    ) {
        let ops: Vec<Operand> = match kind {
            0 => vec![],
            1 => vec![indexed_list(rt, n, "s", 0)],
            2 => vec![Operand::Reg(vreg(rt)), mem0(rn)],
            3 => vec![neon_arr(rt, "s"), mem0(rn)],
            4 => vec![mem0(rn), indexed_list(rt, n, "s", 0)],
            _ => vec![indexed_list(rt, n, "s", 0), Operand::Imm(0)],
        };
        prop_assert!(
            encode_neon_ld_st_single(&ops, load, n).is_err(),
            "ld/st{} kind={} must Err",
            n,
            kind
        );
    }

    #[test]
    fn encode_neon_ld_st_single_neg_count_size_names(
        n in n_structs(),
        wrong_len in 0u32..=5,
        t_idx in 0u32..=3,
        name_idx in 0u32..=4,
        rn in 0u32..=30,
    ) {
        let bad_sz = ["8b", "4s", "q", ""][t_idx as usize];
        let ops_sz = [indexed_list(0, n, bad_sz, 0), mem0(rn)];
        prop_assert!(
            encode_neon_ld_st_single(&ops_sz, true, n).is_err(),
            "ld{} unsupported size={} must Err",
            n,
            bad_sz
        );
        let len = if wrong_len == n {
            if n == 1 { 2 } else { n - 1 }
        } else {
            wrong_len
        };
        let regs: Vec<Operand> = (0..len).map(|i| neon_arr(i % 32, "s")).collect();
        let ops_cnt = [
            Operand::RegListIndexed {
                regs,
                index: 0,
            },
            mem0(rn),
        ];
        prop_assert!(
            encode_neon_ld_st_single(&ops_cnt, true, n).is_err(),
            "ld{} with {} regs must Err",
            n,
            len
        );
        let bad_name = ["foo", "v32", "x32", "r0", ""][name_idx as usize];
        let ops_name = [
            Operand::RegListIndexed {
                regs: vec![Operand::RegArrangement {
                    reg: bad_name.to_string(),
                    arrangement: "s".into(),
                }],
                index: 0,
            },
            mem0(rn),
        ];
        prop_assert!(
            encode_neon_ld_st_single(&ops_name, true, 1).is_err(),
            "ld1 invalid name {} must Err",
            bad_name
        );
    }

    #[test]
    fn encode_neon_ld_st_single_neg_extra(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        rn in 0u32..=30,
        load in any::<bool>(),
        extra_kind in 0u32..=3,
    ) {
        let extra = match extra_kind {
            0 => Operand::Cond("eq".into()),
            1 => Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            },
            2 => neon_arr(0, sz),
            _ => Operand::Label("1f".into()),
        };
        let ops = vec![indexed_list(rt, n, sz, idx), mem0(rn), extra];
        prop_assert!(
            encode_neon_ld_st_single(&ops, load, n).is_err(),
            "ld/st{} extra operand must Err (llvm-mc/gas reject a surplus operand)",
            n
        );
    }

    #[test]
    fn encode_neon_ld_st_single_neg_invalid_base(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        load in any::<bool>(),
        base in prop::sample::select(vec!["w0", "w31", "wsp", "xzr", "x31", "s0", "d0", "v0", "q0"]),
    ) {
        let ops = [
            indexed_list(rt, n, sz, idx),
            Operand::Mem {
                base: base.to_string(),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_neon_ld_st_single(&ops, load, n).is_err(),
            "ld/st{} [ {} ] must Err (llvm-mc rejects W/XZR/x31/FP base)",
            n,
            base
        );
    }

    #[test]
    fn encode_neon_ld_st_single_diff_alt_spellings(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in 0u32..=30,
        rn in 0u32..=30,
        load in any::<bool>(),
    ) {
        let parts: Vec<String> = (0..n)
            .map(|i| format!("V{}.{}", (rt + i) % 32, sz.to_uppercase()))
            .collect();
        let asm = format!(
            "{} {{{}}}[{}], [X{}]",
            mnem(load, n),
            parts.join(", "),
            idx,
            rn
        );
        let regs: Vec<Operand> = (0..n)
            .map(|i| Operand::RegArrangement {
                reg: format!("V{}", (rt + i) % 32),
                arrangement: sz.to_uppercase(),
            })
            .collect();
        let ops = [
            Operand::RegListIndexed {
                regs,
                index: idx,
            },
            Operand::Mem {
                base: format!("X{}", rn),
                offset: 0,
            },
        ];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld_st_single_neg_index_oor(
        n in n_structs(),
        sz in prop::sample::select(vec!["b", "h", "s", "d"]),
        rt in reg_num(),
        rn in 0u32..=30,
        load in any::<bool>(),
        over in 1u32..=8,
    ) {
        let max = match sz {
            "b" => 15u32,
            "h" => 7,
            "s" => 3,
            _ => 1,
        };
        let idx = max + over;
        let ops = [indexed_list(rt, n, sz, idx), mem0(rn)];
        prop_assert!(
            encode_neon_ld_st_single(&ops, load, n).is_err(),
            "ld/st{} {{v{}.{}}}[{}] must Err (llvm-mc lane range)",
            n,
            rt,
            sz,
            idx
        );
    }

    #[test]
    fn encode_neon_ld_st_single_neg_nonconsecutive(
        n in prop::sample::select(vec![2u32, 3, 4]),
        (sz, idx) in sz_and_idx(),
        rt in 0u32..=28,
        rn in 0u32..=30,
        load in any::<bool>(),
    ) {
        let mut regs: Vec<Operand> = (0..n).map(|i| neon_arr(rt + i, sz)).collect();
        // skip one register so the list is not consecutive
        if let Operand::RegArrangement { reg, .. } = &mut regs[1] {
            *reg = vreg((rt + 2) % 32);
        }
        let ops = [
            Operand::RegListIndexed {
                regs,
                index: idx,
            },
            mem0(rn),
        ];
        prop_assert!(
            encode_neon_ld_st_single(&ops, load, n).is_err(),
            "ld/st{} non-consecutive list must Err (ARM ISA requirement; neon.rs:921 TODO)",
            n
        );
    }

    #[test]
    fn encode_neon_ld_st_single_neg_reg_post(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        rn in 0u32..=30,
        rm in 0u32..=30,
        load in any::<bool>(),
    ) {
        prop_assume!(rm != 31);
        let ops = [
            indexed_list(rt, n, sz, idx),
            mem0(rn),
            Operand::Reg(format!("x{}", rm)),
        ];
        let asm = format!(
            "{} {}, [{}], {}",
            mnem(load, n),
            list_asm(rt, n, sz, idx),
            xreg(rn),
            format!("x{}", rm)
        );
        let mc = llvm_mc_word(&asm);
        prop_assert!(
            mc.is_ok(),
            "llvm-mc must accept register post-index {}",
            asm
        );
        let sut = sut_word(&ops, load, n);
        match (mc, sut) {
            (Ok(want), Ok(got)) => {
                prop_assert_eq!(got, want, "register post-index must match llvm-mc {}", asm);
            }
            (Ok(_), Err(_)) => {}
            (Err(e), _) => panic!("llvm-mc rejected {}: {}", asm, e),
        }
    }

    #[test]
    fn encode_neon_ld_st_single_neg_bad_post_imm(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        rn in 0u32..=30,
        load in any::<bool>(),
        bad in prop::sample::select(vec![0i64, 1, 3, 5, 7, 9, 64, -1]),
    ) {
        let legal = n as i64 * esize(sz);
        prop_assume!(bad != legal);
        let ops = [indexed_list(rt, n, sz, idx), mem_post(rn, bad)];
        prop_assert!(
            encode_neon_ld_st_single(&ops, load, n).is_err(),
            "ld/st{} post-index #{} (legal #{}) must Err",
            n,
            bad,
            legal
        );
    }

    #[test]
    fn encode_neon_ld_st_single_neg_mem_offset(
        n in n_structs(),
        (sz, idx) in sz_and_idx(),
        rt in reg_num(),
        rn in 0u32..=30,
        load in any::<bool>(),
        off in prop::sample::select(vec![1i64, 4, 8, -4, 16]),
    ) {
        let ops = [
            indexed_list(rt, n, sz, idx),
            Operand::Mem {
                base: xreg(rn),
                offset: off,
            },
        ];
        prop_assert!(
            encode_neon_ld_st_single(&ops, load, n).is_err(),
            "ld/st{} [x{}, #{}] must Err (not a valid single-structure addressing mode)",
            n,
            rn,
            off
        );
    }
}

/// Deterministic regression: extra operand ignored.
#[test]
fn test_encode_neon_ld_st_single_regression_extra_operand() {
    let ops = [
        indexed_list(0, 1, "b", 0),
        mem0(0),
        Operand::Cond("eq".into()),
    ];
    assert!(
        encode_neon_ld_st_single(&ops, false, 1).is_err(),
        "st1 {{v0.b}}[0], [x0], eq must Err"
    );
}

/// Deterministic regression: W-register base accepted.
#[test]
fn test_encode_neon_ld_st_single_regression_w_base() {
    let ops = [
        indexed_list(0, 1, "b", 0),
        Operand::Mem {
            base: "w0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_neon_ld_st_single(&ops, false, 1).is_err(),
        "st1 {{v0.b}}[0], [w0] must Err"
    );
}

/// Deterministic regression: uppercase arrangement rejected.
#[test]
fn test_encode_neon_ld_st_single_regression_alt_spellings() {
    let want = 0x0d000000u32;
    let ops = [
        Operand::RegListIndexed {
            regs: vec![Operand::RegArrangement {
                reg: "V0".into(),
                arrangement: "B".into(),
            }],
            index: 0,
        },
        Operand::Mem {
            base: "X0".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, false, 1).expect("st1 {V0.B}[0], [X0] must encode");
    assert_eq!(sut, want, "st1 {{V0.B}}[0], [X0] must be 0x0d000000");
}

/// Deterministic regression: out-of-range lane index accepted.
#[test]
fn test_encode_neon_ld_st_single_regression_index_oor() {
    let ops = [indexed_list(0, 1, "b", 16), mem0(0)];
    assert!(
        encode_neon_ld_st_single(&ops, false, 1).is_err(),
        "st1 {{v0.b}}[16], [x0] must Err"
    );
}

/// Deterministic regression: non-consecutive register list accepted.
#[test]
fn test_encode_neon_ld_st_single_regression_nonconsecutive() {
    let ops = [
        Operand::RegListIndexed {
            regs: vec![neon_arr(0, "b"), neon_arr(2, "b")],
            index: 0,
        },
        mem0(0),
    ];
    assert!(
        encode_neon_ld_st_single(&ops, false, 2).is_err(),
        "st2 {{v0.b, v2.b}}[0], [x0] must Err"
    );
}

/// Deterministic regression: register post-index encoded as no-offset.
#[test]
fn test_encode_neon_ld_st_single_regression_reg_post() {
    let want = 0x0dc28020u32;
    let ops = [
        indexed_list(0, 1, "s", 0),
        mem0(1),
        Operand::Reg("x2".into()),
    ];
    let sut = sut_word(&ops, true, 1).expect("SUT reg post");
    assert_eq!(
        sut, want,
        "ld1 {{v0.s}}[0], [x1], x2 must be 0x0dc28020"
    );
}

/// Deterministic regression: illegal post-index immediate accepted.
#[test]
fn test_encode_neon_ld_st_single_regression_bad_post_imm() {
    let ops = [indexed_list(0, 1, "b", 0), mem_post(0, 0)];
    assert!(
        encode_neon_ld_st_single(&ops, false, 1).is_err(),
        "st1 {{v0.b}}[0], [x0], #0 must Err"
    );
}
