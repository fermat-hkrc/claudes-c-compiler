# PBT Campaign Report: encode_fcvt_precision

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fcvt_precision
**Tests:** 9 properties (6 passing, 3 failing) plus 8 KAT + 6 failing regression witnesses
**Result:** 6 passing, 3 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 metamorphic/differential required, 1 coverage_gaps sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fcvt_precision | 9 properties (6 pass / 3 fail) | 3 | differential (llvm-mc), algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **Extra operand ignored** — a 3rd operand is dropped. Shrunk: `[Reg("s0"), Reg("d0"), Reg("s0")]` → Word(0x1e624000) (`fcvt s0, d0`). llvm-mc rejects `fcvt d0, s1, s2` ("invalid operand"). Serial reconfirm with PBT_TEST_JOBS=1. Report: `pbt-out/bug_reports/encode_fcvt_precision_extra_operand.md`

2. **Same-precision FCVT encoded (unallocated)** — ARM ARM FCVT with ftype==opc is unallocated. Shrunk: `[Reg("s0"), Reg("s0")]` → Word(0x1e224000). llvm-mc rejects `fcvt s0, s1` / `d,d` / `h,h`. The word would SIGILL on hardware. Serial reconfirm. Report: `pbt-out/bug_reports/encode_fcvt_precision_same_precision.md`

3. **SP treated as an S register** — parse_reg_num maps `"sp"` to 31 and dest/src first char `'s'` selects S precision, so `fcvt sp, d0` encodes as `fcvt s31, d0`. llvm-mc rejects SP in either slot. Property shrink `(sp, s0)`; dest and src regressions. Serial reconfirm. Report: `pbt-out/bug_reports/encode_fcvt_precision_sp_as_s.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/fp_scalar.rs (mod encode_fcvt_precision_pbt) | 9 properties + 8 KAT + 6 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist (Scan/Plan/Test/Review complete; sweep 1/1)
- pbt-out/PROPERTIES.md — 9 properties (6 passing, 3 failing)
- pbt-out/REPORT.md — this file
- pbt-out/COVERAGE.md — encode_fcvt_precision row appended
- pbt-out/COVERAGE_STATUS.md — candidates 74, tested 74
- pbt-out/FUNCTION_INDEX.md — encode_fcvt_precision marked yes
- pbt-out/INVARIANTS.md — encode_fcvt_precision section prepended
- pbt-out/bug_reports/encode_fcvt_precision_extra_operand.md
- pbt-out/bug_reports/encode_fcvt_precision_same_precision.md
- pbt-out/bug_reports/encode_fcvt_precision_sp_as_s.md

Sweep closed because the tier's one coverage_gaps-driven round was spent (tool had no LLVM profraw; manual arm audit of arity / extra / same-precision S,D,H / GPR / QVB / WSP / SP dest+src / half ftype / nonreg / invalid-name) and every documented behavior of encode_fcvt_precision has a property.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 17:27 (campaign: coverage)
> Files: 9/9 scanned (100%) | Functions: 74/267 total | PBT candidates: 74 | Tested: 74 (100%) | 0 pass, 74 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 9 |
| Files scanned | 9 / 9 (100%) |
| Total functions (all files) | 267 |
| PBT candidates (from FUNCTION_INDEX) | 74 |
| **Tested (of PBT candidates)** | **74 / 74 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 74 / 0 |
| **Overall (tested / all functions)** | **74 / 267 (28%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 74 | 74 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 74 | 74 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 14 | 5 | 5 | 100% | covered |
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
| encode_fcvt_precision | fp_scalar.rs |
