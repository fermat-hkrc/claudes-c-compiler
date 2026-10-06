# PBT Campaign Report: encode_stop

## Summary

**Verdict:** 4 medium: encode_stop silently accepts extra operands, SP as Rs / XZR-or-W as base, FP Rs / STADDB-with-X, and nonzero Mem offsets that llvm-mc/gas reject, so callers get a wrong encoding instead of an assembler error.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_stop
**Tests:** 11
**Result:** 7 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter, unrelated binaries, claimed NOT LINKED). Cargo tests executed encode_stop (9 KATs passed). Sweep round 1/1 spent on a manual arm audit plus invalid-name / alt-spellings / unknown-op properties.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_stop | 11 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_stop ignores a third operand

**Formal:** ∀ valid (op,suf,Rs,Rn,wide), extra ∈ Operand. encode_stop(op+suf, [Reg(Rs), Mem{base,0}, extra]) is Err
**Contract evidence:** inferred (README gas-compat plus llvm-mc rejecting a 3rd STADD operand; body checks only `len() < 2`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_stop("stadd", [Reg("w0"), Mem{base:"x0", offset:0}, Reg("x2")])
**Expected / Actual:** Err / Ok(Word) — extra operand ignored
**Impact:** `stadd w0, [x0], x2` is assembled as `stadd w0, [x0]`. Callers that pass a trailing operand get a silent wrong encoding instead of an assembler error
**Root cause:** load_store.rs:928 checks `operands.len() < 2` and never rejects `len() > 2`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:928`
```rust
    if operands.len() < 2 {
        return Err(format!("{} requires 2 operands", mnemonic));
    }
```
**Suggested fix:** Require exactly two operands
```rust
    if operands.len() != 2 {
        return Err(format!("{} requires 2 operands", mnemonic));
    }
```
**Bug report:** bug_reports/encode_stop_extra_operand.md
**Repro seed:** cc b3dea4defd2d280c2de41066705f582787b224ec3f0aace6a7306f56cf06297f
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_extra_operand' (2610720) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:657:5:
stadd w0, [x0], x2 must Err; llvm-mc/gas reject a 3rd operand
```

### B2: encode_stop accepts SP as Rs and XZR/W as base

**Formal:** ∀ valid (op,suf), kind ∈ {SP-as-Rs, WSP-as-Rs, W-base, WSP-base, XZR-base, x31-base, WZR-base}. encode_stop(op+suf, ops(kind)) is Err
**Contract evidence:** inferred (ARM Rs is ZR not SP; Rn=31 is SP not XZR; llvm-mc rejects `stadd sp, [x1]` and `stadd w0, [xzr]`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_stop("stadd", [Reg("sp"), Mem{base:"x1", offset:0}])
**Expected / Actual:** Err / Ok(Word) — SP encoded as XZR (register 31)
**Impact:** `stadd sp, [x1]` encodes as `stadd xzr, [x1]`; `stadd w0, [xzr]` encodes as `stadd w0, [sp]`
**Root cause:** load_store.rs:931-933 uses get_reg/parse_reg_num, which map both SP and XZR to 31 and accept W names as a base
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:931`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let rn = match operands.get(1) {
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or_else(|| format!("{}: invalid base", mnemonic))?,
        _ => return Err(format!("{} requires memory operand [Xn]", mnemonic)),
    };
```
**Suggested fix:** Reject SP/WSP as Rs; require the base to be Xn or SP
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    if matches_sp_name(operands, 0) {
        return Err(format!("{}: SP is not a valid Rs", mnemonic));
    }
```
**Bug report:** bug_reports/encode_stop_sp_as_rs.md
**Repro seed:** (deterministic regression; property shrunk to op=0, suf=0, n=0, kind=0, is_64=false)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_sp_as_rs' (2610723) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:672:5:
stadd sp, [x1] must Err; llvm-mc/gas reject SP as Rs
```

### B3: encode_stop accepts FP/SIMD Rs and X registers on STADDB/STADDH

**Formal:** ∀ n ∈ {0..30}, kind ∈ {FP-Rs, STADDB-X, STADDH-X, STADDLB-X, STADDLH-X}. encode_stop(mnem(kind), ops(kind,n)) is Err
**Contract evidence:** inferred (llvm-mc rejects `stadd s0, [x1]` and `staddb x0, [x1]`; ARM STADDB/STADDH require W registers)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_stop("stadd", [Reg("b0"), Mem{base:"x1", offset:0}])
**Expected / Actual:** Err / Ok(Word) — b0 parsed as register 0, 32-bit
**Impact:** A SIMD or 64-bit source is silently recoded as a 32-bit GPR
**Root cause:** load_store.rs:931 get_reg/parse_reg_num accept b/h/s/d/q/v prefixes; size comes from a 'b'/'h' suffix so X registers still encode for STADDB/STADDH
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:931`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** Require a GPR Rs; for byte/half variants require a W register
```rust
    let (rs, is_64) = get_gpr_rs(operands, 0)?;
    if is_byte_or_half && is_64 {
        return Err(format!("{} requires a W register", mnemonic));
    }
```
**Bug report:** bug_reports/encode_stop_fp_reg.md
**Repro seed:** (deterministic regression; property shrunk to n=0, kind=0, fp='b')
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_fp_reg' (2610721) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:687:5:
stadd b0, [x1] must Err; llvm-mc/gas require integer registers
```

### B4: encode_stop ignores a nonzero Mem offset

**Formal:** ∀ valid (op,suf,Rs,Rn,wide), off ∈ Z excluding 0. encode_stop(op+suf, [Reg(Rs), Mem{base, off}]) is Err
**Contract evidence:** inferred (llvm-mc rejects `stadd w0, [x1, #1]`; ARM optional offset can only be #0)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_stop("stadd", [Reg("w0"), Mem{base:"x0", offset:-1}])
**Expected / Actual:** Err / Ok(Word) — offset ignored
**Impact:** An offset the programmer wrote is silently dropped
**Root cause:** load_store.rs:933 matches `Operand::Mem { base, .. }` and never reads `offset`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:933`
```rust
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or_else(|| format!("{}: invalid base", mnemonic))?,
```
**Suggested fix:** Reject a nonzero offset
```rust
        Some(Operand::Mem { base, offset }) if *offset == 0 => parse_reg_num(base).ok_or_else(|| format!("{}: invalid base", mnemonic))?,
        Some(Operand::Mem { offset, .. }) => return Err(format!("{}: offset must be #0, got {}", mnemonic, offset)),
```
**Bug report:** bug_reports/encode_stop_nonzero_offset.md
**Repro seed:** (deterministic regression; property shrunk to op=0, suf=0, rs=0, rn=0, is_64=false, off=-1)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_stop_pbt::test_encode_stop_regression_nonzero_offset' (2610722) panicked at src/backend/arm/assembler/encoder/encode_stop_pbt.rs:702:5:
stadd w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_stop_pbt.rs | 11 properties + 9 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_stop_pbt` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_stop -- --test-threads=1
```

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_extra_operand -- --test-threads=1
```

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_sp_as_rs -- --test-threads=1
```

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_fp_reg -- --test-threads=1
```

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_stop_regression_nonzero_offset -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/bug_reports/encode_stop_extra_operand.md
- pbt-out/bug_reports/encode_stop_extra_operand.html
- pbt-out/bug_reports/encode_stop_sp_as_rs.md
- pbt-out/bug_reports/encode_stop_sp_as_rs.html
- pbt-out/bug_reports/encode_stop_fp_reg.md
- pbt-out/bug_reports/encode_stop_fp_reg.html
- pbt-out/bug_reports/encode_stop_nonzero_offset.md
- pbt-out/bug_reports/encode_stop_nonzero_offset.html
- pbt-out/run/ (scratch)
- proptest-regressions/backend/arm/assembler/encoder/encode_stop_pbt.txt (proptest failure file from the extra-operand property)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 05:26 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 165/307 total | PBT candidates: 165 | Tested: 165 (100%) | 1 pass, 165 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 165 |
| **Tested (of PBT candidates)** | **165 / 165 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 165 / -1 |
| **Overall (tested / all functions)** | **165 / 307 (54%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 165 | 165 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 165 | 165 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
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
