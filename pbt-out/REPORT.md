# PBT Campaign Report: encode_sys

## Summary

**Verdict:** 1 high and 3 medium: encode_sys masks out-of-range op1/CRn/CRm/op2 into a different SYS encoding, ignores extra operands, treats W/SP/SIMD as X registers, and rejects the GNU `fp` alias for x29.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_sys
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_sys), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). encode_sys ran via `cargo test --lib encode_sys`; sweep was a manual arm audit of the 22-line body plus two parse-error properties.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_sys | 10 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_sys rejects the GNU Xt alias fp

**Formal:** ∀ op1 ∈ [0,7], CRn ∈ [0,15], CRm ∈ [0,15], op2 ∈ [0,7], Xt ∈ {x0..x30, xzr, x31, lr, fp, omitted}, case ∈ ASCII, ws ∈ {space,tab}*. encode_sys(raw) = llvm-mc("sys "+raw) = 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt  where Rt(omitted)=31, Rt(xzr)=Rt(x31)=31, Rt(lr)=30, Rt(fp)=29
**Contract evidence:** inferred (README.md:11 gas-compat; llvm-mc and GNU as accept `fp` as x29)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_sys("#0, c0, c0, #0, fp") then llvm-mc("sys #0, c0, c0, #0, fp")
**Expected / Actual:** Ok(Word(0xd508001d)) / Err("sys: invalid register: fp")
**Impact:** Assembly that uses the standard `fp` alias for x29 fails to assemble `sys` instead of producing SYS with Rt=29.
**Root cause:** system.rs:462-463 lowercases Xt and calls parse_reg_num, which has no `fp` arm, so a valid GNU alias is reported as an invalid register.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:462`
```rust
        let reg = parts[4].trim().to_lowercase();
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```
**Suggested fix:** Treat `fp` as x29 before the lookup.
```rust
        let mut reg = parts[4].trim().to_lowercase();
        if reg == "fp" {
            reg = "x29".to_string();
        }
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```
**Bug report:** bug_reports/encode_sys_fp_alias.md
**Repro seed:** case = (0, 0, 0, 0, 29, "#0, c0, c0, #0, fp", "#0,c0,c0,#0,fp")
**Raw output:**
```text
Test failed: SUT vs llvm-mc for sys #0,c0,c0,#0,fp: sys: invalid register: fp.
minimal failing input: case = (0, 0, 0, 0, 29, "#0, c0, c0, #0, fp", "#0,c0,c0,#0,fp")
```

### B2: encode_sys ignores a sixth operand

**Formal:** ∀ valid SYS fields, extra ∈ {x0..x31, #imm, foo}. llvm-mc("sys "+raw+", "+extra) fails ∧ encode_sys(raw+", "+extra) = Err
**Contract evidence:** inferred (llvm-mc "invalid operand"; gas "unexpected characters following instruction at operand 5")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_sys("#0, c0, c0, #0, x0, x0")
**Expected / Actual:** Err / Ok(Word(0xd5080000))
**Impact:** A stray operand after Xt is not diagnosed; `sys #0, c0, c0, #0, x0, x0` assembles as SYS with the first five fields.
**Root cause:** system.rs:451 only rejects parts.len() < 4; when parts.len() >= 5 the fifth token is Xt and later comma-fields are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:451`
```rust
    if parts.len() < 4 {
        return Err(format!("sys needs at least 4 operands, got: {}", raw_operands));
    }
```
**Suggested fix:** Also reject more than five operands.
```rust
    if parts.len() < 4 || parts.len() > 5 {
        return Err(format!("sys needs 4 or 5 operands, got: {}", raw_operands));
    }
