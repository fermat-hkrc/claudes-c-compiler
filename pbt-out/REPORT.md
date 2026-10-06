# PBT Campaign Report: encode_c_li

## Summary

**Verdict:** 1 high: encode_c_li silently wraps out-of-range immediates (`c.li x0, 32` encodes as `c.li x0, -32`), plus 1 medium: extra operands are ignored instead of rejected.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_c_li
**Tests:** 7
**Result:** 5 passing, 2 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (build tree not instrumented for C++ coverage) and listed unrelated C++ binaries with encode_c_li NOT LINKED; Rust cargo tests are not those binaries. Execution evidence is `cargo test --lib encode_c_li` (11 passed, 4 failed of the encode_c_li harness).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_c_li | 7 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_c_li ignores extra operands

**Formal:** ∀ rd ∈ GPR, ∀ imm ∈ [-32,31], ∀ extra. encode_c_li([Reg(rd), Imm(imm), extra]) = Err
**Contract evidence:** inferred (two-operand mnemonic compressed.rs:17 `c.li rd, imm`; llvm-mc rejects a third operand; public dispatch encoder/mod.rs:918 passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_c_li([Reg("x0"), Imm(0), Imm(0)])
**Expected / Actual:** Err / Ok(Half(0x4001))
**Impact:** A third (or later) operand is silently dropped, so `c.li x0, 0, 0` encodes as `c.li x0, 0` instead of being rejected. Handwritten assembly that accidentally passes extra tokens gets a valid-looking 16-bit instruction rather than an assembler error.
**Root cause:** compressed.rs:18-23 reads only operands[0] and operands[1] via get_reg/get_imm and never checks operands.len(), so a third token is ignored and the CI-type halfword is still returned.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:18`
```rust
pub(crate) fn encode_c_li(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let imm = get_imm(operands, 1)? as i32;
    let bit5 = ((imm >> 5) & 1) as u16;
    let bits4_0 = (imm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | (bits4_0 << 2) | ((rd as u16) << 7) | (bit5 << 12) | (0b010 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.li: expected 2 operands, got {}", operands.len()));
}
```
**Bug report:** bug_reports/encode_c_li_extra_operand.md
**Repro seed:** cc 5723fdcb91ba698431c9caaab531629e955639ac7b68bc9725af8d2eb8b7ba98
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_li_pbt::encode_c_li_neg_extra' (2842760) panicked at src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs:289:1:
Test failed: extra operand must Err for c.li x0, 0 (llvm-mc rejects extra operands); got Ok(Half(16385)) at src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs:364.
minimal failing input: rd = "x0", imm = 0, extra = Imm(
    0,
)
```

### B2: encode_c_li truncates out-of-range immediates to 6 bits

**Formal:** ∀ rd ∈ GPR, ∀ imm ∉ [-32,31]. llvm-mc rejects "c.li rd, imm" ⇒ encode_c_li([Reg(rd), Imm(imm)]) = Err
**Contract evidence:** inferred (RISC-V Unprivileged ISA C.LI signed 6-bit imm; llvm-mc `immediate must be an integer in the range [-32, 31]`; compress.rs:99 `if !(-32..=31).contains(&imm) { return None; }`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_c_li([Reg("x0"), Imm(32)])
**Expected / Actual:** Err / Ok(Half(0x5001)) — the encoding of `c.li x0, -32`
**Impact:** An immediate that cannot be represented in C.LI's signed 6-bit field is silently wrapped. `c.li x0, 32` encodes as C.LI of imm=-32, so a caller that meant to load 32 gets -32.
**Root cause:** compressed.rs:21-23 take bit 5 and bits 4:0 of imm with no range check, so 32 (0b100000) is packed as the 6-bit pattern of -32.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:21`
```rust
    let bit5 = ((imm >> 5) & 1) as u16;
    let bits4_0 = (imm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | (bits4_0 << 2) | ((rd as u16) << 7) | (bit5 << 12) | (0b010 << 13)))
```
**Suggested fix:** Reject immediates that do not fit in signed 6 bits before packing.
```rust
if !(-32..=31).contains(&imm) {
    return Err("c.li: imm out of range".into());
}
```
**Bug report:** bug_reports/encode_c_li_imm_oob.md
**Repro seed:** (deterministic; shrunk to rd="x0", imm=32)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_li_pbt::encode_c_li_neg_imm_oob' (2842761) panicked at src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs:289:1:
Test failed: oob imm 32 must Err (llvm-mc range [-32, 31]); got Ok(Half(20481)) at src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs:352.
minimal failing input: rd = "x0", imm = 32
```

## Design Caveats

| Observation | Doc evidence |
|-------------|--------------|

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs | 7 properties + 6 KAT + 2 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_li -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_li_neg_extra -- --test-threads=1
```

B2 out-of-range immediate:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_li_neg_imm_oob -- --test-threads=1
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
- pbt-out/bug_reports/encode_c_li_extra_operand.md
- pbt-out/bug_reports/encode_c_li_extra_operand.html
- pbt-out/bug_reports/encode_c_li_imm_oob.md
- pbt-out/bug_reports/encode_c_li_imm_oob.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 21:34 (campaign: coverage)
> Files: 15/15 scanned (100%) | Functions: 215/367 total | PBT candidates: 215 | Tested: 215 (100%) | 1 pass, 215 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 15 |
| Files scanned | 15 / 15 (100%) |
| Total functions (all files) | 367 |
| PBT candidates (from FUNCTION_INDEX) | 215 |
| **Tested (of PBT candidates)** | **215 / 215 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 215 / -1 |
| **Overall (tested / all functions)** | **215 / 367 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 215 | 215 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 215 | 215 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 19 | 19 | 100% | covered |
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
| encode_ldnp_stnp | load_store.rs |
| encode_adrp | load_store.rs |
| encode_mov | data_processing.rs |
| encode_cond_branch | compare_branch.rs |
| encode_ldr_str_auto | load_store.rs |
| encode_lui | base.rs |
| encode_auipc | base.rs |
| encode_jal | base.rs |
| encode_jalr | base.rs |
| encode_branch_instr | base.rs |
| encode_load | base.rs |
| encode_store | base.rs |
| encode_alu_imm | base.rs |
| encode_alu_reg | base.rs |
| encode_alu_imm_w | base.rs |
| encode_alu_reg_w | base.rs |
| encode_csr | system.rs |
| encode_fence | system.rs |
| encode_amo | atomics.rs |
| encode_lr | atomics.rs |
| encode_sc | atomics.rs |
| encode_sfence_vma | system.rs |
| encode_csri | system.rs |
| encode_float_load | float.rs |
| encode_float_store | float.rs |
| encode_fp_arith | float.rs |
| encode_fp_arith_d | float.rs |
| encode_fp_unary | float.rs |
| encode_fp_sgnj | float.rs |
| encode_fp_cmp | float.rs |
| encode_fclass | float.rs |
| encode_fcvt_int | float.rs |
| encode_fcvt_from_int | float.rs |
| encode_fcvt_fp | float.rs |
| encode_fmv_x_f | float.rs |
| encode_fmv_f_x | float.rs |
| encode_fma | float.rs |
| encode_c_lui | compressed.rs |
| encode_c_li | compressed.rs |
