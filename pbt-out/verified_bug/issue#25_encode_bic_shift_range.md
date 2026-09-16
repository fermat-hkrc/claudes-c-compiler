# Verified Bug: Issue #25 — encode_bic masks out-of-range shift amounts instead of rejecting them

**Issue:** [fermat-hkrc/claudes-c-compiler#25](https://github.com/fermat-hkrc/claudes-c-compiler/issues/25)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert); reference cross-check: clang AArch64 assembler rejects the input.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** scratch test + `clang --target=aarch64-linux-gnu` (file-based and corrected stdin one-liner)
**Date:** 2026-09-15

## Claim

BIC (shifted register) shift amount must be in [0, 31] for W-form / [0, 63] for X-form;
`bic w0, w0, w0, lsl #32` must be Err — imm6=32 (bit 5 set) is UNALLOCATED in the 32-bit
logical-shifted-register space.

## Evidence

```console
$ cargo test --lib scratch::manual_bic_shift_range -- --nocapture
bic w0,w0,w0,lsl #32 -> 0x0a208000  (should be Err; imm6=32 UNALLOCATED for W-form)
shift amount 32 out of range for W-form must be Err, got Ok(0x0a208000)
test …manual_bic_shift_range ... FAILED
```

Reference:

```console
$ clang --target=aarch64-linux-gnu -x assembler -c - -o /dev/null <<< 'bic w0, w0, w0, lsl #32'
error: expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 31]
```

## Root Cause

`encode_bic` stores the amount as `(shift_amount & 0x3F) << 10` with no range check
against sf — the same masking pattern as issue #14's ADD/SUB path
(`src/backend/arm/assembler/encoder/data_processing.rs`).

## Suggested Fix

Range-check per width before encoding: `if !is_64 && amount > 31 { return Err(...) }`
(and ≤ 63 for X-form). Scratch test asserts the W-form boundary violation.

## Severity

As claimed: **medium** — silently emits an unallocated encoding.

## Artifacts

- Scratch regression test: `data_processing.rs` → `mod scratch::manual_bic_shift_range`
- Reference: `/tmp/ref/i25.s` + clang output above
- Tracker: `pbt-out/sampling/sample_100_tracker.md` (#25 in-sample)
