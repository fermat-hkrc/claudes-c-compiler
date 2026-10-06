# PBT Campaign Report: encode_at

## Summary

**Verdict:** 2 medium and 1 low (documented by the author): encode_at encodes a missing Xt as XZR, treats W/SP/SIMD as 64-bit GPRs, and rejects ARM AT ops S1E2/S1E3/S12E* that gas/llvm-mc assemble.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_at
**Tests:** 10
**Result:** 7 passing, 3 bugs
**Change surface:** 1 changed function (encode_at), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). encode_at ran via `cargo test --lib encode_at_`; sweep was a manual arm audit of the 20-line body plus encode_at_neg_invalid_reg.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_at | 10 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_at encodes AT without Xt as AT XZR

**Formal:** ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ case/ws of op with no comma. llvm-mc("at "+raw) fails ∧ encode_at([], raw) = Err(_)
**Contract evidence:** inferred (ARM ARM AT requires Xt; llvm-mc "specified at op requires a register"; gas "comma expected between operands at operand 2"; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_at(&[], "s1e1r") then llvm-mc("at s1e1r")
**Expected / Actual:** Err / Ok(Word(0xd508781f))
**Impact:** `at s1e1r` with a missing address register assembles as `at s1e1r, xzr`. A dropped operand becomes a real address-translate instruction against XZR instead of an assembler error.
**Root cause:** system.rs:432-434 default Rt to 31 (XZR) whenever raw_operands contains no comma. Every ARM AT op requires Xt, so this default is never valid.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:432`
```rust
    } else {
        31
    };
```
**Suggested fix:** Require a register operand for every AT op.
```rust
    } else {
        return Err("at: operation requires a register".to_string());
    };
```
**Bug report:** bug_reports/encode_at_missing_xt.md
**Repro seed:** cc bdf1400b0d97cdf266f4f651c302f6aadeaff2e8393d3bd6a8c8ea68c0546865 (raw = "s1e1r")
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_at_pbt::encode_at_neg_missing_reg' (2583788) panicked at src/backend/arm/assembler/encoder/encode_at_pbt.rs:337:1:
Test failed: AT without register must Err (llvm-mc rejects at s1e1r); SUT raw "s1e1r": Word(3574102047).
minimal failing input: raw = "s1e1r"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_at accepts W/SP/SIMD registers as AT Xt

**Formal:** ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ bad ∈ {w0..w30,wzr,wsp,sp,w31,dN,sN,qN,vN,hN,bN}. llvm-mc("at op, bad") fails ∧ encode_at([], op+", "+bad) = Err(_)
**Contract evidence:** inferred (ARM ARM AT Xt is a 64-bit GPR; llvm-mc "invalid operand for instruction"; gas "operand mismatch" / "must be an integer register"; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_at(&[], "s1e1r, w0") then llvm-mc("at s1e1r, w0")
**Expected / Actual:** Err / Ok(Word(0xd5087800))
**Impact:** `at s1e1r, w0` encodes identically to `at s1e1r, x0`. A 32-bit, SP, or SIMD register is silently treated as the corresponding 5-bit encoding, so a width/class typo is not diagnosed.
**Root cause:** system.rs:431 uses parse_reg_num, which accepts W/SP/WZR/WSP and SIMD/FP prefixes (d/s/q/v/h/b) as 5-bit numbers, with no 64-bit GPR check for AT Xt.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:431`
```rust
        parse_reg_num(reg_str).ok_or_else(|| format!("at: invalid register '{}'", reg_str))?
```
**Suggested fix:** Restrict AT Xt to 64-bit integer registers (Xn / XZR / LR).
```rust
        let rt = parse_reg_num(reg_str).ok_or_else(|| format!("at: invalid register '{}'", reg_str))?;
        let low = reg_str.trim().to_lowercase();
        let is_x = low.starts_with('x') || low == "xzr" || low == "lr";
        if !is_x {
            return Err(format!("at: Xt must be a 64-bit GPR, got '{}'", reg_str));
        }
```
**Bug report:** bug_reports/encode_at_wrong_reg_class.md
**Repro seed:** op = "s1e1r", bad = "w0"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_at_pbt::encode_at_neg_wrong_reg_class' (2584808) panicked at src/backend/arm/assembler/encoder/encode_at_pbt.rs:337:1:
Test failed: AT with non-X register must Err (llvm-mc rejects at s1e1r, w0); SUT raw "s1e1r, w0": Word(3574102016).
minimal failing input: op = "s1e1r", bad = "w0"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_at rejects ARM AT ops S1E2/S1E3/S12E*

