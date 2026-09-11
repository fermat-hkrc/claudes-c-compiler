# PBT Campaign Report: encode_adc

## Summary

**Date:** 2026-09-11
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_adc
**Tests:** 10 properties (plus 1 KAT + 4 regression witnesses)
**Result:** 6 passing, 4 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_adc | 10 properties | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_adc silently ignores a trailing shift operand**
   - Law: ARM ADC has no shifted-register form; extra Shift must Err.
   - Minimal input: `[w0, w0, w0, lsl #0]`, set_flags=false
   - Expected: Err. Actual: Ok(Word) — extra operand ignored.
   - Severity: medium
   - Bug report: pbt-out/bug_reports/encode_adc_extra_shift_ignored.md

2. **encode_adc accepts mixed 32/64-bit register operands**
   - Law: all three registers must be the same width.
   - Minimal input: `[w0, w0, x0]`, set_flags=false
   - Expected: Err. Actual: Ok(Word) using only Rd's sf bit.
   - Severity: medium
   - Bug report: pbt-out/bug_reports/encode_adc_mixed_width.md

3. **encode_adc treats SP/WSP as XZR/WZR**
   - Law: register 31 in ADC is WZR/XZR, not WSP/SP; SP operands must Err.
   - Minimal input: `[wsp, w0, w0]`, set_flags=false
   - Expected: Err. Actual: Ok(Word) identical to `adc wzr, w0, w0`.
   - Severity: high
   - Bug report: pbt-out/bug_reports/encode_adc_sp_as_zr.md

4. **encode_adc accepts FP/SIMD register names as GPRs**
   - Law: ADC operands are W/X GPRs only.
   - Minimal input: `[d0, x1, x2]`, set_flags=false
   - Expected: Err. Actual: Ok(Word) treated as w0.
   - Severity: medium
   - Bug report: pbt-out/bug_reports/encode_adc_fp_reg.md

All four reproduced serially (`PBT_TEST_JOBS=1`).

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_adc_pbt) | 10 properties + KAT + 4 regressions |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/bug_reports/encode_adc_extra_shift_ignored.md
- pbt-out/bug_reports/encode_adc_mixed_width.md
- pbt-out/bug_reports/encode_adc_sp_as_zr.md
- pbt-out/bug_reports/encode_adc_fp_reg.md

## Contract-surface sweep

STANDARD owes 1 round. `coverage_gaps` had no LLVM profraw (same environment quirk as prior campaigns). Manual ARM-field audit of encode_adc added `encode_adc_neg_invalid_reg_name` (passing) and `encode_adc_neg_fp_reg` (failing bug). Closed because the tier's 1 round is done.

## SUT observations (not bugs)

- Valid same-width GPR ADC/ADCS, including XZR/WZR (register 31), matches llvm-mc for 1000 random cases per property.
- ADC vs ADCS encodings differ only by bit 29 (S).
- ARM ARM field layout (sf, op=0, S, opcode 11010000, Rm/Rn/Rd, bits 15:10 zero) holds on the success path.
- Fewer than 3 operands and a non-register in any of the three slots return Err.
- Invalid register names (x32, empty, foo, r0) return Err via parse_reg_num.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 11:18 (campaign: coverage)
> Files: 3/3 scanned (100%) | Functions: 4/76 total | PBT candidates: 4 | Tested: 4 (100%) | 0 pass, 4 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 3 |
| Files scanned | 3 / 3 (100%) |
| Total functions (all files) | 76 |
| PBT candidates (from FUNCTION_INDEX) | 4 |
| **Tested (of PBT candidates)** | **4 / 4 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 4 / 0 |
| **Overall (tested / all functions)** | **4 / 76 (5%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 4 | 4 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 4 | 4 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 2 | 2 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
| classify_cast_with_f128 | cast.rs |
| encode_adc | data_processing.rs |
