# PBT Campaign Report: encode_neon_dup

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_dup
**Tests:** 10 properties + 1 KAT + 5 regression witnesses
**Result:** 6 passing, 4 failing (4 bugs)
**Tier:** standard (1 contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_dup | 10 properties (6 passing, 4 failing); 1 KAT passing; 5 regressions failing | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **Extra operand ignored.** Failing property `encode_neon_dup_neg_extra_operands`. Shrunk counterexample: rd=0, rn=0, t="8b", extra_kind=0 — `dup v0.8b, w0, w0` encodes as the 2-operand form instead of Err. Law: GNU-style DUP is 2-operand; llvm-mc rejects a 3rd operand. Serial reconfirmation with `PBT_TEST_JOBS=1`. Report: `pbt-out/bug_reports/encode_neon_dup_extra_operand.md`.

2. **Wrong-width GPR source accepted.** Failing property `encode_neon_dup_neg_gpr_width`. Shrunk counterexample: rd=0, t="8b", n=0, alias="sp" — `dup v0.8b, x0` encodes as `dup v0.8b, w0`. Law: ARM DUP (general) source is Wn for T≠2D and Xn for T=2D. Serial reconfirmation. Report: `pbt-out/bug_reports/encode_neon_dup_wrong_width_gpr.md`.

3. **Out-of-range lane index masked.** Failing property `encode_neon_dup_neg_index_oor`. Shrunk counterexample: rd=0, rn=0, t="8b", i_extra=1 — `dup v0.8b, v0.b[16]` encodes as `dup v0.8b, v0.b[0]` via `index & 0xF`. Law: ARM DUP (element) b-index range is [0,15]; llvm-mc rejects 16. Serial reconfirmation. Report: `pbt-out/bug_reports/encode_neon_dup_index_oor.md`.

4. **Dest T vs element-size mismatch encoded as a different instruction.** Failing property `encode_neon_dup_neg_size_mismatch`. Shrunk counterexample: rd=0, rn=0, t="8b", ts_other="h", i_mis=0 — `dup v0.8b, v0.h[0]` encodes as Q=0 halfword DUP (`dup v0.4h, v0.h[0]`). Law: dest T must match element size. Serial reconfirmation. Report: `pbt-out/bug_reports/encode_neon_dup_size_mismatch.md`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_dup_pbt) | 10 properties + 1 KAT + 5 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_dup_extra_operand.md
- pbt-out/bug_reports/encode_neon_dup_wrong_width_gpr.md
- pbt-out/bug_reports/encode_neon_dup_index_oor.md
- pbt-out/bug_reports/encode_neon_dup_size_mismatch.md

Sweep close-out: coverage_gaps had no LLVM profraw; manual arm audit of encode_neon_dup added encode_neon_dup_neg_elem_invalid (passing). Closed: tier round spent and documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 23:44 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 100/289 total | PBT candidates: 100 | Tested: 100 (100%) | 0 pass, 100 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 100 |
| **Tested (of PBT candidates)** | **100 / 100 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 100 / 0 |
| **Overall (tested / all functions)** | **100 / 289 (35%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 100 | 100 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 100 | 100 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 10 | 10 | 100% | covered |
| neon.rs | 68 | 16 | 16 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_ubfx | bitfield.rs |
| encode_ubfm | bitfield.rs |
| encode_sbfx | bitfield.rs |
| encode_sbfm | bitfield.rs |
| encode_sbfiz | bitfield.rs |
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
| encode_prfm | load_store.rs |
| encode_smulh | data_processing.rs |
| encode_fcvt_rounding | fp_scalar.rs |
| encode_fp_1src | fp_scalar.rs |
| encode_int_to_float | fp_scalar.rs |
| encode_fcmp | fp_scalar.rs |
| encode_fcvt_precision | fp_scalar.rs |
| encode_neon_aes | neon.rs |
| encode_bfi | bitfield.rs |
| encode_bfxil | bitfield.rs |
| encode_cas | load_store.rs |
| encode_cls | bitfield.rs |
| encode_clz | bitfield.rs |
| encode_extr | bitfield.rs |
| encode_fmov | fp_scalar.rs |
| encode_fp_arith | fp_scalar.rs |
| encode_rbit | bitfield.rs |
| encode_rev | bitfield.rs |
| encode_rev16 | bitfield.rs |
| encode_rev32 | bitfield.rs |
| encode_ubfiz | bitfield.rs |
| encode_bfm | bitfield.rs |
| encode_neon_float_two_misc | neon.rs |
| encode_fabs | fp_scalar.rs |
| encode_fmadd_fmsub | fp_scalar.rs |
| encode_fneg | fp_scalar.rs |
| encode_fsqrt | fp_scalar.rs |
| encode_neon_dup | neon.rs |
