# PBT Campaign Report: encode_uxtw

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_uxtw
**Tests:** 11 properties + 4 KAT + 5 regression witnesses
**Result:** 8 failing (5 SUT bugs), 3 passing
**Effort tier:** standard (1 coverage-driven sweep round; ≥1000 generator cases; ≥1 metamorphic/differential required)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_uxtw | 8 failing / 3 passing properties + 4 KAT fail + 5 regression fail | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_uxtw emits MOV Wd,Wn (ORR) instead of UBFM Xd,Xn,#0,#31**
   - Law: ARM ARM C6 UXTW is the alias of UBFM Xd, Xn, #0, #31. llvm-mc encodes `uxtw x0, w1` as `0xD3407C20`.
   - Minimal input: `[Reg("x0"), Reg("w1")]` (also shrunk `uxtw x0, w0`)
   - Expected: `Ok(Word(0xD3407C20))`
   - Actual: `Ok(Word(0x2A0103E0))` — 32-bit `ORR Wd, WZR, Wn` (`mov w0, w1`)
   - Root cause: body uses `(0b001010100 << 23) | (rn << 16) | (31 << 5) | rd` instead of `0xD3407C00 | (rn << 5) | rd`
   - Impact: every `uxtw` word disagrees with gas/llvm-mc; disassembly shows MOV not UBFX
   - Severity: high
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_uxtw_mov_not_ubfm.md`
   - Regression: `test_encode_uxtw_regression_mov_not_ubfm` (fails as witness)
   - Also fails: `encode_uxtw_alias_ubfm`, `encode_uxtw_arm_fields`, `encode_uxtw_diff_alt_spellings`, 4 KATs

2. **encode_uxtw silently ignores a 3rd operand**
   - Law: UXTW is 2-operand (`Xd, Wn`); extra operand must Err (llvm-mc `invalid operand`).
   - Minimal input: `[Reg("x0"), Reg("w0"), Reg("x0")]`
   - Expected: `Err`
   - Actual: `Ok(Word)` — `get_reg` only reads indices 0..1
   - Root cause: no `operands.len()` check
   - Impact: typos/extra operands assemble silently
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_uxtw_extra_operand.md`
   - Regression: `test_encode_uxtw_regression_extra_operand` (fails as witness)

3. **encode_uxtw accepts W dest**
   - Law: ARM ARM form is `UXTW <Xd>, <Wn>` only. llvm-mc rejects `uxtw w0, w0`.
   - Minimal input: `[Reg("w0"), Reg("w0")]`
   - Expected: `Err`
   - Actual: `Ok(Word(0x2A0003E0))`; `is_64` from `get_reg` discarded
   - Root cause: `let (rd, _) = get_reg(...)` ignores width
   - Impact: 32-bit dest mnemonic encoded instead of rejected
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_uxtw_wd.md`
   - Regression: `test_encode_uxtw_regression_wd` (fails as witness)

4. **encode_uxtw encodes SP/WSP as XZR/WZR**
   - Law: Bitfield register 31 is ZR, never SP. llvm-mc rejects `uxtw sp, w1` and `uxtw x0, wsp`.
   - Minimal input: `[Reg("wsp"), Reg("w0")]`
   - Expected: `Err`
   - Actual: `Ok(Word)` with that slot as register 31
   - Root cause: `parse_reg_num` maps `sp`/`wsp` to 31 with no SP-vs-ZR check
   - Impact: stack-pointer operands silently rewritten to the zero register
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_uxtw_sp.md`
   - Regression: `test_encode_uxtw_regression_sp` (fails as witness)

5. **encode_uxtw encodes FP/SIMD register names as GPRs**
   - Law: UXTW operands are GPRs. llvm-mc rejects `uxtw d0, w1`.
   - Minimal input: `[Reg("d0"), Reg("w1")]`
   - Expected: `Err`
   - Actual: `Ok(Word)` treating `d0` as GPR 0
   - Root cause: `parse_reg_num` accepts `d`/`s`/`q`/`v`/`h`/`b` prefixes
   - Impact: FP dest/source silently treated as same-numbered GPR
   - Severity: medium
   - Serial reconfirmation: `PBT_TEST_JOBS=1` / `--test-threads=1`
   - Bug report: `pbt-out/bug_reports/encode_uxtw_fp.md`
   - Regression: `test_encode_uxtw_regression_fp` (fails as witness)

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (`mod encode_uxtw_pbt`) | 11 properties + 4 KAT + 5 regression witnesses |

## Output Directories

- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/REPORT.md` — this report
- `pbt-out/COVERAGE.md` — per-function coverage row
- `pbt-out/COVERAGE_STATUS.md` — campaign coverage stats
- `pbt-out/FUNCTION_INDEX.md` — merged function index
- `pbt-out/INVARIANTS.md` — confirmed invariants
- `pbt-out/bug_reports/encode_uxtw_mov_not_ubfm.md`
- `pbt-out/bug_reports/encode_uxtw_extra_operand.md`
- `pbt-out/bug_reports/encode_uxtw_wd.md`
- `pbt-out/bug_reports/encode_uxtw_sp.md`
- `pbt-out/bug_reports/encode_uxtw_fp.md`

## Contract-surface sweep

Tier `standard` owes 1 coverage-driven round. `coverage_gaps` reported no instrumented LLVM profraw in this session. Sweep was a manual arm audit of encode_uxtw (arity / extra / Wd / SP / FP / nonreg / invalid name / alt-spellings / UBFM alias / ARM fields). Added `encode_uxtw_diff_alt_spellings` (fails, same MOV-vs-UBFM bug), `encode_uxtw_neg_nonreg` (passing), `encode_uxtw_neg_invalid_name` (passing). Function is 8 lines with no remaining documented branch without a property. Sweep closed: documented behaviors have properties.

## Skipped targets

(none) — in-scope symbol `encode_uxtw` compiled and ran under `cargo test --lib`.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 14:48 (campaign: coverage)
> Files: 8/8 scanned (100%) | Functions: 64/253 total | PBT candidates: 64 | Tested: 64 (100%) | 0 pass, 64 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 8 |
| Files scanned | 8 / 8 (100%) |
| Total functions (all files) | 253 |
| PBT candidates (from FUNCTION_INDEX) | 64 |
| **Tested (of PBT candidates)** | **64 / 64 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 64 / 0 |
| **Overall (tested / all functions)** | **64 / 253 (25%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 64 | 64 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 64 | 64 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 24 | 24 | 100% | covered |
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
| encode_uxtw | data_processing.rs |
