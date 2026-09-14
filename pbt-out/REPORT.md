# PBT Campaign Report: encode_rbit

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_rbit (src/backend/arm/assembler/encoder/bitfield.rs)
**Tests:** 12 properties (8 passing, 4 failing) plus 6 passing KAT and 4 failing regression witnesses
**Result:** 8 passing properties, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; generator runs set to 1000; ≥1 metamorphic and ≥1 differential)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_rbit | 12 properties (8 pass / 4 fail) + 6 KAT + 4 regressions | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **encode_rbit ignores extra operands** — `rbit w0, w0, x0` encodes as `rbit w0, w0` (0x5ac00000). llvm-mc/gas reject a 3rd operand. Law: RBIT takes exactly two registers. Shrunk: `[Reg("w0"), Reg("w0"), Reg("x0")]`. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_rbit_extra_operand.md`

2. **encode_rbit accepts SP/WSP as a GPR** — `rbit wsp, w0` encodes as `rbit wzr, w0` (0x5ac0001f). llvm-mc rejects SP/WSP. Law: ARM register 31 is ZR not SP. Shrunk: which=0, sp=wsp, other=0. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_rbit_sp.md`

3. **encode_rbit accepts mixed W/X register widths** — `rbit x0, w0` encodes as `rbit x0, x0` (0xdac00000); sf is taken from Rd only. llvm-mc rejects mixed width. Law: matching W/W or X/X. Shrunk: rd=0, rn=0, rd64=true, rn64=false. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_rbit_mixed_width.md`

4. **encode_rbit accepts FP/SIMD registers as scalar operands** — `rbit d0, x1` encodes as `rbit w0, w1` (0x5ac00020). llvm-mc rejects FP prefixes. Law: scalar RBIT operands are GPRs only. Shrunk: which=0, prefix="d", n=0. Serial reconfirm with PBT_TEST_JOBS=1. Severity: medium. Report: `pbt-out/bug_reports/encode_rbit_fp.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_rbit_pbt) | 12 properties + 6 KAT + 4 regressions |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row for encode_rbit
- pbt-out/COVERAGE_STATUS.md — campaign coverage stats
- pbt-out/FUNCTION_INDEX.md — encode_rbit marked yes
- pbt-out/INVARIANTS.md — confirmed encode_rbit invariants
- pbt-out/bug_reports/encode_rbit_extra_operand.md
- pbt-out/bug_reports/encode_rbit_sp.md
- pbt-out/bug_reports/encode_rbit_mixed_width.md
- pbt-out/bug_reports/encode_rbit_fp.md

Sweep round 1/1 closed: `coverage_gaps` had no LLVM profraw; manual arm audit added `encode_rbit_diff_alt_spellings`, `encode_rbit_neg_nonreg`, `encode_rbit_neg_invalid_name`, `encode_rbit_diff_valid_neon` (all passing). Documented contract surface covered; tier round spent.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 20:09 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 84/284 total | PBT candidates: 84 | Tested: 84 (100%) | 0 pass, 84 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 284 |
| PBT candidates (from FUNCTION_INDEX) | 84 |
| **Tested (of PBT candidates)** | **84 / 84 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 84 / 0 |
| **Overall (tested / all functions)** | **84 / 284 (30%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 84 | 84 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 84 | 84 | 0 | 100% |

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
| encode_rbit | bitfield.rs |
