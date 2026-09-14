# PBT Campaign Report: encode_csinc

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_csinc
**Tests:** 10 properties (plus 4 KAT + 4 regression witnesses)
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_csinc | 10 properties (8 passing, 2 failing) + 4 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_csinc ignores extra operands** — `encode_csinc_neg_extra_operand`. Shrunk: `[Reg("x0"), Reg("x0"), Reg("x0"), Cond("eq"), Reg("x3")]`. Expected Err (llvm-mc: invalid operand). Actual `Ok(Word)` — operands beyond index 3 are ignored. Severity: medium. Report: `pbt-out/bug_reports/encode_csinc_extra_operand.md`. Serial: `PBT_TEST_JOBS=1 cargo test --lib encode_csinc_neg -- --test-threads=1` reproduced.

2. **encode_csinc encodes SP as XZR** — `encode_csinc_neg_wrong_reg`. Shrunk: `[Reg("sp"), Reg("x0"), Reg("x0"), Cond("eq")]` (kind=0, n=0). Expected Err (register 31 is XZR/WZR). Actual `Ok(Word)` = `csinc xzr, x0, x0, eq`. Severity: medium. Report: `pbt-out/bug_reports/encode_csinc_sp_as_zr.md`. Serial reproduced.

3. **encode_csinc accepts mixed x/w register widths** — `encode_csinc_neg_wrong_reg` / `test_encode_csinc_regression_mixed_width`. Witness: `[Reg("x0"), Reg("w1"), Reg("x2"), Cond("eq")]`. Expected Err. Actual `Ok(Word)` — sf is taken only from operand 0. Severity: medium. Report: `pbt-out/bug_reports/encode_csinc_mixed_width.md`. Serial: wrong_reg shrinks to SP; mixed width confirmed by the dedicated regression test.

4. **encode_csinc accepts FP/SIMD register names as GPRs** — `encode_csinc_neg_wrong_reg` / `test_encode_csinc_regression_fp_reg`. Witness: `[Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]`. Expected Err. Actual `Ok(Word)` = W-form CSINC of w0/w1/w2. Severity: medium. Report: `pbt-out/bug_reports/encode_csinc_fp_reg.md`. Serial: wrong_reg shrinks to SP; FP confirmed by the dedicated regression test.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (`mod encode_csinc_pbt`) | 10 properties + 4 KAT + 4 regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_csinc now a candidate)
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/INVARIANTS.md` — confirmed invariants for encode_csinc
- `pbt-out/REPORT.md` — this report
- `pbt-out/bug_reports/encode_csinc_extra_operand.md`
- `pbt-out/bug_reports/encode_csinc_sp_as_zr.md`
- `pbt-out/bug_reports/encode_csinc_mixed_width.md`
- `pbt-out/bug_reports/encode_csinc_fp_reg.md`

## Contract-surface sweep

STANDARD owes 1 round. `coverage_gaps` had no LLVM profraw in this session. Sweep was a manual arm audit of documented error paths in `encode_csinc` / `get_reg` / `encode_cond` / `parse_reg_num`: encode_cond None, parse_reg_num None, get_reg non-Reg, cond-not-Cond. Two properties added (`encode_csinc_neg_invalid_name`, `encode_csinc_neg_bad_operand_kind`); both passing (1000 cases). Extra/SP/mixed/FP remain failing witnesses of documented gas-compat / ARM ARM contracts. Closed because the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 04:12 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 23/184 total | PBT candidates: 23 | Tested: 23 (100%) | 0 pass, 23 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 23 |
| **Tested (of PBT candidates)** | **23 / 23 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 23 / 0 |
| **Overall (tested / all functions)** | **23 / 184 (13%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 23 | 23 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 23 | 23 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 15 | 15 | 100% | covered |
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
