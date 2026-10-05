# Bug: f64_to_f128_bytes_lossless corrupts every |x| < 1.0 (u128 underflow)

**Issue Synopsis**
`f64_to_f128_bytes_lossless` computes the f128 biased exponent as
`(d.biased_exp as u128 - 1023 + 16383)`. For every f64 with magnitude below 1.0
(biased_exp < 1023) the intermediate `biased_exp - 1023` underflows in unsigned
arithmetic: in debug builds the compiler panics (`attempt to subtract with
overflow`); in release builds it wraps to a ~2^128 value whose low bits survive
the `<< 112` shift, producing a garbage exponent — a silently wrong constant.

**Detection and Validation Methodology**
Property P7 (algebraic round-trip: ∀ finite x:f64. f128_bytes_to_f64(f64_to_f128_bytes_lossless(x)) = x bitwise) and P9 (f128 single-rounding differential) over 1024 proptest cases each, shrunk to subnormal/near-1.0 inputs; reproduced serially with `--test-threads=1` and pinned deterministically in the regression test below (fails today, `#[ignore]`d so the suite stays green).

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --ignored common::long_double::pbt_regression::test_f64_to_f128_bytes_lossless_regression_lt1_underflow
# panics: src/common/long_double.rs:1040:18: attempt to subtract with overflow
```

**Remediation Strategy**
Compute the exponent without an intermediate underflow, e.g. `(d.biased_exp as u128 + 16383 - 1023)` or `((d.biased_exp as i32 - 1023 + 16383) as u128)`, and add a subnormal-f64 branch (normalize the subnormal mantissa to 113 bits with a correspondingly smaller exponent), then un-ignore the regression test.

---

**Law:** For any finite f64 x, converting x to f128 and back must be lossless (bitwise identical), as the function's name and its use as the f128 emission path require.
**Impact:** On ARM64/RISC-V (long double = f128) every `double`/`long double` constant or folded value with magnitude < 1.0 panics the compiler in debug builds and is emitted as a garbage-magnitude constant in release builds. This includes subnormals (biased_exp = 0) and ordinary values like 0.5, 0.1, 0.999.
**Function:** f64_to_f128_bytes_lossless
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs:1040
**Detected by:** Algebraic — Round-trip (P7), plus P9 metamorphic differential
**Minimal input:** `f64_to_f128_bytes_lossless(0.5)` (biased_exp = 1022)
**Expected:** `f128_bytes_to_f64(f64_to_f128_bytes_lossless(0.5)).to_bits() == 0.5f64.to_bits()` (0x3FE0000000000000)
**Actual:** panic `attempt to subtract with overflow` at long_double.rs:1040 in debug; garbage exponent in release
**Severity:** critical
**Repro seed:** proptest deterministic regression test (no seed needed); property seeds recorded in pbt-out/rounds/01_common/run/
**Regression test:** src/common/long_double.rs `pbt_regression::test_f64_to_f128_bytes_lossless_regression_lt1_underflow` (#[ignore] witness)
**Raw output:**
```
thread 'common::long_double::pbt_tests::pbt_p7_f128_lossless_roundtrip' panicked at src/common/long_double.rs:1040:18:
attempt to subtract with overflow
```
