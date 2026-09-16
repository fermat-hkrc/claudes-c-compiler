# Verified Bug: Issues #100, #103, #109 — CSETM SP-as-XZR / CSINC mixed width / CSNEG extra operand

**Issues:** [#100](https://github.com/fermat-hkrc/claudes-c-compiler/issues/100), [#103](https://github.com/fermat-hkrc/claudes-c-compiler/issues/103), [#109](https://github.com/fermat-hkrc/claudes-c-compiler/issues/109)
**Verdict:** ✅ all three **TRUE BUG** — unit probes fail; gcc (aarch64-linux-gnu-gcc 13.3) rejects all three inputs.
**Verified on:** branch `explore/pbt-test`, commit `bc6f5170`; **Date:** 2026-09-15

## Evidence

```console
$ cargo test --lib scratch_cond_ops::manual_cond_ops_defects -- --nocapture
csetm sp,eq          -> Ok(Word(3667858431))  [#100]   (= 0xDA9F13FF, issue's Actual verbatim)
csinc x0,w1,x2,eq    -> Ok(Word(2592212000))  [#103]
csneg x0,x0,x0,eq,x3 -> Ok(Word(3665822720))  [#109]   (= 0xDA800400, issue's Actual verbatim)
#100: SP must be Err for csetm (reg 31 = XZR here)
test …manual_cond_ops_defects ... FAILED
```

gcc reference (all REJECTED):

```console
csetm sp, eq              Error: expected an integer or zero register at operand 1
csinc x0, w1, x2, eq      Error: operand mismatch
csneg x0, x0, x0, eq, x3  Error: unexpected characters following instruction at operand 4
```

## Root Cause (established families)

- **#100** SP/XZR aliasing (`compare_branch.rs:155+`): `parse_reg_num("sp")` = 31 and
  CSETM's register-31 slot means XZR/WZR → SP silently becomes the zero register
  (family: #49/#54/#73/#83).
- **#103** width discarded (`compare_branch.rs:99+`): sf from Rd only (family: #24/#65/#81).
- **#109** arity not checked beyond 0..3 (`compare_branch.rs:127+`) (family: #5/#38/#42/#46/#53/#85).

## Suggested Fix

Reject SP/WSP names for CSETM Rd; enforce uniform width across CSINC GPRs;
`operands.len() != 4 → Err` for CSNEG.

## Severity

Medium ×3 (as claimed).

## Artifacts

- `compare_branch.rs` → `mod scratch_cond_ops::manual_cond_ops_defects`; tracker: #100, #103, #109 in-sample
