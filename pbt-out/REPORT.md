# PBT Campaign Report: encode_neon_shll

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_shll (src/backend/arm/assembler/encoder/neon.rs)
**Tests:** 11 properties + 2 KAT + 6 regression witnesses
**Result:** 6 passing properties, 5 failing properties (5 bugs); 2 KAT passing; 6 regression witnesses failing as intended
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 metamorphic/differential required, 1 strengthening/coverage-sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_shll | 11 properties (6 pass / 5 fail), 2 KAT, 6 regressions | 5 | differential (llvm-mc aarch64), algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. encode_neon_shll_neg_extra_operand — extra operands ignored
- **Failing property:** encode_neon_shll_neg_extra_operand (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, extra=0, tb=8b, shift=0, u_bit=0 → `sshll v0.8h, v0.8b, #0, v0.8h`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) — body only checks `operands.len() < 3`
- **Severity:** medium
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced.
- **Bug report:** pbt-out/bug_reports/encode_neon_shll_extra_operand.md
- **Regression test:** `test_encode_neon_shll_regression_extra_operand`

### 2. encode_neon_shll_neg_dest_tb — destination arrangement ignored
- **Failing property:** encode_neon_shll_neg_dest_tb (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, tb=8b, ta=8b, shift=0, u_bit=0 → `sshll v0.8b, v0.8b, #0`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) — dest arrangement discarded
- **Severity:** medium
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced.
- **Bug report:** pbt-out/bug_reports/encode_neon_shll_mismatched_dest_ta.md
- **Regression test:** `test_encode_neon_shll_regression_mismatched_dest_ta`

### 3. encode_neon_shll_neg_shift_oob — out-of-range shift accepted / Imm(-1) overflow
- **Failing property:** encode_neon_shll_neg_shift_oob (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, tb=8b, shift=-1, u_bit=0 (debug `attempt to add with overflow`). Related bound+1: shift=8 → `sshll v0.8h, v0.8b, #8` encodes as 16-bit esize.
- **Expected:** Err (llvm-mc: immediate in range [0, 7])
- **Actual:** panic on #-1; Ok(Word) with wrong immh on #8
- **Severity:** high
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced.
- **Bug report:** pbt-out/bug_reports/encode_neon_shll_shift_oob.md
- **Regression test:** `test_encode_neon_shll_regression_shift_neg` and `test_encode_neon_shll_regression_shift_oob`

### 4. encode_neon_shll_neg_gpr_dest — GPR/FP dest encoded as Vd
- **Failing property:** encode_neon_shll_neg_gpr_dest (negative_error)
- **Shrunk counterexample:** prefix=x, n=0, rn=0, tb=8b, shift=0, u_bit=0 → `sshll x0, v0.8b, #0`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) encoding Rd=0 via parse_reg_num
- **Severity:** medium
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced.
- **Bug report:** pbt-out/bug_reports/encode_neon_shll_gpr_dest.md
- **Regression test:** `test_encode_neon_shll_regression_gpr_dest`

### 5. encode_neon_shll_neg_src_vs_high — Q / Tb mismatch accepted
- **Failing property:** encode_neon_shll_neg_src_vs_high (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, tb=8b, shift=0, u_bit=0, is_high=true → `sshll2 v0.8h, v0.8b, #0`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) with Q=1 and 8-bit immh
- **Severity:** medium
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced.
- **Bug report:** pbt-out/bug_reports/encode_neon_shll_src_vs_high.md
- **Regression test:** `test_encode_neon_shll_regression_src_vs_high`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_shll_pbt) | 11 properties, 2 KAT, 6 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_shll_extra_operand.md
- pbt-out/bug_reports/encode_neon_shll_mismatched_dest_ta.md
- pbt-out/bug_reports/encode_neon_shll_shift_oob.md
- pbt-out/bug_reports/encode_neon_shll_gpr_dest.md
- pbt-out/bug_reports/encode_neon_shll_src_vs_high.md

## Contract-surface sweep

STANDARD owes 1 round. `coverage_gaps` had no LLVM profraw; sweep was a manual arm audit of encode_neon_shll (arity < 3, unsupported Tb, non-matching kinds, invalid names, GPR dest, Q vs Tb). Added `encode_neon_shll_neg_gpr_dest` (failing), `encode_neon_shll_neg_src_vs_high` (failing), and `encode_neon_shll_neg_arity_kinds` (passing). Close: tier round done.

## Harness

- **Test layout:** inline `#[cfg(test)] mod encode_neon_shll_pbt` in neon.rs
- **Buildability probe:** `cargo test --lib test_ascii_passthrough` → 1 passed, 1521 filtered out
- **Harness placement:** rung 1 — extend existing `cargo test --lib` (proptest already in Cargo.toml)
- **Build contract:** `cargo check --lib` (user-supplied); tests via `cargo test --lib encode_neon_shll_pbt`

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 12:18 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 54/253 total | PBT candidates: 54 | Tested: 54 (100%) | 0 pass, 54 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 54 |
| **Tested (of PBT candidates)** | **54 / 54 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 54 / 0 |
| **Overall (tested / all functions)** | **54 / 253 (21%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 54 | 54 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 54 | 54 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 17 | 17 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 10 | 10 | 100% | covered |
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
