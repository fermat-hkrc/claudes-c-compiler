# PBT Campaign Report: encode_sxth

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_sxth
**Tests:** 11 properties (7 passing, 4 failing) plus 4 passing KAT and 4 failing regression witnesses
**Result:** 7 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; closed because the tier's round is done — coverage_gaps had no profraw, so the round was a manual ARM-contract audit)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_sxth | 11 properties + 4 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### encode_sxth_neg_extra_operand
- **Failing property:** encode_sxth_neg_extra_operand (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, is_64=false, extra=Reg("x0") — `sxth w0, w0, x0`
- **Expected:** Err
- **Actual:** Ok(Word) — third operand ignored
- **Law:** SXTH has exactly two register operands (llvm-mc rejects a third)
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxth_extra_operand.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg_extra_operand -- --test-threads=1`

### encode_sxth_neg_wd_xn
- **Failing property:** encode_sxth_neg_wd_xn (negative_error)
- **Shrunk counterexample:** rd=0, rn=0 — `sxth w0, x0`
- **Expected:** Err
- **Actual:** Ok(Word) — 32-bit SXTH encoded
- **Law:** ARM ARM 32-bit form is SXTH Wd, Wn; llvm-mc rejects W dest with X source
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxth_wd_xn.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg_wd_xn -- --test-threads=1`

### encode_sxth_neg_sp
- **Failing property:** encode_sxth_neg_sp (negative_error)
- **Shrunk counterexample:** which=0, is_64_sp=false, a=0, dest64=false — `sxth wsp, w0`
- **Expected:** Err
- **Actual:** Ok(Word) — WSP encoded as WZR
- **Law:** SXTH register 31 is WZR/XZR, never SP/WSP; llvm-mc rejects SP/WSP
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxth_sp.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg_sp -- --test-threads=1`

### encode_sxth_neg_fp
- **Failing property:** encode_sxth_neg_fp (negative_error)
- **Shrunk counterexample:** which=0, prefix="d", n=0 — `sxth d0, w1`
- **Expected:** Err
- **Actual:** Ok(Word) — FP name encoded as GPR w0
- **Law:** SXTH is a GPR instruction; llvm-mc rejects FP/SIMD operands
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxth_fp.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxth_neg_fp -- --test-threads=1`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_sxth_pbt) | 11 properties + 4 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_sxth_extra_operand.md
- pbt-out/bug_reports/encode_sxth_wd_xn.md
- pbt-out/bug_reports/encode_sxth_sp.md
- pbt-out/bug_reports/encode_sxth_fp.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 13:08 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 57/253 total | PBT candidates: 57 | Tested: 57 (100%) | 0 pass, 57 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 57 |
| **Tested (of PBT candidates)** | **57 / 57 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 57 / 0 |
| **Overall (tested / all functions)** | **57 / 253 (23%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 57 | 57 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 57 | 57 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 19 | 19 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 11 | 11 | 100% | covered |
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
| encode_neon_sqshrun | neon.rs |
| encode_smull | data_processing.rs |
| encode_sxth | data_processing.rs |
