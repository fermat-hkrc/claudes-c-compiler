//! Shared constant expression evaluation functions.
//!
//! This module extracts the pure constant-evaluation logic that was duplicated
//! between `sema::const_eval` (SemaConstEval) and `ir::lowering::const_eval`
//! (Lowerer). Both evaluate compile-time constant expressions from the same AST,
//! but differ in how they resolve types:
//! - Sema uses CType (C-level types from the type checker)
//! - Lowering uses IrType (IR-level types from the lowerer)
//!
//! The functions here are parameterized by closures that abstract over these
//! differences, allowing both callers to share the same evaluation logic for:
//! - Literal evaluation
//! - Builtin function constant folding (__builtin_bswap, __builtin_clz, etc.)
//! - Sub-int promotion for unary operations
//! - Bit-width evaluation through cast chains
//!
//! Functions that require caller-specific state (global address resolution,
//! sizeof/alignof, binary operations with type inference) remain in the
//! respective callers.

use crate::ir::reexports::IrConst;
use crate::frontend::parser::ast::{BinOp, Expr};
use super::const_arith;
use super::types::target_is_32bit;

/// Evaluate a literal expression to an IrConst.
/// Returns None for non-literal expressions.
#[inline]
pub fn eval_literal(expr: &Expr) -> Option<IrConst> {
    match expr {
        // IntLiteral is `int` (32-bit) when the value fits, otherwise `long`.
        Expr::IntLiteral(val, _) => {
            if *val >= i32::MIN as i64 && *val <= i32::MAX as i64 {
                Some(IrConst::I32(*val as i32))
            } else {
                Some(IrConst::I64(*val))
            }
        }
        Expr::LongLiteral(val, _) => Some(IrConst::I64(*val)),
        Expr::LongLongLiteral(val, _) => Some(IrConst::I64(*val)),
        // UIntLiteral stays as I64 to preserve the unsigned value.
        Expr::UIntLiteral(val, _) => Some(IrConst::I64(*val as i64)),
        Expr::ULongLiteral(val, _) => Some(IrConst::I64(*val as i64)),
        Expr::ULongLongLiteral(val, _) => Some(IrConst::I64(*val as i64)),
        Expr::CharLiteral(ch, _) => {
            // Sign-extend from signed char to int, matching GCC behavior.
            // '\xEF' should be -17, not 239, when char is signed.
            Some(IrConst::I32(*ch as u8 as i8 as i32))
        }
        Expr::FloatLiteral(val, _) => Some(IrConst::F64(*val)),
        Expr::FloatLiteralF32(val, _) => Some(IrConst::F32(*val as f32)),
        Expr::FloatLiteralLongDouble(val, bytes, _) => {
            Some(IrConst::long_double_with_bytes(*val, *bytes))
        }
        _ => None,
    }
}

