# PBT Campaign Report: encode_prfm

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_prfm
**Tests:** 11 properties (plus 6 KAT + 9 regression witnesses)
**Result:** 6 passing properties, 5 failing properties, 7 bugs
**Effort tier:** standard (5–8 properties, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_prfm | 6 passing / 5 failing properties (5 passing KAT, 1 failing KAT, 9 failing regressions) | 7 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

1. **PRFM (register) wrong opcode bits** — `encode_prfm` uses `(0b10 << 23)` instead of `(0b10 << 22)`. `prfm pldl1keep, [x0, x1]` encodes as 0xF9216800 vs llvm-mc/ARM ARM 0xF8A16800.
   - Law: ARM ARM / load_store.rs:764 `11 111 0 00 10 1 Rm option S 10 Rn Rt`
   - Minimal input: `prfm pldl1keep, [x0, x1]` (also shrunk `[x0, x0]`)
   - Expected: 0xF8A16800 — Actual: 0xF9216800
   - Severity: high
   - Report: pbt-out/bug_reports/encode_prfm_regoff_encoding.md

2. **Extra operands ignored** — `operands.len() < 2` does not reject a third operand. `prfm #0, [x0]` plus `Reg("x2")` encodes as 0xF9800000.
   - Expected: Err — Actual: Ok(Word)
   - Severity: medium
   - Report: pbt-out/bug_reports/encode_prfm_extra_operand.md

3. **W/WSP base accepted** — `prfm #0, [w0]` encodes as `[x0]`; `[wsp]` as `[sp]`.
   - Expected: Err — Actual: Ok(Word)
   - Severity: medium
   - Report: pbt-out/bug_reports/encode_prfm_w_base.md

4. **XZR/x31 base accepted** — `prfm #0, [xzr]` and `[x31]` encode as `[sp]`.
   - Expected: Err — Actual: Ok(Word)
   - Severity: medium
   - Report: pbt-out/bug_reports/encode_prfm_xzr_base.md

5. **FP/SIMD base accepted** — `prfm #0, [d0]` encodes as `[x0]`.
   - Expected: Err — Actual: Ok(Word)
   - Severity: medium
   - Report: pbt-out/bug_reports/encode_prfm_fp_base.md

6. **Bare W-index accepted** — `prfm pldl1keep, [x0, w0]` encodes as UXTW; llvm-mc requires explicit uxtw/sxtw.
   - Expected: Err — Actual: Ok(Word)
   - Severity: medium
   - Report: pbt-out/bug_reports/encode_prfm_w_index.md

7. **Illegal shift amount accepted** — `lsl #1` encodes as S=1 (same as `lsl #3`). llvm-mc requires #0 or #3.
   - Expected: Err — Actual: Ok(Word)
   - Severity: medium
   - Report: pbt-out/bug_reports/encode_prfm_bad_shift.md

Serial reconfirm: all seven reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`.

## Design Caveats (if any)

- **PRFM (literal) is not implemented.** `Operand::Symbol` as the address operand returns `Err("prfm with symbol/label operand not yet supported")`.
  Doc evidence: load_store.rs:759-761 — quote: `// PRFM (literal) with symbol reference is not yet supported` / `Err("prfm with symbol/label operand not yet supported")`.
  Property `encode_prfm_neg_bad_prfop_and_name` (kind=Symbol address) asserts this Err and passes. README.md:11 gas-compatibility would include `prfm pldl1keep, label` (llvm-mc emits a PC-relative fixup); that is a documented gap, not a silent wrong encoding.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_prfm_pbt) | 11 properties + 6 KAT + 9 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_prfm_regoff_encoding.md
- pbt-out/bug_reports/encode_prfm_extra_operand.md
- pbt-out/bug_reports/encode_prfm_w_base.md
- pbt-out/bug_reports/encode_prfm_xzr_base.md
- pbt-out/bug_reports/encode_prfm_fp_base.md
- pbt-out/bug_reports/encode_prfm_w_index.md
- pbt-out/bug_reports/encode_prfm_bad_shift.md

Sweep close: tier round 1/1 spent. `coverage_gaps` had no LLVM profraw; manual arm audit added encode_prfm_neg_w_index (failing), encode_prfm_neg_bad_shift (failing), and encode_prfm_neg_bad_prfop_and_name (passing). Documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 16:03 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 68/253 total | PBT candidates: 68 | Tested: 68 (100%) | 0 pass, 68 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 68 |
| **Tested (of PBT candidates)** | **68 / 68 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 68 / 0 |
| **Overall (tested / all functions)** | **68 / 253 (27%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 68 | 68 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 68 | 68 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 24 | 24 | 100% | covered |
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
