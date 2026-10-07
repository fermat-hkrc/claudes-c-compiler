# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_beqz)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw for Rust cargo tests; encode_beqz NOT LINKED in C++ pbt binaries. Execution evidence is cargo test of encode_beqz_pbt (13 items).

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_beqz (pseudo.rs:282) |
| Properties | 10 (9 passing, 1 failing) |
| Bugs | 1 (extra operand ignored) |
| Effort tier | standard |
| Contract-surface sweep | 1 round spent (manual audit; coverage_gaps file-level only) |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_beqz | pseudo.rs | yes | 9 pass / 1 fail (extra operand) |

## Coverage evidence level

file-level (symbol presence) — native line coverage unavailable for this Rust lib-test target under the campaign's C++ coverage reporter.
