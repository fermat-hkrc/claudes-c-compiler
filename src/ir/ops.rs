//! IR operation enums: binary, unary, comparison, and atomic operations.
//!
//! Each enum carries its own evaluation methods (eval_i64, eval_i128, eval_f64)
//! for use by constant folding and simplification passes.

/// Atomic read-modify-write operations.
#[derive(Debug, Clone, Copy)]
pub enum AtomicRmwOp {
    /// Add: *ptr += val
    Add,
    /// Sub: *ptr -= val
    Sub,
    /// And: *ptr &= val
    And,
    /// Or: *ptr |= val
    Or,
    /// Xor: *ptr ^= val
    Xor,
    /// Nand: *ptr = ~(*ptr & val)
    Nand,
    /// Exchange: *ptr = val (returns old value)
    Xchg,
    /// Test and set: *ptr = 1 (returns old value)
    TestAndSet,
}

/// Memory ordering for atomic operations.
#[derive(Debug, Clone, Copy)]
pub enum AtomicOrdering {
    Relaxed,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
}

/// Binary operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrBinOp {
    Add,
    Sub,
    Mul,
    SDiv,
    UDiv,
    SRem,
    URem,
    And,
    Or,
    Xor,
    Shl,
    AShr,
    LShr,
}

impl IrBinOp {
    /// Returns true if this operation is commutative (a op b == b op a).
    pub fn is_commutative(self) -> bool {
        matches!(self, IrBinOp::Add | IrBinOp::Mul | IrBinOp::And | IrBinOp::Or | IrBinOp::Xor)
    }

    /// Returns true if this operation can trap at runtime (e.g., divide by zero causes SIGFPE).
    /// Such operations must not be speculatively executed by if-conversion.
    pub fn can_trap(self) -> bool {
        matches!(self, IrBinOp::SDiv | IrBinOp::UDiv | IrBinOp::SRem | IrBinOp::URem)
    }

    /// Evaluate this binary operation on two i64 operands using wrapping arithmetic.
    ///
    /// Signed operations use Rust's native i64 arithmetic.
    /// Unsigned operations (UDiv, URem, LShr) reinterpret the bits as u64.
    /// Returns None for division/remainder by zero.
    pub fn eval_i64(self, lhs: i64, rhs: i64) -> Option<i64> {
        Some(match self {
            IrBinOp::Add => lhs.wrapping_add(rhs),
            IrBinOp::Sub => lhs.wrapping_sub(rhs),
            IrBinOp::Mul => lhs.wrapping_mul(rhs),
            IrBinOp::And => lhs & rhs,
            IrBinOp::Or => lhs | rhs,
            IrBinOp::Xor => lhs ^ rhs,
            IrBinOp::Shl => lhs.wrapping_shl(rhs as u32),
            IrBinOp::AShr => lhs.wrapping_shr(rhs as u32),
            IrBinOp::LShr => (lhs as u64).wrapping_shr(rhs as u32) as i64,
            IrBinOp::SDiv => {
                if rhs == 0 { return None; }
                lhs.wrapping_div(rhs)
            }
            IrBinOp::UDiv => {
                if rhs == 0 { return None; }
                ((lhs as u64).wrapping_div(rhs as u64)) as i64
            }
            IrBinOp::SRem => {
                if rhs == 0 { return None; }
                lhs.wrapping_rem(rhs)
            }
            IrBinOp::URem => {
                if rhs == 0 { return None; }
                ((lhs as u64).wrapping_rem(rhs as u64)) as i64
            }
        })
    }

    /// Evaluate this binary operation on two i128 operands using wrapping arithmetic.
    ///
    /// Unsigned operations (UDiv, URem, LShr) reinterpret the bits as u128.
    /// Returns None for division/remainder by zero.
    pub fn eval_i128(self, lhs: i128, rhs: i128) -> Option<i128> {
        Some(match self {
            IrBinOp::Add => lhs.wrapping_add(rhs),
            IrBinOp::Sub => lhs.wrapping_sub(rhs),
            IrBinOp::Mul => lhs.wrapping_mul(rhs),
            IrBinOp::And => lhs & rhs,
            IrBinOp::Or => lhs | rhs,
            IrBinOp::Xor => lhs ^ rhs,
            IrBinOp::Shl => lhs.wrapping_shl(rhs as u32),
            IrBinOp::AShr => lhs.wrapping_shr(rhs as u32),
            IrBinOp::LShr => (lhs as u128).wrapping_shr(rhs as u32) as i128,
            IrBinOp::SDiv => {
                if rhs == 0 { return None; }
                lhs.wrapping_div(rhs)
            }
            IrBinOp::UDiv => {
                if rhs == 0 { return None; }
                (lhs as u128).wrapping_div(rhs as u128) as i128
            }
            IrBinOp::SRem => {
                if rhs == 0 { return None; }
                lhs.wrapping_rem(rhs)
            }
            IrBinOp::URem => {
                if rhs == 0 { return None; }
                (lhs as u128).wrapping_rem(rhs as u128) as i128
            }
        })
    }
}

