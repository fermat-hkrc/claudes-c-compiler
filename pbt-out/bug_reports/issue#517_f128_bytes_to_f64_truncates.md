# Bug: f128_bytes_to_f64 truncates the mantissa instead of rounding to nearest

**Issue Synopsis**
`f128_bytes_to_f64` narrows the 112-bit f128 mantissa with
`let mantissa52 = (stored >> 60) as u64;` — a plain truncation (round toward
zero). IEEE 754 / C11 default rounding for conversion is round-to-nearest-even,
and the sibling same-job converter `x87_bytes_to_f64` implements exactly that
("Round to nearest: check bit 10 (the first dropped bit)" + sticky + tie-even).
Because x87 → f128 is exact (same bias, 63-bit → 112-bit mantissa), the two
converters must agree; they differ by 1 ulp on ~half of all mantissas.

**Detection and Validation Methodology**
Property P10 (differential between the two same-job converters across the exact x87→f128 bridge), 1024 cases; plus the single-rounding metamorphic property P9 (f128 arithmetic on f64-exact operands is exact, so one correctly-rounded narrowing must equal native f64 arithmetic — currently blocked earlier by B1's panic); reproduced serially; deterministic regression witness below.

**Reproduction Protocol**
```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --ignored common::long_double::pbt_regression::test_f128_bytes_to_f64_regression_truncation
# assertion failed: left: 4611686018427387903 (1.9999999999999998), right: 4611686018427387904 (2.0)
```

**Remediation Strategy**
Mirror the rounding logic already present in `x87_bytes_to_f64` (round bit =
stored bit 59, sticky = bits 58..0, tie-to-even), including the mantissa-overflow
carry into the exponent; also round (not truncate) in `u128_to_f128_bytes`'s
`val >> (bl - 113)`.

---

**Law:** Two same-job narrowings to f64 (direct x87→f64, documented round-to-nearest; x87→f128→f64 across an exact widening) must produce the identical f64.
**Impact:** Compile-time `long double` arithmetic on ARM64/RISC-V folds constants up to 1 ulp low whenever the dropped 60 bits are ≥ half an ulp (≈50% of random mantissas). Also affects `eval_const_binop_float`'s `approx` value and any `f128_bytes_to_f64` consumer.
**Function:** f128_bytes_to_f64
**Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs:906
**Detected by:** Differential (same-job sibling converters, P10)
**Minimal input:** x87 value 2^0 × 1.111...1 (all 64 mantissa bits set): `bytes[..8] = u64::MAX.to_le_bytes()`, `bytes[8..10] = {0xFF, 0x3F}`
**Expected:** f64 2.0 (round-to-nearest carries into the exponent; produced by x87_bytes_to_f64 and by hardware/libm)
**Actual:** 1.9999999999999998 (truncated)
**Severity:** medium
**Repro seed:** deterministic
**Regression test:** src/common/long_double.rs `pbt_regression::test_f128_bytes_to_f64_regression_truncation` (#[ignore] witness)
**Raw output:**
```
Test failed: assertion failed: `(left == right)`
  left: `2355634373914480375`,
 right: `2355634373914480376`: direct=3.2257192316651656e-151 via_f128=3.225719231665165e-151
```
