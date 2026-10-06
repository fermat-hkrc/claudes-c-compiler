# PBT Campaign Report: encode_swp

## Summary

**Verdict:** 4 medium: encode_swp silently encodes assembler input that llvm-mc/gas reject — extra operands, SP as Rs/Rt, mixed W/X data registers, and nonzero memory offsets — so the built-in assembler emits the wrong word instead of an error.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_swp
**Tests:** 10 properties (plus 8 KAT + 8 failing regression witnesses)
**Result:** 6 passing, 4 failing properties, 4 bugs
**Change surface:** 1 changed function (encode_swp), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test, C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of the 30-line body: every documented branch has a property. Sweep round 1/1 spent (invalid-name / alt-spellings passing).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_swp | 10 properties (8 KAT, 8 regression) | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_swp ignores a fourth operand

**Formal:** ∀ v,rs,rt,rn,is_64 in the valid domain, ∀ extra ∈ Operand. encode_swp(v, valid_ops(v,rs,rt,rn,is_64) ++ [extra]) = Err(_)
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects a 4th SWP operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** v=0, rs=0, rt=0, rn=0, is_64=false, extra=Reg("x2")
**Expected / Actual:** Err / Ok(Word) — extra operand ignored
**Impact:** Trailing operands are dropped; gas/llvm-mc reject the same line
**Root cause:** load_store.rs:850 checks `operands.len() < 3` and never rejects `len() > 3`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:850`
```rust
    if operands.len() < 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
```
**Suggested fix:** Require exactly three operands
```rust
    if operands.len() != 3 {
        return Err(format!("{} requires 3 operands", mnemonic));
    }
```
**Bug report:** bug_reports/encode_swp_extra_operand.md
**Repro seed:** v=0, rs=0, rt=0, rn=0, is_64=false, extra=Reg("x2")
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_swp_pbt::test_encode_swp_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_swp_pbt.rs:751:5:
swp w0, w0, [x0], x2 must Err; llvm-mc/gas reject a 4th operand
```

### B2: encode_swp accepts SP/WSP as Rs or Rt

**Formal:** ∀ v in variants, ∀ n ∈ [0,30]. encode_swp(v, ops with SP/WSP as Rs or Rt, or W/WSP/XZR/x31/WZR as base) = Err(_)
**Contract evidence:** inferred (ARM SWP Rs/Rt are ZR not SP; llvm-mc "invalid operand for instruction")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** v=0, n=0, kind=0, is_64=false
**Expected / Actual:** Err / Ok(Word) — SP encoded as register 31 (ZR)
**Impact:** The stack pointer is silently rewritten as the zero register. The same property also accepts XZR/x31/W as base (regression witnesses test_encode_swp_regression_xzr_as_base and test_encode_swp_regression_w_base).
**Root cause:** load_store.rs:853-854 call get_reg; parse_reg_num maps "sp"/"wsp" to 31
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:853`
```rust
    let (rs, is_64) = get_reg(operands, 0)?;
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP/WSP as Rs and Rt
```rust
    if is_sp_name(operands, 0) || is_sp_name(operands, 1) {
        return Err("swp: Rs/Rt cannot be SP".to_string());
    }
```
**Bug report:** bug_reports/encode_swp_sp_as_rs.md
**Repro seed:** v=0, n=0, kind=0, is_64=false
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_swp_pbt::test_encode_swp_regression_sp_as_rs' panicked at src/backend/arm/assembler/encoder/encode_swp_pbt.rs:767:5:
swp sp, w1, [x2] must Err; llvm-mc/gas reject SP as Rs
```

### B3: encode_swp accepts mixed W/X data registers

**Formal:** ∀ n ∈ [0,30]. encode_swp on mixed W/X, FP/SIMD Rs/Rt, or swpb/swph/swpab/swpalh with X registers = Err(_)
**Contract evidence:** inferred (llvm-mc rejects mixed W/X on SWP)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** n=0, kind=0, fp='b'
**Expected / Actual:** Err / Ok(Word) — Rt width discarded; size follows Rs
**Impact:** Mixed-width assembly is silently coerced to the Rs width. The same property also accepts FP/SIMD Rs/Rt and SWPB with X registers (regression witnesses test_encode_swp_regression_fp_reg and test_encode_swp_regression_swpb_x_reg).
**Root cause:** load_store.rs:854 `let (rt, _) = get_reg(operands, 1)?` ignores Rt width
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:854`
```rust
    let (rt, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Require Rs and Rt to share the same GPR width
```rust
    let (rt, rt_64) = get_reg(operands, 1)?;
    if is_64 != rt_64 {
        return Err("swp: Rs and Rt must be the same width".to_string());
    }
