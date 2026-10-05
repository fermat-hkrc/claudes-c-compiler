# PBT Coverage Status

> Last updated: 2026-10-05 18:45 (campaign: encode_neon_logical)
> Files: 10/10 scanned (100%) | Functions: 135/289 total | PBT candidates: 135 | Tested: 135 | this campaign: encode_neon_logical 7 pass / 5 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 135 |
| **Tested this campaign** | encode_neon_logical (12 properties) |
| Coverage evidence | file-level (symbol presence) — no LLVM profraw; cargo tests execute encode_neon_logical |

## Module Breakdown

| Module | Tests | Bugs | Notes |
|--------|-------|------|-------|
| encode_neon_logical | 12 | 5 | differential + metamorphic + invariant + negative_error |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_neon_logical | neon.rs | yes | 7 passing / 5 failing |

## Sweep

Round 1/1 spent. coverage_gaps: no .gcda/.profraw (C++ reporter claimed NOT LINKED). Added arity, unsupported opc, uppercase V (passing) and ANDS (failing B5).
