# Verified Bug: Issue #53 — encode_cbz ignores extra operands
# Verified Bug: Issue #54 — encode_cbz accepts FP/SIMD register names as Rt

**Issues:** [#53](https://github.com/fermat-hkrc/claudes-c-compiler/issues/53), [#54](https://github.com/fermat-hkrc/claudes-c-compiler/issues/54)
**Verdict:** ✅ both **TRUE BUG** — one scratch test covers both; clang rejects both inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_cbz::manual_cbz_two_defects -- --nocapture
cbz x0, L, x1 -> Ok(WordWithReloc { word: 3019898880, CondBr19, "L" })  [#53]
cbz d0, L      -> Ok(WordWithReloc { word: 872415232, CondBr19, "L" })  [#54]
#53: third operand must be Err for cbz
test …manual_cbz_two_defects ... FAILED
```

- #53: word `0xB4000000` — the issue's claimed Actual byte-for-byte; the trailing `x1` is dropped.
- #54: word `0x34000000` = sf=0, Rt=0 — i.e. **`cbz w0, L`**: the FP register name `d0`
  is silently recoded as the 32-bit GPR w0.

Reference (both REJECTED):

```console
cbz x0, L, x1   error: invalid operand for instruction
cbz d0, L       error: invalid operand for instruction
```

## Root Cause

`src/backend/arm/assembler/encoder/compare_branch.rs:237-243` (`encode_cbz`):
no arity check beyond operands 0/1 (#53, arity family: #5/#38/#42/#46);
`parse_reg_num` accepts d/s/q/v/h/b prefixes and `is_64bit_reg` is false for them,
so FP names encode as low-numbered W regs (#54, register-class family: #24/#49).

## Suggested Fix

`if operands.len() != 2 { Err }`; and validate the Rt token is a GPR name
(x*/w*/xzr/wzr) before `parse_reg_num`.

## Severity

#53 medium; #54 **high** (as claimed) — FP register becomes a wrong 32-bit GPR branch.

## Artifacts

- `compare_branch.rs` → `mod scratch_cbz::manual_cbz_two_defects`; tracker: #53, #54 in-sample
