# PBT Campaign Report: encode_ubfiz

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_ubfiz
**Tests:** 13 properties (plus 6 KAT + 5 regression witnesses)
**Result:** 8 passing, 5 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ubfiz | 13 properties (8 pass / 5 fail); 6 KAT pass; 5 regression witnesses fail | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_ubfiz silently ignores a 5th operand** — law: UBFIZ has exactly four operands. Minimal input: `ubfiz w0, w0, #0, #1, x0`. Expected Err; actual Ok(Word) because `encode_ubfiz` never checks `operands.len()`. Serial reconfirm with `PBT_TEST_JOBS=1`. Severity: medium. Report: `pbt-out/bug_reports/encode_ubfiz_extra_operand.md`. Regression: `test_encode_ubfiz_regression_extra_operand`.

2. **encode_ubfiz accepts SP/WSP as register 31** — law: register 31 is ZR not SP. Minimal input: `ubfiz wsp, w0, #0, #1`. Expected Err; actual Ok(Word) via `parse_reg_num` mapping sp/wsp to 31. Serial reconfirm. Severity: medium. Report: `pbt-out/bug_reports/encode_ubfiz_sp.md`. Regression: `test_encode_ubfiz_regression_sp`.

3. **encode_ubfiz panics or encodes out-of-range #lsb/#width** — law: 0<=lsb<R, 1<=width<=R-lsb. Minimal input: `ubfiz w0, w0, #0, #0` panics in debug at `width - 1` (bitfield.rs:85). llvm-mc rejects with range error. Serial reconfirm. Severity: high. Report: `pbt-out/bug_reports/encode_ubfiz_lsb_width.md`. Regression: `test_encode_ubfiz_regression_width_zero`.

4. **encode_ubfiz accepts mixed W/X register widths** — law: Rd and Rn must share datasize. Minimal input: `ubfiz x0, w0, #0, #1`. Expected Err; actual Ok(Word) because sf is taken from Rd only. Serial reconfirm. Severity: medium. Report: `pbt-out/bug_reports/encode_ubfiz_mixed_width.md`. Regression: `test_encode_ubfiz_regression_mixed_width`.

5. **encode_ubfiz accepts FP/SIMD registers as GPR operands** — law: UBFIZ operands are GPRs only. Minimal input: `ubfiz d0, x1, #0, #1`. Expected Err; actual Ok(Word) because `parse_reg_num` accepts d/s/q/v/h/b. Serial reconfirm. Severity: medium. Report: `pbt-out/bug_reports/encode_ubfiz_fp.md`. Regression: `test_encode_ubfiz_regression_fp`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_ubfiz_pbt) | 13 properties + 6 KAT + 5 regressions |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row for encode_ubfiz
- pbt-out/COVERAGE_STATUS.md — scanned vs tested
- pbt-out/FUNCTION_INDEX.md — encode_ubfiz marked PBT candidate
- pbt-out/INVARIANTS.md — confirmed encode_ubfiz invariants
- pbt-out/bug_reports/encode_ubfiz_extra_operand.md
- pbt-out/bug_reports/encode_ubfiz_sp.md
- pbt-out/bug_reports/encode_ubfiz_lsb_width.md
- pbt-out/bug_reports/encode_ubfiz_mixed_width.md
- pbt-out/bug_reports/encode_ubfiz_fp.md

## Sweep

Contract-surface sweep round 1/1: `coverage_gaps` had no LLVM profraw. Manual arm audit of encode_ubfiz (invalid-name / nonreg / alt-spellings / mixed / FP). Added encode_ubfiz_diff_alt_spellings, encode_ubfiz_neg_nonreg, encode_ubfiz_neg_invalid_name (passing) and encode_ubfiz_neg_mixed_width / encode_ubfiz_neg_fp (failing, filed). Closed: tier round spent and documented surface covered.

First batch was not all-green (3 negative contracts failed), so the extra all-pass strengthening round was not owed; the required metamorphic/differential properties ran (llvm-mc differential + UBFIZ/UBFM alias + Rd/Rn field independence).

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 21:13 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 89/289 total | PBT candidates: 89 | Tested: 89 (100%) | 0 pass, 89 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 89 |
| **Tested (of PBT candidates)** | **89 / 89 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 89 / 0 |
| **Overall (tested / all functions)** | **89 / 289 (31%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 89 | 89 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 89 | 89 | 0 | 100% |

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
