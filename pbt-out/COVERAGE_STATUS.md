# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_store, English, standard)
> Files: 12/12 scanned | Functions: 324 total | PBT candidates: 189 | This campaign tested: encode_store
> Coverage evidence: file-level (symbol presence) — coverage_gaps had no .gcda/.profraw

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 189 |
| This campaign target | encode_store (base.rs:194) |
| Properties this campaign | 9 (4 passing, 5 failing) |
| Bugs this campaign | 5 |
| Coverage evidence | file-level (symbol presence); no LLVM profraw |

## This campaign

| Function | Source | Tested | Notes |
|----------|--------|--------|-------|
| encode_store | base.rs | yes | 4 passing / 5 failing; Mem, MemSymbol, Err arms executed |

## Sweep

coverage_gaps: no line-level data (C++ reporter, unrelated binaries, claimed NOT LINKED). Manual arm audit plus encode_store_neg_other_modifier. Closed: standard tier 1/1 round spent.
