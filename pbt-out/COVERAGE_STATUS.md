# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_ld1r, English, standard)
> Files: 10/10 scanned | Functions: 103/289 PBT candidates | Tested this campaign: encode_neon_ld1r
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 103 |
| Tested this campaign | encode_neon_ld1r (9 passing / 4 failing properties) |
| Coverage evidence | file-level (symbol presence); cargo test --lib encode_neon_ld1r executed the symbol |

## This campaign

| Function | Source file | Result |
|----------|-------------|--------|
| encode_neon_ld1r | neon.rs | 9 pass / 4 fail (KAT + 13 properties + 6 regression witnesses) |

## Sweep

coverage_gaps: no line-level data. Manual arm audit covered register post-index, illegal post-imm, alt spellings, invalid names, and [Xn, #imm]. Closed: standard tier 1/1 round spent.
