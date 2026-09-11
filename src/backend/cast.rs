//! Shared cast and float operation classification, plus F128 soft-float libcall mapping.
//!
//! All four backends use the same decision logic to determine what kind of cast
//! to emit — only the actual machine instructions differ. By classifying the cast
//! once in shared code, we eliminate duplicated Ptr-normalization and F128-reduction
//! logic from each backend. This module also provides the shared mnemonic-to-libcall
//! mapping for F128 soft-float arithmetic and comparisons (ARM, RISC-V).

use crate::common::types::IrType;
use crate::ir::reexports::{IrBinOp, IrCmpOp, IrConst, Operand};

/// Classification of type casts. All four backends use the same control flow
/// to decide which kind of cast to emit; only the actual instructions differ.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastKind {
    /// No conversion needed (same type, or Ptr <-> I64/U64, or F128 <-> F64).
    Noop,
    /// Float to signed integer (from_ty is F32 or F64).
    FloatToSigned { from_f64: bool },
    /// Float to unsigned integer (from_ty is F32 or F64).
    FloatToUnsigned { from_f64: bool, to_u64: bool },
    /// Signed integer to float (to_ty is F32 or F64).
    /// `from_ty` is the source integer type, needed to sign-extend sub-64-bit values
    /// before conversion (e.g., I32 in rax must be sign-extended to 64 bits).
    SignedToFloat { to_f64: bool, from_ty: IrType },
    /// Unsigned integer to float. `from_ty` is the source unsigned integer type,
    /// needed for proper zero-extension on RISC-V (where W-suffix instructions
    /// sign-extend) and for U64 overflow handling on x86.
    UnsignedToFloat { to_f64: bool, from_ty: IrType },
    /// Float-to-float conversion (F32 <-> F64).
    FloatToFloat { widen: bool },
    /// Integer widening: sign- or zero-extend a smaller type to a larger one.
    IntWiden { from_ty: IrType, to_ty: IrType },
    /// Integer narrowing: truncate a larger type to a smaller one.
    IntNarrow { to_ty: IrType },
    /// Same-size signed-to-unsigned (need to mask/clear upper bits).
    SignedToUnsignedSameSize { to_ty: IrType },
    /// Same-size unsigned-to-signed. On most architectures this is a noop,
    /// but on RISC-V 64-bit, U32->I32 needs sign-extension because the ABI
    /// requires all 32-bit values to be sign-extended in 64-bit registers.
    UnsignedToSignedSameSize { to_ty: IrType },
    /// Signed integer -> F128 via softfloat (__floatsitf / __floatditf).
    /// Used on ARM/RISC-V where long double is IEEE binary128.
    SignedToF128 { from_ty: IrType },
    /// Unsigned integer -> F128 via softfloat (__floatunsitf / __floatunditf).
    UnsignedToF128 { from_ty: IrType },
    /// F128 -> signed integer via softfloat (__fixtfsi / __fixtfdi).
    F128ToSigned { to_ty: IrType },
    /// F128 -> unsigned integer via softfloat (__fixunstfsi / __fixunstfdi).
    F128ToUnsigned { to_ty: IrType },
    /// F32/F64 -> F128 widening via softfloat (__extendsftf2 / __extenddftf2).
    FloatToF128 { from_f32: bool },
    /// F128 -> F32/F64 narrowing via softfloat (__trunctfsf2 / __trunctfdf2).
    F128ToFloat { to_f32: bool },
}

