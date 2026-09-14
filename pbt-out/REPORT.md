# PBT Campaign Report: encode_rev

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_rev
**Tests:** 11 properties (7 passing, 4 failing) plus 6 passing KAT and 4 failing regression witnesses
**Result:** 7 passing, 4 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 strengthening/sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_rev | 11 properties + 6 KAT + 4 regressions | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### encode_rev ignores extra operands
- **Failing property:** encode_rev_neg_extra_operand (negative_error)
- **Shrunk counterexample:** is_64 = false, rd = 0, rn = 0, extra = Reg("x0")
- **Falsifiable:** ∀ extra. encode_rev([w{rd}, w{rn}, extra]) is Err — fails on extra=Reg("x0")
- **Expected:** Err (llvm-mc rejects a 3rd operand)
- **Actual:** Ok(Word) — get_reg only reads indices 0 and 1
- **reproduce:** PBT_TEST_JOBS=1 cargo test --lib encode_rev_neg_extra_operand
- **Bug report:** pbt-out/bug_reports/encode_rev_extra_operand.md

### encode_rev accepts SP/WSP as register 31
- **Failing property:** encode_rev_neg_sp (negative_error)
- **Shrunk counterexample:** which = 0, sp64 = false, is_64 = false, other = 0 (wsp, w0)
- **Falsifiable:** ∀ which,sp. encode_rev(ops with SP) is Err — fails on wsp at slot 0
- **Expected:** Err (ARM ARM register 31 is ZR not SP)
- **Actual:** Ok(Word)
- **reproduce:** PBT_TEST_JOBS=1 cargo test --lib encode_rev_neg_sp
- **Bug report:** pbt-out/bug_reports/encode_rev_sp.md

### encode_rev accepts mixed W/X widths
- **Failing property:** encode_rev_neg_mixed_width (negative_error)
- **Shrunk counterexample:** rd = 0, rn = 0, rd64 = true, rn64 = false (x0, w0)
- **Falsifiable:** ∀ rd,rn,rd64≠rn64. encode_rev mixed W/X is Err — fails on x0, w0
- **Expected:** Err (llvm-mc rejects mixed widths)
- **Actual:** Ok(Word) — sf/opc taken from Rd only
- **reproduce:** PBT_TEST_JOBS=1 cargo test --lib encode_rev_neg_mixed_width
- **Bug report:** pbt-out/bug_reports/encode_rev_mixed_width.md

### encode_rev accepts FP/SIMD registers as GPR
- **Failing property:** encode_rev_neg_fp (negative_error)
- **Shrunk counterexample:** which = 0, prefix = "d", n = 0 (d0, x1)
- **Falsifiable:** ∀ which,prefix,n. encode_rev FP prefix is Err — fails on d0 at slot 0
- **Expected:** Err (llvm-mc rejects d/s/q/v/h/b)
- **Actual:** Ok(Word) — parse_reg_num accepts those prefixes
- **reproduce:** PBT_TEST_JOBS=1 cargo test --lib encode_rev_neg_fp
- **Bug report:** pbt-out/bug_reports/encode_rev_fp.md

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_rev_pbt) | 11 properties + 6 KAT + 4 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_rev_extra_operand.md
- pbt-out/bug_reports/encode_rev_sp.md
- pbt-out/bug_reports/encode_rev_mixed_width.md
- pbt-out/bug_reports/encode_rev_fp.md

Sweep close: coverage_gaps had no LLVM profraw; manual ARM audit added encode_rev_diff_alt_spellings, encode_rev_neg_nonreg, encode_rev_neg_invalid_name (all passing). Tier round spent and documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 20:46 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 87/289 total | PBT candidates: 87 | Tested: 87 (100%) | 0 pass, 87 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 87 |
| **Tested (of PBT candidates)** | **87 / 87 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 87 / 0 |
| **Overall (tested / all functions)** | **87 / 289 (30%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 87 | 87 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 87 | 87 | 0 | 100% |

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
| encode_rev | bitfield.rs |
| encode_rev16 | bitfield.rs |
| encode_rev32 | bitfield.rs |
