# Verified Bug: Issue #83 — encode_cmp accepts XZR/WZR as immediate-form Rn
# Verified Bug: Issue #85 — encode_cneg ignores extra operands

**Issues:** [#83](https://github.com/fermat-hkrc/claudes-c-compiler/issues/83), [#85](https://github.com/fermat-hkrc/claudes-c-compiler/issues/85)
**Verdict:** ✅ both **TRUE BUG** — unit probes fail; gcc (aarch64-linux-gnu-gcc 13.3) rejects both inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_cmp_zr::manual_cmp_zr_imm -- --nocapture
cmp xzr,#0 -> Ok(Word(4043310079))  [#83]     (= 0xF10003FF — the issue's claimed Actual: `cmp sp,#0`)
cmp wzr,#0 -> Ok(Word(1895826431))  [#83]
#83: xzr must be Err in cmp immediate form
FAILED

$ cargo test --lib scratch_cneg::manual_cneg_extra_operand -- --nocapture
cneg x0,x0,eq,x2 -> Ok(Word(3665826816))  [#85]   (4th operand silently dropped)
#85: fourth operand must be Err for cneg
FAILED
```

gcc reference (both REJECTED):

```console
cmp xzr, #0         Error: integer register expected in the extended/shifted operand register at operand 2
cneg x0, x0, eq, x2 Error: unexpected characters following instruction at operand 3
```

## Root Cause

- **#83**: `encode_cmp` prepends XZR and delegates to `encode_add_sub`; in the
  immediate form, register 31 is **SP**, so the source `xzr` (also 31) encodes as
  `cmp sp, #0` — a different instruction (compares the stack pointer).
  Observed word `0xF10003FF` matches the issue's Actual byte-for-byte.
- **#85**: `encode_cneg` (`compare_branch.rs:276+`) inspects operands 0..2 only —
  arity family (#5/#38/#42/#46/#53).

## Suggested Fix

#83: reject XZR/WZR as CMP/CMN immediate-form Rn before delegation.
#85: `if operands.len() != 3 { Err }`.

## Severity

Both medium (as claimed).

## Artifacts

- `compare_branch.rs` → `mod scratch_cmp_zr` + `mod scratch_cneg`; tracker: #83, #85 in-sample
