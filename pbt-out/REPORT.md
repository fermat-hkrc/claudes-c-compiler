# PBT Campaign Report: encode_svc

## Summary

**Verdict:** 2 medium: encode_svc ignores extra operands (`svc #0, x0` encodes as `svc #0`) and masks out-of-range immediates (`svc #-1` encodes as `svc #65535`, `svc #65536` as `svc #0`), so a mistyped syscall number or trailing operand is assembled instead of rejected.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_svc
**Tests:** 7 properties (plus 3 KAT + 3 regression witnesses)
**Result:** 5 passing, 2 failing properties, 2 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo tests executed encode_svc. Sweep was a manual arm audit of valid-imm/layout/isolation/empty/wrong-kind/extra/oob. Closed: tier round spent.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_svc | 7 properties | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_svc ignores extra operands

**Formal:** ∀ imm ∈ 0..=65535, extra ∈ Operand. llvm-mc("svc #"+imm+", "+asm(extra))=Err ⇒ encode_svc([Imm(imm), extra])=Err
**Contract evidence:** inferred (ARM ARM SVC is one immediate; llvm-mc rejects extra; gas "unexpected characters following instruction"; README.md:12 gas-compatible assembly; encoder/mod.rs:977 passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_svc([Imm(0), Reg("x0")])  (`svc #0, x0`)
**Expected / Actual:** Err / Ok(Word(0xd4000001))
**Impact:** Typos such as `svc #0, x0` assemble as a silent `svc #0`. An extra operand that should have been an encode error is dropped.
**Root cause:** system.rs:390 reads only operand 0 via get_imm; `operands.len()` is never checked, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:390`
```rust
    let imm = get_imm(operands, 0)?;
```
**Suggested fix:** Reject a slice longer than one operand before encoding.
```rust
    if operands.len() != 1 {
        return Err("svc: expected a single immediate".to_string());
    }
    let imm = get_imm(operands, 0)?;
```
**Bug report:** bug_reports/encode_svc_extra_operand.md
**Repro seed:** cc 3242536d092cbc6bad08edb03332c11384a2aa554ca335f57fa34449697115eb
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_svc_pbt::encode_svc_neg_extra' (2534251) panicked at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:218:1:
Test failed: extra operand must Err (llvm-mc rejects svc #0, x0) at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:270.
minimal failing input: imm = 0, extra = Reg(
    "x0",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_svc masks immediates outside 0..=65535 instead of rejecting

**Formal:** ∀ imm ∈ ℤ \ [0, 65535]. llvm-mc("svc #"+imm)=Err ⇒ encode_svc([Imm(imm)])=Err
**Contract evidence:** inferred (ARM ARM SVC imm16 ∈ 0..=65535; llvm-mc "immediate must be an integer in range [0, 65535]"; gas "immediate value out of range 0 to 65535"; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_svc([Imm(-1)])  (`svc #-1`)
**Expected / Actual:** Err / Ok(Word(0xd41fffe1))
**Impact:** `svc #-1` encodes as `svc #65535`; `svc #65536` encodes as `svc #0`. An out-of-range supervisor-call immediate silently wraps, so the assembled instruction invokes the wrong syscall number.
**Root cause:** system.rs:391 `let word = 0xd4000001 | ((imm as u32 & 0xFFFF) << 5);` truncates instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:391`
```rust
    let word = 0xd4000001 | ((imm as u32 & 0xFFFF) << 5);
```
**Suggested fix:** Reject immediates outside 0..=65535.
```rust
    let imm = get_imm(operands, 0)?;
    if !(0..=65535).contains(&imm) {
        return Err("svc: immediate must be in 0..=65535".to_string());
    }
    let word = 0xd4000001 | ((imm as u32) << 5);
```
**Bug report:** bug_reports/encode_svc_oob_imm.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_svc_pbt::encode_svc_neg_oob_imm' (2534286) panicked at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:218:1:
Test failed: imm -1 outside 0..=65535 must Err (llvm-mc rejects svc #-1) at src/backend/arm/assembler/encoder/encode_svc_pbt.rs:286.
minimal failing input: imm = -1
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_svc_pbt.rs | 7 properties + 3 KAT + 3 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_svc_pbt` registration |

## Reproduction

Whole suite (serial; 5 properties pass, 2 fail, 3 KAT pass, 3 regressions fail):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_svc -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_svc_regression_extra_x0 -- --test-threads=1 --nocapture
```

B2 oob imm (`#-1` and `#65536`):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_svc_regression_imm_neg1 -- --test-threads=1 --nocapture
cargo test --lib test_encode_svc_regression_imm_65536 -- --test-threads=1 --nocapture
```

All of the above ran with `PBT_TEST_JOBS=1` / `--test-threads=1` (serial). Failures reproduced serially.

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_svc_extra_operand.md
- pbt-out/bug_reports/encode_svc_extra_operand.html
- pbt-out/bug_reports/encode_svc_oob_imm.md
- pbt-out/bug_reports/encode_svc_oob_imm.html
- pbt-out/run/encode_svc_round1.log
- pbt-out/run/encode_svc_regression.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 00:50 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 153/307 total | PBT candidates: 153 | Tested: 153 (100%) | 0 pass, 153 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 153 |
| **Tested (of PBT candidates)** | **153 / 153 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 153 / 0 |
| **Overall (tested / all functions)** | **153 / 307 (50%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 153 | 153 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 153 | 153 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 11 | 11 | 100% | covered |
| neon.rs | 68 | 63 | 63 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_ubfx | bitfield.rs |
| encode_ubfm | bitfield.rs |
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
| encode_neon_float_two_misc | neon.rs |
| encode_fabs | fp_scalar.rs |
| encode_fmadd_fmsub | fp_scalar.rs |
| encode_fneg | fp_scalar.rs |
| encode_fsqrt | fp_scalar.rs |
| encode_neon_dup | neon.rs |
| encode_ldrs | load_store.rs |
| encode_neon_ldnr | neon.rs |
| encode_neon_ld1r | neon.rs |
| encode_neon_ld_st_single | neon.rs |
| encode_neon_ld_st_multi | neon.rs |
| encode_neon_tbx | neon.rs |
| encode_neon_ins | neon.rs |
| encode_neon_umov | neon.rs |
| encode_neon_ext | neon.rs |
| encode_neon_movi | neon.rs |
| encode_neon_mvni | neon.rs |
| encode_cnt | neon.rs |
| encode_neon_not | neon.rs |
| encode_neon_rev64 | neon.rs |
| encode_neon_bsl | neon.rs |
| encode_neon_addv | neon.rs |
| encode_neon_across | neon.rs |
| encode_neon_zip_uzp | neon.rs |
| encode_neon_eor3 | neon.rs |
| encode_neon_pmull | neon.rs |
| encode_neon_add_sub | neon.rs |
| encode_neon_ushr | neon.rs |
| encode_neon_sshr | neon.rs |
| encode_neon_shl | neon.rs |
| encode_neon_sri | neon.rs |
| encode_neon_shrn | neon.rs |
| encode_neon_two_misc | neon.rs |
| encode_neon_xtl | neon.rs |
| encode_neon_mul | neon.rs |
| encode_neon_pmul | neon.rs |
| encode_neon_mla | neon.rs |
| encode_neon_mls | neon.rs |
| encode_neon_three_same | neon.rs |
| encode_neon_three_diff | neon.rs |
| encode_neon_logical | neon.rs |
| encode_neon_cmp_zero | neon.rs |
| encode_neon_elem_long | neon.rs |
| encode_neon_elem | neon.rs |
| encode_neon_float_elem | neon.rs |
| encode_neon_fcvtl | neon.rs |
| encode_neon_fcvtn | neon.rs |
| encode_neon_bitwise_insert | neon.rs |
| encode_neon_faddp | neon.rs |
| encode_neon_scalar_three_same | neon.rs |
| encode_neon_scalar_addp | neon.rs |
| encode_neon_scalar_two_misc | neon.rs |
| encode_neon_scalar_qshrn | neon.rs |
| encode_neon_two_misc_narrow | neon.rs |
| encode_dmb | system.rs |
| encode_dsb | system.rs |
| encode_mrs | system.rs |
| encode_msr | system.rs |
| encode_svc | system.rs |
