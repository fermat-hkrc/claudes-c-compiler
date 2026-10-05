# PBT Coverage Status

> Last updated: 2026-10-05 23:33 (campaign: encode_dmb)
> Files: 11/11 scanned | Functions: 307 total | PBT candidates: 149 | Tested this campaign: encode_dmb | Coverage evidence: file-level (symbol presence)

## Summary

| Metric | Value |
|--------|-------|
| Total source files (FUNCTION_INDEX) | 11 |
| Files scanned | 11 / 11 |
| Total functions (all files) | 307 |
| PBT candidates | 149 |
| **Tested this campaign** | **encode_dmb** |
| This campaign pass / fail | 4 / 4 properties (plus 3 KAT pass, 4 regression fail) |
| Coverage evidence | file-level (symbol presence) — coverage_gaps had no .gcda/.profraw; C++ reporter claimed NOT LINKED. cargo test --lib encode_dmb executed the real symbol. |

## Module Breakdown

| Module | Scanned | Tested this campaign | Skipped | Notes |
|--------|---------|----------------------|---------|-------|
| encode_dmb (system.rs) | 18 | 1 | 17 | HARD: test only encode_dmb |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Passing | Failing |
|-------------|-------|---------|---------|
| differential | 2 | 1 | 1 |
| algebraic.metamorphic | 1 | 1 | 0 |
| algebraic.invariant | 1 | 1 | 0 |
| negative_error | 4 | 1 | 3 |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| system.rs | 18 | 1 | 1 | 100% of in-scope | covered (encode_dmb) |

## Untested in-scope functions

(none — encode_dmb is the only in-scope candidate)

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of encode_dmb: named Barrier/Symbol, unknown-name Err, and `_ => 0b1111` default are all driven. Closed: tier round spent.
