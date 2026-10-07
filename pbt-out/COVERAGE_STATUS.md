# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_v_arith_vv, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_v_arith_vv NOT LINKED in C++ reporter binaries. Rust cargo tests executed the production symbol via `cargo test --lib encode_v_arith_vv`.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_v_arith_vv |
| Source | vector.rs:123 |
| Properties | 7 (5 passing, 2 failing) |
| Bugs | 2 |
| Sweep | 1 round (manual; extra + mask-v0.t remaining gaps) |

## File Coverage (this campaign)

| Source File | Funcs in scope | Candidates | Tested | Coverage | Status |
|-------------|----------------|------------|--------|----------|--------|
| vector.rs | 16 | 1 (HARD: encode_v_arith_vv only) | 1 | 100% of in-scope | covered |

Historical per-function rows live in COVERAGE.md.
