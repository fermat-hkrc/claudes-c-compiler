# PBT Coverage Status

> Last updated: 2026-10-06 08:46 (campaign: encode_uxtb)
> Files: 11/11 scanned (100%) | Functions: 174/307 total | PBT candidates: 175 | Tested: 174 | Coverage evidence: file-level (symbol presence)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 175 |
| **Tested (of PBT candidates)** | **174 / 175** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 173 / 0 |
| **Overall (tested / all functions)** | **174 / 307 (57%)** |
| Untested | 0 (this campaign: encode_uxtb) |
| Skipped | 0 |
| Coverage evidence | file-level (symbol presence) — coverage_gaps found no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed encode_uxtb NOT LINKED. Sweep was a manual arm audit of the 7-line body. |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
| encode_uxtb | 1 | 1 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| data_processing.rs | 36 | 30 | 30 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> encode_uxtb has 5 failing properties (X dest, extra operand, X source, SP, FP).
