# PBT Coverage Status

> Last updated: 2026-10-05 10:51 (campaign: English campaign)
> Files: 8/53 scanned (15%) | Functions: 22/834 total | PBT candidates: 22 | Tested: 22 (100%) | 12 pass, 7 fail, 3 other

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 53 |
| Files scanned | 8 / 53 (15%) |
| Total functions (all files) | 834 |
| PBT candidates (from FUNCTION_INDEX) | 22 |
| **Tested (of PBT candidates)** | **22 / 22 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 12 / 7 / 3 |
| **Overall (tested / all functions)** | **22 / 834 (3%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 22 | 22 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 22 | 22 | 0 | 100% |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| IrConst::cast_float_to_target | constants.rs |
| IrConst::cast_long_double_to_target | constants.rs |
| IrConst::coerce_to_with_src / coerce_to | constants.rs |
| f64_to_f128_bytes | constants.rs |
| f64_to_x87_bytes | constants.rs |
| build_cfg | analysis.rs |
| compute_dominance_frontiers | analysis.rs |