/// Classify a cast between two IR types. This captures the shared decision logic
/// that all four backends use identically. Backends then match on the returned
/// `CastKind` to emit architecture-specific instructions.
///
/// Handles Ptr normalization (Ptr treated as U64) and F128 reduction (F128 treated
/// as F64 for computation purposes on x86) before classification.
///
/// `f128_is_native`: true on ARM/RISC-V where F128 is IEEE binary128 and requires
/// softfloat library calls for conversions. false on x86 where F128 is x87 80-bit
/// and is approximated as F64.
pub fn classify_cast_with_f128(from_ty: IrType, to_ty: IrType, f128_is_native: bool) -> CastKind {
    if from_ty == to_ty {
        return CastKind::Noop;
    }

    // F128 (long double) handling depends on architecture.
    if from_ty == IrType::F128 || to_ty == IrType::F128 {
        if f128_is_native {
            // ARM/RISC-V: F128 is true IEEE binary128. Use softfloat library calls.
            return classify_f128_cast_native(from_ty, to_ty);
        }
        // x86: F128 (x87 80-bit) is computed as F64. Treat F128 <-> F64 as noop,
        // and F128 <-> other as F64 <-> other.
        let effective_from = if from_ty == IrType::F128 { IrType::F64 } else { from_ty };
        let effective_to = if to_ty == IrType::F128 { IrType::F64 } else { to_ty };
        if effective_from == effective_to {
            return CastKind::Noop;
        }
        return classify_cast(effective_from, effective_to);
    }

    // Ptr is equivalent to U64 on LP64 targets, U32 on ILP32 targets.
    if (from_ty == IrType::Ptr || to_ty == IrType::Ptr) && !from_ty.is_float() && !to_ty.is_float() {
        let ptr_int_ty = if crate::common::types::target_is_32bit() { IrType::U32 } else { IrType::U64 };
        let effective_from = if from_ty == IrType::Ptr { ptr_int_ty } else { from_ty };
        let effective_to = if to_ty == IrType::Ptr { ptr_int_ty } else { to_ty };
        let ptr_sz = crate::common::types::target_ptr_size();
        if effective_from == effective_to || (effective_from.size() == ptr_sz && effective_to.size() == ptr_sz) {
            return CastKind::Noop;
        }
        return classify_cast(effective_from, effective_to);
    }

    // Float-to-int
    if from_ty.is_float() && !to_ty.is_float() {
        let is_unsigned_dest = to_ty.is_unsigned() || to_ty == IrType::Ptr;
        let from_f64 = from_ty == IrType::F64;
        if is_unsigned_dest {
            let to_u64 = to_ty == IrType::U64 || to_ty == IrType::Ptr;
            return CastKind::FloatToUnsigned { from_f64, to_u64 };
        } else {
            return CastKind::FloatToSigned { from_f64 };
        }
    }

    // Int-to-float
    if !from_ty.is_float() && to_ty.is_float() {
        let is_unsigned_src = from_ty.is_unsigned();
        let to_f64 = to_ty == IrType::F64;
        if is_unsigned_src {
            return CastKind::UnsignedToFloat { to_f64, from_ty };
        } else {
            return CastKind::SignedToFloat { to_f64, from_ty };
        }
    }

    // Float-to-float
    if from_ty.is_float() && to_ty.is_float() {
        let widen = from_ty == IrType::F32 && to_ty == IrType::F64;
        return CastKind::FloatToFloat { widen };
    }

    // Integer-to-integer
    let from_size = from_ty.size();
    let to_size = to_ty.size();

    if from_size == to_size {
        if from_ty.is_signed() && to_ty.is_unsigned() {
            return CastKind::SignedToUnsignedSameSize { to_ty };
        }
        if from_ty.is_unsigned() && to_ty.is_signed() {
            return CastKind::UnsignedToSignedSameSize { to_ty };
        }
        return CastKind::Noop;
    }

    if to_size > from_size {
        return CastKind::IntWiden { from_ty, to_ty };
    }

    CastKind::IntNarrow { to_ty }
}

/// Backward-compatible wrapper: classifies casts with x86 F128 semantics
/// (F128 treated as F64 approximation).
pub fn classify_cast(from_ty: IrType, to_ty: IrType) -> CastKind {
    classify_cast_with_f128(from_ty, to_ty, false)
}

/// Classify F128 casts on targets where F128 is true IEEE binary128 (ARM/RISC-V).
/// These require softfloat library calls for full precision.
fn classify_f128_cast_native(from_ty: IrType, to_ty: IrType) -> CastKind {
    debug_assert!(from_ty == IrType::F128 || to_ty == IrType::F128);

    if to_ty == IrType::F128 {
        // Something -> F128
        if from_ty == IrType::F64 {
            return CastKind::FloatToF128 { from_f32: false };
        }
        if from_ty == IrType::F32 {
            return CastKind::FloatToF128 { from_f32: true };
        }
        // Integer -> F128
        if from_ty.is_float() {
            // F128 -> F128: should not happen (handled by from_ty == to_ty check)
            return CastKind::Noop;
        }
        if from_ty.is_unsigned() {
            return CastKind::UnsignedToF128 { from_ty };
        }
        return CastKind::SignedToF128 { from_ty };
    }

    // F128 -> something
    if to_ty == IrType::F64 {
        return CastKind::F128ToFloat { to_f32: false };
    }
    if to_ty == IrType::F32 {
        return CastKind::F128ToFloat { to_f32: true };
    }
    // F128 -> integer
    if to_ty.is_float() {
        return CastKind::Noop;
    }
    if to_ty.is_unsigned() || to_ty == IrType::Ptr {
        return CastKind::F128ToUnsigned { to_ty };
    }
    CastKind::F128ToSigned { to_ty }
}

