# PBT Campaign Report: encode_sbfx

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_sbfx
**Tests:** 13 properties (plus 6 KAT + 5 regression witnesses)
**Result:** 8 passing, 5 bugs
**Effort tier:** standard (5-8 properties/target, ≥1000 cases, 1 strengthening round, 1 coverage_gaps sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_sbfx | 13 properties (8 passing / 5 failing) + 6 KAT + 5 regression | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operand ignored
- **Law:** SBFX takes exactly four operands; a fifth must be rejected.
- **Shrunk input:** `[Reg("w0"), Reg("w0"), Imm(0), Imm(1), Reg("x0")]`
- **Expected:** Err
- **Actual:** Ok(Word) — indices 0..3 only
- **Root cause:** encode_sbfx never checks `operands.len()`
- **Impact:** invalid GNU-style assembly encodes instead of error
- **Severity:** medium
- **Fix:** reject `operands.len() != 4`
- **Bug report:** pbt-out/bug_reports/encode_sbfx_extra_operand.md

### 2. SP/WSP encoded as ZR
- **Law:** Register 31 is WZR/XZR, not SP/WSP.
- **Shrunk input:** `sbfx wsp, w0, #0, #1` (which=0, sp64=false, is_64=false, other=0)
- **Expected:** Err
- **Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31
- **Root cause:** no SP-vs-ZR distinction after parse_reg_num
- **Impact:** `sbfx sp, ...` silently becomes `sbfx xzr, ...`
- **Severity:** medium
- **Fix:** reject sp/wsp in Rd/Rn
- **Bug report:** pbt-out/bug_reports/encode_sbfx_sp.md

### 3. Out-of-range #lsb/#width panics or encodes
- **Law:** 0 <= lsb < R and 1 <= width <= R-lsb; invalid immediates must Err, not panic.
- **Shrunk input:** `sbfx w0, w0, #0, #0` (lsb=0, width=0, is_64=false)
- **Expected:** Err
- **Actual:** debug panic `attempt to subtract with overflow` at `lsb + width - 1` (bitfield.rs:30)
- **Root cause:** no range check; `u32` wrapping arithmetic
- **Impact:** assembler crash on valid-looking but illegal extract; other out-of-range pairs encode illegal immr/imms
- **Severity:** high
- **Fix:** reject width < 1, lsb >= R, width > R-lsb before computing imms
- **Bug report:** pbt-out/bug_reports/encode_sbfx_lsb_width.md

### 4. Mixed W/X accepted
- **Law:** SBFX requires same-width GPR pair (Wd,Wn or Xd,Xn).
- **Shrunk input:** `sbfx x0, w0, #0, #1` (rd=0, rn=0, rd64=true, rn64=false)
- **Expected:** Err
- **Actual:** Ok(Word) using Rd's sf and Rn's number
- **Root cause:** `let (rn, _) = get_reg(operands, 1)` discards Rn width
- **Impact:** mixed-width assembly encodes a same-width SBFM word
- **Severity:** medium
- **Fix:** require Rn width == Rd width
- **Bug report:** pbt-out/bug_reports/encode_sbfx_mixed_width.md

### 5. FP/SIMD registers accepted as GPR
- **Law:** SBFX Rd/Rn must be W/X (or ZR), not S/D/Q/V/H/B.
- **Shrunk input:** `sbfx d0, x1, #0, #1` (which=0, prefix="d", n=0)
- **Expected:** Err
- **Actual:** Ok(Word) — parse_reg_num maps d0 to register 0
- **Root cause:** encode_sbfx does not require a GPR prefix
- **Impact:** `sbfx d0, ...` encodes as `sbfx w0/x0, ...`
- **Severity:** medium
- **Fix:** reject FP/SIMD prefixes
- **Bug report:** pbt-out/bug_reports/encode_sbfx_fp.md

All five failures reproduced serially with `PBT_TEST_JOBS=1`. They are SUT bugs, not test-isolation defects.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_sbfx_pbt) | 13 properties + 6 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_sbfx_extra_operand.md
- pbt-out/bug_reports/encode_sbfx_sp.md
- pbt-out/bug_reports/encode_sbfx_lsb_width.md
- pbt-out/bug_reports/encode_sbfx_mixed_width.md
- pbt-out/bug_reports/encode_sbfx_fp.md

## Sweep

Round 1/1: `coverage_gaps` had no LLVM profraw. Manual arm audit of encode_sbfx (get_reg Rd/Rn, get_imm lsb/width, sf/N, immr=lsb, imms=lsb+width-1 overflow, extra operands). Added encode_sbfx_diff_alt_spellings, encode_sbfx_neg_nonreg, encode_sbfx_neg_invalid_name (passing) and encode_sbfx_neg_mixed_width, encode_sbfx_neg_fp (failing, filed). Closed: tier round spent and documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 21:51 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 92/289 total | PBT candidates: 92 | Tested: 92 (100%) | 0 pass, 92 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 92 |
| **Tested (of PBT candidates)** | **92 / 92 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 92 / 0 |
| **Overall (tested / all functions)** | **92 / 289 (32%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 92 | 92 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 92 | 92 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 6 | 7 | 117% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 10 | 10 | 100% | covered |
| neon.rs | 68 | 14 | 14 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_sbfx | bitfield.rs |
| encode_sbfm | bitfield.rs |
| encode_sbfiz | bitfield.rs |
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
| encode_ldaxr_stlxr | load_store.rs |
| encode_ldrsw | load_store.rs |
| encode_ldtr_sized | load_store.rs |
| encode_prfm | load_store.rs |
| encode_smulh | data_processing.rs |
| encode_fcvt_rounding | fp_scalar.rs |
| encode_fp_1src | fp_scalar.rs |
| encode_int_to_float | fp_scalar.rs |
| encode_fcmp | fp_scalar.rs |
| encode_fcvt_precision | fp_scalar.rs |
| encode_neon_aes | neon.rs |
| encode_bfi | bitfield.rs |
| encode_bfxil | bitfield.rs |
| encode_cas | load_store.rs |
| encode_cls | bitfield.rs |
| encode_clz | bitfield.rs |
| encode_extr | bitfield.rs |
| encode_fmov | fp_scalar.rs |
| encode_fp_arith | fp_scalar.rs |
| encode_rbit | bitfield.rs |
| encode_rev | bitfield.rs |
| encode_rev16 | bitfield.rs |
| encode_rev32 | bitfield.rs |
| encode_ubfiz | bitfield.rs |
| encode_bfm | bitfield.rs |
