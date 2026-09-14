# PBT Campaign Report: encode_cset

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_cset (src/backend/arm/assembler/encoder/compare_branch.rs)
**Tests:** 11 properties (8 passing, 3 failing) plus 3 passing KATs and 5 failing regression witnesses
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (5–8 properties, ≥1000 cases, 1 contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cset | 11 properties + 3 KAT + 5 regressions | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_cset ignores extra operands.** Law: CSET takes exactly two operands; a third must be Err. Shrunk input: `[Reg("x0"), Cond("eq"), Reg("x2")]`. Expected Err; actual Ok(Word(0x9a9f17e0)) because only operands 0 and 1 are inspected. Severity: medium. Report: `pbt-out/bug_reports/encode_cset_extra_operand.md`.

2. **encode_cset accepts AL and NV.** Law: CSET is not a valid CSINC alias when cond is AL or NV. Shrunk input: `[Reg("x0"), Cond("al")]`. Expected Err; actual Ok(Word(0x9a9ff7e0)) (`cset x0, nv` encodes as 0x9a9fe7e0). llvm-mc: "condition codes AL and NV are invalid for this instruction". Severity: medium. Report: `pbt-out/bug_reports/encode_cset_al_nv.md`.

3. **encode_cset encodes SP as XZR.** Law: register 31 is XZR/WZR, never SP/WSP. Shrunk input: `[Reg("sp"), Cond("eq")]`. Expected Err; actual Ok(Word(0x9a9f17ff)) (`cset xzr, eq`). parse_reg_num maps sp/wsp to 31. Severity: medium. Report: `pbt-out/bug_reports/encode_cset_sp_as_zr.md`.

4. **encode_cset accepts FP/SIMD names as GPRs.** Law: CSET takes Wt/Xt only. Witness: `[Reg("d0"), Cond("eq")]`. Expected Err; actual Ok(Word(0x1a9f17e0)) (W-form CSET of w0). parse_reg_num accepts d/s/q/v/h/b prefixes. Severity: medium. Report: `pbt-out/bug_reports/encode_cset_fp_reg.md`.

Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_cset_pbt -- --test-threads=1` reproduced all three property failures and all five regression witnesses.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_cset_pbt) | 11 properties, 3 KATs, 5 regressions |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — per-function coverage ledger (appended encode_cset)
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_cset marked yes)
- `pbt-out/INVARIANTS.md` — confirmed encode_cset invariants prepended
- `pbt-out/bug_reports/encode_cset_extra_operand.md`
- `pbt-out/bug_reports/encode_cset_al_nv.md`
- `pbt-out/bug_reports/encode_cset_sp_as_zr.md`
- `pbt-out/bug_reports/encode_cset_fp_reg.md`

## Contract-surface sweep

Round 1 of 1 (standard). `coverage_gaps` reported no instrumented profraw. Manual arm audit of `encode_cset` added three properties that reach `encode_cond` None, `parse_reg_num` None, and get_reg/cond non-Reg/non-Cond; all three passed. Extra/AL-NV/SP/FP remain filed as bugs. Closed because the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 03:48 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 21/184 total | PBT candidates: 21 | Tested: 21 (100%) | 0 pass, 21 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 21 |
| **Tested (of PBT candidates)** | **21 / 21 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 21 / 0 |
| **Overall (tested / all functions)** | **21 / 184 (11%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 21 | 21 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 21 | 21 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 13 | 13 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 4 | 4 | 100% | covered |
| load_store.rs | 20 | 1 | 1 | 100% | covered |
| neon.rs | 68 | 1 | 1 | 100% | covered |

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
