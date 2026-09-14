# PBT Campaign Report: encode_mvn

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mvn
**Tests:** 31 total (7 passing properties + 10 failing properties + 4 passing KAT + 10 failing regression witnesses)
**Result:** 7 passing properties, 10 failing properties (9 bug-report files)
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 strengthening round, ≥1 metamorphic/differential, 1 coverage_gaps round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mvn | 31 | 9 | differential (llvm-mc), algebraic.metamorphic (ORN/ZR alias, sf XOR), algebraic.invariant (ARM fields), negative_error |

## Bugs Found

Each entry is a failing proptest property with a shrunk counterexample, reconfirmed serially (`PBT_TEST_JOBS=1`, `--test-threads=1`). Expected: Err. Actual: Ok(Word).

1. **encode_mvn_neg_extra_operand** — Falsifiable. Shrunk counterexample: `rd=0, rm=0, is_64=false, extra=Reg("x0")`. Law: trailing non-shift operand must Err. Report: `pbt-out/bug_reports/encode_mvn_extra_operand.md`.

2. **encode_mvn_neg_trailing_after_shift** — Falsifiable. Shrunk counterexample: `rd=0, rm=0, is_64=false, extra=Reg("x0")` after `lsl #1`. Law: operand after a valid shift must Err. Same report: `pbt-out/bug_reports/encode_mvn_extra_operand.md`.

3. **encode_mvn_neg_mixed_width** — Falsifiable. Shrunk counterexample: `rd=0, rm=0, rd64=false, rm64=true` (`mvn w0, x0`). Law: mixed X/W must Err. Report: `pbt-out/bug_reports/encode_mvn_mixed_width.md`.

4. **encode_mvn_neg_sp** — Falsifiable. Shrunk counterexample: `which=0, is_64=false, other=0` (`mvn wsp, w0`). Law: SP/WSP must Err. Report: `pbt-out/bug_reports/encode_mvn_sp.md`.

5. **encode_mvn_neg_fp** — Falsifiable. Shrunk counterexample: `which=0, prefix="d", n=0` (`mvn d0, x1`). Law: FP/SIMD name must Err. Report: `pbt-out/bug_reports/encode_mvn_fp_reg.md`.

6. **encode_mvn_neg_shift_range** — Falsifiable. Shrunk counterexample: `rd=0, rm=0, is_64=false, kind="lsl", amt_w=32` (`mvn w0, w0, lsl #32`). Law: 32-bit imm6 range is 0..31. Report: `pbt-out/bug_reports/encode_mvn_shift_range.md`.

7. **encode_mvn_neg_neon_t** — Falsifiable. Shrunk counterexample: `vd=0, vn=0, t="4h"` (`mvn v0.4h, v0.4h`). Law: NEON T in {8b,16b} only. Report: `pbt-out/bug_reports/encode_mvn_neon_t.md`.

8. **encode_mvn_neg_neon_mismatch_t** — Falsifiable. Shrunk counterexample: `vd=0, vn=0, td="16b", tn="8b"` (`mvn v0.16b, v0.8b`). Law: matching T required. Report: `pbt-out/bug_reports/encode_mvn_neon_mismatch_t.md`.

9. **encode_mvn_neg_bad_shift_kind** — Falsifiable. Shrunk counterexample: `rd=0, rm=0, is_64=false, kind="foo", amt=0`. Law: unknown shift kind must Err. Report: `pbt-out/bug_reports/encode_mvn_bad_shift_kind.md`.

10. **encode_mvn_neg_neon_extra** — Falsifiable. Shrunk counterexample: `vd=0, vn=0, extra=Reg("x0")` (`mvn v0.16b, v0.16b, x0`). Law: NEON MVN has no 3rd operand. Report: `pbt-out/bug_reports/encode_mvn_neon_extra.md`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_mvn_pbt) | 31 (4 KAT + 17 properties + 10 regressions) |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_mvn_extra_operand.md
- pbt-out/bug_reports/encode_mvn_mixed_width.md
- pbt-out/bug_reports/encode_mvn_sp.md
- pbt-out/bug_reports/encode_mvn_fp_reg.md
- pbt-out/bug_reports/encode_mvn_shift_range.md
- pbt-out/bug_reports/encode_mvn_neon_t.md
- pbt-out/bug_reports/encode_mvn_neon_mismatch_t.md
- pbt-out/bug_reports/encode_mvn_bad_shift_kind.md
- pbt-out/bug_reports/encode_mvn_neon_extra.md

Contract-surface sweep closed after 1 round (STANDARD allowance): `coverage_gaps` had no profraw; manual arm audit of get_reg 0..1, sf, Shift at index 2, shift-kind default, imm6 mask, Rn=31, neon Q, neon extra. Added `encode_mvn_neg_bad_shift_kind`, `encode_mvn_neg_trailing_after_shift`, `encode_mvn_neg_neon_extra`.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 09:42 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 44/184 total | PBT candidates: 44 | Tested: 44 (100%) | 0 pass, 44 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 44 |
| **Tested (of PBT candidates)** | **44 / 44 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 44 / 0 |
| **Overall (tested / all functions)** | **44 / 184 (24%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 44 | 44 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 44 | 44 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 14 | 14 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 6 | 6 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
| classify_cast_with_f128 | cast.rs |
| encode_adc | data_processing.rs |
| encode_adr | load_store.rs |
| encode_bic | data_processing.rs |
| encode_neon_three_diff_narrow | neon.rs |
| encode_bics | data_processing.rs |
| encode_bl | compare_branch.rs |
| encode_blr | compare_branch.rs |
| encode_br | compare_branch.rs |
| encode_branch | compare_branch.rs |
| encode_cbz | compare_branch.rs |
| encode_ccmp_ccmn | compare_branch.rs |
| encode_cinc | compare_branch.rs |
| encode_cinv | compare_branch.rs |
| encode_cmn | compare_branch.rs |
| encode_cmp | compare_branch.rs |
| encode_cneg | compare_branch.rs |
| encode_csel | compare_branch.rs |
| encode_cset | compare_branch.rs |
| encode_csetm | compare_branch.rs |
| encode_csinc | compare_branch.rs |
| encode_csinv | compare_branch.rs |
| encode_csneg | compare_branch.rs |
| encode_div | data_processing.rs |
| encode_eon | data_processing.rs |
| encode_ldar_stlr | load_store.rs |
| encode_neon_across_long | neon.rs |
| encode_neon_float_cmp_zero | neon.rs |
| encode_neon_sli | neon.rs |
| encode_ldur_stur | load_store.rs |
| encode_ldxp_stxp | load_store.rs |
| encode_neon_float_three_same | neon.rs |
| encode_ldxr_stxr | load_store.rs |
| encode_logical | data_processing.rs |
| encode_madd | data_processing.rs |
| encode_movk | data_processing.rs |
| encode_movn | data_processing.rs |
| encode_movz | data_processing.rs |
| encode_neon_qshrn | neon.rs |
| encode_msub | data_processing.rs |
| encode_mul | data_processing.rs |
| encode_mvn | data_processing.rs |
