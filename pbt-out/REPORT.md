# PBT Campaign Report: encode_ic

## Summary

**Verdict:** 3 medium: encode_ic silently encodes invalid IC syntax that llvm-mc and GNU as reject — a register on IALLUIS/IALLU, a missing Xt on IVAU (becomes IVAU XZR), and W/SP/SIMD as IVAU Xt (W0 encodes as X0).
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ic
**Tests:** 9 properties + 4 KAT + 3 regression witnesses
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function (encode_ic), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of the 17-line body plus a sweep property on the invalid-register Err path.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ic | 9 properties (6 passing / 3 failing) + 4 KAT + 3 regression | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_ic accepts a register on IALLUIS/IALLU

**Formal:** ∀ op ∈ {ialluis, iallu}. ∀ t ∈ 0..31 ∪ {xzr, lr, sp}. llvm-mc("ic " · op · ", " · xt(t)) = Err ⇒ encode_ic(op · ", " · xt(t)) = Err
**Contract evidence:** inferred (README.md:12 gas-compat; ARM ARM IALLUIS/IALLU have no Xt; llvm-mc "specified ic op does not use a register"; gas "extraneous register at operand 2")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ic("ialluis, x0")
**Expected / Actual:** Err / Ok(Word(0xd5087100))
**Impact:** A stray register on IALLUIS/IALLU is patched into Rt instead of being rejected, so a typo is not diagnosed.
**Root cause:** system.rs:416 always writes `(base & !0x1F) | rt` after parsing an optional register; IALLUIS/IALLU never check that a second operand is forbidden.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:416`
```rust
    let word = (base & !0x1F) | rt;
```
**Suggested fix:** Reject a register operand for IALLUIS and IALLU before encoding.
```rust
    if matches!(op_name.as_str(), "ialluis" | "iallu") && parts.len() > 1 {
        return Err(format!("ic: {} does not take a register", op_name));
    }
    let word = (base & !0x1F) | rt;
```
**Bug report:** bug_reports/encode_ic_iallu_with_reg.md
**Repro seed:** op = "ialluis", xt = "x0"
**Raw output:**
```text
Test failed: IALLU* with register must Err (llvm-mc rejects ic ialluis, x0); SUT raw "ialluis, x0": Word(3574100224).
minimal failing input: op = "ialluis", xt = "x0"
```

### B2: encode_ic encodes IC IVAU without Xt as IVAU XZR

**Formal:** ∀ pad ∈ {ε, space, tab}*. ∀ case ∈ ASCII-case-fold("ivau"). llvm-mc("ic " · pad · case · pad) = Err ⇒ encode_ic(pad · case · pad) = Err
**Contract evidence:** inferred (README.md:12 gas-compat; ARM ARM IVAU requires Xt; llvm-mc "specified ic op requires a register"; gas "missing register at operand 2")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ic("ivau")
**Expected / Actual:** Err / Ok(Word(0xd50b753f))
**Impact:** A dropped Xt becomes `ic ivau, xzr`, a real cache-maintenance instruction, instead of an assembler error.
**Root cause:** system.rs:408 default Rt to 31 (XZR) whenever raw_operands contains no comma, including for IVAU which requires Xt.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:408`
```rust
        31 // xzr
```
**Suggested fix:** Require a register operand for IVAU.
```rust
    } else if op_name == "ivau" {
        return Err("ic: ivau requires a register".to_string());
    } else {
        31 // xzr
    };
```
**Bug report:** bug_reports/encode_ic_ivau_missing_reg.md
**Repro seed:** raw = "ivau"
**Raw output:**
```text
Test failed: IVAU without register must Err (llvm-mc rejects ic ivau); SUT raw "ivau": Word(3574297919).
minimal failing input: raw = "ivau"
```

### B3: encode_ic accepts W/SP/SIMD registers as IVAU Xt

**Formal:** ∀ bad ∈ {w0..w30, wzr, wsp, sp, d0, s0, q0, v0, h0, b0}. llvm-mc("ic ivau, " · bad) = Err ⇒ encode_ic("ivau, " · bad) = Err
**Contract evidence:** inferred (README.md:12 gas-compat; ARM ARM IVAU Xt is a 64-bit GPR; llvm-mc "invalid operand for instruction"; gas "operand mismatch")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ic("ivau, w0")
**Expected / Actual:** Err / Ok(Word(0xd50b7520)) (same as `ic ivau, x0`)
**Impact:** A 32-bit, SP, or SIMD register is silently treated as the corresponding 5-bit encoding, so a width/class typo is not diagnosed.
**Root cause:** system.rs:406 uses parse_reg_num, which accepts W/SP/WZR/WSP and SIMD/FP prefixes as 5-bit numbers, with no 64-bit GPR check for IVAU Xt.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:406`
```rust
        parse_reg_num(reg_str).ok_or_else(|| format!("ic: invalid register '{}'", reg_str))?
```
**Suggested fix:** Restrict IVAU Xt to 64-bit integer registers (Xn / XZR / LR).
```rust
        let rt = parse_reg_num(reg_str).ok_or_else(|| format!("ic: invalid register '{}'", reg_str))?;
        let low = reg_str.trim().to_lowercase();
        let is_x = low.starts_with('x') || low == "xzr" || low == "lr";
        if !is_x {
            return Err(format!("ic: Xt must be a 64-bit GPR, got '{}'", reg_str));
        }
```
**Bug report:** bug_reports/encode_ic_wrong_reg_class.md
**Repro seed:** bad = "w0"
**Raw output:**
```text
Test failed: IVAU with non-X register must Err (llvm-mc rejects ic ivau, w0); SUT raw "ivau, w0": Word(3574297888).
minimal failing input: bad = "w0"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_ic_pbt.rs | 9 properties (6 passing / 3 failing) + 4 KAT + 3 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_ic_pbt` registration |

## Reproduction

Whole suite (expected: 10 passed, 6 failed — 3 properties + 3 regression witnesses):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_ic -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ic_regression_ialluis_x0 -- --test-threads=1 --nocapture
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ic_regression_ivau_missing -- --test-threads=1 --nocapture
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ic_regression_ivau_w0 -- --test-threads=1 --nocapture
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage table
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/FUNCTION_INDEX.md — function index (encode_ic marked yes)
- pbt-out/report.json — machine-readable report
- pbt-out/bug_reports/encode_ic_iallu_with_reg.md + .html
- pbt-out/bug_reports/encode_ic_ivau_missing_reg.md + .html
- pbt-out/bug_reports/encode_ic_wrong_reg_class.md + .html
- pbt-out/build.log — pre-campaign user build log
- proptest-regressions/backend/arm/assembler/encoder/encode_ic_pbt.txt — shrunk seeds

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 02:16 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 158/307 total | PBT candidates: 158 | Tested: 158 (100%) | 1 pass, 158 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 158 |
| **Tested (of PBT candidates)** | **158 / 158 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 158 / -1 |
| **Overall (tested / all functions)** | **158 / 307 (51%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 158 | 158 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 158 | 158 | 0 | 100% |

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
