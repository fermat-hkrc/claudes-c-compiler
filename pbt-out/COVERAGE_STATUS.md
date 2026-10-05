# PBT Coverage Status

> Last updated: 2026-10-05 13:30 (campaign: encode_neon_eor3)
> Files: 10/10 scanned (100%) | Functions: 119/289 total | PBT candidates: 119 | Tested: 119 | encode_neon_eor3: 6 pass / 4 fail
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and listed encode_neon_eor3 as NOT LINKED in unrelated C++ binaries. cargo test --lib encode_neon_eor3 executed the SUT.

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 119 |
| **Tested (of PBT candidates)** | **119 / 119** |
| This campaign target | encode_neon_eor3 (6 passing / 4 failing properties) |
| **Overall (tested / all functions)** | **119 / 289 (41%)** |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
| encode_neon_eor3 | 1 | 1 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered |
|-------------|-------|---------|
| differential | 2 | 2 |
| algebraic.metamorphic | 1 | 1 |
| algebraic.invariant | 1 | 1 |
| negative_error | 6 | 6 |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| neon.rs | 68 | 34 | 34 | 100% | covered |

## This campaign — encode_neon_eor3

Passing: diff_llvm_mc, metamorphic_rd_rn_rm_rk, invariant_arm_fields, neg_arity, diff_alt_spellings, neg_nonreg
Failing: neg_extra_operand, neg_invalid_t, neg_mismatched_t, neg_gpr_or_bare
