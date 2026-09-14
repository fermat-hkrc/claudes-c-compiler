# PBT Campaign Report: encode_neon_sqshrun

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_sqshrun
**Tests:** 11 properties + 2 KAT + 5 regression witnesses
**Result:** 6 passing, 5 bugs
**Effort tier:** standard (1 coverage-driven sweep round; ≥1000 generator runs; ≥1 metamorphic/differential required)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_sqshrun | 11 properties (6 pass / 5 fail) + 2 KAT + 5 failing regression witnesses | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **Shift above dest_esize is accepted and silently recoded.** `sqshrun v0.8b, v0.8h, #9` must Err (ARM/llvm-mc range [1, 8]); SUT checks against source size 16 and encodes #9 as #1 via `immh = (immhb >> 3) | immh_base`. Law: encode_neon_sqshrun_neg_shift_oob. Minimal input: Imm(9), Ta=8h. Severity: high. Report: `pbt-out/bug_reports/encode_neon_sqshrun_shift_oob.md`. Serial reconfirmed (PBT_TEST_JOBS=1).

2. **Destination arrangement Tb is ignored.** `sqshrun v0.4h, v0.8h, #1` must Err (Tb must be 8b); SUT binds `_arr_d` and never consults it. Law: encode_neon_sqshrun_neg_dest_tb. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_sqshrun_mismatched_dest_tb.md`. Serial reconfirmed.

3. **GPR/scalar-FP dest encodes as NEON Rd.** `sqshrun x0, v0.8h, #1` must Err; `get_neon_reg` accepts `Operand::Reg` and `parse_reg_num("x0")` returns 0. Law: encode_neon_sqshrun_neg_gpr_dest. Severity: high. Report: `pbt-out/bug_reports/encode_neon_sqshrun_gpr_dest.md`. Serial reconfirmed.

4. **Operands beyond index 2 are ignored.** `sqshrun v0.8b, v0.8h, #1, v0.8b` must Err (exactly 3 operands); SUT only checks `len < 3`. Law: encode_neon_sqshrun_neg_extra_operand. Severity: medium. Report: `pbt-out/bug_reports/encode_neon_sqshrun_extra_operand.md`. Serial reconfirmed.

5. **i64 shift is truncated with `as u32`.** Imm(4294967297) (= 2^32+1) encodes as shift #1. Law: encode_neon_sqshrun_neg_shift_i64_trunc (coverage sweep). Severity: medium. Report: `pbt-out/bug_reports/encode_neon_sqshrun_shift_i64_trunc.md`. Serial reconfirmed.

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_sqshrun_pbt) | 11 properties + 2 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_sqshrun_shift_oob.md
- pbt-out/bug_reports/encode_neon_sqshrun_mismatched_dest_tb.md
- pbt-out/bug_reports/encode_neon_sqshrun_gpr_dest.md
- pbt-out/bug_reports/encode_neon_sqshrun_extra_operand.md
- pbt-out/bug_reports/encode_neon_sqshrun_shift_i64_trunc.md

## Sweep close-out

Contract-surface sweep: 1 round (standard tier). `coverage_gaps` had no LLVM profraw; manual arm audit of arity < 3, unsupported Ta, Imm vs other, 8h/4s/2d, shift==0 / shift > source_esize, is_high, is_rounding, `*v as u32`, get_neon_reg Reg dest+source, extra operands, mismatched Tb. Added `neg_shift_i64_trunc` (failing, bug 5) and `neg_reg_source` (passing). Closed because the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 12:35 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 55/253 total | PBT candidates: 55 | Tested: 55 (100%) | 0 pass, 55 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 55 |
| **Tested (of PBT candidates)** | **55 / 55 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 55 / 0 |
| **Overall (tested / all functions)** | **55 / 253 (22%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 55 | 55 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 55 | 55 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 17 | 17 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 11 | 11 | 100% | covered |
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
