# PBT Campaign Report: encode_alu_imm

## Summary

**Verdict:** 4 high: encode_alu_imm silently wraps out-of-range immediates, ignores extra operands, remaps %hi/%pcrel_hi/%tprel_hi to lo-12 I-type relocs, and accepts GOT/TLS/plain symbols that llvm-mc rejects, so any caller assembling addi/slti/sltiu/xori/ori/andi on those inputs emits the wrong word or the wrong relocation.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_alu_imm
**Tests:** 9
**Result:** 5 passing, 4 bugs
**Change surface:** (no change source given)
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries). Sweep was a manual arm audit of encode_alu_imm plus encode_alu_imm_neg_other_modifier. Tier: standard.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_alu_imm | 9 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_alu_imm ignores extra operands

**Formal:** ∀ mn, rd, rs1, imm ∈ [-2048,2047], extra. encode_alu_imm([Reg(rd),Reg(rs1),Imm(imm), extra], f3) = Err
**Contract evidence:** inferred (llvm-mc `-triple=riscv64` rejects extra operands on addi/slti/sltiu/xori/ori/andi; README.md:300 three-operand I-type OP-IMM)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_alu_imm([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], funct3=0) // addi x0, x0, 0, 0
**Expected / Actual:** Err / Ok(Word(0x00000013))
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed OP-IMM word instead of diagnosing the line.
**Root cause:** base.rs:226 matches only operands.get(2) and never checks operands.len(), so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:226`
```rust
    match &operands.get(2) {
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
    if operands.len() != 3 {
        return Err("alu_imm: expected rd, rs1, imm".to_string());
    }
    match &operands.get(2) {
```
**Bug report:** bug_reports/encode_alu_imm_extra_operand.md
**Repro seed:** cc 489679c0787af63a7f0cd6e5e5732fc48d9d118248498f0a3cf0f566128302fc
**Raw output:** Test failed: extra operand must Err for addi x0, x0, 0 (llvm-mc rejects extra operands); got Ok(Word(19)) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:519. minimal failing input: (mn, f3) = ("addi", 0), rd = "x0", rs1 = "x0", imm = 0, extra = Imm(0)

### B2: encode_alu_imm wraps out-of-range OP-IMM immediates

**Formal:** ∀ mn, rd, rs1, imm ∉ [-2048,2047]. llvm-mc rejects mn rd, rs1, imm ∧ encode_alu_imm([Reg(rd),Reg(rs1),Imm(imm)], f3) = Err
**Contract evidence:** inferred (README.md:353 12-bit I-type immediate; llvm-mc: operand must be an integer in [-2048, 2047])
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_alu_imm([Reg("x0"), Reg("x0"), Imm(2048)], funct3=0) // addi x0, x0, 2048
**Expected / Actual:** Err / Ok(Word(0x80000013)) wrapping to addi x0, x0, -2048
**Impact:** Immediates such as 2048 are encoded as the wrapped 12-bit pattern, so an addi/andi/xori that the source wrote with a large constant silently computes the wrong value.
**Root cause:** base.rs:228 casts `*imm as i32` into encode_i, which keeps only imm[11:0] (`& 0xFFF`) and never range-checks the 12-bit signed field.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:228`
```rust
            Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, funct3, rs1, *imm as i32)))
```
**Suggested fix:** Reject immediates outside [-2048, 2047] before packing.
```rust
            if !(-2048..=2047).contains(imm) {
                return Err("alu_imm: immediate out of range [-2048, 2047]".to_string());
            }
            Ok(EncodeResult::Word(encode_i(OP_OP_IMM, rd, funct3, rs1, *imm as i32)))
```
**Bug report:** bug_reports/encode_alu_imm_imm_oob.md
**Repro seed:** (none — deterministic first shrink)
**Raw output:** Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483667)) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:501. minimal failing input: (mn, f3) = ("addi", 0), rd = "x0", rs1 = "x0", imm = 2048

### B3: encode_alu_imm accepts %hi/%pcrel_hi/%tprel_hi on OP-IMM immediates

