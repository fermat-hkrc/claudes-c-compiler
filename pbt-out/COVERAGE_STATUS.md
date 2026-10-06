# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fmv_f_x, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_fmv_f_x NOT LINKED in C++ binaries (Rust cargo tests are not those binaries). Execution evidence: cargo test --lib encode_fmv_f_x.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_fmv_f_x (float.rs:168) |
| Properties | 8 (6 passing, 2 failing) |
| Bugs | 2 medium (extra operand, RoundingMode third) |
| Sweep | 1 round spent (manual audit of 2-op / R-type / ABI / S-vs-D / w-vs-s / arity-class / extra / rm-third). Closed: tier round spent; remaining documented gaps are the two filed bugs. |

## File Coverage (this campaign)

| Source File | Funcs in scope | Candidates | Tested | Coverage | Status |
|-------------|----------------|------------|--------|----------|--------|
| float.rs | 14 | 1 (HARD: encode_fmv_f_x only) | 1 | 100% of campaign target | covered |