```
**Bug report:** bug_reports/encode_swp_mixed_width.md
**Repro seed:** n=0, kind=0, fp='b'
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_swp_pbt::test_encode_swp_regression_mixed_width' panicked at src/backend/arm/assembler/encoder/encode_swp_pbt.rs:815:5:
swp x0, w0, [x1] must Err; llvm-mc/gas reject mixed W/X
```

### B4: encode_swp ignores a nonzero memory offset

**Formal:** ∀ v,rs,rt,rn,is_64 in the valid domain, ∀ off ∈ ℤ\{0}. encode_swp(v, [Reg(Rs), Reg(Rt), Mem{base(rn), off}]) = Err(_)
**Contract evidence:** inferred (ARM optional offset only #0; llvm-mc rejects [x2, #8])
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** v=0, rs=0, rt=0, rn=0, is_64=false, off=-1
**Expected / Actual:** Err / Ok(Word) — offset ignored; encodes as [x0]
**Impact:** The encoded address is not the one written
**Root cause:** load_store.rs:856 matches `Operand::Mem { base, .. }` and discards offset
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:856`
```rust
        Some(Operand::Mem { base, .. }) => parse_reg_num(base).ok_or("swp: invalid base")?,
```
**Suggested fix:** Require offset == 0
```rust
        Some(Operand::Mem { base, offset }) if *offset == 0 => {
            parse_reg_num(base).ok_or("swp: invalid base")?
        }
        Some(Operand::Mem { .. }) => return Err("swp: optional offset can only be 0".to_string()),
```
**Bug report:** bug_reports/encode_swp_nonzero_offset.md
**Repro seed:** v=0, rs=0, rt=0, rn=0, is_64=false, off=-1
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_swp_pbt::test_encode_swp_regression_nonzero_offset' panicked at src/backend/arm/assembler/encoder/encode_swp_pbt.rs:863:5:
swp w0, w0, [x0, #-1] must Err; gas: optional immediate offset can only be 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_swp_pbt.rs | 10 properties, 8 KAT, 8 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_swp -- --test-threads=1
```

Per-bug regression (each fails while the bug is unfixed):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_swp_regression_extra_operand -- --test-threads=1
cargo test --lib test_encode_swp_regression_sp_as_rs -- --test-threads=1
cargo test --lib test_encode_swp_regression_mixed_width -- --test-threads=1
cargo test --lib test_encode_swp_regression_nonzero_offset -- --test-threads=1
```

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/run/encode_swp_test.log
- pbt-out/run/encode_swp_test2.log
- pbt-out/run/encode_swp_test3.log
- pbt-out/bug_reports/encode_swp_extra_operand.md
- pbt-out/bug_reports/encode_swp_extra_operand.html
- pbt-out/bug_reports/encode_swp_sp_as_rs.md
- pbt-out/bug_reports/encode_swp_sp_as_rs.html
- pbt-out/bug_reports/encode_swp_mixed_width.md
- pbt-out/bug_reports/encode_swp_mixed_width.html
- pbt-out/bug_reports/encode_swp_nonzero_offset.md
- pbt-out/bug_reports/encode_swp_nonzero_offset.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 04:32 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 163/307 total | PBT candidates: 163 | Tested: 163 (100%) | 1 pass, 163 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 163 |
| **Tested (of PBT candidates)** | **163 / 163 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 163 / -1 |
| **Overall (tested / all functions)** | **163 / 307 (53%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 163 | 163 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 163 | 163 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 12 | 12 | 100% | covered |
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
