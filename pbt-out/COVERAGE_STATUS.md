# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_c_mv, English, tier standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and encode_c_mv NOT LINKED. Rust `cargo test --lib encode_c_mv` executed the production symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_c_mv |
| Source | src/backend/riscv/assembler/encoder/compressed.rs:36 |
| Test file | src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs |
| Properties | 7 (5 passing, 2 failing) |
| KAT | 6 passing |
| Regression witnesses | 2 failing (as expected) |
| Bugs | 2 |
| Sweep | 1 round (manual; remaining gaps are the two filed bugs) |

## File Coverage (campaign scope)

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| compressed.rs | 16 | 1 (encode_c_mv this campaign) | 1 | 100% of this campaign's candidate | covered |
