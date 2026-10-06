// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md:217 Loads/Stores table lists ldr, str, ldrb, strb, ldrh, strh;
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:473-478 "ldr"/"str" via encode_ldr_str_auto, "ldrb"/"strb"/"ldrh"/"strh" => encode_ldr_str;
//   ARM ARM LDR/STR unsigned: size 111 V 01 opc imm12 Rn Rt (pimm = imm12*(1<<size));
//   unscaled LDUR/STUR: bits[25:24]=00 bits[11:10]=00 simm9 [-256,255];
//   pre bits[11:10]=11, post bits[11:10]=01; register: bit21=1 option S bits[11:10]=10.
//   Rt is Wt (size 00/01/10) or Xt (size 11); 31=WZR/XZR, never SP. Rn is Xn|SP (not XZR).
// Stronger considered:
//   - State machine: rejected — encode_ldr_str is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no LDR/STR decoder
//   - encode_ldur_stur / encode_ldrsw / encode_ldrs as differential siblings: rejected —
//     same-job gate (LDUR mnemonic, LDRSW signed-word, LDRSB/LDRSH; shared get_reg / same crate)
// Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (Rt/Rn/imm12, load/store, pre/post),
//   negative_error (arity / extra / SP-dest / XZR-base / W-base / W-index / writeback-Rt==Rn / offset range)
// Differential: candidate=encode_ldr_str, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=(Reg(Rt), Mem/Pre/Post/RegOffset, is_load, size) <-> `ldr/str/ldrb/strb/ldrh/strh Rt, <addr>`

use super::encode_ldr_str;
use super::EncodeResult;
use super::RelocType;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;
const SIMM_MIN: i64 = -256;
const SIMM_MAX: i64 = 255;

fn cfg() -> ProptestConfig {
    ProptestConfig::with_cases(CASES)
}

fn xt_name(n: u32) -> String {
    if n == 31 {
        "xzr".into()
    } else {
        format!("x{n}")
    }
}

fn wt_name(n: u32) -> String {
    if n == 31 {
        "wzr".into()
    } else {
        format!("w{n}")
    }
}

fn rn_name(n: u32) -> String {
    if n == 31 {
        "sp".into()
    } else {
        format!("x{n}")
    }
}

fn rm_x(n: u32) -> String {
    if n == 31 {
        "xzr".into()
    } else {
        format!("x{n}")
    }
}

fn rm_w(n: u32) -> String {
    if n == 31 {
        "wzr".into()
    } else {
        format!("w{n}")
    }
}

fn gp_rt(size: u32, n: u32) -> String {
    if size == 0b11 {
        xt_name(n)
    } else {
        wt_name(n)
    }
}

fn gp_mnemonic(is_load: bool, size: u32) -> &'static str {
    match (is_load, size) {
        (true, 0b00) => "ldrb",
        (false, 0b00) => "strb",
        (true, 0b01) => "ldrh",
        (false, 0b01) => "strh",
        (true, _) => "ldr",
        (false, _) => "str",
    }
}

fn scale_of(size: u32) -> i64 {
    1i64 << size
}

fn pimm_max(size: u32) -> i64 {
    4095 * scale_of(size)
}

fn asm_mem(rn: &str, offset: i64) -> String {
    if offset == 0 {
        format!("[{rn}]")
    } else {
        format!("[{rn}, #{offset}]")
    }
}

fn sut_word(
    ops: &[Operand],
    is_load: bool,
    size: u32,
    is_128bit: bool,
) -> Result<u32, String> {
    match encode_ldr_str(ops, is_load, size, false, is_128bit)? {
        EncodeResult::Word(w) => Ok(w),
        other => Err(format!("expected Word, got {other:?}")),
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

fn unpack_unsigned(word: u32) -> (u32, u32, u32, u32, u32, u32, u32, u32) {
    let size = (word >> 30) & 0b11;
    let bits29_27 = (word >> 27) & 0b111;
    let v = (word >> 26) & 1;
    let bits25_24 = (word >> 24) & 0b11;
    let opc = (word >> 22) & 0b11;
    let imm12 = (word >> 10) & 0xfff;
    let rn = (word >> 5) & 0x1f;
    let rt = word & 0x1f;
    (size, bits29_27, v, bits25_24, opc, imm12, rn, rt)
}

fn reg_edge() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn size_edge() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(2u32), Just(3u32), 0u32..=3]
}

