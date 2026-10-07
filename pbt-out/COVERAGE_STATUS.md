# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_bgtu)
> Coverage evidence: file-level (symbol presence) — `coverage_gaps` found no .gcda/.profraw (Rust cargo test is not C++-instrumented); reporter listed unrelated OH C++ binaries. Execution evidence is the cargo test run log (pbt-out/run/encode_bgtu_test.log): 14 tests, encode_bgtu exercised.

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_bgtu (pseudo.rs:356) |
| Properties | 12 (11 passing, 1 failing) |
| KAT + regression | 1 KAT pass, 1 regression fail (witness) |
| Bugs | 1 (extra operand silently accepted) |
| Sweep | 1 round (standard tier) — all documented behaviors covered |

## This campaign

| Function | Source | Tested | Notes |
|----------|--------|--------|-------|
| encode_bgtu | pseudo.rs | yes | differential llvm-mc + metamorphic + invariant + negative; extra-operand bug |

## Untested in scope

(none — single-symbol campaign; encode_bgtu fully property-covered)
