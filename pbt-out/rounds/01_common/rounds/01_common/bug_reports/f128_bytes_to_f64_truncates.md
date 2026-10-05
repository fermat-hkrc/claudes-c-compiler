<!-- PBT bug report template v1: rendered from validated report.json; do not hand-edit generated files. -->
# Defect Report: f128_bytes_to_f64 truncates instead of rounding to nearest — disagrees with same-job sibling x87_to_f64 called by the same caller (eval_const_binop_float) into the same IrConst slot

**ID:** b3
**Severity:** medium

## Issue Synopsis

- **Impact assessment:** Every folded long double Add/Sub/Mul/Div/Mod constant on ARM64/RISC-V (const_arith.rs:174-190) stores an approx 1 ulp away from the correctly-rounded value; by the single-rounding law it differs bitwise from native f64 arithmetic on f64-exact operands.
- **Expected behavior:** round-to-nearest-even narrowing, bitwise-equal to the sibling converter for the same value (2.0 for the witness)
- **Observed behavior:** truncated mantissa: 1.9999999999999998 for the witness
- **Minimal counterexample:** x87 2^0 x 1.111...1 (all-ones 64-bit mantissa, bytes[..8]=u64::MAX.to_le_bytes(), exp=0x3FFF): direct (round-to-nearest) = 2.0, via exact f128 bridge (truncating) = 1.9999999999999998
- **Documentation:** src/common/long_double.rs:655 "Round to nearest: check bit 10 (the first dropped bit)" (same-job sibling x87_bytes_to_f64; the same caller const_arith.rs:239 uses it for the identical IrConst approx slot)

## Detection and Validation Methodology

- **Property test:** x87_f128_bridge_differential (p10)
- **Formal property:** forall normal x87 bytes b. f128_bytes_to_f64(x87_bytes_to_f128_bytes(b)).to_bits() = x87_bytes_to_f64(b).to_bits()
- **Oracle:** differential
- **System under test:** f128_bytes_to_f64
- **Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs:906
- **Test file:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs
- **Test target:** cargo test --lib common::long_double

## Reproduction Protocol

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib --no-run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --test-threads=1 --ignored common::long_double::pbt_regression::test_f128_bytes_to_f64_regression_truncation
```

**Replay seed:** No random seed is required for this deterministic reproduction.

## Remediation Strategy

Mirror x87_bytes_to_f64's rounding (round bit = stored bit 59, sticky = bits 58..0, tie-to-even, mantissa-overflow carry into the exponent).