/// Evaluate a compile-time builtin function call in a constant expression.
///
/// Handles all pure builtins that can be folded at compile time:
/// - __builtin_choose_expr, __builtin_constant_p, __builtin_expect
/// - __builtin_bswap{16,32,64}
/// - __builtin_clz{,l,ll}, __builtin_ctz{,l,ll}
/// - __builtin_popcount{,l,ll}, __builtin_ffs{,l,ll}
/// - __builtin_parity{,l,ll}, __builtin_clrsb{,l,ll}
///
/// `eval_fn` is the caller's recursive eval_const_expr function.
pub fn eval_builtin_call(
    name: &str,
    args: &[Expr],
    eval_fn: &dyn Fn(&Expr) -> Option<IrConst>,
) -> Option<IrConst> {
    match name {
        "__builtin_choose_expr" if args.len() >= 3 => {
            let cond = eval_fn(&args[0])?;
            if cond.is_nonzero() {
                eval_fn(&args[1])
            } else {
                eval_fn(&args[2])
            }
        }
        "__builtin_constant_p" => {
            let is_const = args.first().is_some_and(|arg| eval_fn(arg).is_some());
            Some(IrConst::I32(if is_const { 1 } else { 0 }))
        }
        "__builtin_expect" | "__builtin_expect_with_probability" => {
            args.first().and_then(eval_fn)
        }
        "__builtin_bswap16" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u16;
            Some(IrConst::I32(v.swap_bytes() as i32))
        }
        "__builtin_bswap32" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u32;
            Some(IrConst::I32(v.swap_bytes() as i32))
        }
        "__builtin_bswap64" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u64;
            Some(IrConst::I64(v.swap_bytes() as i64))
        }
        // For 'l' suffix builtins, the operand width depends on the target:
        // LP64 (x86-64, ARM64, RISC-V 64): long is 64-bit
        // ILP32 (i686): long is 32-bit
        "__builtin_clz" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u32;
            Some(IrConst::I32(v.leading_zeros() as i32))
        }
        "__builtin_clzl" => {
            let val = eval_fn(args.first()?)?;
            if target_is_32bit() {
                let v = val.to_i64()? as u32;
                Some(IrConst::I32(v.leading_zeros() as i32))
            } else {
                let v = val.to_i64()? as u64;
                Some(IrConst::I32(v.leading_zeros() as i32))
            }
        }
        "__builtin_clzll" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u64;
            Some(IrConst::I32(v.leading_zeros() as i32))
        }
        "__builtin_ctz" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u32;
            if v == 0 { Some(IrConst::I32(32)) }
            else { Some(IrConst::I32(v.trailing_zeros() as i32)) }
        }
        "__builtin_ctzl" => {
            let val = eval_fn(args.first()?)?;
            if target_is_32bit() {
                let v = val.to_i64()? as u32;
                if v == 0 { Some(IrConst::I32(32)) }
                else { Some(IrConst::I32(v.trailing_zeros() as i32)) }
            } else {
                let v = val.to_i64()? as u64;
                if v == 0 { Some(IrConst::I32(64)) }
                else { Some(IrConst::I32(v.trailing_zeros() as i32)) }
            }
        }
        "__builtin_ctzll" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u64;
            if v == 0 { Some(IrConst::I32(64)) }
            else { Some(IrConst::I32(v.trailing_zeros() as i32)) }
        }
        "__builtin_popcount" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u32;
            Some(IrConst::I32(v.count_ones() as i32))
        }
        "__builtin_popcountl" => {
            let val = eval_fn(args.first()?)?;
            if target_is_32bit() {
                let v = val.to_i64()? as u32;
                Some(IrConst::I32(v.count_ones() as i32))
            } else {
                let v = val.to_i64()? as u64;
                Some(IrConst::I32(v.count_ones() as i32))
            }
        }
        "__builtin_popcountll" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u64;
            Some(IrConst::I32(v.count_ones() as i32))
        }
        "__builtin_ffs" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u32;
            if v == 0 { Some(IrConst::I32(0)) }
            else { Some(IrConst::I32(v.trailing_zeros() as i32 + 1)) }
        }
        "__builtin_ffsl" => {
            let val = eval_fn(args.first()?)?;
            if target_is_32bit() {
                let v = val.to_i64()? as u32;
                if v == 0 { Some(IrConst::I32(0)) }
                else { Some(IrConst::I32(v.trailing_zeros() as i32 + 1)) }
            } else {
                let v = val.to_i64()? as u64;
                if v == 0 { Some(IrConst::I32(0)) }
                else { Some(IrConst::I32(v.trailing_zeros() as i32 + 1)) }
            }
        }
        "__builtin_ffsll" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u64;
            if v == 0 { Some(IrConst::I32(0)) }
            else { Some(IrConst::I32(v.trailing_zeros() as i32 + 1)) }
        }
        "__builtin_parity" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u32;
            Some(IrConst::I32((v.count_ones() % 2) as i32))
        }
        "__builtin_parityl" => {
            let val = eval_fn(args.first()?)?;
            if target_is_32bit() {
                let v = val.to_i64()? as u32;
                Some(IrConst::I32((v.count_ones() % 2) as i32))
            } else {
                let v = val.to_i64()? as u64;
                Some(IrConst::I32((v.count_ones() % 2) as i32))
            }
        }
        "__builtin_parityll" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as u64;
            Some(IrConst::I32((v.count_ones() % 2) as i32))
        }
        "__builtin_clrsb" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()? as i32;
            let result = if v < 0 { (!v as u32).leading_zeros() as i32 - 1 }
                         else { (v as u32).leading_zeros() as i32 - 1 };
            Some(IrConst::I32(result))
        }
        "__builtin_clrsbl" => {
            let val = eval_fn(args.first()?)?;
            if target_is_32bit() {
                let v = val.to_i64()? as i32;
                let result = if v < 0 { (!v as u32).leading_zeros() as i32 - 1 }
                             else { (v as u32).leading_zeros() as i32 - 1 };
                Some(IrConst::I32(result))
            } else {
                let v = val.to_i64()?;
                let result = if v < 0 { (!v as u64).leading_zeros() as i32 - 1 }
                             else { (v as u64).leading_zeros() as i32 - 1 };
                Some(IrConst::I32(result))
            }
        }
        "__builtin_clrsbll" => {
            let val = eval_fn(args.first()?)?;
            let v = val.to_i64()?;
            let result = if v < 0 { (!v as u64).leading_zeros() as i32 - 1 }
                         else { (v as u64).leading_zeros() as i32 - 1 };
            Some(IrConst::I32(result))
        }
        // Float constant builtins: NaN, Infinity, huge_val
        // These are critical for static/global initializers like:
        //   volatile double d = NAN;  // expands to __builtin_nan("")
        //   double inf = INFINITY;    // expands to __builtin_inff()
        // Without these, eval_const_expr returns None and global_init.rs
        // falls through to GlobalInit::Zero, silently zero-initializing
        // the variable instead of storing NaN/Infinity.
        "__builtin_nan" => Some(IrConst::F64(f64::NAN)),
        "__builtin_nanf" => Some(IrConst::F32(f32::NAN)),
        "__builtin_nanl" => Some(IrConst::long_double(f64::NAN)),
        "__builtin_inf" => Some(IrConst::F64(f64::INFINITY)),
        "__builtin_inff" => Some(IrConst::F32(f32::INFINITY)),
        "__builtin_infl" => Some(IrConst::long_double(f64::INFINITY)),
        "__builtin_huge_val" => Some(IrConst::F64(f64::INFINITY)),
        "__builtin_huge_valf" => Some(IrConst::F32(f32::INFINITY)),
        "__builtin_huge_vall" => Some(IrConst::long_double(f64::INFINITY)),
        _ => None,
    }
}

