# PBT Campaign Report: encode_madd

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_madd
**Tests:** 12 properties (plus 3 KAT + 4 regression witnesses)
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 contract-surface sweep round; coverage_gaps had no profraw — closed after manual arm audit added the lr alias property)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_madd | 12 properties (8 passing, 4 failing) + 3 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_madd ignores a fifth operand.** Failing property: `encode_madd_neg_extra_operand`. Shrunk counterexample: `rd=0, rn=0, rm=0, ra=0, is_64=false, extra=Reg("x0")` (`madd w0, w0, w0, w0, x0`). Expected: Err. Actual: Ok(Word) same as four-operand MADD. Serial reconfirm `PBT_TEST_JOBS=1`. Severity: medium. Report: `pbt-out/bug_reports/encode_madd_extra_operand.md`. Regression: `test_encode_madd_regression_extra_operand`.

2. **encode_madd accepts mixed X/W widths.** Failing property: `encode_madd_neg_mixed_width`. Shrunk counterexample: `rd=0, rn=0, rm=0, ra=0, rd64=false, rn64=false, rm64=false, ra64=true` (`madd w0, w0, w0, x0`). Expected: Err. Actual: Ok(Word); sf taken only from Rd. Serial reconfirm `PBT_TEST_JOBS=1`. Severity: medium. Report: `pbt-out/bug_reports/encode_madd_mixed_width.md`. Regression: `test_encode_madd_regression_mixed_width`.

3. **encode_madd treats SP/WSP as ZR.** Failing property: `encode_madd_neg_sp`. Shrunk counterexample: `which=0, is_64=false, a=0, b=0, c=0` (`madd wsp, w0, w0, w0`). Expected: Err. Actual: Ok(Word) same as `madd wzr, w0, w0, w0`. Serial reconfirm `PBT_TEST_JOBS=1`. Severity: high. Report: `pbt-out/bug_reports/encode_madd_sp.md`. Regression: `test_encode_madd_regression_sp`.

4. **encode_madd accepts FP/SIMD names as GPRs.** Failing property: `encode_madd_neg_fp`. Shrunk counterexample: `which=0, prefix="d", n=0` (`madd d0, x1, x2, x3`). Expected: Err. Actual: Ok(Word) same as `madd w0, x1, x2, x3`. Serial reconfirm `PBT_TEST_JOBS=1`. Severity: medium. Report: `pbt-out/bug_reports/encode_madd_fp_reg.md`. Regression: `test_encode_madd_regression_fp_reg`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_madd_pbt) | 12 properties + 3 KAT + 4 regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_madd marked yes)
- `pbt-out/INVARIANTS.md` — confirmed invariants including encode_madd
- `pbt-out/bug_reports/encode_madd_extra_operand.md`
- `pbt-out/bug_reports/encode_madd_mixed_width.md`
- `pbt-out/bug_reports/encode_madd_sp.md`
- `pbt-out/bug_reports/encode_madd_fp_reg.md`

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 08:02 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 37/184 total | PBT candidates: 37 | Tested: 37 (100%) | 0 pass, 37 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 37 |
| **Tested (of PBT candidates)** | **37 / 37 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 37 / 0 |
| **Overall (tested / all functions)** | **37 / 184 (20%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 37 | 37 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 37 | 37 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 8 | 8 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 5 | 5 | 100% | covered |

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
