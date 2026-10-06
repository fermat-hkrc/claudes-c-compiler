# PBT Campaign Report: encode_msr

## Summary

**Verdict:** 1 high, 5 medium: encode_msr writes CNTV_CVAL_EL0 as op2=4 (ARM op2=2), so `msr cntv_cval_el0, x0` programs the wrong timer register; it also encodes read-only OSLSR_EL1, ignores extra operands, masks out-of-range S-form fields, accepts Wt/SP/FP as Xt, and wraps PSTATE immediates outside 0..=15.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_msr
**Tests:** 11 properties (plus 7 KAT + 6 regression witnesses)
**Result:** 5 passing, 6 failing properties, 6 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo tests executed encode_msr. Sweep was a manual arm audit of named/generic/numbered/imm/layout/case-fold/extra/wrong-src/unknown/arity/oob. Closed: tier round spent.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_msr | 11 properties | 6 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_msr encodes CNTV_CVAL_EL0 with the wrong sysreg field

**Formal:** ∀ name ∈ NAMED, xt ∈ Xt. let asm = "msr "+name+", "+xt; let sut = encode_msr([Symbol(name), Reg(xt)]); (llvm-mc(asm)=Ok(w) ∧ sut=Ok(Word(w))) ∨ (llvm-mc(asm)=Err ∧ sut=Err)
**Contract evidence:** inferred (ARM ARM CNTV_CVAL_EL0 is S3_3_C14_C3_2; llvm-mc encodes 0xd51be340; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_msr([Symbol("cntv_cval_el0"), Reg("x0")])  (`msr cntv_cval_el0, x0`)
**Expected / Actual:** Ok(Word(0xd51be340)) / Ok(Word(0xd51be380))
**Impact:** Kernel/runtime code that programs CNTV_CVAL_EL0 through this assembler writes a reserved encoding (op2=4) instead of the virtual timer compare-value register.
**Root cause:** system.rs:364 `"cntv_cval_el0" => 0xdf1c` uses op2=4; ARM encoding is 0xdf1a (op2=2).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:364`
```rust
        "cntv_cval_el0" => 0xdf1c,
```
**Suggested fix:** Use the ARM encoding 0xdf1a.
```rust
        "cntv_cval_el0" => 0xdf1a,
```
**Bug report:** bug_reports/encode_msr_cntv_cval_el0_encoding.md
**Repro seed:** cc fbe1e436ef5d0b120e6509dc52ba7417a0e23c84a8ecfcb4e5fb884b5c9f0deb
**Raw output:**
```text
Test failed: SUT vs llvm-mc for msr cntv_cval_el0, x0: sut=d51be380 mc=d51be340
minimal failing input: name = "cntv_cval_el0", xt = "x0"
```

### B2: encode_msr encodes read-only OSLSR_EL1

**Formal:** ∀ name ∈ NAMED, xt ∈ Xt. let asm = "msr "+name+", "+xt; let sut = encode_msr([Symbol(name), Reg(xt)]); (llvm-mc(asm)=Ok(w) ∧ sut=Ok(Word(w))) ∨ (llvm-mc(asm)=Err ∧ sut=Err)
**Contract evidence:** inferred (OSLSR_EL1 is MRS-only; llvm-mc rejects `msr oslsr_el1, x0`; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_msr([Symbol("oslsr_el1"), Reg("x0")])  (`msr oslsr_el1, x0`)
**Expected / Actual:** Err / Ok(Word(0xd5101180))
**Impact:** A write of a read-only OS-lock status register is assembled instead of rejected.
**Root cause:** system.rs:311 `"oslsr_el1" => 0x808c` is in the MSR named table; OSLSR_EL1 is read-only.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:311`
```rust
        "oslsr_el1" => 0x808c,
```
**Suggested fix:** Drop `oslsr_el1` from the MSR table so it falls through to Err.
```rust
        // oslsr_el1 is read-only (MRS); do not match it here
```
**Bug report:** bug_reports/encode_msr_oslsr_el1_read_only.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
msr oslsr_el1, x0 must Err (OSLSR_EL1 is read-only)
```

### B3: encode_msr ignores extra operands

**Formal:** ∀ name ∈ NAMED, xt ∈ Xt, extra ∈ ExtraOperand. llvm-mc("msr "+name+", "+xt+", "+extra)=Err ⇒ encode_msr([Symbol(name), Reg(xt), extra])=Err
**Contract evidence:** inferred (ARM ARM MSR is two-operand; llvm-mc rejects extra; README.md:12)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_msr([Symbol("sp_el0"), Reg("x0"), Reg("x0")])  (`msr sp_el0, x0, x0`)
**Expected / Actual:** Err / Ok(Word) encoding as `msr sp_el0, x0`
**Impact:** Typos with a trailing operand assemble silently as the two-operand form.
**Root cause:** system.rs:295 `get_reg(operands, 1)` — only the first two operands are examined; `operands.len()` is never checked.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:295`
```rust
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject a slice longer than two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("msr: expected system_reg, Xt".to_string());
    }
```
**Bug report:** bug_reports/encode_msr_extra_operand.md
**Repro seed:** (deterministic; first example)
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects msr sp_el0, x0, x0)
minimal failing input: name = "sp_el0", xt = "x0", extra = Reg("x0")
```

### B4: encode_msr masks out-of-range generic S-form fields

**Formal:** ∀ dest ∉ Xt, name ∈ NAMED. encode_msr([Symbol(name), Reg(dest)])=Err. ∀ unknown ∉ NAMED∪S-form∪numbered. encode_msr([Symbol(unknown), Reg(x0)])=Err. encode_msr([])=Err. encode_msr([Symbol(name)])=Err. ∀ oob S-form/numbered. encode_msr(...)=Err. ∀ field ∈ {daifset,daifclr,spsel}, imm ∉ 0..=15. encode_msr([Symbol(field), Imm(imm)])=Err
**Contract evidence:** inferred (ARM generic S-form widths op0 0..=3; llvm-mc rejects s4_*; README.md:12)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_msr([Symbol("s4_0_c1_c0_1"), Reg("x0")])  (`msr s4_0_c1_c0_1, x0`)
**Expected / Actual:** Err / Ok(Word) with op0 masked to 0
**Impact:** An illegal sysreg name silently becomes a different legal one.
**Root cause:** system.rs:381 `_ => parse_generic_sysreg(&sysreg)?` then sysreg_encoding masks `op0 & 3`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:381`
```rust
        _ => parse_generic_sysreg(&sysreg)?,
```
**Suggested fix:** Reject out-of-range generic fields before masking.
```rust
        _ => parse_generic_sysreg(&sysreg)?, // range-check op0..=3, op1..=7, CRn/CRm..=15, op2..=7
```
**Bug report:** bug_reports/encode_msr_oob_generic.md
**Repro seed:** cc 4599dc9e4b7fc390546eef5e82cb0e015ebc399c5e06a2b641c417937b05a887
**Raw output:**
```text
Test failed: oob generic sysreg must Err (llvm-mc rejects msr s4_0_c1_c0_1, x0)
minimal failing input: kind = 4, oob_g = "s4_0_c1_c0_1", xt = "x0"
```

