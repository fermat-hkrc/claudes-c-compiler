# Properties: classify_cast_with_f128

## classify_cast_identity_is_noop
- Tier: 4
- Rationale: Algebraic invariant from CastKind::Noop ("No conversion needed (same type, ...)") at src/backend/cast.rs:16-17 and README src/backend/README.md:652. State machine rejected — pure function, no lifecycle. Differential rejected — classify_cast is a same-source wrapper (src/backend/cast.rs:151-154); f128_emit_cast emits libcalls rather than CastKind (different job). Round-trip rejected — classification is not an invertible pair. Idempotence rejected — not a normalizer.
- Seed: (none)
- Formal: ∀ ty ∈ IrType, native ∈ bool. classify_cast_with_f128(ty, ty, native) = CastKind::Noop.
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [ty, native]
  domain: { ty: IrType, native: bool }
  relation:
    op: eq
    lhs: classify_cast_with_f128(ty, ty, native)
    rhs: CastKind::Noop
generators:
  ty: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, F128, Ptr, Void] }
  native: { gen: bool }
evidence: src/backend/cast.rs:16-17; src/backend/README.md:652
```

## classify_cast_f128_non_native_reduces_to_f64
- Tier: 4
- Rationale: Algebraic metamorphic — documented F128 reduction on x86: "F128 treated as F64 for computation purposes on x86" (src/backend/cast.rs:61-62, :80-81) and README "on x86, F128 is approximated as F64" (src/backend/README.md:649-650). Transform: replace F128 with F64 in either endpoint; classification with f128_is_native=false must be unchanged. Stronger oracles rejected as in identity property.
- Seed: (none)
- Formal: ∀ from, to ∈ IrType. let from' = F64 if from = F128 else from; let to' = F64 if to = F128 else to. classify_cast_with_f128(from, to, false) = classify_cast_with_f128(from', to', false).
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [from, to]
  domain: { from: IrType, to: IrType }
  relation:
    op: eq
    lhs: classify_cast_with_f128(from, to, false)
    rhs: classify_cast_with_f128(from_f128_as_f64(from), from_f128_as_f64(to), false)
generators:
  from: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, F128, Ptr, Void] }
  to: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, F128, Ptr, Void] }
evidence: src/backend/cast.rs:61-62; src/backend/cast.rs:80-81; src/backend/README.md:649-650
```

## classify_cast_native_flag_irrelevant_without_f128
- Tier: 4
- Rationale: Algebraic metamorphic — F128 handling is gated on `from_ty == F128 || to_ty == F128` (src/backend/cast.rs:72-73); when neither endpoint is F128 the `f128_is_native` flag must not change the result. Transform: flip the flag. Stronger oracles rejected as above.
- Seed: (none)
- Formal: ∀ from, to ∈ IrType \ {F128}. classify_cast_with_f128(from, to, true) = classify_cast_with_f128(from, to, false).
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [from, to]
  domain: { from: IrType minus F128, to: IrType minus F128 }
  relation:
    op: eq
    lhs: classify_cast_with_f128(from, to, true)
    rhs: classify_cast_with_f128(from, to, false)
generators:
  from: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, Ptr, Void] }
  to: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, Ptr, Void] }
