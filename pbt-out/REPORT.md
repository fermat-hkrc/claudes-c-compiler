# PBT Campaign Report: encode_sbc

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_sbc
**Tests:** 13
**Result:** 9 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; coverage_gaps had no profraw — manual arm audit of get_reg success / None / other / extra / mixed width / SP / FP / lr)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_sbc | 13 properties + 3 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **Extra shift operand ignored.** Failing property `encode_sbc_neg_extra_shift`. Shrunk witness: rd=0, rn=0, rm=0, is_64=false, kind="lsl", amt=0 — `sbc w0, w0, w0, lsl #0` encodes as Word(0x5a000000) instead of Err. llvm-mc: invalid operand. Law: ARM ARM Add/subtract (with carry) has no shift field (bits 15:10 fixed 000000); README.md:14 gas-compat. Severity: medium. Report: `pbt-out/bug_reports/encode_sbc_extra_shift_ignored.md`.

2. **Mixed X/W widths accepted.** Failing property `encode_sbc_neg_mixed_width`. Shrunk witness: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true — `sbc w0, w0, x0` encodes as Word(0x5a000000) instead of Err. llvm-mc rejects mixed width; ARM ARM Rd/Rn/Rm same width; sf is taken only from Rd. Severity: medium. Report: `pbt-out/bug_reports/encode_sbc_mixed_width.md`.

3. **SP/WSP encoded as ZR.** Failing property `encode_sbc_neg_sp`. Shrunk witness: which=0, is_64=false, a=0, b=0 — `sbc wsp, w0, w0` encodes as Word(0x5a00001f) instead of Err. llvm-mc rejects SP; ARM ARM register 31 is XZR/WZR never SP. Severity: medium. Report: `pbt-out/bug_reports/encode_sbc_sp_as_zr.md`.

4. **FP/SIMD names encoded as GPRs.** Failing property `encode_sbc_neg_fp_reg`. Shrunk witness: which=0, prefix="d", n=0 — `sbc d0, x1, x2` encodes as Word(0x5a020020) instead of Err. llvm-mc rejects FP operands; ARM ARM Rd/Rn/Rm are GPRs. Severity: medium. Report: `pbt-out/bug_reports/encode_sbc_fp_reg.md`.

All four reproduced serially (`PBT_TEST_JOBS=1 --test-threads=1`).

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_sbc_pbt) | 3 KAT + 13 properties + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/FUNCTION_INDEX.md — encode_sbc marked yes
- pbt-out/COVERAGE.md — encode_sbc row appended
- pbt-out/COVERAGE_STATUS.md — updated
- pbt-out/INVARIANTS.md — encode_sbc section prepended
- pbt-out/bug_reports/encode_sbc_extra_shift_ignored.md
- pbt-out/bug_reports/encode_sbc_mixed_width.md
- pbt-out/bug_reports/encode_sbc_sp_as_zr.md
- pbt-out/bug_reports/encode_sbc_fp_reg.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 11:43 (campaign: coverage)
> Files: 7/7 scanned (100%) | Functions: 52/229 total | PBT candidates: 52 | Tested: 52 (100%) | 0 pass, 52 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 7 |
| Files scanned | 7 / 7 (100%) |
| Total functions (all files) | 229 |
| PBT candidates (from FUNCTION_INDEX) | 52 |
| **Tested (of PBT candidates)** | **52 / 52 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 52 / 0 |
| **Overall (tested / all functions)** | **52 / 229 (23%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 52 | 52 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 52 | 52 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 17 | 17 | 100% | covered |
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
| encode_sbc | data_processing.rs |
