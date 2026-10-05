// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md "accepts the same textual assembly that GCC's gas would consume";
//   README.md:235 ld1/st1 (1-4 regs), ld2/st2, ld3/st3, ld4/st4 (with post-index);
//   encoder/mod.rs:1-7 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:735-742 ld1-4/st1-4 => encode_neon_ld_st_dispatch => encode_neon_ld_st_multi on RegList;
//   ARM AdvSIMD load/store multiple structures: 0 Q 001100 L(bit22) post(bit23) Rm opcode size Rn Rt;
//   no-offset Rm=00000; imm post-index Rm=11111 bit23=1; register post-index Rm=Xm bit23=1;
//   gas aarch64-linux-gnu-as agrees with llvm-mc on KAT vectors.
// Stronger considered:
//   - State machine: rejected — encode_neon_ld_st_multi is a pure function with no lifecycle
//   - Algebraic round-trip: rejected — no in-tree LD/ST multiple-structures decoder
//   - Differential vs encode_neon_ld_st_single: rejected — same-job gate (single-element encoding)
//   - Differential vs encode_neon_ld1r / encode_neon_ldnr: rejected — replicate class, different mnemonic
// Weaker available: algebraic.metamorphic (Rt/Rn/L), algebraic.invariant (ARM fields), negative_error
// Differential: candidate=encode_neon_ld_st_multi, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=[RegList({Vt.T..}), Mem{Xn|SP}|MemPostIndex|(Mem,Reg|Imm)] <-> `ldN/stN {Vt.T..}, [Xn|SP{], #imm|Xm}]`

use super::encode_neon_ld_st_multi;
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

fn arr_ld1() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"])
}

fn arr_ldn() -> impl Strategy<Value = &'static str> {
    prop::sample::select(vec!["8b", "16b", "4h", "8h", "2s", "4s", "2d"])
}

/// (num_structs, n_regs, arrangement)
fn valid_triple() -> impl Strategy<Value = (u32, u32, &'static str)> {
    prop_oneof![
        (Just(1u32), prop::sample::select(vec![1u32, 2, 3, 4]), arr_ld1()),
        (Just(2u32), Just(2u32), arr_ldn()),
        (Just(3u32), Just(3u32), arr_ldn()),
        (Just(4u32), Just(4u32), arr_ldn()),
    ]
}

fn n_structs() -> impl Strategy<Value = u32> {
    prop::sample::select(vec![1u32, 2, 3, 4])
}

fn reg_num() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn q_size(t: &str) -> (u32, u32) {
    match t {
        "8b" => (0, 0b00),
        "16b" => (1, 0b00),
        "4h" => (0, 0b01),
        "8h" => (1, 0b01),
        "2s" => (0, 0b10),
        "4s" => (1, 0b10),
        "1d" => (0, 0b11),
        "2d" => (1, 0b11),
        _ => (0, 0),
    }
}

fn opcode(n: u32, n_regs: u32) -> u32 {
    match n {
        1 => match n_regs {
            1 => 0b0111,
            2 => 0b1010,
            3 => 0b0110,
            4 => 0b0010,
            _ => 0,
        },
        2 => 0b1000,
        3 => 0b0100,
        4 => 0b0000,
        _ => 0,
    }
}

fn legal_imm(n_regs: u32, t: &str) -> i64 {
    let (q, _) = q_size(t);
    n_regs as i64 * if q == 1 { 16 } else { 8 }
}

fn neon_arr(reg: u32, t: &str) -> Operand {
    Operand::RegArrangement {
        reg: vreg(reg),
        arrangement: t.to_string(),
    }
}

fn reg_list(rt: u32, n_regs: u32, t: &str) -> Operand {
    let regs: Vec<Operand> = (0..n_regs).map(|i| neon_arr((rt + i) % 32, t)).collect();
    Operand::RegList(regs)
}

