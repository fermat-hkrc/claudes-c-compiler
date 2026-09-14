# PBT Campaign Report: encode_umulh

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_umulh
**Tests:** 11 properties + 4 KAT + 4 regression witnesses
**Result:** 7 passing properties, 4 failing properties (4 bugs)
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_umulh | 11 properties (7 passing, 4 failing); 4 KAT passing; 4 regression witnesses failing | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_umulh ignores extra operands.** Law: UMULH takes exactly Xd, Xn, Xm. Shrunk input: `[Reg("x0"), Reg("x0"), Reg("x0"), Reg("x0")]`. Expected Err; actual Ok(Word) because get_reg only reads indices 0..2. Serial reconfirm PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_umulh_extra_operand.md`. Regression: `test_encode_umulh_regression_extra_operand`.

2. **encode_umulh accepts 32-bit W registers.** Law: UMULH has no W form. Shrunk input: `umulh w0, w0, w0`. Expected Err; actual Ok(Word) because is_64 is discarded. Serial reconfirm PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_umulh_wrong_width.md`. Regression: `test_encode_umulh_regression_wrong_width`.

3. **encode_umulh encodes SP/WSP as ZR.** Law: register 31 is XZR, never SP. Shrunk input: `umulh wsp, x0, x0`. Expected Err; actual Ok encoding 31. Serial reconfirm PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_umulh_sp_as_zr.md`. Regression: `test_encode_umulh_regression_sp`.

4. **encode_umulh encodes FP/SIMD names as GPRs.** Law: UMULH operands are X registers. Shrunk input: `umulh d0, x1, x2`. Expected Err; actual Ok(Word) with Rd=0. Serial reconfirm PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_umulh_fp_as_gpr.md`. Regression: `test_encode_umulh_regression_fp`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_umulh_pbt) | 11 proptest properties (1000 cases each), 4 llvm-mc KAT, 4 failing regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_umulh_extra_operand.md
- pbt-out/bug_reports/encode_umulh_wrong_width.md
- pbt-out/bug_reports/encode_umulh_sp_as_zr.md
- pbt-out/bug_reports/encode_umulh_fp_as_gpr.md

Contract-surface sweep closed after 1 round (standard tier): coverage_gaps had no LLVM profraw; manual arm audit of extra / width / SP / FP / non-Reg / invalid-name / alt-spellings. Added encode_umulh_neg_fp (failing), encode_umulh_neg_nonreg (passing), encode_umulh_neg_invalid_name (passing).

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 14:03 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 61/253 total | PBT candidates: 61 | Tested: 61 (100%) | 0 pass, 61 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 61 |
| **Tested (of PBT candidates)** | **61 / 61 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 61 / 0 |
| **Overall (tested / all functions)** | **61 / 253 (24%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 61 | 61 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 61 | 61 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 22 | 22 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 12 | 12 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_shift | gp_integer.rs |
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
| encode_neon_shift_right | neon.rs |
| encode_neg | pseudo.rs |
| encode_negs | data_processing.rs |
| encode_neon_shift_imm | neon.rs |
| encode_neon_tbl | neon.rs |
| encode_orn | data_processing.rs |
| encode_ret | compare_branch.rs |
| encode_sbc | data_processing.rs |
| encode_neon_shll | neon.rs |
| encode_neon_sqshrun | neon.rs |
| encode_smull | data_processing.rs |
| encode_sxth | data_processing.rs |
| encode_sxtw | data_processing.rs |
| encode_neon_shift_left_imm | neon.rs |
| encode_umaddl | data_processing.rs |
| encode_umulh | data_processing.rs |
