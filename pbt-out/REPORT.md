# PBT Campaign Report: encode_neon_three_diff_narrow

## Summary

**Date:** 2026-09-11
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_three_diff_narrow (src/backend/arm/assembler/encoder/neon.rs)
**Tests:** 12 properties (8 passing, 4 failing) plus 1 KAT and 4 regression witnesses
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_three_diff_narrow | 12 properties + 1 KAT + 4 regressions | 4 | differential (llvm-mc), algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **Destination arrangement Tb ignored.** Law: ARM ARM ADDHN Vd.Tb is paired with Vn.Ta (8H→8B / 16B for *2, 4S→4H / 8H, 2D→2S / 4S). Counterexample: `addhn2 v0.8b, v0.8h, v0.8h`. Expected Err (llvm-mc rejects); actual Ok(Word) because dest arrangement is discarded and Q comes only from `is_high`. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_three_diff_narrow_mismatched_dest_tb.md`. Regression: `test_encode_neon_three_diff_narrow_regression_mismatched_dest_tb`. Serial reconfirm: yes (`PBT_TEST_JOBS=1`).

2. **Rm arrangement Ta ignored.** Law: Vm.Ta must equal Vn.Ta. Counterexample: `addhn v0.4h, v0.4s, v0.8h`. Expected Err; actual Ok(Word) — size taken only from operand 1; Rm arrangement discarded. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_three_diff_narrow_rm_ta_mismatch.md`. Regression: `test_encode_neon_three_diff_narrow_regression_rm_ta_mismatch`. Serial reconfirm: yes.

3. **Fourth operand silently ignored.** Law: ADDHN is a three-register instruction. Counterexample: `addhn v0.8b, v0.8h, v0.8h, v0.8h`. Expected Err; actual Ok(Word) — only `len < 3` is checked. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_three_diff_narrow_extra_operand.md`. Regression: `test_encode_neon_three_diff_narrow_regression_extra_operand`. Serial reconfirm: yes.

4. **GPR/FP names accepted as NEON Vd.** Law: dest is Vd.Tb. Counterexample: `addhn x0, v0.8h, v0.8h`. Expected Err; actual encoded as `addhn v0.8b, v0.8h, v0.8h` (`parse_reg_num("x0")` = 0). Severity: medium. Report: `pbt-out/bug_reports/encode_neon_three_diff_narrow_gpr_dest.md`. Regression: `test_encode_neon_three_diff_narrow_regression_gpr_dest`. Serial reconfirm: yes.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_three_diff_narrow_pbt) | 12 properties + 1 KAT + 4 regressions |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/FUNCTION_INDEX.md` — merged function index (neon.rs appended)
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/INVARIANTS.md` — confirmed invariants
- `pbt-out/REPORT.md` — this report
- `pbt-out/bug_reports/encode_neon_three_diff_narrow_mismatched_dest_tb.md`
- `pbt-out/bug_reports/encode_neon_three_diff_narrow_rm_ta_mismatch.md`
- `pbt-out/bug_reports/encode_neon_three_diff_narrow_extra_operand.md`
- `pbt-out/bug_reports/encode_neon_three_diff_narrow_gpr_dest.md`

## Coverage-sweep close

Closed after exactly 1 coverage-driven round (standard tier). `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset by the environment). Manual arm audit of `encode_neon_three_diff_narrow`: every match arm (Ta=8h/4s/2d and unsupported), both Q values, both U values, both opcodes, arity 0..=2, non-reg operands, and get_neon_reg parse failure (v32/foo/empty) now have properties. Remaining documented contract holes (dest Tb, Rm Ta, extra operand, GPR dest) are failing properties with bug reports, not untested surface.

## Skipped targets

(none) — campaign scoped to encode_neon_three_diff_narrow; `cargo check --lib` / `cargo test --lib` harness used throughout. No easier-target swap.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 12:19 (campaign: coverage)
> Files: 5/5 scanned (100%) | Functions: 7/164 total | PBT candidates: 7 | Tested: 7 (100%) | 0 pass, 7 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 5 |
| Files scanned | 5 / 5 (100%) |
| Total functions (all files) | 164 |
| PBT candidates (from FUNCTION_INDEX) | 7 |
| **Tested (of PBT candidates)** | **7 / 7 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 7 / 0 |
| **Overall (tested / all functions)** | **7 / 164 (4%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 7 | 7 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 7 | 7 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 3 | 3 | 100% | covered |
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
