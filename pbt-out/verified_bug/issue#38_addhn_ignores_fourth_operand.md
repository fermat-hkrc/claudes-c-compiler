# Verified Bug: Issue #38 — encode_neon_three_diff_narrow silently ignores a fourth operand

**Issue:** [fermat-hkrc/claudes-c-compiler#38](https://github.com/fermat-hkrc/claudes-c-compiler/issues/38)
**Verdict:** ✅ **TRUE BUG — reproduced at unit level (failing assert); reference cross-check: clang AArch64 assembler rejects the input.**
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`
**Verified by:** scratch test + `clang --target=aarch64-linux-gnu`
**Date:** 2026-09-15

## Claim

ARM ADDHN/SUBHN have a three-register form; four operands must be `Err`
(llvm-mc / GNU as: "invalid operand for instruction").

## Evidence

```console
$ cargo test --lib scratch::manual_addhn_four_operands -- --nocapture
addhn v0.8b,v0.8h,v0.8h,v1.8h -> 0x0e204000  (should be Err; 4th operand dropped)
fourth operand must be Err, got Ok(0x0e204000) — assembled as 3-register addhn
test …manual_addhn_four_operands ... FAILED
```

The word equals the 3-register encoding — the trailing `v1.8h` has zero effect.

Reference:

```console
$ clang --target=aarch64-linux-gnu -x assembler -c - -o /dev/null <<< 'addhn v0.8b, v0.8h, v0.8h, v0.8h'
error: invalid operand for instruction
```

## Root Cause

`src/backend/arm/assembler/encoder/neon.rs:1515`:

```rust
if operands.len() < 3 { return Err("addhn/subhn requires 3 operands".to_string()); }
```

Only the lower bound is checked; surplus operands are ignored — the same arity-check
defect family as issue #5 (`encode_adc`).

## Suggested Fix

`if operands.len() != 3 { return Err(format!("addhn/subhn requires exactly 3 operands, got {}", operands.len())); }`

## Severity

As claimed: **medium** — invalid asm silently assembled as a different (3-register) instruction.

## Artifacts

- Scratch regression test: `neon.rs` → `mod scratch::manual_addhn_four_operands`
- Tracker: `pbt-out/sampling/sample_100_tracker.md` (#38 in-sample)
