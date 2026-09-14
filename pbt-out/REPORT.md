# PBT Campaign Report: encode_msub

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_msub
**Tests:** 12 properties (plus 3 KAT + 4 regression witnesses)
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; coverage_gaps had no profraw — manual arm audit of get_reg 0..3 / sf from Rd / o0=1 / extra operand / mixed width / SP / FP)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_msub | 12 properties + 3 KAT + 4 regressions | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operands beyond index 3 are ignored
- **Law:** MSUB takes exactly four register operands
- **Shrunk input:** `msub w0, w0, w0, w0, x0`
- **Expected:** Err (llvm-mc: invalid operand for instruction)
- **Actual:** Ok(Word) — same as `msub w0, w0, w0, w0`; extra operand ignored
- **Root cause:** encode_msub only calls get_reg(0..3); no arity upper bound
- **Impact:** a typo extra operand is assembled instead of diagnosed
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_msub_extra_operand.md
- **Regression:** `test_encode_msub_regression_extra_operand` (fails, as required)

### 2. Mixed X/W register widths are accepted
- **Law:** ARM MSUB uses a single sf bit; all four registers must be the same width
- **Shrunk input:** `msub w0, w0, w0, x0`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) — sf taken only from Rd; Rn/Rm/Ra widths discarded
- **Root cause:** `let (rn, _) = get_reg(...)` (and Rm/Ra) ignore is_64
- **Impact:** the object file contains a different instruction than the source text
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_msub_mixed_width.md
- **Regression:** `test_encode_msub_regression_mixed_width` (fails, as required)

### 3. SP/WSP is encoded as XZR/WZR
- **Law:** register 31 is WZR/XZR, never WSP/SP
- **Shrunk input:** `msub wsp, w0, w0, w0`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) — same encoding as `msub wzr, w0, w0, w0`
- **Root cause:** parse_reg_num maps sp/wsp to 31
- **Impact:** writes WZR instead of addressing the stack pointer
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/encode_msub_sp.md
- **Regression:** `test_encode_msub_regression_sp` (fails, as required)

### 4. FP/SIMD register names are accepted as GPRs
- **Law:** integer MSUB is GPR-only
- **Shrunk input:** `msub d0, x1, x2, x3`
- **Expected:** Err (llvm-mc: invalid operand)
- **Actual:** Ok(Word) — encodes as 32-bit MSUB with Rd=0
- **Root cause:** parse_reg_num accepts d/s/q/v/h/b prefixes; is_64bit_reg is false so sf=0
- **Impact:** object file contains a GPR multiply-subtract, not an FP instruction and not a diagnostic
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_msub_fp_reg.md
- **Regression:** `test_encode_msub_regression_fp_reg` (fails, as required)

Serial reconfirm: all four property failures reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_msub_pbt) | 12 properties + 3 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign phases
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/FUNCTION_INDEX.md — function index (encode_msub marked candidate)
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/bug_reports/encode_msub_extra_operand.md
- pbt-out/bug_reports/encode_msub_mixed_width.md
- pbt-out/bug_reports/encode_msub_sp.md
- pbt-out/bug_reports/encode_msub_fp_reg.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 09:10 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 42/184 total | PBT candidates: 42 | Tested: 42 (100%) | 0 pass, 42 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 42 |
| **Tested (of PBT candidates)** | **42 / 42 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 42 / 0 |
| **Overall (tested / all functions)** | **42 / 184 (23%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 42 | 42 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 42 | 42 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 12 | 12 | 100% | covered |
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
| encode_msub | data_processing.rs |
