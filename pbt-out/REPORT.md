# PBT Campaign Report: encode_smaddl

## Summary

**Verdict:** 4 medium: encode_smaddl silently accepts extra operands, W/X mixes, SP/WSP, and FP/SIMD registers that llvm-mc/gas reject, so illegal GNU-style SMADDL is assembled instead of failed.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_smaddl
**Tests:** 13 properties (plus 4 KAT + 4 regression witnesses)
**Result:** 9 passing, 4 bugs
**Change surface:** 1 changed function (encode_smaddl), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 12-line body.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_smaddl | 13 properties (9 pass / 4 fail) + 4 KAT + 4 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_smaddl silently ignores a 5th operand

**Formal:** ∀ rd,rn,rm,ra ∈ 0..31, extra ∈ Operand. encode_smaddl([Xd(rd), Wn(rn), Wm(rm), Xa(ra), extra]) is Err
**Contract evidence:** inferred (rustdoc data_processing.rs:652 names four operands; llvm-mc rejects a 5th)
**Documentation conflict:** (none) — the rustdoc lists Xd, Wn, Wm, Xa and does not declare extra operands invalid; the body has no arity upper bound
**Severity:** medium
**Counterexample:** encode_smaddl([Reg("x0"), Reg("w0"), Reg("w0"), Reg("x0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x9b200000))
**Impact:** A typo or extra operand is silently dropped instead of failing the assemble.
**Root cause:** data_processing.rs:654-657 reads only operands 0..3 via get_reg and has no operands.len() upper bound.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 4 {
        return Err(format!("smaddl: expected 4 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_smaddl_extra_operand.md
**Repro seed:** cc 5dd5f4875fcede75f7f06bb16d8be72f4a27dc0234ba29aa31f22f3c981cd53d
**Raw output:**
```text
Test failed: SMADDL has no 5th operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:403.
minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, extra = Reg("x0")
```

### B2: encode_smaddl accepts W/X mixes that llvm-mc rejects

**Formal:** ∀ rd,rn,rm,ra ∈ 0..30, rd64,rn64,rm64,ra64 ∈ bool. ¬(rd64 ∧ ¬rn64 ∧ ¬rm64 ∧ ra64) ⇒ encode_smaddl(mixed_width_ops) is Err
**Contract evidence:** documented data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)"
**Documentation conflict:** data_processing.rs:652 states the operands ARE Xd, Wn, Wm, Xa; the code discards get_reg's width flag. Mark (not independently verified) only in that the comment is the contract the body violates.
**Severity:** medium
**Counterexample:** encode_smaddl([Reg("w0"), Reg("w0"), Reg("w0"), Reg("w0")])
**Expected / Actual:** Err / Ok(Word(0x9b200000))
**Impact:** 32-bit sources/dest are encoded as a 64-bit SMADDL word.
**Root cause:** data_processing.rs:654-657 binds `let (rd, _) = get_reg(...)`, discarding is_64.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Require Xd/Xa and Wn/Wm before packing.
```rust
    let (rd, rd64) = get_reg(operands, 0)?;
    let (rn, rn64) = get_reg(operands, 1)?;
    let (rm, rm64) = get_reg(operands, 2)?;
    let (ra, ra64) = get_reg(operands, 3)?;
    if !rd64 || rn64 || rm64 || !ra64 {
        return Err("smaddl requires Xd, Wn, Wm, Xa".into());
    }
```
**Bug report:** bug_reports/encode_smaddl_wrong_width.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
Test failed: SMADDL requires Xd, Wn, Wm, Xa; rd64=false rn64=false rm64=false ra64=false must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:427.
minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, rd64 = false, rn64 = false, rm64 = false, ra64 = false
```

### B3: encode_smaddl encodes SP/WSP as XZR/WZR

**Formal:** ∀ which ∈ 0..3, is_64 ∈ bool, a,b ∈ 0..30. encode_smaddl(valid_ops with slot which replaced by SP if is_64 else WSP) is Err
**Contract evidence:** inferred (ARM ARM register 31 in SMADDL is ZR not SP; rustdoc names Xd/Xa)
**Documentation conflict:** (none) — parse_reg_num maps SP and XZR both to 31; encode_smaddl does not distinguish them
**Severity:** medium
**Counterexample:** encode_smaddl([Reg("wsp"), Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x9b20001f))
**Impact:** Using the stack pointer as a multiply-add operand is silently rewritten to the zero register.
**Root cause:** encoder/mod.rs:270 parse_reg_num maps "sp"/"wsp" to 31; encode_smaddl:654-657 does not reject SP.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject SP/WSP in every SMADDL slot.
```rust
    if name.eq_ignore_ascii_case("sp") || name.eq_ignore_ascii_case("wsp") {
        return Err("smaddl: SP/WSP is not a valid operand".into());
    }
```
**Bug report:** bug_reports/encode_smaddl_sp_as_zr.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
Test failed: SP/WSP is not a valid SMADDL operand (which=0 names=["wsp", "w0", "w0", "x0"]) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:453.
minimal failing input: which = 0, is_64 = false, a = 0, b = 0
```

### B4: encode_smaddl accepts FP/SIMD registers as GPRs

**Formal:** ∀ which ∈ 0..3, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_smaddl(valid_ops with slot which = prefix∥n) is Err
**Contract evidence:** documented data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)"
**Documentation conflict:** data_processing.rs:652 states GPR operand names; parse_reg_num accepts d/s/q/v/h/b. The comment is the contract the body violates.
**Severity:** medium
**Counterexample:** encode_smaddl([Reg("d0"), Reg("w1"), Reg("w2"), Reg("x3")])
**Expected / Actual:** Err / Ok(Word(0x9b220c20))
**Impact:** A mistyped SIMD register is silently treated as the same-numbered GPR.
**Root cause:** encoder/mod.rs:276 parse_reg_num accepts FP prefixes; encode_smaddl:654-657 does not reject them.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:654`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject FP/SIMD register names in every SMADDL slot.
```rust
    if is_fp_reg(name) {
        return Err(format!("smaddl: FP/SIMD register {} is not a GPR", name));
    }
```
**Bug report:** bug_reports/encode_smaddl_fp_as_gpr.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
Test failed: FP/SIMD register d0 is not a valid SMADDL operand (which=0) at src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs:521.
minimal failing input: which = 0, prefix = "d", n = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs | 13 properties + 4 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_smaddl_pbt;` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_smaddl -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_extra_operand -- --test-threads=1
```

B2 wrong width:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_wrong_width -- --test-threads=1
```

B3 SP as ZR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_sp -- --test-threads=1
```

B4 FP as GPR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_smaddl_regression_fp -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_smaddl_extra_operand.md
- pbt-out/bug_reports/encode_smaddl_extra_operand.html
- pbt-out/bug_reports/encode_smaddl_wrong_width.md
- pbt-out/bug_reports/encode_smaddl_wrong_width.html
- pbt-out/bug_reports/encode_smaddl_sp_as_zr.md
- pbt-out/bug_reports/encode_smaddl_sp_as_zr.html
- pbt-out/bug_reports/encode_smaddl_fp_as_gpr.md
- pbt-out/bug_reports/encode_smaddl_fp_as_gpr.html
- pbt-out/run/encode_smaddl_round1.log
- pbt-out/run/encode_smaddl_round2.log
- pbt-out/run/encode_smaddl_round3.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 07:32 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 170/307 total | PBT candidates: 170 | Tested: 170 (100%) | 1 pass, 170 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 170 |
| **Tested (of PBT candidates)** | **170 / 170 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 170 / -1 |
| **Overall (tested / all functions)** | **170 / 307 (55%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 170 | 170 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 170 | 170 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 26 | 26 | 100% | covered |
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
| encode_smaddl | data_processing.rs |
