# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_scalar_three_same)
> Files: 10/10 scanned | Functions: 144/289 PBT candidates tested this lineage | This campaign: encode_neon_scalar_three_same
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and listed unrelated C++ binaries as NOT LINKED. cargo test --lib executed the real symbol.

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 144 |
| This campaign target | encode_neon_scalar_three_same |
| This campaign properties | 8 (6 passing, 2 failing) |
| Coverage evidence | file-level (symbol presence) |

## File Coverage

| Source File | Notes |
|-------------|-------|
| neon.rs | encode_neon_scalar_three_same exercised by cargo test --lib encode_neon_scalar_three_same |

## This campaign

| Function | Result |
|----------|--------|
| encode_neon_scalar_three_same | 6 passing / 2 failing (plus KAT pass, 2 failing regressions, sweep alt-spellings passing) |
