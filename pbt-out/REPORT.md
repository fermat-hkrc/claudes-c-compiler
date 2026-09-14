# PBT Campaign Report: encode_umull

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_umull
**Tests:** 12 properties + 4 KAT + 4 regression witnesses
**Result:** 4 failing (SUT bugs), 8 passing
**Effort tier:** standard (1 coverage-driven sweep round; ≥1000 generator cases; ≥1 metamorphic/differential required)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_umull | 4 failing / 8 passing properties + 4 KAT pass + 4 regression fail | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_umull silently ignores a 4th operand**
   - Law: UMULL is 3-operand (`Xd, Wn, Wm`); extra operand must Err (llvm-mc `invalid operand`).
   - Minimal input: `[Reg("x0"), Reg("w0"), Reg("w0"), Reg("x0")]`
   - Expected: `Err`
   - Actual: `Ok(Word(0x9ba07c00))` — `get_reg` only reads indices 0..2
   - Root cause: no `operands.len()` check
   - Impact: typos/extra operands assemble silently
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `RUST_TEST_THREADS=1`
   - Bug report: `pbt-out/bug_reports/encode_umull_extra_operand.md`
   - Regression: `test_encode_umull_regression_extra_operand` (fails as witness)

2. **encode_umull accepts W dest and X sources**
   - Law: ARM ARM form is `UMULL <Xd>, <Wn>, <Wm>` only. llvm-mc rejects `umull w0, w0, w0`.
   - Minimal input: `[Reg("w0"), Reg("w0"), Reg("w0")]`
   - Expected: `Err`
   - Actual: `Ok(Word(...))` with sf forced to 1; `is_64` from `get_reg` discarded
   - Root cause: `let (rd, _) = get_reg(...)` ignores width
   - Impact: 32-bit dest mnemonic encoded as 64-bit unsigned long multiply
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `RUST_TEST_THREADS=1`
   - Bug report: `pbt-out/bug_reports/encode_umull_wrong_width.md`
   - Regression: `test_encode_umull_regression_wrong_width` (fails as witness)

3. **encode_umull encodes SP/WSP as XZR/WZR**
   - Law: 3-source register 31 is ZR, never SP. llvm-mc rejects `umull sp, w1, w2`.
   - Minimal input: `[Reg("wsp"), Reg("w0"), Reg("w0")]`
   - Expected: `Err`
   - Actual: `Ok(Word(...))` with that slot as register 31
   - Root cause: `parse_reg_num` maps `sp`/`wsp` to 31 with no SP-vs-ZR check
   - Impact: stack-pointer operands silently rewritten to the zero register
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `RUST_TEST_THREADS=1`
   - Bug report: `pbt-out/bug_reports/encode_umull_sp_as_zr.md`
   - Regression: `test_encode_umull_regression_sp` (fails as witness)

4. **encode_umull encodes FP/SIMD register names as GPRs**
   - Law: Scalar UMULL operands are GPRs. llvm-mc rejects `umull d0, w1, w2`.
   - Minimal input: `[Reg("d0"), Reg("w1"), Reg("w2")]`
   - Expected: `Err`
   - Actual: `Ok(Word(...))` treating `d0` as GPR 0
   - Root cause: `parse_reg_num` accepts `d`/`s`/`q`/`v`/`h`/`b` prefixes
   - Impact: FP dest/source silently treated as same-numbered GPR
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `RUST_TEST_THREADS=1`
   - Bug report: `pbt-out/bug_reports/encode_umull_fp_as_gpr.md`
   - Regression: `test_encode_umull_regression_fp` (fails as witness)

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (`mod encode_umull_pbt`) | 12 properties + 4 KAT + 4 regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — coverage ledger row for encode_umull
- `pbt-out/COVERAGE_STATUS.md` — campaign coverage stats
- `pbt-out/FUNCTION_INDEX.md` — encode_umull marked PBT candidate
- `pbt-out/INVARIANTS.md` — confirmed encode_umull invariants
- `pbt-out/bug_reports/encode_umull_extra_operand.md`
- `pbt-out/bug_reports/encode_umull_wrong_width.md`
- `pbt-out/bug_reports/encode_umull_sp_as_zr.md`
- `pbt-out/bug_reports/encode_umull_fp_as_gpr.md`

## Sweep close-out

Contract-surface sweep (standard tier, 1 round): `coverage_gaps` reported no LLVM profraw in this session. Manual arm audit of `encode_umull` added SP / FP / non-reg / invalid-name properties. Non-reg and invalid-name pass; SP and FP fail as additional bugs. Sweep closed: documented error-path surface has a property.

## Results (session)

Results: 4 failed, 8 passed (plus 4 KAT passed, 4 regression witnesses failed as intended)

Failing tests:
- encode_umull_neg_extra_operand: SUT bug — extra operand ignored — pbt-out/bug_reports/encode_umull_extra_operand.md
- encode_umull_neg_wrong_width: SUT bug — W dest / X sources accepted — pbt-out/bug_reports/encode_umull_wrong_width.md
- encode_umull_neg_sp: SUT bug — SP/WSP encoded as ZR — pbt-out/bug_reports/encode_umull_sp_as_zr.md
- encode_umull_neg_fp: SUT bug — FP/SIMD names encoded as GPRs — pbt-out/bug_reports/encode_umull_fp_as_gpr.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 14:36 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 63/253 total | PBT candidates: 63 | Tested: 63 (100%) | 0 pass, 63 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 63 |
| **Tested (of PBT candidates)** | **63 / 63 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 63 / 0 |
| **Overall (tested / all functions)** | **63 / 253 (25%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 63 | 63 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 63 | 63 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 23 | 23 | 100% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 13 | 13 | 100% | covered |
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
| encode_umaddl | data_processing.rs |
| encode_umulh | data_processing.rs |
| encode_neon_rbit | neon.rs |
| encode_umull | data_processing.rs |
