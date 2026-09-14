# PBT Campaign Report: encode_csneg

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_csneg
**Tests:** 10 properties (plus 4 KAT + 4 regression witnesses)
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_csneg | 10 properties (8 passing, 2 failing) + 4 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_csneg ignores extra operands** — `encode_csneg_neg_extra_operand`. Shrunk: `[Reg("x0"), Reg("x0"), Reg("x0"), Cond("eq"), Reg("x3")]`. Expected Err (llvm-mc: invalid operand). Actual `Ok(Word(0xda800400))` — operands beyond index 3 are ignored. Severity: medium. Report: `pbt-out/bug_reports/encode_csneg_extra_operand.md`. Serial: `PBT_TEST_JOBS=1 cargo test --lib encode_csneg_neg -- --test-threads=1` reproduced.

2. **encode_csneg encodes SP as XZR** — `encode_csneg_neg_wrong_reg`. Shrunk: `[Reg("sp"), Reg("x0"), Reg("x0"), Cond("eq")]` (kind=0, n=0). Expected Err (register 31 is XZR/WZR). Actual `Ok(Word)` = `csneg xzr, x0, x0, eq`. Severity: medium. Report: `pbt-out/bug_reports/encode_csneg_sp_as_zr.md`. Serial reproduced.

3. **encode_csneg accepts mixed x/w register widths** — `encode_csneg_neg_wrong_reg` / `test_encode_csneg_regression_mixed_width`. Witness: `[Reg("x0"), Reg("w1"), Reg("x2"), Cond("eq")]`. Expected Err. Actual `Ok(Word(0xda820420))` — sf is taken only from operand 0. Severity: medium. Report: `pbt-out/bug_reports/encode_csneg_mixed_width.md`. Serial: wrong_reg shrinks to SP; mixed width confirmed by the dedicated regression test.

4. **encode_csneg accepts FP/SIMD register names as GPRs** — `encode_csneg_neg_wrong_reg` / `test_encode_csneg_regression_fp_reg`. Witness: `[Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]`. Expected Err. Actual `Ok(Word(0x5a820420))` = W-form CSNEG of w0/w1/w2. Severity: medium. Report: `pbt-out/bug_reports/encode_csneg_fp_as_gpr.md`. Serial: wrong_reg shrinks to SP; FP confirmed by the dedicated regression test.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (`mod encode_csneg_pbt`) | 10 properties + 4 KAT + 4 regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_csneg now a candidate)
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/INVARIANTS.md` — confirmed invariants for encode_csneg
- `pbt-out/REPORT.md` — this report
- `pbt-out/bug_reports/encode_csneg_extra_operand.md`
- `pbt-out/bug_reports/encode_csneg_sp_as_zr.md`
- `pbt-out/bug_reports/encode_csneg_mixed_width.md`
- `pbt-out/bug_reports/encode_csneg_fp_as_gpr.md`

## Contract-surface sweep

STANDARD owes 1 round. `coverage_gaps` had no LLVM profraw in this session. Sweep was a manual arm audit of documented error paths in `encode_csneg` / `get_reg` / `encode_cond` / `parse_reg_num`: encode_cond None, parse_reg_num None, get_reg non-Reg, cond-not-Cond. Two properties added (`encode_csneg_neg_invalid_name`, `encode_csneg_neg_bad_operand_kind`); both passing (1000 cases). Extra/SP/mixed/FP remain failing witnesses of documented gas-compat / ARM ARM contracts. Closed because the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 04:33 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 25/184 total | PBT candidates: 25 | Tested: 25 (100%) | 0 pass, 25 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 25 |
| **Tested (of PBT candidates)** | **25 / 25 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 25 / 0 |
| **Overall (tested / all functions)** | **25 / 184 (14%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 25 | 25 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 25 | 25 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 4 | 4 | 100% | covered |
| load_store.rs | 20 | 1 | 1 | 100% | covered |
| neon.rs | 68 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
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
