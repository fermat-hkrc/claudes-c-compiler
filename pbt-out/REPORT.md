# PBT Campaign Report: encode_ldxr_stxr

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldxr_stxr
**Tests:** 7 properties + 8 KAT + 9 regression witnesses
**Result:** 3 passing properties, 9 bugs
**Effort tier:** standard (5–8 properties/target; 7 written; ≥1000 cases; 1 coverage-driven sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldxr_stxr | 7 properties (3 pass / 4 fail) + 8 KAT pass + 9 failing regressions | 9 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **Extra operand ignored.** Law: LDXR takes exactly two operands (`Rt, [Xn|SP]`); STXR takes exactly three (`Ws, Rt, [Xn|SP]`). Shrunk input: `stxr w0, w0, [x0], x2`. Expected Err (llvm-mc rejects a trailing operand); actual Ok(Word(0x88007c00)). Root cause: no arity upper bound; operands past the memory slot are unread. Impact: invalid GNU-style text is assembled. Severity: medium. Fix: reject `operands.len() != 2` (load) / `!= 3` (store). Report: `pbt-out/bug_reports/encode_ldxr_stxr_extra_operand.md`

2. **SP as Rt encoded as ZR.** Law: Rt is Wt/Xt with 31 = WZR/XZR, never SP/WSP. Shrunk input: `ldxr sp, [x0]`. Expected Err; actual Ok(Word) of `ldxr xzr, [x0]`. Root cause: `parse_reg_num` / `is_64bit_reg` treat `sp` as X31. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_sp_as_rt.md`

3. **W-register base accepted.** Law: Rn is Xn|SP. Input: `ldxr x0, [w1]`. Expected Err; actual Ok(Word) of `ldxr x0, [x1]`. Root cause: `parse_reg_num` accepts the `w` prefix. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_w_base.md`

4. **XZR as base encoded as SP.** Law: address-register 31 is SP, not XZR. Input: `ldxr x0, [xzr]`. Expected Err; actual Ok(Word) of `ldxr x0, [sp]`. Root cause: `parse_reg_num("xzr")` returns 31. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_xzr_as_base.md`

5. **SIMD/FP Rt encoded as GPR.** Law: Rt is a general-purpose Wt/Xt. Input: `ldxr d0, [x1]`. Expected Err; actual Ok(Word) with Rt=0. Root cause: `parse_reg_num` accepts d/s/q/v/h/b prefixes. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_fp_as_rt.md`

6. **X register accepted as STXR status.** Law: STXR status is Ws (31=WZR). Input: `stxr x0, x1, [x2]`. Expected Err; actual Ok(Word) using the X number in Rs. Root cause: store path takes `get_reg` number and ignores width. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_x_as_ws.md`

7. **X data register on byte/half exclusive.** Law: LDXRB/LDXRH data is Wt. Input: `ldxrb x0, [x1]`. Expected Err; actual Ok(Word) with size=00 and Xt number. Root cause: `forced_size` overrides size but not register class. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_x_data_byte.md`

8. **Nonzero offset ignored.** Law: exclusive memory form is `[Xn|SP]` or `[Xn|SP, #0]`. Shrunk input: `stxr w1, x0, [x2, #-1]` (shape=8, offset=-1). Expected Err (`index must be absent or #0`); actual Ok(Word) of the offset-0 encoding. Root cause: `Operand::Mem { base, .. }` discards offset. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_nonzero_offset.md`

9. **STXR Ws overlapping a source.** Law: ARM CONSTRAINED UNPREDICTABLE / llvm-mc "status is also a source" when Ws aliases Rt or Xn (WZR vs SP allowed). Shrunk input: `stxr w0, w0, [x0]`. Expected Err; actual Ok(Word(0x88007c00)). Root cause: no alias check. Severity: medium. Report: `pbt-out/bug_reports/encode_ldxr_stxr_ws_overlap.md`

All nine reproduced serially (`PBT_TEST_JOBS=1`). Regression witnesses assert the gas-compatible Err contract and fail while the bugs exist.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_ldxr_stxr_pbt) | 7 properties + 8 KAT + 9 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_ldxr_stxr_extra_operand.md
- pbt-out/bug_reports/encode_ldxr_stxr_sp_as_rt.md
- pbt-out/bug_reports/encode_ldxr_stxr_w_base.md
- pbt-out/bug_reports/encode_ldxr_stxr_xzr_as_base.md
- pbt-out/bug_reports/encode_ldxr_stxr_fp_as_rt.md
- pbt-out/bug_reports/encode_ldxr_stxr_x_as_ws.md
- pbt-out/bug_reports/encode_ldxr_stxr_x_data_byte.md
- pbt-out/bug_reports/encode_ldxr_stxr_nonzero_offset.md
- pbt-out/bug_reports/encode_ldxr_stxr_ws_overlap.md

Contract-surface sweep: 1 round (standard). `coverage_gaps` had no LLVM profraw; manual arm audit of `encode_ldxr_stxr` (is_load true/false, get_reg miss, Mem vs non-Mem, parse_reg_num None, forced_size Some/None, is_64 true/false) showed every documented branch of this symbol was reached. Closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 07:23 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 35/184 total | PBT candidates: 35 | Tested: 35 (100%) | 0 pass, 35 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 35 |
| **Tested (of PBT candidates)** | **35 / 35 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 35 / 0 |
| **Overall (tested / all functions)** | **35 / 184 (19%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 35 | 35 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 35 | 35 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
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
