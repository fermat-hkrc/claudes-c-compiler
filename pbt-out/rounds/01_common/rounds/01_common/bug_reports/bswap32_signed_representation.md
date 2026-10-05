<!-- PBT bug report template v1: rendered from validated report.json; do not hand-edit generated files. -->
# Defect Report: __builtin_bswap32 folds unsigned results >= 2^31 into signed I32, violating the module's own zero-extension rule

**ID:** b4
**Severity:** medium

## Issue Synopsis

- **Impact assessment:** (__int128)__builtin_bswap32(x)-style widening via eval_const_binop_i128 sign-extends the wrong representation; any consumer reading to_i64() gets a negative value for an unsigned-int constant.
- **Expected behavior:** folded constant zero-extended: 4290586368 for the witness
- **Observed behavior:** folded constant -4380928 (I32 sign-extended)
- **Minimal counterexample:** __builtin_bswap32 with v = -5495501120125551105 (low u32 after bswap = 0xFFBBD010 >= 2^31): folded constant reads back -4380928 (signed I32) instead of 4290586368 (zero-extended unsigned int)
- **Documentation:** src/common/const_arith.rs:88-96 "For unsigned 32-bit results, we must use I64 with zero-extension because IrConst::I32 is signed and cannot correctly represent unsigned values >= 2^31. ... Using I64(4294967295) preserves the correct unsigned value."

## Detection and Validation Methodology

- **Property test:** bswap_semantics (p12b)
- **Formal property:** forall v:i64. bswapN(v) = (v as uN).swap_bytes() zero-extended for the result width, and bswap32(bswap32(v)) = (v as u32)
- **Oracle:** reference
- **System under test:** eval_builtin_call
- **Source location:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/const_eval.rs:95
- **Test file:** /home/shuhao/fermat-users/leo/github/claudes-c-compiler/src/common/const_eval.rs
- **Test target:** cargo test --lib common::const_eval

## Reproduction Protocol

```bash
cd /home/shuhao/fermat-users/leo/github/claudes-c-compiler
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib --no-run
PATH="$HOME/.cargo/bin:$PATH" cargo test --lib -- --test-threads=1 --ignored common::const_eval::pbt_regression::test_eval_builtin_bswap32_regression_unsigned_repr
```

**Replay seed:** No random seed is required for this deterministic reproduction.

## Remediation Strategy

Store the result zero-extended like the arithmetic path does: Some(IrConst::I64(v.swap_bytes() as u64 as i64)).
