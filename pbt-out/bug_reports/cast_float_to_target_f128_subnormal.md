# Bug: cast_float_to_target panics on f64 subnormals when targeting F128
**Law:** ∀ fv ∈ f64. `cast_float_to_target(fv, IrType::F128)` returns `Some(LongDouble(fv, bytes))` without panicking. The F128 arm is `IrConst::long_double(fv)`, whose f64 approximation field is `fv` (`src/ir/constants.rs:167-168, 279`). Subnormals are valid f64 values; the API does not exclude them.
**Impact:** Any compile-time cast of a subnormal `double` to `long double` (or coerce-to F128) panics the compiler in debug builds (`attempt to subtract with overflow`). In overflow-unchecked builds the same `u128` subtraction wraps and emits garbage binary128 bytes.
**Function:** IrConst::cast_float_to_target
**Detected by:** Algebraic invariant (F128 approximation field)
**Minimal input:** `bits = 9223372036854775809` (`0x8000000000000001`) — negative f64 subnormal (sign=1, biased_exp=0, mantissa=1). Any subnormal (e.g. bits=1) hits the same path.
**Expected:** `Some(IrConst::LongDouble(fv, bytes))` with `fv.to_bits() == 0x8000000000000001`, no panic.
**Actual:** panic at `src/common/long_double.rs:1040` in `f64_to_f128_bytes_lossless`: `(d.biased_exp as u128 - 1023 + 16383)` underflows because subnormals have `biased_exp == 0` and are not classified as zero (`mantissa != 0`) or special.
**Severity:** high
**Regression test:** src/ir/constants.rs::cast_float_to_target_pbt::test_cast_float_to_target_regression_f128_neg_subnormal
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib cast_float_to_target_f128 -- --test-threads=1`
