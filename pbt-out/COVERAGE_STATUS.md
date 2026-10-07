# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_v_crypto_vs, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_v_crypto_vs NOT LINKED in C++ reporter binaries (Rust cargo tests are not those binaries).

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_v_crypto_vs | vector.rs | yes | 4 passing / 3 failing (3 bugs) |

## Summary

| Metric | Value |
|--------|-------|
| Scope | src/backend/riscv/assembler/encoder/vector.rs#encode_v_crypto_vs |
| PBT candidates in FUNCTION_INDEX | 236 |
| This campaign target | encode_v_crypto_vs |
| Properties | 7 (4 passing, 3 failing) |
| Sweep | 1 round (standard); remaining documented gaps are the three filed bugs |

## File Coverage (this campaign)

| Source File | Target | Test file | Status |
|-------------|--------|-----------|--------|
| vector.rs | encode_v_crypto_vs | encode_v_crypto_vs_pbt.rs | covered (4 pass / 3 fail) |
