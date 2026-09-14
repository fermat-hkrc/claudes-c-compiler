# PBT Campaign Report: encode_orn

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_orn
**Tests:** 13
**Result:** 9 passing, 10 bugs
**Effort tier:** standard (1 coverage-driven sweep round; coverage_gaps had no profraw — manual arm audit of arity / Imm / Shift / NEON T / extra operands / get_reg kinds)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_orn | 13 properties + 4 KAT + 10 regression | 10 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **GNU ORN-immediate alias rejected.** `orn w0, w0, #0xaaaaaaaa` must encode as `orr w0, w0, #0x55555555` (llvm-mc 0x3200f000). SUT returns Err("expected register at operand 2"). Law: README.md:14 gas-compatible text; dispatch `"orn" => encode_orn`. Severity: medium. Report: `pbt-out/bug_reports/encode_orn_imm_alias.md`.

2. **Trailing non-shift 4th operand ignored.** `orn w0, w0, w0, w0` encodes as three-operand ORN. llvm-mc rejects it. Severity: low. Report: `pbt-out/bug_reports/encode_orn_extra_operand.md`.

3. **Mixed X/W widths accepted.** `orn w0, w0, x0` encodes using sf from Rd only. llvm-mc rejects it. Severity: medium. Report: `pbt-out/bug_reports/encode_orn_mixed_width.md`.

4. **SP/WSP encoded as ZR.** `orn wsp, w0, w0` assembles as WZR. ARM ARM register 31 is ZR, never SP. Severity: medium. Report: `pbt-out/bug_reports/encode_orn_sp_as_zr.md`.

5. **FP/SIMD names encoded as GPRs.** `orn d0, x1, x2` assembles as `orn x0, x1, x2`. parse_reg_num accepts d/s/q/v/h/b. Severity: medium. Report: `pbt-out/bug_reports/encode_orn_fp_as_gpr.md`.

6. **Out-of-range shift masked.** `orn w0, w0, w0, lsl #32` encodes imm6=32 (UNALLOCATED for sf=0) instead of Err. Severity: medium. Report: `pbt-out/bug_reports/encode_orn_shift_out_of_range.md`.

7. **Unknown shift kind defaults to LSL.** `orn w0, w0, w0, lslx #0` encodes as LSL #0. Severity: low. Report: `pbt-out/bug_reports/encode_orn_unknown_shift_kind.md`.

8. **NEON T other than 8b/16b encoded as 8b.** `orn v0.8h, v0.8h, v0.8h` encodes Q=0 bitwise ORN. llvm-mc rejects it. Severity: medium. Report: `pbt-out/bug_reports/encode_orn_invalid_neon_arr.md`.

9. **Mismatched NEON arrangements / bare Reg sources accepted.** `orn v0.8b, v0.16b, v0.8b` encodes as 8b ORN; Vn/Vm T discarded. Severity: medium. Report: `pbt-out/bug_reports/encode_orn_neon_mismatch.md`.

10. **Trailing operand after a valid shift ignored.** `orn w0, w0, w0, lsl #1, w0` encodes as shifted ORN. Severity: low. Report: `pbt-out/bug_reports/encode_orn_trailing_after_shift.md`.

All ten reproduced serially (`PBT_TEST_JOBS=1`) or are pure-function failures with the same shrunk witness as the parallel run.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_orn_pbt) | 4 KAT + 19 properties + 10 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/FUNCTION_INDEX.md — encode_orn marked yes
- pbt-out/COVERAGE.md — encode_orn row appended
- pbt-out/COVERAGE_STATUS.md — updated
- pbt-out/INVARIANTS.md — encode_orn section prepended
- pbt-out/bug_reports/encode_orn_*.md — 10 bug reports

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 11:17 (campaign: coverage)
> Files: 7/7 scanned (100%) | Functions: 50/229 total | PBT candidates: 50 | Tested: 50 (100%) | 0 pass, 50 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 7 |
| Files scanned | 7 / 7 (100%) |
| Total functions (all files) | 229 |
| PBT candidates (from FUNCTION_INDEX) | 50 |
| **Tested (of PBT candidates)** | **50 / 50 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 50 / 0 |
| **Overall (tested / all functions)** | **50 / 229 (22%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 50 | 50 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 50 | 50 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 16 | 16 | 100% | covered |
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
| encode_orn | data_processing.rs |
