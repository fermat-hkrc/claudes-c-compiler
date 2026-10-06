# PBT Campaign Report: encode_uxth

## Summary

**Verdict:** 5 medium: encode_uxth encodes an X destination as 64-bit UBFM instead of the 32-bit UXTH alias, and silently encodes extra operands, X-register sources, SP/WSP, and FP/SIMD registers instead of rejecting them the way llvm-mc / GNU as do.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_uxth
**Tests:** 12 properties (plus 3 KAT + 5 regression witnesses)
**Result:** 7 passing, 5 bugs
**Change surface:** 1 changed function (encode_uxth), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps found no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_uxth NOT LINKED). Sweep was a manual arm audit of the 7-line body.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_uxth | 12 properties (7 passing / 5 failing) + 3 KAT + 5 regressions | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_uxth emits 64-bit UBFM for an X destination

**Formal:** ∀ rd ∈ 0..31, rn ∈ 0..31, dest64 ∈ {0,1}. let dest = GPR(dest64, rd); src = W(rn). encode_uxth([Reg(dest), Reg(src)]) = llvm_mc("uxth dest, src") as Word
**Contract evidence:** inferred (README.md:12 gas-compat; ARM C6 UXTH is 32-bit-only UBFM Wd,Wn,#0,#15; llvm-mc/gas canonicalize `uxth x0, w0` to `uxth w0, w0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_uxth([Reg("x0"), Reg("w0")])
**Expected / Actual:** Ok(Word(0x53003C00)) / Ok(Word(0xD3403C00))
**Impact:** `uxth x0, w0` assembles as `ubfx x0, x0, #0, #16`; object files diverge from GNU as / llvm-mc.
**Root cause:** data_processing.rs:892-895 takes sf and N from Rd's width, so an X destination sets the 64-bit UBFM form.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:892`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let sf = sf_bit(is_64);
    let n = if is_64 { 1u32 } else { 0 };
```
**Suggested fix:** Always encode the 32-bit UXTH form; do not take sf from Rd.
```rust
    let (rd, _is_64) = get_reg(operands, 0)?;
    let (rn, rn_is_64) = get_reg(operands, 1)?;
    if rn_is_64 {
        return Err("uxth: source must be a W register".to_string());
    }
    let word = (0b10u32 << 29) | (0b100110 << 23) | (15 << 10) | (rn << 5) | rd;
```
**Bug report:** bug_reports/encode_uxth_x_dest.md
**Repro seed:** rd = 0, rn = 0, dest64 = true, use_lr = false
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `3544202240`,
 right: `1392524288`: UXTH mismatch for uxth x0, w0 at src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:233.
minimal failing input: rd = 0, rn = 0, dest64 = true, use_lr = false
```

### B2: encode_uxth silently ignores a 3rd operand

**Formal:** ∀ rd ∈ 0..31, rn ∈ 0..31, extra ∈ Operand. encode_uxth([W(rd), W(rn), extra]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects `uxth w0, w1, x0`; ARM C6 UXTH is two-operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_uxth([Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x53003C00))
**Impact:** Illegal syntax such as `uxth w0, w0, x0` assembles as two-operand UXTH; a leftover operand is silently dropped.
**Root cause:** data_processing.rs:892-893 reads only operands 0 and 1 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at data_processing.rs:897.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:892`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 2 {
        return Err(format!("uxth: expected 2 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_uxth_extra_operand.md
**Repro seed:** rd = 0, rn = 0, extra = Reg("x0")
**Raw output:**
```text
Test failed: UXTH has no 3rd operand; extra operand must Err (llvm-mc rejects it) at src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:310.
minimal failing input: rd = 0, rn = 0, extra = Reg("x0")
```

### B3: encode_uxth accepts an X-register source

**Formal:** ∀ rd ∈ 0..31, rn ∈ 0..31, dest64 ∈ {0,1}. encode_uxth([Reg(GPR(dest64, rd)), Reg(X(rn))]) is Err
**Contract evidence:** inferred (ARM C6 source is Wn; llvm-mc/gas reject `uxth w0, x0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_uxth([Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x53003C00))
**Impact:** Mixed-width `uxth w0, x0` is encoded as 32-bit UXTH with Rn taken from the X register number.
**Root cause:** data_processing.rs:893 binds `(rn, _)` and discards the source width.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:893`
```rust
    let (rn, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require a W-register source.
```rust
    let (rn, rn_is_64) = get_reg(operands, 1)?;
    if rn_is_64 {
        return Err("uxth: source must be a W register".to_string());
    }
```
**Bug report:** bug_reports/encode_uxth_x_src.md
**Repro seed:** rd = 0, rn = 0, dest64 = false
**Raw output:**
```text
Test failed: UXTH w0, x0 must Err (llvm-mc rejects X-register source) at src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:323.
minimal failing input: rd = 0, rn = 0, dest64 = false
```

### B4: encode_uxth treats SP/WSP as ZR

**Formal:** ∀ which ∈ {0,1}, sp ∈ {sp, wsp}, a ∈ 0..30. encode_uxth with SP/WSP in slot which is Err
**Contract evidence:** inferred (ARM C6 Rd/Rn are ZR not SP; llvm-mc rejects `uxth wsp, w0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_uxth([Reg("wsp"), Reg("w0")])
**Expected / Actual:** Err / Ok(Word)
**Impact:** `uxth wsp, w0` encodes as `uxth wzr, w0`; using the stack pointer as a UXTH operand is silently rewritten to ZR.
**Root cause:** data_processing.rs:892 calls get_reg → parse_reg_num, which maps SP/WSP and XZR/WZR both to 31; encode_uxth does not distinguish them.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:892`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject SP/WSP before encoding.
```rust
    fn is_sp(name: &str) -> bool {
        matches!(name.to_ascii_lowercase().as_str(), "sp" | "wsp")
    }
    if operands.iter().any(|op| matches!(op, Operand::Reg(n) if is_sp(n))) {
        return Err("uxth: SP/WSP is not a valid operand".to_string());
    }
```
**Bug report:** bug_reports/encode_uxth_sp.md
**Repro seed:** which = 0, is_64_sp = false, a = 0, dest64 = false
**Raw output:**
```text
Test failed: SP/WSP is not a valid UXTH operand (which=0 names=["wsp", "w0"]) at src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:345.
minimal failing input: which = 0, is_64_sp = false, a = 0, dest64 = false
```

### B5: encode_uxth accepts FP/SIMD registers as GPRs

**Formal:** ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_uxth with Reg(prefix+n) in slot which is Err
**Contract evidence:** inferred (UXTH operands are GPRs; llvm-mc rejects `uxth d0, w1`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_uxth([Reg("d0"), Reg("w1")])
**Expected / Actual:** Err / Ok(Word(0x53003C20))
**Impact:** `uxth d0, w1` encodes as `uxth w0, w1` because parse_reg_num accepts the `d` prefix and returns 0.
**Root cause:** data_processing.rs:892 calls get_reg → parse_reg_num, which accepts prefixes d/s/q/v/h/b; encode_uxth does not reject FP/SIMD names.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:892`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject FP/SIMD register names before encoding.
```rust
    fn is_fp_name(name: &str) -> bool {
        matches!(name.chars().next().unwrap_or(' ').to_ascii_lowercase(), 'd' | 's' | 'q' | 'v' | 'h' | 'b')
    }
    if operands.iter().any(|op| matches!(op, Operand::Reg(n) if is_fp_name(n))) {
        return Err("uxth: FP/SIMD register is not a valid operand".to_string());
    }
```
**Bug report:** bug_reports/encode_uxth_fp.md
**Repro seed:** which = 0, prefix = "d", n = 0
**Raw output:**
```text
Test failed: FP/SIMD register d0 is not a valid UXTH operand (which=0) at src/backend/arm/assembler/encoder/encode_uxth_pbt.rs:365.
minimal failing input: which = 0, prefix = "d", n = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_uxth_pbt.rs | 12 properties + 3 KAT + 5 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_uxth -- --test-threads=1
```

B1 (X dest as 64-bit UBFM):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxth_regression_x_dest -- --test-threads=1
```

B2 (extra operand):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxth_regression_extra_operand -- --test-threads=1
```

B3 (X source):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxth_regression_x_src -- --test-threads=1
```

B4 (SP/WSP):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxth_regression_sp -- --test-threads=1
```

B5 (FP/SIMD):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_uxth_regression_fp -- --test-threads=1
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
- pbt-out/bug_reports/encode_uxth_x_dest.md
- pbt-out/bug_reports/encode_uxth_x_dest.html
- pbt-out/bug_reports/encode_uxth_extra_operand.md
- pbt-out/bug_reports/encode_uxth_extra_operand.html
- pbt-out/bug_reports/encode_uxth_x_src.md
- pbt-out/bug_reports/encode_uxth_x_src.html
- pbt-out/bug_reports/encode_uxth_sp.md
- pbt-out/bug_reports/encode_uxth_sp.html
- pbt-out/bug_reports/encode_uxth_fp.md
- pbt-out/bug_reports/encode_uxth_fp.html
- pbt-out/run/encode_uxth_round1.log
- pbt-out/run/encode_uxth_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 08:35 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 173/307 total | PBT candidates: 173 | Tested: 173 (100%) | 1 pass, 173 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 173 |
| **Tested (of PBT candidates)** | **173 / 173 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 173 / -1 |
| **Overall (tested / all functions)** | **173 / 307 (56%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 173 | 173 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 173 | 173 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 29 | 29 | 100% | covered |
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
| encode_uxth | data_processing.rs |
