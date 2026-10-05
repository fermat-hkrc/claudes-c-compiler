# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_elem, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). cargo test --lib encode_neon_elem_pbt executed encode_neon_elem (2 KAT + 6 passing properties at 1000 cases).

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_neon_elem (neon.rs:1591) |
| Properties | 12 (6 passing / 6 failing) |
| KAT | 2 passing |
| Regression witnesses | 6 failing |
| Bugs | 6 medium |
| Sweep | 1/1 (unsupported-t / alt-spellings passing; index-oob / lane-elem failing) |
| Tier | standard |

## File Coverage (this campaign)

| Source File | Function | Tested | Status |
|-------------|----------|--------|--------|
| neon.rs | encode_neon_elem | yes | 6 pass / 6 fail |
