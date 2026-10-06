# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_vsetivli)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_vsetivli NOT LINKED in C++ reporter binaries. Rust cargo tests executed the symbol (6 passing / 5 failing properties plus KAT and regression witnesses).

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_vsetivli (vector.rs:59) |
| Properties | 11 |
| Passing | 6 |
| Failing | 5 |
| Bugs filed | 5 |
| Sweep | 1 round (standard): dedicated FP/vector-rd property passing |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| vector.rs | 16 | 1 (encode_vsetivli) | 1 | 100% of in-scope candidate | covered (5 failing properties) |
