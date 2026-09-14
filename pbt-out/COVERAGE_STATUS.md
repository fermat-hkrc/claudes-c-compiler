# PBT Coverage Status

> Last updated: 2026-09-11 12:33 (campaign: English campaign)
> Files: 5/5 scanned (100%) | Functions: 8/164 total | PBT candidates: 8 | Tested: 8 (100%) | 0 pass, 8 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 5 |
| Files scanned | 5 / 5 (100%) |
| Total functions (all files) | 164 |
| PBT candidates (from FUNCTION_INDEX) | 8 |
| **Tested (of PBT candidates)** | **8 / 8 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 8 / 0 |
| **Overall (tested / all functions)** | **8 / 164 (5%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 8 | 8 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 8 | 8 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
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