fn list_asm(rt: u32, n_regs: u32, t: &str) -> String {
    let parts: Vec<String> = (0..n_regs)
        .map(|i| format!("{}.{}", vreg((rt + i) % 32), t))
        .collect();
    format!("{{{}}}", parts.join(", "))
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
    match encode_neon_ld_st_multi(ops, load, n)? {
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
fn encode_neon_ld_st_multi_kat_llvm_mc() {
    let want = 0x4c407000u32;
    let mc = llvm_mc_word("ld1 {v0.16b}, [x0]").expect("llvm-mc KAT ld1.16b");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken for ld1 {{v0.16b}}, [x0]");
    let sut = sut_word(&[reg_list(0, 1, "16b"), mem0(0)], true, 1).expect("SUT KAT ld1.16b");
    assert_eq!(sut, want);

    let want8 = 0x0c407020u32;
    assert_eq!(llvm_mc_word("ld1 {v0.8b}, [x1]").unwrap(), want8);
    let sut8 = sut_word(&[reg_list(0, 1, "8b"), mem0(1)], true, 1).expect("SUT KAT ld1.8b");
    assert_eq!(sut8, want8);

    let want_st = 0x4c007000u32;
    assert_eq!(llvm_mc_word("st1 {v0.16b}, [x0]").unwrap(), want_st);
    let sut_st = sut_word(&[reg_list(0, 1, "16b"), mem0(0)], false, 1).expect("SUT KAT st1");
    assert_eq!(sut_st, want_st);

    let want2 = 0x4c40a840u32;
    assert_eq!(llvm_mc_word("ld1 {v0.4s, v1.4s}, [x2]").unwrap(), want2);
    let sut2 = sut_word(&[reg_list(0, 2, "4s"), mem0(2)], true, 1).expect("SUT KAT ld1 2reg");
    assert_eq!(sut2, want2);

    let want3 = 0x4c406860u32;
    assert_eq!(
        llvm_mc_word("ld1 {v0.4s, v1.4s, v2.4s}, [x3]").unwrap(),
        want3
    );
    let sut3 = sut_word(&[reg_list(0, 3, "4s"), mem0(3)], true, 1).expect("SUT KAT ld1 3reg");
    assert_eq!(sut3, want3);

    let want4 = 0x4c402880u32;
    assert_eq!(
        llvm_mc_word("ld1 {v0.4s, v1.4s, v2.4s, v3.4s}, [x4]").unwrap(),
        want4
    );
    let sut4 = sut_word(&[reg_list(0, 4, "4s"), mem0(4)], true, 1).expect("SUT KAT ld1 4reg");
    assert_eq!(sut4, want4);

    let want_ld2 = 0x4c408020u32;
    assert_eq!(llvm_mc_word("ld2 {v0.16b, v1.16b}, [x1]").unwrap(), want_ld2);
    let sut_ld2 = sut_word(&[reg_list(0, 2, "16b"), mem0(1)], true, 2).expect("SUT KAT ld2");
    assert_eq!(sut_ld2, want_ld2);

    let want_ld3 = 0x4c404440u32;
    assert_eq!(
        llvm_mc_word("ld3 {v0.8h, v1.8h, v2.8h}, [x2]").unwrap(),
        want_ld3
    );
    let sut_ld3 = sut_word(&[reg_list(0, 3, "8h"), mem0(2)], true, 3).expect("SUT KAT ld3");
    assert_eq!(sut_ld3, want_ld3);

    let want_ld4 = 0x4c400860u32;
    assert_eq!(
        llvm_mc_word("ld4 {v0.4s, v1.4s, v2.4s, v3.4s}, [x3]").unwrap(),
        want_ld4
    );
    let sut_ld4 = sut_word(&[reg_list(0, 4, "4s"), mem0(3)], true, 4).expect("SUT KAT ld4");
    assert_eq!(sut_ld4, want_ld4);

    let want_st2 = 0x4c008020u32;
    assert_eq!(llvm_mc_word("st2 {v0.16b, v1.16b}, [x1]").unwrap(), want_st2);

    let want_post = 0x4cdf7020u32;
    let mc_post = llvm_mc_word("ld1 {v0.16b}, [x1], #16").expect("llvm-mc KAT post");
    assert_eq!(mc_post, want_post);
    let sut_post = sut_word(&[reg_list(0, 1, "16b"), mem_post(1, 16)], true, 1)
        .expect("SUT KAT post");
    assert_eq!(sut_post, want_post);

    let want_sp = 0x4c4073e0u32;
    assert_eq!(llvm_mc_word("ld1 {v0.16b}, [sp]").unwrap(), want_sp);
    let sut_sp = sut_word(&[reg_list(0, 1, "16b"), mem0(31)], true, 1).expect("SUT KAT sp");
    assert_eq!(sut_sp, want_sp);

    let want_wrap = 0x4c407fdfu32;
    assert_eq!(llvm_mc_word("ld1 {v31.2d}, [x30]").unwrap(), want_wrap);
    let sut_wrap = sut_word(&[reg_list(31, 1, "2d"), mem0(30)], true, 1).expect("SUT KAT wrap");
    assert_eq!(sut_wrap, want_wrap);

    let want_ld2wrap = 0x4c40803fu32;
    assert_eq!(
        llvm_mc_word("ld2 {v31.16b, v0.16b}, [x1]").unwrap(),
        want_ld2wrap
    );
    let sut_ld2wrap =
        sut_word(&[reg_list(31, 2, "16b"), mem0(1)], true, 2).expect("SUT KAT ld2 wrap");
    assert_eq!(sut_ld2wrap, want_ld2wrap);

    let want_1d = 0x0c407c00u32;
    assert_eq!(llvm_mc_word("ld1 {v0.1d}, [x0]").unwrap(), want_1d);
    let sut_1d = sut_word(&[reg_list(0, 1, "1d"), mem0(0)], true, 1).expect("SUT KAT 1d");
    assert_eq!(sut_1d, want_1d);

    let want_rp = 0x4cc27020u32;
    let mc_rp = llvm_mc_word("ld1 {v0.16b}, [x1], x2").expect("llvm-mc KAT reg post");
    assert_eq!(mc_rp, want_rp, "llvm-mc KAT mapping broken for register post-index");
}

proptest! {
    #![proptest_config(cfg())]

    #[test]
    fn encode_neon_ld_st_multi_diff_no_offset_llvm_mc(
        (n, n_regs, t) in valid_triple(),
        rt in reg_num(),
        rn in reg_num(),
        load in any::<bool>(),
    ) {
        let asm = format!(
            "{} {}, [{}]",
            mnem(load, n),
            list_asm(rt, n_regs, t),
            xreg(rn)
        );
        let ops = [reg_list(rt, n_regs, t), mem0(rn)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld_st_multi_diff_post_imm_llvm_mc(
        (n, n_regs, t) in valid_triple(),
        rt in reg_num(),
        rn in reg_num(),
        load in any::<bool>(),
    ) {
        let imm = legal_imm(n_regs, t);
        let asm = format!(
            "{} {}, [{}], #{}",
            mnem(load, n),
            list_asm(rt, n_regs, t),
            xreg(rn),
            imm
        );
        let ops = [reg_list(rt, n_regs, t), mem_post(rn, imm)];
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld_st_multi_arm_fields(
        (n, n_regs, t) in valid_triple(),
        rt in reg_num(),
        rn in reg_num(),
        load in any::<bool>(),
        post in any::<bool>(),
    ) {
        let ops = if post {
            vec![reg_list(rt, n_regs, t), mem_post(rn, legal_imm(n_regs, t))]
        } else {
            vec![reg_list(rt, n_regs, t), mem0(rn)]
        };
        let w = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid ld/st multi: {}", e));
        let (q, size) = q_size(t);
        let l = if load { 1u32 } else { 0u32 };
        let rm = if post { 0b11111u32 } else { 0u32 };
        let bit23 = if post { 1u32 } else { 0u32 };
        prop_assert_eq!(w >> 31, 0, "bit31 must be 0");
        prop_assert_eq!((w >> 30) & 1, q, "Q bit");
        prop_assert_eq!((w >> 24) & 0b111111, 0b001100, "bits[29:24]=001100");
        prop_assert_eq!((w >> 23) & 1, bit23, "post-index bit23");
        prop_assert_eq!((w >> 22) & 1, l, "L bit");
        prop_assert_eq!((w >> 21) & 1, 0, "bit21 must be 0");
        prop_assert_eq!((w >> 16) & 0b11111, rm, "Rm");
        prop_assert_eq!((w >> 12) & 0b1111, opcode(n, n_regs), "opcode");
        prop_assert_eq!((w >> 10) & 0b11, size, "size");
        prop_assert_eq!((w >> 5) & 0b11111, rn, "Rn");
        prop_assert_eq!(w & 0b11111, rt, "Rt");
    }

    #[test]
    fn encode_neon_ld_st_multi_metamorphic_rt_rn_l(
        t in arr_ld1(),
        rt in 0u32..=30,
        rn in 0u32..=30,
    ) {
        let w = sut_word(&[reg_list(rt, 1, t), mem0(rn)], true, 1)
            .unwrap_or_else(|e| panic!("SUT rejected: {}", e));
        let w_rt = sut_word(&[reg_list(rt + 1, 1, t), mem0(rn)], true, 1)
            .unwrap_or_else(|e| panic!("SUT rejected rt+1: {}", e));
        let w_rn = sut_word(&[reg_list(rt, 1, t), mem0(rn + 1)], true, 1)
            .unwrap_or_else(|e| panic!("SUT rejected rn+1: {}", e));
        let w_st = sut_word(&[reg_list(rt, 1, t), mem0(rn)], false, 1)
            .unwrap_or_else(|e| panic!("SUT rejected store: {}", e));
        prop_assert_eq!(w_rt & 0x1f, rt + 1, "Rt+1 must increment bits[4:0]");
        prop_assert_eq!(w_rt & !0x1f, w & !0x1f, "Rt+1 must not change other fields");
        prop_assert_eq!((w_rn >> 5) & 0x1f, rn + 1, "Rn+1 must increment bits[9:5]");
        prop_assert_eq!(w_rn & !(0x1f << 5), w & !(0x1f << 5), "Rn+1 must not change other fields");
        prop_assert_eq!(w_st ^ w, 1u32 << 22, "load vs store must flip only L");
    }

    #[test]
    fn encode_neon_ld_st_multi_neg_arity_kinds(
        kind in 0u32..=5,
        n in n_structs(),
        load in any::<bool>(),
        rt in reg_num(),
        rn in 0u32..=30,
    ) {
        let n_regs = n;
        let ops: Vec<Operand> = match kind {
            0 => vec![],
            1 => vec![reg_list(rt, n_regs, "16b")],
            2 => vec![Operand::Reg(vreg(rt)), mem0(rn)],
            3 => vec![neon_arr(rt, "16b"), mem0(rn)],
            4 => vec![mem0(rn), reg_list(rt, n_regs, "16b")],
            _ => vec![reg_list(rt, n_regs, "16b"), Operand::Imm(0)],
        };
        prop_assert!(
            encode_neon_ld_st_multi(&ops, load, n).is_err(),
            "ld/st{} kind={} must Err",
            n,
            kind
        );
    }

    #[test]
    fn encode_neon_ld_st_multi_neg_count_arr_names(
        n in n_structs(),
        wrong_len in 1u32..=5,
        t_idx in 0u32..=4,
        name_idx in 0u32..=4,
        rn in 0u32..=30,
    ) {
        let bad_t = ["b", "s", "q", "8B", ""][t_idx as usize];
        let ops_sz = [reg_list(0, n, bad_t), mem0(rn)];
        prop_assert!(
            encode_neon_ld_st_multi(&ops_sz, true, n).is_err(),
            "ld{} unsupported arrangement={} must Err",
            n,
            bad_t
        );
        let legal = n;
        let len = if wrong_len == legal {
            if n == 4 { 3 } else { n + 1 }
        } else if n == 1 && (1..=4).contains(&wrong_len) {
            5
        } else {
            wrong_len
        };
        let regs: Vec<Operand> = (0..len).map(|i| neon_arr(i % 32, "16b")).collect();
        let ops_cnt = [Operand::RegList(regs), mem0(rn)];
        prop_assert!(
            encode_neon_ld_st_multi(&ops_cnt, true, n).is_err(),
            "ld{} with {} regs must Err (llvm-mc rejects wrong vector count)",
            n,
            len
        );
        let bad_name = ["foo", "v32", "x32", "r0", ""][name_idx as usize];
        let ops_name = [
            Operand::RegList(vec![Operand::RegArrangement {
                reg: bad_name.to_string(),
                arrangement: "16b".into(),
            }]),
            mem0(rn),
        ];
        prop_assert!(
            encode_neon_ld_st_multi(&ops_name, true, 1).is_err(),
            "ld1 invalid name {} must Err",
            bad_name
        );
    }

    #[test]
    fn encode_neon_ld_st_multi_neg_extra(
        (n, n_regs, t) in valid_triple(),
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
            2 => neon_arr(0, t),
            _ => Operand::Label("1f".into()),
        };
        let ops = vec![reg_list(rt, n_regs, t), mem0(rn), extra];
        prop_assert!(
            encode_neon_ld_st_multi(&ops, load, n).is_err(),
            "ld/st{} extra operand must Err (llvm-mc/gas reject a surplus operand)",
            n
        );
    }

    #[test]
    fn encode_neon_ld_st_multi_neg_invalid_base(
        (n, n_regs, t) in valid_triple(),
        rt in reg_num(),
        load in any::<bool>(),
        base in prop::sample::select(vec!["w0", "w31", "wsp", "xzr", "x31", "s0", "d0", "v0", "q0"]),
    ) {
        let ops = [
            reg_list(rt, n_regs, t),
            Operand::Mem {
                base: base.to_string(),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_neon_ld_st_multi(&ops, load, n).is_err(),
            "ld/st{} [ {} ] must Err (llvm-mc rejects W/XZR/x31/FP base)",
            n,
            base
        );
    }

    #[test]
    fn encode_neon_ld_st_multi_diff_reg_post_llvm_mc(
        (n, n_regs, t) in valid_triple(),
        rt in reg_num(),
        rn in 0u32..=30,
        rm in 0u32..=30,
        load in any::<bool>(),
    ) {
        let ops = [
            reg_list(rt, n_regs, t),
            mem0(rn),
            Operand::Reg(format!("x{}", rm)),
        ];
        let asm = format!(
            "{} {}, [{}], x{}",
            mnem(load, n),
            list_asm(rt, n_regs, t),
            xreg(rn),
            rm
        );
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        let sut = sut_word(&ops, load, n)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "mismatch for {}", asm);
    }

    #[test]
    fn encode_neon_ld_st_multi_diff_alt_spellings(
        (n, n_regs, t) in valid_triple(),
        rt in 0u32..=30,
        rn in 0u32..=30,
        load in any::<bool>(),
    ) {
        let parts: Vec<String> = (0..n_regs)
            .map(|i| format!("V{}.{}", (rt + i) % 32, t.to_uppercase()))
            .collect();
        let asm = format!(
            "{} {}, [X{}]",
            mnem(load, n),
            format!("{{{}}}", parts.join(", ")),
            rn
        );
        let regs: Vec<Operand> = (0..n_regs)
            .map(|i| Operand::RegArrangement {
                reg: format!("V{}", (rt + i) % 32),
                arrangement: t.to_uppercase(),
            })
            .collect();
        let ops = [
            Operand::RegList(regs),
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
    fn encode_neon_ld_st_multi_neg_nonconsecutive(
        n in prop::sample::select(vec![2u32, 3, 4]),
        t in arr_ldn(),
        rt in 0u32..=28,
        rn in 0u32..=30,
        load in any::<bool>(),
    ) {
        let mut regs: Vec<Operand> = (0..n).map(|i| neon_arr(rt + i, t)).collect();
        if let Operand::RegArrangement { reg, .. } = &mut regs[1] {
            *reg = vreg((rt + 2) % 32);
        }
        let ops = [Operand::RegList(regs), mem0(rn)];
        prop_assert!(
            encode_neon_ld_st_multi(&ops, load, n).is_err(),
            "ld/st{} non-consecutive list must Err (ARM ISA sequential registers)",
            n
        );
    }

    #[test]
    fn encode_neon_ld_st_multi_neg_bad_post_imm(
        (n, n_regs, t) in valid_triple(),
        rt in reg_num(),
        rn in 0u32..=30,
        load in any::<bool>(),
        bad in prop::sample::select(vec![0i64, 1, 3, 5, 7, 9, 15, 17, 64, -1, 48]),
    ) {
        let legal = legal_imm(n_regs, t);
        prop_assume!(bad != legal);
        let ops = [reg_list(rt, n_regs, t), mem_post(rn, bad)];
        prop_assert!(
            encode_neon_ld_st_multi(&ops, load, n).is_err(),
            "ld/st{} post-index #{} (legal #{}) must Err",
            n,
            bad,
            legal
        );
    }

    #[test]
    fn encode_neon_ld_st_multi_neg_mem_offset(
        (n, n_regs, t) in valid_triple(),
        rt in reg_num(),
        rn in 0u32..=30,
        load in any::<bool>(),
        off in prop::sample::select(vec![1i64, 4, 8, -4, 16]),
    ) {
        let ops = [
            reg_list(rt, n_regs, t),
            Operand::Mem {
                base: xreg(rn),
                offset: off,
            },
        ];
        prop_assert!(
            encode_neon_ld_st_multi(&ops, load, n).is_err(),
            "ld/st{} [x{}, #{}] must Err (not a valid multiple-structure addressing mode)",
            n,
            rn,
            off
        );
    }

    #[test]
    fn encode_neon_ld_st_multi_neg_1d_ldn(
        n in prop::sample::select(vec![2u32, 3, 4]),
        rt in reg_num(),
        rn in 0u32..=30,
        load in any::<bool>(),
    ) {
        let ops = [reg_list(rt, n, "1d"), mem0(rn)];
        prop_assert!(
            encode_neon_ld_st_multi(&ops, load, n).is_err(),
            "ld/st{} {{v{}.1d..}} must Err (llvm-mc rejects .1d for LD2/3/4)",
            n,
            rt
        );
    }
}

/// Deterministic regression: extra operand ignored.
#[test]
fn test_encode_neon_ld_st_multi_regression_extra_operand() {
    let ops = [
        reg_list(0, 1, "8b"),
        mem0(0),
        Operand::Cond("eq".into()),
    ];
    assert!(
        encode_neon_ld_st_multi(&ops, false, 1).is_err(),
        "st1 {{v0.8b}}, [x0], eq must Err"
    );
}

/// Deterministic regression: W-register base accepted.
#[test]
fn test_encode_neon_ld_st_multi_regression_w_base() {
    let ops = [
        reg_list(0, 1, "8b"),
        Operand::Mem {
            base: "w0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_neon_ld_st_multi(&ops, false, 1).is_err(),
        "st1 {{v0.8b}}, [w0] must Err"
    );
}

/// Deterministic regression: LD2 with a 1-register list.
#[test]
fn test_encode_neon_ld_st_multi_regression_ld2_one_reg() {
    let ops = [reg_list(0, 1, "16b"), mem0(0)];
    assert!(
        encode_neon_ld_st_multi(&ops, true, 2).is_err(),
        "ld2 {{v0.16b}}, [x0] must Err"
    );
}

/// Deterministic regression: uppercase arrangement rejected while llvm-mc accepts it.
#[test]
fn test_encode_neon_ld_st_multi_regression_uppercase_arr() {
    let ops = [
        Operand::RegList(vec![Operand::RegArrangement {
            reg: "V0".into(),
            arrangement: "8B".into(),
        }]),
        Operand::Mem {
            base: "X0".into(),
            offset: 0,
        },
    ];
    let got = sut_word(&ops, true, 1);
    let want = llvm_mc_word("ld1 {V0.8B}, [X0]").expect("llvm-mc uppercase");
    assert_eq!(got, Ok(want), "uppercase arrangement must match llvm-mc");
}

/// Deterministic regression: non-consecutive register list.
#[test]
fn test_encode_neon_ld_st_multi_regression_nonconsecutive() {
    let ops = [
        Operand::RegList(vec![neon_arr(0, "8b"), neon_arr(2, "8b")]),
        mem0(0),
    ];
    assert!(
        encode_neon_ld_st_multi(&ops, false, 2).is_err(),
        "st2 {{v0.8b, v2.8b}}, [x0] must Err"
    );
}

/// Deterministic regression: illegal post-index immediate encoded.
#[test]
fn test_encode_neon_ld_st_multi_regression_bad_post_imm() {
    let ops = [reg_list(0, 1, "8b"), mem_post(0, 0)];
    assert!(
        encode_neon_ld_st_multi(&ops, false, 1).is_err(),
        "st1 {{v0.8b}}, [x0], #0 must Err"
    );
}

/// Deterministic regression: LD2 .1d is not a valid arrangement.
#[test]
fn test_encode_neon_ld_st_multi_regression_1d_ld2() {
    let ops = [reg_list(0, 2, "1d"), mem0(0)];
    assert!(
        encode_neon_ld_st_multi(&ops, false, 2).is_err(),
        "st2 {{v0.1d, v1.1d}}, [x0] must Err"
    );
}

