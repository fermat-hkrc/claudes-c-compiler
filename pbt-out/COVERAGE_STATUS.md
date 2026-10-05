# PBT Coverage Status

> Last updated: 2026-10-05 21:25 (campaign: encode_neon_faddp)
> Files: 10/10 scanned (100%) | Functions: 143/289 total | PBT candidates: 143 | Tested: 143 | encode_neon_faddp: 6 pass / 3 fail
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). The cargo lib tests did execute encode_neon_faddp.

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 143 |
| **Tested this campaign** | encode_neon_faddp (9 properties) |
| This campaign pass / fail | 6 / 3 |
| Coverage evidence | file-level (symbol presence) |
| Effort tier | standard |
| Sweep | 1/1: coverage_gaps had no LLVM profraw; manual arm audit plus encode_neon_faddp_diff_alt_spellings passing |

## This campaign

| Function | Source File | Tested | Notes |
|----------|-------------|--------|-------|
| encode_neon_faddp | neon.rs | yes | 6 passing / 3 failing; 3 bugs |

## Sweep

coverage_gaps: no .gcda/.profraw; C++ reporter claimed NOT LINKED (false for this Rust cargo target). Documented behaviors already had properties (vector/scalar differential, extra, mismatch, invalid T, dest-size). Sweep property: uppercase V alt-spellings, passing. Closed: tier round spent.
