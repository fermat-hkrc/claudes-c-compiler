# PBT Campaign Report: encode_ldop

## Summary

**Verdict:** 4 medium: encode_ldop silently accepts extra operands, SP/WSP as Rs/Rt (and W/XZR as base), mixed W/X plus FP and X-on-byte/half, and nonzero Mem offsets that llvm-mc/gas reject, so GNU-style assembly that should fail is encoded as a valid LSE atomic.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldop
**Tests:** 11
**Result:** 7 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_ldop NOT LINKED). Manual arm audit of the 45-line body plus three sweep properties.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldop | 11 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_ldop ignores a fourth operand

**Formal:** ∀ op,suf,rs,rt,rn,is_64 in the valid domain, ∀ extra ∈ Operand. encode_ldop(v, valid_ops ++ [extra]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc/gas reject a fourth operand; encode_instruction passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ldop("ldadd", [Reg("w0"), Reg("w0"), Mem{base:"x0", offset:0}, Reg("x2")])
**Expected / Actual:** Err / Ok(Word) — extra operand ignored
**Impact:** `ldadd w0, w0, [x0], x2` is assembled as `ldadd w0, w0, [x0]`. Callers that pass a trailing operand get a silent wrong encoding instead of an assembler error
**Root cause:** load_store.rs:883 checks `operands.len() < 3` and never rejects `len() > 3`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:883`
```rust
    if operands.len() < 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
```
**Suggested fix:** Require exactly three operands
```rust
    if operands.len() != 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
```
**Bug report:** bug_reports/encode_ldop_extra_operand.md
**Repro seed:** cc 573a81fb157a3eb187d6bf5fa7a9a68da01c3f930beda93337d353f6208238b3
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:830:5:
ldadd w0, w0, [x0], x2 must Err; llvm-mc/gas reject a 4th operand
```

### B2: encode_ldop accepts SP/WSP as Rs or Rt and W/XZR as base

**Formal:** ∀ op,suf in the documented variants, ∀ n ∈ [0,30]. encode_ldop(v, [Reg(sp|wsp), …]) is Err ∧ encode_ldop(v, […, Mem{xzr|x31|wN|wsp|wzr, 0}]) is Err
**Contract evidence:** inferred (ARM ARM Rs/Rt are ZR not SP; Rn is Xn|SP not ZR; llvm-mc/gas reject)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ldop("ldadd", [Reg("sp"), Reg("w1"), Mem{base:"x2", offset:0}])
**Expected / Actual:** Err / Ok(Word) — SP encoded as register 31 (ZR)
**Impact:** `ldadd sp, w1, [x2]` is assembled as `ldadd wzr, w1, [x2]`. The stack pointer is silently rewritten as the zero register
**Root cause:** load_store.rs:886-887 call get_reg, and parse_reg_num maps "sp"/"wsp" to 31 with no LDADD-specific rejection of SP
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:886`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP/WSP as Rs and Rt
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
    if is_sp_name(operands, 0) || is_sp_name(operands, 1) {
        return Err("ldop: Rs/Rt cannot be SP".to_string());
    }
```
**Bug report:** bug_reports/encode_ldop_sp_as_rs.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_sp_as_rs' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:846:5:
ldadd sp, w1, [x2] must Err; llvm-mc/gas reject SP as Rs
```

### B3: encode_ldop accepts mixed W/X, FP registers, and X registers on byte/half variants

**Formal:** ∀ n ∈ [0,30], ∀ fp ∈ {b,h,s,d,q,v}. encode_ldop("ldadd", [Reg(xN), Reg(wN), Mem{xN',0}]) is Err ∧ encode_ldop("ldadd", [Reg(fpN), …]) is Err ∧ encode_ldop("ldaddb"|"ldaddh"|"ldaddab"|"ldsetlh", [Reg(xN), Reg(xN'), Mem{xN'',0}]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc/gas reject mixed W/X, FP, and X-on-byte/half)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ldop("ldadd", [Reg("x0"), Reg("w0"), Mem{base:"x1", offset:0}])
**Expected / Actual:** Err / Ok(Word) — size from Rs; Rt width ignored
**Impact:** Mixed-width, FP, and `ldaddb x0, x1, [x2]` assemble instead of failing, producing encodings llvm-mc/gas refuse
**Root cause:** load_store.rs:886-887 take is_64 only from Rs and discard Rt's width; parse_reg_num accepts FP prefixes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:886`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require matching GPR widths and reject FP prefixes
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, rt_64) = get_reg(operands, 1)?;
    if is_64 != rt_64 {
        return Err("ldop: Rs and Rt must be the same width".to_string());
    }
