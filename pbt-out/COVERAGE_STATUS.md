# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_addv)
> Files: 10/10 scanned | Functions: 116/289 PBT candidates | Tested this campaign: encode_neon_addv | Coverage evidence: file-level (symbol presence)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 116 |
| **Tested this campaign** | encode_neon_addv |
| This campaign properties | 8 (2 passing, 4 failing, 2 retired duplicate oracles) |
| This campaign bugs | 4 |
| Coverage evidence | file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo lib tests executed encode_neon_addv. |

## Module Breakdown

| Module | Scanned | Tested this campaign | Notes |
|--------|---------|----------------------|-------|
| encode_neon_addv | yes | yes | 8 properties; 4 bugs |

## File Coverage

| Source File | Status |
|-------------|--------|
| neon.rs | encode_neon_addv covered this campaign (2 passing / 4 failing / 2 retired) |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw. Manual arm audit of encode_neon_addv: arity Err, get_neon_reg Err, neon_arr_to_q_size Err, and Ok(Word) are all driven by existing properties. Closed: tier round spent.

## Recommended Focus

Fix encode_neon_addv encoding (`0b110111 << 10` → `(0b11011 << 12) | (0b10 << 10)`), then extra-operand / reserved-T / dest-type checks.
