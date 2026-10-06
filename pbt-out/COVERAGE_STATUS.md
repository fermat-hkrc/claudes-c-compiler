# PBT Coverage Status

> Last updated: 2026-10-06 04:08 (campaign: PBT campaign)
> Files: 8/17 scanned (47%) | Functions: 8/441 total | PBT candidates: 8 | Tested: 8 (100%) | 1 pass, 1 fail, 6 other

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 8 / 17 (47%) |
| Total functions (all files) | 441 |
| PBT candidates (from FUNCTION_INDEX) | 8 |
| **Tested (of PBT candidates)** | **8 / 8 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 1 / 6 |
| **Overall (tested / all functions)** | **8 / 441 (2%)** |
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
| src/passes/cfg_simplify.rs | 2 | 2 | 0 | 0% | untested |
| src/passes/constant_fold.rs | 3 | 3 | 0 | 0% | untested |
| src/passes/copy_prop.rs | 2 | 2 | 0 | 0% | untested |
| src/passes/dce.rs | 1 | 1 | 0 | 0% | untested |
| src/passes/dead_statics.rs | 1 | 1 | 0 | 0% | untested |
| src/passes/div_by_const.rs | 5 | 5 | 0 | 0% | untested |
| src/passes/gvn.rs | 2 | 2 | 0 | 0% | untested |
| src/passes/if_convert.rs | 1 | 1 | 0 | 0% | untested |
| src/passes/inline.rs | 1 | 1 | 0 | 0% | untested |
| src/passes/ipcp.rs | 1 | 1 | 0 | 0% | untested |
| src/passes/iv_strength_reduce.rs | 2 | 2 | 0 | 0% | untested |
| src/passes/licm.rs | 1 | 1 | 0 | 0% | untested |
| src/passes/loop_analysis.rs | 3 | 3 | 0 | 0% | untested |
| src/passes/mod.rs | 3 | 2 | 0 | 0% | untested |
| src/passes/narrow.rs | 1 | 1 | 0 | 0% | untested |
| src/passes/simplify.rs | 2 | 2 | 0 | 0% | untested |

## Files Not Yet Scanned (16)

| Source File | Module |
|-------------|--------|
| src/passes/cfg_simplify.rs | src |
| src/passes/constant_fold.rs | src |
| src/passes/copy_prop.rs | src |
| src/passes/dce.rs | src |
| src/passes/dead_statics.rs | src |
| src/passes/div_by_const.rs | src |
| src/passes/gvn.rs | src |
| src/passes/if_convert.rs | src |
| src/passes/inline.rs | src |
| src/passes/ipcp.rs | src |
| src/passes/iv_strength_reduce.rs | src |
| src/passes/licm.rs | src |
| src/passes/loop_analysis.rs | src |
| src/passes/mod.rs | src |
| src/passes/narrow.rs | src |
| src/passes/simplify.rs | src |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| x86 builtin assembler (parser + encoders) | assembler/parser.rs |

> **Priority 3 — Scan uncovered files**
> 16 file(s) not yet scanned: src (16 files)
> Run `pi-pbt scan <dir>` to add them to FUNCTION_INDEX.md.