fn imm12_edge() -> impl Strategy<Value = u32> {
    prop_oneof![
        Just(0u32),
        Just(1u32),
        Just(4094u32),
        Just(4095u32),
        0u32..=4095
    ]
}

fn simm9_in_range() -> impl Strategy<Value = i64> {
    prop_oneof![
        Just(SIMM_MIN),
        Just(SIMM_MIN + 1),
        Just(-1i64),
        Just(0i64),
        Just(1i64),
        Just(SIMM_MAX - 1),
        Just(SIMM_MAX),
        SIMM_MIN..=SIMM_MAX,
    ]
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
        Just(Operand::Label("L0".into())),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Cond("eq".into())),
        Just(Operand::RegArrangement {
            reg: "v0".into(),
            arrangement: "8b".into(),
        }),
    ]
}

fn bad_address_kind() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Imm(8)),
        Just(Operand::Reg("x1".into())),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 3,
        }),
        Just(Operand::Cond("eq".into())),
        Just(Operand::Label("L0".into())),
        Just(Operand::Extend {
            kind: "uxtw".into(),
            amount: 0,
        }),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_ldr_str_kat_llvm_mc_x0_x1() {
    let want = 0xF940_0020u32;
    let mc = llvm_mc_word("ldr x0, [x1]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, true, 0b11, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldr_str_kat_llvm_mc_x0_x1_imm8() {
    let want = 0xF940_0420u32;
    let mc = llvm_mc_word("ldr x0, [x1, #8]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 8,
        },
    ];
    let sut = sut_word(&ops, true, 0b11, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldr_str_kat_llvm_mc_str_w2() {
    let want = 0xB900_0462u32;
    let mc = llvm_mc_word("str w2, [x3, #4]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("w2".into()),
        Operand::Mem {
            base: "x3".into(),
            offset: 4,
        },
    ];
    let sut = sut_word(&ops, false, 0b10, false).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldr_str_kat_llvm_mc_ldrb_ldrh() {
    let want_b = 0x3940_0420u32;
    let mc_b = llvm_mc_word("ldrb w0, [x1, #1]").expect("llvm-mc LDRB KAT");
    assert_eq!(mc_b, want_b, "llvm-mc LDRB KAT mapping broken");
    let ops_b = [
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 1,
        },
    ];
    let sut_b = sut_word(&ops_b, true, 0b00, false).expect("SUT LDRB");
    assert_eq!(sut_b, want_b);

    let want_h = 0x7940_0420u32;
    let mc_h = llvm_mc_word("ldrh w0, [x1, #2]").expect("llvm-mc LDRH KAT");
    assert_eq!(mc_h, want_h, "llvm-mc LDRH KAT mapping broken");
    let ops_h = [
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 2,
        },
    ];
    let sut_h = sut_word(&ops_h, true, 0b01, false).expect("SUT LDRH");
    assert_eq!(sut_h, want_h);
}

#[test]
fn encode_ldr_str_kat_llvm_mc_unscaled_pre_post_regoff() {
    let want_u = 0xF840_4020u32;
    let mc_u = llvm_mc_word("ldr x0, [x1, #4]").expect("llvm-mc LDUR-alias KAT");
    assert_eq!(mc_u, want_u, "llvm-mc unscaled KAT mapping broken");
    let ops_u = [
        Operand::Reg("x0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 4,
        },
    ];
    assert_eq!(sut_word(&ops_u, true, 0b11, false).expect("SUT unscaled"), want_u);

    let want_post = 0xF840_8420u32;
    let mc_post = llvm_mc_word("ldr x0, [x1], #8").expect("llvm-mc post KAT");
    assert_eq!(mc_post, want_post);
    let ops_post = [
        Operand::Reg("x0".into()),
        Operand::MemPostIndex {
            base: "x1".into(),
            offset: 8,
        },
    ];
    assert_eq!(
        sut_word(&ops_post, true, 0b11, false).expect("SUT post"),
        want_post
    );

    let want_pre = 0xF840_8C20u32;
    let mc_pre = llvm_mc_word("ldr x0, [x1, #8]!").expect("llvm-mc pre KAT");
    assert_eq!(mc_pre, want_pre);
    let ops_pre = [
        Operand::Reg("x0".into()),
        Operand::MemPreIndex {
            base: "x1".into(),
            offset: 8,
        },
    ];
    assert_eq!(
        sut_word(&ops_pre, true, 0b11, false).expect("SUT pre"),
        want_pre
    );

    let want_ro = 0xF862_6820u32;
    let mc_ro = llvm_mc_word("ldr x0, [x1, x2]").expect("llvm-mc regoff KAT");
    assert_eq!(mc_ro, want_ro);
    let ops_ro = [
        Operand::Reg("x0".into()),
        Operand::MemRegOffset {
            base: "x1".into(),
            index: "x2".into(),
            extend: None,
            shift: None,
        },
    ];
    assert_eq!(
        sut_word(&ops_ro, true, 0b11, false).expect("SUT regoff"),
        want_ro
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — llvm-mc unsigned-offset LDR/STR/LDRB/STRB/LDRH/STRH
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_diff_unsigned_llvm_mc(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        imm12 in imm12_edge(),
    ) {
        let dest = gp_rt(size, rt);
        let base = rn_name(rn);
        let offset = (imm12 as i64) * scale_of(size);
        let mnemonic = gp_mnemonic(is_load, size);
        let asm = format!("{mnemonic} {dest}, {}", asm_mem(&base, offset));
        let ops = [
            Operand::Reg(dest),
            Operand::Mem { base, offset },
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load, size, false).expect("SUT");
        prop_assert_eq!(sut, mc, "unsigned mismatch for {}", asm);
    }

    // Oracle: differential — llvm-mc unscaled / pre-index / post-index
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_diff_unscaled_pre_post_llvm_mc(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        simm in simm9_in_range(),
        form in 0u32..=2,
    ) {
        // ARM UNPREDICTABLE: writeback with Rt == Rn (and Rn != SP).
        // Valid-domain generator excludes it; negative_error covers it.
        prop_assume!(form == 0 || rt != rn || rn == 31);
        let dest = gp_rt(size, rt);
        let base = rn_name(rn);
        let mnemonic = gp_mnemonic(is_load, size);
        let (ops, asm) = match form {
            0 => {
                let ops = [
                    Operand::Reg(dest.clone()),
                    Operand::Mem {
                        base: base.clone(),
                        offset: simm,
                    },
                ];
                let asm = format!("{mnemonic} {dest}, {}", asm_mem(&base, simm));
                (ops, asm)
            }
            1 => {
                let ops = [
                    Operand::Reg(dest.clone()),
                    Operand::MemPreIndex {
                        base: base.clone(),
                        offset: simm,
                    },
                ];
                let asm = format!("{mnemonic} {dest}, [{base}, #{simm}]!");
                (ops, asm)
            }
            _ => {
                let ops = [
                    Operand::Reg(dest.clone()),
                    Operand::MemPostIndex {
                        base: base.clone(),
                        offset: simm,
                    },
                ];
                let asm = format!("{mnemonic} {dest}, [{base}], #{simm}");
                (ops, asm)
            }
        };
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load, size, false).expect("SUT");
        prop_assert_eq!(sut, mc, "unscaled/pre/post mismatch for {}", asm);
    }

    // Oracle: differential — llvm-mc register-offset
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_diff_regoff_llvm_mc(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        rm in reg_edge(),
        w_index in any::<bool>(),
        ext_sel in 0u32..=1,
        s_bit in 0u32..=1,
    ) {
        let dest = gp_rt(size, rt);
        let base = rn_name(rn);
        let (index, extend, shift, asm_ext) = if w_index {
            let index = rm_w(rm);
            let ext = if ext_sel == 0 { "uxtw" } else { "sxtw" };
            let (shift, asm_ext) = if s_bit == 0 {
                (None, ext.to_string())
            } else {
                let sh = size as u8;
                (Some(sh), format!("{ext} #{sh}"))
            };
            (index, Some(ext.to_string()), shift, asm_ext)
        } else {
            let index = rm_x(rm);
            if ext_sel == 0 {
                // LSL
                if s_bit == 0 {
                    (index, None, None, String::new())
                } else {
                    let sh = size as u8;
                    (
                        index,
                        Some("lsl".into()),
                        Some(sh),
                        format!("lsl #{sh}"),
                    )
                }
            } else {
                let ext = "sxtx";
                let (shift, asm_ext) = if s_bit == 0 {
                    (None, ext.to_string())
                } else {
                    let sh = size as u8;
                    (Some(sh), format!("{ext} #{sh}"))
                };
                (index.clone(), Some(ext.to_string()), shift, asm_ext)
            }
        };
        let mnemonic = gp_mnemonic(is_load, size);
        let addr = if asm_ext.is_empty() {
            format!("[{base}, {index}]")
        } else {
            format!("[{base}, {index}, {asm_ext}]")
        };
        let asm = format!("{mnemonic} {dest}, {addr}");
        let ops = [
            Operand::Reg(dest),
            Operand::MemRegOffset {
                base,
                index,
                extend,
                shift,
            },
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load, size, false).expect("SUT");
        prop_assert_eq!(sut, mc, "regoff mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant — ARM unsigned field layout
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_arm_fields(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        imm12 in imm12_edge(),
    ) {
        let ops = [
            Operand::Reg(gp_rt(size, rt)),
            Operand::Mem {
                base: rn_name(rn),
                offset: (imm12 as i64) * scale_of(size),
            },
        ];
        let w = sut_word(&ops, is_load, size, false).expect("SUT");
        let (sz, b29_27, v, b25_24, opc, i12, rn_f, rt_f) = unpack_unsigned(w);
        prop_assert_eq!(sz, size, "size [31:30]");
        prop_assert_eq!(b29_27, 0b111, "bits[29:27]");
        prop_assert_eq!(v, 0, "V must be 0 for GPR");
        prop_assert_eq!(b25_24, 0b01, "unsigned bits[25:24]=01");
        prop_assert_eq!(opc, if is_load { 0b01 } else { 0b00 }, "opc");
        prop_assert_eq!(i12, imm12, "imm12");
        prop_assert_eq!(rn_f, rn, "Rn");
        prop_assert_eq!(rt_f, rt, "Rt");
        let want = (size << 30)
            | (0b111 << 27)
            | (0b01 << 24)
            | ((if is_load { 0b01 } else { 0b00 }) << 22)
            | (imm12 << 10)
            | (rn << 5)
            | rt;
        prop_assert_eq!(w, want, "ARM unsigned pack");
    }

    // Oracle: algebraic.metamorphic — Rt/Rn/imm12 isolation, load/store opc, pre XOR post
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_meta_rt_rn_imm(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in 0u32..=30,
        rn in 0u32..=30,
        imm12 in 0u32..=4094,
        simm in simm9_in_range(),
    ) {
        let enc = |rt: u32, rn: u32, imm12: u32| {
            sut_word(
                &[
                    Operand::Reg(gp_rt(size, rt)),
                    Operand::Mem {
                        base: rn_name(rn),
                        offset: (imm12 as i64) * scale_of(size),
                    },
                ],
                is_load,
                size,
                false,
            )
            .expect("unsigned")
        };
        let w = enc(rt, rn, imm12);
        prop_assert_eq!(enc(rt + 1, rn, imm12).wrapping_sub(w), 1, "Rt+1 adds 1");
        prop_assert_eq!(enc(rt, rn + 1, imm12).wrapping_sub(w), 32, "Rn+1 adds 32");
        prop_assert_eq!(
            enc(rt, rn, imm12 + 1).wrapping_sub(w),
            1 << 10,
            "imm12+1 adds 1<<10"
        );
        let store = sut_word(
            &[
                Operand::Reg(gp_rt(size, rt)),
                Operand::Mem {
                    base: rn_name(rn),
                    offset: (imm12 as i64) * scale_of(size),
                },
            ],
            false,
            size,
            false,
        )
        .expect("store");
        let load = sut_word(
            &[
                Operand::Reg(gp_rt(size, rt)),
                Operand::Mem {
                    base: rn_name(rn),
                    offset: (imm12 as i64) * scale_of(size),
                },
            ],
            true,
            size,
            false,
        )
        .expect("load");
        prop_assert_eq!(load ^ store, 1 << 22, "load XOR store = opc bit 22");

        // Pre/post writeback with Rt==Rn is UNPREDICTABLE; keep them distinct.
        let rn_wb = if rn == rt { (rn + 1) % 31 } else { rn };
        let pre = sut_word(
            &[
                Operand::Reg(gp_rt(size, rt)),
                Operand::MemPreIndex {
                    base: rn_name(rn_wb),
                    offset: simm,
                },
            ],
            is_load,
            size,
            false,
        )
        .expect("pre");
        let post = sut_word(
            &[
                Operand::Reg(gp_rt(size, rt)),
                Operand::MemPostIndex {
                    base: rn_name(rn_wb),
                    offset: simm,
                },
            ],
            is_load,
            size,
            false,
        )
        .expect("post");
        prop_assert_eq!(pre ^ post, 0b10 << 10, "pre XOR post = 0b10<<10");
    }

    // Oracle: negative_error — too few operands / non-memory address
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_neg_arity_kinds(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in reg_edge(),
        kind in bad_address_kind(),
    ) {
        prop_assert!(
            encode_ldr_str(&[], is_load, size, false, false).is_err(),
            "zero operands must Err"
        );
        let one = [Operand::Reg(gp_rt(size, rt))];
        prop_assert!(
            encode_ldr_str(&one, is_load, size, false, false).is_err(),
            "one operand must Err: {:?}",
            encode_ldr_str(&one, is_load, size, false, false)
        );
        let ops = [Operand::Reg(gp_rt(size, rt)), kind.clone()];
        prop_assert!(
            encode_ldr_str(&ops, is_load, size, false, false).is_err(),
            "non-memory address operand must Err, got {:?} for {:?}",
            encode_ldr_str(&ops, is_load, size, false, false),
            kind
        );
    }

    // Oracle: negative_error — SP dest, XZR/W base, W-index without extend, writeback Rt==Rn
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_neg_invalid_regs(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in reg_edge(),
        rn in 0u32..=30,
    ) {
        let mem_x = Operand::Mem {
            base: rn_name(rn),
            offset: 0,
        };
        let sp_dest = [Operand::Reg("sp".into()), mem_x.clone()];
        prop_assert!(
            encode_ldr_str(&sp_dest, is_load, size, false, false).is_err(),
            "SP dest must Err (llvm-mc: invalid operand); got {:?}",
            encode_ldr_str(&sp_dest, is_load, size, false, false)
        );
        let w_base = [
            Operand::Reg(gp_rt(size, rt)),
            Operand::Mem {
                base: wt_name(rn),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_ldr_str(&w_base, is_load, size, false, false).is_err(),
            "W base must Err (llvm-mc: invalid operand); got {:?}",
            encode_ldr_str(&w_base, is_load, size, false, false)
        );
        let xzr_base = [
            Operand::Reg(gp_rt(size, rt)),
            Operand::Mem {
                base: "xzr".into(),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_ldr_str(&xzr_base, is_load, size, false, false).is_err(),
            "XZR base must Err (llvm-mc: invalid operand); got {:?}",
            encode_ldr_str(&xzr_base, is_load, size, false, false)
        );
        let x31_base = [
            Operand::Reg(gp_rt(size, rt)),
            Operand::Mem {
                base: "x31".into(),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_ldr_str(&x31_base, is_load, size, false, false).is_err(),
            "x31 base (XZR) must Err; got {:?}",
            encode_ldr_str(&x31_base, is_load, size, false, false)
        );
        let w_index = [
            Operand::Reg(gp_rt(size, rt)),
            Operand::MemRegOffset {
                base: rn_name(rn),
                index: wt_name(rt),
                extend: None,
                shift: None,
            },
        ];
        prop_assert!(
            encode_ldr_str(&w_index, is_load, size, false, false).is_err(),
            "W index without uxtw/sxtw must Err; got {:?}",
            encode_ldr_str(&w_index, is_load, size, false, false)
        );
        if rt != 31 {
            let overlap_pre = [
                Operand::Reg(gp_rt(size, rt)),
                Operand::MemPreIndex {
                    base: format!("x{rt}"),
                    offset: scale_of(size),
                },
            ];
            prop_assert!(
                encode_ldr_str(&overlap_pre, is_load, size, false, false).is_err(),
                "pre-index Rt==Rn must Err (llvm-mc: unpredictable); got {:?}",
                encode_ldr_str(&overlap_pre, is_load, size, false, false)
            );
        }
    }

    // Oracle: negative_error — out-of-range offset and extra operand
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_neg_offset_extra(
        is_load in any::<bool>(),
        size in size_edge(),
        rt in reg_edge(),
        rn in reg_edge(),
        extra in extra_operand(),
        prepost in 0u32..=1,
        which_off in 0u32..=3,
    ) {
        let off = match which_off {
            0 => SIMM_MIN - 1,
            1 => pimm_max(size) + 1,
            2 => i64::MIN,
            _ => i64::MAX,
        };
        let mem = [
            Operand::Reg(gp_rt(size, rt)),
            Operand::Mem {
                base: rn_name(rn),
                offset: off,
            },
        ];
        prop_assert!(
            encode_ldr_str(&mem, is_load, size, false, false).is_err(),
            "out-of-range Mem offset {} must Err (llvm-mc range); got {:?}",
            off,
            encode_ldr_str(&mem, is_load, size, false, false)
        );
        let three = vec![
            Operand::Reg(gp_rt(size, rt)),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
            extra.clone(),
        ];
        prop_assert!(
            encode_ldr_str(&three, is_load, size, false, false).is_err(),
            "extra operand must Err; got {:?}",
            encode_ldr_str(&three, is_load, size, false, false)
        );
        let wb = if prepost == 0 {
            vec![
                Operand::Reg(gp_rt(size, rt)),
                Operand::MemPreIndex {
                    base: rn_name(rn),
                    offset: off,
                },
            ]
        } else {
            vec![
                Operand::Reg(gp_rt(size, rt)),
                Operand::MemPostIndex {
                    base: rn_name(rn),
                    offset: off,
                },
            ]
        };
        prop_assert!(
            encode_ldr_str(&wb, is_load, size, false, false).is_err(),
            "out-of-range pre/post offset {} must Err; got {:?}",
            off,
            encode_ldr_str(&wb, is_load, size, false, false)
        );
    }
}

#[test]
fn test_encode_ldr_str_regression_sp_dest() {
    let ops = [
        Operand::Reg("sp".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldr_str(&ops, false, 0b00, false, false).is_err(),
        "STRB SP, [X0] must Err; SP is not a valid Rt (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldr_str_regression_xzr_base() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Mem {
            base: "xzr".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldr_str(&ops, true, 0b11, false, false).is_err(),
        "LDR X0, [XZR] must Err; Rn=31 is SP not XZR (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldr_str_regression_w_base() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Mem {
            base: "w0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldr_str(&ops, true, 0b11, false, false).is_err(),
        "LDR X0, [W0] must Err; base must be Xn|SP (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldr_str_regression_w_index() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::MemRegOffset {
            base: "x1".into(),
            index: "w2".into(),
            extend: None,
            shift: None,
        },
    ];
    assert!(
        encode_ldr_str(&ops, true, 0b11, false, false).is_err(),
        "LDR X0, [X1, W2] must Err; W index requires uxtw/sxtw (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldr_str_regression_writeback_overlap() {
    let ops = [
        Operand::Reg("x0".into()),
        Operand::MemPreIndex {
            base: "x0".into(),
            offset: 8,
        },
    ];
    assert!(
        encode_ldr_str(&ops, true, 0b11, false, false).is_err(),
        "LDR X0, [X0, #8]! must Err; writeback Rt==Rn is unpredictable (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldr_str_regression_imm9_range() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: -257,
        },
    ];
    assert!(
        encode_ldr_str(&ops, false, 0b00, false, false).is_err(),
        "STRB W0, [X0, #-257] must Err; simm9/pimm out of range (llvm-mc rejects it)"
    );
}

#[test]
fn test_encode_ldr_str_regression_extra_operand() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
        Operand::Reg("x0".into()),
    ];
    assert!(
        encode_ldr_str(&ops, false, 0b00, false, false).is_err(),
        "STRB W0, [X0], X0 must Err; extra operand is invalid (llvm-mc rejects it)"
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential — llvm-mc SIMD S/D/Q unsigned LDR/STR
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_diff_simd_llvm_mc(
        is_load in any::<bool>(),
        fp_kind in 0u32..=2,
        rt in reg_edge(),
        rn in reg_edge(),
        imm12 in imm12_edge(),
    ) {
        let (reg, size, is_128, scale) = match fp_kind {
            0 => (format!("s{rt}"), 0b10u32, false, 4i64),
            1 => (format!("d{rt}"), 0b11u32, false, 8i64),
            _ => (format!("q{rt}"), 0b00u32, true, 16i64),
        };
        let base = rn_name(rn);
        let offset = (imm12 as i64) * scale;
        let mnemonic = if is_load { "ldr" } else { "str" };
        let asm = format!("{mnemonic} {reg}, {}", asm_mem(&base, offset));
        let ops = [
            Operand::Reg(reg),
            Operand::Mem { base, offset },
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load, size, is_128).expect("SUT");
        prop_assert_eq!(sut, mc, "SIMD unsigned mismatch for {}", asm);
    }

    // Oracle: differential — llvm-mc alt spellings x31 / uppercase / lr
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_diff_alt_spellings(
        is_load in any::<bool>(),
        rt in reg_edge(),
        rn in reg_edge(),
        spelling in 0u32..=2,
    ) {
        let size = 0b11u32;
        let (rt_s, rn_s) = match spelling {
            0 => {
                let rt_s = if rt == 31 { "x31".to_string() } else { format!("x{rt}") };
                let rn_s = if rn == 31 { "sp".to_string() } else { format!("x{rn}") };
                (rt_s, rn_s)
            }
            1 => {
                let rt_s = if rt == 31 { "XZR".to_string() } else { format!("X{rt}") };
                let rn_s = if rn == 31 { "SP".to_string() } else { format!("X{rn}") };
                (rt_s, rn_s)
            }
            _ => {
                let rt_s = if rt == 30 {
                    "lr".to_string()
                } else if rt == 31 {
                    "xzr".to_string()
                } else {
                    format!("x{rt}")
                };
                let rn_s = if rn == 31 { "sp".to_string() } else { format!("x{rn}") };
                (rt_s, rn_s)
            }
        };
        let mnemonic = if is_load { "ldr" } else { "str" };
        let asm = format!("{mnemonic} {rt_s}, {}", asm_mem(&rn_s, 0));
        let ops = [
            Operand::Reg(rt_s),
            Operand::Mem {
                base: rn_s,
                offset: 0,
            },
        ];
        let mc = llvm_mc_word(&asm).expect("llvm-mc");
        let sut = sut_word(&ops, is_load, size, false).expect("SUT");
        prop_assert_eq!(sut, mc, "alt-spelling mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant — LDR literal reloc (load-only)
    // Target: encoder.load_store.encode_ldr_str
    #[test]
    fn encode_ldr_str_literal_reloc(
        size in prop_oneof![Just(0b10u32), Just(0b11u32)],
        rt in reg_edge(),
    ) {
        let dest = gp_rt(size, rt);
        let ops = [Operand::Reg(dest.clone()), Operand::Symbol("foo".into())];
        match encode_ldr_str(&ops, true, size, false, false) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                let opc = if size == 0b11 { 0b01u32 } else { 0b00 };
                let want = (opc << 30) | (0b011 << 27) | rt;
                prop_assert_eq!(word, want, "LDR literal opcode/Rt");
                prop_assert!(
                    matches!(reloc.reloc_type, RelocType::Ldr19),
                    "reloc type must be Ldr19, got {:?}",
                    reloc.reloc_type
                );
                prop_assert_eq!(reloc.symbol, "foo");
                prop_assert_eq!(reloc.addend, 0);
            }
            other => prop_assert!(false, "expected WordWithReloc, got {:?}", other),
        }
        let str_ops = [Operand::Reg(dest), Operand::Symbol("foo".into())];
        prop_assert!(
            encode_ldr_str(&str_ops, false, size, false, false).is_err(),
            "STR literal must Err (llvm-mc rejects it)"
        );
    }
}

#[test]
fn test_encode_ldr_str_regression_byte_lsl0() {
    let ops = [
        Operand::Reg("w0".into()),
        Operand::MemRegOffset {
            base: "x0".into(),
            index: "x0".into(),
            extend: Some("lsl".into()),
            shift: Some(0),
        },
    ];
    let sut = sut_word(&ops, false, 0b00, false).expect("SUT");
    let mc = llvm_mc_word("strb w0, [x0, x0, lsl #0]").expect("llvm-mc");
    assert_eq!(
        sut, mc,
        "STRB W0, [X0, X0, LSL #0] must match llvm-mc S=1 encoding"
    );
}
