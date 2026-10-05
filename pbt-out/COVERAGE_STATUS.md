# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_zip_uzp)
> Files: 10/10 scanned (100%) | Functions: 118/289 total | PBT candidates: 118 | Tested: 118 | Coverage evidence: file-level (symbol presence)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 118 |
| **Tested (of PBT candidates)** | **118 / 118** |
| This campaign target | encode_neon_zip_uzp |
| This campaign properties | 5 passing / 4 failing (+ 1 passing alt-spellings sweep) |
| **Coverage evidence** | file-level (symbol presence) — coverage_gaps found no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED. Manual arm audit: arity Err, get_neon_reg Err, neon_arr_to_q_size Err, Ok Word all driven. |

## File Coverage (this campaign)

| Source File | Target | Tested | Notes |
|-------------|--------|--------|-------|
| neon.rs | encode_neon_zip_uzp | yes | 9 properties on cargo test --lib encode_neon_zip_uzp |

## Recommended Focus

Fix the four failing encode_neon_zip_uzp negative contracts (extra operand, reserved 1d, mismatched T, bare/GPR source) before treating the permute encoder as gas-compatible.
