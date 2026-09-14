# PBT Campaign Report: encode_ldxp_stxp

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_ldxp_stxp
**Tests:** 9 properties (plus 7 KAT + 9 regression witnesses)
**Result:** 5 passing, 9 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep; generator runs=1000)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldxp_stxp | 9 properties (5 pass, 4 fail) + 7 KAT pass + 9 regression fail | 9 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operands ignored
- **Law:** LDXP/LDAXP take exactly 3 operands; STXP/STLXP take exactly 4. A surplus operand must be Err.
- **Shrunk counterexample:** `stxp w0, w0, w0, [x0], x2` (rt=0, extra=Reg("x2")).
- **Expected:** Err. **Actual:** Ok(Word) — no upper arity check.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_extra_operand.md
- **Regression test:** `test_encode_ldxp_stxp_regression_extra_operand` (fails, as intended)

### 2. SP/WSP accepted as Rt/Rt2/Ws
- **Law:** Rt/Rt2 are Wt/Xt (31 = ZR), never SP/WSP; STXP Ws is Wt (31 = WZR).
- **Shrunk counterexample:** `stxp w0, sp, w0, [x0]` (kind=0). Encodes as 0xc81f003f (`stxp w0, xzr, w0, [x0]` with sz forced 64-bit because `is_64bit_reg("sp")`).
- **Expected:** Err. **Actual:** Ok(Word) of the ZR encoding.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_sp_as_rt.md
- **Regression test:** `test_encode_ldxp_stxp_regression_sp_as_rt` (fails, as intended)

### 3. W register accepted as memory base
- **Law:** Rn is Xn|SP; a W-width base must be Err.
- **Minimal input:** `ldxp w0, w1, [w0]`. Encodes as `ldxp w0, w1, [x0]`.
- **Expected:** Err. **Actual:** Ok(Word) of the X-width encoding.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_w_base.md
- **Regression test:** `test_encode_ldxp_stxp_regression_w_base` (fails, as intended)

### 4. XZR/WZR accepted as memory base (encoded as SP)
- **Law:** Rn is Xn|SP; XZR/WZR is not a valid exclusive-pair base.
- **Minimal input:** `ldxp x0, x1, [xzr]`. Encodes as `ldxp x0, x1, [sp]`.
- **Expected:** Err. **Actual:** Ok(Word) of the SP encoding.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_xzr_as_base.md
- **Regression test:** `test_encode_ldxp_stxp_regression_xzr_as_base` (fails, as intended)

### 5. SIMD/FP register accepted as Rt
- **Law:** Data registers are Wt/Xt only; llvm-mc rejects `ldxp d0, x1, [x2]`.
- **Minimal input:** `ldxp d0, x1, [x2]`. Encodes as the matching-number GPR form.
- **Expected:** Err. **Actual:** Ok(Word).
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_fp_as_rt.md
- **Regression test:** `test_encode_ldxp_stxp_regression_fp_as_rt` (fails, as intended)

### 6. Mixed X/W exclusive pair accepted
- **Law:** Rt and Rt2 must be the same width.
- **Minimal input:** `ldxp x0, w1, [x2]`. sz taken only from the first data register.
- **Expected:** Err. **Actual:** Ok(Word) of `ldxp x0, x1, [x2]`.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_mixed_width.md
- **Regression test:** `test_encode_ldxp_stxp_regression_mixed_width` (fails, as intended)

### 7. X register accepted as STXP status
- **Law:** STXP/STLXP status is Ws (32-bit), never Xt.
- **Minimal input:** `stxp x0, x1, x2, [x3]`. Width of Ws is ignored.
- **Expected:** Err. **Actual:** Ok(Word) of `stxp w0, x1, x2, [x3]`.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_x_as_ws.md
- **Regression test:** `test_encode_ldxp_stxp_regression_x_as_ws` (fails, as intended)

### 8. Nonzero offset ignored
- **Law:** llvm-mc: "index must be absent or #0".
- **Shrunk counterexample:** `stxp w0, w0, w0, [x0, #-1]` (offset=-1, shape=0). `Mem { base, .. }` discards offset.
- **Expected:** Err. **Actual:** Ok(Word(0x88200000)) of `[x0]`.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_nonzero_offset.md
- **Regression test:** `test_encode_ldxp_stxp_regression_nonzero_offset` (fails, as intended)

### 9. STXP Ws overlapping a source register
- **Law:** ARM CONSTRAINED UNPREDICTABLE / llvm-mc: "unpredictable STXP instruction, status is also a source" when Ws aliases Rt, Rt2, or Xn (WZR vs SP is allowed).
- **Shrunk counterexample:** `stxp w0, w0, w0, [x0]` (kind=0, ws==rt).
- **Expected:** Err. **Actual:** Ok(Word(0x88200000)).
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldxp_stxp_ws_overlap.md
- **Regression test:** `test_encode_ldxp_stxp_regression_ws_overlap` (fails, as intended)

Serial reconfirmation: all failures reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldxp_stxp_neg -- --test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_ldxp_stxp_pbt) | 9 properties + 7 KAT + 9 regressions |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row
- pbt-out/COVERAGE_STATUS.md — scanned vs tested
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/bug_reports/encode_ldxp_stxp_*.md — 9 bug reports

Contract-surface sweep: 1 round (standard). `coverage_gaps` had no LLVM profraw; sweep was a manual arm audit of too-few / non-Reg / non-Mem / parse_reg_num None — passing as `encode_ldxp_stxp_neg_too_short_nonmem_badname`. Closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 06:50 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 33/184 total | PBT candidates: 33 | Tested: 33 (100%) | 0 pass, 33 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 33 |
| **Tested (of PBT candidates)** | **33 / 33 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 33 / 0 |
| **Overall (tested / all functions)** | **33 / 184 (18%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 33 | 33 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 33 | 33 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
| load_store.rs | 20 | 4 | 4 | 100% | covered |
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
| encode_ldur_stur | load_store.rs |
| encode_ldxp_stxp | load_store.rs |
