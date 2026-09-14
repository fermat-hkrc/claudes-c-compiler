# PBT Campaign Report: encode_cas

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_cas (src/backend/arm/assembler/encoder/load_store.rs)
**Tests:** 10 properties (6 passing, 4 failing) + 8 passing KAT + 8 failing regression witnesses
**Result:** 6 passing, 8 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cas | 10 properties (6 pass / 4 fail) + 8 KAT + 8 regressions | 8 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

Four failing, shrunk PBT properties. Serial reconfirm: `PBT_TEST_JOBS=1` / `--test-threads=1`. Additional generator arms of the same properties that also fail are listed under the parent witness (each has a failing regression).

1. **encode_cas_neg_extra_operand** (Negative/Error Contract). Shrunk: v=0, rs=0, rt=0, rn=0, is_64=false, extra=Reg("x2") — `cas w0, w0, [x0], x2`. Expected Err; actual Ok(Word) of `cas w0, w0, [x0]`. Severity: medium. Report: `pbt-out/bug_reports/encode_cas_extra_operand.md`. Regression: `test_encode_cas_regression_extra_operand` (fails).
2. **encode_cas_neg_sp_zr_base** (Negative/Error Contract). Shrunk: v=0, n=0, kind=0, is_64=false — `cas sp, w1, [x2]`. Expected Err; actual Ok(Word) aliasing SP to ZR. Severity: medium. Report: `pbt-out/bug_reports/encode_cas_sp_as_rs.md`. Regression: `test_encode_cas_regression_sp_as_rs` (fails).
   - Same property, kind=6/7/8 (reproduce=`test_encode_cas_regression_xzr_as_base`): `cas x0, x1, [xzr]` encodes as `[sp]`. Report: `pbt-out/bug_reports/encode_cas_xzr_as_base.md`. Severity: high.
   - Same property, kind=4 (reproduce=`test_encode_cas_regression_w_base`): `cas w0, w1, [w2]` encodes as `[x2]`. Report: `pbt-out/bug_reports/encode_cas_w_base.md`. Severity: medium.
3. **encode_cas_neg_mixed_fp_xbyte** (Negative/Error Contract). Shrunk: n=0, kind=0, fp='b' — `cas x0, w0, [x1]`. Expected Err; actual Ok(Word) 64-bit CAS. Severity: medium. Report: `pbt-out/bug_reports/encode_cas_mixed_width.md`. Regression: `test_encode_cas_regression_mixed_width` (fails).
   - Same property, kind=2 (reproduce=`test_encode_cas_regression_fp_reg`): `cas s0, s1, [x2]` encodes as `cas w0, w1, [x2]`. Report: `pbt-out/bug_reports/encode_cas_fp_reg.md`.
   - Same property, kind=4 (reproduce=`test_encode_cas_regression_casb_x_reg`): `casb x0, x1, [x2]` encodes as CASB W. Report: `pbt-out/bug_reports/encode_cas_casb_x_reg.md`.
4. **encode_cas_neg_nonzero_offset** (Negative/Error Contract). Shrunk: v=0, rs=0, rt=0, rn=0, is_64=false, off=-1 — `cas w0, w0, [x0, #-1]`. Expected Err; actual Ok(Word) of `cas w0, w0, [x0]`. Severity: medium. Report: `pbt-out/bug_reports/encode_cas_nonzero_offset.md`. Regression: `test_encode_cas_regression_nonzero_offset` (fails).

Results: 6 passed, 4 failed (property tests); 8 passed KAT; 8 failed regressions.

Failing tests:
- encode_cas_neg_extra_operand: SUT bug — extra operand ignored (shrunk `cas w0, w0, [x0], x2`)
- encode_cas_neg_sp_zr_base: SUT bug — SP as Rs (shrunk `cas sp, w1, [x2]`); related arms XZR-base / W-base
- encode_cas_neg_mixed_fp_xbyte: SUT bug — mixed W/X (shrunk `cas x0, w0, [x1]`); related arms FP / casb-X
- encode_cas_neg_nonzero_offset: SUT bug — nonzero offset discarded (shrunk off=-1)

SUT observations (not bugs, but notable):
- gas accepts optional `#0` offset (ARM `{,#0}`); llvm-mc 15 rejects it. At encode_cas, `[Xn]` and `[Xn, #0]` are the same `Mem{offset:0}`.
- Valid encodings (all 12 mnemonics, W/X as specified, zr/sp) match llvm-mc 1000/1000.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_cas_pbt) | 10 properties + 8 KAT + 8 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_cas_extra_operand.md
- pbt-out/bug_reports/encode_cas_sp_as_rs.md
- pbt-out/bug_reports/encode_cas_xzr_as_base.md
- pbt-out/bug_reports/encode_cas_w_base.md
- pbt-out/bug_reports/encode_cas_mixed_width.md
- pbt-out/bug_reports/encode_cas_fp_reg.md
- pbt-out/bug_reports/encode_cas_casb_x_reg.md
- pbt-out/bug_reports/encode_cas_nonzero_offset.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 18:35 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 78/284 total | PBT candidates: 78 | Tested: 78 (100%) | 0 pass, 78 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 284 |
| PBT candidates (from FUNCTION_INDEX) | 78 |
| **Tested (of PBT candidates)** | **78 / 78 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 78 / 0 |
| **Overall (tested / all functions)** | **78 / 284 (27%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 78 | 78 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 78 | 78 | 0 | 100% |

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