/// Unary operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrUnaryOp {
    Neg,
    Not,
    Clz,
    Ctz,
    Bswap,
    Popcount,
    /// __builtin_constant_p: returns 1 if operand is a compile-time constant, 0 otherwise.
    /// Lowered as an IR instruction so it can be resolved after inlining and constant propagation.
    IsConstant,
}

/// Comparison operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IrCmpOp {
    Eq,
    Ne,
    Slt,
    Sle,
    Sgt,
    Sge,
    Ult,
    Ule,
    Ugt,
    Uge,
}

impl IrCmpOp {
    /// Evaluate this comparison on two i64 operands.
    ///
    /// Signed comparisons use Rust's native i64 ordering.
    /// Unsigned comparisons reinterpret the bits as u64.
    pub fn eval_i64(self, lhs: i64, rhs: i64) -> bool {
        match self {
            IrCmpOp::Eq => lhs == rhs,
            IrCmpOp::Ne => lhs != rhs,
            IrCmpOp::Slt => lhs < rhs,
            IrCmpOp::Sle => lhs <= rhs,
            IrCmpOp::Sgt => lhs > rhs,
            IrCmpOp::Sge => lhs >= rhs,
            IrCmpOp::Ult => (lhs as u64) < (rhs as u64),
            IrCmpOp::Ule => (lhs as u64) <= (rhs as u64),
            IrCmpOp::Ugt => (lhs as u64) > (rhs as u64),
            IrCmpOp::Uge => (lhs as u64) >= (rhs as u64),
        }
    }

    /// Evaluate this comparison on two i128 operands.
    ///
    /// Signed comparisons use Rust's native i128 ordering.
    /// Unsigned comparisons reinterpret the bits as u128.
    pub fn eval_i128(self, lhs: i128, rhs: i128) -> bool {
        match self {
            IrCmpOp::Eq => lhs == rhs,
            IrCmpOp::Ne => lhs != rhs,
            IrCmpOp::Slt => lhs < rhs,
            IrCmpOp::Sle => lhs <= rhs,
            IrCmpOp::Sgt => lhs > rhs,
            IrCmpOp::Sge => lhs >= rhs,
            IrCmpOp::Ult => (lhs as u128) < (rhs as u128),
            IrCmpOp::Ule => (lhs as u128) <= (rhs as u128),
            IrCmpOp::Ugt => (lhs as u128) > (rhs as u128),
            IrCmpOp::Uge => (lhs as u128) >= (rhs as u128),
        }
    }

    /// Evaluate this comparison on two f64 operands using IEEE 754 semantics.
    ///
    /// For floats, signed and unsigned comparison variants are equivalent since
    /// IEEE 754 defines a total ordering (NaN comparisons return false for
    /// ordered ops, true for Ne).
    pub fn eval_f64(self, lhs: f64, rhs: f64) -> bool {
        match self {
            IrCmpOp::Eq => lhs == rhs,
            IrCmpOp::Ne => lhs != rhs,
            IrCmpOp::Slt | IrCmpOp::Ult => lhs < rhs,
            IrCmpOp::Sle | IrCmpOp::Ule => lhs <= rhs,
            IrCmpOp::Sgt | IrCmpOp::Ugt => lhs > rhs,
            IrCmpOp::Sge | IrCmpOp::Uge => lhs >= rhs,
        }
    }
}

// ── PBT properties (round 06) ─────────────────────────────────────────────────
#[cfg(test)]
mod pbt_tests {
    use super::*;
    use proptest::prelude::*;

