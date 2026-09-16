# Verified Bug: Issue #59 — encode_ccmp_ccmn truncates out-of-range imm5 and nzcv instead of rejecting them

**Issue:** [fermat-hkrc/claudes-c-compiler/issues/59](https://github.com/fermat-hkrc/claudes-c-compiler/issues/59)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (both probes); reference cross-check: clang rejects both with the exact range messages from the issue's law.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_ccmp::manual_ccmp_imm_ranges -- --nocapture
ccmn x0,#-1,#0,eq -> Ok(Word(3126790144))  (imm5 -1 & 0x1F = 31, silent)  [#59]
ccmp x0,#0,#16,eq -> Ok(Word(4198500352))  (nzcv 16 & 0xF  = 0,  silent)  [#59]
#59: imm5 = -1 must be Err
test …manual_ccmp_imm_ranges ... FAILED
```

Reference — note the error messages match the issue's documented ranges verbatim:

```console
ccmn x0, #-1, #0, eq   error: immediate must be an integer in range [0, 31].
ccmp x0, #0, #16, eq   error: immediate must be an integer in range [0, 15].
```

## Root Cause

`src/backend/arm/assembler/encoder/compare_branch.rs:52+` (`encode_ccmp_ccmn`):
`imm5` stored as `(*imm5 as u32 & 0x1F)`, `nzcv` as `(*nzcv as u32 & 0xF)` — masked,
never range-checked (masking family: #14/#17/#25).

## Suggested Fix

Validate before encoding: `imm5` ∈ [0, 31], `nzcv` ∈ [0, 15], else `Err`.

## Severity

As claimed: **medium** — silently changes the compared immediate / condition-fail NZCV.

## Artifacts

- `compare_branch.rs` → `mod scratch_ccmp::manual_ccmp_imm_ranges`; tracker: #59 in-sample
