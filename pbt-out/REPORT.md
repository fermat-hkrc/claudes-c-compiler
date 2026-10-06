# PBT Campaign Report: encode_tst

## Summary

**Verdict:** 5 medium: encode_tst silently accepts extra operands, SP/WSP, mixed W/X, FP/SIMD registers, and out-of-range shifts that llvm-mc/gas reject, encoding them as a different well-formed TST/ANDS word.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_tst
**Tests:** 13
**Result:** 8 passing, 5 bugs
**Change surface:** 1 changed function (encode_tst), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo tests executed encode_tst (6 KATs matched llvm-mc). Manual arm audit of the 12-line body completed the tier's 1 sweep round.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_tst | 13 | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_tst ignores extra operands

**Formal:** ∀ rn,rm same-width GPR, extra ∉ {valid Shift}. encode_tst([Rn,Rm,extra]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects `tst w0, w0, x0`; compare_branch.rs:38 names a two-operand alias, not a varargs form)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tst([Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x6a00001f))
**Impact:** Invalid GNU-style assembly with a trailing register is silently encoded as the two-operand form.
**Root cause:** compare_branch.rs:40 clones every operand into the ANDS alias list with no upper-bound arity check; encode_logical only requires len>=3 after the ZR prepend.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:40`
```rust
    new_ops.extend(operands.iter().cloned());
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() > 3
        || (operands.len() == 3 && !matches!(operands.get(2), Some(Operand::Shift { .. })))
    {
        return Err("tst: unexpected extra operand".to_string());
    }
```
**Bug report:** bug_reports/encode_tst_extra_operand.md
**Repro seed:** cc daac17e3532e5b052a7c558b67fc9ad1d605caf9c4632c872697dee66db66029
**Raw output:**
```text
Test failed: tst Rn, Rm, extra must Err (llvm-mc: invalid operand)
minimal failing input: rn = 0, rm = 0, is_64 = false, extra = Reg("x0")
```

### B2: encode_tst encodes SP/WSP as XZR/WZR

**Formal:** ∀ which ∈ {SP-Rn, SP-Rm, WSP-Rn, WSP-Rm}, n ∈ 0..30. encode_tst(sp_ops(which,n)) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects `tst sp, x0`; ARM TST Rn is a GPR not SP)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tst([Reg("sp"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0xea0003ff)) (same as tst xzr, x0)
**Impact:** A stack-pointer operand is turned into ZR, so invalid assembly becomes a different well-formed instruction.
**Root cause:** parse_reg_num maps "sp"|"wsp" and "xzr"|"wzr" both to 31; encode_tst does not reject SP.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/mod.rs:266`
```rust
        "sp" | "wsp" => Some(31),
        "xzr" | "wzr" => Some(31),
```
**Suggested fix:** Reject SP/WSP in encode_tst before the ANDS alias.
```rust
    if operands.iter().any(|o| matches!(o, Operand::Reg(r) if is_sp(r))) {
        return Err("tst: SP/WSP is not a valid GPR".to_string());
    }
```
**Bug report:** bug_reports/encode_tst_sp_as_zr.md
**Repro seed:** which = 0, n = 0
**Raw output:**
```text
Test failed: tst SP/WSP kind=0 must Err (llvm-mc: invalid operand)
minimal failing input: which = 0, n = 0
```

### B3: encode_tst accepts mixed W/X register pairs

**Formal:** ∀ n ∈ 0..30. encode_tst([Reg("xN"), Reg("wN")]) is Err ∧ encode_tst([Reg("wN"), Reg("xN")]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc “expected compatible register”)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tst([Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x6a00001f)) (encoded as tst w0, w0)
**Impact:** Mixed-width assembly is silently reinterpreted as same-width using Rm's number only.
**Root cause:** encode_tst derives sf only from the first operand; encode_logical takes Rm as a 5-bit number with no width check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:480`
```rust
        let rm = parse_reg_num(rm_name).ok_or("invalid rm")?;
```
**Suggested fix:** Reject mixed widths in encode_tst.
```rust
    if let (Some(Operand::Reg(rn)), Some(Operand::Reg(rm))) = (operands.get(0), operands.get(1)) {
        if is_32bit_reg(rn) != is_32bit_reg(rm) {
            return Err("tst: mixed register widths".to_string());
        }
    }
