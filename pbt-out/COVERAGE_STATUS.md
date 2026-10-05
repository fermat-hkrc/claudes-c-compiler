# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_mul, English)
> Files: 10/10 scanned | Functions: 289 total | PBT candidates: 129 | This campaign tested: encode_neon_mul
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw; coverage_gaps claimed NOT LINKED on unrelated C++ binaries. Manual arm audit of neon.rs:323-332.

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 129 |
| This campaign target | encode_neon_mul |
| Properties | 10 (6 passing, 4 failing) |
| Coverage evidence | file-level (symbol presence) |

## File Coverage

| Source File | Funcs | This campaign |
|-------------|-------|---------------|
| neon.rs | 68 | encode_neon_mul tested (6 pass / 4 fail) |

## Oracle Type Distribution

| Oracle Type | Count |
|-------------|-------|
| differential | 2 |
| algebraic.metamorphic | 1 |
| algebraic.invariant | 1 |
| negative_error | 6 |
