# PBT Campaign Report: encode_eon

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_eon
**Tests:** 11 properties (plus 2 KAT + 7 regression witnesses)
**Result:** 5 passing, 6 failing properties (6 SUT bugs)
**Effort tier:** standard
**Contract-surface sweep:** 1 round (coverage_gaps had no profraw; manual arm audit of unknown-shift `_ => 0b00` and parse_reg_num None). Closed because the tier's 1 round is done.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_eon | 11 properties (5 pass, 6 fail) | 6 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **Missing EON-immediate alias** — property `encode_eon_diff_imm_llvm_mc`. Shrunk counterexample: `eon w0, w0, #0xaaaaaaaa` (rd=0, rn=0, is_64=false, seed=0). Expected Word(0x5200f000); actual Err("expected register at operand 2"). Path: `pbt-out/bug_reports/encode_eon_imm_alias.md`. Severity: medium.

2. **Extra operand ignored** — property `encode_eon_neg_extra_operand`. Shrunk counterexample: `eon w0, w0, w0, w0` (is_64=false, rd=rn=rm=0, which=0). Expected Err; actual Ok(Word). Path: `pbt-out/bug_reports/encode_eon_extra_operand.md`. Severity: low.

3. **Mixed x/w widths accepted** — property `encode_eon_neg_mixed_width`. Shrunk counterexample: `eon w0, w0, x0` (rd=rn=rm=0, rd64=false, rn64=false, rm64=true). Expected Err; actual Ok(Word). Path: `pbt-out/bug_reports/encode_eon_mixed_width.md`. Severity: medium.

4. **SP/WSP encoded as ZR** — property `encode_eon_neg_sp_fp`. Shrunk counterexample: `eon wsp, w0, w0` (which=0, is_64=false, kind=0). Expected Err; actual Ok(Word) via parse_reg_num mapping sp/wsp→31. The same property domain also includes FP/SIMD prefixes (d/s/q/v/h/b); regression `test_encode_eon_regression_fp_reg` (`eon d0, x1, x2`) fails the same way. Path: `pbt-out/bug_reports/encode_eon_sp_register_form.md`. Severity: medium.

5. **Out-of-range shift amount masked** — property `encode_eon_neg_shift_range`. Shrunk counterexample: `eon w0, w0, w0, lsl #32` (rd=rn=rm=0, is_64=false, kind="lsl", amt_w=32). Expected Err; actual Ok(Word) with imm6=amount&0x3F. Path: `pbt-out/bug_reports/encode_eon_shift_out_of_range.md`. Severity: medium.

6. **Unknown shift kind defaults to LSL** — property `encode_eon_neg_unknown_shift`. Shrunk counterexample: `eon w0, w0, w0, lslx #0` (unknown="lslx", amt_ok=0). Expected Err; actual Ok(Word) via `_ => 0b00`. Path: `pbt-out/bug_reports/encode_eon_unknown_shift_kind.md`. Severity: medium.

All six reproduced serially with `PBT_TEST_JOBS=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_eon_pbt) | 11 properties + 2 KAT + 7 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — per-function coverage row
- pbt-out/COVERAGE_STATUS.md — campaign coverage stats
- pbt-out/FUNCTION_INDEX.md — merged function index (encode_eon marked yes)
- pbt-out/INVARIANTS.md — confirmed encode_eon invariants
- pbt-out/bug_reports/encode_eon_imm_alias.md
- pbt-out/bug_reports/encode_eon_extra_operand.md
- pbt-out/bug_reports/encode_eon_mixed_width.md
- pbt-out/bug_reports/encode_eon_sp_register_form.md
- pbt-out/bug_reports/encode_eon_fp_reg.md
- pbt-out/bug_reports/encode_eon_shift_out_of_range.md
- pbt-out/bug_reports/encode_eon_unknown_shift_kind.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 04:57 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 27/184 total | PBT candidates: 27 | Tested: 27 (100%) | 0 pass, 27 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 27 |
| **Tested (of PBT candidates)** | **27 / 27 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 27 / 0 |
| **Overall (tested / all functions)** | **27 / 184 (15%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 27 | 27 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 27 | 27 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
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
| encode_eon | data_processing.rs |
