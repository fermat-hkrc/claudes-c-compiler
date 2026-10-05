# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_scalar_qshrn, English)
> Files: 10/10 scanned (100%) | Functions: 147/289 PBT candidates | Tested this campaign: encode_neon_scalar_qshrn
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 147 |
| This campaign target | encode_neon_scalar_qshrn |
| This campaign properties | 10 (5 passing, 5 failing) |
| This campaign bugs | 3 unique defects (5 failing properties; encoding hit by 3 properties) |
| Coverage evidence | file-level (symbol presence) |

## This campaign

| Function | Properties | Pass | Fail | Notes |
|----------|------------|------|------|-------|
| encode_neon_scalar_qshrn | 10 | 5 | 5 | differential vs llvm-mc; ARM asisdshf invariant; extra operand; wrong class; sweep dest/nonreg |

## Sweep

Round 1/1 spent. coverage_gaps: no native LLVM/gcov data; C++ reporter claimed NOT LINKED. Manual arm audit of neon.rs:1835-1851. Added neg_unsupported_dest and neg_nonreg (both passing).
