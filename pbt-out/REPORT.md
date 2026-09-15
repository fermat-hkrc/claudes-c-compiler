# PBT Campaign Report: encode_neon_ldnr

## Summary

**Date:** 2026-09-15
**Repository:** claudes-c-compiler
**Modules tested:** encode_neon_ldnr
**Tests:** 15 properties + 1 KAT + 12 regression witnesses
**Result:** 5 passing properties, 10 failing properties, 1 passing KAT, 12 failing regressions; 7 bugs
**Effort tier:** standard (1 strengthening round; 1 contract-surface sweep; ≥1000 generator runs)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_ldnr | 15 properties (5 passing, 10 failing) + KAT + 12 regressions | 7 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **LD2R/LD4R S bit at bit 12 not bit 21**
   - Law: AdvSIMD replicate encoding (gas/llvm-mc) places S at bit 21 (1 for LD2R/LD4R) with bit 12 clear.
   - Shrunk: n=4, t="8b", rt=0, rn=0 — `ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x0]` SUT 0x0d40f000 vs llvm-mc 0x0d60e000. Also KAT `ld2r {v0.8b, v1.8b}, [x1]` SUT 0x0d40d020 vs 0x0d60c020.
   - Properties: encode_neon_ldnr_diff_no_offset_llvm_mc, encode_neon_ldnr_diff_post_imm_llvm_mc, encode_neon_ldnr_arm_fields, encode_neon_ldnr_diff_alt_spellings.
   - Severity: high. `pbt-out/bug_reports/encode_neon_ldnr_s_bit.md`

2. **Surplus operand ignored**
   - Law: llvm-mc/gas reject a surplus operand after a complete ldNr (README.md:12).
   - Shrunk: n=2, t="8b", rt=0, rn=0, extra_kind=0 (Cond "eq") — encode returns Ok.
   - Property: encode_neon_ldnr_neg_extra.
   - Severity: medium. `pbt-out/bug_reports/encode_neon_ldnr_extra_operand.md`

3. **Invalid base (W / XZR / x31 / FP) accepted**
   - Law: ARM/gas/llvm-mc require base Xn|SP.
   - Shrunk: n=2, t="8b", rt=0, base="w0" — encode returns Ok. Same property domain also encodes xzr, x31, d0 (regressions).
   - Property: encode_neon_ldnr_neg_invalid_base.
   - Severity: medium. `pbt-out/bug_reports/encode_neon_ldnr_w_base.md` (also encode_neon_ldnr_xzr_base.md, encode_neon_ldnr_fp_base.md)

4. **Register post-index `[Xn], Xm` ignored**
   - Law: README.md:235 post-index; llvm-mc `ld2r {v0.8b, v1.8b}, [x1], x2` = 0x0de2c020.
   - Shrunk: n=2, t="8b", rt=0, rn=0, rm=0 — `ld2r {v0.8b, v1.8b}, [x0], x0` SUT 0x0d40d000 vs llvm-mc 0x0de0c000.
   - Property: encode_neon_ldnr_diff_post_reg_llvm_mc.
   - Severity: high. `pbt-out/bug_reports/encode_neon_ldnr_reg_post.md`

5. **Illegal post-index immediate accepted**
   - Law: post-index #imm must equal n*esize.
   - Shrunk: n=2, t="8b", rt=0, rn=0, imm=-1 — encode returns Ok.
   - Property: encode_neon_ldnr_neg_bad_post_imm.
   - Severity: medium. `pbt-out/bug_reports/encode_neon_ldnr_bad_post_imm.md`

6. **Non-consecutive register list accepted**
   - Law: gas/llvm-mc require consecutive wrapping same-T lists.
   - Shrunk: n=2, t="8b", rt=0, skip=1, rn=0 — `{v0.8b, v2.8b}` encodes.
   - Property: encode_neon_ldnr_neg_nonconsecutive.
   - Severity: medium. `pbt-out/bug_reports/encode_neon_ldnr_nonconsecutive.md`

7. **`[Xn, #imm]` accepted as no-offset**
   - Law: only `[Xn]` or post-index is valid.
   - Shrunk: n=2, t="8b", rt=0, rn=0, off=-1 — encode returns Ok.
   - Property: encode_neon_ldnr_neg_mem_offset.
   - Severity: medium. `pbt-out/bug_reports/encode_neon_ldnr_mem_offset.md`

Serial reconfirmation: all failures reproduced with `cargo test --lib encode_neon_ldnr -- --test-threads=1`.

## Design Caveats (if any)

- Empty `Operand::RegList(vec![])` panics at `regs[0]` (neon.rs:1532). Not a filed bug: `encode_neon_ldnr` is `pub(crate)`; the only in-tree caller is encoder dispatch, and the parser rejects empty lists. Doc evidence: parser.rs:2069-2071 `if regs.is_empty() { return Err("empty register list".to_string()); }`
- Mixed arrangements (`{v0.8b, v1.16b}`) are the same list-validation gap as bug 6 (only `regs[0]` and `len` are read). Regression `test_encode_neon_ldnr_regression_mixed_arr` fails; not a separate shrunk property.
- `num_structs` not in {1,2,3,4} returns Err (neon.rs:1554). Dispatch only passes 2/3/4.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_ldnr_pbt) | 15 properties + 1 KAT + 12 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_ldnr_s_bit.md
- pbt-out/bug_reports/encode_neon_ldnr_extra_operand.md
- pbt-out/bug_reports/encode_neon_ldnr_w_base.md
- pbt-out/bug_reports/encode_neon_ldnr_xzr_base.md
- pbt-out/bug_reports/encode_neon_ldnr_fp_base.md
- pbt-out/bug_reports/encode_neon_ldnr_reg_post.md
- pbt-out/bug_reports/encode_neon_ldnr_bad_post_imm.md
- pbt-out/bug_reports/encode_neon_ldnr_nonconsecutive.md
- pbt-out/bug_reports/encode_neon_ldnr_mem_offset.md

Sweep: `coverage_gaps` had no LLVM profraw; manual arm audit added mixed-arrangement regression and LD3R-only differential (1000 llvm-mc cases). Closed: tier round spent.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-15 00:33 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 102/289 total | PBT candidates: 102 | Tested: 102 (100%) | 0 pass, 102 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 102 |
| **Tested (of PBT candidates)** | **102 / 102 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 102 / 0 |
| **Overall (tested / all functions)** | **102 / 289 (35%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 102 | 102 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 102 | 102 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 11 | 11 | 100% | covered |
| neon.rs | 68 | 17 | 17 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_ubfx | bitfield.rs |
| encode_ubfm | bitfield.rs |
| encode_sbfx | bitfield.rs |
| encode_sbfm | bitfield.rs |
| encode_sbfiz | bitfield.rs |
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
| encode_ubfiz | bitfield.rs |
| encode_bfm | bitfield.rs |
| encode_neon_float_two_misc | neon.rs |
| encode_fabs | fp_scalar.rs |
| encode_fmadd_fmsub | fp_scalar.rs |
| encode_fneg | fp_scalar.rs |
| encode_fsqrt | fp_scalar.rs |
| encode_neon_dup | neon.rs |
| encode_ldrs | load_store.rs |
| encode_neon_ldnr | neon.rs |
