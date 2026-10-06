# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_vload, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_vload NOT LINKED in C++ reporter binaries. Rust `cargo test --lib encode_vload` executed the production symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_vload (vector.rs:81) |
| Properties | 8 (7 passing, 1 failing) |
| KAT | 6 passing |
| Regression | 1 failing (extra operand) |
| Bugs | 1 medium |
| Sweep | 1 round (standard); remaining documented gap is extra-operand |

## File coverage (this campaign)

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| vector.rs | 16 | 1 (encode_vload) | 1 | 100% of in-scope | covered |
