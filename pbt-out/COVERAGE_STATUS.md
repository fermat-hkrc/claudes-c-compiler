# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_sri)
> Files: 10/10 scanned (100%) | Functions: 125/289 PBT candidates | Tested this campaign: encode_neon_sri
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw from cargo test

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 125 |
| **Tested this campaign** | encode_neon_sri (10 properties) |
| Coverage evidence | file-level (symbol presence) |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_neon_sri | neon.rs:1285 | yes | 6 passing / 4 failing |

## Sweep

coverage_gaps: no LLVM profraw (C++ reporter listed unrelated binaries, claimed NOT LINKED). Manual arm audit of encode_neon_sri: arity Err, get_neon_reg, get_imm, neon_arr_to_q_size, match T, Ok Word all driven. Added alt-spellings differential and non-register negative-error (both passing). Closed: standard tier 1/1 round spent.
