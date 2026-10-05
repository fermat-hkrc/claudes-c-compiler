<!-- PBT bug report template v1: rendered from validated report.json; do not hand-edit generated files. -->
# Defect Report: f64_to_f128_bytes_lossless corrupts every |x| < 1.0 (u128 underflow: panic in debug, garbage exponent in release)

**ID:** b1
**Severity:** critical

## Issue Synopsis

- **Impact assessment:** Every double/long double constant with magnitude < 1.0 on ARM64/RISC-V: compiler panic (debug) or silently wrong constant emitted (release).
- **Expected behavior:** lossless bitwise round-trip for every finite f64
- **Observed behavior:** panic 'attempt to subtract with overflow' at long_double.rs:1040 in debug; wrapped garbage exponent in release
- **Minimal counterexample:** x = 0.5 (biased_exp = 1022 < 1023): 'd.biased_exp as u128 - 1023' underflows -> panic 'attempt to subtract with overflow' in debug, garbage exponent in release. Also x = f64::from_bits(1) (subnormal, biased_exp = 0).


## Detection and Validation Methodology

- **Property test:** f128_lossless_roundtrip (p7)
- **Formal property:** forall finite x:f64. f128_bytes_to_f64(f64_to_f128_bytes_lossless(x)).to_bits() = x.to_bits()
- **Oracle:** algebraic.round_trip
- **System under test:** f64_to_f128_bytes_lossless
- **Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs:1028
- **Test file:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs
- **Test target:** cargo test --lib common::long_double

## Reproduction Protocol

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib --no-run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --test-threads=1 --ignored common::long_double::pbt_regression::test_f64_to_f128_bytes_lossless_regression_lt1_underflow
```

**Replay seed:** No random seed is required for this deterministic reproduction.

## Remediation Strategy

Compute the exponent without an unsigned intermediate underflow, e.g. (d.biased_exp as u128 + 16383 - 1023) or via i32; add a subnormal branch that normalizes the mantissa.
