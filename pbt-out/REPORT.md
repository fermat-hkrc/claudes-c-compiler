# PBT Campaign Report: encode_shift

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_shift (src/backend/x86/assembler/encoder/gp_integer.rs)
**Tests:** 12 properties + 6 KAT + 4 regression witnesses
**Result:** 9 passing properties, 3 failing properties (3 bugs); 6 KAT passing; 4 regression witnesses failing as intended
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 metamorphic/differential required, 1 strengthening/coverage-sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_shift | 12 properties (9 pass / 3 fail), 6 KAT, 4 regressions | 3 | differential (llvm-mc x86_64), algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. encode_shift_diff_mem — missing FS segment prefix
- **Failing property:** encode_shift_diff_mem (differential vs llvm-mc)
- **Shrunk counterexample:** kind=shl, suf=b, base=rax, form=imm, count=0, seg=Some("fs") → AT&T `shlb $0, %fs:(%rax)`
- **Expected:** `[0x64, 0xc0, 0x20, 0x00]`
- **Actual:** `[0xc0, 0x20, 0x00]`
- **Severity:** high
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced.
- **Bug report:** pbt-out/bug_reports/encode_shift_missing_segment_prefix.md
- **Regression test:** `test_encode_shift_regression_fs_segment_prefix`

### 2. encode_shift_neg_mixed_size_and_non_gp — size-mismatched dest accepted
- **Failing property:** encode_shift_neg_mixed_size_and_non_gp (negative_error)
- **Shrunk counterexample:** kind=shl, suf=w, dest=al → AT&T `shlw $1, %al`
- **Expected:** Err
- **Actual:** Ok([0x66, 0xd1, 0xe0]) (encodes as `shlw %ax`)
- **Severity:** high
- **Serial reconfirm:** PBT_TEST_JOBS=1 reproduced.
- **Bug report:** pbt-out/bug_reports/encode_shift_accepts_mismatched_and_non_gp_dest.md
- **Regression test:** `test_encode_shift_regression_mixed_size_shlw_al`

### 3. encode_shift_neg_imm_overflow — imm8 count truncated
- **Failing property:** encode_shift_neg_imm_overflow (negative_error)
- **Shrunk counterexample:** kind=shl, suf=b, dst=al, count=256 → AT&T `shlb $256, %al`
- **Expected:** Err
- **Actual:** Ok (256 truncated to 0 via `count as u8`)
- **Severity:** medium
- **Serial reconfirm:** deterministic regression `shlq $256, %rax` → Ok([0x48, 0xc1, 0xe0, 0x00])
- **Bug report:** pbt-out/bug_reports/encode_shift_truncates_imm8_count.md
- **Regression test:** `test_encode_shift_regression_imm8_overflow_256`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/x86/assembler/encoder/gp_integer.rs (mod encode_shift_pbt) | 12 properties, 6 KAT, 4 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_shift_missing_segment_prefix.md
- pbt-out/bug_reports/encode_shift_accepts_mismatched_and_non_gp_dest.md
- pbt-out/bug_reports/encode_shift_truncates_imm8_count.md

## Contract-surface sweep

STANDARD owes 1 round. `coverage_gaps` had no LLVM profraw; sweep was a manual arm audit of encode_shift. Added `encode_shift_neg_one_operand_non_rm` (1-operand Imm/Label/Indirect Err) and `encode_shift_rip_reloc_addend` (RIP + trailing imm8 addend -5). Both passing. Close: tier round done.

## Harness

- **Test layout:** inline `#[cfg(test)] mod encode_shift_pbt` in gp_integer.rs
- **Buildability probe:** `cargo test --lib test_ascii_passthrough` → 1 passed
- **Harness placement:** rung 1, `cargo test --lib encode_shift_pbt`, proptest 1.11 already in Cargo.toml
- **Build contract:** `cargo check --lib`; tests via `cargo test --lib` (official cargo harness)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 12:01 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 53/253 total | PBT candidates: 53 | Tested: 53 (100%) | 0 pass, 53 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 53 |
| **Tested (of PBT candidates)** | **53 / 53 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 53 / 0 |
| **Overall (tested / all functions)** | **53 / 253 (21%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 53 | 53 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 53 | 53 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 17 | 17 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 9 | 9 | 100% | covered |
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
