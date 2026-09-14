# PBT Campaign Report: encode_fmov

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_fmov
**Tests:** 11 properties (6 passing, 5 failing) plus 9 passing KAT gates and 7 failing regression witnesses
**Result:** 6 passing, 5 bugs
**Effort tier:** standard (5–8 properties, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fmov | 11 properties (6 pass / 5 fail) + 9 KAT + 7 regression | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### 1. Extra operand silently ignored
- **Law:** Scalar FMOV (register/general) is two-operand; a third operand must Err (llvm-mc: invalid operand; gas: unexpected characters).
- **Shrunk counterexample:** `[Reg("s0"), Reg("s0"), Reg("s0")]` (`fmov s0, s0, s0`)
- **Expected:** Err
- **Actual:** Ok(Word) — same as `fmov s0, s0`
- **Root cause:** encode_fmov only checks `operands.len() < 2`.
- **Impact:** Trailing garbage is assembled instead of rejected.
- **Severity:** medium
- **Fix:** Reject `operands.len() != 2`.
- **Bug report:** pbt-out/bug_reports/encode_fmov_extra_operand.md
- **Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_fmov_neg_ -- --test-threads=1`

### 2. Incompatible register classes encoded
- **Law:** FMOV (register) requires matching Sd,Sn / Dd,Dn; FMOV (general) requires matching widths Sd/Wn or Dd/Xn. Mixed S/D, size-mismatched GP/FP, Q/V/B scalar, and two GP registers must Err.
- **Shrunk counterexample:** `[Reg("q0"), Reg("s0")]` (`fmov q0, s0`). Also `s0,d1` and `d0,w1`.
- **Expected:** Err
- **Actual:** Ok(Word) — Q treated as FP with ftype=00; mixed S/D uses OR of d-prefix so `fmov s0, d1` encodes as double; `fmov d0, w1` encodes as `fmov d0, x1`.
- **Root cause:** is_fp_reg includes q/v/h/b; is_double = dest-or-src starts with 'd'; GP-FP path takes sf/ftype only from the FP register.
- **Impact:** Invalid GNU-style assembly becomes the wrong instruction.
- **Severity:** high
- **Fix:** Require matching S/S, D/D, S/W, D/X (and H/H, H/W) and reject Q/V/B/two-GP.
- **Bug report:** pbt-out/bug_reports/encode_fmov_wrong_types.md
- **Serial reconfirmation:** reproduced serially as above

### 3. SP/WSP accepted
- **Law:** FMOV (general) uses ZR not SP at register 31. llvm-mc/gas reject `fmov s0, sp` and `fmov wsp, s0`.
- **Shrunk counterexample:** `[Reg("wsp"), Reg("s0")]` (`fmov wsp, s0`)
- **Expected:** Err
- **Actual:** Ok(Word) for FMOV WZR, S0. `fmov s0, sp` is encoded as FMOV S0, S31 because is_fp_reg("sp") is true (prefix 's').
- **Root cause:** parse_reg_num maps sp/wsp to 31; is_fp_reg does not exclude "sp".
- **Impact:** SP operands encode as ZR or as S31.
- **Severity:** medium
- **Fix:** Reject SP/WSP in either slot; do not treat "sp" as an FP register.
- **Bug report:** pbt-out/bug_reports/encode_fmov_sp.md
- **Serial reconfirmation:** reproduced serially as above

### 4. Half-precision encoded as single (ftype=00)
- **Law:** ARM FMOV (register) uses ftype=11 for Hn (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fmov h0, h0` as 0x1ee04000. gas `-march=armv8.2-a+fp16` agrees (`fmov h0, h1` = 0x1ee04020).
- **Shrunk counterexample:** `[Reg("h0"), Reg("h0")]` (`fmov h0, h0`)
- **Expected:** Word(0x1ee04000)
- **Actual:** Word(0x1e204000) — ftype=00 (FMOV S0, S0)
- **Root cause:** ftype is only 01 when a name starts with 'd', else 00; H is accepted by is_fp_reg/parse_reg_num.
- **Impact:** Half-precision FMOV silently operates on the S view of the same number.
- **Severity:** high
- **Fix:** Set ftype=11 for H registers (and reject mixed H/S, H/D).
- **Bug report:** pbt-out/bug_reports/encode_fmov_half_ftype.md
- **Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_fmov_diff_half -- --test-threads=1`

### 5. FMOV Xd, Vn.D[1] / Vd.D[1], Xn not encoded
- **Law:** ARM FMOV (general) top-half form: FMOV <Xd>, <Vn>.D[1] / FMOV <Vd>.D[1], <Xn> (sf=1, ftype=10, rmode=01, opcode 110/111). llvm-mc/gas encode `fmov x0, v0.d[1]` as 0x9eae0000.
- **Shrunk counterexample:** `[Reg("x0"), RegLane { reg: "v0", elem_size: "d", index: 1 }]`
- **Expected:** Word(0x9eae0000)
- **Actual:** Err("fmov needs register operands")
- **Root cause:** encode_fmov only matches Operand::Reg; RegLane falls through.
- **Impact:** Valid GNU-style assembly that gas accepts cannot be assembled.
- **Severity:** medium
- **Fix:** Encode RegLane D[1] with X as FMOV (general) top-half; reject other lane sizes/indices.
- **Bug report:** pbt-out/bug_reports/encode_fmov_vd1.md
- **Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib test_encode_fmov_regression_vd1 -- --test-threads=1`

## Design Caveats

- FMOV (immediate) (`fmov Sd, #imm` / `fmov Dd, #imm`) is accepted by gas and llvm-mc (`fmov s0, #1.0` = 0x1e2e1000) but the SUT returns Err. **Doc evidence:** fp_scalar.rs:14-15 `// TODO: implement fmov with float immediate encoding` / `return Err("fmov with immediate operand not yet supported")`. Spec-vs-codebase: README.md:11 claims gas-compatible assembly; the TODO asserts the current rejection. Not filed as a SUT bug; the immediate path was excluded from encode_fmov_neg_nonreg so that property does not pin the TODO as a contract.
- `fmov s0, #0.0` is an llvm-mc/gas alias of `fmov s0, wzr` (0.0 is not an 8-bit FP immediate). Same TODO path at the encode_fmov boundary.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/fp_scalar.rs (mod encode_fmov_pbt) | 11 properties + 9 KAT + 7 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_fmov_extra_operand.md
- pbt-out/bug_reports/encode_fmov_wrong_types.md
- pbt-out/bug_reports/encode_fmov_sp.md
- pbt-out/bug_reports/encode_fmov_half_ftype.md
- pbt-out/bug_reports/encode_fmov_vd1.md

Sweep close: `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fmov (nonreg / invalid-name / V.D[1]). Tier round 1/1 spent. Documented immediate form recorded as a Design Caveat rather than a new encode-must-match property.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 19:43 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 82/284 total | PBT candidates: 82 | Tested: 82 (100%) | 0 pass, 82 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 284 |
| PBT candidates (from FUNCTION_INDEX) | 82 |
| **Tested (of PBT candidates)** | **82 / 82 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 82 / 0 |
| **Overall (tested / all functions)** | **82 / 284 (29%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 82 | 82 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 82 | 82 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 5 | 6 | 120% | covered |
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
| encode_fmov | fp_scalar.rs |
