# PBT Campaign Report: encode_neon_float_three_same

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_float_three_same
**Tests:** 11 properties + 2 KAT + 4 regression witnesses
**Result:** 7 passing, 4 bugs
**Effort tier:** standard (5–8 properties/target aim; 11 written after splitting negatives to surface all bugs; ≥1000 cases; 1 coverage-driven sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_float_three_same | 11 properties (7 pass / 4 fail) + 2 KAT pass + 4 failing regressions | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **Extra operand ignored.** Law: exactly three operands. Shrunk input: `fadd v0.2s, v0.2s, v0.2s, v0.2s`. Expected Err (llvm-mc rejects); actual Ok(Word) of the three-register form. Root cause: no arity check; operands beyond index 2 are unread. Impact: invalid GNU-style text is assembled. Severity: medium. Fix: reject `operands.len() != 3`. Report: `pbt-out/bug_reports/encode_neon_float_three_same_extra_operand.md`

2. **Bare source register accepted.** Law: Vn/Vm must be `Vn.T`. Shrunk input: `fadd v0.2s, v0, v0.2s`. Expected Err; actual Ok(Word). Root cause: `get_neon_reg` accepts `Operand::Reg` and source arrangement is discarded. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_float_three_same_src_reg_no_arrangement.md`

3. **Non-V prefix encoded as V.** Law: V registers only. Shrunk input: `fadd x0.2s, v0.2s, v0.2s`. Expected Err; actual Ok(Word) of `v0.2s`. Root cause: `parse_reg_num` accepts x/w/d/s/q/v/h/b. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_float_three_same_non_v_prefix.md`

4. **Arrangement mismatch encoded using dest T.** Law: dest T = Vn T = Vm T. Shrunk input: `fadd v0.2d, v0.2s, v0.2s`. Expected Err; actual Ok(Word) of the 2d form. Root cause: `let (rn, _)` / `let (rm, _)` discard source T. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_float_three_same_arrangement_mismatch.md`

All four reproduced serially (`PBT_TEST_JOBS=1`). Regression witnesses assert the gas-compatible Err contract and fail while the bugs exist.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_float_three_same_pbt) | 11 properties + 2 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_float_three_same_extra_operand.md
- pbt-out/bug_reports/encode_neon_float_three_same_src_reg_no_arrangement.md
- pbt-out/bug_reports/encode_neon_float_three_same_non_v_prefix.md
- pbt-out/bug_reports/encode_neon_float_three_same_arrangement_mismatch.md

Contract-surface sweep: 1 round (standard). `coverage_gaps` had no LLVM profraw; manual arm audit of `encode_neon_float_three_same` (get_neon_reg dest/Vn/Vm, match 2s/4s/2d/_, arity) showed every documented branch of this symbol was reached. Closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 07:06 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 34/184 total | PBT candidates: 34 | Tested: 34 (100%) | 0 pass, 34 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 34 |
| **Tested (of PBT candidates)** | **34 / 34 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 34 / 0 |
| **Overall (tested / all functions)** | **34 / 184 (18%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 34 | 34 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 34 | 34 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
| load_store.rs | 20 | 4 | 4 | 100% | covered |
| neon.rs | 68 | 5 | 5 | 100% | covered |

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
| encode_neon_sli | neon.rs |
| encode_ldur_stur | load_store.rs |
| encode_ldxp_stxp | load_store.rs |
| encode_neon_float_three_same | neon.rs |
