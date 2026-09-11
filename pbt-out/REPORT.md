# PBT Campaign Report: encode_bics

## Summary

**Date:** 2026-04-08
**Repository:** claudes-c-compiler
**Modules tested:** encode_bics
**Tests:** 10 properties (plus 2 KAT + 7 regression witnesses)
**Result:** 5 passing, 5 failing (7 SUT bugs)
**Effort tier:** standard (1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bics | 10 properties (5 pass / 5 fail) | 7 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **Mixed-width GPRs accepted.** `bics w0, w0, x0` encodes instead of Err. sf taken only from Rd. Counterexample: rd=rn=rm=0, rd64=false, rn64=false, rm64=true. Serial reconfirm. Report: `pbt-out/bug_reports/encode_bics_mixed_width.md`. Regression: `test_encode_bics_regression_mixed_width`.

2. **SP/WSP accepted as register 31.** `bics wsp, w0, w0` encodes as BICS WZR. `parse_reg_num` maps sp/wsp to 31. Serial reconfirm. Report: `pbt-out/bug_reports/encode_bics_sp_register_form.md`. Regression: `test_encode_bics_regression_sp`.

3. **FP/SIMD names accepted as GPRs.** `bics d0, x1, x2` encodes as 32-bit BICS w0. Serial reconfirm. Report: `pbt-out/bug_reports/encode_bics_fp_reg.md`. Regression: `test_encode_bics_regression_fp_reg`.

4. **Out-of-range shift amount accepted.** `bics w0, w0, w0, lsl #32` encodes; amount masked with 0x3F. Serial reconfirm. Report: `pbt-out/bug_reports/encode_bics_shift_out_of_range.md`. Regression: `test_encode_bics_regression_shift32`.

5. **Unknown shift kind defaults to LSL.** `bics w0, w0, w0, lslx #0` encodes as unshifted BICS. Serial reconfirm. Report: `pbt-out/bug_reports/encode_bics_unknown_shift_kind.md`. Regression: `test_encode_bics_regression_unknown_shift_kind`.

6. **GNU BICS-immediate alias missing.** `bics w0, w0, #0xaaaaaaaa` is rejected; llvm-mc assembles it as `ands w0, w0, #0x55555555` (0x7200f000). Serial reconfirm. Report: `pbt-out/bug_reports/encode_bics_imm_alias.md`. Regression: `test_encode_bics_regression_imm`.

7. **Trailing non-shift 4th operand ignored.** `bics x0, x1, x2, x3` encodes as `bics x0, x1, x2`. Sweep round. Serial reconfirm. Report: `pbt-out/bug_reports/encode_bics_extra_operand.md`. Regression: `test_encode_bics_regression_extra_operand`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_bics_pbt) | 2 KAT + 10 properties + 7 regressions |

## Output Directories

- `pbt-out/PLAN.md` — campaign phases
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/FUNCTION_INDEX.md` — merged function index
- `pbt-out/INVARIANTS.md` — confirmed invariants (encode_bics section appended)
- `pbt-out/bug_reports/encode_bics_*.md` — 7 bug reports

Sweep close-out: `coverage_gaps` had no LLVM profraw; one manual arm-audit round added invalid-rm (passing) and extra-operand (failing) properties. Tier allowance is 1 round; closed after that round.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 12:32 (campaign: coverage)
> Files: 5/5 scanned (100%) | Functions: 8/164 total | PBT candidates: 8 | Tested: 8 (100%) | 0 pass, 8 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 5 |
| Files scanned | 5 / 5 (100%) |
| Total functions (all files) | 164 |
| PBT candidates (from FUNCTION_INDEX) | 8 |
| **Tested (of PBT candidates)** | **8 / 8 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 8 / 0 |
| **Overall (tested / all functions)** | **8 / 164 (5%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 8 | 8 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 8 | 8 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 4 | 4 | 100% | covered |
| load_store.rs | 20 | 1 | 1 | 100% | covered |
| neon.rs | 68 | 1 | 1 | 100% | covered |

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
| encode_neon_three_diff_narrow | neon.rs |
| encode_bics | data_processing.rs |
