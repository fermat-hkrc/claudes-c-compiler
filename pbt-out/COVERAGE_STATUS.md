# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_hint, English)
> Files: 11/11 scanned (100%) | Functions: 307 total | PBT candidates: 157 | This campaign tested: encode_hint
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and claimed NOT LINKED against unrelated C++ binaries; cargo test --lib encode_hint executed the production symbol.

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 157 |
| This campaign target | encode_hint |
| encode_hint properties | 5 passing / 2 failing |
| Coverage evidence | file-level (symbol presence) |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_hint | system.rs | yes | 5 passing / 2 failing (2 bugs) |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw. Manual arm audit of encode_hint (valid imm7, ARM layout, imm isolation, empty, wrong-kind, extra, oob). Closed: every documented behavior has a property; tier round spent.
