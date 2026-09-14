# PBT Campaign Report: encode_movk

## Summary

**Date:** 2026-03-26
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_movk
**Tests:** 14 property tests + 1 KAT + 5 regression witnesses (20 cargo tests in encode_movk_pbt)
**Result:** 9 passing properties (plus 3-vector KAT), 5 failing properties, 5 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_movk | 9 passing / 5 failing (plus KAT + 5 regressions) | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

Each entry is a failing proptest property that shrank to a concrete witness and reproduced with `PBT_TEST_JOBS=1`.

1. **encode_movk_neg_imm_oob** — Law: imm16 ∈ [0, 65535] must Err. Shrunk counterexample: `rd=0, is_64=false, imm=-1` (`movk w0, #-1`). Expected Err; actual Ok (imm16 truncated to 0xFFFF). Severity: medium. `pbt-out/bug_reports/encode_movk_imm_oob.md`
2. **encode_movk_neg_invalid_shift** — Law: only lsl with {0,16} (W) or {0,16,32,48} (X). Shrunk counterexample: `rd=0, imm=0, is_64=false, kind="lsr", amount=0` (`movk w0, #0, lsr #0`). Expected Err; actual Ok (hw=0). Severity: medium. `pbt-out/bug_reports/encode_movk_invalid_shift.md`
3. **encode_movk_neg_extra_operand** — Law: no operand after optional lsl. Shrunk counterexample: `rd=0, is_64=true, hw=0, imm=0, extra=Reg("x0")` (`movk x0, #0, x0`). Expected Err; actual Ok. Severity: medium. `pbt-out/bug_reports/encode_movk_extra_operand.md`
4. **encode_movk_neg_sp** — Law: Rd=31 is XZR/WZR, never SP/WSP. Shrunk counterexample: `is_64=false, imm=0, hw=0` (`movk wsp, #0`). Expected Err; actual Ok (Rd=31 WZR). Severity: medium. `pbt-out/bug_reports/encode_movk_sp.md`
5. **encode_movk_neg_fp** — Law: FP/SIMD names are not MOVK Rd. Shrunk counterexample: `fp="d0", imm=0` (`movk d0, #0`). Expected Err; actual Ok (`movk w0, #0`). Severity: medium. `pbt-out/bug_reports/encode_movk_fp_as_gpr.md`

## Design Caveats (if any)

- Unresolved `:abs_gN:symbol` (non-constant) returns Err via get_imm rather than a relocation. `RelocType` has no MOVW/G0–G3 variants. Codegen never emits abs_g for movk (only `movk Rd, #imm16 [, lsl #N]`). Constant abs_g is folded and matches llvm-mc of the resolved instruction. Doc evidence: `data_processing.rs:179-181` "If the expression contains a symbol reference, returns None (needs relocation)"; RelocType enum has no MOVW member.
- Sweep close-out: `coverage_gaps` had no profraw; one manual arm-audit round added `encode_movk_neg_bad_second` (Modifier-None / non-Imm second operand), which passed. Standard tier owes exactly 1 sweep round — done.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_movk_pbt) | 9 passing properties, 5 failing properties, 1 KAT, 5 failing regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_movk_imm_oob.md
- pbt-out/bug_reports/encode_movk_invalid_shift.md
- pbt-out/bug_reports/encode_movk_extra_operand.md
- pbt-out/bug_reports/encode_movk_sp.md
- pbt-out/bug_reports/encode_movk_fp_as_gpr.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 08:18 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 38/184 total | PBT candidates: 38 | Tested: 38 (100%) | 0 pass, 38 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 38 |
| **Tested (of PBT candidates)** | **38 / 38 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 38 / 0 |
| **Overall (tested / all functions)** | **38 / 184 (21%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 38 | 38 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 38 | 38 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 9 | 9 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 5 | 5 | 100% | covered |

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
