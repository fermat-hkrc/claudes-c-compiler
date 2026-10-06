# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_bti, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed encode_bti NOT LINKED. Manual arm audit of the 12-line body used instead.
> Files: 11/11 scanned | Functions: 307 total | PBT candidates: 158 | This campaign tested: encode_bti (6/6 properties passing)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 158 |
| **This campaign target** | encode_bti |
| **This campaign result** | 6 / 6 passing (plus 4 KAT) |
| Coverage evidence | file-level (symbol presence); no LLVM profraw |

## Module Breakdown

| Module | Scanned | Tested this campaign | Notes |
|--------|---------|----------------------|-------|
| encode_bti (system.rs) | 1 | 1 | all match arms driven |
| other system.rs symbols | indexed | skipped | HARD single-symbol campaign |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Covered |
|-------------|-------|---------|
| differential | 1 | 1 |
| algebraic.invariant | 1 | 1 |
| algebraic.metamorphic | 2 | 2 |
| negative_error | 2 | 2 |

## File Coverage

| Source File | Funcs | This campaign |
|-------------|-------|---------------|
| system.rs | 18 | encode_bti tested |

## Sweep

coverage_gaps: no line-level data (build tree not instrumented / reporter missing). File-level claimed NOT LINKED against unrelated C++ binaries. Manual arm audit: omitted, c, j, jc, layout, j/c bits, case/whitespace, unknown, extra all have properties. Closed: every documented behavior has a property; standard tier 1/1 round spent.