/// Promote a sub-int IrConst (I8/I16) to I32 for unary arithmetic.
///
/// C11 6.3.1.1: Integer promotion converts unsigned char/short to int by
/// zero-extending, and signed char/short by sign-extending. Without this,
/// negate_const(I16(-1)) for unsigned short 65535 would compute -(-1) = 1
/// instead of -(65535) = -65535.
///
/// `is_unsigned` indicates whether the expression has an unsigned type.
#[inline]
pub fn promote_sub_int(val: IrConst, is_unsigned: bool) -> IrConst {
    match &val {
        IrConst::I8(v) => {
            if is_unsigned {
                IrConst::I32(*v as u8 as i32)
            } else {
                IrConst::I32(*v as i32)
            }
        }
        IrConst::I16(v) => {
            if is_unsigned {
                IrConst::I32(*v as u16 as i32)
            } else {
                IrConst::I32(*v as i32)
            }
        }
        _ => val,
    }
}

/// Evaluate a constant expression as raw u64 bits, with fallback to value conversion.
///
/// Used by `eval_const_expr_as_bits` for non-cast expressions. Converts the
/// IrConst result to its raw bit representation.
#[inline]
pub fn irconst_to_bits(val: &IrConst) -> (u64, bool) {
    let bits = match val {
        IrConst::F32(v) => *v as i64 as u64,
        IrConst::F64(v) => *v as i64 as u64,
        _ => val.to_i64().unwrap_or(0) as u64,
    };
    (bits, true) // default to signed
}

