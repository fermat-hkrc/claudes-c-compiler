# PBT Campaign Report: encode_umaddl

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_umaddl
**Tests:** 12 properties (8 passing, 4 failing) plus 4 passing KAT gates and 4 failing regression witnesses
**Result:** 8 passing, 4 bugs
**Effort tier:** standard (1 coverage-driven sweep round; generator runs = 1000; ≥1 metamorphic required)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_umaddl | 12 properties (8 pass / 4 fail) + 4 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_umaddl silently ignores a 5th operand**
   - Law: UMADDL is four-operand (`Xd, Wn, Wm, Xa`); extra operand must Err (llvm-mc: invalid operand).
   - Shrunk input: `[Reg("x0"), Reg("w0"), Reg("w0"), Reg("x0"), Reg("x0")]`
   - Expected: `Err`
   - Actual: `Ok(Word(0x9ba00000))` — `get_reg` only reads indices 0..3
   - Root cause: no arity-upper-bound check
   - Impact: typos / extra operands assemble silently
   - Severity: medium
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`
   - Bug report: `pbt-out/bug_reports/encode_umaddl_extra_operand.md`

2. **encode_umaddl accepts W dest, X sources, and W accumulator**
   - Law: ARM ARM form is `UMADDL <Xd>, <Wn>, <Wm>, <Xa>` only; other widths must Err.
   - Shrunk input: `[Reg("w0"), Reg("w0"), Reg("w0"), Reg("w0")]`
   - Expected: `Err`
   - Actual: `Ok(Word(0x9ba00000))` — `is_64` discarded, sf forced to 1
   - Root cause: `let (rd, _) = get_reg(...)` discards width
   - Impact: 32-bit dest/acc mnemonics emit the 64-bit long-multiply-add encoding
   - Severity: medium
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`
   - Bug report: `pbt-out/bug_reports/encode_umaddl_wrong_width.md`

3. **encode_umaddl encodes SP/WSP as XZR/WZR**
   - Law: register 31 is XZR/WZR, never SP/WSP; llvm-mc rejects sp/wsp.
   - Shrunk input: `[Reg("wsp"), Reg("w0"), Reg("w0"), Reg("x0")]`
   - Expected: `Err`
   - Actual: `Ok(Word(0x9ba0001f))` — slot encoded as register 31
   - Root cause: `parse_reg_num` maps `sp`/`wsp` to 31
   - Impact: stack-pointer operands silently become the zero register
   - Severity: medium
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`
   - Bug report: `pbt-out/bug_reports/encode_umaddl_sp_as_zr.md`

4. **encode_umaddl encodes FP/SIMD register names as GPRs**
   - Law: UMADDL operands are GPRs; llvm-mc rejects `d`/`s`/`q`/`v`/`h`/`b` names.
   - Shrunk input: `[Reg("d0"), Reg("w1"), Reg("w2"), Reg("x3")]`
   - Expected: `Err`
   - Actual: `Ok(Word(0x9ba20c20))` — encoded as `umaddl x0, w1, w2, x3`
   - Root cause: `parse_reg_num` accepts FP/SIMD prefixes and returns the numeric suffix
   - Impact: a floating-point dest/source/acc is silently treated as the same-numbered GPR
   - Severity: medium
   - Serial reconfirmation: reproduced with `PBT_TEST_JOBS=1`
   - Bug report: `pbt-out/bug_reports/encode_umaddl_fp_as_gpr.md`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (`mod encode_umaddl_pbt`) | 12 properties + 4 KAT + 4 regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — coverage ledger row for encode_umaddl
- `pbt-out/COVERAGE_STATUS.md` — coverage statistics
- `pbt-out/FUNCTION_INDEX.md` — merged function index (encode_umaddl marked yes)
- `pbt-out/INVARIANTS.md` — confirmed invariants for encode_umaddl
- `pbt-out/bug_reports/encode_umaddl_extra_operand.md`
- `pbt-out/bug_reports/encode_umaddl_wrong_width.md`
- `pbt-out/bug_reports/encode_umaddl_sp_as_zr.md`
- `pbt-out/bug_reports/encode_umaddl_fp_as_gpr.md`

## Sweep

Contract-surface sweep round 1: `coverage_gaps` reported no LLVM profraw in this session. Manual arm audit of `encode_umaddl` + `get_reg`/`parse_reg_num` as used by it (arity, extra operand, X/W width, SP/WSP, FP prefixes, non-Reg, invalid names, x31/XZR/LR/uppercase). Strengthening round added alt-spellings (pass), FP (fail/bug), nonreg (pass), invalid-name (pass). Close reason: the tier's 1 coverage-driven round is done and every documented behavior has a property.

## Results summary (session)

Results: 8 properties passed, 4 failed (plus 4 KAT pass, 4 regression witnesses fail as designed)

Failing tests:
- encode_umaddl_neg_extra_operand: SUT bug — 5th operand ignored
- encode_umaddl_neg_wrong_width: SUT bug — W dest / X sources / W acc encoded
- encode_umaddl_neg_sp: SUT bug — SP/WSP encoded as ZR
- encode_umaddl_neg_fp: SUT bug — FP/SIMD names encoded as GPRs

SUT observations (not bugs, but notable):
- UMADDL with Ra=XZR is encoded identically to UMULL (ARM ARM alias; llvm-mc disassembles it as umull).
- `lr` in dest or acc encodes as X30 and matches llvm-mc.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 13:50 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 60/253 total | PBT candidates: 60 | Tested: 60 (100%) | 0 pass, 60 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 60 |
| **Tested (of PBT candidates)** | **60 / 60 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 60 / 0 |
| **Overall (tested / all functions)** | **60 / 253 (24%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 60 | 60 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 60 | 60 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 21 | 21 | 100% | covered |
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
| encode_umaddl | data_processing.rs |
