# Verified Bug: Issue #46 — encode_br ignores extra operands
# Verified Bug: Issue #49 — encode_br accepts 32-bit W registers as BR Rn

**Issues:** [#46](https://github.com/fermat-hkrc/claudes-c-compiler/issues/46), [#49](https://github.com/fermat-hkrc/claudes-c-compiler/issues/49)
**Verdict:** ✅ both **TRUE BUG** — one scratch test covers both probes; reference: clang rejects both inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_br::manual_br_two_defects -- --nocapture
br x0,x1 -> Ok(Word(3592355840))  [#46]     (= 0xd61f0000)
br w0    -> Ok(Word(3592355840))  [#49]     (identical word — W width silently discarded)
#46: extra operand must be Err for br
test …manual_br_two_defects ... FAILED
```

Reference (clang AArch64, both REJECTED):

```console
br x0, x1   error: invalid operand for instruction
br w0       error: invalid operand for instruction
```

## Root Cause

`src/backend/arm/assembler/encoder/compare_branch.rs` `encode_br`: reads only
`get_reg(operands, 0)` — no arity check (#46, same family as #5/#38/#42) and the
`is_64` flag is discarded (`let (rn, _)`), so `w0` encodes exactly as `x0` (#49,
same family as #24). Observed words match the issues' claimed Actuals byte-for-byte.

## Suggested Fix

`if operands.len() != 1 { Err(...) }` and require the 64-bit flag from `get_reg`.

## Severity

#46 medium (as claimed); #49 **high** (as claimed) — `br w0` silently becomes a 64-bit
indirect jump through x0.

## Artifacts

- `compare_branch.rs` → `mod scratch_br::manual_br_two_defects`; tracker: #46, #49 in-sample
