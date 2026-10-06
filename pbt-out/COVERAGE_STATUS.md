# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_vsetvl, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_vsetvl NOT LINKED in C++ reporter binaries. Rust `cargo test --lib encode_vsetvl_` executed the production symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_vsetvl (vector.rs:70) |
| Properties | 7 (6 passing, 1 failing) |
| KAT | 6 passing |
| Regression witnesses | 1 failing (extra operand) |
| Bugs | 1 medium (extra operand ignored) |
| Sweep | 1 round (standard); manual audit of 3-op / format / ABI / isolation / arity-FP / extra / nonreg |

## File Coverage (this campaign)

| Source File | Funcs in file | This-campaign candidate | Tested | Status |
|-------------|---------------|-------------------------|--------|--------|
| vector.rs | 16 | encode_vsetvl | encode_vsetvl | covered (1 fail: extra) |
