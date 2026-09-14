# PBT Campaign Report: encode_fp_1src

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fp_1src
**Tests:** 8 properties (5 passing, 3 failing) plus 7 passing KAT and 3 failing regression witnesses
**Result:** 5 passing, 3 bugs
**Effort tier:** standard (5–8 properties, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fp_1src | 8 properties (5 pass / 3 fail) | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### 1. Extra operands ignored
- **Law:** Scalar FRINT* takes exactly two registers; a third operand must be rejected.
- **Shrunk counterexample:** `[Reg("s0"), Reg("s0"), Reg("s0")]` opcode=0b001000 (`frintn s0, s0, s0`)
- **Expected:** Err
- **Actual:** Ok(Word) — get_reg only reads indices 0 and 1
- **Root cause:** No arity-upper-bound check
- **Impact:** Invalid GNU-style assembly is silently encoded
- **Severity:** medium
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced
- **Bug report:** pbt-out/bug_reports/encode_fp_1src_extra_operand.md
- **Regression test:** `test_encode_fp_1src_regression_extra_operand` (fails as witness)

### 2. Mixed S/D (and GPR/SP/QVB) encoded instead of rejected
- **Law:** FRINTN syntax is Sd,Sn or Dd,Dn (same precision). Mixed S/D, GPR, SP/WSP, and Q/V/B must Err.
- **Shrunk counterexample:** `[Reg("s0"), Reg("d0")]` opcode=0b001000 (`frintn s0, d0`)
- **Expected:** Err
- **Actual:** Ok(Word(0x1e244000)) — ftype taken only from dest; source type ignored
- **Root cause:** No matching-type / FP-class check after parse_reg_num
- **Impact:** `frintn s0, d0` becomes single-precision FRINTN s0, s0
- **Severity:** high
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced
- **Bug report:** pbt-out/bug_reports/encode_fp_1src_wrong_types.md
- **Regression test:** `test_encode_fp_1src_regression_mixed_sd` (fails as witness)

### 3. Half-precision H registers encoded as ftype=00 (single)
- **Law:** ARM ARM ftype=11 for half-precision FRINT*. llvm-mc +fullfp16 `frintn h0, h0` = 0x1ee44000.
- **Shrunk counterexample:** `[Reg("h0"), Reg("h0")]` opcode=0b001000
- **Expected:** Word(0x1ee44000)
- **Actual:** Word(0x1e244000) — `rd_name.starts_with('d')` is the only ftype check
- **Root cause:** H is treated as S
- **Impact:** Half-precision rounding silently uses the S register view
- **Severity:** high
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced
- **Bug report:** pbt-out/bug_reports/encode_fp_1src_half_ftype.md
- **Regression test:** `test_encode_fp_1src_regression_half_ftype` (fails as witness)

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/fp_scalar.rs (mod encode_fp_1src_pbt) | 8 properties + 7 KAT + 3 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row for encode_fp_1src
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/FUNCTION_INDEX.md — merged function index (encode_fp_1src now a PBT candidate)
- pbt-out/INVARIANTS.md — confirmed invariants for encode_fp_1src
- pbt-out/bug_reports/encode_fp_1src_extra_operand.md
- pbt-out/bug_reports/encode_fp_1src_wrong_types.md
- pbt-out/bug_reports/encode_fp_1src_half_ftype.md

## Sweep close-out

Tier `standard` owes exactly 1 coverage-driven round. `coverage_gaps` had no LLVM profraw in this session. Manual arm audit of encode_fp_1src covered arity, extra operand, mixed S/D, GPR/QVB/SP, half ftype, non-register kinds, invalid names, uppercase spellings, S vs D, and all 7 FRINT opcodes. Closed: tier round spent and documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 16:48 (campaign: coverage)
> Files: 9/9 scanned (100%) | Functions: 71/267 total | PBT candidates: 71 | Tested: 71 (100%) | 0 pass, 71 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 9 |
| Files scanned | 9 / 9 (100%) |
| Total functions (all files) | 267 |
| PBT candidates (from FUNCTION_INDEX) | 71 |
| **Tested (of PBT candidates)** | **71 / 71 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 71 / 0 |
| **Overall (tested / all functions)** | **71 / 267 (27%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 71 | 71 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 71 | 71 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 14 | 2 | 2 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 9 | 9 | 100% | covered |
| neon.rs | 68 | 13 | 13 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_shift | gp_integer.rs |
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
| encode_neg | pseudo.rs |
| encode_negs | data_processing.rs |
| encode_neon_shift_imm | neon.rs |
| encode_neon_tbl | neon.rs |
| encode_orn | data_processing.rs |
| encode_ret | compare_branch.rs |
| encode_sbc | data_processing.rs |
| encode_neon_shll | neon.rs |
| encode_neon_sqshrun | neon.rs |
| encode_smull | data_processing.rs |
| encode_sxth | data_processing.rs |
| encode_sxtw | data_processing.rs |
| encode_neon_shift_left_imm | neon.rs |
| encode_umaddl | data_processing.rs |
| encode_umulh | data_processing.rs |
| encode_neon_rbit | neon.rs |
| encode_umull | data_processing.rs |
| encode_uxtw | data_processing.rs |
| encode_ldaxr_stlxr | load_store.rs |
| encode_ldrsw | load_store.rs |
| encode_ldtr_sized | load_store.rs |
| encode_prfm | load_store.rs |
| encode_smulh | data_processing.rs |
| encode_fcvt_rounding | fp_scalar.rs |
| encode_fp_1src | fp_scalar.rs |
