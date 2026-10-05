# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_ins, English)
> Files: 10/10 scanned | Functions: 107/289 PBT candidates | Tested this campaign: encode_neon_ins
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw; cargo libtest executed encode_neon_ins

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 107 |
| Tested this campaign | encode_neon_ins (10 properties: 6 passing / 4 failing) |
| Coverage evidence | file-level (symbol presence) |

## This campaign

| Function | Source file | Tested | Result |
|----------|-------------|--------|--------|
| encode_neon_ins | neon.rs | yes | 6 passing / 4 failing (4 bugs) |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit added arity < 2 and uppercase V/W/X alt-spellings (both passing). Closed: tier round spent.
