# Properties: IrConst::cast_float_to_target

## cast_float_to_target_diff_from_i64_in_range
- Tier: 2
- Rationale: Strongest applicable oracle is Differential against the same-job sibling `IrConst::from_i64`. State machine rejected — pure function, no lifecycle. Same-job evidence: `src/passes/constant_fold.rs:606` folds float-to-int via `from_i64(val as i64, to_ty)` while `src/passes/simplify.rs:407` folds the same cast via `cast_float_to_target`; TODO at `src/passes/constant_fold.rs:583-584` says the paths should be unified. Storage contract for unsigned sub-64-bit types is documented on `from_i64` (`src/ir/constants.rs:448-451`). SUT-boundary=internal-helper. Mapping: for finite fv whose trunc-toward-zero integer part n lies in the closed range of `ty`, expected = `from_i64(n, ty)`. Independent code path (integer constructor vs float `as` casts). Algebraic forms are weaker backups.
- Seed: src/passes/constant_fold.rs:1049 (3.125 to I32 yields 3)
- Formal: ∀ fv ∈ finite f64, ty ∈ {I8,U8,I16,U16,I32,U32,I64,U64,I128,U128}. let n = trunc_toward_zero(fv) as i64. If n is exactly representable in ty's value range (signed min..=max, unsigned 0..=max) then IrConst::cast_float_to_target(fv, ty) = Some(IrConst::from_i64(n, ty)).
- Test file: src/ir/constants.rs
- Status: failing
- Counterexample: n=0, frac=0.0, ty=U8 (cast_float_to_target(0.0, U8)=I8(0) != from_i64(0, U8)=I64(0)). Same storage bug as 128.0/U8: unsigned sub-64-bit targets stored as I8/I16 instead of I64.
- Bug report: pbt-out/bug_reports/cast_float_to_target_unsigned_storage.md

```property
function: IrConst::cast_float_to_target
oracle: differential
predicate:
  quantifier: forall
  vars: [fv, ty]
  domain: { fv: finite f64 whose trunc-toward-zero integer part n is in-range for ty, ty: integer IrType }
  relation:
    op: eq
    lhs: IrConst::cast_float_to_target(fv, ty)
    rhs: Some(IrConst::from_i64(n, ty))
generators:
  n: { gen: int, min: -128, max: 255, type: i64 }
  frac: { gen: float, min: 0.0, max: 0.999, type: f64 }
  ty: { gen: oneof, options: [I8, U8, I16, U16, I32, U32, I64, U64, I128, U128] }
evidence: src/ir/constants.rs:448-451; src/passes/constant_fold.rs:606; src/passes/simplify.rs:407
```

## cast_float_to_target_unsigned_to_i64_zero_extended
- Tier: 4
- Rationale: Algebraic invariant from the `from_i64` storage convention (`src/ir/constants.rs:448-451`: unsigned U8/U16/U32 stored as I64 zero-extended so `to_i64()` does not sign-extend; example U8(255) must not become -1) plus this function's own docstring (`src/ir/constants.rs:275-276`: `200.0 as u8 = 200`). Differential (property 1) is stronger for full variant equality; this isolates the `to_i64()` observer that callers actually use. Round-trip rejected — cast is lossy (fractional part discarded). Idempotence rejected — not a normalizer.
- Seed: src/ir/constants.rs:276 (docstring example 200.0 as u8 = 200)
- Formal: ∀ n ∈ ℕ, ty ∈ {U8,U16,U32,U64}. If n ≤ max(ty) then (IrConst::cast_float_to_target(n as f64, ty)).and_then(|c| c.to_i64()) = Some(n as i64).
- Test file: src/ir/constants.rs
- Status: failing
- Counterexample: n=128, ty=U8; cast_float_to_target(128.0, U8).to_i64() = Some(-128), expected Some(128)
- Bug report: pbt-out/bug_reports/cast_float_to_target_unsigned_storage.md

```property
function: IrConst::cast_float_to_target
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n, ty]
  domain: { n: integer in 0..=max(ty), ty: {U8,U16,U32,U64} }
  relation:
    op: eq
    lhs: IrConst::cast_float_to_target(n as f64, ty).and_then(IrConst::to_i64)
    rhs: Some(n as i64)
generators:
  n: { gen: int, min: 0, max: 65535, type: u64 }
  ty: { gen: oneof, options: [U8, U16, U32, U64] }
evidence: src/ir/constants.rs:448-451; src/ir/constants.rs:275-276
```

