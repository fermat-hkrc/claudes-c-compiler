# PBT Campaign Report: encode_neon_float_cmp_zero

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_neon_float_cmp_zero
**Tests:** 9 properties (plus 3 KAT + 3 regression witnesses)
**Result:** 6 passing, 3 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep; generator runs=1000)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_float_cmp_zero | 9 properties (6 pass, 3 fail) + 3 KAT pass + 3 regression fail | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operands ignored
- **Law:** Vector FCM* #0.0 takes `Vd.T, Vn.T, #0.0` only; a fourth operand must be Err.
- **Shrunk counterexample:** `fcmeq v0.2s, v0.2s, #0.0, v0.2s` (rd=0, rn=0, extra=0, t="2s", extra_kind=0).
- **Expected:** Err. **Actual:** Ok(Word) — only operands[0] and operands[1] are read; no arity check.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_float_cmp_zero_extra_operand.md
- **Regression test:** `test_encode_neon_float_cmp_zero_regression_extra_operand` (fails, as intended)

### 2. Source arrangement ignored
- **Law:** ARM ARM requires the same T on dest and source (`<Vd>.<T>, <Vn>.<T>, #0.0`).
- **Shrunk counterexample:** `fcmeq v0.4s, v0.2s, #0.0` (rd=0, rn=0, td="4s", tn="2s"). Encodes as `fcmeq v0.4s, v0.4s, #0.0`.
- **Expected:** Err. **Actual:** Ok(Word) — Q/sz taken only from dest; `let (rn, _) = get_neon_reg(operands, 1)`.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_float_cmp_zero_arrangement_mismatch.md
- **Regression test:** `test_encode_neon_float_cmp_zero_regression_arrangement_mismatch` (fails, as intended)

### 3. Non-V register prefix accepted
- **Law:** Vector FCM* #0.0 takes V registers; x/w/d/s/q/h/b prefixes must be Err.
- **Shrunk counterexample:** `fcmeq x0.2s, v0.2s, #0.0` (rd=0, rn=0, t="2s", prefix="x", which=0). Encodes as `fcmeq v0.2s, v0.2s, #0.0`.
- **Expected:** Err. **Actual:** Ok(Word) — `parse_reg_num` maps x/w/d/s/q/v/h/b with num<=31 to the same 5-bit number.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_float_cmp_zero_non_v_prefix.md
- **Regression test:** `test_encode_neon_float_cmp_zero_regression_non_v_prefix` (fails, as intended)

Serial reconfirmation: all three failures reproduced with `PBT_TEST_JOBS=1 cargo test --lib … -- --test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_float_cmp_zero_pbt) | 9 properties + 3 KAT + 3 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_float_cmp_zero_extra_operand.md
- pbt-out/bug_reports/encode_neon_float_cmp_zero_arrangement_mismatch.md
- pbt-out/bug_reports/encode_neon_float_cmp_zero_non_v_prefix.md

Contract-surface sweep closed after 1 round (standard tier): `coverage_gaps` had no LLVM profraw; manual arm audit of get_neon_reg Operand::Reg dest (empty arrangement → Err as specified) and non-V prefix (new failing property, bug 3). Sweep closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 05:41 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 30/184 total | PBT candidates: 30 | Tested: 30 (100%) | 0 pass, 30 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 30 |
| **Tested (of PBT candidates)** | **30 / 30 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 30 / 0 |
| **Overall (tested / all functions)** | **30 / 184 (16%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 30 | 30 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 30 | 30 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
| load_store.rs | 20 | 2 | 2 | 100% | covered |
| neon.rs | 68 | 3 | 3 | 100% | covered |

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
| encode_div | data_processing.rs |
| encode_eon | data_processing.rs |
| encode_ldar_stlr | load_store.rs |
| encode_neon_across_long | neon.rs |
| encode_neon_float_cmp_zero | neon.rs |