/// Float arithmetic operations that all four backends support.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FloatOp {
    Add,
    Sub,
    Mul,
    Div,
}

/// Classify a binary operation on floats. Returns None if the operation is not
/// meaningful on floats (e.g., bitwise And, Or, Xor, shifts, integer remainder).
pub fn classify_float_binop(op: IrBinOp) -> Option<FloatOp> {
    match op {
        IrBinOp::Add => Some(FloatOp::Add),
        IrBinOp::Sub => Some(FloatOp::Sub),
        IrBinOp::Mul => Some(FloatOp::Mul),
        IrBinOp::SDiv | IrBinOp::UDiv => Some(FloatOp::Div),
        _ => None,
    }
}

/// Map a float binop mnemonic (fadd/fsub/fmul/fdiv) to the corresponding F128
/// soft-float libcall. Used by ARM and RISC-V backends (x86 uses x87 for F128).
/// Returns None for unrecognized mnemonics (caller should fall back to f64 hardware).
/// How to interpret an F128 comparison libcall result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum F128CmpKind {
    /// Result == 0 means true (equality)
    Eq,
    /// Result != 0 means true (inequality)
    Ne,
    /// Result < 0 means true (less than)
    Lt,
    /// Result <= 0 means true (less or equal)
    Le,
    /// Result > 0 means true (greater than)
    Gt,
    /// Result >= 0 means true (greater or equal)
    Ge,
}

/// Map a comparison operation to the F128 soft-float libcall and result interpretation.
pub fn f128_cmp_libcall(op: IrCmpOp) -> (&'static str, F128CmpKind) {
    match op {
        IrCmpOp::Eq => ("__eqtf2", F128CmpKind::Eq),
        IrCmpOp::Ne => ("__eqtf2", F128CmpKind::Ne),
        IrCmpOp::Slt | IrCmpOp::Ult => ("__lttf2", F128CmpKind::Lt),
        IrCmpOp::Sle | IrCmpOp::Ule => ("__letf2", F128CmpKind::Le),
        IrCmpOp::Sgt | IrCmpOp::Ugt => ("__gttf2", F128CmpKind::Gt),
        IrCmpOp::Sge | IrCmpOp::Uge => ("__getf2", F128CmpKind::Ge),
    }
}

/// Extract the IEEE f128 low/high u64 halves from an F128 constant operand.
/// The f128 bytes are already in IEEE binary128 format.
/// Returns None for non-constant operands (caller must use runtime conversion).
pub fn f128_const_halves(op: &Operand) -> Option<(u64, u64)> {
    if let Operand::Const(IrConst::LongDouble(_, f128_bytes)) = op {
        let lo = u64::from_le_bytes(f128_bytes[0..8].try_into().unwrap());
        let hi = u64::from_le_bytes(f128_bytes[8..16].try_into().unwrap());
        Some((lo, hi))
    } else {
        None
    }
}

#[cfg(test)]
mod classify_cast_with_f128_pbt {
    // Oracle: algebraic.invariant / algebraic.metamorphic
    // Evidence: CastKind::Noop docs (src/backend/cast.rs:16-17); F128-as-F64
    //   (src/backend/cast.rs:61-62, :80-81; src/backend/README.md:649-650);
    //   Ptr-as-U64/U32 (src/backend/cast.rs:61, :88-89; README:649,652;
    //   src/common/types.rs:20-21); native F128 kinds (cast.rs:42-54;
    //   README:662-668); float/int and widen/narrow (cast.rs:18-41;
    //   README:652-661). f128_is_native gated on F128 endpoints (cast.rs:72-73).
    // Stronger considered:
    //   - State machine: rejected — pure function, no lifecycle/state
    //   - Differential: rejected — classify_cast is a same-source wrapper
    //     (cast.rs:151-154); f128_emit_cast emits libcalls, not CastKind
    //     (different job; ARM uses it instead of this classifier)
    //   - Algebraic round-trip / idempotence: rejected — not an inverse pair
    //     or normalizer
    // Weaker available: crash_only
    // SUT-boundary: internal-helper (shared backend classifier)