```
**Bug report:** bug_reports/encode_ldop_mixed_width.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_mixed_width' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:894:5:
ldadd x0, w0, [x1] must Err; llvm-mc/gas reject mixed W/X
```

### B4: encode_ldop ignores a nonzero Mem offset

**Formal:** ∀ op,suf,rs,rt in the valid domain, ∀ rn ∈ [0,30], ∀ off ∈ ℤ\{0}. encode_ldop(v, [Reg(gpr(rs)), Reg(gpr(rt)), Mem{base(rn), off}]) is Err
**Contract evidence:** inferred (ARM optional offset only #0; llvm-mc/gas reject [Xn, #1])
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ldop("ldadd", [Reg("w0"), Reg("w0"), Mem{base:"x0", offset:-1}])
**Expected / Actual:** Err / Ok(Word) — offset ignored
**Impact:** `ldadd w0, w0, [x0, #-1]` is assembled as `ldadd w0, w0, [x0]`. A nonzero offset is silently dropped
**Root cause:** load_store.rs:888-889 matches `Operand::Mem { base, .. }` and never inspects offset
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:888`
```rust
    let rn = match operands.get(2) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("ldop: invalid base")?,
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
```
**Suggested fix:** Reject a nonzero offset
```rust
    let rn = match operands.get(2) {
        Some(Operand::Mem { base, offset }) if *offset == 0 => {
            parse_reg_num(base).ok_or("ldop: invalid base")?
        }
        Some(Operand::Mem { .. }) => return Err(format!("{}: offset must be #0", mnemonic)),
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
```
**Bug report:** bug_reports/encode_ldop_nonzero_offset.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_ldop_pbt::test_encode_ldop_regression_nonzero_offset' panicked at src/backend/arm/assembler/encoder/encode_ldop_pbt.rs:942:5:
ldadd w0, w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_ldop_pbt.rs | 11 properties + 11 KAT + 8 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_ldop_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_ldop -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_extra_operand -- --test-threads=1
```

B2 SP as Rs:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_sp_as_rs -- --test-threads=1
```

B3 mixed width:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_mixed_width -- --test-threads=1
```

B4 nonzero offset:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldop_regression_nonzero_offset -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/report.json
- pbt-out/FUNCTION_INDEX.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/bug_reports/encode_ldop_extra_operand.md
- pbt-out/bug_reports/encode_ldop_extra_operand.html
- pbt-out/bug_reports/encode_ldop_sp_as_rs.md
- pbt-out/bug_reports/encode_ldop_sp_as_rs.html
- pbt-out/bug_reports/encode_ldop_mixed_width.md
- pbt-out/bug_reports/encode_ldop_mixed_width.html
- pbt-out/bug_reports/encode_ldop_nonzero_offset.md
- pbt-out/bug_reports/encode_ldop_nonzero_offset.html
- pbt-out/run/encode_ldop_test.log
- pbt-out/run/encode_ldop_test2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 04:59 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 164/307 total | PBT candidates: 164 | Tested: 164 (100%) | 1 pass, 164 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 164 |
| **Tested (of PBT candidates)** | **164 / 164 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 164 / -1 |
| **Overall (tested / all functions)** | **164 / 307 (53%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 164 | 164 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 164 | 164 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 13 | 13 | 100% | covered |
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
| encode_hvc | system.rs |
| encode_brk | system.rs |
| encode_hint | system.rs |
| encode_bti | system.rs |
| encode_ic | system.rs |
| encode_dc | system.rs |
| encode_sys | system.rs |
| encode_at | system.rs |
| encode_tlbi | system.rs |
| encode_swp | load_store.rs |
| encode_ldop | load_store.rs |
