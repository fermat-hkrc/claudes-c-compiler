# PBT Campaign Report: encode_ldrsw

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldrsw
**Tests:** 12 properties (8 passing, 4 failing) plus 5 passing KATs and 11 failing regression witnesses
**Result:** 8 passing properties, 10 bugs
**Effort tier:** standard (1 coverage-gaps sweep round; closed because the tier round was spent and the documented surface of encode_ldrsw was covered)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldrsw | 12 properties + 5 KAT + 11 regressions | 10 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **W dest accepted** — `ldrsw w0, [x1]` encodes as Xt Rt=0 (0xb9800000). Law: dest is Xt. Detected by encode_ldrsw_neg_invalid_regs. Counterexample: rt=0, rn=0. Serial reconfirmed. Report: `pbt-out/bug_reports/encode_ldrsw_w_dest.md`. Regression: `test_encode_ldrsw_regression_w_dest`.

2. **SP dest encoded as XZR** — `ldrsw sp, [x1]` uses parse_reg_num("sp")=31. Report: `pbt-out/bug_reports/encode_ldrsw_sp_dest.md`. Regression: `test_encode_ldrsw_regression_sp_dest`.

3. **SIMD/FP dest accepted** — `ldrsw d0, [x1]` encodes Rt=0. Report: `pbt-out/bug_reports/encode_ldrsw_fp_dest.md`. Regression: `test_encode_ldrsw_regression_fp_dest`.

4. **W base accepted** — `ldrsw x0, [w1]` encodes as `[x1]`. Report: `pbt-out/bug_reports/encode_ldrsw_w_base.md`. Regression: `test_encode_ldrsw_regression_w_base`.

5. **XZR/X31 base encoded as SP** — `ldrsw x0, [xzr]` encodes Rn=31. Report: `pbt-out/bug_reports/encode_ldrsw_xzr_base.md`. Regression: `test_encode_ldrsw_regression_xzr_base`.

6. **W index without extend encoded as LSL** — `ldrsw x0, [x1, w2]` uses option=011. Report: `pbt-out/bug_reports/encode_ldrsw_w_index.md`. Regression: `test_encode_ldrsw_regression_w_index_no_extend`.

7. **Writeback Rt==Rn encoded** — `ldrsw x0, [x0, #4]!` is unpredictable; llvm-mc rejects it. Report: `pbt-out/bug_reports/encode_ldrsw_writeback_overlap.md`. Regression: `test_encode_ldrsw_regression_writeback_rt_eq_rn`.

8. **Out-of-range offset truncated** — `#-257` encodes imm9=255; `#16384` encodes imm9=0. High severity silent wrong address. Report: `pbt-out/bug_reports/encode_ldrsw_offset_range.md`. Regressions: `test_encode_ldrsw_regression_imm9_range`, `test_encode_ldrsw_regression_pimm_overflow`.

9. **Extra operand ignored** — three operands encode as the first two. Report: `pbt-out/bug_reports/encode_ldrsw_extra_operand.md`. Regression: `test_encode_ldrsw_regression_extra_operand`.

10. **LDRSW (literal) missing** — `ldrsw x0, foo` returns Err; ARM/llvm-mc require RelocType::Ldr19. Report: `pbt-out/bug_reports/encode_ldrsw_literal.md`. Regression: `test_encode_ldrsw_regression_literal`.

All failures reproduced serially with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrsw -- --test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_ldrsw_pbt) | 12 properties, 5 KAT, 11 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_ldrsw_w_dest.md
- pbt-out/bug_reports/encode_ldrsw_sp_dest.md
- pbt-out/bug_reports/encode_ldrsw_fp_dest.md
- pbt-out/bug_reports/encode_ldrsw_w_base.md
- pbt-out/bug_reports/encode_ldrsw_xzr_base.md
- pbt-out/bug_reports/encode_ldrsw_w_index.md
- pbt-out/bug_reports/encode_ldrsw_writeback_overlap.md
- pbt-out/bug_reports/encode_ldrsw_offset_range.md
- pbt-out/bug_reports/encode_ldrsw_extra_operand.md
- pbt-out/bug_reports/encode_ldrsw_literal.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 15:32 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 66/253 total | PBT candidates: 66 | Tested: 66 (100%) | 0 pass, 66 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 66 |
| **Tested (of PBT candidates)** | **66 / 66 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 66 / 0 |
| **Overall (tested / all functions)** | **66 / 253 (26%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 66 | 66 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 66 | 66 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 24 | 24 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 7 | 7 | 100% | covered |
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
