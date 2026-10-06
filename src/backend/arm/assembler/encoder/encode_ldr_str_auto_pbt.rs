// Oracle: differential — llvm-mc AArch64 assembler
// Evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume";
//   README.md Loads/Stores table lists ldr, str (size from the data register);
//   encoder/mod.rs:3 "Encodes AArch64 instructions into 32-bit machine code words";
//   encoder/mod.rs:484-486 "Loads/stores - size determined from register width";
//   "ldr"/"str" => encode_ldr_str_auto;
//   load_store.rs:7 "Auto-detect LDR/STR size from the first register operand.";
//   load_store.rs:8-9 Wn->size=10, Xn->size=11, Sn->32-bit, Dn->64-bit, Qn->128-bit;
//   ARM ARM LDR/STR unsigned: size 111 V 01 opc imm12 Rn Rt (pimm = imm12*(1<<shift));
//   SIMD&FP: Bt size=00 V=1, Ht size=01 V=1, St size=10 V=1, Dt size=11 V=1,
//   Qt size=00 V=1 opc=11/10 shift=4.
// Stronger considered:
//   - State machine: rejected — encode_ldr_str_auto is a pure function with no lifecycle
//   - Algebraic round-trip via in-tree decoder: rejected — no LDR/STR decoder
//   - encode_ldr_str as differential sibling: rejected — same-job gate (callee, explicit size,
//     shared get_reg / same crate)
// Weaker available: algebraic.invariant (ARM size/V/opc from register prefix),
//   algebraic.metamorphic (load XOR store = bit 22; W vs X size bits),
//   negative_error (empty / non-Reg first operand)
// Differential: candidate=encode_ldr_str_auto, reference=llvm-mc -triple=aarch64 -show-encoding,
//   SUT-boundary=internal-helper of GNU-style assembler,
//   mapping=(Reg(Rt), Mem/Symbol, is_load) <-> `ldr/str Rt, <addr>`

use super::encode_ldr_str_auto;
use super::EncodeResult;
use super::RelocType;
use crate::backend::arm::assembler::parser::Operand;
use proptest::prelude::*;
use std::io::Write;
use std::process::{Command, Stdio};

const LLVM_MC: &str = "/home/toan/tools/llvm15-official/bin/llvm-mc";
const CASES: u32 = 1000;

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

fn simd_name(pref: char, n: u32) -> String {
    format!("{pref}{n}")
}

fn mnemonic(is_load: bool) -> &'static str {
    if is_load {
        "ldr"
    } else {
        "str"
    }
}

fn asm_mem(rn: &str, offset: i64) -> String {
    if offset == 0 {
        format!("[{rn}]")
    } else {
        format!("[{rn}, #{offset}]")
    }
}

