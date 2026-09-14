# PBT Campaign Report: encode_bfi

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_bfi
**Tests:** 13 properties (8 passing, 5 failing) plus 6 passing KAT and 5 failing regression witnesses
**Result:** 8 passing, 5 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 strengthening round, 1 coverage_gaps sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bfi | 13 properties + 6 KAT + 5 regressions | 5 | differential (llvm-mc), algebraic.metamorphic (BFI alias of BFM; Rd/Rn fields), algebraic.invariant (ARM BFM layout), negative_error |

## Bugs Found

### 1. Extra operand ignored
- **Law:** BFI takes exactly four operands; a fifth must be Err.
- **Minimal input:** `[Reg("w0"), Reg("w0"), Imm(0), Imm(1), Reg("x0")]`
- **Expected:** Err
- **Actual:** Ok(Word) — get_reg/get_imm read only indices 0..3
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_bfi_extra_operand.md

### 2. SP/WSP encoded as ZR
- **Law:** Register 31 is WZR/XZR, not SP/WSP.
- **Minimal input:** `bfi wsp, w0, #0, #1`
- **Expected:** Err
- **Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_bfi_sp.md

### 3. Out-of-range #lsb/#width panics or encodes
- **Law:** 0 <= lsb < R and 1 <= width <= R-lsb; otherwise Err.
- **Minimal input:** `bfi w0, w0, #0, #0` (debug overflow at `width - 1`)
- **Expected:** Err
- **Actual:** panic in debug; other out-of-range values encode Ok(Word)
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/encode_bfi_lsb_width.md

### 4. Mixed W/X accepted
- **Law:** Both registers must be the same width (Wd,Wn or Xd,Xn).
- **Minimal input:** `bfi x0, w0, #0, #1`
- **Expected:** Err
- **Actual:** Ok(Word) — is_64 taken only from Rd
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_bfi_mixed_width.md

### 5. FP/SIMD registers accepted as GPR
- **Law:** Rd/Rn must be W/X (or ZR), not S/D/Q/V/H/B.
- **Minimal input:** `bfi d0, x1, #0, #1`
- **Expected:** Err
- **Actual:** Ok(Word) — parse_reg_num accepts prefix d
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_bfi_fp.md

All five reproduced serially with `PBT_TEST_JOBS=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_bfi_pbt) | 13 properties + 6 KAT + 5 regressions |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger (appended encode_bfi)
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/FUNCTION_INDEX.md — merged bitfield.rs functions
- pbt-out/INVARIANTS.md — confirmed encode_bfi invariants
- pbt-out/bug_reports/encode_bfi_extra_operand.md
- pbt-out/bug_reports/encode_bfi_sp.md
- pbt-out/bug_reports/encode_bfi_lsb_width.md
- pbt-out/bug_reports/encode_bfi_mixed_width.md
- pbt-out/bug_reports/encode_bfi_fp.md

## Sweep

Contract-surface sweep round 1/1: `coverage_gaps` had no LLVM profraw. Manual arm audit of encode_bfi (arity / extra / SP / mixed W-X / FP / lsb-width / nonreg / invalid-name / alt-spellings). Added encode_bfi_neg_invalid_name (passing). Closed: tier round spent and documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 18:04 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 76/284 total | PBT candidates: 76 | Tested: 76 (100%) | 0 pass, 76 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 284 |
| PBT candidates (from FUNCTION_INDEX) | 76 |
| **Tested (of PBT candidates)** | **76 / 76 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 76 / 0 |
| **Overall (tested / all functions)** | **76 / 284 (27%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 76 | 76 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 76 | 76 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 4 | 5 | 125% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 9 | 9 | 100% | covered |
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
