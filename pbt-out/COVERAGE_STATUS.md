# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fence)
> Coverage evidence: file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw
> This campaign: encode_fence tested (3 passing / 5 failing properties)

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_fence |
| Properties this campaign | 8 |
| Passing / failing | 3 / 5 |
| Bugs | 5 |
| Coverage evidence | file-level (symbol presence) |
| Sweep | 1/1 manual arm audit (empty, FenceArg pair, non-FenceArg wildcard, len==1) |

## File Coverage (this campaign)

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| system.rs (encode_fence; requested via base.rs scope) | 1 | 1 | 1 | 100% | covered |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Covered |
|-------------|-------|---------|
| differential | 2 | 2 |
| algebraic.metamorphic | 1 | 1 |
| algebraic.invariant | 1 | 1 |
| negative_error | 4 | 4 |
