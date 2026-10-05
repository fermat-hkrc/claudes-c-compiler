# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_across)
> Files: 10/10 scanned | Functions: 117/289 PBT candidates | Tested this campaign: encode_neon_across | Coverage evidence: file-level (symbol presence)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 117 |
| **Tested this campaign** | encode_neon_across |
| This campaign properties | 9 (6 passing, 3 failing) |
| This campaign bugs | 3 |
| Coverage evidence | file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo lib tests executed encode_neon_across. |

## Module Breakdown

| Module | Scanned | Tested this campaign | Notes |
|--------|---------|----------------------|-------|
| encode_neon_across | yes | yes | 9 properties; 3 bugs |

## File Coverage

| Source File | Status |
|-------------|--------|
| neon.rs | encode_neon_across covered this campaign (6 passing / 3 failing) |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw. Manual arm audit of encode_neon_across: arity Err, get_neon_reg Err, neon_arr_to_q_size Err, and Ok(Word) are all driven by existing properties. Added alt-spellings differential. Closed: tier round spent.

## Recommended Focus

Fix encode_neon_across extra-operand check (`len < 2` → `len != 2`), reject reserved T (2s/1d/2d), and require dest Bd/Hd/Sd matching T.
