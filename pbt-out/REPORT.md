# PBT Campaign Report: encode_fcmp

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fcmp
**Tests:** 11 properties (7 passing, 4 failing) plus 8 KAT + 4 failing regression witnesses
**Result:** 7 passing, 4 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 metamorphic/differential required, 1 coverage_gaps sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fcmp | 11 properties (7 pass / 4 fail) | 4 | differential (llvm-mc), algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **One operand encoded as #0.0** — `encode_fcmp` treats `operands.len() < 2` as FCMP #0.0. Shrunk: `[Reg("s0")]` → Word(0x1e202008) (`fcmp s0, #0.0`). llvm-mc rejects `fcmp s0` ("too few operands"). Empty slice correctly Errs via get_reg. Serial reconfirm with PBT_TEST_JOBS=1. Report: `pbt-out/bug_reports/encode_fcmp_arity.md`

2. **Extra operand ignored** — a 3rd operand is dropped. Shrunk: `[Reg("s0"), Reg("s0"), Reg("s0")]` → Word(0x1e202000). llvm-mc rejects `fcmp s0, s0, s0`. FCCMP is a different mnemonic. Serial reconfirm. Report: `pbt-out/bug_reports/encode_fcmp_extra_operand.md`

3. **Mixed S/D, GPR, Q/V/B, SP encoded as FCMP** — ftype is taken only from whether operand 0 starts with `d`; no matching-type or FP-class check. Shrunk: `[Reg("s0"), Reg("d0")]` → Word(0x1e202000) (same bits as `fcmp s0, s0`). llvm-mc rejects mixed S/D, GPR, Q/V/B, and SP. Serial reconfirm. Report: `pbt-out/bug_reports/encode_fcmp_wrong_types.md`

4. **H registers use ftype=00 (single) not 11** — llvm-mc `-mattr=+fullfp16` encodes `fcmp h0, h0` as 0x1ee02000; SUT emits 0x1e202000 because `rn_name.starts_with('d')` is the only ftype check. Shrunk: `[Reg("h0"), Reg("h0")]`. Serial reconfirm. Report: `pbt-out/bug_reports/encode_fcmp_half_ftype.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/fp_scalar.rs (mod encode_fcmp_pbt) | 11 properties + 8 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist (Scan/Plan/Test/Review complete; sweep 1/1)
- pbt-out/PROPERTIES.md — 11 properties (7 passing, 4 failing)
- pbt-out/REPORT.md — this file
- pbt-out/COVERAGE.md — encode_fcmp row appended
- pbt-out/COVERAGE_STATUS.md — candidates 73, tested 73
- pbt-out/FUNCTION_INDEX.md — encode_fcmp marked yes
- pbt-out/INVARIANTS.md — encode_fcmp section prepended
- pbt-out/bug_reports/encode_fcmp_arity.md
- pbt-out/bug_reports/encode_fcmp_extra_operand.md
- pbt-out/bug_reports/encode_fcmp_wrong_types.md
- pbt-out/bug_reports/encode_fcmp_half_ftype.md

Sweep closed because the tier's one coverage_gaps-driven round was spent (tool had no LLVM profraw; manual arm audit of arity / extra / mixed S-D / GPR / QVB / SP / half / nonzero imm / nonreg / invalid-name) and every documented behavior of encode_fcmp has a property.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 17:13 (campaign: coverage)
> Files: 9/9 scanned (100%) | Functions: 73/267 total | PBT candidates: 73 | Tested: 73 (100%) | 0 pass, 73 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 9 |
| Files scanned | 9 / 9 (100%) |
| Total functions (all files) | 267 |
| PBT candidates (from FUNCTION_INDEX) | 73 |
| **Tested (of PBT candidates)** | **73 / 73 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 73 / 0 |
| **Overall (tested / all functions)** | **73 / 267 (27%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 73 | 73 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 73 | 73 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 14 | 4 | 4 | 100% | covered |
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
| encode_int_to_float | fp_scalar.rs |
| encode_fcmp | fp_scalar.rs |
