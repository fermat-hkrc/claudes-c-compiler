# PBT Campaign Report: encode_smull

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_smull
**Tests:** 9 properties (6 passing, 3 failing) + 4 passing KAT + 4 failing regression witnesses
**Result:** 6 passing, 4 bugs
**Effort tier:** standard (1 coverage-sweep round; coverage_gaps had no profraw — manual arm audit added alt-spelling differential)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_smull | 9 properties (6 pass / 3 fail) + 4 KAT + 4 regressions | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_smull silently ignores a 4th operand**
   - Law: scalar SMULL is 3-operand; extra operand must Err (llvm-mc `invalid operand`).
   - Shrunk input: `[Reg("x0"), Reg("w0"), Reg("w0"), Reg("x0")]` → `Ok(Word(0x9b207c00))`.
   - Root cause: `get_reg` only reads indices 0..2.
   - Severity: medium.
   - Report: `pbt-out/bug_reports/encode_smull_extra_operand.md`
   - Regression: `test_encode_smull_regression_extra_operand` (fails as witness).
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`.

2. **encode_smull accepts W dest and X sources**
   - Law: ARM ARM form is `SMULL Xd, Wn, Wm` only; llvm-mc rejects W dest / X sources.
   - Shrunk input: `[Reg("w0"), Reg("w0"), Reg("w0")]` → `Ok`.
   - Root cause: `is_64` from `get_reg` is discarded; bit 31 is hardcoded 1.
   - Severity: medium.
   - Report: `pbt-out/bug_reports/encode_smull_wrong_width.md`
   - Regression: `test_encode_smull_regression_wrong_width` (fails as witness).
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`.

3. **encode_smull encodes SP/WSP as XZR/WZR**
   - Law: register 31 is XZR/WZR, never SP/WSP; llvm-mc rejects SP/WSP.
   - Shrunk input: `[Reg("wsp"), Reg("w0"), Reg("w0")]` → `Ok` with Rd=31.
   - Root cause: `parse_reg_num` maps `sp`/`wsp` to 31.
   - Severity: medium.
   - Report: `pbt-out/bug_reports/encode_smull_sp_as_zr.md`
   - Regression: `test_encode_smull_regression_sp` (fails as witness).
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`.

4. **encode_smull encodes FP/SIMD names as GPRs**
   - Law: scalar SMULL operands are GPRs; llvm-mc rejects `d`/`s`/`q`/`v`/`h`/`b`.
   - Shrunk input: `[Reg("d0"), Reg("w1"), Reg("w2")]` → `Ok` as if `x0`.
   - Root cause: `parse_reg_num` accepts FP/SIMD prefixes.
   - Severity: medium.
   - Report: `pbt-out/bug_reports/encode_smull_fp_as_gpr.md`
   - Regression: `test_encode_smull_regression_fp` (fails as witness).
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_smull_pbt) | 8 passing PBT + 4 passing KAT + 4 failing PBT + 4 failing regressions |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — coverage ledger row for encode_smull
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/FUNCTION_INDEX.md` — encode_smull marked PBT candidate
- `pbt-out/INVARIANTS.md` — encode_smull invariants prepended
- `pbt-out/bug_reports/encode_smull_extra_operand.md`
- `pbt-out/bug_reports/encode_smull_wrong_width.md`
- `pbt-out/bug_reports/encode_smull_sp_as_zr.md`
- `pbt-out/bug_reports/encode_smull_fp_as_gpr.md`

## Sweep close-out

Contract-surface sweep (standard, 1 round): `coverage_gaps` reported no instrumented profraw. Manual arm audit of `encode_smull` / `get_reg`: success path, arity 0..2, extra operand, wrong width, SP, FP, non-Reg kinds, invalid names. Added `encode_smull_diff_alt_spellings` (x31/w31, XZR/LR, uppercase) — passing, 1000 cases. Closed because the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 12:53 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 56/253 total | PBT candidates: 56 | Tested: 56 (100%) | 0 pass, 56 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 56 |
| **Tested (of PBT candidates)** | **56 / 56 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 56 / 0 |
| **Overall (tested / all functions)** | **56 / 253 (22%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 56 | 56 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 56 | 56 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 18 | 18 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 11 | 11 | 100% | covered |
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
