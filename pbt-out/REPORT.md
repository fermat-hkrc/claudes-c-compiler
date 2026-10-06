# PBT Campaign Report: encode_dc

## Summary

**Verdict:** 4 medium: encode_dc silently encodes unknown substring op names (civacs/gzva), a missing Xt as x0, extra operands, and W/SP/SIMD registers, so typos assemble as real cache-maintenance instructions.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_dc
**Tests:** 10 properties (plus 7 KAT + 5 regression witnesses)
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_dc), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit plus two sweep properties drove the invalid-register and non-substring Err arms. Tier: standard.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_dc | 10 properties (7 KAT, 5 regression) | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_dc matches DC op names by substring

**Formal:** ∀ name ∉ {civac,cvac,cvap,cvau,ivac,zva} (after trim+casefold of first comma field). llvm-mc("dc name, x0") = Err ⇒ encode_dc([Symbol(name), Reg("x0")], raw) = Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc/gas reject unknown DC names; body uses contains() without documenting substring matching)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_dc([Symbol("civacs"), Reg("x0")], "civacs, x0")
**Expected / Actual:** Err / Ok(Word(0xd50b7e20)) (DC CIVAC, x0). Related: gzva, x0 → Ok(Word(0xd50b7420)) (DC ZVA)
**Impact:** Typos and other ARM DC names that merely contain an implemented token assemble as the wrong cache op. `dc gzva, x0` becomes DC ZVA (zero cache line) instead of being rejected.
**Root cause:** system.rs:583 uses `op.contains("civac")` (and later contains("cvac")/contains("zva")) instead of an exact match.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:583`
```rust
    if op.contains("civac") {
```
**Suggested fix:** Match the trimmed op name exactly, not as a substring.
```rust
    if op.trim() == "civac" {
```
**Bug report:** bug_reports/encode_dc_substring_op.md
**Repro seed:** s = "civacs"
**Raw output:**
```text
Test failed: unknown DC op "civacs" must Err (llvm-mc rejects dc civacs, x0): Word(3574300192).
minimal failing input: s = "civacs"
```

### B2: encode_dc encodes a missing Xt as x0

**Formal:** ∀ op ∈ {civac,cvac,cvap,cvau,ivac,zva}. encode_dc([Symbol(op)], op) = Err
**Contract evidence:** inferred (ARM ARM SYS Xt required; llvm-mc/gas reject `dc civac`; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_dc([Symbol("civac")], "civac")
**Expected / Actual:** Err / Ok(Word(0xd50b7e20)) (DC CIVAC, x0)
**Impact:** A dropped Xt is not diagnosed and silently targets x0.
**Root cause:** system.rs:578 defaults Rt to 0 when no register operand is present.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:578`
```rust
                0
```
**Suggested fix:** Require a register operand; do not default Rt to x0.
```rust
                return Err("dc: missing Xt register".into())
```
**Bug report:** bug_reports/encode_dc_missing_xt.md
**Repro seed:** op = "civac"
**Raw output:**
```text
Test failed: DC civac without Xt must Err (llvm-mc rejects dc civac); SUT raw "civac": Word(3574300192).
minimal failing input: op = "civac"
```

### B3: encode_dc ignores a third operand

**Formal:** ∀ op ∈ six names, ∀ xt ∈ X-regs, ∀ extra. encode_dc([Symbol(op), Reg(xt), extra], raw) = Err
**Contract evidence:** inferred (llvm-mc/gas reject extra operands; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_dc([Symbol("civac"), Reg("x0"), Reg("x0")], "civac, x0, x0")
**Expected / Actual:** Err / Ok(Word(0xd50b7e20)) (DC CIVAC, x0)
**Impact:** A stray third operand is not diagnosed.
**Root cause:** system.rs:572 takes Rt from operands.get(1) and never checks operands.len().
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:572`
```rust
    let rt = match operands.get(1) {
```
**Suggested fix:** Reject anything other than exactly two operands (op, Xt).
```rust
    if operands.len() != 2 {
        return Err(format!("dc: unexpected extra operand in {}", raw_operands));
    }
    let rt = match operands.get(1) {
```
**Bug report:** bug_reports/encode_dc_extra_operand.md
**Repro seed:** op = "civac", xt = "x0", extra = "x0"
**Raw output:**
```text
Test failed: DC with extra operand must Err (llvm-mc rejects dc civac, x0, x0); SUT raw "civac, x0, x0": Word(3574300192).
minimal failing input: op = "civac", xt = "x0", extra = "x0"
```

### B4: encode_dc accepts W, SP, and SIMD registers as Xt

**Formal:** ∀ op ∈ six names, ∀ bad ∈ {w0..w30, wzr, wsp, sp, d/s/q/v/h/b0..31}. encode_dc([Symbol(op), Reg(bad)], raw) = Err
**Contract evidence:** inferred (llvm-mc invalid operand; gas operand mismatch / must be an integer register; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_dc([Symbol("civac"), Reg("w0")], "civac, w0")
**Expected / Actual:** Err / Ok(Word(0xd50b7e20)) (DC CIVAC, x0). Related: sp → Ok(Word(0xd50b7e3f)) (XZR)
**Impact:** A 32-bit, stack, or SIMD register is silently treated as the corresponding 5-bit number.
**Root cause:** system.rs:573 calls parse_reg_num, which accepts w/sp/wsp/d/s/q/v/h/b, with no 64-bit GPR check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:573`
```rust
        Some(Operand::Reg(name)) => parse_reg_num(name).ok_or("invalid register for dc")?,
```
**Suggested fix:** Reject non-X GPRs (W, SP, SIMD) before encoding.
```rust
        Some(Operand::Reg(name)) => {
            let low = name.trim().to_lowercase();
            let is_x = low.starts_with('x') || low == "xzr" || low == "lr";
            if !is_x {
                return Err(format!("dc: Xt must be a 64-bit GPR, got {name}"));
            }
            parse_reg_num(name).ok_or("invalid register for dc")?
        }
```
**Bug report:** bug_reports/encode_dc_wrong_reg_class.md
**Repro seed:** op = "civac", bad = "w0"
**Raw output:**
```text
Test failed: DC with non-X register must Err (llvm-mc rejects dc civac, w0); SUT raw "civac, w0": Word(3574300192).
minimal failing input: op = "civac", bad = "w0"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_dc_pbt.rs | 10 properties, 7 KAT, 5 regression witnesses |

## Reproduction

Valid-domain / full suite (fails on the four bugs):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_dc -- --test-threads=1
```

B1 substring:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_gzva -- --test-threads=1 --nocapture
```

B2 missing Xt:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_missing_xt -- --test-threads=1 --nocapture
```

B3 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_extra_operand -- --test-threads=1 --nocapture
```

B4 wrong register class:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_dc_regression_w0 -- --test-threads=1 --nocapture
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
- pbt-out/run/encode_dc_test.log
- pbt-out/run/encode_dc_test_sweep.log
- pbt-out/bug_reports/encode_dc_substring_op.md
- pbt-out/bug_reports/encode_dc_substring_op.html
- pbt-out/bug_reports/encode_dc_missing_xt.md
- pbt-out/bug_reports/encode_dc_missing_xt.html
- pbt-out/bug_reports/encode_dc_extra_operand.md
- pbt-out/bug_reports/encode_dc_extra_operand.html
- pbt-out/bug_reports/encode_dc_wrong_reg_class.md
- pbt-out/bug_reports/encode_dc_wrong_reg_class.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 02:35 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 159/307 total | PBT candidates: 159 | Tested: 159 (100%) | 1 pass, 159 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 159 |
| **Tested (of PBT candidates)** | **159 / 159 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 159 / -1 |
| **Overall (tested / all functions)** | **159 / 307 (52%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 159 | 159 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 159 | 159 | 0 | 100% |

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
