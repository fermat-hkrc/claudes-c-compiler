# Bug: float-to-unsigned casts store U8/U16 as signed I8/I16, so to_i64() sign-extends
**Law:** Unsigned sub-64-bit IR constants must be stored as I64 with a zero-extended value so `to_i64()` does not sign-extend (e.g. U8 128 → 128, not -128). Documented on `IrConst::from_i64` and this function's own docstring (`200.0 as u8 = 200`). Same-job float-to-int folding in `constant_fold` uses `from_i64`.
**Impact:** Compile-time `(unsigned char)128.0` / `(unsigned short)40000.0` folded via `cast_float_to_target` (simplify, const_eval, coerce_to) become negative `to_i64()` values. Subsequent integer promotion, comparisons, and GVN hashing disagree with the `constant_fold` path, which stores `I64(128)`.
**Function:** IrConst::cast_float_to_target
**Detected by:** Differential vs from_i64; Algebraic invariant (to_i64 zero-extend)
**Minimal input:** `cast_float_to_target(128.0, IrType::U8)` — `to_i64()` yields `Some(-128)`. Also `cast_float_to_target(0.0, IrType::U8) = I8(0)` vs `from_i64(0, U8) = I64(0)`.
**Expected:** `Some(IrConst::I64(128))` (and `to_i64() == Some(128)`), matching `from_i64(128, IrType::U8)`. Docstring example: `200.0 as u8 = 200`.
**Actual:** `Some(IrConst::I8(-128))`; `to_i64() == Some(-128)`. U16 is the same class (`I16` instead of `I64`). U32 already uses I64 and is consistent.
**Severity:** medium
**Regression test:** src/ir/constants.rs::cast_float_to_target_pbt::test_cast_float_to_target_regression_u8_high_bit
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib cast_float_to_target_unsigned_to_i64 -- --test-threads=1`
