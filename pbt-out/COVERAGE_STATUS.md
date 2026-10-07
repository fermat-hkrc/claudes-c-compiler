# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_negw)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_negw NOT LINKED in C++ reporter binaries. Rust cargo test --lib encode_negw_pbt executed the symbol.

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_negw |
| Properties | 8 |
| Passing | 7 |
| Failing | 1 |
| Bugs | 1 |
| Sweep | 1 round (standard); remaining documented gap is extra-operand rejection |

## This campaign

| Function | Source file | Tested | Result |
|----------|-------------|--------|--------|
| encode_negw | pseudo.rs | yes | 7 passing / 1 failing (extra operand) |
