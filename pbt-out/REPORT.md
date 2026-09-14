# PBT Campaign Report: encode_div

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_div
**Tests:** 10 properties (plus 2 KAT + 4 regression witnesses)
**Result:** 6 passing, 4 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_div | 10 properties | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### encode_div_neg_extra_operand
- **Law:** UDIV/SDIV take exactly three register operands; a fourth must be rejected.
- **Failing property:** encode_div_neg_extra_operand (negative_error)
- **Shrunk counterexample:** `rd = 0, rn = 0, rm = 0, is_64 = false, unsigned = false, extra = Reg("x0")` — `sdiv w0, w0, w0, x0`
- **Expected:** Err
- **Actual:** Ok (fourth operand ignored)
- **Serial reconfirm:** `PBT_TEST_JOBS=1` reproduced
- **Bug report:** pbt-out/bug_reports/encode_div_extra_operand.md

### encode_div_neg_mixed_width
- **Law:** ARM ARM UDIV/SDIV require same-width GPRs; llvm-mc rejects mixed x/w.
- **Failing property:** encode_div_neg_mixed_width (negative_error)
- **Shrunk counterexample:** `rd = 0, rn = 0, rm = 0, rd64 = false, rn64 = false, rm64 = true, unsigned = false` — `sdiv w0, w0, x0`
- **Expected:** Err
- **Actual:** Ok (sf taken from Rd only; Rn/Rm widths unchecked)
- **Serial reconfirm:** `PBT_TEST_JOBS=1` reproduced
- **Bug report:** pbt-out/bug_reports/encode_div_mixed_width.md

### encode_div_neg_sp
- **Law:** ARM ARM UDIV/SDIV encode register 31 as XZR/WZR, never SP/WSP; llvm-mc rejects `sdiv wsp, ...`.
- **Failing property:** encode_div_neg_sp (negative_error)
- **Shrunk counterexample:** `which = 0, is_64 = false, unsigned = false, a = 0, b = 0` — `sdiv wsp, w0, w0`
- **Expected:** Err
- **Actual:** Ok (SP/WSP encoded as WZR)
- **Serial reconfirm:** `PBT_TEST_JOBS=1` reproduced
- **Bug report:** pbt-out/bug_reports/encode_div_sp.md

### encode_div_neg_fp
- **Law:** ARM ARM UDIV/SDIV take Wt/Xt only; llvm-mc rejects `sdiv d0, ...`.
- **Failing property:** encode_div_neg_fp (negative_error)
- **Shrunk counterexample:** `which = 0, unsigned = false, prefix = "d", n = 0` — `sdiv d0, x1, x2`
- **Expected:** Err
- **Actual:** Ok (FP/SIMD name accepted as a GPR number)
- **Serial reconfirm:** `PBT_TEST_JOBS=1` reproduced
- **Bug report:** pbt-out/bug_reports/encode_div_fp_reg.md

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_div_pbt) | 10 properties + 2 KAT + 4 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/FUNCTION_INDEX.md — merged index (encode_div now a candidate)
- pbt-out/COVERAGE.md — coverage ledger row for encode_div
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed encode_div invariants
- pbt-out/REPORT.md — this report
- pbt-out/bug_reports/encode_div_extra_operand.md
- pbt-out/bug_reports/encode_div_mixed_width.md
- pbt-out/bug_reports/encode_div_sp.md
- pbt-out/bug_reports/encode_div_fp_reg.md

Contract-surface sweep closed after 1 round (standard tier): `coverage_gaps` had no LLVM profraw; manual arm audit of get_reg/parse_reg_num None and FP prefixes. Invalid-register-name property passed; FP rejection filed as a bug. Extra/mixed/SP already had properties from the first batch.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 04:43 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 26/184 total | PBT candidates: 26 | Tested: 26 (100%) | 0 pass, 26 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 26 |
| **Tested (of PBT candidates)** | **26 / 26 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 26 / 0 |
| **Overall (tested / all functions)** | **26 / 184 (14%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 26 | 26 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 26 | 26 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 5 | 5 | 100% | covered |
| load_store.rs | 20 | 1 | 1 | 100% | covered |
| neon.rs | 68 | 1 | 1 | 100% | covered |

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
