# PBT Campaign Report: encode_movz

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_movz
**Tests:** 19 (13 properties + 1 KAT + 5 regression witnesses)
**Result:** 8 passing properties, 5 bugs (plus 1 passing KAT; 5 failing regression witnesses)
**Effort tier:** standard (1 coverage-driven contract-surface sweep; close reason: sweep round done)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_movz | 13 properties + 1 KAT + 5 regressions | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_movz silently truncates immediates outside [0, 65535]**
   - Law: imm16 must be in [0, 65535]
   - Shrunk input: `movz w0, #-1` (`[Reg("w0"), Imm(-1)]`)
   - Expected: Err (llvm-mc range error)
   - Actual: Ok(Word) with imm16=0xFFFF via `(imm as u32) & 0xFFFF`
   - Root cause: no range check on get_imm result
   - Impact: assembler accepts invalid GNU as and emits a different immediate
   - Severity: medium
   - Serial reconfirmation: PBT_TEST_JOBS=1, same witness
   - Bug report: pbt-out/bug_reports/encode_movz_imm_oob.md

2. **encode_movz accepts non-lsl shifts and out-of-set lsl amounts**
   - Law: optional shift must be lsl with 0/16 (W) or 0/16/32/48 (X)
   - Shrunk input: `movz w0, #0, lsr #0`
   - Expected: Err
   - Actual: Ok(Word) with hw=0
   - Root cause: non-lsl kinds default to hw=0; lsl amount is `amount / 16` with no set check
   - Impact: invalid GNU as encodes as a different MOVZ
   - Severity: medium
   - Serial reconfirmation: PBT_TEST_JOBS=1, same witness
   - Bug report: pbt-out/bug_reports/encode_movz_invalid_shift.md

3. **encode_movz ignores extra operands after optional lsl**
   - Law: exactly Rd + imm16 + optional lsl
   - Shrunk input: `movz x0, #0, x0`
   - Expected: Err
   - Actual: Ok(Word) encoding `movz x0, #0`
   - Root cause: operands beyond index 2 are never inspected; a non-Shift at index 2 sets hw=0
   - Impact: trailing garbage is dropped
   - Severity: medium
   - Serial reconfirmation: PBT_TEST_JOBS=1, same witness
   - Bug report: pbt-out/bug_reports/encode_movz_extra_operand.md

4. **encode_movz encodes SP/WSP as XZR/WZR**
   - Law: Rd=31 is XZR/WZR, never SP/WSP
   - Shrunk input: `movz wsp, #0`
   - Expected: Err
   - Actual: Ok(Word) 32-bit MOVZ Rd=31
   - Root cause: parse_reg_num maps sp/wsp to 31
   - Impact: stack-pointer destination silently becomes the zero register
   - Severity: medium
   - Serial reconfirmation: PBT_TEST_JOBS=1, same witness
   - Bug report: pbt-out/bug_reports/encode_movz_sp.md

5. **encode_movz encodes FP/SIMD register names as GPRs**
   - Law: MOVZ Rd is a GPR; FP/SIMD names must be rejected
   - Shrunk input: `movz d0, #0`
   - Expected: Err
   - Actual: Ok(Word) encoding `movz w0, #0`
   - Root cause: parse_reg_num accepts d/s/q/v/h/b prefixes; is_64bit_reg is false for `d`
   - Impact: SIMD names silently retarget a GPR
   - Severity: medium
   - Serial reconfirmation: PBT_TEST_JOBS=1, same witness
   - Bug report: pbt-out/bug_reports/encode_movz_fp.md

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_movz_pbt) | 13 properties + 1 KAT + 5 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_movz_imm_oob.md
- pbt-out/bug_reports/encode_movz_invalid_shift.md
- pbt-out/bug_reports/encode_movz_extra_operand.md
- pbt-out/bug_reports/encode_movz_sp.md
- pbt-out/bug_reports/encode_movz_fp.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 08:41 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 40/184 total | PBT candidates: 40 | Tested: 40 (100%) | 0 pass, 40 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 40 |
| **Tested (of PBT candidates)** | **40 / 40 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 40 / 0 |
| **Overall (tested / all functions)** | **40 / 184 (22%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 40 | 40 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 40 | 40 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 11 | 11 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
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
| encode_ldxr_stxr | load_store.rs |
| encode_logical | data_processing.rs |
| encode_madd | data_processing.rs |
| encode_movk | data_processing.rs |
| encode_movn | data_processing.rs |
| encode_movz | data_processing.rs |
