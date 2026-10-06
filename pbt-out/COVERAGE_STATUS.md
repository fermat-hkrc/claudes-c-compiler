# PBT Coverage Status

> Last updated: 2026-10-06 18:38 (campaign: encode_fp_unary)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw
> Files: 14/14 scanned | Functions: 351 total | PBT candidates: 205 | This campaign tested: encode_fp_unary

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 14 / 14 |
| Total functions (all files) | 351 |
| PBT candidates (from FUNCTION_INDEX) | 205 |
| This campaign target | encode_fp_unary |
| This campaign properties | 8 (6 passing, 2 failing) |
| Coverage evidence | file-level (symbol presence) |

## This campaign

| Function | Source file | Tested | Notes |
|----------|-------------|--------|-------|
| encode_fp_unary | float.rs | yes | 6 passing / 2 failing; extra operand and non-rm 3rd are filed bugs |

## Sweep

coverage_gaps: no line-level data (no .gcda/.profraw). File-level reporter listed unrelated C++ binaries and marked encode_fp_unary NOT LINKED (reporter gap; the cargo test binary did execute the symbol). Manual audit: 2-op, rm, R-type, ABI, dyn-default, arity/GPR, extra, non-rm 3rd all have properties. Closed: standard tier 1/1 round spent.
