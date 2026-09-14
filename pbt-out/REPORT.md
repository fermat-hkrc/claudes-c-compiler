# PBT Campaign Report: encode_csel

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_csel
**Tests:** 10 properties (8 passing, 2 failing) plus 3 passing KATs and 4 failing regression witnesses
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_csel | 10 properties | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **extra operand ignored** — `encode_csel_neg_extra_operand`. Shrunk CE: `[Reg("x0"), Reg("x1"), Reg("x2"), Cond("eq"), Reg("x3")]`. Expected Err; actual Ok(Word) encoding `csel x0, x1, x2, eq`. The encoder only inspects operands 0..3. Severity: medium. Report: `pbt-out/bug_reports/encode_csel_extra_operand.md`.

2. **SP/WSP encoded as XZR/WZR** — `encode_csel_neg_wrong_reg`. Shrunk CE: kind=0 n=0, `[Reg("sp"), Reg("x0"), Reg("x0"), Cond("eq")]`. Expected Err; actual Ok(Word) with Rd=31 (XZR). Severity: medium. Report: `pbt-out/bug_reports/encode_csel_sp_as_zr.md`.

3. **mixed x/w accepted** — same property, isolated via regression. CE: `[Reg("x0"), Reg("w1"), Reg("x2"), Cond("eq")]`. Expected Err; actual Ok(Word) with sf from Rd only. Severity: medium. Report: `pbt-out/bug_reports/encode_csel_mixed_width.md`.

4. **FP/SIMD names accepted as GPRs** — same property, isolated via regression. CE: `[Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]`. Expected Err; actual encodes as W-form CSEL of W0/W1/W2. Severity: medium. Report: `pbt-out/bug_reports/encode_csel_fp_reg.md`.

Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_csel_neg -- --test-threads=1` reproduced both property failures; `PBT_TEST_JOBS=1 cargo test --lib test_encode_csel_regression -- --test-threads=1` reproduced all four regression witnesses.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_csel_pbt) | 10 properties + 3 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — per-function coverage ledger
- pbt-out/COVERAGE_STATUS.md — aggregate coverage
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/INVARIANTS.md — confirmed invariants (prepended)
- pbt-out/bug_reports/encode_csel_extra_operand.md
- pbt-out/bug_reports/encode_csel_sp_as_zr.md
- pbt-out/bug_reports/encode_csel_mixed_width.md
- pbt-out/bug_reports/encode_csel_fp_reg.md

Contract-surface sweep: 1 round (standard tier). `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit of encode_csel's encode_cond None arm, get_reg parse_reg_num None arm, and get_reg non-Reg / cond-not-Cond arms. Two targeted properties (`encode_csel_neg_unknown_cond`, `encode_csel_neg_invalid_name`) were added and both passed. Close reason: the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 03:37 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 20/184 total | PBT candidates: 20 | Tested: 20 (100%) | 0 pass, 20 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 20 |
| **Tested (of PBT candidates)** | **20 / 20 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 20 / 0 |
| **Overall (tested / all functions)** | **20 / 184 (11%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 20 | 20 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 20 | 20 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 12 | 12 | 100% | covered |
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
