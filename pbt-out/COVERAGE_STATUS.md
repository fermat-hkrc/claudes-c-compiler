# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_amo, English, standard)
> Files: 14 scanned in FUNCTION_INDEX | Functions: 337 total | PBT candidates: 196 | This campaign tested: encode_amo

## Summary

| Metric | Value |
|--------|-------|
| Total source files (FUNCTION_INDEX) | 14 |
| Total functions (all files) | 337 |
| PBT candidates (from FUNCTION_INDEX) | 196 |
| **This campaign target** | encode_amo (atomics.rs:21) |
| **This campaign properties** | 7 (5 passing, 2 failing) |
| Coverage evidence | file-level (symbol presence) |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_amo | atomics.rs | yes | 5 pass / 2 fail (2 bugs) |

coverage_gaps: no .gcda/.profraw (C++ reporter; unrelated binaries; claimed NOT LINKED). Rust `cargo test --lib encode_amo` executed the production symbol. Sweep round 1/1 spent on a manual audit of all six body statements.

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Passing | Failing |
|-------------|-------|---------|---------|
| differential | 1 | 1 | 0 |
| algebraic.invariant | 1 | 1 | 0 |
| algebraic.metamorphic | 1 | 1 | 0 |
| negative_error | 4 | 2 | 2 |
