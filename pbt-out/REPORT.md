# PBT Campaign Report: encode_negs

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_negs (src/backend/arm/assembler/encoder/data_processing.rs)
**Tests:** 10 properties (7 passing, 3 failing groups covering 7 failing tests) plus 3 passing KAT and 7 failing regression witnesses
**Result:** 7 passing properties, 6 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-gaps round; first batch did not all pass so no extra strengthening round beyond the sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_negs | 10 properties + 3 KAT + 7 regression | 6 | differential (llvm-mc), algebraic.metamorphic (sf XOR, SUBS alias), algebraic.invariant (ARM fields), negative_error |

## Bugs Found

1. **encode_negs_neg_extra_operand** (also encode_negs_neg_trailing_after_shift). Law: NEGS is Rd, Rm{, shift} only. Shrunk counterexample: rd=0, rm=0, is_64=false, extra=Reg("x0") — `encode_negs([Reg("w0"), Reg("w0"), Reg("x0")])` returns Ok(Word) not Err. Second shrunk witness: extra after Shift{lsl,1}. Serial reconfirm: PBT_TEST_JOBS=1, `--test-threads=1` reproduced. Path: `pbt-out/bug_reports/encode_negs_extra_operand.md`
2. **encode_negs_neg_mixed_width.** Law: Rd and Rm same width. Shrunk counterexample: rd=0, rm=0, rd64=false, rm64=true — `encode_negs([Reg("w0"), Reg("x0")])` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_negs_mixed_width.md`
3. **encode_negs_neg_sp.** Law: register 31 is XZR/WZR, never SP/WSP. Shrunk counterexample: which=0, is_64=false, other=0 — `encode_negs([Reg("wsp"), Reg("w0")])` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_negs_sp.md`
4. **encode_negs_neg_fp.** Law: NEGS operands are integer GPRs. Shrunk counterexample: which=0, prefix="d", n=0 — `encode_negs([Reg("d0"), Reg("x1")])` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_negs_fp_reg.md`
5. **encode_negs_neg_shift_range.** Law: imm6 0..31 (sf=0) / 0..63 (sf=1). Shrunk counterexample: rd=0, rm=0, is_64=false, kind="lsl", amt_w=32 — `encode_negs([Reg("w0"), Reg("w0"), Shift{lsl,32}])` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_negs_shift_range.md`
6. **encode_negs_neg_bad_shift_kind.** Law: shift ∈ {LSL,LSR,ASR}. Shrunk counterexample: rd=0, rm=0, is_64=false, kind="ror", amt=0 — `encode_negs([Reg("w0"), Reg("w0"), Shift{ror,0}])` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_negs_bad_shift_kind.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_negs_pbt) | 3 KAT + 15 proptest properties + 7 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_negs_extra_operand.md
- pbt-out/bug_reports/encode_negs_mixed_width.md
- pbt-out/bug_reports/encode_negs_sp.md
- pbt-out/bug_reports/encode_negs_fp_reg.md
- pbt-out/bug_reports/encode_negs_shift_range.md
- pbt-out/bug_reports/encode_negs_bad_shift_kind.md

## Contract-surface sweep

Round 1 of 1 (standard). `coverage_gaps` had no LLVM profraw. Manual arm audit of encode_negs: get_reg(0)/get_reg(1) error paths (invalid name, non-Reg) were untested; added encode_negs_neg_invalid_name and encode_negs_neg_non_reg (both passing, 1000 cases). Remaining branches (Shift present vs absent, lsl/lsr/asr/default, sf) were already reached. Closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 10:29 (campaign: coverage)
> Files: 7/7 scanned (100%) | Functions: 47/229 total | PBT candidates: 47 | Tested: 47 (100%) | 0 pass, 47 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 7 |
| Files scanned | 7 / 7 (100%) |
| Total functions (all files) | 229 |
| PBT candidates (from FUNCTION_INDEX) | 47 |
| **Tested (of PBT candidates)** | **47 / 47 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 47 / 0 |
| **Overall (tested / all functions)** | **47 / 229 (21%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 47 | 47 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 47 | 47 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 15 | 15 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 7 | 7 | 100% | covered |
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
