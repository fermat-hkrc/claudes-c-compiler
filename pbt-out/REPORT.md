# PBT Campaign Report: encode_neon_shift_left_imm

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_shift_left_imm
**Tests:** 10 properties (5 passing, 5 failing) plus 2 passing KAT and 7 failing regression witnesses
**Result:** 5 passing, 5 bugs
**Effort tier:** standard (1 coverage-driven sweep round; closed because the tier's round is done — coverage_gaps had no profraw, so the round was a manual ARM-contract audit of extra operand / shift range / T mismatch / non-V prefix / Operand::Reg without arrangement / arity / unsupported T)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_shift_left_imm | 10 properties + 2 KAT + 7 regression | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### encode_neon_shift_left_imm_neg_extra_operands
- **Failing property:** encode_neon_shift_left_imm_neg_extra_operands (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, extra=0, t="8b", shift=0, u=0, extra_kind=0 — `sqshl v0.8b, v0.8b, #0, v0.8b`
- **Expected:** Err
- **Actual:** Ok(Word) — 4th operand ignored (`len() < 3`)
- **Law:** SQSHL/UQSHL immediate takes exactly 3 operands; llvm-mc rejects a 4th
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_shift_left_imm_extra_operand.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_neon_shift_left_imm_neg_extra_operands -- --test-threads=1`

### encode_neon_shift_left_imm_neg_shift_oob
- **Failing property:** encode_neon_shift_left_imm_neg_shift_oob (negative_error)
- **Shrunk counterexample:** rd=0, rn=0, t="8b", shift=-1, u=0 — `sqshl v0.8b, v0.8b, #-1`
- **Expected:** Err
- **Actual:** debug panic `attempt to add with overflow` at neon.rs:1768 (`esize + (shift as u32)`). `Imm(8)` silently encodes as a 16-bit lane; `Imm(1<<32)` truncates to `#0`.
- **Law:** ARM ARM / llvm-mc require shift in `[0, esize-1]`
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/encode_neon_shift_left_imm_shift_oob.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_neon_shift_left_imm_neg_shift_oob -- --test-threads=1`

### encode_neon_shift_left_imm_neg_arity_shape
- **Failing property:** encode_neon_shift_left_imm_neg_arity_shape (negative_error)
- **Shrunk counterexample:** dest T=8b, src T=16b, shift=0, u=0 — `sqshl v0.8b, v0.16b, #0`
- **Expected:** Err
- **Actual:** Ok(Word) — source arrangement discarded
- **Law:** Vd.T and Vn.T must match; llvm-mc rejects mismatched T
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_shift_left_imm_arrangement_mismatch.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_neon_shift_left_imm_neg_arity_shape -- --test-threads=1`

### encode_neon_shift_left_imm_neg_non_v_prefix
- **Failing property:** encode_neon_shift_left_imm_neg_non_v_prefix (negative_error)
- **Shrunk counterexample:** prefix="x", which=0, t="8b", shift=0 — `sqshl x0.8b, v0.8b, #0`
- **Expected:** Err
- **Actual:** Ok(Word) — `parse_reg_num` accepts x/w/d/s/q/h/b
- **Law:** Vector SQSHL/UQSHL requires V registers; llvm-mc rejects non-V prefixes
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_shift_left_imm_non_v_prefix.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_neon_shift_left_imm_neg_non_v_prefix -- --test-threads=1`

### encode_neon_shift_left_imm_neg_reg_no_arrangement
- **Failing property:** encode_neon_shift_left_imm_neg_reg_no_arrangement (negative_error)
- **Shrunk counterexample:** which=1 (source), t="8b", shift=0 — `sqshl v0.8b, v0, #0`
- **Expected:** Err
- **Actual:** Ok(Word) — `get_neon_reg` accepts Operand::Reg and the encoder ignores the empty arrangement
- **Law:** Both operands need matching T; llvm-mc rejects a bare Vn
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_shift_left_imm_src_reg_no_arrangement.md
- **Reproduce:** `PBT_TEST_JOBS=1 cargo test --lib encode_neon_shift_left_imm_neg_reg_no_arrangement -- --test-threads=1`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_shift_left_imm_pbt) | 10 properties + 2 KAT + 7 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_shift_left_imm_extra_operand.md
- pbt-out/bug_reports/encode_neon_shift_left_imm_shift_oob.md
- pbt-out/bug_reports/encode_neon_shift_left_imm_arrangement_mismatch.md
- pbt-out/bug_reports/encode_neon_shift_left_imm_non_v_prefix.md
- pbt-out/bug_reports/encode_neon_shift_left_imm_src_reg_no_arrangement.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 13:37 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 59/253 total | PBT candidates: 59 | Tested: 59 (100%) | 0 pass, 59 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 59 |
| **Tested (of PBT candidates)** | **59 / 59 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 59 / 0 |
| **Overall (tested / all functions)** | **59 / 253 (23%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 59 | 59 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 59 | 59 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 20 | 20 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 12 | 12 | 100% | covered |
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
