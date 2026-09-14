# PBT Campaign Report: encode_ubfx

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ubfx
**Tests:** 13 properties (8 passing, 5 failing) plus 6 passing KAT and 5 failing regression witnesses
**Result:** 8 passing, 5 bugs
**Effort tier:** standard (1 contract-surface sweep round; ≥1000 proptest cases; ≥1 metamorphic/differential)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ubfx | 13 properties + 6 KAT + 5 regressions | 5 | differential (llvm-mc), algebraic.metamorphic (UBFX alias of UBFM; Rd/Rn fields), algebraic.invariant (ARM fields), negative_error (arity / extra / SP / lsb-width / mixed / FP / nonreg / invalid-name) |

## Bugs Found

1. **encode_ubfx_neg_extra_operand** (Negative/Error Contract). Law: UBFX takes exactly four operands. Shrunk counterexample: `[Reg("w0"), Reg("w0"), Imm(0), Imm(1), extra=Reg("x0")]`. Expected Err; actual Ok(Word) because get_reg/get_imm only read indices 0..3. Serial reconfirm PBT_TEST_JOBS=1. llvm-mc rejects the extra operand.
   - Path: pbt-out/bug_reports/encode_ubfx_extra_operand.md
   - Regression: test_encode_ubfx_regression_extra_operand

2. **encode_ubfx_neg_sp** (Negative/Error Contract). Law: register 31 is ZR not SP. Shrunk counterexample: which=0, sp64=false, is_64=false, other=0 — `ubfx wsp, w0, #0, #1`. Expected Err; actual Ok(Word) (parse_reg_num maps wsp to 31). Serial reconfirm PBT_TEST_JOBS=1.
   - Path: pbt-out/bug_reports/encode_ubfx_sp.md
   - Regression: test_encode_ubfx_regression_sp

3. **encode_ubfx_neg_lsb_width** (Negative/Error Contract). Law: 0<=lsb<R and 1<=width<=R-lsb. Shrunk counterexample: is_64=false, rd=0, rn=0, lsb=0, width=0 — `ubfx w0, w0, #0, #0`. Expected Err; actual debug panic (`attempt to subtract with overflow` at bitfield.rs:15 `lsb + width - 1`). Serial reconfirm PBT_TEST_JOBS=1.
   - Path: pbt-out/bug_reports/encode_ubfx_lsb_width.md
   - Regression: test_encode_ubfx_regression_width_zero

4. **encode_ubfx_neg_mixed_width** (Negative/Error Contract). Law: Rd and Rn must be the same width. Shrunk counterexample: rd=0, rn=0, rd64=true, rn64=false — `ubfx x0, w0, #0, #1`. Expected Err; actual Ok(Word) (sf taken only from Rd). Serial reconfirm PBT_TEST_JOBS=1.
   - Path: pbt-out/bug_reports/encode_ubfx_mixed_width.md
   - Regression: test_encode_ubfx_regression_mixed_width

5. **encode_ubfx_neg_fp** (Negative/Error Contract). Law: Rd/Rn must be GPR, not FP/SIMD. Shrunk counterexample: which=0, prefix="d", n=0 — `ubfx d0, x1, #0, #1`. Expected Err; actual Ok(Word) (parse_reg_num maps d0 to 0). Serial reconfirm PBT_TEST_JOBS=1.
   - Path: pbt-out/bug_reports/encode_ubfx_fp.md
   - Regression: test_encode_ubfx_regression_fp

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_ubfx_pbt) | 13 properties + 6 KAT + 5 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_ubfx_extra_operand.md
- pbt-out/bug_reports/encode_ubfx_sp.md
- pbt-out/bug_reports/encode_ubfx_lsb_width.md
- pbt-out/bug_reports/encode_ubfx_mixed_width.md
- pbt-out/bug_reports/encode_ubfx_fp.md

Sweep close: tier round 1/1 spent; coverage_gaps had no LLVM profraw so a manual arm audit covered arity / extra / SP / mixed / FP / nonreg / invalid-name / alt-spellings / lsb-width. Documented contract surface has a property against each clause.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 22:17 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 94/289 total | PBT candidates: 94 | Tested: 94 (100%) | 0 pass, 94 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 94 |
| **Tested (of PBT candidates)** | **94 / 94 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 94 / 0 |
| **Overall (tested / all functions)** | **94 / 289 (33%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 94 | 94 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 94 | 94 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 6 | 7 | 117% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 10 | 10 | 100% | covered |
| neon.rs | 68 | 14 | 14 | 100% | covered |
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
