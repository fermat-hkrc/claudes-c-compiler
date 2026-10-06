# PBT Campaign Report: encode_float_load

## Summary

**Verdict:** 3 bugs (2 high, 1 medium): encode_float_load truncates out-of-range immediates (2048 encodes as -2048), silently drops extra operands, and remaps %hi/%pcrel_hi to lo12 relocs instead of rejecting them.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_float_load
**Tests:** 8
**Result:** 5 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED for encode_float_load). The cargo test run executed the Rust symbol; the C++ coverage reporter cannot see it.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_float_load | 8 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_float_load silently truncates out-of-range I-type immediates

**Formal:** ∀ mn ∈ {flw, fld}, rd ∈ FPRegs, rs1 ∈ GPRs, imm ∉ [-2048, 2047]. llvm-mc rejects mn rd, imm(rs1) ∧ encode_float_load([Reg(rd), Mem{rs1, imm}], funct3(mn)) = Err(_)
**Contract evidence:** inferred (RISC-V I-type imm[11:0] at encoder/mod.rs:317; llvm-mc rejects integers outside [-2048, 2047]; encode_instruction passes operands through)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_float_load([Reg("f0"), Mem { base: "x0", offset: 2048 }], 0b010)
**Expected / Actual:** Err / Ok(Word(2147491847)) = 0x80002007 (flw f0, -2048(x0))
**Impact:** A load whose offset is 2048 is assembled as offset -2048 (off by 4096), so the runtime reads the wrong address.
**Root cause:** float.rs:10 passes `*offset as i32` into encode_i, which masks to 12 bits with no range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:10`
```rust
            Ok(EncodeResult::Word(encode_i(OP_LOAD_FP, rd, funct3, rs1, *offset as i32)))
```
**Suggested fix:** Reject offsets outside signed imm12 before packing.
```rust
            if !(-2048..=2047).contains(offset) {
                return Err(format!("float load immediate out of range: {}", offset));
            }
            Ok(EncodeResult::Word(encode_i(OP_LOAD_FP, rd, funct3, rs1, *offset as i32)))
```
**Bug report:** bug_reports/encode_float_load_imm_oob.md
**Repro seed:** (none — shrunk input is deterministic; regression test_encode_float_load_regression_imm_oob)
**Raw output:**
```
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147491847)) at src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs:481.
minimal failing input: (mn, f3) = ("flw", 2), rd = "f0", rs1 = "x0", imm = 2048
```

### B2: encode_float_load ignores extra operands

**Formal:** ∀ mn ∈ {flw, fld}, rd ∈ FPRegs, rs1 ∈ GPRs, imm ∈ [-2048, 2047], extra ∈ Operands. encode_float_load([Reg(rd), Mem{rs1, imm}, extra], funct3(mn)) = Err(_)
**Contract evidence:** inferred (llvm-mc "invalid operand for instruction" on a third operand; encode_instruction at mod.rs:715/743 passes the full operand slice through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_float_load([Reg("f0"), Mem { base: "x0", offset: 0 }, Imm(0)], 0b010)
**Expected / Actual:** Err / Ok(Word(8199)) = 0x00002007 (flw f0, 0(x0))
**Impact:** A third operand is dropped; malformed FLW/FLD still assembles.
**Root cause:** float.rs:7 matches only operands.get(1) and never checks operands.len().
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:7`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Require exactly two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("float load: unexpected extra operand".to_string());
    }
    match &operands.get(1) {
```
**Bug report:** bug_reports/encode_float_load_extra_operand.md
**Repro seed:** cc dbd5062a5b0302848715acddbe367c05ad0e1c9b5239180e68fe5f5e8efcd16d
**Raw output:**
```
Test failed: extra operand must Err for flw f0, 0(x0) (llvm-mc rejects extra operands); got Ok(Word(8199)) at src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs:499.
minimal failing input: (mn, f3) = ("flw", 2), rd = "f0", rs1 = "x0", off = 0, extra = Imm(0)
```

### B3: encode_float_load accepts %hi/%pcrel_hi/%tprel_hi on FLW/FLD

**Formal:** ∀ mn ∈ {flw, fld}, rd ∈ FPRegs, rs1 ∈ GPRs, s ∈ Idents, hi ∈ {%hi, %pcrel_hi, %tprel_hi}. llvm-mc rejects mn rd, hi(s)(rs1) ∧ encode_float_load([Reg(rd), MemSymbol{rs1, hi(s)}], funct3(mn)) = Err(_)
**Contract evidence:** inferred (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on LOAD-FP; encoder/mod.rs:95 documents PCREL_LO12_I for loads)
**Documentation conflict:** (none) — float.rs:15-18 remaps hi to lo without a comment declaring hi-type modifiers valid
**Severity:** high
**Counterexample:** encode_float_load([Reg("f0"), MemSymbol { base: "x0", symbol: "%hi(foo)", modifier: "" }], 0b010)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 8199, reloc_type: Lo12I, symbol: "foo", addend: 0 })
**Impact:** A high reloc requested by the author is emitted as R_RISCV_LO12_I, so the linker patches the wrong 12 bits of the symbol.
**Root cause:** float.rs:16 remaps PcrelHi20 to PcrelLo12I and Hi20 to Lo12I instead of rejecting hi-type modifiers.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:16`
```rust
                RelocType::PcrelHi20 => RelocType::PcrelLo12I,
```
**Suggested fix:** Accept only lo12 reloc kinds on LOAD-FP; reject hi-type modifiers.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                other => {
                    return Err(format!("float load: unsupported reloc modifier {:?}", other));
                }
            };
```
**Bug report:** bug_reports/encode_float_load_hi_modifier.md
**Repro seed:** (none — shrunk input is deterministic; regression test_encode_float_load_regression_hi_modifier)
**Raw output:**
```
Test failed: hi-type modifier %hi(foo) must Err on float load (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 8199, reloc: Relocation { reloc_type: Lo12I, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs:568.
minimal failing input: (mn, f3) = ("flw", 2), rd = "f0", rs1 = "x0", s = "foo", hi = "%hi"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs | 8 properties + 4 KAT + 3 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_load -- --test-threads=1
```

B1 (imm oob):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_load_neg_imm_oob -- --test-threads=1
```

B2 (extra operand):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_load_neg_extra -- --test-threads=1
```

B3 (hi modifier):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_load_neg_hi_modifier -- --test-threads=1
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
- pbt-out/bug_reports/encode_float_load_imm_oob.md
- pbt-out/bug_reports/encode_float_load_imm_oob.html
- pbt-out/bug_reports/encode_float_load_extra_operand.md
- pbt-out/bug_reports/encode_float_load_extra_operand.html
- pbt-out/bug_reports/encode_float_load_hi_modifier.md
- pbt-out/bug_reports/encode_float_load_hi_modifier.html
- pbt-out/run/encode_float_load_test.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 17:35 (campaign: coverage)
> Files: 14/14 scanned (100%) | Functions: 200/351 total | PBT candidates: 200 | Tested: 200 (100%) | 1 pass, 200 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 14 / 14 (100%) |
| Total functions (all files) | 351 |
| PBT candidates (from FUNCTION_INDEX) | 200 |
| **Tested (of PBT candidates)** | **200 / 200 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 200 / -1 |
| **Overall (tested / all functions)** | **200 / 351 (57%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 200 | 200 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 200 | 200 | 0 | 100% |

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
