# PBT Campaign Report: encode_neon_aes

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_aes
**Tests:** 8 properties (plus 2 KAT, 1 isolated invalid-name, 1 sweep, 6 regression witnesses)
**Result:** 4 passing, 4 failing properties; 6 SUT bugs
**Effort tier:** standard (5–8 properties, ≥1000 cases, 1 contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_aes | 8 properties (4 passing / 4 failing) | 6 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **Extra operand ignored.** Law: AES takes exactly two operands. Shrunk input: `aese v0.16b, v0.16b, v0.16b`. Expected Err; actual Ok(Word(0x4e284800)) because arity is `len() < 2`. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_aes_extra_operand.md`. Regression: `test_encode_neon_aes_regression_extra_operand`. Serial reconfirm: `PBT_TEST_JOBS=1 cargo test --lib encode_neon_aes -- --test-threads=1`.

2. **Arrangement other than .16B accepted.** Law: ARM ARM Cryptographic AES is only Vd.16B, Vn.16B. Shrunk input: `aese v0.8b, v0.8b`. Expected Err; actual Ok(Word(0x4e284800)) because arrangement is discarded. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_aes_bad_arrangement.md`. Regression: `test_encode_neon_aes_regression_8b_arrangement`.

3. **Mismatched dest/src arrangement accepted.** Law: dest and src T must both be .16B. Shrunk input: `aese v0.16b, v0.8b`. Expected Err; actual Ok(Word(0x4e284800)). Severity: medium. Report: `pbt-out/bug_reports/encode_neon_aes_mismatch_arrangement.md`. Regression: `test_encode_neon_aes_regression_mismatch_arr`.

4. **Bare Vn without arrangement accepted.** Law: source must be Vn.16B. Input: `[RegArrangement{v0,"16b"}, Reg("v0")]`. Expected Err; actual Ok(Word(0x4e284800)) because `get_neon_reg` accepts `Operand::Reg`. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_aes_bare_src.md`. Regression: `test_encode_neon_aes_regression_bare_src`.

5. **Non-V register prefix accepted.** Law: operands are SIMD Vd/Vn. Shrunk input: `aese x0.16b, x0.16b`. Expected Err; actual Ok(Word(0x4e284800)) because `parse_reg_num` accepts x/w/d/s/q/h/b. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_aes_non_v_prefix.md`. Regression: `test_encode_neon_aes_regression_x_prefix`.

6. **SP/WSP encoded as V31.** Law: SP is not an AES operand. Input: `aese sp.16b, v0.16b` (sweep also `wsp`). Expected Err; actual Ok with Rd=31. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_aes_sp_as_neon.md`. Regression: `test_encode_neon_aes_regression_sp`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_aes_pbt) | 8 properties + 2 KAT + 1 isolated invalid-name + 1 sweep + 6 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_aes_extra_operand.md
- pbt-out/bug_reports/encode_neon_aes_bad_arrangement.md
- pbt-out/bug_reports/encode_neon_aes_mismatch_arrangement.md
- pbt-out/bug_reports/encode_neon_aes_bare_src.md
- pbt-out/bug_reports/encode_neon_aes_non_v_prefix.md
- pbt-out/bug_reports/encode_neon_aes_sp_as_neon.md

## Contract-surface sweep

Round 1/1: `coverage_gaps` had no LLVM profraw; manual arm audit of encode_neon_aes. Added `encode_neon_aes_neg_src_nonreg_dest_reg_wsp` (non-register src passes; WSP dest fails — same SP-as-V31 bug). Closed because the tier round is spent and the documented surface is covered.

Skipped target: (none). Build contract `cargo check --lib` / test target `cargo test --lib` succeeded.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 17:43 (campaign: coverage)
> Files: 9/9 scanned (100%) | Functions: 75/267 total | PBT candidates: 75 | Tested: 75 (100%) | 0 pass, 75 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 9 |
| Files scanned | 9 / 9 (100%) |
| Total functions (all files) | 267 |
| PBT candidates (from FUNCTION_INDEX) | 75 |
| **Tested (of PBT candidates)** | **75 / 75 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 75 / 0 |
| **Overall (tested / all functions)** | **75 / 267 (28%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 75 | 75 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 75 | 75 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 14 | 5 | 5 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 9 | 9 | 100% | covered |
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
