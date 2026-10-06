# PBT Campaign Report: encode_crc32

## Summary

**Verdict:** 4 medium: encode_crc32 accepts extra operands, SP/WSP, wrong W/X widths, and FP/SIMD registers that llvm-mc/gas reject, so invalid GNU-style CRC32 assembly silently becomes a machine-code word.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_crc32
**Tests:** 12 properties (8 passing, 4 failing) plus 8 passing KAT and 5 failing regression witnesses
**Result:** 8 passing, 4 bugs
**Change surface:** 1 changed function (encode_crc32), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter, NOT LINKED). Sweep was a manual arm audit of the 20-line body. Tier: standard.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_crc32 | 12 properties (8 pass / 4 fail) | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_crc32 ignores extra operands

**Formal:** ∀ mnemonic ∈ CRC32_8, valid 3-operand CRC32 ops, extra ∈ Operand. encode_crc32(mnemonic, ops++[extra]) = Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc "invalid operand for instruction" on a 4th operand; ARM CRC32 is a 3-register instruction)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_crc32("crc32b", [Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x1ac04000))
**Impact:** Invalid GNU-style CRC32 with a trailing operand is assembled instead of rejected.
**Root cause:** bitfield.rs:227-229 reads only operands 0..2 via get_reg and has no operands.len() upper bound.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 3 {
        return Err(format!("crc32: expected 3 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_crc32_extra_operand.md
**Repro seed:** cc 58218e6499d74ab0c5a1ae329b88808b4bb7de692e08d6baab02d50a0fd4985c
**Raw output:**
```text
Test failed: CRC32 has no 4th operand; extra operand must Err (llvm-mc rejects it)
minimal failing input: m = "crc32b", rd = 0, rn = 0, rm = 0, extra = Reg("x0")
```

### B2: encode_crc32 accepts SP/WSP as register 31

**Formal:** ∀ mnemonic ∈ CRC32_8, slot ∈ {0,1,2}, sp ∈ {sp,wsp}. encode_crc32(mnemonic, ops with slot=sp) = Err
**Contract evidence:** inferred (ARM CRC32 Rd/Rn/Rm are ZR not SP; llvm-mc "invalid operand for instruction" on sp/wsp; README.md:12)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_crc32("crc32b", [Reg("wsp"), Reg("w0"), Reg("w0")])
**Expected / Actual:** Err / Ok(Word(0x1ac0401f)) encoding Rd=31 as WZR
**Impact:** Stack-pointer operands are silently rewritten as WZR/XZR.
**Root cause:** bitfield.rs:227-229 calls get_reg; parse_reg_num maps sp/wsp to 31 the same as xzr/wzr.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject SP/WSP after parsing each register name.
```rust
    if name.eq_ignore_ascii_case("sp") || name.eq_ignore_ascii_case("wsp") {
        return Err(format!("crc32: SP/WSP is not a valid operand ({name})"));
    }
```
**Bug report:** bug_reports/encode_crc32_sp_as_zr.md
**Repro seed:** (deterministic; shrunk to slot=0 sp=wsp other=0)
**Raw output:**
```text
Test failed: SP/WSP is not a valid CRC32 operand (slot=0 sp=wsp)
minimal failing input: m = "crc32b", slot = 0, sp64 = false, other = 0
```

### B3: encode_crc32 ignores W vs X register width

**Formal:** ∀ mnemonic ∈ CRC32_8, rd,rn,rm ∈ 0..31, widths that violate (Rd=W ∧ Rn=W ∧ Rm=W_if_BHW else X). encode_crc32(mnemonic, ops) = Err
**Contract evidence:** inferred (ARM CRC32 always Wd/Wn, Rm is Wm for B/H/W and Xm for X; llvm-mc "invalid operand for instruction"; README.md:12)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_crc32("crc32b", [Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x1ac04000))
**Impact:** Mixed-width CRC32 text is encoded as if the widths were legal.
**Root cause:** bitfield.rs:227-229 discards get_reg's is_64 flag; sf/sz come only from the mnemonic.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Keep is_64 and require Wd/Wn plus Wm or Xm by mnemonic.
```rust
    let (rd, rd64) = get_reg(operands, 0)?;
    let (rn, rn64) = get_reg(operands, 1)?;
    let (rm, rm64) = get_reg(operands, 2)?;
    if rd64 || rn64 || rm64 != mnemonic.ends_with('x') {
        return Err("crc32: Rd/Rn must be W; Rm must be W (B/H/W) or X (X)".into());
    }
```
**Bug report:** bug_reports/encode_crc32_wrong_width.md
**Repro seed:** (deterministic; shrunk to crc32b w0,w0,x0)
**Raw output:**
```text
Test failed: CRC32 crc32b wrong-width rd64=false rn64=false rm64=true must Err (llvm-mc rejects it)
minimal failing input: m = "crc32b", rd = 0, rn = 0, rm = 0, rd64 = false, rn64 = false, rm64 = true
```

### B4: encode_crc32 accepts FP/SIMD registers as GPRs

**Formal:** ∀ mnemonic ∈ CRC32_8, slot ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_crc32(mnemonic, ops with slot=prefix∥n) = Err
**Contract evidence:** inferred (ARM CRC32 operands are GPRs; llvm-mc "invalid operand for instruction" on d/s/q/v/h/b; README.md:12)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_crc32("crc32b", [Reg("d0"), Reg("w1"), Reg("w2")])
**Expected / Actual:** Err / Ok(Word(0x1ac24020))
**Impact:** A SIMD register is treated as GPR number N and encoded.
**Root cause:** bitfield.rs:227-229 calls get_reg; parse_reg_num accepts prefixes d/s/q/v/h/b; encode_crc32 never checks is_fp_reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/bitfield.rs:227`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject FP/SIMD names after parsing each operand.
```rust
    let c = name.chars().next().unwrap_or(' ').to_ascii_lowercase();
    if matches!(c, 'd' | 's' | 'q' | 'v' | 'h' | 'b') {
        return Err(format!("crc32: FP/SIMD register {name} is not a valid operand"));
    }
```
**Bug report:** bug_reports/encode_crc32_fp_as_gpr.md
**Repro seed:** (deterministic; shrunk to crc32b d0, w1, w2)
**Raw output:**
```text
Test failed: FP/SIMD register d0 is not a valid CRC32 operand (slot=0)
minimal failing input: m = "crc32b", slot = 0, prefix = "d", n = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_crc32_pbt.rs | 12 properties + 8 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_crc32_pbt` |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_crc32_pbt -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_extra_operand -- --test-threads=1
```

B2 SP as ZR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_sp -- --test-threads=1
```

B3 wrong width:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_wrong_width -- --test-threads=1
```

B4 FP as GPR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_crc32_regression_fp -- --test-threads=1
```

Build command (user contract, target swapped):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_crc32_pbt -- --test-threads=1
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
- pbt-out/bug_reports/encode_crc32_extra_operand.md
- pbt-out/bug_reports/encode_crc32_extra_operand.html
- pbt-out/bug_reports/encode_crc32_sp_as_zr.md
- pbt-out/bug_reports/encode_crc32_sp_as_zr.html
- pbt-out/bug_reports/encode_crc32_wrong_width.md
- pbt-out/bug_reports/encode_crc32_wrong_width.html
- pbt-out/bug_reports/encode_crc32_fp_as_gpr.md
- pbt-out/bug_reports/encode_crc32_fp_as_gpr.html
- pbt-out/build.log
- pbt-out/CHANGE_SURFACE.md
- src/backend/arm/assembler/encoder/encode_crc32_pbt.rs
- proptest-regressions/backend/arm/assembler/encoder/encode_crc32_pbt.txt

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 07:06 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 169/307 total | PBT candidates: 169 | Tested: 169 (100%) | 1 pass, 169 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 169 |
| **Tested (of PBT candidates)** | **169 / 169 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 169 / -1 |
| **Overall (tested / all functions)** | **169 / 307 (55%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 169 | 169 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 169 | 169 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
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
| encode_tbz | compare_branch.rs |
| encode_crc32 | bitfield.rs |
