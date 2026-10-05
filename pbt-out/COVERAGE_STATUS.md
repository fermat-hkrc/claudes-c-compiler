# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_mla, English)
> Files: 10/10 scanned (100%) | Functions: 131/289 total | PBT candidates: 131 | Tested: 131 (100%) | Coverage evidence: file-level (symbol presence); no .gcda/.profraw
>
> encode_neon_mla: 6 passing / 4 failing properties (4 bugs). Sweep: reserved-T failing, nonreg passing.

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 131 |
| **Tested (of PBT candidates)** | **131 / 131 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 131 / 0 |
| **Overall (tested / all functions)** | **131 / 289 (45%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 131 | 131 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 131 | 131 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 11 | 11 | 100% | covered |
| neon.rs | 68 | 45 | 45 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding more tests.

| Function | Source |
|----------|--------|
| encode_neon_mla | neon.rs |
