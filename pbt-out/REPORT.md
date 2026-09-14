# PBT Campaign Report: encode_sxtw

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_sxtw
**Tests:** 11 properties (7 passing, 4 failing) plus 4 passing KAT and 4 failing regression witnesses
**Result:** 7 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; closed because the tier's round is done — coverage_gaps had no profraw, so the round was a manual ARM-contract audit of arity / extra / W dest / SP / FP / non-Reg / invalid name / x31 / uppercase / LR / Xd,Xn)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_sxtw | 11 properties + 4 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### encode_sxtw_neg_extra_operand
- **Failing property:** encode_sxtw_neg_extra_operand (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, extra=Reg("x0") — `sxtw x0, w0, x0`
- **Expected:** Err
- **Actual:** Ok(Word) — third operand ignored
- **Law:** SXTW has exactly two register operands (llvm-mc rejects a third)
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxtw_extra_operand.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxtw_neg_extra_operand -- --test-threads=1`

### encode_sxtw_neg_wd
- **Failing property:** encode_sxtw_neg_wd (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, src64=false — `sxtw w0, w0`
- **Expected:** Err
- **Actual:** Ok(Word(0x93407c00)) — 64-bit SXTW encoded (sf hardcoded to 1)
- **Law:** ARM ARM SXTW dest is Xd only; llvm-mc rejects W dest
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxtw_wd.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxtw_neg_wd -- --test-threads=1`

### encode_sxtw_neg_sp
- **Failing property:** encode_sxtw_neg_sp (negative_error)
- **Shrunk counterexample:** which=0, is_64_sp=false, a=0 — `sxtw wsp, w0`
- **Expected:** Err
- **Actual:** Ok(Word) — WSP encoded as XZR
- **Law:** SXTW register 31 is XZR/WZR, never SP/WSP; llvm-mc rejects SP/WSP
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxtw_sp.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxtw_neg_sp -- --test-threads=1`

### encode_sxtw_neg_fp
- **Failing property:** encode_sxtw_neg_fp (negative_error)
- **Shrunk counterexample:** which=0, prefix="d", n=0 — `sxtw d0, w1`
- **Expected:** Err
- **Actual:** Ok(Word) — FP name encoded as GPR x0
- **Law:** SXTW is a GPR instruction; llvm-mc rejects FP/SIMD operands
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_sxtw_fp.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_sxtw_neg_fp -- --test-threads=1`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_sxtw_pbt) | 11 properties + 4 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_sxtw_extra_operand.md
- pbt-out/bug_reports/encode_sxtw_wd.md
- pbt-out/bug_reports/encode_sxtw_sp.md
- pbt-out/bug_reports/encode_sxtw_fp.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 13:22 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 58/253 total | PBT candidates: 58 | Tested: 58 (100%) | 0 pass, 58 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 58 |
| **Tested (of PBT candidates)** | **58 / 58 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 58 / 0 |
| **Overall (tested / all functions)** | **58 / 253 (23%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 58 | 58 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 58 | 58 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 20 | 20 | 100% | covered |
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
| encode_sxtw | data_processing.rs |
