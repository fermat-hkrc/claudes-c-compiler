# PBT Campaign Report: encode_br

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_br
**Tests:** 9 properties (plus 2 KAT + 4 regression witnesses)
**Result:** 6 passing, 3 failing (4 SUT bugs)
**Effort tier:** standard (1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_br | 9 properties (6 pass / 3 fail) | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_br_neg_w_reg** (negative_error). Law: ∀ n ∈ {0..30} ∪ {wzr, wsp}. encode_br([Reg(wn)]) is Err. Shrunk counterexample: `n = 0` (`br w0`). Expected: Err. Actual: Ok(Word(0xd61f0000)) — same as `br x0`. encode_br discards the is_64 flag from get_reg. Serial reconfirm (`PBT_TEST_JOBS=1`). Report: `pbt-out/bug_reports/encode_br_w_reg.md`. Regression: `test_encode_br_regression_w_reg`.

2. **encode_br_neg_extra_operand** (negative_error). Law: ∀ extra ∈ {Reg,Imm,Symbol,Mem}, encode_br([Reg(xn), extra]) is Err. Shrunk counterexample: `n = 0, which = 0` (`br x0, x1`). Expected: Err. Actual: Ok(Word(0xd61f0000)). get_reg only inspects operand 0. Serial reconfirm (`PBT_TEST_JOBS=1`). Report: `pbt-out/bug_reports/encode_br_extra_operand.md`. Regression: `test_encode_br_regression_extra_operand`.

3. **encode_br_neg_wrong_reg_class** (negative_error) — SP. Law: encode_br([Reg("sp")]) is Err. Shrunk counterexample: `which = 0, n = 0` (`br sp`). Expected: Err. Actual: Ok(Word(0xd61f03e0)) — same as `br xzr`. parse_reg_num maps sp to 31. Serial reconfirm (`PBT_TEST_JOBS=1`). Report: `pbt-out/bug_reports/encode_br_sp_as_zr.md`. Regression: `test_encode_br_regression_sp`.

4. **encode_br_neg_wrong_reg_class** (negative_error) — FP. Law: encode_br([Reg("d0")]) is Err. Witness: `br d0`. Expected: Err. Actual: Ok(Word(0xd61f0000)) — treated as x0. parse_reg_num maps d/s/q/v/h/b prefixes. Serial reconfirm via `test_encode_br_regression_fp_reg` (`PBT_TEST_JOBS=1`). Report: `pbt-out/bug_reports/encode_br_fp_reg.md`. Regression: `test_encode_br_regression_fp_reg`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_br_pbt) | 9 properties + 2 KAT + 4 regressions |

## Output Directories

- `pbt-out/PLAN.md` — campaign phases
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_br marked yes)
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/INVARIANTS.md` — confirmed invariants (encode_br section appended)
- `pbt-out/REPORT.md` — this report
- `pbt-out/bug_reports/encode_br_w_reg.md`
- `pbt-out/bug_reports/encode_br_extra_operand.md`
- `pbt-out/bug_reports/encode_br_sp_as_zr.md`
- `pbt-out/bug_reports/encode_br_fp_reg.md`

Contract-surface sweep: 1 round (tier allowance). `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_reg arms reachable from encode_br (empty / non-Reg covered by arity and bad_operand; parse_reg_num None covered by `encode_br_neg_invalid_name`; SP/W/FP/extra filed as bugs). Closed because the tier's 1 round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 01:45 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 11/184 total | PBT candidates: 11 | Tested: 11 (100%) | 0 pass, 11 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 11 |
| **Tested (of PBT candidates)** | **11 / 11 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 11 / 0 |
| **Overall (tested / all functions)** | **11 / 184 (6%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 11 | 11 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 11 | 11 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 3 | 3 | 100% | covered |
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