### B5: encode_msr accepts Wt/SP/FP as Xt

**Formal:** ∀ dest ∉ Xt, name ∈ NAMED. llvm-mc("msr "+name+", "+dest)=Err ⇒ encode_msr([Symbol(name), Reg(dest)])=Err
**Contract evidence:** inferred (ARM ARM MSR requires Xt, a 64-bit GPR not SP; llvm-mc rejects Wt/SP/FP; system.rs:294 "msr sysreg, Xt")
**Documentation conflict:** (none) — the comment states Xt; it does not declare Wt valid.
**Severity:** medium
**Counterexample:** encode_msr([Symbol("sp_el0"), Reg("w0")])  (`msr sp_el0, w0`)
**Expected / Actual:** Err / Ok(Word(0xd5184100)) encoding as `msr sp_el0, x0`
**Impact:** A width or register-class error is silently rewritten to Xt.
**Root cause:** system.rs:295 `let (rt, _) = get_reg(operands, 1)?` discards is_64; parse_reg_num accepts w/sp/d/s prefixes.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:295`
```rust
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require a 64-bit GPR and reject SP.
```rust
    let (rt, is_64) = get_reg(operands, 1)?;
    if !is_64 {
        return Err("msr: Xt must be a 64-bit GPR".to_string());
    }
```
**Bug report:** bug_reports/encode_msr_w_src.md
**Repro seed:** (first example)
**Raw output:**
```text
Test failed: non-Xt source must Err (llvm-mc rejects msr sp_el0, w0), got Ok(Word(3575136512))
minimal failing input: dest = "w0", name = "sp_el0"
```

