<!-- PBT bug report template v1: rendered from validated report.json; do not hand-edit generated files. -->
# Defect Report: f64_to_x87_bytes_simple mis-encodes subnormal f64 values (assumes implicit leading 1)

**ID:** b2
**Severity:** high

## Issue Synopsis

- **Impact assessment:** Subnormal double constants through the x86/i686 path become ~2^752 times larger — silent wrong constant.
- **Expected behavior:** decode-back to the original subnormal value (widening is exact per its own doc)
- **Observed behavior:** 5e-324 encodes as ~2.22e-308 and decodes to +0.0
- **Minimal counterexample:** x = f64::from_bits(1) = 5e-324 (smallest positive subnormal): encoded as ~2.22e-308 (implicit-leading-1 assumption on a subnormal mantissa), decodes to +0.0 (bits 0x0 vs expected 0x1)
- **Documentation:** src/common/long_double.rs:1141 "This is a widening conversion that zero-fills the extra mantissa bits."

## Detection and Validation Methodology

- **Property test:** x87_f64_roundtrip (p11)
- **Formal property:** forall finite x:f64. x87_bytes_to_f64(f64_to_x87_bytes_simple(x)).to_bits() = x.to_bits()
- **Oracle:** algebraic.round_trip
- **System under test:** f64_to_x87_bytes_simple
- **Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs:1143
- **Test file:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/long_double.rs
- **Test target:** cargo test --lib common::long_double

## Reproduction Protocol

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib --no-run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --test-threads=1 --ignored common::long_double::pbt_regression::test_f64_to_x87_bytes_simple_regression_subnormal
```

**Replay seed:** No random seed is required for this deterministic reproduction.

## Remediation Strategy

Add a subnormal branch: normalize the mantissa to explicit-integer-bit form with a correspondingly smaller exponent.
