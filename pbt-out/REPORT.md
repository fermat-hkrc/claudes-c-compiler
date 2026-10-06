# PBT Campaign Report: encode_sxtb

## Summary

**Verdict:** 4 medium: encode_sxtb silently encodes extra operands, SXTB Wd,Xn, SP/WSP, and FP/SIMD registers instead of rejecting them the way llvm-mc / GNU as do, so illegal assembly becomes a well-formed SBFM word.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_sxtb
**Tests:** 12 properties (plus 4 KAT + 4 regression witnesses)
**Result:** 8 passing, 4 bugs
**Change surface:** 1 changed function (encode_sxtb), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps found no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_sxtb NOT LINKED). Sweep was a manual arm audit of the 8-line body.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_sxtb | 12 properties (8 passing / 4 failing) + 4 KAT + 4 regressions | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_sxtb silently ignores a 3rd operand

**Formal:** ∀ rd, rn ∈ {0..31}, is_64 ∈ Bool, extra ∈ Operand. encode_sxtb([Rd, Wn, extra]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects `sxtb w0, w1, x0`; ARM C6 SXTB is two-operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_sxtb([Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x13001c00))
**Impact:** Illegal syntax such as `sxtb w0, w0, x0` assembles as two-operand SXTB; a leftover operand is silently dropped.
**Root cause:** data_processing.rs:873-874 reads only operands 0 and 1 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at data_processing.rs:878.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:873`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 2 {
        return Err(format!("sxtb: expected 2 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_sxtb_extra_operand.md
**Repro seed:** rd = 0, rn = 0, is_64 = false, extra = Reg("x0")
**Raw output:**
```text
Test failed: SXTB has no 3rd operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:326.
minimal failing input: rd = 0, rn = 0, is_64 = false, extra = Reg("x0")
```

### B2: encode_sxtb accepts SXTB Wd, Xn

**Formal:** ∀ rd, rn ∈ {0..31}. encode_sxtb([Reg(wreg(rd)), Reg(xreg(rn))]) is Err
**Contract evidence:** inferred (ARM C6 source is Wn; llvm-mc rejects `sxtb w0, x0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_sxtb([Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x13001c00))
**Impact:** Mixed-width `sxtb w0, x0` is encoded as 32-bit SXTB with Rn taken from the X register number.
**Root cause:** data_processing.rs:874 binds `(rn, _)` and discards the source width; sf comes only from Rd at data_processing.rs:873-875.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:874`
```rust
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require a W-register source when the destination is 32-bit.
```rust
    let (rn, rn_is_64) = get_reg(operands, 1)?;
    if !is_64 && rn_is_64 {
        return Err("sxtb: 32-bit dest requires Wn source".to_string());
    }
```
**Bug report:** bug_reports/encode_sxtb_wd_xn.md
**Repro seed:** rd = 0, rn = 0
**Raw output:**
```text
Test failed: SXTB Wd, Xn must Err (llvm-mc rejects sxtb w0, x0) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:338.
minimal failing input: rd = 0, rn = 0
```

### B3: encode_sxtb treats SP/WSP as ZR

**Formal:** ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_sxtb with SP/WSP in slot which is Err
**Contract evidence:** inferred (ARM C6 register 31 is ZR not SP; llvm-mc rejects `sxtb wsp, w0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_sxtb([Reg("wsp"), Reg("w0")])
**Expected / Actual:** Err / Ok(Word(0x13001c1f))
**Impact:** `sxtb wsp, w0` is encoded as `sxtb wzr, w0` (rd=31); stack-pointer operands are silently rewritten to the zero register.
**Root cause:** encode_sxtb does not distinguish SP from ZR; get_reg/parse_reg_num map both to 31, and data_processing.rs:878 returns Ok(Word) with rd=31.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:873`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP/WSP by name before encoding.
```rust
    if matches!(name.to_lowercase().as_str(), "sp" | "wsp") {
        return Err("sxtb: SP/WSP is not a valid operand (use ZR)".to_string());
    }
```
**Bug report:** bug_reports/encode_sxtb_sp.md
**Repro seed:** which = 0, is_64_sp = false, a = 0, dest64 = false
**Raw output:**
```text
Test failed: SP/WSP is not a valid SXTB operand (which=0 names=["wsp", "w0"]) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:360.
minimal failing input: which = 0, is_64_sp = false, a = 0, dest64 = false
```

### B4: encode_sxtb accepts FP/SIMD registers as GPRs

**Formal:** ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_sxtb with FP register prefix+n in slot which is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects `sxtb d0, w1`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_sxtb([Reg("d0"), Reg("w1")])
**Expected / Actual:** Err / Ok(Word(0x13001c20))
**Impact:** `sxtb d0, w1` becomes the same word as `sxtb w0, w1`; illegal SIMD operands assemble as GPR SXTB.
**Root cause:** encode_sxtb does not reject FP/SIMD names; parse_reg_num maps prefix+N to N, and data_processing.rs:878 returns Ok(Word) with rd=0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:873`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject floating-point / SIMD register names.
```rust
    if is_fp_reg(name) {
        return Err(format!("sxtb: FP/SIMD register {name} is not a valid operand"));
    }
```
**Bug report:** bug_reports/encode_sxtb_fp.md
**Repro seed:** which = 0, prefix = "d", n = 0
**Raw output:**
```text
Test failed: FP/SIMD register d0 is not a valid SXTB operand (which=0) at src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs:380.
minimal failing input: which = 0, prefix = "d", n = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_sxtb_pbt.rs | 12 properties + 4 KAT + 4 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_sxtb -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_extra_operand -- --test-threads=1
```

B2 Wd,Xn:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_wd_xn -- --test-threads=1
```

B3 SP/WSP:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_sp -- --test-threads=1
```

B4 FP/SIMD:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_sxtb_regression_fp -- --test-threads=1
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
- pbt-out/bug_reports/encode_sxtb_extra_operand.md
- pbt-out/bug_reports/encode_sxtb_extra_operand.html
- pbt-out/bug_reports/encode_sxtb_wd_xn.md
- pbt-out/bug_reports/encode_sxtb_wd_xn.html
- pbt-out/bug_reports/encode_sxtb_sp.md
- pbt-out/bug_reports/encode_sxtb_sp.html
- pbt-out/bug_reports/encode_sxtb_fp.md
- pbt-out/bug_reports/encode_sxtb_fp.html
- pbt-out/run/encode_sxtb_round1.log
- pbt-out/run/encode_sxtb_round2.log
- proptest-regressions/backend/arm/assembler/encoder/encode_sxtb_pbt.txt

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 08:17 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 172/307 total | PBT candidates: 172 | Tested: 172 (100%) | 1 pass, 172 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 172 |
| **Tested (of PBT candidates)** | **172 / 172 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 172 / -1 |
| **Overall (tested / all functions)** | **172 / 307 (56%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 172 | 172 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 172 | 172 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 28 | 28 | 100% | covered |
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
| encode_mneg | data_processing.rs |
| encode_sxtb | data_processing.rs |
