# PBT Campaign Report: encode_neon_qshrn

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_qshrn
**Tests:** 10 properties (plus 2 KAT + 5 regression witnesses)
**Result:** 6 passing, 5 bugs
**Effort tier:** standard (1 coverage-driven sweep round; coverage_gaps had no profraw — manual arm audit of arity / Ta match / get_imm as u32 / get_neon_reg Reg dest and source)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_qshrn | 10 properties + 2 KAT + 5 regressions | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Shift range uses source element size, not dest element size
- **Law:** shift ∈ [1, dest_esize] (8/16/32 for Ta 8H/4S/2D)
- **Shrunk input:** `sqshrn v0.8b, v0.8h, #9`
- **Expected:** Err (llvm-mc: immediate must be in [1, 8])
- **Actual:** Ok(Word) with immh=0000 (reserved / different encoding group)
- **Root cause:** `if shift == 0 || shift > element_bits` uses source size 16/32/64; sibling `encode_neon_shrn` correctly uses `half_bits = element_bits / 2`
- **Impact:** silent wrong encoding of an assembler-invalid shift
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/encode_neon_qshrn_shift_oob.md
- **Regression:** `test_encode_neon_qshrn_regression_shift_oob_dest_esize` (fails, as required)

### 2. Destination arrangement Tb is ignored
- **Law:** Vd.Tb must match Ta and the `2` suffix
- **Shrunk input:** `sqshrn v0.4h, v0.8h, #1`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) — `get_neon_reg(operands, 0)` discards dest arrangement
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_qshrn_mismatched_dest_tb.md
- **Regression:** `test_encode_neon_qshrn_regression_mismatched_dest_tb` (fails, as required)

### 3. Extra operands beyond index 2 are ignored
- **Law:** exactly three operands
- **Shrunk input:** `sqshrn v0.8b, v0.8h, #1, v0.8b`
- **Expected:** Err
- **Actual:** Ok(Word) — only `len < 3` is checked
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_qshrn_extra_operand.md
- **Regression:** `test_encode_neon_qshrn_regression_extra_operand` (fails, as required)

### 4. GPR/FP dest is accepted as Vd
- **Law:** dest must be Vd.Tb
- **Shrunk input:** dest=`x0` (`Operand::Reg("x0")`)
- **Expected:** Err
- **Actual:** Ok(Word) via `get_neon_reg` `Operand::Reg` + `parse_reg_num`. Reachable from `uqshrn`/`sqshrn2`/`sqrshrn`/`uqrshrn` (+2), which dispatch unconditionally.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_qshrn_gpr_dest.md
- **Regression:** `test_encode_neon_qshrn_regression_gpr_dest` (fails, as required)

### 5. i64 shift truncated with `as u32`
- **Law:** the assembler immediate must be in [1, dest_esize]; high bits must not be dropped
- **Shrunk input:** `Imm(4294967297)` (`1 + 2^32`)
- **Expected:** Err
- **Actual:** Ok(Word) encoding shift `#1`
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_qshrn_shift_i64_trunc.md
- **Regression:** `test_encode_neon_qshrn_regression_shift_i64_trunc` (fails, as required)

Serial reconfirm: all five failures reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_qshrn_pbt) | 10 properties + 2 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_qshrn_shift_oob.md
- pbt-out/bug_reports/encode_neon_qshrn_mismatched_dest_tb.md
- pbt-out/bug_reports/encode_neon_qshrn_extra_operand.md
- pbt-out/bug_reports/encode_neon_qshrn_gpr_dest.md
- pbt-out/bug_reports/encode_neon_qshrn_shift_i64_trunc.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 08:57 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 41/184 total | PBT candidates: 41 | Tested: 41 (100%) | 0 pass, 41 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 41 |
| **Tested (of PBT candidates)** | **41 / 41 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 41 / 0 |
| **Overall (tested / all functions)** | **41 / 184 (22%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 41 | 41 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 41 | 41 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 11 | 11 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 6 | 6 | 100% | covered |

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
| encode_ldxr_stxr | load_store.rs |
| encode_logical | data_processing.rs |
| encode_madd | data_processing.rs |
| encode_movk | data_processing.rs |
| encode_movn | data_processing.rs |
| encode_movz | data_processing.rs |
| encode_neon_qshrn | neon.rs |