/// Evaluate a binary operation on constant operands with given type parameters.
///
/// This wraps `const_arith::eval_const_binop` with the C usual arithmetic
/// conversion logic (C11 6.3.1.8). For shifts, only the LHS type determines
/// the result type (C11 6.5.7); for other ops, use the wider of both types.
///
/// Parameters:
/// - `lhs_size`: byte size of LHS type (minimum 4 after integer promotion)
/// - `lhs_unsigned`: whether LHS type is unsigned
/// - `rhs_size`: byte size of RHS type (minimum 4 after integer promotion)
/// - `rhs_unsigned`: whether RHS type is unsigned
pub fn eval_binop_with_types(
    op: &BinOp,
    lhs: &IrConst,
    rhs: &IrConst,
    lhs_size: usize,
    lhs_unsigned: bool,
    rhs_size: usize,
    rhs_unsigned: bool,
) -> Option<IrConst> {
    let is_shift = matches!(op, BinOp::Shl | BinOp::Shr);

    // For shifts (C11 6.5.7): result type is the promoted LHS type only.
    // For other ops: apply C's usual arithmetic conversions using both operand types.
    let (is_32bit, is_unsigned) = if is_shift {
        (lhs_size <= 4, lhs_unsigned)
    } else {
        let result_size = lhs_size.max(rhs_size);
        let is_unsigned = if lhs_size == rhs_size {
            lhs_unsigned || rhs_unsigned
        } else if lhs_size > rhs_size {
            lhs_unsigned
        } else {
            rhs_unsigned
        };
        (result_size <= 4, is_unsigned)
    };
    const_arith::eval_const_binop(op, lhs, rhs, is_32bit, is_unsigned, lhs_unsigned, rhs_unsigned)
}


// ============================================================================
// Property-based tests (pi-pbt campaign, round 01_common)
// ============================================================================

#[cfg(test)]
mod pbt_tests {
    use super::*;
    use crate::common::source::Span;
    use crate::frontend::parser::ast::Expr;
    use proptest::prelude::*;

