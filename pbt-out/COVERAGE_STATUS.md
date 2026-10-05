# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_two_misc)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw
> Files: 10/10 scanned | Functions: 127/289 PBT candidates | This campaign tested encode_neon_two_misc

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 127 |
| This campaign target | encode_neon_two_misc |
| Properties | 10 (6 passing, 4 failing) |
| Coverage evidence | file-level (symbol presence); no LLVM profraw. Manual arm audit of encode_neon_two_misc. |

## This campaign

| Function | Source file | Tested | Notes |
|----------|-------------|--------|-------|
| encode_neon_two_misc | neon.rs | yes | 6 passing / 4 failing; sweep alt-spellings + nonreg |

## Sweep

coverage_gaps: no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed encode_neon_two_misc NOT LINKED (false negative — cargo test --lib ran the symbol). Manual arm audit: get_neon_reg dest, get_neon_reg src, neon_arr_to_q_size, Ok Word all driven. Added alt-spellings and nonreg (both passing). Tier round 1/1 spent.
