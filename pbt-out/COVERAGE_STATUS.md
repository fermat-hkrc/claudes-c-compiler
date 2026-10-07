# PBT Coverage Status

> Last updated: 2026-10-07 02:39 (campaign: encode_li)
> Files: 16/16 scanned (100%) | Functions: 383 total | PBT candidates: 237 | This campaign tested: encode_li
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_li NOT LINKED in C++ reporter binaries (Rust cargo tests are not those binaries).

## This campaign (encode_li)

| Metric | Value |
|--------|-------|
| Target | encode_li (pseudo.rs:5) |
| Properties | 8 |
| Passing | 7 |
| Failing | 1 (extra operand) |
| Bugs | 1 |
| Test target | cargo test --lib encode_li |
| Sweep | 1 round (manual; coverage_gaps file-level NOT LINKED) |

## File Coverage (this campaign)

| Source File | Funcs | Candidates this campaign | Tested | Coverage | Status |
|-------------|-------|--------------------------|--------|----------|--------|
| pseudo.rs | 49 | 1 (encode_li) | 1 | encode_li executed via cargo test --lib encode_li | covered |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Covered |
|-------------|-------|---------|
| differential | 2 | 2 |
| algebraic.invariant | 1 | 1 |
| algebraic.metamorphic | 2 | 2 |
| negative_error | 3 | 3 (2 pass, 1 fail) |
