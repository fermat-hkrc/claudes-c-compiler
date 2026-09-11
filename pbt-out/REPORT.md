# PBT Campaign Report: encode_bic

## Summary

**Date:** 2026-09-11
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_bic (src/backend/arm/assembler/encoder/data_processing.rs)
**Tests:** 15 properties (8 passing, 7 failing) plus 3 KAT and 7 regression witnesses
**Result:** 8 passing, 7 bugs
**Effort tier:** standard (1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bic | 15 properties + 3 KAT + 7 regressions | 7 | differential (llvm-mc), algebraic.metamorphic, negative_error |

## Bugs Found

1. **Mixed-width GPRs accepted.** Law: all three BIC GPRs must be the same width. Counterexample: `bic w0, w0, x0`. Expected Err (llvm-mc rejects); actual Ok(Word) because sf is taken only from Rd. Severity: medium. Report: `pbt-out/bug_reports/encode_bic_mixed_width.md`. Regression: `test_encode_bic_regression_mixed_width`. Serial reconfirm: yes.

2. **SP/WSP accepted in register form.** Law: BIC shifted-register register 31 is XZR/WZR, never SP. Counterexample: `bic wsp, w0, w0`. Expected Err; actual encoded as `bic wzr, w0, w0`. Severity: medium. Report: `pbt-out/bug_reports/encode_bic_sp_register_form.md`. Regression: `test_encode_bic_regression_sp`. Serial reconfirm: yes.

3. **Out-of-range shift amount masked, not rejected.** Law: W-form shift amount in [0, 31]; bound+1 must Err. Counterexample: `bic w0, w0, w0, lsl #32`. Expected Err; actual Ok with imm6=32 (UNALLOCATED). Severity: medium. Report: `pbt-out/bug_reports/encode_bic_shift_out_of_range.md`. Regression: `test_encode_bic_regression_shift32`. Serial reconfirm: yes.

4. **FP/SIMD names accepted as GPRs.** Law: scalar BIC operands are GPRs only. Counterexample: `bic d0, x1, x2`. Expected Err; actual encoded as 32-bit `bic w0, x1, x2`. Severity: medium. Report: `pbt-out/bug_reports/encode_bic_fp_reg.md`. Regression: `test_encode_bic_regression_fp_reg`. Serial reconfirm: yes.

5. **NEON arrangements other than 8b/16b accepted.** Law: ARM ARM BIC vector T is 8B|16B. Counterexample: `bic v0.8h, v1.8h, v2.8h`. Expected Err; actual encoded as `bic v0.8b, ...` (Q=0). Severity: medium. Report: `pbt-out/bug_reports/encode_bic_invalid_neon_arr.md`. Regression: `test_encode_bic_regression_neon_8h`. Serial reconfirm: yes.

6. **Immediate-form XZR/WZR treated as SP/WSP.** Law: BIC-imm is AND-imm; Rd of 31 is SP, not XZR. Counterexample: `bic wzr, w0, #1`. Expected Err; actual encoded as `and wsp, w0, #0xfffffffe` (writes the stack pointer). Severity: high. Report: `pbt-out/bug_reports/encode_bic_imm_xzr_rd.md`. Regression: `test_encode_bic_regression_imm_xzr_rd`. Serial reconfirm: yes.

7. **Unknown shift kind mapped to LSL.** Law: shift kind ∈ {lsl, lsr, asr, ror}. Counterexample: Shift kind `"lslx"` amount 0 on `bic w0, w0, w0`. Expected Err; actual encoded as LSL (`_ => 0b00`). Severity: medium. Report: `pbt-out/bug_reports/encode_bic_unknown_shift_kind.md`. Regression: `test_encode_bic_regression_unknown_shift_kind`. Serial reconfirm: yes.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_bic_pbt) | 15 properties + 3 KAT + 7 regressions |

## Output Directories

- pbt-out/PLAN.md — campaign phases
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row for encode_bic
- pbt-out/FUNCTION_INDEX.md — encode_bic marked PBT candidate
- pbt-out/INVARIANTS.md — confirmed encode_bic invariants
- pbt-out/bug_reports/encode_bic_mixed_width.md
- pbt-out/bug_reports/encode_bic_sp_register_form.md
- pbt-out/bug_reports/encode_bic_shift_out_of_range.md
- pbt-out/bug_reports/encode_bic_fp_reg.md
- pbt-out/bug_reports/encode_bic_invalid_neon_arr.md
- pbt-out/bug_reports/encode_bic_imm_xzr_rd.md
- pbt-out/bug_reports/encode_bic_unknown_shift_kind.md

## Sweep

Contract-surface sweep: 1 round (standard). `coverage_gaps` had no LLVM profraw. Manual arm audit of encode_bic added properties for invalid bitmask immediate, unsupported third operand, invalid Rm name (all passing) and unknown shift kind (failing — bug 7). Close reason: tier's one sweep round done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 12:05 (campaign: coverage)
> Files: 4/4 scanned (100%) | Functions: 6/96 total | PBT candidates: 6 | Tested: 6 (100%) | 0 pass, 6 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 4 |
| Files scanned | 4 / 4 (100%) |
| Total functions (all files) | 96 |
| PBT candidates (from FUNCTION_INDEX) | 6 |
| **Tested (of PBT candidates)** | **6 / 6 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 6 / 0 |
| **Overall (tested / all functions)** | **6 / 96 (6%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 6 | 6 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 6 | 6 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 3 | 3 | 100% | covered |
| load_store.rs | 20 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
| classify_cast_with_f128 | cast.rs |
| encode_adc | data_processing.rs |
| encode_adr | load_store.rs |
| encode_bic | data_processing.rs |
