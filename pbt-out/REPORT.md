# PBT Campaign Report: encode_neon_shift_right

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_shift_right
**Tests:** 15 total (6 passing properties + 4 failing properties + 1 passing KAT + 4 failing regression witnesses)
**Result:** 6 passing properties, 4 failing properties (4 bug-report files)
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 strengthening/sweep round, ≥1 metamorphic/differential, 1 coverage_gaps round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_shift_right | 15 | 4 | differential (llvm-mc), algebraic.metamorphic (U bit, Q/opcode), algebraic.invariant (ARM fields), negative_error |

## Bugs Found

Each entry is a failing proptest property with a shrunk counterexample, reconfirmed serially (`PBT_TEST_JOBS=1`). Expected: Err. Actual: Ok(Word).

1. **encode_neon_shift_right_neg_mismatched_t** — Falsifiable. Shrunk counterexample: `rd=0, rn=0, td="8b", ts="16b", shift=1, u_bit=0, opcode=9` (`srshr v0.8b, v0.16b, #1`). Law: dest T must equal source T. Report: `pbt-out/bug_reports/encode_neon_shift_right_mismatched_t.md`.

2. **encode_neon_shift_right_neg_extra_operand** — Falsifiable. Shrunk counterexample: `rd=0, rn=0, extra=0, t="8b", shift=1, u_bit=0, opcode=9` (`srshr v0.8b, v0.8b, #1, v0.8b`). Law: exactly three operands. Report: `pbt-out/bug_reports/encode_neon_shift_right_extra_operand.md`.

3. **encode_neon_shift_right_neg_shift_i64_trunc** — Falsifiable. Shrunk counterexample: `Imm(4294967297)` for T=8b (`get_imm as u32` → 1). Law: i64 shift outside [1, esize] must Err. Report: `pbt-out/bug_reports/encode_neon_shift_right_shift_i64_trunc.md`.

4. **encode_neon_shift_right_neg_reg_source** — Falsifiable. Shrunk counterexample: `prefix="x", n=0` (`srshr v0.8b, x0, #1`). Law: source must be Vn.T, not a bare register. Report: `pbt-out/bug_reports/encode_neon_shift_right_reg_source.md`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_shift_right_pbt) | 15 (1 KAT + 10 properties + 4 regressions) |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_shift_right_mismatched_t.md
- pbt-out/bug_reports/encode_neon_shift_right_extra_operand.md
- pbt-out/bug_reports/encode_neon_shift_right_shift_i64_trunc.md
- pbt-out/bug_reports/encode_neon_shift_right_reg_source.md

Contract-surface sweep closed after 1 round (STANDARD allowance): `coverage_gaps` had no profraw; manual arm audit of arity (`len < 3`), dest T / Q, discarded source T, `get_imm as u32`, `get_neon_reg` Operand::Reg dest+source, extra operands, 1d Reserved. Added `encode_neon_shift_right_neg_shift_i64_trunc` and `encode_neon_shift_right_neg_reg_source`.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 09:58 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 45/184 total | PBT candidates: 45 | Tested: 45 (100%) | 0 pass, 45 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 45 |
| **Tested (of PBT candidates)** | **45 / 45 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 45 / 0 |
| **Overall (tested / all functions)** | **45 / 184 (24%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 45 | 45 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 45 | 45 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 14 | 14 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 7 | 7 | 100% | covered |

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
| encode_blr | compare_branch.rs |
| encode_br | compare_branch.rs |
| encode_branch | compare_branch.rs |
| encode_cbz | compare_branch.rs |
| encode_ccmp_ccmn | compare_branch.rs |
| encode_cinc | compare_branch.rs |
| encode_cinv | compare_branch.rs |
| encode_cmn | compare_branch.rs |
| encode_cmp | compare_branch.rs |
| encode_cneg | compare_branch.rs |
| encode_csel | compare_branch.rs |
| encode_cset | compare_branch.rs |
| encode_csetm | compare_branch.rs |
| encode_csinc | compare_branch.rs |
| encode_csinv | compare_branch.rs |
| encode_csneg | compare_branch.rs |
| encode_div | data_processing.rs |
| encode_eon | data_processing.rs |
| encode_ldar_stlr | load_store.rs |
| encode_neon_across_long | neon.rs |
| encode_neon_float_cmp_zero | neon.rs |
| encode_neon_sli | neon.rs |
| encode_ldur_stur | load_store.rs |
| encode_ldxp_stxp | load_store.rs |
| encode_neon_float_three_same | neon.rs |
| encode_ldxr_stxr | load_store.rs |
| encode_logical | data_processing.rs |
| encode_madd | data_processing.rs |
| encode_movk | data_processing.rs |
| encode_movn | data_processing.rs |
| encode_movz | data_processing.rs |
| encode_neon_qshrn | neon.rs |
| encode_msub | data_processing.rs |
| encode_mul | data_processing.rs |
| encode_mvn | data_processing.rs |
| encode_neon_shift_right | neon.rs |
