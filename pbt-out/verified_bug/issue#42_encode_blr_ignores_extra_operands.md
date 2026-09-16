# Verified Bug: Issue #42 — encode_blr ignores extra operands

**Issue:** [fermat-hkrc/claudes-c-compiler#42](https://github.com/fermat-hkrc/claudes-c-compiler/issues/42)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert); reference cross-check: clang AArch64 assembler rejects the input.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** scratch test + `clang --target=aarch64-linux-gnu`
**Date:** 2026-09-15

## Claim

BLR takes a single Xn operand; `blr x0, x1` must be `Err` (gas/llvm-mc reject it).

## Evidence

```console
$ cargo test --lib scratch_blr::manual_blr_extra_operand -- --nocapture
blr x0,x1 -> 0xd63f0000  (should be Err; extra operand dropped)
extra operand must be Err for blr, got Ok(0xd63f0000)
test …manual_blr_extra_operand ... FAILED
```

Word `0xd63f0000` matches the issue's claimed Actual exactly.

Reference:

```console
$ clang --target=aarch64-linux-gnu -x assembler -c - -o /dev/null <<< 'blr x0, x1'
error: invalid operand for instruction
```

## Root Cause

`src/backend/arm/assembler/encoder/compare_branch.rs:219-224`: `encode_blr` reads only
`get_reg(operands, 0)`; no arity check — arity-defect family (#5, #38, this).

## Suggested Fix

`if operands.len() != 1 { return Err(format!("blr requires exactly 1 operand, got {}", operands.len())); }`

## Severity

As claimed: **medium**.

## Artifacts

- Scratch regression test: `compare_branch.rs` → `mod scratch_blr::manual_blr_extra_operand`
- Tracker: `pbt-out/sampling/sample_100_tracker.md` (#42 in-sample)