### B6: encode_msr masks PSTATE immediates outside 0..=15

**Formal:** ∀ field ∈ {daifset, daifclr, spsel}, imm ∉ 0..=15. llvm-mc("msr "+field+", #"+imm)=Err ⇒ encode_msr([Symbol(field), Imm(imm)])=Err
**Contract evidence:** inferred (ARM CRm is 4 bits; llvm-mc requires integer in range [0, 15]; system.rs:269 CRm[11:8])
**Documentation conflict:** (none) — the comment names the CRm field; it does not declare values outside 0..=15 valid.
**Severity:** medium
**Counterexample:** encode_msr([Symbol("daifset"), Imm(-2)])  (`msr daifset, #-2`)
**Expected / Actual:** Err / Ok(Word) with CRm = 14
**Impact:** An out-of-range PSTATE immediate silently wraps, so DAIF set/clear hits the wrong bits (`#16` becomes `#0`).
**Root cause:** system.rs:273 `get_imm(operands, 1)? as u32 & 0xF` truncates instead of range-checking.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:273`
```rust
            let imm = get_imm(operands, 1)? as u32 & 0xF;
```
**Suggested fix:** Reject immediates outside 0..=15.
```rust
            let imm = get_imm(operands, 1)?;
            if !(0..=15).contains(&imm) {
                return Err("msr: PSTATE immediate must be in 0..=15".to_string());
            }
```
**Bug report:** bug_reports/encode_msr_oob_imm.md
**Repro seed:** (first example)
**Raw output:**
```text
Test failed: oob PSTATE imm must Err (llvm-mc rejects msr daifset, #-2)
minimal failing input: field = "daifset", imm = -2
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_msr_pbt.rs | 11 properties, 7 KAT, 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_msr_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_msr -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_cntv_cval_el0 -- --test-threads=1 --nocapture
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_oslsr_el1 -- --test-threads=1 --nocapture
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_extra_x0 -- --test-threads=1 --nocapture
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_oob_generic_s4 -- --test-threads=1 --nocapture
```

B5:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_w0_src -- --test-threads=1 --nocapture
```

B6:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_msr_regression_daifset_imm16 -- --test-threads=1 --nocapture
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/bug_reports/encode_msr_cntv_cval_el0_encoding.md
- pbt-out/bug_reports/encode_msr_cntv_cval_el0_encoding.html
- pbt-out/bug_reports/encode_msr_oslsr_el1_read_only.md
- pbt-out/bug_reports/encode_msr_oslsr_el1_read_only.html
- pbt-out/bug_reports/encode_msr_extra_operand.md
- pbt-out/bug_reports/encode_msr_extra_operand.html
- pbt-out/bug_reports/encode_msr_oob_generic.md
- pbt-out/bug_reports/encode_msr_oob_generic.html
- pbt-out/bug_reports/encode_msr_w_src.md
- pbt-out/bug_reports/encode_msr_w_src.html
- pbt-out/bug_reports/encode_msr_oob_imm.md
- pbt-out/bug_reports/encode_msr_oob_imm.html
- pbt-out/run/ (kat.log, encode_msr.log, regression.log)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 00:38 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 152/307 total | PBT candidates: 152 | Tested: 152 (100%) | 0 pass, 152 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 152 |
| **Tested (of PBT candidates)** | **152 / 152 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 152 / 0 |
| **Overall (tested / all functions)** | **152 / 307 (50%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 152 | 152 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 152 | 152 | 0 | 100% |

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
