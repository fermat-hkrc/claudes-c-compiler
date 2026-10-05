# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_ld_st_multi, English)
> Files: 10/10 scanned (100%) | Functions: 105/289 total | PBT candidates: 105 | Tested: 105 | encode_neon_ld_st_multi: 7 pass / 7 fail
> Coverage evidence: none — coverage_gaps reported no .gcda/.profraw (Rust cargo test tree was configured before the campaign). File-level fallback listed unrelated C++ binaries and incorrectly said encode_neon_ld_st_multi is NOT LINKED. Manual evidence: `cargo test --lib encode_neon_ld_st_multi` executed the production symbol (KAT + 1000-case properties).

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 105 |
| **Tested (of PBT candidates)** | **105 / 105 (100%)** |
| **Overall (tested / all functions)** | **105 / 289 (36%)** |
| Untested PBT candidates | 0 |
| Skipped | 0 |
| Effort tier | standard |

## This campaign (encode_neon_ld_st_multi)

| Metric | Value |
|--------|-------|
| Properties | 14 |
| Passing | 7 |
| Failing | 7 |
| KAT | 1 passing |
| Regression witnesses | 7 failing |
| Bugs | 7 |

## File Coverage

| Source File | Notes |
|-------------|-------|
| neon.rs | encode_neon_ld_st_multi tested this campaign; other neon.rs encode_* covered by prior campaigns |
