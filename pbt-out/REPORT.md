# PBT Campaign Report: encode_ldur_stur

## Summary

**Date:** 2026-09-14
**Repository:** claudes-c-compiler
**Modules tested:** encode_ldur_stur
**Tests:** 10 properties (plus 3 KAT + 8 regression witnesses)
**Result:** 6 passing, 7 bugs
**Effort tier:** standard (1 coverage-driven contract-surface sweep; generator runs=1000)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldur_stur | 10 properties (6 pass, 4 fail) + 3 KAT pass + 8 regression fail | 7 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. Extra operands ignored
- **Law:** LDUR/STUR/LDTR/STTR take exactly two operands; a third must be Err.
- **Shrunk counterexample:** `stur w0, [x0, #-256], x2` (rt=0, rn=0, offset=-256, extra=Reg("x2")).
- **Expected:** Err. **Actual:** Ok(Word) — arity check is `operands.len() < 2`.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldur_stur_extra_operand.md
- **Regression test:** `test_encode_ldur_stur_regression_extra_operand` (fails, as intended)

### 2. Out-of-range simm9 silently wraps
- **Law:** Unscaled/unprivileged offset is signed 9-bit in [-256, 255]; llvm-mc rejects values outside that range.
- **Shrunk counterexample:** `stur w0, [x0, #-257]` (rt=0, rn=0, offset=-257). Encodes as `stur w0, [x0, #255]` because `(-257 as u32) & 0x1FF == 0xFF`.
- **Expected:** Err. **Actual:** Ok(Word) with wrapped imm9.
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/encode_ldur_stur_imm9_range.md
- **Regression test:** `test_encode_ldur_stur_regression_imm9_range` (fails, as intended)

### 3. SP/WSP accepted as Rt
- **Law:** Rt is Wt/Xt (31 = ZR), never SP/WSP.
- **Shrunk counterexample:** `stur sp, [x0, #-256]` (kind=0, offset=-256). Encodes as `stur wzr, [x0, #-256]` (0xbc10001f) because `"sp"` does not start with `x` so size=32-bit and parse_reg_num maps SP→31.
- **Expected:** Err. **Actual:** Ok(Word) of STUR WZR.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldur_stur_sp_as_rt.md
- **Regression test:** `test_encode_ldur_stur_regression_sp_as_rt` (fails, as intended)

### 4. W register (or XZR/WZR/WSP) accepted as base
- **Law:** Rn is Xn|SP; W-width bases and XZR/WZR must be Err (register 31 as base is SP, never ZR).
- **Shrunk counterexample:** `ldur x0, [w0]`. Encodes as `ldur x0, [x0]`. Same path treats `[xzr]` as `[sp]`.
- **Expected:** Err. **Actual:** Ok(Word) of the X-width encoding.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldur_stur_w_base.md
- **Regression test:** `test_encode_ldur_stur_regression_w_base` / `test_encode_ldur_stur_regression_xzr_base` (fail, as intended)

### 5. SIMD Rt encoded on LDTR/STTR
- **Law:** LDTR/STTR take Wt/Xt only; llvm-mc rejects `ldtr d0, [x1]`.
- **Shrunk counterexample:** kind=6 SIMD Rt with op2=0b10. Encodes V=1 unprivileged form, which is not a valid instruction.
- **Expected:** Err. **Actual:** Ok(Word) with V=1 and bits [11:10]=10.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldur_stur_simd_ldtr.md
- **Regression test:** `test_encode_ldur_stur_regression_simd_ldtr` (fails, as intended)

### 6. V-register Rt encoded as D
- **Law:** SIMD unscaled Rt is Bt/Ht/St/Dt/Qt; llvm-mc rejects `ldur v0, [x1]`.
- **Shrunk counterexample:** kind=7 `ldur v0, [x0]`. `is_fp_reg('v')` is true, then the q/d/s/h/b chain falls through to size=11 opc=01 — `ldur d0, [x0]`.
- **Expected:** Err. **Actual:** Ok(Word) of LDUR D0.
- **Severity:** medium
- **Bug report:** pbt-out/bug_reports/encode_ldur_stur_v_reg.md
- **Regression test:** `test_encode_ldur_stur_regression_v_reg` (fails, as intended)

### 7. LR encoded as 32-bit W30
- **Law:** `lr` is the alias of X30. llvm-mc accepts `ldur lr, [x0]` as `ldur x30, [x0]`. Siblings `is_64bit_reg` and `encode_ldr_str_auto` treat `lr` as 64-bit.
- **Shrunk counterexample:** `stur lr, [x0, #-256]` (rn=0, offset=-256, is_load=false, unpriv=false). SUT 0xb810001e (`stur w30`) vs llvm-mc 0xf810001e (`stur x30`).
- **Expected:** 64-bit X30 encoding. **Actual:** 32-bit W30 encoding (`starts_with('x')` is false for `"lr"`).
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/encode_ldur_stur_lr_as_w30.md
- **Regression test:** `test_encode_ldur_stur_regression_lr_as_x30` (fails, as intended)

Serial reconfirmation: all failures reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldur_stur -- --test-threads=1`.

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_ldur_stur_pbt) | 10 properties + 3 KAT + 8 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_ldur_stur_extra_operand.md
- pbt-out/bug_reports/encode_ldur_stur_imm9_range.md
- pbt-out/bug_reports/encode_ldur_stur_sp_as_rt.md
- pbt-out/bug_reports/encode_ldur_stur_w_base.md
- pbt-out/bug_reports/encode_ldur_stur_simd_ldtr.md
- pbt-out/bug_reports/encode_ldur_stur_v_reg.md
- pbt-out/bug_reports/encode_ldur_stur_lr_as_w30.md

Contract-surface sweep closed after 1 round (standard tier): `coverage_gaps` had no LLVM profraw; manual arm audit of `len < 2` / get_reg non-Reg / non-Mem second operand / parse_reg_num None (arity/shape property passes) and the `lr` alias (bug 7). Sweep closed because the tier's one round is done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 06:31 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 32/184 total | PBT candidates: 32 | Tested: 32 (100%) | 0 pass, 32 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 32 |
| **Tested (of PBT candidates)** | **32 / 32 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 32 / 0 |
| **Overall (tested / all functions)** | **32 / 184 (17%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 32 | 32 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 32 | 32 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 6 | 6 | 100% | covered |
| load_store.rs | 20 | 3 | 3 | 100% | covered |
| neon.rs | 68 | 4 | 4 | 100% | covered |

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