evidence: src/backend/cast.rs:72-73
```

## classify_cast_ptr_normalized_as_unsigned_int
- Tier: 4
- Rationale: Algebraic metamorphic — documented Ptr normalization happens before classification: "Ptr treated as U64" (src/backend/cast.rs:61, README:649) and "Ptr is equivalent to U64 on LP64 targets, U32 on ILP32 targets" (src/backend/cast.rs:88-89; src/common/types.rs:20-21). Transform: replace Ptr with that unsigned pointer-width integer. Documented Noop exception: CastKind::Noop includes "Ptr <-> I64/U64" (src/backend/cast.rs:16-17; README:652), generalized to pointer-width signed/unsigned integers (I32/U32 on ILP32). Stronger oracles rejected as above.
- Seed: (none)
- Formal: ∀ from, to ∈ IrType, native ∈ bool, ptr_size ∈ {4,8}. let u = U32 if ptr_size=4 else U64. let from' = u if from=Ptr else from; let to' = u if to=Ptr else to. If from=Ptr ∨ to=Ptr: if ¬from.is_float() ∧ ¬to.is_float() ∧ size(from')=ptr_size ∧ size(to')=ptr_size then classify_cast_with_f128(from,to,native)=Noop else classify_cast_with_f128(from,to,native)=classify_cast_with_f128(from',to',native).
- Test file: src/backend/cast.rs
- Status: failing
- Counterexample: from=Ptr, to=F32, native=false, ptr_size=4 → SignedToFloat { to_f64: false, from_ty: Ptr } != UnsignedToFloat { to_f64: false, from_ty: U32 }
- Bug report: pbt-out/bug_reports/classify_cast_ptr_not_normalized_for_float.md

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [from, to, native, ptr_size]
  domain: { from: IrType, to: IrType, native: bool, ptr_size: {4,8} }
  body: ptr_replaced_equals_or_pointer_width_noop(from, to, native, ptr_size)
generators:
  from: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, F128, Ptr, Void] }
  to: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, F128, Ptr, Void] }
  native: { gen: bool }
  ptr_size: { gen: oneof, options: [4, 8], type: usize }
evidence: src/backend/cast.rs:16-17; src/backend/cast.rs:61; src/backend/cast.rs:88-89; src/backend/README.md:649,652; src/common/types.rs:20-21
```

## classify_cast_native_f128_float_kinds
- Tier: 4
- Rationale: Algebraic invariant from CastKind docs: FloatToF128 is "F32/F64 -> F128 widening via softfloat" with from_f32 flag (src/backend/cast.rs:51-52); F128ToFloat is "F128 -> F32/F64 narrowing" with to_f32 flag (src/backend/cast.rs:53-54). README lists these as IEEE binary128 conversions when f128_is_native (src/backend/README.md:662-668). Domain is the four (F32|F64)x F128 pairs. Stronger oracles rejected as above.
- Seed: (none)
- Formal: ∀ from_f32 ∈ bool. classify_cast_with_f128(F32 if from_f32 else F64, F128, true) = FloatToF128 { from_f32 }. ∀ to_f32 ∈ bool. classify_cast_with_f128(F128, F32 if to_f32 else F64, true) = F128ToFloat { to_f32 }.
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [from_f32, to_f32]
  domain: { from_f32: bool, to_f32: bool }
  body: classify_cast_with_f128(F32 if from_f32 else F64, F128, true) == FloatToF128{from_f32} && classify_cast_with_f128(F128, F32 if to_f32 else F64, true) == F128ToFloat{to_f32}
generators:
  from_f32: { gen: bool }
  to_f32: { gen: bool }
evidence: src/backend/cast.rs:51-54; src/backend/README.md:662-668
```

## classify_cast_native_f128_int_signedness
- Tier: 4
- Rationale: Algebraic invariant from CastKind docs: SignedToF128 / UnsignedToF128 / F128ToSigned / F128ToUnsigned (src/backend/cast.rs:42-50) plus Ptr-as-unsigned (src/backend/cast.rs:61, :88-89). Integer endpoints keep their signedness; Ptr is the unsigned pointer-width integer. Documented bounds: every integer IrType plus Ptr sampled. Stronger oracles rejected as above.
- Seed: (none)
- Formal: ∀ ty ∈ integer IrType ∪ {Ptr}, native=true. If ty is unsigned or ty=Ptr then classify_cast_with_f128(ty, F128, true) = UnsignedToF128 { from_ty: u_of(ty) } and classify_cast_with_f128(F128, ty, true) = F128ToUnsigned { to_ty: u_of(ty) } where u_of(Ptr)=U64/U32 else ty. If ty is signed then classify_cast_with_f128(ty, F128, true) = SignedToF128 { from_ty: ty } and classify_cast_with_f128(F128, ty, true) = F128ToSigned { to_ty: ty }.
- Test file: src/backend/cast.rs
- Status: failing
- Counterexample: ty=Ptr, ptr_size=4 → classify(Ptr, F128, true)=SignedToF128 { from_ty: Ptr } != UnsignedToF128 { from_ty: U32 }
- Bug report: pbt-out/bug_reports/classify_cast_ptr_not_normalized_for_float.md

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [ty, ptr_size]
  domain: { ty: integer IrType or Ptr, ptr_size: {4,8} }
  body: native_f128_int_kind_matches_signedness(ty, ptr_size)
generators:
  ty: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, Ptr] }
  ptr_size: { gen: oneof, options: [4, 8], type: usize }
evidence: src/backend/cast.rs:42-50; src/backend/cast.rs:61; src/backend/cast.rs:88-89
```

