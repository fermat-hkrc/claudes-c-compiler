# PBT Campaign Report: encode_ldtr_sized

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldtr_sized
**Tests:** 10 properties (6 passing, 4 failing) plus 5 passing KATs and 7 failing regression witnesses
**Result:** 6 passing properties, 7 bugs
**Effort tier:** standard (1 coverage-gaps sweep round; closed because the tier round was spent and the documented surface of encode_ldtr_sized was covered)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldtr_sized | 10 properties + 5 KAT + 7 regressions | 7 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **Extra operand ignored** — `sttrb w0, [x0, #-256], x2` encodes as the first two operands. Law: exactly two operands. Detected by encode_ldtr_sized_neg_extra_operand. Counterexample: rt=0, rn=0, simm=-256, extra=Reg("x2"). Serial reconfirmed. Report: `pbt-out/bug_reports/encode_ldtr_sized_extra_operand.md`. Regression: `test_encode_ldtr_sized_regression_extra_operand`.

2. **Xt dest accepted** — `sttrb x0, [x0, #-256]` encodes as Wt Rt=0 because get_reg discards is_64. Law: dest is Wt. Detected by encode_ldtr_sized_neg_xt_dest. Counterexample: rt=0, rn=0, simm=-256. Serial reconfirmed. Report: `pbt-out/bug_reports/encode_ldtr_sized_xt_dest.md`. Regression: `test_encode_ldtr_sized_regression_xt_dest`.

3. **SP/WSP dest encoded as WZR** — `sttrb sp, [x0, #-256]` uses parse_reg_num("sp")=31. Report: `pbt-out/bug_reports/encode_ldtr_sized_sp_as_rt.md`. Regression: `test_encode_ldtr_sized_regression_sp_as_rt`.

4. **SIMD/FP dest accepted** — `ldtrb d0, [x0]` encodes Rt=0. Report: `pbt-out/bug_reports/encode_ldtr_sized_fp_dest.md`. Regression: `test_encode_ldtr_sized_regression_fp_dest`.

5. **W base accepted** — `ldtrb w0, [w0]` encodes as `[x0]`. Report: `pbt-out/bug_reports/encode_ldtr_sized_w_base.md`. Regression: `test_encode_ldtr_sized_regression_w_base`.

6. **XZR base encoded as SP** — `ldtrb w0, [xzr]` encodes Rn=31. Report: `pbt-out/bug_reports/encode_ldtr_sized_xzr_base.md`. Regression: `test_encode_ldtr_sized_regression_xzr_base`.

7. **Out-of-range offset truncated** — `#-257` encodes imm9=255 (`(-257 as u32) & 0x1FF == 0xFF`). High severity silent wrong address. Detected by encode_ldtr_sized_neg_offset_and_form. Counterexample: rt=0, rn=0, bad_offset=-257. Serial reconfirmed. Report: `pbt-out/bug_reports/encode_ldtr_sized_imm9_range.md`. Regression: `test_encode_ldtr_sized_regression_imm9_range`.

All failures reproduced serially with `PBT_TEST_JOBS=1 cargo test --lib encode_ldtr_sized_neg -- --test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_ldtr_sized_pbt) | 10 properties, 5 KAT, 7 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_ldtr_sized_extra_operand.md
- pbt-out/bug_reports/encode_ldtr_sized_xt_dest.md
- pbt-out/bug_reports/encode_ldtr_sized_sp_as_rt.md
- pbt-out/bug_reports/encode_ldtr_sized_fp_dest.md
- pbt-out/bug_reports/encode_ldtr_sized_w_base.md
- pbt-out/bug_reports/encode_ldtr_sized_xzr_base.md
- pbt-out/bug_reports/encode_ldtr_sized_imm9_range.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 15:45 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 67/253 total | PBT candidates: 67 | Tested: 67 (100%) | 0 pass, 67 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 67 |
| **Tested (of PBT candidates)** | **67 / 67 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 67 / 0 |
| **Overall (tested / all functions)** | **67 / 253 (26%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 67 | 67 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 67 | 67 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 24 | 24 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 8 | 8 | 100% | covered |
| neon.rs | 68 | 13 | 13 | 100% | covered |
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
| encode_neon_rbit | neon.rs |
| encode_umull | data_processing.rs |
| encode_uxtw | data_processing.rs |
| encode_ldaxr_stlxr | load_store.rs |
| encode_ldrsw | load_store.rs |
| encode_ldtr_sized | load_store.rs |