    use super::{classify_cast_with_f128, CastKind};
    use crate::common::types::{set_target_ptr_size, target_ptr_size, IrType};
    use proptest::prelude::*;

    const CASES: u32 = 1000;

    fn cfg() -> ProptestConfig {
        ProptestConfig::with_cases(CASES)
    }

    const ALL_IR: &[IrType] = &[
        IrType::I8,
        IrType::I16,
        IrType::I32,
        IrType::I64,
        IrType::I128,
        IrType::U8,
        IrType::U16,
        IrType::U32,
        IrType::U64,
        IrType::U128,
        IrType::F32,
        IrType::F64,
        IrType::F128,
        IrType::Ptr,
        IrType::Void,
    ];

    const INT_IR: &[IrType] = &[
        IrType::I8,
        IrType::I16,
        IrType::I32,
        IrType::I64,
        IrType::I128,
        IrType::U8,
        IrType::U16,
        IrType::U32,
        IrType::U64,
        IrType::U128,
    ];

    const INT_OR_PTR: &[IrType] = &[
        IrType::I8,
        IrType::I16,
        IrType::I32,
        IrType::I64,
        IrType::I128,
        IrType::U8,
        IrType::U16,
        IrType::U32,
        IrType::U64,
        IrType::U128,
        IrType::Ptr,
    ];

    const NON_F128: &[IrType] = &[
        IrType::I8,
        IrType::I16,
        IrType::I32,
        IrType::I64,
        IrType::I128,
        IrType::U8,
        IrType::U16,
        IrType::U32,
        IrType::U64,
        IrType::U128,
        IrType::F32,
        IrType::F64,
        IrType::Ptr,
        IrType::Void,
    ];

    fn ir_types() -> proptest::sample::Select<IrType> {
        proptest::sample::select(ALL_IR)
    }

    fn int_types() -> proptest::sample::Select<IrType> {
        proptest::sample::select(INT_IR)
    }

    fn int_or_ptr() -> proptest::sample::Select<IrType> {
        proptest::sample::select(INT_OR_PTR)
    }

    fn non_f128() -> proptest::sample::Select<IrType> {
        proptest::sample::select(NON_F128)
    }

    struct PtrSizeGuard {
        prev: usize,
    }
    impl Drop for PtrSizeGuard {
        fn drop(&mut self) {
            set_target_ptr_size(self.prev);
        }
    }

    fn with_ptr_size<R>(size: usize, f: impl FnOnce() -> R) -> R {
        let prev = target_ptr_size();
        set_target_ptr_size(size);
        let _g = PtrSizeGuard { prev };
        f()
    }

    fn ptr_int_ty(ptr_size: usize) -> IrType {
        if ptr_size == 4 {
            IrType::U32
        } else {
            IrType::U64
        }
    }

    fn replace_ptr(ty: IrType, u: IrType) -> IrType {
        if ty == IrType::Ptr {
            u
        } else {
            ty
        }
    }

    fn as_f64_if_f128(ty: IrType) -> IrType {
        if ty == IrType::F128 {
            IrType::F64
        } else {
            ty
        }
    }

    fn expected_int_cast(from: IrType, to: IrType) -> CastKind {
        if from == to {
            return CastKind::Noop;
        }
        let fs = from.size();
        let ts = to.size();
        if ts > fs {
            CastKind::IntWiden {
                from_ty: from,
                to_ty: to,
            }
        } else if ts < fs {
            CastKind::IntNarrow { to_ty: to }
        } else if from.is_signed() && to.is_unsigned() {
            CastKind::SignedToUnsignedSameSize { to_ty: to }
        } else if from.is_unsigned() && to.is_signed() {
            CastKind::UnsignedToSignedSameSize { to_ty: to }
        } else {
            CastKind::Noop
        }
    }

