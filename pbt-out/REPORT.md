# PBT Campaign Report: encode_fclass

## Summary

**Verdict:** 2 medium: encode_fclass silently ignores a 3rd operand (including RoundingMode), so `fclass.s x0, f0, 0` and `fclass.s x0, f0, rne` assemble as `fclass.s x0, f0` instead of being rejected like llvm-mc.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fclass
**Tests:** 7
**Result:** 5 passing, 2 bugs
**Change surface:** 1 changed function (encode_fclass), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw and marked encode_fclass NOT LINKED against unrelated C++ binaries; Rust `cargo test --lib encode_fclass` executed the symbol (KAT + 7 properties).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fclass | 7 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_fclass ignores a 3rd operand

**Formal:** ∀ mn ∈ {fclass.s, fclass.d}, rd ∈ GPRNames, rs1 ∈ FPRNames, extra ∈ Operand. encode_fclass([Reg(rd), Reg(rs1), extra], funct7(mn)) is Err
**Contract evidence:** inferred (public wrapper encoder/mod.rs:742 and :770 passes the operand slice through unchanged; llvm-mc rejects extra operands; README.md:352 documents a 6-field R-type with no extra operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fclass([Reg("x0"), Reg("f0"), Imm(0)], 0b1110000)
**Expected / Actual:** Err / Ok(Word(0xe0001053)) (encoding of fclass.s x0, f0)
**Impact:** A typo or extra token in FCLASS.S/D is silently dropped, so the assembler emits a valid OP-FP FCLASS word instead of diagnosing the extra operand.
**Root cause:** float.rs:111-113 reads only operands 0 and 1 via get_reg/get_freg and returns Ok without checking operands.len() > 2, so a 3rd operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:113`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, 0, funct7)))
```
**Suggested fix:** Reject more than two operands before packing the R-type word.
```rust
    if operands.len() > 2 {
        return Err("fclass: unexpected extra operand".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, 0, funct7)))
```
**Bug report:** bug_reports/encode_fclass_extra_operand.md
**Repro seed:** (none — deterministic regression; proptest shrunk to x0, f0, Imm(0))
**Raw output:**
```text
Test failed: 3rd operand must Err for fclass.s x0, f0 (llvm-mc rejects extra operands); got Ok(Word(3758100563)) at src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs:439.
minimal failing input: (mn, f7) = (
    "fclass.s",
    112,
), rd = "x0", rs1 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_fclass ignores a 3rd RoundingMode

**Formal:** ∀ mn ∈ {fclass.s, fclass.d}, rd ∈ GPRNames, rs1 ∈ FPRNames, rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fclass([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn)) is Err
**Contract evidence:** inferred (RISC-V Unprivileged ISA FCLASS hardwires funct3=001 and rs2=00000 with no rm field; llvm-mc rejects fclass.s a0, fa1, rne; encoder/mod.rs:742/770 pass the operand slice through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fclass([Reg("x0"), Reg("f0"), RoundingMode("rne")], 0b1110000)
**Expected / Actual:** Err / Ok(Word(0xe0001053)) (encoding of fclass.s x0, f0; funct3 remains 001, not rne)
**Impact:** A caller who copied an FSQRT-style 3-operand form (`fclass.s rd, rs1, rne`) gets a silently accepted FCLASS with the rounding token dropped, hiding a real assembly error.
**Root cause:** float.rs:111-113 reads only operands 0 and 1 and returns Ok without checking operands.len() > 2, so a 3rd RoundingMode is ignored and does not overwrite the hardwired funct3=001.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:113`
```rust
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, 0, funct7)))
```
**Suggested fix:** Reject more than two operands before packing the R-type word.
```rust
    if operands.len() > 2 {
        return Err("fclass: unexpected extra operand".to_string());
    }
    let rd = get_reg(operands, 0)?;
    let rs1 = get_freg(operands, 1)?;
    Ok(EncodeResult::Word(encode_r(OP_OP_FP, rd, 0b001, rs1, 0, funct7)))
```
**Bug report:** bug_reports/encode_fclass_rm_third.md
**Repro seed:** (none — deterministic regression; proptest shrunk to x0, f0, rne)
**Raw output:**
```text
Test failed: 3rd RoundingMode must Err for fclass.s x0, f0, rne (FCLASS has no rm); got Ok(Word(3758100563)) at src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs:457.
minimal failing input: (mn, f7) = (
    "fclass.s",
    112,
), rd = "x0", rs1 = "f0", rm = "rne"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs | 7 properties + 2 KAT + 2 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fclass -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fclass_regression_extra_operand -- --test-threads=1
```

B2 RoundingMode third:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fclass_regression_rm_third -- --test-threads=1
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
- pbt-out/CHANGE_SURFACE.md
- pbt-out/bug_reports/encode_fclass_extra_operand.md
- pbt-out/bug_reports/encode_fclass_extra_operand.html
- pbt-out/bug_reports/encode_fclass_rm_third.md
- pbt-out/bug_reports/encode_fclass_rm_third.html
- src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- proptest-regressions/backend/riscv/assembler/encoder/encode_fclass_pbt.txt (framework failure file)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 19:26 (campaign: coverage)
> Files: 14/14 scanned (100%) | Functions: 207/351 total | PBT candidates: 207 | Tested: 207 (100%) | 1 pass, 207 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 14 / 14 (100%) |
| Total functions (all files) | 351 |
| PBT candidates (from FUNCTION_INDEX) | 207 |
| **Tested (of PBT candidates)** | **207 / 207 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 207 / -1 |
| **Overall (tested / all functions)** | **207 / 351 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 207 | 207 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 207 | 207 | 0 | 100% |

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
