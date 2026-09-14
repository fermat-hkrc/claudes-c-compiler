# PBT Campaign Report: encode_smulh

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_smulh
**Tests:** 13 properties (plus 4 KAT + 5 regression witnesses)
**Result:** 8 passing, 5 failing properties, 4 unique SUT bugs (wzr is the register-31 case of wrong-width)
**Effort tier:** standard (1 strengthening round + 1 contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_smulh | 13 properties (8 passing, 5 failing) | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_smulh ignores extra operands.** Law: SMULH takes exactly three registers. Counterexample: `[Reg("x0"), Reg("x0"), Reg("x0"), Reg("x0")]`. Expected Err; actual Ok(Word) because get_reg only reads indices 0..2. Severity: medium. Root cause: no operands.len() check. Impact: invalid GNU-style assembly is silently encoded. Fix: reject operands.len() != 3. Report: pbt-out/bug_reports/encode_smulh_extra_operand.md

2. **encode_smulh accepts 32-bit W registers (including WZR).** Law: SMULH is 64-bit only (Xd, Xn, Xm). Counterexample: `smulh w0, w0, w0`; sweep also `smulh wzr, x0, x0`. Expected Err; actual Ok(Word) because is_64 from get_reg is discarded. Severity: medium. Impact: W-form assembly that llvm-mc rejects is encoded as the X-form with the same numbers. Fix: require is_64 on all three registers. Report: pbt-out/bug_reports/encode_smulh_wrong_width.md

3. **encode_smulh treats SP/WSP as XZR.** Law: register 31 is XZR, not SP. Counterexample: `smulh wsp, x0, x0`. Expected Err; actual Ok(Word) because parse_reg_num maps sp/wsp to 31. Severity: medium. Impact: SP operands encode as XZR. Fix: reject sp/wsp (or require XZR/Xn). Report: pbt-out/bug_reports/encode_smulh_sp_as_zr.md

4. **encode_smulh accepts FP/SIMD registers as GPRs.** Law: operands must be 64-bit GPRs. Counterexample: `smulh d0, x1, x2`. Expected Err; actual Ok(Word) because parse_reg_num accepts d/s/q/v/h/b and encode_smulh never calls is_fp_reg. Severity: medium. Impact: FP names encode using the numeric suffix. Fix: reject FP prefixes. Report: pbt-out/bug_reports/encode_smulh_fp_as_gpr.md

Serial reconfirm: all four reproduced with `PBT_TEST_JOBS=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_smulh_pbt) | 13 properties + 4 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row for encode_smulh
- pbt-out/COVERAGE_STATUS.md — campaign coverage stats
- pbt-out/FUNCTION_INDEX.md — encode_smulh marked as PBT candidate
- pbt-out/INVARIANTS.md — confirmed encode_smulh invariants
- pbt-out/bug_reports/encode_smulh_extra_operand.md
- pbt-out/bug_reports/encode_smulh_wrong_width.md
- pbt-out/bug_reports/encode_smulh_sp_as_zr.md
- pbt-out/bug_reports/encode_smulh_fp_as_gpr.md
- pbt-out/bug_reports/encode_smulh_wzr.md

Contract-surface sweep closed: coverage_gaps had no LLVM profraw; manual arm audit added encode_smulh_neg_wzr (failing, same wrong-width bug). Tier round spent; documented SMULH surface covered (arity / extra / W-width / WZR / SP / FP / nonreg / invalid-name / alt-spellings / field independence / llvm-mc differential).

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 16:17 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 69/253 total | PBT candidates: 69 | Tested: 69 (100%) | 0 pass, 69 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 69 |
| **Tested (of PBT candidates)** | **69 / 69 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 69 / 0 |
| **Overall (tested / all functions)** | **69 / 253 (27%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 69 | 69 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 69 | 69 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
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
