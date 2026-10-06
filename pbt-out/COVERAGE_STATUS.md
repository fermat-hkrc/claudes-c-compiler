# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_sc)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED for encode_sc). The Rust `cargo test --lib encode_sc` run executed the production symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_sc |
| Source | atomics.rs:13 |
| Properties | 7 (5 passing, 2 failing) |
| KAT | 3 passing |
| Regression witnesses | 2 failing (intentional) |
| Bugs | 2 |
| Sweep | 1/1 spent (manual audit of valid 3-operand / extra / nonzero offset / arity / FP / non-Mem) |

## File Coverage (this campaign)

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| atomics.rs | 7 | 1 (encode_sc; others HARD-scope skipped) | 1 | encode_sc covered | covered |