## cast_float_to_target_trunc_toward_zero_signed
- Tier: 4
- Rationale: Algebraic invariant — sibling test `test_fold_float_cast_float_to_int` (`src/passes/constant_fold.rs:1049-1062`) asserts 3.125 → I32(3), i.e. truncation toward zero. Domain is in-range finite floats for signed integer targets so the conversion is defined. Round-trip rejected — fractional part is discarded. Differential covered by property 1; this property additionally samples fractional parts and negative values against the integer payload.
- Seed: src/passes/constant_fold.rs:1049
- Formal: ∀ fv ∈ finite f64, ty ∈ {I8,I16,I32,I64}. If trunc_toward_zero(fv) ∈ [min(ty), max(ty)] then the signed integer payload of IrConst::cast_float_to_target(fv, ty) equals trunc_toward_zero(fv) as that signed type.
- Test file: src/ir/constants.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: IrConst::cast_float_to_target
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [fv, ty]
  domain: { fv: finite f64 with trunc-toward-zero in signed ty range, ty: {I8,I16,I32,I64} }
  body: signed_payload(cast_float_to_target(fv, ty)) == (fv.trunc() as signed ty)
generators:
  mag: { gen: int, min: 0, max: 127, type: i64 }
  frac: { gen: float, min: 0.0, max: 0.999, type: f64 }
  neg: { gen: bool }
  ty: { gen: oneof, options: [I8, I16, I32, I64] }
evidence: src/passes/constant_fold.rs:1049-1062
```

## cast_float_to_target_f64_identity
- Tier: 4
- Rationale: Algebraic identity — F64 arm is `IrConst::F64(fv)` (`src/ir/constants.rs:278`) and the IR README lists F64 as 64-bit float storage (`src/ir/README.md:466`). Casting a float to F64 must preserve the bit pattern, including -0.0, infinities, and NaN payloads. State machine / differential rejected (no sibling F64 wrapper with a distinct implementation). Round-trip rejected — there is no inverse pair.
- Seed: (none)
- Formal: ∀ fv ∈ f64. IrConst::cast_float_to_target(fv, IrType::F64) = Some(IrConst::F64(fv)) with equality on to_bits() (NaN payload and sign of zero preserved).
- Test file: src/ir/constants.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: IrConst::cast_float_to_target
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [fv]
  domain: { fv: f64 (all bit patterns) }
  relation:
    op: eq
    lhs: match IrConst::cast_float_to_target(fv, IrType::F64) { Some(IrConst::F64(x)) => x.to_bits(), _ => 0 }
    rhs: fv.to_bits()
generators:
  fv: { gen: float, type: f64 }
evidence: src/ir/constants.rs:278; src/ir/README.md:466
```

## cast_float_to_target_void_none
- Tier: 4e
- Rationale: Negative/error contract — the match's `_` arm returns None (`src/ir/constants.rs:293`); `IrType::Void` is the only remaining variant (`src/common/types.rs:1718`). Signature is `Option<IrConst>`, so unsupported targets must reject. Stronger oracles do not apply to the unsupported-type path.
- Seed: (none)
- Formal: ∀ fv ∈ f64. IrConst::cast_float_to_target(fv, IrType::Void) = None.
- Test file: src/ir/constants.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: IrConst::cast_float_to_target
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fv]
  domain: { fv: f64 }
  relation:
    op: eq
    lhs: IrConst::cast_float_to_target(fv, IrType::Void)
    rhs: None
generators:
  fv: { gen: float, type: f64 }
expected_error: None
evidence: src/ir/constants.rs:293; src/common/types.rs:1718
```

## cast_float_to_target_f128_approx_field
- Tier: 4
- Rationale: Algebraic invariant — F128 arm is `IrConst::long_double(fv)` (`src/ir/constants.rs:279`) whose documented contract (`src/ir/constants.rs:167-168`) stores `fv` as the f64 approximation field of `LongDouble`. Differential against `long_double` would compare the function to the callee it wraps (not independent). Round-trip rejected — f64→f128 is widening but the stored approximation is the original f64.
- Seed: (none)
- Formal: ∀ fv ∈ f64. ∃ bytes. IrConst::cast_float_to_target(fv, IrType::F128) = Some(IrConst::LongDouble(fv, bytes)) with the approximation field equal to fv on to_bits().
- Test file: src/ir/constants.rs
- Status: failing
- Counterexample: bits=9223372036854775809 (0x8000000000000001, negative f64 subnormal). Panics: attempt to subtract with overflow at src/common/long_double.rs:1040 in f64_to_f128_bytes_lossless (biased_exp=0 as u128 - 1023).
- Bug report: pbt-out/bug_reports/cast_float_to_target_f128_subnormal.md

```property
function: IrConst::cast_float_to_target
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [fv]
  domain: { fv: f64 }
  relation:
    op: eq
    lhs: match IrConst::cast_float_to_target(fv, IrType::F128) { Some(IrConst::LongDouble(x, _)) => x.to_bits(), _ => 0 }
    rhs: fv.to_bits()
