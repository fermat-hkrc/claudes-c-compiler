# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_sxtb, English)
> Files: 11/11 scanned (100%) | Functions: 173/307 total PBT candidates | Tested this campaign: encode_sxtb

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 173 |
| **Tested this campaign** | encode_sxtb (12 properties) |
| Coverage evidence | file-level (symbol presence) — no .gcda/.profraw |

## This campaign

| Function | Source file | Result |
|----------|-------------|--------|
| encode_sxtb | data_processing.rs | 8 passing / 4 failing (plus 4 KAT + 4 regression witnesses; 4 bugs) |

## Sweep

coverage_gaps: no LLVM profraw (C++ reporter listed unrelated binaries and claimed encode_sxtb NOT LINKED). Manual arm audit of the 8-line body. Every documented behavior has a property. Tier round 1/1 spent.

## Oracle Type Distribution (this campaign)

| Oracle Type | Count |
|-------------|-------|
| differential | 2 |
| algebraic.metamorphic | 2 |
| algebraic.invariant | 1 |
| negative_error | 7 |
