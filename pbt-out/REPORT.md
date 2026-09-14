# PBT Campaign Report: encode_cmp

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_cmp
**Tests:** 11 properties (8 passing, 3 failing) plus 7 passing KATs and 7 failing regression witnesses
**Result:** 8 passing, 6 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cmp | 11 properties | 6 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **extra operand ignored** — `encode_cmp_neg_extra_operand`. Shrunk CE: `[Reg("x0"), Reg("x0"), Reg("x2")]`. Expected Err; actual Ok(Word) encoding `cmp x0, x0`. encode_add_sub only treats operand 3 as Shift/Extend. Severity: medium. Report: `pbt-out/bug_reports/encode_cmp_extra_operand.md`.

2. **XZR/WZR accepted as immediate-form Rn** — `encode_cmp_neg_wrong_reg`. Shrunk CE: kind=0 n=0 imm=0, `[Reg("xzr"), Imm(0)]`. Expected Err; actual Ok(Word(0xf10003ff)) encoding `cmp sp, #0`. WZR similarly encodes as `cmp wsp, #0`. Severity: medium. Report: `pbt-out/bug_reports/encode_cmp_xzr_imm.md`.

3. **mixed x/w accepted** — same property, isolated via regression. CE: `[Reg("x0"), Reg("w0")]`. Expected Err; actual Ok(Word) with sf from prepended XZR and Rm number from w0. Severity: medium. Report: `pbt-out/bug_reports/encode_cmp_mixed_width.md`.

4. **FP/SIMD names accepted as GPRs** — same property, isolated via regression. CE: `[Reg("d0"), Imm(0)]`. Expected Err; actual encodes as `cmp x0, #0`. Severity: medium. Report: `pbt-out/bug_reports/encode_cmp_fp_reg.md`.

5. **SP as Rm encoded as XZR** — same property, isolated via regression. CE: `[Reg("x0"), Reg("sp")]`. Expected Err; actual Ok(Word) with Rm=31 (XZR). Severity: medium. Report: `pbt-out/bug_reports/encode_cmp_sp_as_rm.md`.

6. **Imm(i64::MIN) panics** — `encode_cmp_neg_imm_oor`. Shrunk CE: rn="x0", imm=-9223372036854775808. Expected Err; actual panic `attempt to negate with overflow` at data_processing.rs:314. Severity: high. Report: `pbt-out/bug_reports/encode_cmp_imm_min_overflow.md`.

Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_cmp_neg_ -- --test-threads=1` reproduced all three property failures.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_cmp_pbt) | 11 properties + 7 KAT + 7 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — per-function coverage ledger
- pbt-out/COVERAGE_STATUS.md — aggregate coverage
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/INVARIANTS.md — confirmed invariants (prepended)
- pbt-out/bug_reports/encode_cmp_extra_operand.md
- pbt-out/bug_reports/encode_cmp_xzr_imm.md
- pbt-out/bug_reports/encode_cmp_mixed_width.md
- pbt-out/bug_reports/encode_cmp_fp_reg.md
- pbt-out/bug_reports/encode_cmp_sp_as_rm.md
- pbt-out/bug_reports/encode_cmp_imm_min_overflow.md

Contract-surface sweep: 1 round (standard tier). `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit of encode_cmp's non-Reg-first else-arm, ARM ARM extended-register CMP, and gas negative-imm rewrite. Three targeted properties were added and all passed. Close reason: the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 03:16 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 18/184 total | PBT candidates: 18 | Tested: 18 (100%) | 0 pass, 18 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 18 |
| **Tested (of PBT candidates)** | **18 / 18 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 18 / 0 |
| **Overall (tested / all functions)** | **18 / 184 (10%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 18 | 18 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 18 | 18 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 10 | 10 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 4 | 4 | 100% | covered |
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
