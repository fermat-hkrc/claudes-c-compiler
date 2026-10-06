# PBT Campaign Report: encode_float_store

## Summary

**Verdict:** 4 bugs (3 high, 1 medium): encode_float_store truncates out-of-range immediates (2048 encodes as -2048), silently drops extra operands, remaps %hi/%pcrel_hi to lo12 relocs instead of rejecting them, and emits I-type lo relocs (Lo12I/PcrelLo12I) on S-type FSW/FSD so the linker patches the wrong bits.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_float_store
**Tests:** 8
**Result:** 4 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED for encode_float_store). The cargo test run executed the Rust symbol; the C++ coverage reporter cannot see it.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_float_store | 8 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_float_store silently truncates out-of-range S-type immediates

**Formal:** ∀ mn ∈ {fsw, fsd}, rs2 ∈ FPRegs, rs1 ∈ GPRs, imm ∉ [-2048, 2047]. llvm-mc rejects mn rs2, imm(rs1) ⇒ encode_float_store([Reg(rs2), Mem{rs1, imm}], funct3(mn)) is Err
**Contract evidence:** inferred (RISC-V S-type imm[11:5]|imm[4:0] at encoder/mod.rs:325; llvm-mc rejects integers outside [-2048, 2047]; encode_instruction passes operands through)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_float_store([Reg("f0"), Mem { base: "x0", offset: 2048 }], 0b010)
**Expected / Actual:** Err / Ok(Word(2147491879)) = 0x80002027 (fsw f0, -2048(x0))
**Impact:** A store whose offset is 2048 is assembled as offset -2048 (off by 4096), so the runtime writes the wrong address.
**Root cause:** float.rs:38 passes `*offset as i32` into encode_s, which takes the low 12 bits with no range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:38`
```rust
            Ok(EncodeResult::Word(encode_s(OP_STORE_FP, funct3, rs1, rs2, *offset as i32)))
```
**Suggested fix:** Reject offsets outside signed imm12 before packing.
```rust
            if !(-2048..=2047).contains(offset) {
                return Err(format!("float store immediate out of range: {}", offset));
            }
            Ok(EncodeResult::Word(encode_s(OP_STORE_FP, funct3, rs1, rs2, *offset as i32)))
```
**Bug report:** bug_reports/encode_float_store_imm_oob.md
**Repro seed:** (none — shrunk input is deterministic; regression test_encode_float_store_regression_imm_oob)
**Raw output:**
```
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147491879)) at src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs:512.
minimal failing input: (mn, f3) = ("fsw", 2), rs2 = "f0", rs1 = "x0", imm = 2048
```

### B2: encode_float_store ignores extra operands

**Formal:** ∀ mn ∈ {fsw, fsd}, rs2 ∈ FPRegs, rs1 ∈ GPRs, imm ∈ [-2048, 2047], extra ∈ Operand. encode_float_store([Reg(rs2), Mem{rs1, imm}, extra], funct3(mn)) is Err
**Contract evidence:** inferred (llvm-mc "invalid operand for instruction" on a third operand; encode_instruction at mod.rs:718/746 passes the full operand slice through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_float_store([Reg("f0"), Mem { base: "x0", offset: 0 }, Imm(0)], 0b010)
**Expected / Actual:** Err / Ok(Word(8231)) = 0x00002027 (fsw f0, 0(x0))
**Impact:** A third operand is dropped; malformed FSW/FSD still assembles.
**Root cause:** float.rs:35 matches only operands.get(1) and never checks operands.len().
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:35`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Require exactly two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("float store: unexpected extra operand".to_string());
    }
    match &operands.get(1) {
```
**Bug report:** bug_reports/encode_float_store_extra_operand.md
**Repro seed:** cc 37bb301224666ee6a1abe2c26cb00fb38e2a778c654818ddc9738bd686a6fc81
**Raw output:**
```
Test failed: extra operand must Err for fsw f0, 0(x0) (llvm-mc rejects extra operands); got Ok(Word(8231)) at src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs:531.
minimal failing input: (mn, f3) = ("fsw", 2), rs2 = "f0", rs1 = "x0", off = 0, extra = Imm(0)
```

### B3: encode_float_store accepts %hi/%pcrel_hi/%tprel_hi on STORE-FP memory operands

**Formal:** ∀ mn ∈ {fsw, fsd}, rs2 ∈ FPRegs, rs1 ∈ GPRs, s ∈ Idents, hi ∈ {%hi, %pcrel_hi, %tprel_hi}. llvm-mc rejects mn rs2, hi(s)(rs1) ⇒ encode_float_store([Reg(rs2), MemSymbol{rs1, hi(s)}], funct3(mn)) is Err
**Contract evidence:** inferred (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on STORE-FP memory operands)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_float_store([Reg("f0"), MemSymbol { base: "x0", symbol: "%hi(foo)", modifier: "" }], 0b010)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 8231, reloc_type: Lo12S, symbol: "foo", addend: 0 })
**Impact:** A hi-type modifier is remapped to Lo12S and a store with imm=0 is emitted; the linker patches the low 12 S-type bits from a high-part symbol.
**Root cause:** float.rs:43-47 remaps PcrelHi20/Hi20 onto S-type lo relocs instead of rejecting hi modifiers.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:43`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
                other => other,
            };
