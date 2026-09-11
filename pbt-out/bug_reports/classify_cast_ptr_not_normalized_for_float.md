# Bug: Ptr is not treated as U64/U32 for float and F128 casts
**Law:** Ptr normalization happens before classification: Ptr is equivalent to U64 on LP64 and U32 on ILP32, so pointer-to-float must be unsigned and float-to-pointer on ILP32 must use the 32-bit unsigned path.
**Impact:** Pointer-to-float/F128 is classified as signed. On i686, `emit_signed_to_f64` / `emit_signed_to_f128` then use `fildl` without the unsigned high-bit correction, so addresses with the high bit set convert to negative floating values. Float-to-pointer on ILP32 is classified as `FloatToUnsigned { to_u64: true }`; i686 matches that arm with `emit_f32_to_i64` and stores an 8-byte conversion into a 4-byte pointer slot.
**Function:** classify_cast_with_f128
**Detected by:** Algebraic — Metamorphic (4c) and Algebraic — Invariant (4d)
**Minimal input:**
- `classify_cast_with_f128(Ptr, F32, false)` with `target_ptr_size=4` → `SignedToFloat { to_f64: false, from_ty: Ptr }`
- `classify_cast_with_f128(Ptr, F128, true)` with `target_ptr_size=4` → `SignedToF128 { from_ty: Ptr }`
- `classify_cast_with_f128(F32, Ptr, false)` with `target_ptr_size=4` → `FloatToUnsigned { from_f64: false, to_u64: true }`
**Expected:**
- Ptr → F32 equals U32 → F32: `UnsignedToFloat { to_f64: false, from_ty: U32 }`
- Ptr → F128 native equals U32 → F128: `UnsignedToF128 { from_ty: U32 }`
- F32 → Ptr on ILP32 equals F32 → U32: `FloatToUnsigned { from_f64: false, to_u64: false }`
**Actual:** Ptr source is classified as signed; Ptr dest forces `to_u64: true` regardless of pointer width. Root cause: F128 handling and the float-to-int / int-to-float arms run before (and skip) the Ptr-normalization block, which is gated on `!from_ty.is_float() && !to_ty.is_float()`.
**Severity:** high
**Regression test:** src/backend/cast.rs (`test_classify_cast_with_f128_regression_ptr_to_f32_unsigned`, `test_classify_cast_with_f128_regression_ptr_to_f128_unsigned`, `test_classify_cast_with_f128_regression_f32_to_ptr_ilp32`)
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1` (same shrunk witnesses).
