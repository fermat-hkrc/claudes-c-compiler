# PBT Campaign Report: encode_ldp_stp

## Summary

**Verdict:** 3 medium: encode_ldp_stp ignores extra operands, accepts invalid register forms (SP dest, XZR/W base, mixed width, writeback overlap, LDP Rt1==Rt2), and wraps out-of-range offsets that llvm-mc/gas reject.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldp_stp
**Tests:** 10 properties (plus 5 KAT + 8 failing regression witnesses)
**Result:** 7 passing, 3 failing properties, 3 bugs
**Change surface:** 1 changed function (encode_ldp_stp), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of encode_ldp_stp plus SIMD/alt-spelling differentials.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldp_stp | 10 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: extra operand ignored

**Formal:** ∀ is_load, is_64, rt1, rt2, rn ∈ 0..31, extra ∈ Operand. encode_ldp_stp([Reg, Reg, Mem{offset:0}, extra], is_load) is Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; llvm-mc rejects a fourth operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** is_load=false, is_64=false, rt1=0, rt2=0, rn=0, extra=Reg("x0") → Ok(Word(0x29000000))
**Expected / Actual:** Err / Ok(Word(0x29000000))
**Impact:** A mistyped extra operand is dropped and a pair store is still emitted
**Root cause:** load_store.rs:453 checks only `operands.len() < 3` and then reads operands[0..2]
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:453`
```rust
    if operands.len() < 3 {
        return Err("ldp/stp requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject `operands.len() != 3`
```rust
    if operands.len() != 3 {
        return Err("ldp/stp requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_ldp_stp_extra_operand.md
**Repro seed:** is_load=false, is_64=false, rt1=0, rt2=0, rn=0, extra=Reg("x0")
**Raw output:** Test failed: extra operand must Err (llvm-mc rejects a fourth operand); got Ok(Word(687865856))

### B2: invalid register forms accepted

**Formal:** ∀ is_load, is_64, rt, rn∈0..30. encode(SP dest) is Err ∧ encode(XZR/x31/W base) is Err ∧ encode(mixed W/X) is Err ∧ (is_load ⇒ encode(Rt1==Rt2) is Err) ∧ encode(pre/post with Rn∈{Rt1,Rt2}, Rn≠31) is Err
**Contract evidence:** inferred (llvm-mc "invalid operand" / "unpredictable LDP/STP"; ARM Rt is ZR not SP, Rn is Xn|SP)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** is_load=false, is_64=false, rt=0, rt2=0, rn=0 → accepted SP dest, XZR/x31/W base, mixed X/W, writeback Rn==Rt1
**Expected / Actual:** Err / Ok(Word) for SP dest, XZR/x31/W base, mixed X/W, writeback Rn==Rt1
**Impact:** SP dest encodes as XZR, XZR/x31 base encodes as SP, W base encodes as Xn, mixed X/W uses Rt1 width, writeback overlap and LDP Rt1==Rt2 produce unpredictable encodings
**Root cause:** load_store.rs:457-458 get_reg/parse_reg_num maps SP and XZR both to 31, discards Rt2 width, and pre/post arms never check writeback overlap or LDP Rt1==Rt2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:457`
```rust
    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP as Rt, XZR/W as base, mixed widths, LDP Rt1==Rt2, and writeback overlap
```rust
    if is_load && rt1 == rt2 {
        return Err("ldp Rt1 and Rt2 must differ".to_string());
    }
    // also reject SP dest, XZR/W base, mixed width, writeback overlap
```
**Bug report:** bug_reports/encode_ldp_stp_invalid_regs.md
**Repro seed:** is_load=false, is_64=false, rt=0, rt2=0, rn=0
**Raw output:** accepted invalid register forms (llvm-mc rejects): ["SP as Rt1", "SP as Rt2", "XZR base", "x31 base", "W base", "mixed X/W pair", "writeback Rn==Rt1"]

### B3: out-of-range / unaligned offset wrapped

**Formal:** ∀ is_load, is_64, rt1, rt2, rn ∈ 0..31, off ∈ {min−1, max+1, 1, i64::MIN, i64::MAX} where min/max are the ARM signed-offset bounds. encode_ldp_stp([Reg, Reg, Mem/Pre/Post{off}], is_load) is Err
**Contract evidence:** inferred (llvm-mc "index must be a multiple of {4,8} in range [min, max]"; ARM imm7 signed scaled)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** is_load=false, is_64=false, rt1=0, rt2=0, rn=0, which_off=0, form=0, offset=-257 → Ok(Word(689930240))
**Expected / Actual:** Err / Ok(Word(689930240))
**Impact:** An out-of-range immediate silently becomes a different in-range displacement
**Root cause:** load_store.rs:504 `(*offset >> shift) as i32 & 0x7F` with no range or alignment check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:502`
```rust
        Some(Operand::Mem { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
```
**Suggested fix:** Require alignment and scaled imm7 in [-64, 63] before masking
```rust
            if offset & ((1i64 << shift) - 1) != 0 {
                return Err("ldp/stp offset not aligned".to_string());
            }
            let scaled = offset >> shift;
            if scaled < -64 || scaled > 63 {
                return Err("ldp/stp offset out of range".to_string());
            }
            let imm7 = (scaled as i32) & 0x7F;
```
**Bug report:** bug_reports/encode_ldp_stp_imm7_range.md
**Repro seed:** is_load=false, is_64=false, rt1=0, rt2=0, rn=0, which_off=0, form=0, offset=-257
**Raw output:** out-of-range/unaligned offset -257 form 0 must Err (llvm-mc range); got Ok(Word(689930240))

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs | 10 properties + 5 KAT + 8 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_ldp_stp_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_ldp_stp -- --test-threads=1
```

Per-bug regression:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldp_stp_regression_extra_operand -- --test-threads=1
cargo test --lib test_encode_ldp_stp_regression_sp_dest -- --test-threads=1
cargo test --lib test_encode_ldp_stp_regression_imm7_range -- --test-threads=1
```

## Output Directories

pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/PROPERTIES.md, pbt-out/PLAN.md, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/bug_reports/encode_ldp_stp_extra_operand.md, pbt-out/bug_reports/encode_ldp_stp_extra_operand.html, pbt-out/bug_reports/encode_ldp_stp_invalid_regs.md, pbt-out/bug_reports/encode_ldp_stp_invalid_regs.html, pbt-out/bug_reports/encode_ldp_stp_imm7_range.md, pbt-out/bug_reports/encode_ldp_stp_imm7_range.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 09:49 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 176/307 total | PBT candidates: 176 | Tested: 176 (100%) | 1 pass, 176 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 176 |
| **Tested (of PBT candidates)** | **176 / 176 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 176 / -1 |
| **Overall (tested / all functions)** | **176 / 307 (57%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 176 | 176 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 176 | 176 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 30 | 30 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 16 | 16 | 100% | covered |
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
| encode_uxtb | data_processing.rs |
| encode_ldr_str | load_store.rs |
| encode_ldp_stp | load_store.rs |
