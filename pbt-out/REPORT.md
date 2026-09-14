# PBT Campaign Report: encode_cls

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_cls
**Tests:** 11 properties + 6 KAT + 4 regression witnesses
**Result:** 7 passing properties, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; generator runs = 1000)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cls | 11 properties (7 passing, 4 failing) + 6 KAT + 4 regression | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **encode_cls ignores extra operands.** `cls w0, w0, x0` encodes as `cls w0, w0` (0x5ac01400) instead of Err. Law: two-operand CLS. Counterexample: [Reg("w0"), Reg("w0"), Reg("x0")]. Root cause: no `operands.len()` check; `get_reg` only reads indices 0 and 1. Severity: medium. Report: `pbt-out/bug_reports/encode_cls_extra_operand.md`. Regression: `test_encode_cls_regression_extra_operand`.

2. **encode_cls accepts SP/WSP as register 31.** `cls wsp, w0` encodes as `cls wzr, w0` (0x5ac0141f) instead of Err. Law: ARM CLS register 31 is ZR not SP. Counterexample: [Reg("wsp"), Reg("w0")]. Root cause: `parse_reg_num` maps sp/wsp to 31; encode_cls does not reject SP. Severity: medium. Report: `pbt-out/bug_reports/encode_cls_sp.md`. Regression: `test_encode_cls_regression_sp`.

3. **encode_cls accepts mixed W/X widths.** `cls x0, w0` encodes as `cls x0, x0` (0xdac01400) instead of Err. Law: matching W/W or X/X. Counterexample: [Reg("x0"), Reg("w0")]. Root cause: sf taken from Rd only; Rn width ignored. Severity: medium. Report: `pbt-out/bug_reports/encode_cls_mixed_width.md`. Regression: `test_encode_cls_regression_mixed_width`.

4. **encode_cls accepts FP/SIMD registers as GPRs.** `cls d0, x1` encodes as `cls w0, w1` (0x5ac01420) instead of Err. Law: scalar CLS is integer GPR only. Counterexample: [Reg("d0"), Reg("x1")]. Root cause: `parse_reg_num` accepts d/s/q/v/h/b; encode_cls does not call `is_fp_reg`. Severity: medium. Report: `pbt-out/bug_reports/encode_cls_fp.md`. Regression: `test_encode_cls_regression_fp`.

All four reproduced serially (`cargo test --lib encode_cls -- --test-threads=1`).

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_cls_pbt) | 11 properties (7 pass / 4 fail) + 6 KAT + 4 failing regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_cls_extra_operand.md
- pbt-out/bug_reports/encode_cls_sp.md
- pbt-out/bug_reports/encode_cls_mixed_width.md
- pbt-out/bug_reports/encode_cls_fp.md

## Sweep

Round 1/1: `coverage_gaps` had no LLVM profraw; manual arm audit of encode_cls (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings). Added encode_cls_diff_alt_spellings, encode_cls_neg_nonreg, encode_cls_neg_invalid_name (all passing). Closed: tier round spent and documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 18:51 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 79/284 total | PBT candidates: 79 | Tested: 79 (100%) | 0 pass, 79 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 284 |
| PBT candidates (from FUNCTION_INDEX) | 79 |
| **Tested (of PBT candidates)** | **79 / 79 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 79 / 0 |
| **Overall (tested / all functions)** | **79 / 284 (28%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 79 | 79 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 79 | 79 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 4 | 5 | 125% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 10 | 10 | 100% | covered |
| neon.rs | 68 | 14 | 14 | 100% | covered |
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
