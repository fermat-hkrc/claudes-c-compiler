# PBT Campaign Report: encode_bl

## Summary

**Date:** 2026-09-11
**Repository:** claudes-c-compiler
**Modules tested:** encode_bl
**Tests:** 9 properties (plus 3 KAT + 3 regression witnesses)
**Result:** 6 passing, 3 failing (3 SUT bugs)
**Effort tier:** standard (1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bl | 9 properties (6 pass / 3 fail) | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_bl_diff_imm_llvm_mc** (differential). Law: ∀ aligned imm in [-2^27, 2^27-4], encode_bl([Imm(imm)]) = Word(llvm-mc("bl #imm")). Shrunk counterexample: `imm = -134217728` (`bl #-134217728`). Expected: Word(0x96000000). Actual: Err("expected symbol at operand 0, got Some(Imm(-134217728))"). Serial reconfirm (`PBT_TEST_JOBS=1`). Report: `pbt-out/bug_reports/encode_bl_imm_offset.md`. Regression: `test_encode_bl_regression_imm_offset`.

2. **encode_bl_neg_extra_operand** (negative_error). Law: ∀ extra ∈ {Reg,Imm,Symbol,Mem}, encode_bl([Symbol(s), extra]) is Err. Shrunk counterexample: `suffix = 0, which = 0` (`bl labl0, x0`). Expected: Err. Actual: Ok(WordWithReloc Call26 symbol=labl0). Serial reconfirm (`PBT_TEST_JOBS=1`). Report: `pbt-out/bug_reports/encode_bl_extra_operand.md`. Regression: `test_encode_bl_regression_extra_operand`.

3. **encode_bl_neg_bad_operand** (negative_error). Law: encode_bl([Modifier|ModifierOffset|...]) is Err. Shrunk counterexample: `which = 4` (Modifier { kind: lo12, symbol: foo }). Expected: Err. Actual: Ok(WordWithReloc Call26 symbol=foo addend=0). Serial reconfirm (`PBT_TEST_JOBS=1`). Report: `pbt-out/bug_reports/encode_bl_modifier.md`. Regression: `test_encode_bl_regression_modifier`.

## Design Caveats

- Parser-misclassified Reg/Cond/Barrier names are treated as BL symbols. `Doc evidence: src/backend/arm/assembler/encoder/mod.rs:982-986` — "The parser misclassifies symbol names that collide with register names, condition codes, or barrier names. These are valid symbols in context." Property `encode_bl_symbol_misclassified` asserts Call26 with that name. llvm-mc accepts `bl eq` / `bl sy` as labels; it rejects `bl x0` as a register, but the encoder comment owns the workaround.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_bl_pbt) | 9 properties + 3 KAT + 3 regressions |

## Output Directories

- `pbt-out/PLAN.md` — campaign phases
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/FUNCTION_INDEX.md` — merged function index (compare_branch.rs appended)
- `pbt-out/COVERAGE.md` — per-function coverage ledger
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/INVARIANTS.md` — confirmed invariants (encode_bl section appended)
- `pbt-out/REPORT.md` — this report
- `pbt-out/bug_reports/encode_bl_imm_offset.md`
- `pbt-out/bug_reports/encode_bl_extra_operand.md`
- `pbt-out/bug_reports/encode_bl_modifier.md`

Contract-surface sweep: 1 round (tier allowance). `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol arms reachable from encode_bl (Reg/Cond/Barrier covered by `encode_bl_symbol_misclassified`; Modifier/ModifierOffset filed as bugs). Closed because the tier's 1 round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 12:44 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 9/184 total | PBT candidates: 9 | Tested: 9 (100%) | 0 pass, 9 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 9 |
| **Tested (of PBT candidates)** | **9 / 9 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 9 / 0 |
| **Overall (tested / all functions)** | **9 / 184 (5%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 9 | 9 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 9 | 9 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 1 | 1 | 100% | covered |
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
| encode_bl | compare_branch.rs |
