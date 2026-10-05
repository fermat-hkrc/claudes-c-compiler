# PBT Coverage Status

> Last updated: 2026-10-05
> Campaign: encode_neon_sshr (standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_neon_sshr |
| Properties | 10 |
| Passing | 6 |
| Failing | 4 |
| Bugs | 4 |
| Sweep | 1/1 (alt-spellings + nonreg; both passing) |

## Function

| Function | Source file | Tested | Notes |
|----------|-------------|--------|-------|
| encode_neon_sshr | neon.rs | yes | 6 passing / 4 failing; 4 medium bugs |

## Sweep

coverage_gaps had no LLVM profraw. Manual arm audit of encode_neon_sshr: arity Err, get_neon_reg, get_imm, neon_arr_to_q_size, match T, Ok Word driven. Added alt-spellings differential and Imm/Mem/Label negative-error (both passing). Closed: tier round spent.
