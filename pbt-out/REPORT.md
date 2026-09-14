# PBT Campaign Report: encode_neon_shift_imm

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_shift_imm (src/backend/arm/assembler/encoder/neon.rs)
**Tests:** 9 properties (4 passing, 5 failing) plus 1 passing KAT and 5 failing regression witnesses
**Result:** 4 passing properties, 5 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-gaps round; first batch did not all pass so no extra strengthening round beyond the sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_shift_imm | 9 properties + 1 KAT + 5 regression | 5 | differential (llvm-mc), algebraic.metamorphic (Q XOR), algebraic.invariant (ARM fields), negative_error |

## Bugs Found

1. **encode_neon_shift_imm_neg_shift_oob.** Law: USHR shift ∈ [1, esize]. Shrunk counterexample: rd=0, rn=0, t=8b, shift=-1 — debug panic `attempt to subtract with overflow` at neon.rs:390. Related: shift=0 and shift=9 return Ok(Word) via wrap/mask (immh often 0000). Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_shift_imm_shift_oob.md`
2. **encode_neon_shift_imm_neg_extra_operand.** Law: USHR is Vd.T, Vn.T, #shift only. Shrunk counterexample: rd=0, rn=0, extra=0, t=8b, shift=1 — `ushr v0.8b, v0.8b, #1, v0.8b` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_shift_imm_extra_operand.md`
3. **encode_neon_shift_imm_neg_mismatched_t.** Law: dest T equals source T. Shrunk counterexample: rd=0, rn=0, td=8b, ts=16b, shift=1 — `ushr v0.8b, v0.16b, #1` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_shift_imm_mismatched_t.md`
4. **encode_neon_shift_imm_neg_shift_i64_trunc.** Law: i64 Imm outside [1, esize] must Err; must not fold modulo 2^32. Shrunk counterexample: Imm(4294967297) encodes as #1. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_shift_imm_shift_i64_trunc.md`
5. **encode_neon_shift_imm_neg_reg_source.** Law: source is Vn.T, not a bare GPR/FP/V. Shrunk counterexample: `ushr v0.8b, x0, #1` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_shift_imm_reg_source.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_shift_imm_pbt) | 1 KAT + 9 proptest properties + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_shift_imm_shift_oob.md
- pbt-out/bug_reports/encode_neon_shift_imm_extra_operand.md
- pbt-out/bug_reports/encode_neon_shift_imm_mismatched_t.md
- pbt-out/bug_reports/encode_neon_shift_imm_shift_i64_trunc.md
- pbt-out/bug_reports/encode_neon_shift_imm_reg_source.md

## Contract-surface sweep

Round 1 of 1 (standard). `coverage_gaps` had no LLVM profraw. Manual arm audit of encode_neon_shift_imm: `shift as u32` (i64 truncation) and get_neon_reg Operand::Reg source were untested; added encode_neon_shift_imm_neg_shift_i64_trunc and encode_neon_shift_imm_neg_reg_source (both failing; bugs 4 and 5). Remaining branches (arity, 1d, GPR dest, invalid names, Q/U/immh fields, valid T×shift) were already reached. Closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 10:47 (campaign: coverage)
> Files: 7/7 scanned (100%) | Functions: 48/229 total | PBT candidates: 48 | Tested: 48 (100%) | 0 pass, 48 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 7 |
| Files scanned | 7 / 7 (100%) |
| Total functions (all files) | 229 |
| PBT candidates (from FUNCTION_INDEX) | 48 |
| **Tested (of PBT candidates)** | **48 / 48 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 48 / 0 |
| **Overall (tested / all functions)** | **48 / 229 (21%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 48 | 48 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 48 | 48 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 15 | 15 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 8 | 8 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

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
| encode_neg | pseudo.rs |
| encode_negs | data_processing.rs |
| encode_neon_shift_imm | neon.rs |
