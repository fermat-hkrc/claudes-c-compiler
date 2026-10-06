# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_uxth, English)
> Files: 11/11 scanned (100%) | Functions: 174/307 total PBT candidates | Tested this campaign: encode_uxth

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 174 |
| **Tested this campaign** | encode_uxth (12 properties) |
| Coverage evidence | file-level (symbol presence) — no .gcda/.profraw |

## This campaign

| Function | Source file | Result |
|----------|-------------|--------|
| encode_uxth | data_processing.rs | 7 passing / 5 failing (plus 3 KAT + 5 regression witnesses; 5 bugs) |

## Sweep

coverage_gaps: no LLVM profraw (C++ reporter listed unrelated binaries and claimed encode_uxth NOT LINKED). Manual arm audit of the 7-line body plus encode_uxth_neg_nonreg / encode_uxth_neg_invalid_name / encode_uxth_diff_alt_spellings / encode_uxth_meta_rd_rn. Closed: every documented behavior has a property; tier round 1/1 spent.

## Oracle Type Distribution (this campaign)

| Oracle Type | Count |
|-------------|-------|
| differential | 2 |
| algebraic.metamorphic | 2 |
| algebraic.invariant | 1 |
| negative_error | 7 |
