# PBT Coverage Status

> Last updated: 2026-10-05 08:10 (campaign: English campaign)
> Files: 7/14 scanned (50%) | Functions: 24/382 total | PBT candidates: 24 | Tested: 24 (100%) | 15 pass, 5 fail, 4 other

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 7 / 14 (50%) |
| Total functions (all files) | 382 |
| PBT candidates (from FUNCTION_INDEX) | 24 |
| **Tested (of PBT candidates)** | **24 / 24 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 15 / 5 / 4 |
| **Overall (tested / all functions)** | **24 / 382 (6%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 24 | 24 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 24 | 24 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| src/common/asm_constraints.rs | 1 | 1 | 0 | 0% | untested |
| src/common/const_arith.rs | 15 | 15 | 0 | 0% | untested |
| src/common/const_eval.rs | 5 | 5 | 0 | 0% | untested |
| src/common/encoding.rs | 10 | 10 | 0 | 0% | untested |
| src/common/error.rs | 45 | 45 | 0 | 0% | untested |
| src/common/fx_hash.rs | 8 | 8 | 0 | 0% | untested |
| src/common/long_double.rs | 183 | 183 | 0 | 0% | untested |
| src/common/source.rs | 17 | 17 | 0 | 0% | untested |
| src/common/symbol_table.rs | 7 | 7 | 0 | 0% | untested |
| src/common/temp_files.rs | 9 | 9 | 0 | 0% | untested |
| src/common/type_builder.rs | 10 | 10 | 0 | 0% | untested |
| src/common/types.rs | 72 | 72 | 0 | 0% | untested |

## Files Not Yet Scanned (12)

| Source File | Module |
|-------------|--------|
| src/common/asm_constraints.rs | src |
| src/common/const_arith.rs | src |
| src/common/const_eval.rs | src |
| src/common/encoding.rs | src |
| src/common/error.rs | src |
| src/common/fx_hash.rs | src |
| src/common/long_double.rs | src |
| src/common/source.rs | src |
| src/common/symbol_table.rs | src |
| src/common/temp_files.rs | src |
| src/common/type_builder.rs | src |
| src/common/types.rs | src |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| eval_builtin_call | const_eval.rs |
| f64_to_f128_bytes_lossless | long_double.rs |
| f128_bytes_to_f64 | long_double.rs |
| x87_bytes_to_f64 | long_double.rs |
| f64_to_x87_bytes_simple | long_double.rs |

> **Priority 3 — Scan uncovered files**
> 12 file(s) not yet scanned: src (12 files)
> Run `pi-pbt scan <dir>` to add them to FUNCTION_INDEX.md.
