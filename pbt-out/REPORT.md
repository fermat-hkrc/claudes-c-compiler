# PBT Campaign Report: encode_neon_sli

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_neon_sli
**Tests:** 8 properties (plus 2 KAT + 6 regression witnesses)
**Result:** 5 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep; generator runs=1000)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_sli | 8 properties (5 pass, 3 fail) + 2 KAT pass + 6 regression fail | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operands ignored
- **Law:** Vector SLI takes `Vd.T, Vn.T, #shift` only; a fourth operand must be Err.
- **Shrunk counterexample:** `sli v0.8b, v0.8b, #0, v0.8b` (rd=0, rn=0, extra=0, t="8b", extra_kind=0).
- **Expected:** Err. **Actual:** Ok(Word) — arity check is `operands.len() < 3`.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_sli_extra_operand.md
- **Regression test:** `test_encode_neon_sli_regression_extra_operand` (fails, as intended)

### 2. Out-of-range shift not rejected
- **Law:** SLI shift must be in [0, esize(T)-1]; llvm-mc rejects values outside that range.
- **Shrunk counterexample:** `sli v0.8b, v0.8b, #-1` (rd=0, rn=0, t="8b", shift=-1) panics in debug (`8 + (-1 as u32)` overflow). Also `sli v0.8b, v0.8b, #8` returns Ok with immh:immb masked to 0 (reserved).
- **Expected:** Err. **Actual:** debug panic on negative; Ok(Word) with wrapped/masked immh:immb on shift >= esize.
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/encode_neon_sli_shift_out_of_range.md
- **Regression test:** `test_encode_neon_sli_regression_negative_shift` and `test_encode_neon_sli_regression_shift_eq_esize` (fail, as intended)

### 3. Source arrangement ignored
- **Law:** ARM ARM requires the same T on dest and source (`<Vd>.<T>, <Vn>.<T>, #shift`).
- **Shrunk counterexample:** `sli v0.8b, v0.16b, #0` (rd=0, rn=0, td="8b", tn="16b"). Encodes as `sli v0.8b, v0.8b, #0`. Coverage sweep: `sli v0.8b, v0, #0` (src Operand::Reg) also Ok.
- **Expected:** Err. **Actual:** Ok(Word) — Q/immh taken only from dest; `let (rn, _) = get_neon_reg(operands, 1)`.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_sli_arrangement_mismatch.md
- **Regression test:** `test_encode_neon_sli_regression_arrangement_mismatch` and `test_encode_neon_sli_regression_src_reg_no_arrangement` (fail, as intended)

### 4. Non-V register prefix accepted
- **Law:** Vector SLI takes V registers; x/w/d/s/q/h/b prefixes must be Err.
- **Shrunk counterexample:** `sli x0.8b, v0.8b, #0` (rd=0, rn=0, t="8b", prefix="x", which=0). Encodes as `sli v0.8b, v0.8b, #0`.
- **Expected:** Err. **Actual:** Ok(Word) — `parse_reg_num` maps x/w/d/s/q/v/h/b with num<=31 to the same 5-bit number.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_neon_sli_non_v_prefix.md
- **Regression test:** `test_encode_neon_sli_regression_non_v_prefix` (fails, as intended)

Serial reconfirmation: all four failures reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_sli_neg -- --test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/neon.rs (mod encode_neon_sli_pbt) | 8 properties (9 proptest cases) + 2 KAT + 6 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_sli_extra_operand.md
- pbt-out/bug_reports/encode_neon_sli_shift_out_of_range.md
- pbt-out/bug_reports/encode_neon_sli_arrangement_mismatch.md
- pbt-out/bug_reports/encode_neon_sli_non_v_prefix.md

Contract-surface sweep closed after 1 round (standard tier): `coverage_gaps` had no LLVM profraw; manual arm audit of get_neon_reg Operand::Reg source (empty arrangement accepted — folded into bug 3) and dest Operand::Reg (Err as specified). Sweep closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 05:59 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 31/184 total | PBT candidates: 31 | Tested: 31 (100%) | 0 pass, 31 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 31 |
| **Tested (of PBT candidates)** | **31 / 31 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 31 / 0 |
| **Overall (tested / all functions)** | **31 / 184 (17%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 31 | 31 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 31 | 31 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
| load_store.rs | 20 | 2 | 2 | 100% | covered |
| neon.rs | 68 | 4 | 4 | 100% | covered |

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