    // Oracle: algebraic.invariant
    // Evidence: src/backend/cast.rs:16-17; src/backend/README.md:652
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_identity_is_noop(ty in ir_types(), native in any::<bool>()) {
            prop_assert_eq!(
                classify_cast_with_f128(ty, ty, native),
                CastKind::Noop,
                "identity classify({:?}, {:?}, {})",
                ty, ty, native
            );
        }
    }

    // Oracle: algebraic.metamorphic
    // Evidence: src/backend/cast.rs:61-62, :80-81; src/backend/README.md:649-650
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: algebraic.invariant, crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_f128_non_native_reduces_to_f64(
            from in ir_types(),
            to in ir_types(),
            // Skew toward F128 endpoints so the reduction path is hit, not only identity.
            force_from_f128 in any::<bool>(),
            force_to_f128 in any::<bool>(),
        ) {
            let from = if force_from_f128 { IrType::F128 } else { from };
            let to = if force_to_f128 { IrType::F128 } else { to };
            let got = classify_cast_with_f128(from, to, false);
            let expected = classify_cast_with_f128(as_f64_if_f128(from), as_f64_if_f128(to), false);
            prop_assert_eq!(
                got, expected,
                "F128-as-F64 reduction: classify({:?}, {:?}, false)={:?} != classify({:?}, {:?}, false)={:?}",
                from, to, got, as_f64_if_f128(from), as_f64_if_f128(to), expected
            );
        }
    }

    // Oracle: algebraic.metamorphic
    // Evidence: src/backend/cast.rs:72-73
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_native_flag_irrelevant_without_f128(
            from in non_f128(),
            to in non_f128(),
        ) {
            let a = classify_cast_with_f128(from, to, true);
            let b = classify_cast_with_f128(from, to, false);
            prop_assert_eq!(
                a, b,
                "native flag changed result for non-F128 {:?}->{:?}: {:?} vs {:?}",
                from, to, a, b
            );
        }
    }

    // Oracle: algebraic.metamorphic
    // Evidence: src/backend/cast.rs:16-17, :61, :88-89; README:649,652; types.rs:20-21
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: algebraic.invariant, crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_ptr_normalized_as_unsigned_int(
            from in ir_types(),
            to in ir_types(),
            native in any::<bool>(),
            ptr_size in prop_oneof![Just(4usize), Just(8usize)],
            force_from_ptr in any::<bool>(),
            force_to_ptr in any::<bool>(),
        ) {
            let from = if force_from_ptr { IrType::Ptr } else { from };
            let to = if force_to_ptr { IrType::Ptr } else { to };
            prop_assume!(from == IrType::Ptr || to == IrType::Ptr);

            let u = ptr_int_ty(ptr_size);
            let from2 = replace_ptr(from, u);
            let to2 = replace_ptr(to, u);
            let (got, expected, expect_noop) = with_ptr_size(ptr_size, || {
                let got = classify_cast_with_f128(from, to, native);
                let expected = classify_cast_with_f128(from2, to2, native);
                let expect_noop = !from.is_float()
                    && !to.is_float()
                    && from2.size() == ptr_size
                    && to2.size() == ptr_size;
                (got, expected, expect_noop)
            });
            if expect_noop {
                prop_assert_eq!(
                    got, CastKind::Noop,
                    "Ptr <-> pointer-width int should be Noop: {:?}->{:?} native={} ptr_size={} got={:?}",
                    from, to, native, ptr_size, got
                );
            } else {
                prop_assert_eq!(
                    got, expected,
                    "Ptr-as-{:?} failed: classify({:?}, {:?}, {})={:?} != classify({:?}, {:?}, {})={:?} (ptr_size={})",
                    u, from, to, native, got, from2, to2, native, expected, ptr_size
                );
            }
        }
    }

    // Oracle: algebraic.invariant
    // Evidence: src/backend/cast.rs:51-54; src/backend/README.md:662-668
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_native_f128_float_kinds(
            from_f32 in any::<bool>(),
            to_f32 in any::<bool>(),
        ) {
            let src = if from_f32 { IrType::F32 } else { IrType::F64 };
            let dst = if to_f32 { IrType::F32 } else { IrType::F64 };
            let up = classify_cast_with_f128(src, IrType::F128, true);
            prop_assert_eq!(
                up,
                CastKind::FloatToF128 { from_f32 },
                "{:?} -> F128 native",
                src
            );
            let down = classify_cast_with_f128(IrType::F128, dst, true);
            prop_assert_eq!(
                down,
                CastKind::F128ToFloat { to_f32 },
                "F128 -> {:?} native",
                dst
            );
        }
    }

    // Oracle: algebraic.invariant
    // Evidence: src/backend/cast.rs:42-50, :61, :88-89
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_native_f128_int_signedness(
            ty in int_or_ptr(),
            ptr_size in prop_oneof![Just(4usize), Just(8usize)],
        ) {
            let u = if ty == IrType::Ptr {
                ptr_int_ty(ptr_size)
            } else {
                ty
            };
            let (to_f128, from_f128) = with_ptr_size(ptr_size, || {
                (
                    classify_cast_with_f128(ty, IrType::F128, true),
                    classify_cast_with_f128(IrType::F128, ty, true),
                )
            });
            if ty.is_unsigned() || ty == IrType::Ptr {
                prop_assert_eq!(
                    to_f128,
                    CastKind::UnsignedToF128 { from_ty: u },
                    "{:?} -> F128 native (ptr_size={})",
                    ty, ptr_size
                );
                prop_assert_eq!(
                    from_f128,
                    CastKind::F128ToUnsigned { to_ty: u },
                    "F128 -> {:?} native (ptr_size={})",
                    ty, ptr_size
                );
            } else {
                prop_assert_eq!(
                    to_f128,
                    CastKind::SignedToF128 { from_ty: ty },
                    "{:?} -> F128 native",
                    ty
                );
                prop_assert_eq!(
                    from_f128,
                    CastKind::F128ToSigned { to_ty: ty },
                    "F128 -> {:?}",
                    ty
                );
            }
        }
    }

    // Oracle: algebraic.invariant
    // Evidence: src/backend/cast.rs:18-29, :61, :88-89
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_float_int_kinds(
            int_ty in int_or_ptr(),
            is_f64 in any::<bool>(),
            native in any::<bool>(),
            ptr_size in prop_oneof![Just(4usize), Just(8usize)],
            int_to_float in any::<bool>(),
        ) {
            let ft = if is_f64 { IrType::F64 } else { IrType::F32 };
            let u = if int_ty == IrType::Ptr {
                ptr_int_ty(ptr_size)
            } else {
                int_ty
            };
            let unsigned = int_ty.is_unsigned() || int_ty == IrType::Ptr;
            let (got, expected) = with_ptr_size(ptr_size, || {
                if int_to_float {
                    let expected = if unsigned {
                        CastKind::UnsignedToFloat {
                            to_f64: is_f64,
                            from_ty: u,
                        }
                    } else {
                        CastKind::SignedToFloat {
                            to_f64: is_f64,
                            from_ty: int_ty,
                        }
                    };
                    (classify_cast_with_f128(int_ty, ft, native), expected)
                } else {
                    let expected = if unsigned {
                        let to_u64 = u == IrType::U64;
                        CastKind::FloatToUnsigned {
                            from_f64: is_f64,
                            to_u64,
                        }
                    } else {
                        CastKind::FloatToSigned {
                            from_f64: is_f64,
                        }
                    };
                    (classify_cast_with_f128(ft, int_ty, native), expected)
                }
            });
            prop_assert_eq!(
                got, expected,
                "float/int {:?}<->{:?} native={} ptr_size={} int_to_float={}",
                int_ty, ft, native, ptr_size, int_to_float
            );
        }
    }

    /// Ptr -> F32 must be unsigned (Ptr ≡ U64 on LP64).
    #[test]
    fn test_classify_cast_with_f128_regression_ptr_to_f32_unsigned() {
        assert_eq!(
            classify_cast_with_f128(IrType::Ptr, IrType::F32, false),
            CastKind::UnsignedToFloat {
                to_f64: false,
                from_ty: IrType::U64,
            },
            "Ptr -> F32 must match U64 -> F32 on LP64"
        );
    }

    /// Ptr -> F128 native must be unsigned (Ptr ≡ U32 on ILP32 / U64 on LP64).
    #[test]
    fn test_classify_cast_with_f128_regression_ptr_to_f128_unsigned() {
        let got = with_ptr_size(4, || classify_cast_with_f128(IrType::Ptr, IrType::F128, true));
        assert_eq!(
            got,
            CastKind::UnsignedToF128 {
                from_ty: IrType::U32,
            },
            "Ptr -> F128 native must match U32 -> F128 on ILP32"
        );
    }

    /// F32 -> Ptr on ILP32 must use the U32 (to_u64=false) unsigned path.
    #[test]
    fn test_classify_cast_with_f128_regression_f32_to_ptr_ilp32() {
        let got = with_ptr_size(4, || classify_cast_with_f128(IrType::F32, IrType::Ptr, false));
        assert_eq!(
            got,
            CastKind::FloatToUnsigned {
                from_f64: false,
                to_u64: false,
            },
            "F32 -> Ptr on ILP32 must match F32 -> U32"
        );
    }

    // Oracle: algebraic.invariant
    // Evidence: src/backend/cast.rs:32-41; src/backend/README.md:656-661
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_int_widen_narrow_same_size(
            from in int_types(),
            to in int_types(),
            native in any::<bool>(),
        ) {
            let got = classify_cast_with_f128(from, to, native);
            let expected = expected_int_cast(from, to);
            prop_assert_eq!(
                got, expected,
                "int cast {:?}->{:?} native={}",
                from, to, native
            );
        }
    }

    // Oracle: algebraic.invariant
    // Evidence: src/backend/cast.rs:30-31 (FloatToFloat F32 <-> F64); README:655
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    // Contract-surface sweep: widen flag was not asserted by the first batch.
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_float_to_float_widen(native in any::<bool>()) {
            prop_assert_eq!(
                classify_cast_with_f128(IrType::F32, IrType::F64, native),
                CastKind::FloatToFloat { widen: true }
            );
            prop_assert_eq!(
                classify_cast_with_f128(IrType::F64, IrType::F32, native),
                CastKind::FloatToFloat { widen: false }
            );
        }
    }

    fn is_f128_libcall_kind(k: CastKind) -> bool {
        matches!(
            k,
            CastKind::SignedToF128 { .. }
                | CastKind::UnsignedToF128 { .. }
                | CastKind::F128ToSigned { .. }
                | CastKind::F128ToUnsigned { .. }
                | CastKind::FloatToF128 { .. }
                | CastKind::F128ToFloat { .. }
        )
    }

    // Oracle: algebraic.invariant
    // Evidence: src/backend/arm/codegen/cast_ops.rs:128
    //   unreachable!("F128 cast variants not produced by classify_cast()");
    //   classify_cast is classify_cast_with_f128(..., false) (cast.rs:151-154).
    // Stronger considered: state machine / differential rejected (see module comment)
    // Weaker available: crash_only
    // Contract-surface sweep: non-native must not emit native-F128 kinds.
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_non_native_never_f128_libcall_kinds(
            from in ir_types(),
            to in ir_types(),
        ) {
            let k = classify_cast_with_f128(from, to, false);
            prop_assert!(
                !is_f128_libcall_kind(k),
                "non-native classify({:?}, {:?}) produced F128 libcall kind {:?}",
                from, to, k
            );
        }
    }

    // Oracle: algebraic.invariant
    // Evidence: CastKind::Noop "Ptr <-> I64/U64" (cast.rs:16-17; README:652);
    //   Ptr ≡ U32 on ILP32 (cast.rs:88-89).
    // Stronger considered: the broader Ptr-normalization metamorphic (failing on float).
    // Weaker available: crash_only
    // Contract-surface sweep: pin the integer-Ptr Noop exception separately.
    proptest! {
        #![proptest_config(cfg())]
        #[test]
        fn classify_cast_ptr_pointer_width_is_noop(
            native in any::<bool>(),
            ptr_size in prop_oneof![Just(4usize), Just(8usize)],
            signed in any::<bool>(),
            ptr_on_left in any::<bool>(),
        ) {
            let int_ty = if ptr_size == 4 {
                if signed { IrType::I32 } else { IrType::U32 }
            } else if signed {
                IrType::I64
            } else {
                IrType::U64
            };
            let (from, to) = if ptr_on_left {
                (IrType::Ptr, int_ty)
            } else {
                (int_ty, IrType::Ptr)
            };
            let got = with_ptr_size(ptr_size, || classify_cast_with_f128(from, to, native));
            prop_assert_eq!(
                got, CastKind::Noop,
                "Ptr <-> {:?} should be Noop at ptr_size={} native={}",
                int_ty, ptr_size, native
            );
        }
    }
}
