# Verified Bug: Issue #152 — encode_ldxp_stxp encodes XZR/WZR as SP when used as the base

**Issue:** [fermat-hkrc/claudes-c-compiler/issues/152](https://github.com/fermat-hkrc/claudes-c-compiler/issues/152)
**Verdict:** ✅ **TRUE BUG** — unit probe fails; gcc (aarch64-linux-gnu-gcc 13.3) rejects the input.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_ldxp2 -- --nocapture
ldxp x0,x1,[xzr] -> Ok(Word(3363768288))  [#152]      (Rn=31 — the SP slot; ZR silently becomes SP)
#152: XZR base must be Err (would encode as SP)
```

gcc reference:

```console
ldxp x0, x1, [xzr]   Error: invalid base register at operand 3
```

## Root Cause

`parse_reg_num("xzr")` = 31, and register 31 in the Rn slot of the exclusive-pair
encoding means **SP** — so the zero register silently becomes the stack pointer
(SP/XZR aliasing family: #49/#100/#112/#116; this time in the base position).

## Suggested Fix

Reject XZR/WZR as the base in `encode_ldxp_stxp` (Rn is Xn|SP only).

## Severity

Medium (silent mis-assembly of an invalid addressing mode).

## Artifacts

- `load_store.rs` → `mod scratch_ldxp2`; tracker: #152 in-sample