    fn builtin64(name: &str, v: i64) -> Option<i64> {
        let arg = if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
            Expr::IntLiteral(v, Span::dummy())
        } else {
            Expr::LongLongLiteral(v, Span::dummy())
        };
        eval_builtin_call(name, &[arg], &|e| eval_literal(e)).and_then(|c| c.to_i64())
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]

        /// P12: __builtin bit operations agree with GCC semantics
        /// (== Rust native leading_zeros/trailing_zeros/count_ones/swap_bytes).
        #[test]
        fn pbt_p12_builtin_bitops(v in any::<i64>()) {
            // clz/ctz on `unsigned int` — 0 is undefined in GCC, excluded from domain.
            prop_assume!((v as u32) != 0 || (v as u64) != 0);
            if (v as u32) != 0 {
                prop_assert_eq!(builtin64("__builtin_clz", v), Some((v as u32).leading_zeros() as i64), "clz v={}", v);
                prop_assert_eq!(builtin64("__builtin_ctz", v), Some((v as u32).trailing_zeros() as i64), "ctz v={}", v);
                prop_assert_eq!(builtin64("__builtin_ffs", v), Some((v as u32).trailing_zeros() as i64 + 1), "ffs v={}", v);
            }
            prop_assert_eq!(builtin64("__builtin_clzll", v), Some((v as u64).leading_zeros() as i64), "clzll v={}", v);
            if (v as u64) != 0 {
                prop_assert_eq!(builtin64("__builtin_ctzll", v), Some((v as u64).trailing_zeros() as i64), "ctzll v={}", v);
            }
            prop_assert_eq!(builtin64("__builtin_popcountll", v), Some((v as u64).count_ones() as i64), "popcountll v={}", v);
            prop_assert_eq!(builtin64("__builtin_popcount", v), Some((v as u32).count_ones() as i64), "popcount v={}", v);
            prop_assert_eq!(builtin64("__builtin_parity", v), Some(((v as u32).count_ones() % 2) as i64), "parity v={}", v);
            prop_assert_eq!(builtin64("__builtin_parityll", v), Some(((v as u64).count_ones() % 2) as i64), "parityll v={}", v);

            // bswap: reference semantics + involution.
            prop_assert_eq!(builtin64("__builtin_bswap16", v), Some((v as u16).swap_bytes() as i32 as i64), "bswap16 v={}", v);
            prop_assert_eq!(builtin64("__builtin_bswap32", v), Some((v as u32).swap_bytes() as i32 as i64), "bswap32 v={}", v);
            prop_assert_eq!(builtin64("__builtin_bswap64", v), Some((v as u64).swap_bytes() as i64), "bswap64 v={}", v);
            let b16 = builtin64("__builtin_bswap16", builtin64("__builtin_bswap16", v).unwrap()).unwrap();
            prop_assert_eq!(b16, (v as u16) as i64);
            let b64 = builtin64("__builtin_bswap64", builtin64("__builtin_bswap64", v).unwrap()).unwrap();
            prop_assert_eq!(b64, v);
        }

        /// P12b: __builtin_bswap* match Rust swap_bytes (GCC semantics) and are
        /// involutions. Unsigned 32-bit results must be zero-extended per the
        /// module's own constant-representation rule (const_arith.rs:88-96).
        #[test]
        fn pbt_p12b_bswap_semantics(v in any::<i64>()) {
            prop_assert_eq!(builtin64("__builtin_bswap16", v), Some((v as u16).swap_bytes() as i64), "bswap16 v={}", v);
            prop_assert_eq!(builtin64("__builtin_bswap32", v), Some((v as u32).swap_bytes() as u64 as i64), "bswap32 v={}", v);
            prop_assert_eq!(builtin64("__builtin_bswap64", v), Some((v as u64).swap_bytes() as i64), "bswap64 v={}", v);
            let b32 = builtin64("__builtin_bswap32", builtin64("__builtin_bswap32", v).unwrap()).unwrap();
            prop_assert_eq!(b32, (v as u32) as i64, "bswap32 involution v={}", v);
        }
    }

    /// P12b: __builtin_ffs(0) == 0 (GCC-documented).
    #[test]
    fn pbt_p12_ffs_zero() {
        assert_eq!(builtin64("__builtin_ffs", 0), Some(0));
    }
}

#[cfg(test)]
mod pbt_regression {
    use super::*;
    use crate::common::source::Span;
    use crate::frontend::parser::ast::Expr;

    fn builtin64(name: &str, v: i64) -> Option<i64> {
        let arg = if v >= i32::MIN as i64 && v <= i32::MAX as i64 {
            Expr::IntLiteral(v, Span::dummy())
        } else {
            Expr::LongLongLiteral(v, Span::dummy())
        };
        eval_builtin_call(name, &[arg], &|e| eval_literal(e)).and_then(|c| c.to_i64())
    }

    /// B4: __builtin_bswap32 returns unsigned int; its folded constant must be
    /// zero-extended (per the module's own rule in const_arith.rs:88-96), but it
    /// is stored as signed I32 and reads back negative.
    #[test]
    #[ignore = "SUT bug B4: __builtin_bswap32 fold uses signed I32 for unsigned >= 2^31"]
    fn test_eval_builtin_bswap32_regression_unsigned_repr() {
        let v: i64 = -5495501120125551105; // low 32 bits >= 2^31 after bswap
        assert_eq!(
            builtin64("__builtin_bswap32", v),
            Some((v as u32).swap_bytes() as u64 as i64)
        );
    }
}