fn sut_word(ops: &[Operand], is_load: bool) -> Result<u32, String> {
    match encode_ldr_str_auto(ops, is_load)? {
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

/// Unpack unsigned-offset LDR/STR fields (ARM ARM, not a copy of the SUT packer).
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

fn rt_no_sp() -> impl Strategy<Value = u32> {
    prop_oneof![Just(0u32), Just(1u32), Just(30u32), Just(31u32), 0u32..=31]
}

fn imm12_edge() -> impl Strategy<Value = u32> {
    prop_oneof![
        Just(0u32),
        Just(1u32),
        Just(2u32),
        Just(4094u32),
        Just(4095u32),
        0u32..=4095
    ]
}

fn class_name(class: u32, n: u32) -> String {
    match class {
        0 => wt_name(n),
        1 => xt_name(n),
        2 => simd_name('s', n),
        3 => simd_name('d', n),
        4 => simd_name('q', n),
        5 => simd_name('b', n),
        _ => simd_name('h', n),
    }
}

/// ARM expected (size, v, opc) for unsigned form from register class.
fn expected_size_v_opc(class: u32, is_load: bool) -> (u32, u32, u32) {
    let load_opc = if is_load { 0b01 } else { 0b00 };
    match class {
        0 => (0b10, 0, load_opc),              // W
        1 => (0b11, 0, load_opc),              // X
        2 => (0b10, 1, load_opc),              // S
        3 => (0b11, 1, load_opc),              // D
        4 => (0b00, 1, if is_load { 0b11 } else { 0b10 }), // Q
        5 => (0b00, 1, load_opc),              // B
        _ => (0b01, 1, load_opc),              // H
    }
}

fn extra_operand() -> impl Strategy<Value = Operand> {
    prop_oneof![
        Just(Operand::Imm(0)),
        Just(Operand::Symbol("foo".into())),
        Just(Operand::Label("bar".into())),
        Just(Operand::Reg("x2".into())),
        Just(Operand::Shift {
            kind: "lsl".into(),
            amount: 0
        }),
        Just(Operand::Cond("eq".into())),
    ]
}

/// Known-answer gate for the llvm-mc differential connection.
#[test]
fn encode_ldr_str_auto_kat_llvm_mc_x0_x1() {
    let want = 0xf9400020u32;
    let mc = llvm_mc_word("ldr x0, [x1]").expect("llvm-mc KAT");
    assert_eq!(mc, want, "llvm-mc KAT mapping broken");
    let ops = [
        Operand::Reg("x0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, true).expect("SUT KAT");
    assert_eq!(sut, want);
}

#[test]
fn encode_ldr_str_auto_kat_llvm_mc_w_s_d_q() {
    let cases = [
        ("ldr w0, [x1]", true, "w0", 0xb9400020u32),
        ("str x0, [x1]", false, "x0", 0xf9000020u32),
        ("ldr s0, [x1]", true, "s0", 0xbd400020u32),
        ("ldr d0, [x1]", true, "d0", 0xfd400020u32),
        ("ldr q0, [x1]", true, "q0", 0x3dc00020u32),
        ("ldr lr, [x1]", true, "lr", 0xf940003eu32),
    ];
    for (asm, is_load, rt, want) in cases {
        let mc = llvm_mc_word(asm).unwrap_or_else(|e| panic!("llvm-mc KAT {asm}: {e}"));
        assert_eq!(mc, want, "llvm-mc KAT mapping broken for {asm}");
        let ops = [
            Operand::Reg(rt.into()),
            Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, is_load).unwrap_or_else(|e| panic!("SUT KAT {asm}: {e}"));
        assert_eq!(sut, want, "SUT KAT {asm}");
    }
}

#[test]
fn test_encode_ldr_str_auto_regression_byte_reg() {
    let want = 0x3d400020u32;
    let mc = llvm_mc_word("ldr b0, [x1]").expect("llvm-mc ldr b0, [x1]");
    assert_eq!(mc, want, "llvm-mc encoding of ldr b0, [x1]");
    let ops = [
        Operand::Reg("b0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, true).expect("SUT must encode ldr b0, [x1]");
    assert_eq!(
        sut, want,
        "LDR Bt must use size=00 V=1 opc=01 (not default size=11 D-form)"
    );
}

#[test]
fn test_encode_ldr_str_auto_regression_half_reg() {
    let want = 0x7d400020u32;
    let mc = llvm_mc_word("ldr h0, [x1]").expect("llvm-mc ldr h0, [x1]");
    assert_eq!(mc, want, "llvm-mc encoding of ldr h0, [x1]");
    let ops = [
        Operand::Reg("h0".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, true).expect("SUT must encode ldr h0, [x1]");
    assert_eq!(
        sut, want,
        "LDR Ht must use size=01 V=1 opc=01 (not default size=11 D-form)"
    );
}

#[test]
fn test_encode_ldr_str_auto_regression_str_byte() {
    let want = 0x3d000000u32;
    let mc = llvm_mc_word("str b0, [x0]").expect("llvm-mc str b0, [x0]");
    assert_eq!(mc, want);
    let ops = [
        Operand::Reg("b0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, false).expect("SUT must encode str b0, [x0]");
    assert_eq!(sut, want, "STR Bt must use size=00 V=1 opc=00");
}

#[test]
fn test_encode_ldr_str_auto_regression_bare_v() {
    let ops = [
        Operand::Reg("v0".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldr_str_auto(&ops, false).is_err(),
        "STR V0, [X0] must Err; llvm-mc rejects bare Vn as LDR/STR Rt, got {:?}",
        encode_ldr_str_auto(&ops, false)
    );
}

#[test]
fn test_encode_ldr_str_auto_regression_fp_alias() {
    let want = 0xf940003du32; // ldr x29, [x1]
    let mc = llvm_mc_word("ldr fp, [x1]").expect("llvm-mc ldr fp, [x1]");
    assert_eq!(mc, want);
    let ops = [
        Operand::Reg("fp".into()),
        Operand::Mem {
            base: "x1".into(),
            offset: 0,
        },
    ];
    let sut = sut_word(&ops, true).expect("SUT must accept GNU fp alias of X29");
    assert_eq!(sut, want, "LDR fp, [X1] must encode as LDR X29, [X1]");
}

#[test]
fn test_encode_ldr_str_auto_regression_sp_dest() {
    let ops = [
        Operand::Reg("sp".into()),
        Operand::Mem {
            base: "x0".into(),
            offset: 0,
        },
    ];
    assert!(
        encode_ldr_str_auto(&ops, true).is_err(),
        "LDR SP, [X0] must Err; SP is not a valid Rt (llvm-mc rejects it), got {:?}",
        encode_ldr_str_auto(&ops, true)
    );
}

proptest! {
    #![proptest_config(cfg())]

    // Oracle: differential
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_diff_gpr(
        is_load in any::<bool>(),
        rt in rt_no_sp(),
        rn in reg_edge(),
        imm12 in imm12_edge(),
        is_64 in any::<bool>(),
    ) {
        let size = if is_64 { 0b11u32 } else { 0b10 };
        let scale = 1i64 << size;
        let offset = (imm12 as i64) * scale;
        let rt_n = if is_64 { xt_name(rt) } else { wt_name(rt) };
        let rn_n = rn_name(rn);
        let asm = format!("{} {}, {}", mnemonic(is_load), rt_n, asm_mem(&rn_n, offset));
        let ops = [
            Operand::Reg(rt_n.clone()),
            Operand::Mem {
                base: rn_n,
                offset,
            },
        ];
        let sut = sut_word(&ops, is_load)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: differential
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_diff_fp_sdq(
        is_load in any::<bool>(),
        rt in 0u32..=31,
        rn in reg_edge(),
        imm12 in imm12_edge(),
        pref in 0u32..=2,
    ) {
        let (ch, shift) = match pref {
            0 => ('s', 2u32),
            1 => ('d', 3u32),
            _ => ('q', 4u32),
        };
        let offset = (imm12 as i64) * (1i64 << shift);
        let rt_n = simd_name(ch, rt);
        let rn_n = rn_name(rn);
        let asm = format!("{} {}, {}", mnemonic(is_load), rt_n, asm_mem(&rn_n, offset));
        let ops = [
            Operand::Reg(rt_n),
            Operand::Mem {
                base: rn_n,
                offset,
            },
        ];
        let sut = sut_word(&ops, is_load)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: differential
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_diff_fp_bh(
        is_load in any::<bool>(),
        rt in 0u32..=31,
        rn in reg_edge(),
        imm12 in imm12_edge(),
        is_h in any::<bool>(),
    ) {
        let (ch, shift) = if is_h { ('h', 1u32) } else { ('b', 0u32) };
        let offset = (imm12 as i64) * (1i64 << shift);
        let rt_n = simd_name(ch, rt);
        let rn_n = rn_name(rn);
        let asm = format!("{} {}, {}", mnemonic(is_load), rt_n, asm_mem(&rn_n, offset));
        let ops = [
            Operand::Reg(rt_n),
            Operand::Mem {
                base: rn_n,
                offset,
            },
        ];
        let sut = sut_word(&ops, is_load)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: algebraic.invariant
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_size_v_bits(
        is_load in any::<bool>(),
        rt in 0u32..=30,
        rn in reg_edge(),
        class in 0u32..=6,
    ) {
        let rt_n = class_name(class, rt);
        let rn_n = rn_name(rn);
        let ops = [
            Operand::Reg(rt_n),
            Operand::Mem {
                base: rn_n,
                offset: 0,
            },
        ];
        let word = sut_word(&ops, is_load)
            .unwrap_or_else(|e| panic!("SUT rejected class {class} rt={rt} rn={rn}: {e}"));
        let (size, bits29_27, v, bits25_24, opc, imm12, got_rn, got_rt) = unpack_unsigned(word);
        let (exp_size, exp_v, exp_opc) = expected_size_v_opc(class, is_load);
        prop_assert_eq!(bits29_27, 0b111, "bits[29:27]");
        prop_assert_eq!(bits25_24, 0b01, "unsigned form bits[25:24]");
        prop_assert_eq!(imm12, 0, "offset 0");
        prop_assert_eq!(got_rt, rt, "Rt");
        prop_assert_eq!(got_rn, rn, "Rn");
        prop_assert_eq!(size, exp_size, "size bits[31:30] class {}", class);
        prop_assert_eq!(v, exp_v, "V bit class {}", class);
        prop_assert_eq!(opc, exp_opc, "opc class {} load {}", class, is_load);
    }

    // Oracle: algebraic.metamorphic
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_meta_load_store(
        rt in 0u32..=30,
        rn in reg_edge(),
        class in 0u32..=6,
    ) {
        let rt_n = class_name(class, rt);
        let rn_n = rn_name(rn);
        let ops = [
            Operand::Reg(rt_n),
            Operand::Mem {
                base: rn_n,
                offset: 0,
            },
        ];
        let load = sut_word(&ops, true)
            .unwrap_or_else(|e| panic!("load rejected class {class}: {e}"));
        let store = sut_word(&ops, false)
            .unwrap_or_else(|e| panic!("store rejected class {class}: {e}"));
        prop_assert_eq!(
            load ^ store,
            1u32 << 22,
            "load XOR store must be opc LSB class={} load={:#010x} store={:#010x}",
            class,
            load,
            store
        );
        if class == 0 {
            let xops = [
                Operand::Reg(xt_name(rt)),
                Operand::Mem {
                    base: rn_name(rn),
                    offset: 0,
                },
            ];
            let xword = sut_word(&xops, true)
                .unwrap_or_else(|e| panic!("X load rejected: {e}"));
            prop_assert_eq!(
                load ^ xword,
                1u32 << 30,
                "W XOR X must be size MSB W={:#010x} X={:#010x}",
                load,
                xword
            );
        }
    }

    // Oracle: negative_error
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_neg_first_operand(
        is_load in any::<bool>(),
        kind in 0u32..=7,
        extra in extra_operand(),
    ) {
        let ops: Vec<Operand> = match kind {
            0 => vec![],
            1 => vec![Operand::Imm(0)],
            2 => vec![Operand::Mem {
                base: "x0".into(),
                offset: 0,
            }],
            3 => vec![Operand::Symbol("foo".into())],
            4 => vec![Operand::Label("bar".into())],
            5 => vec![Operand::Shift {
                kind: "lsl".into(),
                amount: 0,
            }],
            6 => vec![Operand::Cond("eq".into())],
            _ => vec![Operand::Reg("x0".into())],
        };
        prop_assert!(
            encode_ldr_str_auto(&ops, is_load).is_err(),
            "non-register / arity-1 first operand must Err; got {:?} for {:?}",
            encode_ldr_str_auto(&ops, is_load),
            ops
        );
        let _ = extra;
    }

    // Oracle: differential
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_diff_aliases(
        is_load in any::<bool>(),
        which in 0u32..=8,
    ) {
        let alias: &str = match which {
            0 => "lr",
            1 => "xzr",
            2 => "wzr",
            3 => "XZR",
            4 => "WZR",
            5 => "X0",
            6 => "W0",
            7 => "x31",
            _ => "w31",
        };
        let asm = format!("{} {}, [x1]", mnemonic(is_load), alias);
        let ops = [
            Operand::Reg(alias.into()),
            Operand::Mem {
                base: "x1".into(),
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, is_load)
            .unwrap_or_else(|e| panic!("SUT rejected valid {asm}: {e}"));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {asm}: {e}"));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }

    // Oracle: differential
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_literal(
        rt in 0u32..=30,
        class in 0u32..=4,
    ) {
        let rt_n = match class {
            0 => wt_name(rt),
            1 => xt_name(rt),
            2 => simd_name('s', rt),
            3 => simd_name('d', rt),
            _ => simd_name('q', rt),
        };
        let ops = [Operand::Reg(rt_n.clone()), Operand::Symbol("foo".into())];
        match encode_ldr_str_auto(&ops, true) {
            Ok(EncodeResult::WordWithReloc { word, reloc }) => {
                let asm0 = format!("ldr {rt_n}, #0");
                let mc = llvm_mc_word(&asm0)
                    .unwrap_or_else(|e| panic!("llvm-mc rejected {asm0}: {e}"));
                prop_assert_eq!(word, mc, "literal word vs llvm-mc ldr {}, #0", rt_n);
                prop_assert!(
                    matches!(reloc.reloc_type, RelocType::Ldr19),
                    "reloc type must be Ldr19, got {:?}",
                    reloc.reloc_type
                );
                prop_assert_eq!(reloc.symbol.as_str(), "foo");
                prop_assert_eq!(reloc.addend, 0);
            }
            other => {
                prop_assert!(
                    false,
                    "LDR literal must be WordWithReloc Ldr19, got {:?}", other
                );
            }
        }
        prop_assert!(
            encode_ldr_str_auto(&ops, false).is_err(),
            "STR literal must Err (llvm-mc rejects str Rt, label); got {:?}",
            encode_ldr_str_auto(&ops, false)
        );
    }

    // Oracle: negative_error — SP is not a valid LDR/STR Rt (llvm-mc: invalid operand; ARM Rt=31 is ZR)
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_neg_sp_dest(
        is_load in any::<bool>(),
        rn in reg_edge(),
    ) {
        let ops = [
            Operand::Reg("sp".into()),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_ldr_str_auto(&ops, is_load).is_err(),
            "SP dest must Err (llvm-mc: invalid operand); got {:?}",
            encode_ldr_str_auto(&ops, is_load)
        );
    }

    // Oracle: negative_error — sweep: bare Vn is not a scalar LDR/STR Rt (llvm-mc rejects)
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_neg_v_reg(
        is_load in any::<bool>(),
        rt in 0u32..=31,
        rn in reg_edge(),
    ) {
        let ops = [
            Operand::Reg(format!("v{rt}")),
            Operand::Mem {
                base: rn_name(rn),
                offset: 0,
            },
        ];
        prop_assert!(
            encode_ldr_str_auto(&ops, is_load).is_err(),
            "bare Vn must Err (llvm-mc: invalid operand); got {:?}",
            encode_ldr_str_auto(&ops, is_load)
        );
    }

    // Oracle: differential — sweep: GNU fp alias of X29
    // Target: encoder.load_store.encode_ldr_str_auto
    #[test]
    fn encode_ldr_str_auto_diff_fp_alias(
        is_load in any::<bool>(),
        rn in 0u32..=30,
    ) {
        let rn_n = rn_name(rn);
        let asm = format!("{} fp, {}", mnemonic(is_load), asm_mem(&rn_n, 0));
        let ops = [
            Operand::Reg("fp".into()),
            Operand::Mem {
                base: rn_n,
                offset: 0,
            },
        ];
        let sut = sut_word(&ops, is_load)
            .unwrap_or_else(|e| panic!("SUT rejected valid {}: {}", asm, e));
        let mc = llvm_mc_word(&asm)
            .unwrap_or_else(|e| panic!("llvm-mc rejected valid {}: {}", asm, e));
        prop_assert_eq!(sut, mc, "SUT vs llvm-mc mismatch for {}", asm);
    }
}