## classify_cast_float_int_kinds
- Tier: 4
- Rationale: Algebraic invariant from CastKind docs: FloatToSigned when from is F32/F64 (src/backend/cast.rs:18-19); FloatToUnsigned when dest is unsigned (src/backend/cast.rs:20-21); SignedToFloat / UnsignedToFloat when dest is F32/F64 (src/backend/cast.rs:22-29). Ptr dest is unsigned per Ptr-as-U64/U32 (so to_u64 = dest is U64, or dest is Ptr on LP64). Domain excludes F128 (covered by native/reduction properties). Stronger oracles rejected as above.
- Seed: (none)
- Formal: ∀ from ∈ {F32,F64}, to ∈ integer IrType ∪ {Ptr}, native ∈ bool, ptr_size ∈ {4,8}. classify_cast_with_f128(from,to,native) = FloatToUnsigned { from_f64: from=F64, to_u64: to=U64 ∨ (to=Ptr ∧ ptr_size=8) } if to is unsigned or to=Ptr, else FloatToSigned { from_f64: from=F64 }. ∀ from ∈ integer IrType ∪ {Ptr}, to ∈ {F32,F64}. classify_cast_with_f128(from,to,native) = UnsignedToFloat { to_f64: to=F64, from_ty: u_of(from) } if from is unsigned or from=Ptr, else SignedToFloat { to_f64: to=F64, from_ty: from }.
- Test file: src/backend/cast.rs
- Status: failing
- Counterexample: int_ty=Ptr, is_f64=false, native=false, ptr_size=4, int_to_float=false → FloatToUnsigned { from_f64: false, to_u64: true } != FloatToUnsigned { from_f64: false, to_u64: false }
- Bug report: pbt-out/bug_reports/classify_cast_ptr_not_normalized_for_float.md

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [int_ty, is_f64, native, ptr_size, int_to_float]
  domain: { int_ty: integer IrType or Ptr, is_f64: bool, native: bool, ptr_size: {4,8}, int_to_float: bool }
  body: float_int_kind_matches(int_ty, is_f64, native, ptr_size, int_to_float)
generators:
  int_ty: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, Ptr] }
  is_f64: { gen: bool }
  native: { gen: bool }
  ptr_size: { gen: oneof, options: [4, 8], type: usize }
  int_to_float: { gen: bool }
evidence: src/backend/cast.rs:18-29; src/backend/cast.rs:61; src/backend/cast.rs:88-89
```

## classify_cast_int_widen_narrow_same_size
- Tier: 4
- Rationale: Algebraic invariant from CastKind docs and README: IntWiden when dest is larger, IntNarrow when dest is smaller (src/backend/cast.rs:32-35; README:656-657); same-size signed-to-unsigned is SignedToUnsignedSameSize and unsigned-to-signed is UnsignedToSignedSameSize (src/backend/cast.rs:36-41; README:658-661). Domain is integer IrType pairs (not Ptr/Void/float). Documented size boundaries 1/2/4/8/16 sampled via the closed integer type set. Stronger oracles rejected as above.
- Seed: (none)
- Formal: ∀ from, to ∈ {I8,I16,I32,I64,I128,U8,U16,U32,U64,U128}, native ∈ bool. If from=to then Noop. Else if size(to)>size(from) then IntWiden { from, to }. Else if size(to)<size(from) then IntNarrow { to }. Else if from signed ∧ to unsigned then SignedToUnsignedSameSize { to }. Else if from unsigned ∧ to signed then UnsignedToSignedSameSize { to }.
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [from, to, native]
  domain: { from: integer IrType, to: integer IrType, native: bool }
  body: int_cast_kind_matches_sizes_and_signedness(from, to, native)
generators:
  from: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128] }
  to: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128] }
  native: { gen: bool }
evidence: src/backend/cast.rs:32-41; src/backend/README.md:656-661
```

