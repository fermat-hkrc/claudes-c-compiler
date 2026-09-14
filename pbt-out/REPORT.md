# PBT Campaign Report: encode_extr

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_extr
**Tests:** 13 properties (8 passing, 5 failing) plus 8 passing KAT gates and 5 failing regression witnesses
**Result:** 8 passing, 5 bugs
**Effort tier:** standard (5–8 properties, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_extr | 13 properties (8 pass / 5 fail) + 8 KAT + 5 regression | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### 1. Extra operand silently ignored
- **Law:** EXTR is four-operand; a fifth operand must Err (llvm-mc: invalid operand).
- **Shrunk counterexample:** `[Reg("w0"), Reg("w0"), Reg("w0"), Imm(0), Reg("x0")]` (`extr w0, w0, w0, #0, x0`)
- **Expected:** Err
- **Actual:** Ok(Word) — same as `extr w0, w0, w0, #0`
- **Root cause:** encode_extr never checks `operands.len()`; get_reg/get_imm only read indices 0..3.
- **Impact:** Trailing garbage is assembled instead of rejected.
- **Severity:** medium
- **Fix:** Reject `operands.len() != 4`.
- **Bug report:** pbt-out/bug_reports/encode_extr_extra_operand.md
- **Serial reconfirmation:** reproduced with `cargo test --lib encode_extr -- --test-threads=1`

### 2. SP/WSP accepted as register 31
- **Law:** ARM EXTR register 31 is ZR not SP; SP/WSP in Rd/Rn/Rm must Err.
- **Shrunk counterexample:** `[Reg("wsp"), Reg("w0"), Reg("w0"), Imm(0)]` (`extr wsp, w0, w0, #0`)
- **Expected:** Err
- **Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31
- **Root cause:** No SP rejection; register 31 is treated as ZR.
- **Impact:** SP operands encode as ZR.
- **Severity:** medium
- **Fix:** Reject SP/WSP in Rd, Rn, and Rm.
- **Bug report:** pbt-out/bug_reports/encode_extr_sp.md
- **Serial reconfirmation:** reproduced serially as above

### 3. Out-of-range #lsb panics or encodes
- **Law:** EXTR requires 0 <= lsb <= 31 (W) / 63 (X). Out-of-range immediates must Err.
- **Shrunk counterexample:** `[Reg("w0"), Reg("w0"), Reg("w0"), Imm(-1)]` (`extr w0, w0, w0, #-1`)
- **Expected:** Err
- **Actual:** panic in debug (`attempt to shift left with overflow` at `lsb << 10` after `-1i64 as u32`). Related: lsb=32 for W encodes Ok(Word) with imms overlapping neighbouring fields.
- **Root cause:** `get_imm(...) as u32` truncates; no ARM range check before shifting into bits[15:10].
- **Impact:** Assembler crash on negative lsb; silent illegal encodings for lsb >= R.
- **Severity:** high
- **Fix:** Reject lsb outside 0..=31 (W) / 0..=63 (X) before packing imms.
- **Bug report:** pbt-out/bug_reports/encode_extr_lsb.md
- **Serial reconfirmation:** reproduced serially as above

### 4. Mixed W/X widths accepted
- **Law:** EXTR requires matching W/W/W or X/X/X; mixed width must Err.
- **Shrunk counterexample:** `[Reg("w0"), Reg("x0"), Reg("w0"), Imm(0)]` (`extr w0, x0, w0, #0`)
- **Expected:** Err
- **Actual:** Ok(Word) — encoded as 32-bit EXTR (sf from Rd only)
- **Root cause:** `let (rn, _) = get_reg(operands, 1)?` and `let (rm, _) = get_reg(operands, 2)?` discard Rn/Rm width.
- **Impact:** Source text and encoding disagree on operand size.
- **Severity:** medium
- **Fix:** Require Rd, Rn, and Rm to have the same GPR width.
- **Bug report:** pbt-out/bug_reports/encode_extr_mixed_width.md
- **Serial reconfirmation:** reproduced serially as above

### 5. FP/SIMD registers accepted as EXTR operands
- **Law:** EXTR operands are GPRs only; `extr d0, x1, x2, #0` must Err.
- **Shrunk counterexample:** `[Reg("d0"), Reg("x1"), Reg("x2"), Imm(0)]` (`extr d0, x1, x2, #0`)
- **Expected:** Err
- **Actual:** Ok(Word) — parse_reg_num accepts 'd' prefix
- **Root cause:** parse_reg_num accepts d/s/q/v/h/b; encode_extr does not call is_fp_reg.
- **Impact:** FP names are silently remapped to GPR encodings.
- **Severity:** medium
- **Fix:** Reject FP/SIMD names in Rd, Rn, and Rm.
- **Bug report:** pbt-out/bug_reports/encode_extr_fp.md
- **Serial reconfirmation:** reproduced serially as above

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/bitfield.rs (mod encode_extr_pbt) | 13 properties + 8 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/FUNCTION_INDEX.md — merged function index (encode_extr now a candidate)
- pbt-out/COVERAGE.md — coverage ledger row for encode_extr
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed encode_extr invariants
- pbt-out/REPORT.md — this report
- pbt-out/bug_reports/encode_extr_extra_operand.md
- pbt-out/bug_reports/encode_extr_sp.md
- pbt-out/bug_reports/encode_extr_lsb.md
- pbt-out/bug_reports/encode_extr_mixed_width.md
- pbt-out/bug_reports/encode_extr_fp.md

Sweep close: coverage_gaps had no LLVM profraw; manual arm audit of encode_extr (arity / extra / SP / mixed W-X / FP / lsb / nonreg / invalid-name / alt-spellings). Added encode_extr_diff_alt_spellings, encode_extr_neg_mixed_width, encode_extr_neg_fp, encode_extr_neg_nonreg, encode_extr_neg_invalid_name. Closed: tier round spent and documented surface covered.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 19:28 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 81/284 total | PBT candidates: 81 | Tested: 81 (100%) | 0 pass, 81 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 284 |
| PBT candidates (from FUNCTION_INDEX) | 81 |
| **Tested (of PBT candidates)** | **81 / 81 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 81 / 0 |
| **Overall (tested / all functions)** | **81 / 284 (29%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 81 | 81 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 81 | 81 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 4 | 5 | 125% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 10 | 10 | 100% | covered |
| neon.rs | 68 | 14 | 14 | 100% | covered |
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