```
**Bug report:** bug_reports/encode_tst_mixed_width.md
**Repro seed:** n = 0, x_first = false
**Raw output:**
```text
Test failed: tst mixed W/X must Err (llvm-mc: expected compatible register)
minimal failing input: n = 0, x_first = false
```

### B4: encode_tst accepts FP/SIMD registers as GPRs

**Formal:** ∀ n,m ∈ 0..31, p ∈ {d,s,q,v,h,b}. encode_tst([Reg(pN), Reg(pM)]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects `tst d0, d0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tst([Reg("d0"), Reg("d0")])
**Expected / Actual:** Err / Ok(Word(0xea00001f)) (encoded as tst x0, x0)
**Impact:** FP/SIMD names are parsed as GPR numbers, producing a well-formed integer TST.
**Root cause:** encode_tst never calls is_fp_reg; parse_reg_num accepts prefixes d/s/q/v/h/b.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/mod.rs:272`
```rust
                'x' | 'w' | 'd' | 's' | 'q' | 'v' | 'h' | 'b' => {
                    let num: u32 = name[1..].parse().ok()?;
                    if num <= 31 { Some(num) } else { None }
```
**Suggested fix:** Reject FP/SIMD names in encode_tst.
```rust
    if operands.iter().any(|o| matches!(o, Operand::Reg(r) if is_fp_reg(r))) {
        return Err("tst: FP/SIMD register is not a GPR".to_string());
    }
```
**Bug report:** bug_reports/encode_tst_fp_as_gpr.md
**Repro seed:** n = 0, m = 0, pref = 0, as_rm = false
**Raw output:**
```text
Test failed: tst FP/SIMD register must Err (llvm-mc: invalid operand)
minimal failing input: n = 0, m = 0, pref = 0, as_rm = false
```

### B5: encode_tst wraps out-of-range shift amounts

**Formal:** ∀ rn,rm ∈ 0..31, is_64, amt > max(is_64). encode_tst([Rn,Rm,Shift(lsl,amt)]) is Err
**Contract evidence:** inferred (ARM ARM 32-bit imm6 < 32; llvm-mc rejects `tst w0, w0, lsl #32`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tst([Reg("w0"), Reg("w0"), Shift { kind: "lsl", amount: 32 }])
**Expected / Actual:** Err / Ok(Word(0x6a00801f))
**Impact:** An illegal W-form lsl #32 is packed into imm6, producing an UNALLOCATED encoding instead of an assembler error.
**Root cause:** encode_logical packs `shift_amount & 0x3F` with no 32-bit max-31 / 64-bit max-63 check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:496`
```rust
            | (rm << 16) | ((shift_amount & 0x3F) << 10) | (rn << 5) | rd;
```
**Suggested fix:** Reject out-of-range shift amounts.
```rust
    let max = if is_64 { 63u32 } else { 31u32 };
    if shift_amount > max {
        return Err(format!("shift amount {shift_amount} out of range 0..{max}"));
    }
```
**Bug report:** bug_reports/encode_tst_shift_oor.md
**Repro seed:** rn = 0, rm = 0, is_64 = false, sk = 0, amt = 32
**Raw output:**
```text
Test failed: tst shift lsl #32 on W must Err (llvm-mc range 0..31)
minimal failing input: rn = 0, rm = 0, is_64 = false, sk = 0, amt = 32
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_tst_pbt.rs | 13 properties + 6 KAT + 6 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_tst -- --test-threads=1
```

Per-bug:

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tst_regression_extra_operand -- --test-threads=1
cargo test --lib test_encode_tst_regression_sp_rn -- --test-threads=1
cargo test --lib test_encode_tst_regression_mixed_width -- --test-threads=1
cargo test --lib test_encode_tst_regression_fp_reg -- --test-threads=1
cargo test --lib test_encode_tst_regression_shift_oor -- --test-threads=1
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
- pbt-out/run/encode_tst.log
- pbt-out/run/encode_tst_round2.log
- pbt-out/bug_reports/encode_tst_extra_operand.md
- pbt-out/bug_reports/encode_tst_extra_operand.html
- pbt-out/bug_reports/encode_tst_sp_as_zr.md
- pbt-out/bug_reports/encode_tst_sp_as_zr.html
- pbt-out/bug_reports/encode_tst_mixed_width.md
- pbt-out/bug_reports/encode_tst_mixed_width.html
- pbt-out/bug_reports/encode_tst_fp_as_gpr.md
- pbt-out/bug_reports/encode_tst_fp_as_gpr.html
- pbt-out/bug_reports/encode_tst_shift_oor.md
- pbt-out/bug_reports/encode_tst_shift_oor.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 06:04 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 167/307 total | PBT candidates: 167 | Tested: 167 (100%) | 1 pass, 167 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 167 |
| **Tested (of PBT candidates)** | **167 / 167 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 167 / -1 |
| **Overall (tested / all functions)** | **167 / 307 (54%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 167 | 167 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 167 | 167 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 19 | 19 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 14 | 14 | 100% | covered |
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
| encode_fnmadd_fnmsub | fp_scalar.rs |
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
| encode_stop | load_store.rs |
| encode_tst | compare_branch.rs |