**Formal:** ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w,s1e2r,s1e2w,s1e3r,s1e3w,s12e1r,s12e1w,s12e0r,s12e0w}, ∀ t ∈ 0..31. encode_at([], op+", xt") = llvm-mc("at "+op+", xt")
**Contract evidence:** documented limitation encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen."
**Documentation conflict:** encoder/mod.rs:4 admits a subset on an input the assembler API accepts (README.md:12 gas-compat; llvm-mc/gas assemble `at s1e2r, x0`). The comment documents the limitation rather than declaring S1E2/S1E3/S12E* invalid.
**Severity:** low (documented by the author)
**Counterexample:** encode_at(&[], "s1e2r, x0") then llvm-mc("at s1e2r, x0")
**Expected / Actual:** Ok(Word(0xd50c7800)) / Err("unsupported at operation: s1e2r")
**Impact:** Kernel/hypervisor assembly using AT at EL2/EL3 or stage-1+2 fails to assemble though gas accepts it.
**Root cause:** system.rs:436-441 match table only lists S1E1R/S1E1W/S1E0R/S1E0W; every other ARM AT op hits the `_` arm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:441`
```rust
        _ => return Err(format!("unsupported at operation: {}", op_name)),
```
**Suggested fix:** Encode the remaining default-CPU AT ops as SYS with CRn=7, CRm=8.
```rust
        "s1e2r" => 0xd50c7800,
        "s1e2w" => 0xd50c7820,
        "s1e3r" => 0xd50e7800,
        "s1e3w" => 0xd50e7820,
        "s12e1r" => 0xd50c7880,
        "s12e1w" => 0xd50c78a0,
        "s12e0r" => 0xd50c78c0,
        "s12e0w" => 0xd50c78e0,
```
**Bug report:** bug_reports/encode_at_unimplemented_ops.md
**Repro seed:** cc e5f189a171b0943795b039cb175abcb5949161c23f40c8c212614d6bc4e95d05 (op = "s1e2r", t = 0)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_at_pbt::encode_at_diff_arm_ops' (2586973) panicked at src/backend/arm/assembler/encoder/encode_at_pbt.rs:377:1:
Test failed: ARM AT op "s1e2r" must encode (llvm-mc accepts at s1e2r, x0): "unsupported at operation: s1e2r".
minimal failing input: op = "s1e2r", t = 0
	successes: 1
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_at_pbt.rs | 10 properties + 5 KAT + 3 regression witnesses |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_at_ -- --test-threads=1
```

B1 missing Xt:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_at_regression_missing_xt -- --test-threads=1 --nocapture
```

B2 wrong register class:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_at_regression_w0 -- --test-threads=1 --nocapture
```

B3 unimplemented AT ops:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_at_regression_s1e2r -- --test-threads=1 --nocapture
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
- pbt-out/bug_reports/encode_at_missing_xt.md
- pbt-out/bug_reports/encode_at_missing_xt.html
- pbt-out/bug_reports/encode_at_wrong_reg_class.md
- pbt-out/bug_reports/encode_at_wrong_reg_class.html
- pbt-out/bug_reports/encode_at_unimplemented_ops.md
- pbt-out/bug_reports/encode_at_unimplemented_ops.html
- pbt-out/run/encode_at.log
- src/backend/arm/assembler/encoder/encode_at_pbt.rs
- proptest-regressions/backend/arm/assembler/encoder/encode_at_pbt.txt

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 03:29 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 161/307 total | PBT candidates: 161 | Tested: 161 (100%) | 1 pass, 161 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 161 |
| **Tested (of PBT candidates)** | **161 / 161 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 161 / -1 |
| **Overall (tested / all functions)** | **161 / 307 (52%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 161 | 161 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 161 | 161 | 0 | 100% |

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
| encode_at | system.rs |
