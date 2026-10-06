# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_c_li, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and encode_c_li NOT LINKED. Rust `cargo test --lib encode_c_li` executed the production symbol (11 passed, 4 failed).

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_c_li (compressed.rs:18) |
| Properties | 7 (5 passing, 2 failing) |
| KAT | 6 passing |
| Regression witnesses | 2 failing (extra operand, imm oob) |
| Sweep | 1 round (manual audit; coverage_gaps had no Rust profraw) |

## File Coverage (this campaign)

| Source File | Funcs in scope | Candidates | Tested | Coverage | Status |
|-------------|----------------|------------|--------|----------|--------|
| compressed.rs | 1 (encode_c_li) | 1 | 1 | 100% | covered |
