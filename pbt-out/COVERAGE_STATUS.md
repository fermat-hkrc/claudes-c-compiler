# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_c_add, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo tests are not the C++ reporter binaries) and listed encode_c_add as NOT LINKED in those binaries.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_c_add (compressed.rs:43) |
| Properties | 7 (5 passing, 2 failing) |
| KAT | 6 passing |
| Regression witnesses | 2 failing (expected) |
| Bugs | 2 (rs2=x0 high, extra operand medium) |
| Sweep | 1 round (standard); documented surface 2-op / CR-type / ABI / isolation / arity-FP / extra / rs2=x0 all have properties; remaining gaps are the two filed bugs |

## File coverage (scoped SUT)

| Source File | Funcs | This-campaign target | Tested | Status |
|-------------|-------|----------------------|--------|--------|
| compressed.rs | 16 | encode_c_add | encode_c_add | covered (cargo test --lib encode_c_add_pbt) |
