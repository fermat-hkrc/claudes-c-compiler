# PBT Campaign Report: encode_neon_rbit

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_rbit
**Tests:** 13 properties (8 passing, 5 failing) plus 3 passing KAT and 5 failing regression witnesses
**Result:** 8 passing, 5 bugs
**Effort tier:** standard (5–8 properties, ≥1000 cases, one strengthening round, one contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_rbit | 13 properties (1000 cases each) | 5 | differential (llvm-mc), algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

- **encode_neon_rbit_neg_extra_operands** (failing, shrunk). Witness: `rd=0, rn=0, extra=0, t="8b", extra_kind=0` → `rbit v0.8b, v0.8b, v0.8b`. Expected Err; actual Ok(Word). Serial reconfirm: `cargo test --lib encode_neon_rbit_pbt -- --test-threads=1`. Report: pbt-out/bug_reports/encode_neon_rbit_extra_operand.md
- **encode_neon_rbit_neg_mismatch_nonreg_invalid** (failing, shrunk). Witness: `rd=0, rn=0, td="8b", tn="16b"` → `rbit v0.8b, v0.16b`. Expected Err; actual Ok(Word). Serial reconfirm as above. Report: pbt-out/bug_reports/encode_neon_rbit_mismatch_arrangement.md
- **encode_neon_rbit_neg_bare_src** (failing, shrunk). Witness: `rd=0, rn=0, t="8b"` → dest `v0.8b`, src `Reg("v0")`. Expected Err; actual Ok(Word). Serial reconfirm as above. Report: pbt-out/bug_reports/encode_neon_rbit_bare_src.md
- **encode_neon_rbit_neg_bad_prefix** (failing, shrunk). Witness: `rd=0, rn=0, t="8b", prefix="x"` → `rbit x0.8b, x0.8b`. Expected Err; actual Ok(Word). Serial reconfirm as above. Report: pbt-out/bug_reports/encode_neon_rbit_non_v_prefix.md
- **encode_neon_rbit_neg_sp** (failing, shrunk). Witness: `rd=0, t="8b", which=0` → `rbit sp.8b, v0.8b`. Expected Err; actual Ok(Word) with Rd=31. Serial reconfirm as above. Report: pbt-out/bug_reports/encode_neon_rbit_sp_as_neon.md

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_rbit_pbt) | 13 properties + 3 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage row for encode_neon_rbit
- pbt-out/FUNCTION_INDEX.md — encode_neon_rbit marked yes
- pbt-out/INVARIANTS.md — confirmed encode_neon_rbit invariants
- pbt-out/bug_reports/encode_neon_rbit_extra_operand.md
- pbt-out/bug_reports/encode_neon_rbit_mismatch_arrangement.md
- pbt-out/bug_reports/encode_neon_rbit_bare_src.md
- pbt-out/bug_reports/encode_neon_rbit_non_v_prefix.md
- pbt-out/bug_reports/encode_neon_rbit_sp_as_neon.md

## Contract-surface sweep

Round 1 of 1 (standard). `coverage_gaps` reported no instrumented profraw in this session. Manual audit of encode_neon_rbit: arity `< 2`, dest T ∉ {8b,16b}, Q bit, Rd/Rn fields, llvm-mc agreement, extra operand, mismatched T, bare src, Imm src, invalid names, non-V prefix, SP. No remaining documented branch without a property. Sweep closed because the tier's one round is done.

## Harness

Rung 1: extend `cargo test --lib`. Probe: `cargo test --lib encode_neon_shift_left_imm_kat_llvm_mc_v0_8b_v1_8b` → 1 passed. Framework: proptest 1.11. Rebuilds used `cargo test --lib encode_neon_rbit_pbt` (swap of `cargo check --lib`).

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 14:23 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 62/253 total | PBT candidates: 62 | Tested: 62 (100%) | 0 pass, 62 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 62 |
| **Tested (of PBT candidates)** | **62 / 62 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 62 / 0 |
| **Overall (tested / all functions)** | **62 / 253 (25%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 62 | 62 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 62 | 62 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 22 | 22 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
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
