# PBT Campaign Report: encode_neon_across_long

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_neon_across_long
**Tests:** 8 properties (plus 3 KAT + 3 regression witnesses)
**Result:** 5 passing, 3 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep; generator runs=1000)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_across_long | 8 properties (5 pass, 3 fail) + 3 KAT pass + 3 regression fail | 3 | differential, algebraic.round_trip, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operands ignored
- **Law:** SADDLV/UADDLV take exactly two operands.
- **Shrunk counterexample:** `saddlv h0, v0.8b, h0` (rd=0, rn=0, extra=0, u=0, t=8b, extra_kind=0).
- **Expected:** Err. **Actual:** Ok(Word) — `len < 2` does not reject len>2.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_across_long_extra_operand.md
- **Regression test:** `test_encode_neon_across_long_regression_extra_operand` (fails, as intended)

### 2. Reserved source arrangement T=2S (also 1D/2D) encoded
- **Law:** ARM ARM T is only 8B/16B/4H/8H/4S; size=11 and 2S are reserved.
- **Shrunk counterexample:** `saddlv h0, v0.2s` (rd=0, rn=0, u=0, t="2s").
- **Expected:** Err. **Actual:** Ok(Word) with Q=0, size=10. Root cause: `neon_arr_to_q_size` maps 2s/1d/2d.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_across_long_reserved_arrangement.md
- **Regression test:** `test_encode_neon_across_long_regression_reserved_2s` (fails, as intended)

### 3. Destination register type ignored
- **Law:** Dest `<V><d>` is H for 8B/16B, S for 4H/8H, D for 4S — never B/Q/GPR/vector arrangement.
- **Shrunk counterexample:** `saddlv b0, v0.8b` (rd=0, rn=0, u=0, t=8b, prefix=b, as_arr=false). Encodes as `saddlv h0, v0.8b`.
- **Expected:** Err. **Actual:** Ok(Word) with Rd=0. Dest prefix and RegArrangement dest arrangement are ignored.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_across_long_dest_type.md
- **Regression test:** `test_encode_neon_across_long_regression_dest_type` (fails, as intended)

Serial reconfirmation: all three failures reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_across_long_neg -- --test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_across_long_pbt) | 8 properties + 3 KAT + 3 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_across_long_extra_operand.md
- pbt-out/bug_reports/encode_neon_across_long_reserved_arrangement.md
- pbt-out/bug_reports/encode_neon_across_long_dest_type.md

Contract-surface sweep closed after 1 round (standard tier): `coverage_gaps` had no LLVM profraw; manual arm audit of dest `_` (non-Reg) and parse_reg_num None on dest RegArrangement — generators extended; those paths Err as specified. Sweep closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 05:26 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 29/184 total | PBT candidates: 29 | Tested: 29 (100%) | 0 pass, 29 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 29 |
| **Tested (of PBT candidates)** | **29 / 29 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 29 / 0 |
| **Overall (tested / all functions)** | **29 / 184 (16%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 29 | 29 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 29 | 29 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
| load_store.rs | 20 | 2 | 2 | 100% | covered |
| neon.rs | 68 | 2 | 2 | 100% | covered |

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
