# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_load, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter claimed NOT LINKED). SUT is Rust `cargo test --lib encode_load`; encode_load ran in that target (13 passed / 8 failed including 4 property failures).

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_load (base.rs:147) |
| Properties | 10 |
| Passing | 6 |
| Failing | 4 |
| Bugs | 4 |
| Sweep | 1/1 (manual arm audit + hi-modifier / SymbolOffset properties) |

## File Coverage (this campaign)

| Source File | Funcs in scope | Candidates | Tested | Coverage | Status |
|-------------|---------------|------------|--------|----------|--------|
| base.rs | 1 (encode_load only) | 1 | 1 | 100% | covered |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Covered |
|-------------|-------|---------|
| differential | 3 | 3 |
| algebraic.invariant | 1 | 1 |
| algebraic.metamorphic | 2 | 2 |
| negative_error | 4 | 4 |
