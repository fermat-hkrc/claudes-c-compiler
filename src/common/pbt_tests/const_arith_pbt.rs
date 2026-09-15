//! PBT properties (proptest) for const_arith.rs — see pbt-out/PROPERTIES.md.
use crate::common::const_arith::eval_const_binop;
use crate::frontend::parser::ast::BinOp;
use crate::ir::reexports::IrConst;
use proptest::prelude::*;

// Property-based tests (proptest) for const-fold integer arithmetic.
// Oracle: differential against an independent C11-semantics reference
// model (integer ops on the type domain: wrapping +,-,*; truncating /,%;
// arithmetic >> on signed; comparisons typed per signedness).
// Domain is UB-free: r != 0 for Div/Mod (and no MIN/-1 when signed),
// shift counts within the type width.
use proptest::prelude::*;

/// Independent C11 reference model on the (width, signedness) domain.
/// Returns the value the SUT must produce, viewed as i64 with the
/// documented representation: signed 32-bit -> sign-extended;
/// unsigned 32-bit -> zero-extended (see eval_const_binop_int comment);
/// 64-bit -> raw value.
fn c_ref(op: &BinOp, l: i64, r: i64, is_32bit: bool, uns: bool) -> i64 {
    let to_bits = |x: i64| -> u64 {
        if is_32bit { (x as u32) as u64 } else { x as u64 }
    };
    let ext = |bits: u64| -> i64 {
        if is_32bit {
            if uns { (bits as u32) as i64 } else { (bits as u32 as i32) as i64 }
        } else {
            bits as i64
        }
    };
    let lb = to_bits(l);
    let rb = to_bits(r);
    match op {
        BinOp::Add => ext(lb.wrapping_add(rb)),
        BinOp::Sub => ext(lb.wrapping_sub(rb)),
        BinOp::Mul => ext(lb.wrapping_mul(rb)),
        BinOp::Div => {
            if uns { ext(lb / rb) } else { ext(l.wrapping_div(r) as u64) }
        }
        BinOp::Mod => {
            if uns { ext(lb % rb) } else { ext(l.wrapping_rem(r) as u64) }
        }
        BinOp::Shl => ext(lb << (rb & 63)),
        BinOp::Shr => {
            if uns {
                ext(lb >> (rb & 63))
            } else {
                // arithmetic shift on the signed domain value
                let sv = if is_32bit { (l as i32) as i64 } else { l };
                let c = (rb & 63) as u32;
                let shifted = sv >> c; // Rust >> on i64 is arithmetic
                if is_32bit { (shifted as i32) as i64 } else { shifted }
            }
        }
        BinOp::BitAnd => ext(lb & rb),
        BinOp::BitOr => ext(lb | rb),
        BinOp::BitXor => ext(lb ^ rb),
        BinOp::Eq => ((lb == rb) as i64),
        BinOp::Ne => ((lb != rb) as i64),
        BinOp::Lt => (if uns { lb < rb } else { ext(lb) < ext(rb) }) as i64,
        BinOp::Gt => (if uns { lb > rb } else { ext(lb) > ext(rb) }) as i64,
        BinOp::Le => (if uns { lb <= rb } else { ext(lb) <= ext(rb) }) as i64,
        BinOp::Ge => (if uns { lb >= rb } else { ext(lb) >= ext(rb) }) as i64,
        BinOp::LogicalAnd => ((l != 0 && r != 0) as i64),
        BinOp::LogicalOr => ((l != 0 || r != 0) as i64),
    }
}

fn boundary_pool() -> Vec<i64> {
    vec![
        0, 1, -1, 2, -2, 3, 7, -7, 31, 32, 63, 64, 255, 256, 1000, -1000,
        32767, -32768, 65535, 65536,
        2147483647, -2147483648, 2147483648, 4294967295, 4294967296,
        9223372036854775807, i64::MIN, -4294967296,
    ]
}

fn value_gen(is_32bit: bool, uns: bool) -> BoxedStrategy<i64> {
    prop::strategy::Union::new_weighted(vec![
        (70, any::<i64>().prop_map(move |v| {
            // represent the value properly extended, as real callers do
            if is_32bit {
                if uns { (v as u32) as i64 } else { (v as i32) as i64 }
            } else {
                v
            }
        }).boxed()),
        (30, proptest::sample::select(boundary_pool()).prop_map(move |v| {
            if is_32bit {
                if uns { (v as u32) as i64 } else { (v as i32) as i64 }
            } else {
                v
            }
        }).boxed()),
    ]).boxed()
}

fn ops() -> Vec<BinOp> {
    use BinOp::*;
    vec![Add, Sub, Mul, Div, Mod, Shl, Shr, BitAnd, BitOr, BitXor,
         Eq, Ne, Lt, Gt, Le, Ge, LogicalAnd, LogicalOr]
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(1000))]

    /// P8: eval_const_binop agrees with the C11 reference model on the
    /// UB-free domain, for every (width, signedness) combination.
    #[test]
    fn const_binop_matches_c_semantics(
        op_idx in 0usize..18,
        l_raw in any::<i64>(),
        r_raw in any::<i64>(),
        l_bnd in proptest::sample::select(boundary_pool()),
        r_bnd in proptest::sample::select(boundary_pool()),
        use_boundary_l in any::<bool>(),
        use_boundary_r in any::<bool>(),
        is_32bit in any::<bool>(),
        is_unsigned in any::<bool>(),
    ) {
        let op = &ops()[op_idx];
        let to_dom = |v: i64| -> i64 {
            if is_32bit {
                if is_unsigned { (v as u32) as i64 } else { (v as i32) as i64 }
            } else { v }
        };
        let mut l = to_dom(if use_boundary_l { l_bnd } else { l_raw });
        let mut r = to_dom(if use_boundary_r { r_bnd } else { r_raw });

        // UB-free domain adjustments
        match op {
            BinOp::Div | BinOp::Mod => {
                if r == 0 { r = 7; }
                if !is_unsigned && r == -1 {
                    let width_min = if is_32bit { i32::MIN as i64 } else { i64::MIN };
                    if l == width_min { l = width_min + 1; } // C UB: MIN / -1
                }
            }
            BinOp::Shl | BinOp::Shr => {
                // shift count within type width, non-negative
                let width = if is_32bit { 32u32 } else { 64 };
                r = (r.rem_euclid(width as i64)) as i64;
            }
            _ => {}
        }

        let expected = c_ref(op, l, r, is_32bit, is_unsigned);
        let got = eval_const_binop(op, &IrConst::I64(l), &IrConst::I64(r), is_32bit, is_unsigned, is_unsigned, is_unsigned);
        let got_v = got.expect("int binop must fold").to_i64().expect("int result");
        prop_assert_eq!(
            got_v, expected,
            "op={:?} l={} r={} w32={} uns={}", op, l, r, is_32bit, is_unsigned
        );
    }
}
