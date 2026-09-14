# PBT Campaign Report: encode_csetm

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_csetm
**Tests:** 11 properties (plus 3 KAT + 5 regression witnesses)
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_csetm | 11 properties (8 passing, 3 failing) + 3 KAT + 5 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_csetm ignores extra operands** — `encode_csetm_neg_extra_operand`. Shrunk: `[Reg("x0"), Cond("eq"), Reg("x2")]`. Expected Err (llvm-mc: invalid operand). Actual `Ok(Word(0xda9f13e0))` — operands beyond index 1 are ignored. Severity: medium. Report: `pbt-out/bug_reports/encode_csetm_extra_operand.md`. Serial: `PBT_TEST_JOBS=1 cargo test --lib encode_csetm_neg -- --test-threads=1` reproduced.

2. **encode_csetm accepts AL and NV** — `encode_csetm_neg_al_nv`. Shrunk: `[Reg("x0"), Cond("al")]`. Expected Err (ARM ARM CSETM alias not valid for AL/NV; llvm-mc: "condition codes AL and NV are invalid for this instruction"). Actual `Ok(Word(0xda9ff3e0))` for AL and `Ok(Word(0xda9fe3e0))` for NV. Severity: medium. Report: `pbt-out/bug_reports/encode_csetm_al_nv.md`. Serial reproduced.

3. **encode_csetm encodes SP as XZR** — `encode_csetm_neg_wrong_reg`. Shrunk: `[Reg("sp"), Cond("eq")]` (kind=0, n=0). Expected Err (register 31 is XZR/WZR). Actual `Ok(Word(0xda9f13ff))` = `csetm xzr, eq`. Severity: medium. Report: `pbt-out/bug_reports/encode_csetm_sp_as_zr.md`. Serial reproduced.

4. **encode_csetm accepts FP/SIMD register names as GPRs** — `encode_csetm_neg_wrong_reg` / `test_encode_csetm_regression_fp_reg`. Witness: `[Reg("d0"), Cond("eq")]`. Expected Err. Actual `Ok(Word(0x5a9f13e0))` = W-form CSETM of w0. Severity: medium. Report: `pbt-out/bug_reports/encode_csetm_fp_reg.md`. Serial: wrong_reg shrinks to SP; FP confirmed by the dedicated regression test.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (`mod encode_csetm_pbt`) | 11 properties + 3 KAT + 5 regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_csetm now a candidate)
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/INVARIANTS.md` — confirmed invariants for encode_csetm
- `pbt-out/REPORT.md` — this report
- `pbt-out/bug_reports/encode_csetm_extra_operand.md`
- `pbt-out/bug_reports/encode_csetm_al_nv.md`
- `pbt-out/bug_reports/encode_csetm_sp_as_zr.md`
- `pbt-out/bug_reports/encode_csetm_fp_reg.md`

## Contract-surface sweep

STANDARD owes 1 round. `coverage_gaps` had no LLVM profraw in this session. Sweep was a manual arm audit of documented error paths in `encode_csetm` / `get_reg` / `encode_cond` / `parse_reg_num`: encode_cond None, parse_reg_num None, get_reg non-Reg, cond-not-Cond. Three properties added (`encode_csetm_neg_unknown_cond`, `encode_csetm_neg_invalid_name`, `encode_csetm_neg_bad_operand_kind`); all passing (1000 cases). Extra/AL-NV/SP/FP remain failing witnesses of documented gas-compat / ARM ARM contracts. Closed because the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 04:00 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 22/184 total | PBT candidates: 22 | Tested: 22 (100%) | 0 pass, 22 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 22 |
| **Tested (of PBT candidates)** | **22 / 22 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 22 / 0 |
| **Overall (tested / all functions)** | **22 / 184 (12%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 22 | 22 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 22 | 22 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 14 | 14 | 100% | covered |
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
