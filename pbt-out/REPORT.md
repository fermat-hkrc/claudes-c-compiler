# PBT Campaign Report: encode_bfm

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_bfm
**Tests:** 12 properties (plus 6 KAT + 5 regression witnesses)
**Result:** 7 passing, 5 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, ≥1 metamorphic/differential, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bfm | 12 properties (7 passing / 5 failing) + 6 KAT + 5 regression | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **encode_bfm ignores extra operands** — Law: BFM takes exactly four operands. Shrunk input: `[Reg("w0"), Reg("w0"), Imm(0), Imm(0), Reg("x0")]`. Expected Err; actual Ok(Word) because get_reg/get_imm only read indices 0..3. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_bfm_extra_operand.md`. Regression: `test_encode_bfm_regression_extra_operand`.

2. **encode_bfm treats SP/WSP as ZR** — Law: register 31 is WZR/XZR, not SP/WSP. Shrunk input: `bfm wsp, w0, #0, #0`. Expected Err; actual Ok(Word) because parse_reg_num maps sp/wsp to 31. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_bfm_sp.md`. Regression: `test_encode_bfm_regression_sp`.

3. **encode_bfm accepts out-of-range immr/imms** — Law: 0 <= immr,imms < R (32 W / 64 X). Shrunk input: `bfm w0, w0, #-1, #0`. Expected Err; actual Ok(Word) via `as u32` wrap with no range check (also encodes immr/imms = R). Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_bfm_immr_imms.md`. Regression: `test_encode_bfm_regression_immr_neg`.

4. **encode_bfm accepts mixed W/X registers** — Law: Rd and Rn must have the same width. Shrunk input: `bfm x0, w0, #0, #0`. Expected Err; actual Ok(Word) using sf from Rd only. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_bfm_mixed_width.md`. Regression: `test_encode_bfm_regression_mixed_width`.

5. **encode_bfm accepts FP/SIMD registers** — Law: Rd/Rn are GPRs. Shrunk input: `bfm d0, x1, #0, #0`. Expected Err; actual Ok(Word) as 32-bit BFM w0. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_bfm_fp.md`. Regression: `test_encode_bfm_regression_fp`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_bfm_pbt) | 12 properties + 6 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/FUNCTION_INDEX.md — merged function index (encode_bfm now a candidate)
- pbt-out/COVERAGE.md — coverage ledger row for encode_bfm
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed encode_bfm invariants
- pbt-out/bug_reports/encode_bfm_extra_operand.md
- pbt-out/bug_reports/encode_bfm_sp.md
- pbt-out/bug_reports/encode_bfm_immr_imms.md
- pbt-out/bug_reports/encode_bfm_mixed_width.md
- pbt-out/bug_reports/encode_bfm_fp.md

Sweep close-out: coverage_gaps had no LLVM profraw in this session; one standard-tier round was a manual arm audit of encode_bfm (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / immr-imms). Closed because the tier round was spent and the documented contract surface has a property.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 21:27 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 90/289 total | PBT candidates: 90 | Tested: 90 (100%) | 0 pass, 90 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 90 |
| **Tested (of PBT candidates)** | **90 / 90 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 90 / 0 |
| **Overall (tested / all functions)** | **90 / 289 (31%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 90 | 90 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 90 | 90 | 0 | 100% |

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