## classify_cast_float_to_float_widen
- Tier: 4
- Rationale: Contract-surface sweep. Algebraic invariant from CastKind::FloatToFloat "F32 <-> F64" (src/backend/cast.rs:30-31) and README:655. First batch never asserted the `widen` flag. Stronger oracles rejected as in identity property.
- Seed: (none)
- Formal: ∀ native ∈ bool. classify_cast_with_f128(F32, F64, native) = FloatToFloat { widen: true } ∧ classify_cast_with_f128(F64, F32, native) = FloatToFloat { widen: false }.
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [native]
  domain: { native: bool }
  body: classify(F32,F64,native)=FloatToFloat{widen:true} && classify(F64,F32,native)=FloatToFloat{widen:false}
generators:
  native: { gen: bool }
evidence: src/backend/cast.rs:30-31; src/backend/README.md:655
```

## classify_cast_non_native_never_f128_libcall_kinds
- Tier: 4
- Rationale: Contract-surface sweep. Algebraic invariant — ARM backend treats F128 libcall kinds as unreachable from classify_cast() (src/backend/arm/codegen/cast_ops.rs:128), and classify_cast is classify_cast_with_f128(..., false) (src/backend/cast.rs:151-154). CastKind docs mark those variants as native-F128 softfloat (src/backend/cast.rs:42-54).
- Seed: (none)
- Formal: ∀ from, to ∈ IrType. classify_cast_with_f128(from, to, false) ∉ {SignedToF128, UnsignedToF128, F128ToSigned, F128ToUnsigned, FloatToF128, F128ToFloat}.
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [from, to]
  domain: { from: IrType, to: IrType }
  body: not is_f128_libcall_kind(classify_cast_with_f128(from, to, false))
generators:
  from: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, F128, Ptr, Void] }
  to: { gen: oneof, options: [I8, I16, I32, I64, I128, U8, U16, U32, U64, U128, F32, F64, F128, Ptr, Void] }
evidence: src/backend/arm/codegen/cast_ops.rs:128; src/backend/cast.rs:151-154; src/backend/cast.rs:42-54
```

## classify_cast_ptr_pointer_width_is_noop
- Tier: 4
- Rationale: Contract-surface sweep. Algebraic invariant from CastKind::Noop "Ptr <-> I64/U64" (src/backend/cast.rs:16-17; README:652) plus Ptr ≡ U32 on ILP32 (src/backend/cast.rs:88-89). Isolates the integer-Ptr Noop exception from the failing float Ptr metamorphic. Documented bounds I32/U32 (ILP32) and I64/U64 (LP64) sampled exactly.
- Seed: (none)
- Formal: ∀ native ∈ bool, ptr_size ∈ {4,8}, signed ∈ bool. let int_ty = (I32|U32) if ptr_size=4 else (I64|U64). classify_cast_with_f128(Ptr, int_ty, native) = Noop ∧ classify_cast_with_f128(int_ty, Ptr, native) = Noop.
- Test file: src/backend/cast.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: backend.cast.classify_cast_with_f128
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [native, ptr_size, signed]
  domain: { native: bool, ptr_size: {4,8}, signed: bool }
  body: classify(Ptr, pointer_width_int, native) = Noop
generators:
  native: { gen: bool }
  ptr_size: { gen: oneof, options: [4, 8], type: usize }
  signed: { gen: bool }
evidence: src/backend/cast.rs:16-17; src/backend/cast.rs:88-89; src/backend/README.md:652
```
