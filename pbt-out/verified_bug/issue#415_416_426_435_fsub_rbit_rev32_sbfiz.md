# Verified Bug: Issues #415, #416, #426, #435 — FP mixed S/D / RBIT extra / REV32 width / SBFIZ FP

**Issues:** [#415](https://github.com/fermat-hkrc/claudes-c-compiler/issues/415), [#416](https://github.com/fermat-hkrc/claudes-c-compiler/issues/416), [#426](https://github.com/fermat-hkrc/claudes-c-compiler/issues/426), [#435](https://github.com/fermat-hkrc/claudes-c-compiler/issues/435)
**Verdict:** ✅ all four **TRUE BUG** — unit probes fail; gcc rejects all inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_farith2 scratch_rbit_gpr scratch_rev32_sbfiz -- --nocapture
fsub d0,s0,s0     -> Ok(Word(509618176))    [#415]   (mixed S/D accepted)
rbit w0,w0,x0     -> Ok(Word(1522532352))   [#416]   (3rd dropped)
rev32 x0,w0       -> Ok(Word(3670018048))   [#426]   (= 0xDAC00800, issue's Actual — width discarded)
sbfiz d0,x1,#0,#1 -> Ok(Word(318767136))    [#435]   (FP name as GPR)
```

gcc reference (all REJECTED):

```console
fsub d0, s0, s0        Error: operand mismatch
rbit w0, w0, x0        Error: unexpected characters following instruction at operand 2
rev32 x0, w0           Error: operand mismatch
sbfiz d0, x1, #0, #1   Error: expected an integer or zero register at operand 1
```

## Root Cause (established families)

- **#415** FP type mix (P2, #359 sibling) · **#416** arity (P1) · **#426** width discard (P2, #397 sibling) · **#435** register class (P2).

## Severity

Medium ×4.

## Artifacts

- `fp_scalar.rs` → `mod scratch_farith2`; `data_processing.rs` → `mod scratch_rbit_gpr`; `bitfield.rs` → `mod scratch_rev32_sbfiz`; tracker: all four in-sample
