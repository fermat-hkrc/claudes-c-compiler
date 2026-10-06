# PBT Campaign Report: encode_mneg

## Summary

**Verdict:** 4 medium: encode_mneg silently accepts extra operands, mixed W/X widths, SP/WSP, and FP/SIMD registers that GNU as / llvm-mc reject, so illegal MNEG syntax assembles to a 32-bit word instead of failing.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mneg
**Tests:** 13
**Result:** 9 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust lib test tree was not LLVM-coverage instrumented for the C++ reporter); it listed unrelated C++ binaries and claimed encode_mneg NOT LINKED. Sweep was a manual arm audit of the 12-line body.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mneg | 13 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_mneg silently ignores a 4th operand

**Formal:** ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool, extra ∈ Operand. encode_mneg([Rd,Rn,Rm,extra]) is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects a 4th operand with "invalid operand for instruction"; ARM ARM MNEG is 3-operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mneg([Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x1b00fc00))
**Impact:** A typo or extra operand is silently dropped; `mneg w0, w0, w0, x0` assembles as `mneg w0, w0, w0`.
**Root cause:** data_processing.rs:678-680 reads only operands 0..2 via get_reg and has no operands.len() upper bound, then returns Ok(Word) at data_processing.rs:685.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 3 {
        return Err(format!("mneg: expected 3 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_mneg_extra_operand.md
**Repro seed:** rd = 0, rn = 0, rm = 0, is_64 = false, extra = Reg("x0")
**Raw output:**
```text
Test failed: MNEG has no 4th operand; extra operand must Err (llvm-mc rejects it)
minimal failing input: rd = 0, rn = 0, rm = 0, is_64 = false, extra = Reg("x0")
```

### B2: encode_mneg accepts mixed W/X register widths

**Formal:** ∀ rd, rn, rm ∈ {0..30}, rd64, rn64, rm64 ∈ Bool. ¬(rd64=rn64=rm64) ⇒ encode_mneg([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)]) is Err
**Contract evidence:** inferred (ARM ARM same-width GPR / single sf bit; llvm-mc "invalid operand" for `mneg w0, w0, x0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mneg([Reg("w0"), Reg("w0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x1b00fc00))
**Impact:** Mixed-width typos assemble using only Rd's width, producing a 32-bit MNEG that is not what the source wrote.
**Root cause:** data_processing.rs:678-681 binds is_64 only from operand 0 and discards Rn/Rm width, then sets sf from that dest-only flag.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let sf = sf_bit(is_64);
```
**Suggested fix:** Require matching widths before encoding.
```rust
    let (rd, rd64) = get_reg(operands, 0)?;
    let (rn, rn64) = get_reg(operands, 1)?;
    let (rm, rm64) = get_reg(operands, 2)?;
    if rd64 != rn64 || rn64 != rm64 {
        return Err("mneg: register size mismatch".into());
    }
```
**Bug report:** bug_reports/encode_mneg_mixed_width.md
**Repro seed:** rd = 0, rn = 0, rm = 0, rd64 = false, rn64 = false, rm64 = true
**Raw output:**
```text
Test failed: mixed-width MNEG registers must Err (rd64=false rn64=false rm64=true)
minimal failing input: rd = 0, rn = 0, rm = 0, rd64 = false, rn64 = false, rm64 = true
```

### B3: encode_mneg encodes SP/WSP as ZR

**Formal:** ∀ which ∈ {0,1,2}, is_64 ∈ Bool, a, b ∈ {0..30}. encode_mneg(triple with slot `which` = sp/wsp) is Err
**Contract evidence:** inferred (ARM ARM register 31 is ZR not SP; llvm-mc "invalid operand" for `mneg wsp, w0, w0`)
**Documentation conflict:** (none) — parse_reg_num's "31 for sp/zr" is that helper's mapping, not encode_mneg's contract
**Severity:** medium
**Counterexample:** encode_mneg([Reg("wsp"), Reg("w0"), Reg("w0")])
**Expected / Actual:** Err / Ok(Word(0x1b00fc1f))
**Impact:** A stack-pointer operand is silently rewritten as the zero register (`mneg wsp, w0, w0` encodes as `mneg wzr, w0, w0`).
**Root cause:** data_processing.rs:678 calls get_reg / parse_reg_num, which maps both "sp"/"wsp" and "xzr"/"wzr" to 31; encode_mneg never distinguishes them.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject SP/WSP before encoding.
```rust
    if name.eq_ignore_ascii_case("sp") || name.eq_ignore_ascii_case("wsp") {
        return Err("mneg: SP/WSP is not a valid operand (use XZR/WZR)".into());
    }
```
**Bug report:** bug_reports/encode_mneg_sp_as_zr.md
**Repro seed:** which = 0, is_64 = false, a = 0, b = 0
**Raw output:**
```text
Test failed: SP/WSP is not a valid MNEG operand (which=0 names=["wsp", "w0", "w0"])
minimal failing input: which = 0, is_64 = false, a = 0, b = 0
```

### B4: encode_mneg encodes FP/SIMD registers as GPRs

**Formal:** ∀ which ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_mneg(triple with slot which = prefix||n) is Err
**Contract evidence:** inferred (MNEG operands are GPRs; llvm-mc "invalid operand" for `mneg d0, w1, w2`; is_fp_reg exists but is unused)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mneg([Reg("d0"), Reg("w1"), Reg("w2")])
**Expected / Actual:** Err / Ok(Word(0x1b02fc20))
**Impact:** An FP register typo is silently rewritten as the same-numbered GPR (`mneg d0, w1, w2` encodes as `mneg w0, w1, w2`).
**Root cause:** data_processing.rs:678 calls get_reg / parse_reg_num, which accepts d/s/q/v/h/b prefixes as GPR numbers; encode_mneg never consults is_fp_reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/data_processing.rs:678`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
```
**Suggested fix:** Reject FP/SIMD register names before encoding.
```rust
    if is_fp_reg(name) {
        return Err(format!("mneg: FP/SIMD register {} is not a valid operand", name));
    }
```
**Bug report:** bug_reports/encode_mneg_fp_as_gpr.md
**Repro seed:** which = 0, prefix = "d", n = 0, is_64 = false
**Raw output:**
```text
Test failed: FP/SIMD register d0 is not a valid MNEG operand (which=0)
minimal failing input: which = 0, prefix = "d", n = 0, is_64 = false
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_mneg_pbt.rs | 13 properties + 5 KAT + 4 regression witnesses |

## Reproduction

Whole suite (includes 4 expected property failures and 4 expected regression failures):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mneg -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_extra_operand -- --test-threads=1
```

B2 mixed width:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_mixed_width -- --test-threads=1
```

B3 SP as ZR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_sp -- --test-threads=1
```

B4 FP as GPR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mneg_regression_fp -- --test-threads=1
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
- pbt-out/bug_reports/encode_mneg_extra_operand.md
- pbt-out/bug_reports/encode_mneg_extra_operand.html
- pbt-out/bug_reports/encode_mneg_mixed_width.md
- pbt-out/bug_reports/encode_mneg_mixed_width.html
- pbt-out/bug_reports/encode_mneg_sp_as_zr.md
- pbt-out/bug_reports/encode_mneg_sp_as_zr.html
- pbt-out/bug_reports/encode_mneg_fp_as_gpr.md
- pbt-out/bug_reports/encode_mneg_fp_as_gpr.html
- pbt-out/run/ (scratch)
- proptest-regressions/backend/arm/assembler/encoder/encode_mneg_pbt.txt (proptest failure cache)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 07:59 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 171/307 total | PBT candidates: 171 | Tested: 171 (100%) | 1 pass, 171 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 171 |
| **Tested (of PBT candidates)** | **171 / 171 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 171 / -1 |
| **Overall (tested / all functions)** | **171 / 307 (56%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 171 | 171 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 171 | 171 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 27 | 27 | 100% | covered |
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
