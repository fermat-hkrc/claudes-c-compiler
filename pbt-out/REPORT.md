# PBT Campaign Report: encode_ret

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ret
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; coverage_gaps had no profraw — manual arm audit of empty-default / get_reg success / get_reg None / get_reg other / extra / W / SP / FP)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ret | 10 properties + 3 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **Extra operand ignored.** Failing property `encode_ret_neg_extra_operand`. Shrunk witness: n=0, which=0 — `ret x0, x1` encodes as Word(0xd65f0000) instead of Err. llvm-mc: invalid operand. Law: ARM ARM RET takes at most one Xn; README.md:14 gas-compat. Severity: medium. Report: `pbt-out/bug_reports/encode_ret_extra_operand.md`.

2. **W-form Rn accepted.** Failing property `encode_ret_neg_w_reg`. Shrunk witness: n=0 — `ret w0` encodes as Word(0xd65f0000) instead of Err. llvm-mc rejects W-form; ARM ARM Rn is Xn. Severity: medium. Report: `pbt-out/bug_reports/encode_ret_w_reg.md`.

3. **SP encoded as XZR.** Failing property `encode_ret_neg_wrong_reg_class`. Shrunk witness: which=0, n=0 — `ret sp` encodes as Word(0xd65f03e0) instead of Err. llvm-mc rejects SP; ARM ARM register 31 is XZR never SP. Severity: medium. Report: `pbt-out/bug_reports/encode_ret_sp.md`.

4. **FP/SIMD names encoded as GPRs.** Failing property `encode_ret_neg_fp_reg`. Shrunk witness: which=0, n=0 — `ret d0` encodes as Word(0xd65f0000) instead of Err. llvm-mc rejects FP Rn; ARM ARM Rn is Xn. Severity: medium. Report: `pbt-out/bug_reports/encode_ret_fp_reg.md`.

All four reproduced serially (`PBT_TEST_JOBS=1 --test-threads=1`).

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_ret_pbt) | 3 KAT + 10 properties + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/FUNCTION_INDEX.md — encode_ret marked yes
- pbt-out/COVERAGE.md — encode_ret row appended
- pbt-out/COVERAGE_STATUS.md — updated
- pbt-out/INVARIANTS.md — encode_ret section prepended
- pbt-out/bug_reports/encode_ret_extra_operand.md
- pbt-out/bug_reports/encode_ret_w_reg.md
- pbt-out/bug_reports/encode_ret_sp.md
- pbt-out/bug_reports/encode_ret_fp_reg.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 11:29 (campaign: coverage)
> Files: 7/7 scanned (100%) | Functions: 51/229 total | PBT candidates: 51 | Tested: 51 (100%) | 0 pass, 51 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 7 |
| Files scanned | 7 / 7 (100%) |
| Total functions (all files) | 229 |
| PBT candidates (from FUNCTION_INDEX) | 51 |
| **Tested (of PBT candidates)** | **51 / 51 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 51 / 0 |
| **Overall (tested / all functions)** | **51 / 229 (22%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 51 | 51 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 51 | 51 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 16 | 16 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 9 | 9 | 100% | covered |
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
| encode_neon_tbl | neon.rs |
| encode_orn | data_processing.rs |
| encode_ret | compare_branch.rs |
