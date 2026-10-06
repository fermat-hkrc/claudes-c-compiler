# PBT Campaign Report: encode_lr

## Summary

**Verdict:** 1 high: encode_lr discards a nonzero memory offset, so `lr.w a0, 8(a1)` encodes as `(a1)` and the load-reserved hits the wrong address; 1 medium: extra operands after a valid LR are silently ignored.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_lr
**Tests:** 7
**Result:** 5 passing, 2 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and listed unrelated C++ binaries as NOT LINKED for encode_lr; the Rust `cargo test --lib encode_lr` run executed the production symbol (5 passing + 2 failing properties, 3 KAT). Manual audit: get_reg, get_mem, funct7=0b0001000, and encode_r were all reached.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_lr | 7 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_lr ignores extra operands

**Formal:** ∀ mn ∈ {lr.w, lr.d}, rd, rs1 ∈ GPR, extra ∈ Operand. encode_lr([Reg(rd), Mem{rs1, 0}, extra], funct3(mn)) is Err
**Contract evidence:** inferred (llvm-mc rejects a third operand as `invalid operand for instruction`; encode_instruction at encoder/mod.rs:655-656 passes the operand slice through unchanged)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_lr([Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0b010)
**Expected / Actual:** Err / Ok(Word(0x1000202f))
**Impact:** Typos and extra tokens after a valid LR are silently dropped, so the assembler accepts instructions other RISC-V assemblers reject
**Root cause:** atomics.rs:10 returns Ok after reading only operands[0..1] via get_reg/get_mem and never checks operands.len()
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:10`
```rust
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, 0, funct7)))
```
**Suggested fix:** Reject any operand list whose length is not exactly 2
```rust
    if operands.len() != 2 {
        return Err(format!("lr: expected 2 operands, got {}", operands.len()));
    }
    Ok(EncodeResult::Word(encode_r(OP_AMO, rd, funct3, rs1, 0, funct7)))
```
**Bug report:** bug_reports/encode_lr_extra_operand.md
**Repro seed:** cc 8b8ff3f0a81c5c67270b4a1d8795c928705a48efc8a356450a77f81fce5c19ea
**Raw output:**
```text
Test failed: extra operand must Err for lr.w x0, (x0) (llvm-mc rejects extra operands); got Ok(Word(268443695)) at src/backend/riscv/assembler/encoder/encode_lr_pbt.rs:351.
minimal failing input: (mn, f3) = (
    "lr.w",
    2,
), rd = "x0", rs1 = "x0", extra = Imm(
    0,
)
```

### B2: encode_lr silently drops a nonzero memory offset

**Formal:** ∀ mn ∈ {lr.w, lr.d}, rd, rs1 ∈ GPR, off ∈ ℤ\{0}. encode_lr([Reg(rd), Mem{rs1, off}], funct3(mn)) is Err
**Contract evidence:** inferred (llvm-mc `optional integer offset must be 0`; RISC-V unprivileged ISA LR has no immediate — address is rs1 only; README.md:352 encoding follows the ISA R-type layout)
**Documentation conflict:** (none) — `_offset` at atomics.rs:7 is the producing statement that discards the offset, not a domain restriction or documented limitation
**Severity:** high
**Counterexample:** encode_lr([Reg("x0"), Mem { base: "x0", offset: 1 }], funct3=0b010)
**Expected / Actual:** Err / Ok(Word(0x1000202f)) identical to offset 0
**Impact:** Source that names a displaced address such as `8(a1)` is assembled as `(a1)`, so the load-reserved operates on a different location than the assembly text says
**Root cause:** atomics.rs:7 `let (rs1, _offset) = get_mem(operands, 1)?` binds the offset and discards it, then encodes rs1 only
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/atomics.rs:7`
```rust
    let (rs1, _offset) = get_mem(operands, 1)?;
```
**Suggested fix:** Reject any nonzero offset before packing the R-type word
```rust
    let (rs1, offset) = get_mem(operands, 1)?;
    if offset != 0 {
        return Err(format!("lr: memory offset must be 0, got {}", offset));
    }
```
**Bug report:** bug_reports/encode_lr_nonzero_offset.md
**Repro seed:** (none — shrunk to offset=1; deterministic regression test_encode_lr_regression_nonzero_offset)
**Raw output:**
```text
Test failed: nonzero offset must Err for lr.w x0, 1(x0) (llvm-mc: optional integer offset must be 0); got Ok(Word(268443695)) at src/backend/riscv/assembler/encoder/encode_lr_pbt.rs:368.
minimal failing input: (mn, f3) = (
    "lr.w",
    2,
), rd = "x0", rs1 = "x0", off = 1
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_lr_pbt.rs | 7 properties + 3 KAT + 2 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lr -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lr_neg_extra -- --test-threads=1
```

B2 nonzero offset:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lr_neg_nonzero_offset -- --test-threads=1
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
- pbt-out/bug_reports/encode_lr_extra_operand.md
- pbt-out/bug_reports/encode_lr_extra_operand.html
- pbt-out/bug_reports/encode_lr_nonzero_offset.md
- pbt-out/bug_reports/encode_lr_nonzero_offset.html
- pbt-out/run/encode_lr_test.log
- proptest-regressions/backend/riscv/assembler/encoder/encode_lr_pbt.txt

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 16:39 (campaign: coverage)
> Files: 13/13 scanned (100%) | Functions: 196/337 total | PBT candidates: 196 | Tested: 196 (100%) | 1 pass, 196 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 13 |
| Files scanned | 13 / 13 (100%) |
| Total functions (all files) | 337 |
| PBT candidates (from FUNCTION_INDEX) | 196 |
| **Tested (of PBT candidates)** | **196 / 196 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 196 / -1 |
| **Overall (tested / all functions)** | **196 / 337 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 196 | 196 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 196 | 196 | 0 | 100% |

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
