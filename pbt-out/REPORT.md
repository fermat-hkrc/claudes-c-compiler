# PBT Campaign Report: encode_mul

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mul
**Tests:** 15 properties (plus 4 KAT + 6 regression witnesses)
**Result:** 9 passing, 6 bugs
**Effort tier:** standard (1 coverage-driven sweep round). `coverage_gaps` had no LLVM profraw; manual arm audit added encode_mul_neg_non_register (passing) and encode_mul_neg_neon_mismatch_t (failing). Closed after the tier's one round.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mul | 15 properties (9 passing, 6 failing) | 6 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operand ignored
- **Law:** MUL takes exactly three register operands; a fourth must be rejected.
- **Minimal input:** `mul w0, w0, w0, x0`
- **Expected:** Err
- **Actual:** Ok(Word) same as `mul w0, w0, w0`
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_mul_extra_operand.md

### 2. Mixed X/W width accepted
- **Law:** Rd, Rn, Rm must all be 64-bit or all 32-bit (single sf bit).
- **Minimal input:** `mul w0, w0, x0`
- **Expected:** Err
- **Actual:** Ok(Word) with sf from Rd only
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_mul_mixed_width.md

### 3. SP/WSP encoded as ZR
- **Law:** Register 31 is XZR/WZR, never SP/WSP.
- **Minimal input:** `mul wsp, w0, w0`
- **Expected:** Err
- **Actual:** Ok(Word) encoding Rd=31 as WZR
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_mul_sp.md

### 4. FP/SIMD names treated as GPRs
- **Law:** Integer MUL is GPR-only.
- **Minimal input:** `mul d0, x1, x2`
- **Expected:** Err
- **Actual:** Ok(Word) encoding `d0` as register 0
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_mul_fp_reg.md

### 5. NEON 64-bit element MUL (size==11 UNDEFINED)
- **Law:** ARM ARM MUL (vector) is UNDEFINED when size==11; T not in {1D,2D}.
- **Minimal input:** `mul v0.1d, v0.1d, v0.1d`
- **Expected:** Err
- **Actual:** Ok(Word) with size=11
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_mul_neon_d.md

### 6. NEON mismatched T ignored
- **Law:** All three NEON operands must share arrangement T.
- **Minimal input:** `mul v0.8b, v0.8b, v0.16b`
- **Expected:** Err
- **Actual:** Ok(Word) using dest T only
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_mul_neon_mismatch_t.md

All six failures reproduced serially (`PBT_TEST_JOBS=1`).

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_mul_pbt) | 15 properties + 4 KAT + 6 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_mul_extra_operand.md
- pbt-out/bug_reports/encode_mul_mixed_width.md
- pbt-out/bug_reports/encode_mul_sp.md
- pbt-out/bug_reports/encode_mul_fp_reg.md
- pbt-out/bug_reports/encode_mul_neon_d.md
- pbt-out/bug_reports/encode_mul_neon_mismatch_t.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 09:27 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 43/184 total | PBT candidates: 43 | Tested: 43 (100%) | 0 pass, 43 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 43 |
| **Tested (of PBT candidates)** | **43 / 43 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 43 / 0 |
| **Overall (tested / all functions)** | **43 / 184 (23%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 43 | 43 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 43 | 43 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 13 | 13 | 100% | covered |
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
| encode_mul | data_processing.rs |