```
**Bug report:** bug_reports/encode_sys_extra_operand.md
**Repro seed:** op1 = 0, crn = 0, crm = 0, op2 = 0, t = 0, extra = "x0"
**Raw output:**
```text
Test failed: SYS with extra operand must Err (llvm-mc rejects sys #0, c0, c0, #0, x0, x0); SUT raw "#0, c0, c0, #0, x0, x0": Word(3574071296).
minimal failing input: op1 = 0, crn = 0, crm = 0, op2 = 0, t = 0, extra = "x0"
```

### B3: encode_sys masks out-of-range op1/CRn/CRm/op2 instead of rejecting

**Formal:** ∀ (op1,CRn,CRm,op2,Xt) valid except exactly one field out of range at bound+1 or above. llvm-mc("sys "+raw) fails ∧ encode_sys(raw) = Err
**Contract evidence:** inferred (ARM ARM SYS field widths; llvm-mc "immediate must be an integer in range [0, 7]" / "Expected cN operand where 0 <= N <= 15"; gas "immediate value out of range 0 to 7")
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_sys("#8, c0, c0, #0, x0")
**Expected / Actual:** Err / Ok(Word(0xd5080000))
**Impact:** An out-of-range immediate silently wraps (8 & 7 = 0) and emits a different SYS instruction. The same wrap applies to CRn/CRm ≥ 16 and op2 ≥ 8.
**Root cause:** system.rs:466 packs fields with (op1 & 7), (crn & 0xF), (crm & 0xF), (op2 & 7) after a successful u32 parse, so values that fit in u32 but not in the ARM field widths are truncated.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:466`
```rust
    let word = 0xd5080000 | ((op1 & 7) << 16) | ((crn & 0xF) << 12) | ((crm & 0xF) << 8) | ((op2 & 7) << 5) | rt;
```
**Suggested fix:** Reject out-of-range fields before packing.
```rust
    if op1 > 7 || crn > 15 || crm > 15 || op2 > 7 {
        return Err(format!("sys: field out of range in {}", raw_operands));
    }
    let word = 0xd5080000 | (op1 << 16) | (crn << 12) | (crm << 8) | (op2 << 5) | rt;
```
**Bug report:** bug_reports/encode_sys_oob_fields.md
**Repro seed:** raw = "#8, c0, c0, #0, x0"
**Raw output:**
```text
Test failed: SYS with out-of-range op1/Cn/Cm/op2 must Err (llvm-mc rejects sys #8, c0, c0, #0, x0); SUT raw "#8, c0, c0, #0, x0": Word(3574071296).
minimal failing input: raw = "#8, c0, c0, #0, x0"
```

### B4: encode_sys accepts W/SP/SIMD Xt as if they were X registers

**Formal:** ∀ valid SYS fields, bad ∈ {w0..w31, wzr, wsp, sp, v0, d0, s0, q0, h0, b0}. llvm-mc("sys "+raw+", "+bad) fails ∧ encode_sys(raw+", "+bad) = Err
**Contract evidence:** inferred (ARM ARM SYS Xt is a 64-bit GPR; llvm-mc "invalid operand"; gas "operand mismatch" / "must be an integer register")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_sys("#0, c0, c0, #0, w0")
**Expected / Actual:** Err / Ok(Word(0xd5080000))
**Impact:** Invalid register-class assembly is not diagnosed and encodes as the 64-bit GPR of the same number (`w0` becomes Xt=x0).
**Root cause:** system.rs:462-463 calls parse_reg_num, which returns Some(n) for w/d/s/q/v/h/b prefixes and for sp/wsp/wzr. encode_sys never checks is_64bit_reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:462`
```rust
        let reg = parts[4].trim().to_lowercase();
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```
**Suggested fix:** Require a 64-bit GPR before packing Rt.
```rust
        let reg = parts[4].trim().to_lowercase();
        if !(reg.starts_with('x') || reg == "xzr" || reg == "lr" || reg == "fp") {
            return Err(format!("sys: invalid register: {}", parts[4]));
        }
        parse_reg_num(&reg).ok_or_else(|| format!("sys: invalid register: {}", parts[4]))?
```
**Bug report:** bug_reports/encode_sys_wrong_reg_class.md
**Repro seed:** op1 = 0, crn = 0, crm = 0, op2 = 0, bad = "w0"
**Raw output:**
```text
Test failed: SYS with non-X register must Err (llvm-mc rejects sys #0, c0, c0, #0, w0); SUT raw "#0, c0, c0, #0, w0": Word(3574071296).
minimal failing input: op1 = 0, crn = 0, crm = 0, op2 = 0, bad = "w0"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_sys_pbt.rs | 10 properties + 5 KAT + 4 regression witnesses |

## Reproduction

Whole suite (expected: 6 properties + 5 KAT passing; 4 properties + 4 regressions failing):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_sys -- --test-threads=1
```

B1 fp alias:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_fp_alias -- --test-threads=1 --nocapture
```

B2 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_extra_operand -- --test-threads=1 --nocapture
```

B3 oob op1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_oob_op1 -- --test-threads=1 --nocapture
```

B4 wrong register class:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sys_regression_w0 -- --test-threads=1 --nocapture
```

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
- pbt-out/bug_reports/encode_sys_fp_alias.md
- pbt-out/bug_reports/encode_sys_fp_alias.html
- pbt-out/bug_reports/encode_sys_extra_operand.md
- pbt-out/bug_reports/encode_sys_extra_operand.html
- pbt-out/bug_reports/encode_sys_oob_fields.md
- pbt-out/bug_reports/encode_sys_oob_fields.html
- pbt-out/bug_reports/encode_sys_wrong_reg_class.md
- pbt-out/bug_reports/encode_sys_wrong_reg_class.html
- pbt-out/run/encode_sys_test.log
- pbt-out/run/encode_sys_test2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 03:00 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 160/307 total | PBT candidates: 160 | Tested: 160 (100%) | 1 pass, 160 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 160 |
| **Tested (of PBT candidates)** | **160 / 160 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 160 / -1 |
| **Overall (tested / all functions)** | **160 / 307 (52%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 160 | 160 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 160 | 160 | 0 | 100% |

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
| encode_hvc | system.rs |
| encode_brk | system.rs |
| encode_hint | system.rs |
| encode_bti | system.rs |
| encode_ic | system.rs |
| encode_dc | system.rs |
| encode_sys | system.rs |
