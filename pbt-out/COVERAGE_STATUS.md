# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_stop, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw
> Files: 11/11 scanned | Functions: 307 total | PBT candidates: 166 | This campaign tested: encode_stop

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 166 |
| This campaign target | encode_stop (load_store.rs:927) |
| This campaign properties | 11 (7 passing, 4 failing) |
| Coverage evidence | file-level (symbol presence); cargo tests executed encode_stop (KATs passed) |

## This campaign

| Function | Source file | Test file | Result |
|----------|-------------|-----------|--------|
| encode_stop | load_store.rs | encode_stop_pbt.rs | 7 passing / 4 failing (4 bugs); sweep invalid-name/alt-spellings/unknown-op passing |

## Sweep

Round 1/1 spent. coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of the 45-line body plus encode_stop_neg_invalid_name, encode_stop_diff_alt_spellings, encode_stop_neg_unknown_op. Closed: every documented behavior has a property.