generators:
  fv: { gen: float, type: f64 }
evidence: src/ir/constants.rs:167-168; src/ir/constants.rs:279
```

## cast_float_to_target_u8_not_i8_saturate
- Tier: 4
- Rationale: Algebraic invariant from this function's docstring (`src/ir/constants.rs:275-276`): unsigned conversion must not saturate to i8::MAX (127). For n ∈ 128..=255, the U8 result's 8-bit pattern equals n as u8 (so 200, not 127). Weaker than property 2 (does not require I64 storage / to_i64()==n) but pins the documented anti-saturation example even if storage form is I8. Documented bound 127/128/255 sampled exactly.
- Seed: src/ir/constants.rs:276
- Formal: ∀ n ∈ {128,...,255}. let r = IrConst::cast_float_to_target(n as f64, IrType::U8). The 8-bit pattern of r equals n as u8 (not 127).
- Test file: src/ir/constants.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: IrConst::cast_float_to_target
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: u8 in 128..=255 }
  body: u8_pattern(cast_float_to_target(n as f64, U8)) == n && u8_pattern(...) != 127
generators:
  n: { gen: int, min: 128, max: 255, type: u8 }
evidence: src/ir/constants.rs:275-276
```

## cast_float_to_target_f32_sign_and_finite
- Tier: 4
- Rationale: Algebraic metamorphic — F32 narrowing preserves sign of finite nonzero inputs and maps infinities to infinities of the same sign (IEEE 754 binary32 conversion implied by IrType::F32 / README F32 row). Round-trip rejected — f64→f32 is lossy. Differential against Rust `as f32` rejected — that is the producing statement. This property checks the sign/finiteness relation, not bit-identity with the `as` implementation.
- Seed: (none)
- Formal: ∀ fv ∈ finite nonzero f64. let Some(IrConst::F32(x)) = cast_float_to_target(fv, F32). Then x.is_sign_negative() = fv.is_sign_negative(). ∀ fv ∈ {+∞,−∞}. x.is_infinite() ∧ x.is_sign_negative() = fv.is_sign_negative().
- Test file: src/ir/constants.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

## cast_float_to_target_ptr_matches_from_i64
- Tier: 2
- Rationale: Contract-surface sweep — IrType::Ptr arm (`src/ir/constants.rs:286` → `ptr_int`) was not in the first-batch integer-type generator. Differential vs from_i64 (same Ptr → ptr_int constructor, `src/ir/constants.rs:438-447`). State machine rejected.
- Seed: (none)
- Formal: ∀ fv ∈ finite f64 with trunc-toward-zero n exactly representable as i64. IrConst::cast_float_to_target(fv, IrType::Ptr) = Some(IrConst::from_i64(n, IrType::Ptr)).
- Test file: src/ir/constants.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: IrConst::cast_float_to_target
oracle: differential
predicate:
  quantifier: forall
  vars: [fv]
  domain: { fv: finite f64 with exact trunc-toward-zero n }
  relation:
    op: eq
    lhs: IrConst::cast_float_to_target(fv, IrType::Ptr)
    rhs: Some(IrConst::from_i64(n, IrType::Ptr))
generators:
  n: { gen: int, min: -1000, max: 1000, type: i64 }
  frac: { gen: float, min: 0.0, max: 0.999, type: f64 }
evidence: src/ir/constants.rs:286; src/ir/constants.rs:438-447
```

```property
function: IrConst::cast_float_to_target
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [fv]
  domain: { fv: finite-nonzero or infinite f64 }
  body: let F32(x) = cast_float_to_target(fv, F32); x.is_sign_negative() == fv.is_sign_negative() && (fv.is_infinite() => x.is_infinite())
generators:
  fv: { gen: float, type: f64 }
evidence: src/ir/README.md:465; src/ir/constants.rs:280
```
