# PBT Campaign Report: encode_neon_tbl

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_tbl (src/backend/arm/assembler/encoder/neon.rs)
**Tests:** 9 properties (4 passing, 5 failing) plus 1 passing KAT and 10 failing regression witnesses
**Result:** 4 passing properties, 10 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-gaps round; first batch did not all pass so no extra strengthening round beyond the sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_tbl | 9 properties + 1 KAT + 10 regression | 10 | differential (llvm-mc), algebraic.metamorphic (Q XOR, len bits), algebraic.invariant (ARM fields), negative_error |

## Bugs Found

1. **encode_neon_tbl_neg_extra_operand.** Law: TBL takes exactly three operands. Shrunk counterexample: rd=0, rn=0, rm=0, extra=0, ta=8b, n=1 — `tbl v0.8b, {v0.16b}, v0.8b, v0.8b` returns Ok(Word) not Err. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_tbl_extra_operand.md`
2. **encode_neon_tbl_neg_invalid_ta.** Law: Ta ∈ {8B,16B}. Shrunk counterexample: ta=4h — `tbl v0.4h, {v0.16b}, v0.4h` encodes as Q=0. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_tbl_invalid_ta.md`
3. **encode_neon_tbl_neg_table_contract (empty list).** Law: invalid table list must Err, not panic. Shrunk counterexample: kind=0 — `RegList([])` panics at `regs[0]`. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_tbl_empty_list_panic.md`
4. **Five table registers wrap.** Law: nregs ∈ {1,2,3,4}. `n=5` encodes `len=(5-1)&3=0` (1-register TBL). Path: `pbt-out/bug_reports/encode_neon_tbl_five_regs.md`
5. **Non-sequential table.** Law: table registers must be consecutive. `{v0.16b, v2.16b}` encodes as `{v0.16b, v1.16b}`. Path: `pbt-out/bug_reports/encode_neon_tbl_nonsequential.md`
6. **Table arrangement not .16B.** Law: table is `.16B`. `{v0.8b}` is accepted. Path: `pbt-out/bug_reports/encode_neon_tbl_table_not_16b.md`
7. **encode_neon_tbl_neg_arity_kinds (GPR dest).** Law: dest is Vd.Ta. Shrunk counterexample: `tbl x0, {v0.16b}, v0.8b` encodes. Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_tbl_gpr_dest.md`
8. **Mismatched Vd.Ta / Vm.Ta.** Law: Vm.Ta equals Vd.Ta. `tbl v0.8b, {v0.16b}, v0.16b` encodes. Path: `pbt-out/bug_reports/encode_neon_tbl_mismatched_t.md`
9. **encode_neon_tbl_neg_list_and_vm_kinds (bare list Reg).** Law: table member is Vn.16B. Shrunk counterexample: kind=0 — `RegList([Reg("v0")])` encodes as Word(0x0e000000). Serial reconfirm: PBT_TEST_JOBS=1 reproduced. Path: `pbt-out/bug_reports/encode_neon_tbl_bare_list_reg.md`
10. **Bare GPR Vm.** Law: Vm is Vm.Ta. `tbl v0.8b, {v0.16b}, x0` encodes via get_neon_reg accepting Operand::Reg. Path: `pbt-out/bug_reports/encode_neon_tbl_bare_vm.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_tbl_pbt) | 1 KAT + 9 proptest properties + 10 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_tbl_extra_operand.md
- pbt-out/bug_reports/encode_neon_tbl_invalid_ta.md
- pbt-out/bug_reports/encode_neon_tbl_empty_list_panic.md
- pbt-out/bug_reports/encode_neon_tbl_five_regs.md
- pbt-out/bug_reports/encode_neon_tbl_nonsequential.md
- pbt-out/bug_reports/encode_neon_tbl_table_not_16b.md
- pbt-out/bug_reports/encode_neon_tbl_gpr_dest.md
- pbt-out/bug_reports/encode_neon_tbl_mismatched_t.md
- pbt-out/bug_reports/encode_neon_tbl_bare_list_reg.md
- pbt-out/bug_reports/encode_neon_tbl_bare_vm.md

## Contract-surface sweep

Round 1 of 1 (standard). `coverage_gaps` had no LLVM profraw. Manual arm audit of encode_neon_tbl: `regs[0]` as Operand::Reg / Imm / bad name, and Vm as Operand::Reg, were untested; added encode_neon_tbl_neg_list_and_vm_kinds (failing; bugs 9 and 10). Remaining branches (arity < 3, missing RegList, invalid dest names, Q/len fields, valid Ta×nregs wrapping) were already reached. Closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 11:02 (campaign: coverage)
> Files: 7/7 scanned (100%) | Functions: 49/229 total | PBT candidates: 49 | Tested: 49 (100%) | 0 pass, 49 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 7 |
| Files scanned | 7 / 7 (100%) |
| Total functions (all files) | 229 |
| PBT candidates (from FUNCTION_INDEX) | 49 |
| **Tested (of PBT candidates)** | **49 / 49 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 49 / 0 |
| **Overall (tested / all functions)** | **49 / 229 (21%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 49 | 49 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 49 | 49 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 15 | 15 | 100% | covered |
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
