# PBT Campaign Report: encode_ldar_stlr

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldar_stlr
**Tests:** 8 properties (plus 6 KAT + 3 regression witnesses)
**Result:** 5 passing, 3 failing properties (3 SUT bugs)
**Effort tier:** standard
**Contract-surface sweep:** 1 round (coverage_gaps had no profraw; manual arm audit of get_reg non-Reg first operand and parse_reg_num None on invalid base names). Closed because the tier's 1 round is done. Sweep extended `encode_ldar_stlr_neg_arity_and_shape` (shapes 7–8) and that property still passes.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldar_stlr | 8 properties (5 pass, 3 fail) | 3 | differential, algebraic.round_trip, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **Extra operand ignored** — property `encode_ldar_stlr_neg_extra_operands`. Shrunk counterexample: `stlr w0, [x0], x2` (rt=0, rn=0, is_load=false, variant=0, is_64=false, extra=Reg("x2")). Expected Err; actual Ok(Word) encoding of `stlr w0, [x0]`. Path: `pbt-out/bug_reports/encode_ldar_stlr_extra_operand.md`. Severity: medium.

2. **SP/WSP encoded as ZR for Rt** — property `encode_ldar_stlr_neg_invalid_rt`. Shrunk counterexample: `stlr sp, [x0]` (is_load=false, kind=0, n=0). Expected Err; actual Ok(Word) with Rt=31 (XZR). llvm-mc rejects SP as Rt. Path: `pbt-out/bug_reports/encode_ldar_stlr_sp_as_rt.md`. Severity: medium.

3. **W register accepted as memory base** — property `encode_ldar_stlr_neg_invalid_base_offset`. Shrunk counterexample: `stlr w0, [w0]` (rt=0, is_load=false, variant=0, is_64=false, kind=0, wn=0). Expected Err; actual Ok(Word) encoding of `stlr w0, [x0]`. llvm-mc rejects a W base. Path: `pbt-out/bug_reports/encode_ldar_stlr_w_base.md`. Severity: medium.

All three reproduced serially with `PBT_TEST_JOBS=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_ldar_stlr_pbt) | 8 properties + 6 KAT + 3 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — per-function coverage row
- pbt-out/COVERAGE_STATUS.md — campaign coverage stats
- pbt-out/FUNCTION_INDEX.md — merged function index (encode_ldar_stlr marked yes)
- pbt-out/INVARIANTS.md — confirmed encode_ldar_stlr invariants
- pbt-out/bug_reports/encode_ldar_stlr_extra_operand.md
- pbt-out/bug_reports/encode_ldar_stlr_sp_as_rt.md
- pbt-out/bug_reports/encode_ldar_stlr_w_base.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 05:14 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 28/184 total | PBT candidates: 28 | Tested: 28 (100%) | 0 pass, 28 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 28 |
| **Tested (of PBT candidates)** | **28 / 28 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 28 / 0 |
| **Overall (tested / all functions)** | **28 / 184 (15%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 28 | 28 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 28 | 28 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
| load_store.rs | 20 | 2 | 2 | 100% | covered |
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
| encode_csetm | compare_branch.rs |
| encode_csinc | compare_branch.rs |
| encode_csinv | compare_branch.rs |
| encode_csneg | compare_branch.rs |
| encode_div | data_processing.rs |
| encode_eon | data_processing.rs |
| encode_ldar_stlr | load_store.rs |