    /// P1: C11 6.5.5p6 division identity — (a/b)*b + a%b == a, |a%b| < |b|,
    /// remainder sign follows the dividend (truncation). For the unsigned forms the
    /// identity is checked over the u64 bit patterns.
    /// Doc contract: "Evaluate this binary operation on two i64 operands using wrapping
    /// arithmetic." (ops.rs eval_i64 doc, fingerprint 42c27343)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p1_div_rem_c11_identity(
            a in any::<i64>(),
            b in any::<i64>().prop_filter("b != 0", |b| *b != 0),
        ) {
            let q = IrBinOp::SDiv.eval_i64(a, b).expect("b != 0");
            let r = IrBinOp::SRem.eval_i64(a, b).expect("b != 0");
            prop_assert_eq!(q.wrapping_mul(b).wrapping_add(r), a);
            // |r| < |b| computed in i128 (i64::MIN.abs() overflows i64)
            prop_assert!((r as i128).abs() < (b as i128).abs());
            // truncation-toward-zero: remainder sign follows the dividend
            prop_assert!(r == 0 || (r < 0) == (a < 0));

            let (ua, ub) = (a as u64, b as u64);
            let qu = IrBinOp::UDiv.eval_i64(a, b).expect("b != 0") as u64;
            let ru = IrBinOp::URem.eval_i64(a, b).expect("b != 0") as u64;
            prop_assert_eq!(qu.checked_mul(ub).and_then(|m| m.checked_add(ru)), Some(ua));
            prop_assert!(ru < ub);
        }
    }

    /// P2: failure-path contract — eval is None exactly for division/remainder by zero,
    /// and can_trap() is exactly the div/rem set.
    /// Doc contract: "Returns None for division/remainder by zero." (ops.rs, 00520b4e)
    /// Doc contract: "Returns true if this operation can trap at runtime (e.g., divide
    /// by zero causes SIGFPE)." (ops.rs can_trap, e34c6594)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p2_div_by_zero_none_iff_zero_rhs(a in any::<i64>(), b in any::<i64>()) {
            for op in [IrBinOp::SDiv, IrBinOp::UDiv, IrBinOp::SRem, IrBinOp::URem] {
                prop_assert_eq!(op.eval_i64(a, b).is_none(), b == 0, "op={:?} a={} b={}", op, a, b);
                prop_assert_eq!(op.eval_i128(a as i128, b as i128).is_none(), b == 0);
                prop_assert!(op.can_trap());
            }
            for op in [IrBinOp::Add, IrBinOp::Sub, IrBinOp::Mul, IrBinOp::And, IrBinOp::Or,
                       IrBinOp::Xor, IrBinOp::Shl, IrBinOp::AShr, IrBinOp::LShr] {
                prop_assert!(!op.can_trap(), "{:?} must not trap", op);
                prop_assert!(op.eval_i64(a, b).is_some());
            }
            // commutativity classification (README: Add, Mul, And, Or, Xor)
            for op in [IrBinOp::Add, IrBinOp::Mul, IrBinOp::And, IrBinOp::Or, IrBinOp::Xor] {
                prop_assert!(op.is_commutative());
            }
            for op in [IrBinOp::Sub, IrBinOp::SDiv, IrBinOp::UDiv, IrBinOp::SRem,
                       IrBinOp::URem, IrBinOp::Shl, IrBinOp::AShr, IrBinOp::LShr] {
                prop_assert!(!op.is_commutative());
            }
        }
    }

    /// P3: width consistency — on non-negative operands with a mathematically exact
    /// (non-wrapping-in-i64) result, eval_i128 must agree with eval_i64.
    /// Doc contract: "same for i128 operands." (ops.rs eval_i128 doc, ba12e9a8)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p3_eval_i64_i128_consistency(
            a in 0i64..=i64::MAX,
            raw in any::<u64>(),
        ) {
            let b_raw = (raw % (i64::MAX as u64 + 1)) as i64; // non-negative rhs
            let ops = [
                IrBinOp::Add, IrBinOp::Sub, IrBinOp::Mul, IrBinOp::And, IrBinOp::Or,
                IrBinOp::Xor, IrBinOp::SDiv, IrBinOp::SRem, IrBinOp::UDiv, IrBinOp::URem,
            ];
            for op in ops {
                let b = if b_raw == 0 && matches!(op, IrBinOp::SDiv | IrBinOp::UDiv | IrBinOp::SRem | IrBinOp::URem) {
                    1
                } else { b_raw };
                let exact: i128 = match op {
                    IrBinOp::Add => a as i128 + b as i128,
                    IrBinOp::Sub => a as i128 - b as i128,
                    IrBinOp::Mul => a as i128 * b as i128,
                    IrBinOp::And => (a & b) as i128,
                    IrBinOp::Or => (a | b) as i128,
                    IrBinOp::Xor => (a ^ b) as i128,
                    IrBinOp::SDiv => a as i128 / b as i128,
                    IrBinOp::SRem => a as i128 % b as i128,
                    IrBinOp::UDiv => (a as u64 / b as u64) as i128,
                    IrBinOp::URem => (a as u64 % b as u64) as i128,
                    _ => unreachable!(),
                };
                if exact >= i64::MIN as i128 && exact <= i64::MAX as i128 {
                    prop_assert_eq!(op.eval_i128(a as i128, b as i128), Some(exact as i128),
                        "op={:?} a={} b={}", op, a, b);
                    prop_assert_eq!(op.eval_i64(a, b), Some(exact as i64),
                        "op={:?} a={} b={}", op, a, b);
                }
            }
            // shifts: agreement whenever the shift count agrees (rhs < 64)
            for op in [IrBinOp::Shl, IrBinOp::AShr, IrBinOp::LShr] {
                let k = b_raw % 64;
                let exact: i128 = match op {
                    IrBinOp::Shl => (a as i128) << k,
                    IrBinOp::AShr => a as i128 >> k,
                    IrBinOp::LShr => ((a as u64) >> k) as i128,
                    _ => unreachable!(),
                };
                if exact >= i64::MIN as i128 && exact <= i64::MAX as i128 {
                    prop_assert_eq!(op.eval_i64(a, k as i64), Some(exact as i64));
                    prop_assert_eq!(op.eval_i128(a as i128, k as i128), Some(exact));
                }
            }
        }
    }

    /// P4: comparison coherence — IEEE 754 signed/unsigned equivalence for floats
    /// (incl. NaN/±inf/±0), two's-complement signed-vs-unsigned order laws for i64,
    /// and duality/reflection laws.
    /// Doc contract: "For floats, signed and unsigned comparison variants are equivalent
    /// since IEEE 754 defines a total ordering (NaN comparisons return false for ordered
    /// ops, true for Ne)." (ops.rs eval_f64 doc, fb72c8f7)
    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p4_cmp_coherence_f64(
            a in prop_oneof![
                any::<f64>(),
                Just(f64::NAN), Just(f64::INFINITY), Just(f64::NEG_INFINITY),
                Just(0.0), Just(-0.0), Just(f64::MIN_POSITIVE), Just(5e-324),
                Just(f64::MAX), Just(-f64::MAX),
            ],
            b in prop_oneof![
                any::<f64>(),
                Just(f64::NAN), Just(f64::INFINITY), Just(f64::NEG_INFINITY),
                Just(0.0), Just(-0.0), Just(f64::MIN_POSITIVE), Just(5e-324),
                Just(f64::MAX), Just(-f64::MAX),
            ],
        ) {
            use IrCmpOp::*;
            // documented equivalence: S* == U* on floats
            prop_assert_eq!(Slt.eval_f64(a, b), Ult.eval_f64(a, b), "a={} b={}", a, b);
            prop_assert_eq!(Sle.eval_f64(a, b), Ule.eval_f64(a, b));
            prop_assert_eq!(Sgt.eval_f64(a, b), Ugt.eval_f64(a, b));
            prop_assert_eq!(Sge.eval_f64(a, b), Uge.eval_f64(a, b));
            // IEEE 754: ordered ops false on NaN, Ne true on NaN
            prop_assert_eq!(Ne.eval_f64(a, b), a != b);
            prop_assert_eq!(Eq.eval_f64(a, b), a == b);
            // duality (NaN excepted: IEEE ordered comparisons are all false on NaN)
            if !a.is_nan() && !b.is_nan() {
                prop_assert_eq!(Slt.eval_f64(a, b), !Sge.eval_f64(a, b));
                prop_assert_eq!(Sle.eval_f64(a, b), !Sgt.eval_f64(a, b));
                prop_assert_eq!(Sgt.eval_f64(a, b), Slt.eval_f64(b, a));
            }
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(1024))]
        #[test]
        fn p4b_cmp_coherence_i64(a in any::<i64>(), b in any::<i64>()) {
            use IrCmpOp::*;
            prop_assert_eq!(Slt.eval_i64(a, b), !Sge.eval_i64(a, b));
            prop_assert_eq!(Sle.eval_i64(a, b), !Sgt.eval_i64(a, b));
            prop_assert_eq!(Ne.eval_i64(a, b), !Eq.eval_i64(a, b));
            prop_assert_eq!(Sgt.eval_i64(a, b), Slt.eval_i64(b, a));
            prop_assert_eq!(Sge.eval_i64(a, b), Sle.eval_i64(b, a));
            // two's complement order facts
            if a >= 0 && b >= 0 {
                prop_assert_eq!(Slt.eval_i64(a, b), Ult.eval_i64(a, b));
                prop_assert_eq!(Sle.eval_i64(a, b), Ule.eval_i64(a, b));
            }
            if a < 0 && b >= 0 {
                // negative < non-negative signed; the negative's u64 pattern is huge,
                // so the unsigned orders flip
                prop_assert!(Slt.eval_i64(a, b));
                prop_assert!(!Ult.eval_i64(a, b));
                prop_assert!(Ugt.eval_i64(a, b));
                prop_assert!(!Sgt.eval_i64(a, b));
            }
            if b < 0 && a >= 0 {
                prop_assert!(Sgt.eval_i64(a, b));
                prop_assert!(!Ugt.eval_i64(a, b));
            }
        }
    }
}
