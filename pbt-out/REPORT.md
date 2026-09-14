# PBT Campaign Report: encode_ccmp_ccmn

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ccmp_ccmn
**Tests:** 12 properties (8 passing, 4 failing) plus 3 passing KATs and 6 failing regression witnesses
**Result:** 8 passing, 5 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ccmp_ccmn | 12 properties | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **imm5/nzcv truncated** — `encode_ccmp_ccmn_neg_imm5_nzcv_oor`. Shrunk CE: `[Reg("x0"), Imm(-1), Imm(0), Cond("eq")]`, is_ccmp=false. Expected Err (imm5 in [0,31]); actual Ok(Word) because `*imm5 as u32 & 0x1F` maps #-1 to #31. Same mask on nzcv: `#16` encodes as `#0`. Severity: medium. Report: `pbt-out/bug_reports/encode_ccmp_ccmn_imm5_nzcv_truncated.md`.

2. **extra operand ignored** — `encode_ccmp_ccmn_neg_extra_operand`. Shrunk CE: `[Reg("x0"), Imm(0), Imm(0), Cond("eq"), Reg("x1")]`, is_ccmp=false. Expected Err; actual Ok(Word(0xba400800)). Encoder only inspects operands 0..3. Severity: medium. Report: `pbt-out/bug_reports/encode_ccmp_ccmn_extra_operand.md`.

3. **SP encoded as XZR** — `encode_ccmp_ccmn_neg_wrong_reg_class`. Shrunk CE: `[Reg("sp"), Imm(0), Imm(0), Cond("eq")]`, is_ccmp=false. Expected Err; actual Ok(Word) with Rn=31. parse_reg_num maps sp/wsp to 31. Severity: medium. Report: `pbt-out/bug_reports/encode_ccmp_ccmn_sp_as_zr.md`.

4. **FP/SIMD names accepted as GPRs** — same property, isolated via regression. CE: `[Reg("d0"), Imm(0), Imm(0), Cond("eq")]`, is_ccmp=true. Expected Err; actual encodes as `ccmp w0, #0, #0, eq`. Severity: medium. Report: `pbt-out/bug_reports/encode_ccmp_ccmn_fp_reg.md`.

5. **mixed x/w accepted** — `encode_ccmp_ccmn_neg_mixed_width`. Shrunk CE: `[Reg("w0"), Reg("x0"), Imm(0), Cond("eq")]`, is_ccmp=false. Expected Err; actual Ok(Word) with sf from Rn only. Severity: medium. Report: `pbt-out/bug_reports/encode_ccmp_ccmn_mixed_width.md`.

Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_ccmp_ccmn_neg -- --test-threads=1` reproduced all four property failures.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/compare_branch.rs (mod encode_ccmp_ccmn_pbt) | 12 properties + 3 KAT + 6 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — per-function coverage ledger
- pbt-out/COVERAGE_STATUS.md — aggregate coverage
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/INVARIANTS.md — confirmed invariants (appended)
- pbt-out/bug_reports/encode_ccmp_ccmn_imm5_nzcv_truncated.md
- pbt-out/bug_reports/encode_ccmp_ccmn_extra_operand.md
- pbt-out/bug_reports/encode_ccmp_ccmn_sp_as_zr.md
- pbt-out/bug_reports/encode_ccmp_ccmn_fp_reg.md
- pbt-out/bug_reports/encode_ccmp_ccmn_mixed_width.md

Contract-surface sweep: 1 round (standard tier). `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit of encode_cond None, parse_reg_num None on Rm, and the unsupported-operands fallthrough. Three targeted negative properties were added and all passed. Close reason: the tier's one sweep round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 02:20 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 14/184 total | PBT candidates: 14 | Tested: 14 (100%) | 0 pass, 14 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 14 |
| **Tested (of PBT candidates)** | **14 / 14 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 14 / 0 |
| **Overall (tested / all functions)** | **14 / 184 (8%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 14 | 14 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 14 | 14 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 6 | 6 | 100% | covered |
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
