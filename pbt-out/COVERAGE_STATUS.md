# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_dc, English)
> Files: 11/11 scanned | Functions: 307 total | PBT candidates: 160 | This campaign tested: encode_dc
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 160 |
| This campaign target | encode_dc (system.rs:564) |
| This campaign properties | 10 (6 passing, 4 failing) |
| Coverage evidence | file-level (symbol presence); no LLVM profraw |

## This campaign

| Function | Source file | Test file | Result |
|----------|-------------|-----------|--------|
| encode_dc | system.rs | encode_dc_pbt.rs | 6 passing / 4 failing (7 KAT pass; 5 regression fail; 4 bugs) |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Passing | Failing |
|-------------|-------|---------|---------|
| differential | 1 | 1 | 0 |
| algebraic.invariant | 1 | 1 | 0 |
| algebraic.metamorphic | 2 | 2 | 0 |
| negative_error | 6 | 2 | 4 |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit added encode_dc_neg_invalid_reg and encode_dc_neg_unknown_nonsubstr. Closed: every documented behavior has a property; tier round spent.
