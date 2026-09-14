# PBT Campaign Report: encode_fp_arith

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fp_arith (src/backend/arm/assembler/encoder/fp_scalar.rs)
**Tests:** 9 properties (6 passing, 3 failing) plus 8 passing KAT and 5 failing regression witnesses
**Result:** 6 passing properties, 3 bugs
**Effort tier:** standard (1 coverage-driven sweep round; generator runs set to 1000; ≥1 metamorphic and ≥1 differential)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fp_arith | 9 properties (6 pass / 3 fail) + 8 KAT + 5 regressions | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **encode_fp_arith ignores extra operands** — `fadd s0, s0, s0, s0` encodes as the 3-operand form. llvm-mc/gas reject a 4th operand. Law: scalar FP 2-source takes exactly three registers. Shrunk: `[Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0")]` opcode=0b0010. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_fp_arith_extra_operand.md`

2. **encode_fp_arith encodes mixed S/D (and GPR/SP/QVB)** — `fadd d0, s0, s0` encodes using dest ftype only; `fadd x0, s1, s2` and `fadd sp, s1, s2` also encode. llvm-mc/gas reject. Law: matching Sd,Sn,Sm or Dd,Dn,Dm. Shrunk: dest=`d0`, src=`s0`, src2=`s0`. Serial reconfirm with PBT_TEST_JOBS=1. Severity: high. Report: `pbt-out/bug_reports/encode_fp_arith_wrong_types.md`

3. **encode_fp_arith encodes H registers as ftype=00 (single)** — `fmul h0, h0, h0` yields 0x1e200800 (S) instead of llvm-mc fp16 0x1ee00800 (H). Law: ARM ftype=11 for half. Root cause: `rd_name.starts_with('d')` is the only ftype check. Serial reconfirm with PBT_TEST_JOBS=1. Severity: high. Report: `pbt-out/bug_reports/encode_fp_arith_half_ftype.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/fp_scalar.rs (mod encode_fp_arith_pbt) | 9 properties + 8 KAT + 5 regressions |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row for encode_fp_arith
- pbt-out/COVERAGE_STATUS.md — campaign coverage stats
- pbt-out/FUNCTION_INDEX.md — encode_fp_arith marked yes
- pbt-out/INVARIANTS.md — confirmed encode_fp_arith invariants
- pbt-out/bug_reports/encode_fp_arith_extra_operand.md
- pbt-out/bug_reports/encode_fp_arith_wrong_types.md
- pbt-out/bug_reports/encode_fp_arith_half_ftype.md

Sweep round 1/1 closed: `coverage_gaps` had no LLVM profraw; manual arm audit added `encode_fp_arith_neg_invalid_name` (passing). Documented contract surface covered; tier round spent.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 19:54 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 83/284 total | PBT candidates: 83 | Tested: 83 (100%) | 0 pass, 83 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 284 |
| PBT candidates (from FUNCTION_INDEX) | 83 |
| **Tested (of PBT candidates)** | **83 / 83 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 83 / 0 |
| **Overall (tested / all functions)** | **83 / 284 (29%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 83 | 83 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 83 | 83 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 6 | 7 | 117% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 10 | 10 | 100% | covered |
| neon.rs | 68 | 14 | 14 | 100% | covered |
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
| encode_neon_aes | neon.rs |
| encode_bfi | bitfield.rs |
| encode_bfxil | bitfield.rs |
| encode_cas | load_store.rs |
| encode_cls | bitfield.rs |
| encode_clz | bitfield.rs |
| encode_extr | bitfield.rs |
| encode_fmov | fp_scalar.rs |
| encode_fp_arith | fp_scalar.rs |