```
**Suggested fix:** Return Err for hi-type modifiers; keep only lo-type S relocs.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I => RelocType::PcrelLo12S,
                RelocType::Lo12I => RelocType::Lo12S,
                RelocType::TprelLo12I => RelocType::TprelLo12S,
                RelocType::PcrelLo12S | RelocType::Lo12S | RelocType::TprelLo12S => reloc_type,
                _ => return Err("float store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```
**Bug report:** bug_reports/encode_float_store_hi_modifier.md
**Repro seed:** (none — shrunk input is deterministic; regression test_encode_float_store_regression_hi_modifier)
**Raw output:**
```
Test failed: hi-type modifier %hi(foo) must Err on float store (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 8231, reloc: Relocation { reloc_type: Lo12S, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs:602.
minimal failing input: (mn, f3) = ("fsw", 2), rs2 = "f0", rs1 = "x0", s = "foo", hi = "%hi"
```

### B4: encode_float_store emits I-type lo relocs on S-type STORE-FP

**Formal:** ∀ rs2 ∈ FPRegs, rs1 ∈ GPRs, f3 ∈ {0b010, 0b011}, s ∈ Idents. encode_float_store([Reg(rs2), MemSymbol{rs1, %pcrel_lo(s)}], f3) = WordWithReloc{word = encode_float_store([Reg(rs2), Mem{rs1, 0}], f3), reloc_type = PcrelLo12S, symbol = s, addend = 0} ∧ likewise %lo → Lo12S ∧ %tprel_lo → TprelLo12S
**Contract evidence:** documented src/backend/riscv/assembler/encoder/mod.rs:99 "R_RISCV_PCREL_LO12_S - for SW/SD (low 12 bits of PC-relative, S-type)"; llvm-mc fixup_riscv_lo12_s / pcrel_lo12_s / tprel_lo12_s on fsw/fsd
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_float_store([Reg("f0"), MemSymbol { base: "x0", symbol: "%pcrel_lo(foo)", modifier: "" }], 0b010)
**Expected / Actual:** reloc_type PcrelLo12S / PcrelLo12I (and Lo12I for %lo)
**Impact:** The ELF writer applies I-type imm[31:20] patching to an S-type instruction, overwriting rs2 and the scattered S-type immediate. Relocatable fsw/fsd with %lo/%pcrel_lo/%tprel_lo link to a corrupted store.
**Root cause:** parse_reloc_modifier returns I-type lo variants; float.rs:43-47 only remaps Hi20 kinds and leaves Lo12I/PcrelLo12I/TprelLo12I unchanged.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:43`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
                other => other,
            };
```
**Suggested fix:** Remap the I-type lo variants that parse_reloc_modifier actually returns.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Lo12I | RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelLo12I | RelocType::TprelHi20 => RelocType::TprelLo12S,
                RelocType::PcrelLo12S | RelocType::Lo12S | RelocType::TprelLo12S => reloc_type,
                _ => return Err("float store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```
**Bug report:** bug_reports/encode_float_store_lo_reloc_i_type.md
**Repro seed:** (none — shrunk input is deterministic; regression test_encode_float_store_regression_lo_is_s_type)
**Raw output:**
```
Test failed: assertion failed: `(left == right)`
  left: `"PcrelLo12I"`,
 right: `"PcrelLo12S"` at src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs:467.
minimal failing input: (_mn, f3) = ("fsw", 2), rs2 = "f0", rs1 = "x0", s = "foo"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs | 8 properties + 4 KAT + 4 regression witnesses |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store -- --test-threads=1
```

B1 (imm oob):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store_neg_imm_oob -- --test-threads=1
cargo test --lib test_encode_float_store_regression_imm_oob -- --test-threads=1
```

B2 (extra operand):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store_neg_extra -- --test-threads=1
cargo test --lib test_encode_float_store_regression_extra_operand -- --test-threads=1
```

B3 (hi modifier):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store_neg_hi_modifier -- --test-threads=1
cargo test --lib test_encode_float_store_regression_hi_modifier -- --test-threads=1
```

B4 (I-type lo reloc):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_float_store_reloc_lo -- --test-threads=1
cargo test --lib test_encode_float_store_regression_lo_is_s_type -- --test-threads=1
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
- pbt-out/bug_reports/encode_float_store_imm_oob.md
- pbt-out/bug_reports/encode_float_store_imm_oob.html
- pbt-out/bug_reports/encode_float_store_extra_operand.md
- pbt-out/bug_reports/encode_float_store_extra_operand.html
- pbt-out/bug_reports/encode_float_store_hi_modifier.md
- pbt-out/bug_reports/encode_float_store_hi_modifier.html
- pbt-out/bug_reports/encode_float_store_lo_reloc_i_type.md
- pbt-out/bug_reports/encode_float_store_lo_reloc_i_type.html
- pbt-out/run/encode_float_store_test.log
- pbt-out/FUNCTION_INDEX.md
- pbt-out/CHANGE_SURFACE.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 17:53 (campaign: coverage)
> Files: 14/14 scanned (100%) | Functions: 201/351 total | PBT candidates: 201 | Tested: 201 (100%) | 1 pass, 201 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 14 / 14 (100%) |
| Total functions (all files) | 351 |
| PBT candidates (from FUNCTION_INDEX) | 201 |
| **Tested (of PBT candidates)** | **201 / 201 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 201 / -1 |
| **Overall (tested / all functions)** | **201 / 351 (57%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 201 | 201 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 201 | 201 | 0 | 100% |

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