**Formal:** ∀ mn, rd, rs1, s, hi ∈ {%hi,%pcrel_hi,%tprel_hi}. llvm-mc rejects mn rd, rs1, hi(s) ∧ encode_alu_imm([Reg(rd),Reg(rs1),Symbol("hi(s)")], f3) = Err
**Contract evidence:** inferred (encoder/mod.rs:69/75 lo-12 I-type relocs for ADDI; llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on OP-IMM)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_alu_imm([Reg("x0"), Reg("x0"), Symbol("%hi(foo)")], funct3=0) // addi x0, x0, %hi(foo)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x00000013, reloc_type: Lo12I, symbol: "foo", addend: 0 })
**Impact:** A hi-type modifier is remapped to an I-type lo reloc and an OP-IMM word with imm=0 is emitted. The linker then patches the low 12 I-type bits from a high-part symbol, producing a wrong immediate.
**Root cause:** base.rs:232-237 remaps PcrelHi20/Hi20/TprelHi20 onto the I-type lo reloc kinds instead of rejecting hi modifiers that llvm-mc does not accept on OP-IMM.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:232`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12I,
                RelocType::Hi20 => RelocType::Lo12I,
                RelocType::TprelHi20 => RelocType::TprelLo12I,
                other => other,
            };
```
**Suggested fix:** Accept only lo-12 I-type modifiers; reject hi/GOT/TLS/plain forms.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                _ => return Err("alu_imm: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```
**Bug report:** bug_reports/encode_alu_imm_hi_modifier.md
**Repro seed:** (none — deterministic first shrink)
**Raw output:** Test failed: hi-type modifier %hi(foo) must Err on OP-IMM (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 19, reloc: Relocation { reloc_type: Lo12I, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:589. minimal failing input: (mn, f3) = ("addi", 0), rd = "x0", rs1 = "x0", s = "foo", hi = "%hi"

### B4: encode_alu_imm accepts GOT/TLS/plain symbols as OP-IMM immediates

**Formal:** ∀ mn, rd, rs1, form ∈ {%got_pcrel_hi(s), %tls_ie_pcrel_hi(s), %tls_gd_pcrel_hi(s), %tprel_add(s), s}. llvm-mc rejects mn rd, rs1, form ∧ encode_alu_imm([Reg(rd),Reg(rs1),Symbol(form)], f3) = Err
**Contract evidence:** inferred (encoder/mod.rs:69 PcrelLo12I for ADDI; llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on OP-IMM)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_alu_imm([Reg("x0"), Reg("x0"), Symbol("%got_pcrel_hi(foo)")], funct3=0) // addi x0, x0, %got_pcrel_hi(foo)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x00000013, reloc_type: GotHi20, symbol: "foo", addend: 0 })
**Impact:** %got_pcrel_hi, TLS hi modifiers, %tprel_add, and a bare symbol are accepted and emitted as WordWithReloc with a non-lo reloc kind (or PcrelLo12I for a bare name). The linker then applies the wrong RISC-V relocation to an I-type immediate.
**Root cause:** base.rs:236 `other => other` keeps GotHi20/TlsGotHi20/TlsGdHi20/TprelAdd, and a plain symbol is classified as PcrelHi20 then remapped to PcrelLo12I.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:236`
```rust
                other => other,
```
**Suggested fix:** Accept only lo-12 I-type modifiers; reject every other reloc kind.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                _ => return Err("alu_imm: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```
**Bug report:** bug_reports/encode_alu_imm_other_modifier.md
**Repro seed:** (none — deterministic first shrink)
**Raw output:** Test failed: non-lo modifier %got_pcrel_hi(foo) must Err on OP-IMM; got Ok(WordWithReloc { word: 19, reloc: Relocation { reloc_type: GotHi20, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs:634. minimal failing input: (mn, f3) = ("addi", 0), rd = "x0", rs1 = "x0", form = "%got_pcrel_hi(foo)"

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs | 9 properties (5 passing / 4 failing) plus 6 passing KAT and 4 failing regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_extra -- --test-threads=1
```

B2 out-of-range immediate:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_imm_oob -- --test-threads=1
```

B3 hi modifier:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_hi_modifier -- --test-threads=1
```

B4 other modifier:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_neg_other_modifier -- --test-threads=1
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
- pbt-out/bug_reports/encode_alu_imm_extra_operand.md
- pbt-out/bug_reports/encode_alu_imm_extra_operand.html
- pbt-out/bug_reports/encode_alu_imm_imm_oob.md
- pbt-out/bug_reports/encode_alu_imm_imm_oob.html
- pbt-out/bug_reports/encode_alu_imm_hi_modifier.md
- pbt-out/bug_reports/encode_alu_imm_hi_modifier.html
- pbt-out/bug_reports/encode_alu_imm_other_modifier.md
- pbt-out/bug_reports/encode_alu_imm_other_modifier.html
- src/backend/riscv/assembler/encoder/encode_alu_imm_pbt.rs
- proptest-regressions/backend/riscv/assembler/encoder/encode_alu_imm_pbt.txt

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 14:41 (campaign: coverage)
> Files: 12/12 scanned (100%) | Functions: 189/324 total | PBT candidates: 189 | Tested: 189 (100%) | 1 pass, 189 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 (100%) |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 189 |
| **Tested (of PBT candidates)** | **189 / 189 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 189 / -1 |
| **Overall (tested / all functions)** | **189 / 324 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 189 | 189 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 189 | 189 | 0 | 100% |

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
