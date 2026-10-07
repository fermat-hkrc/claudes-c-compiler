# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_v_crypto_vv, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_v_crypto_vv NOT LINKED in C++ reporter binaries. The Rust `cargo test --lib encode_v_crypto_vv` run executed the symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_v_crypto_vv |
| Source | vector.rs:201 |
| Properties | 7 (4 passing, 3 failing) |
| Bugs | 3 |
| Sweep | 1 round (standard); remaining gaps are the filed bugs |

## File Coverage (this target)

| Source File | Function | Tested | Notes |
|-------------|----------|--------|-------|
| vector.rs | encode_v_crypto_vv | yes | 4 pass / 3 fail; KATs linked the symbol |
