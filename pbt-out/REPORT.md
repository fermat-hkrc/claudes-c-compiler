# PBT Campaign Report: encode_logical

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_logical
**Tests:** 11 properties (2 KAT + 9 regression witnesses)
**Result:** 10 passing, 9 bugs
**Effort tier:** standard
**Contract-surface sweep:** 1 round (coverage_gaps had no LLVM profraw; manual arm audit of arity / NEON / Imm / Reg / unsupported-third / invalid-reg / sf). Added encode_logical_metamorphic_sf, encode_logical_neg_unsupported_third, encode_logical_neg_invalid_reg. Closed because the tier's 1 round is done.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_logical | 11 properties + 2 KAT + 9 regression | 9 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

All nine were surfaced by failing, shrunk proptest properties in `encode_logical_neg_rejected_operands` (serial reconfirm with PBT_TEST_JOBS=1). Expected: Err. Actual: Ok(Word).

1. **encode_logical_neg_extra_operand** — shrunk counterexample: `rd=0, rn=0, rm=0, extra=0, is_64=false, opc=0` (`and w0, w0, w0, w0`). Report: pbt-out/bug_reports/encode_logical_extra_operand.md. Regression: test_encode_logical_regression_extra_operand.
2. **encode_logical_neg_sp_shifted** — shrunk counterexample: `other=0, is_64=false, opc=0, pos=0` (`and wsp, w0, w0`). Report: pbt-out/bug_reports/encode_logical_sp_shifted.md. Regression: test_encode_logical_regression_sp_shifted.
3. **encode_logical_neg_mixed_width** — shrunk counterexample: `rd=0, rn=0, rm=0, opc=0, rd64=false, rn64=false, rm64=true` (`and w0, w0, x0`). Report: pbt-out/bug_reports/encode_logical_mixed_width.md. Regression: test_encode_logical_regression_mixed_width.
4. **encode_logical_neg_fp_as_gpr** — shrunk counterexample: `n=0, opc=0, prefix="d", pos=0` (`and d0, x0, x0`). Report: pbt-out/bug_reports/encode_logical_fp_as_gpr.md. Regression: test_encode_logical_regression_fp_as_gpr.
5. **encode_logical_neg_shift_oob** — shrunk counterexample: `rd=0, rn=0, rm=0, is_64=false, opc=0, kind="lsl", amt=32` (`and w0, w0, w0, lsl #32`). Report: pbt-out/bug_reports/encode_logical_shift_oob.md. Regression: test_encode_logical_regression_shift_oob.
6. **encode_logical_neg_unknown_shift** — shrunk counterexample: `rd=0, rn=0, rm=0, is_64=false, opc=0, kind="lslx"`. Report: pbt-out/bug_reports/encode_logical_unknown_shift.md. Regression: test_encode_logical_regression_unknown_shift.
7. **encode_logical_neg_neon_bad_arr** — shrunk counterexample: `vd=0, vn=0, vm=0, arr="4s", opc=0` (`and v0.4s, v0.4s, v0.4s`). Report: pbt-out/bug_reports/encode_logical_neon_bad_arr.md. Regression: test_encode_logical_regression_neon_bad_arr.
8. **encode_logical_neg_neon_mismatch** — shrunk counterexample: `vd=0, vn=0, vm=0, opc=0` (`and v0.16b, v0.8b, v0.16b`). Report: pbt-out/bug_reports/encode_logical_neon_mismatch.md. Regression: test_encode_logical_regression_neon_mismatch.
9. **encode_logical_neg_ands_neon** — shrunk counterexample: `vd=0, vn=0, vm=0, arr="8b"` (`ands v0.8b, v0.8b, v0.8b`). Report: pbt-out/bug_reports/encode_logical_ands_neon.md. Regression: test_encode_logical_regression_ands_neon.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_logical_pbt) | 2 KAT + 11 properties + 9 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_logical_extra_operand.md
- pbt-out/bug_reports/encode_logical_sp_shifted.md
- pbt-out/bug_reports/encode_logical_mixed_width.md
- pbt-out/bug_reports/encode_logical_fp_as_gpr.md
- pbt-out/bug_reports/encode_logical_shift_oob.md
- pbt-out/bug_reports/encode_logical_unknown_shift.md
- pbt-out/bug_reports/encode_logical_neon_bad_arr.md
- pbt-out/bug_reports/encode_logical_neon_mismatch.md
- pbt-out/bug_reports/encode_logical_ands_neon.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 07:44 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 36/184 total | PBT candidates: 36 | Tested: 36 (100%) | 0 pass, 36 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 36 |
| **Tested (of PBT candidates)** | **36 / 36 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 36 / 0 |
| **Overall (tested / all functions)** | **36 / 184 (20%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 36 | 36 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 36 | 36 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 7 | 7 | 100% | covered |
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
