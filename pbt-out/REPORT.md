# PBT Campaign Report: encode_alu_imm_w

## Summary

**Verdict:** 2 high, 1 medium: encode_alu_imm_w silently wraps out-of-range immediates (2048 → -2048), ignores extra operands, and rejects valid I-type %lo/%pcrel_lo/%tprel_lo relocs that llvm-mc accepts.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_alu_imm_w
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** requested encode_op_imm32 unresolved in base.rs; mapped to encode_alu_imm_w (1 function with properties, 3 error-path properties)
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (Rust cargo test is not the C++ gcov reporter); listed unrelated host binaries as NOT LINKED. Manual audit of encode_alu_imm_w plus encode_alu_imm_w_neg_invalid_name.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_alu_imm_w | 9 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_alu_imm_w ignores extra operands

**Formal:** ∀ rd, rs1 ∈ GPRNames, imm ∈ [-2048, 2047], extra ∈ Operand. encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm), extra], 0) = Err(_)
**Contract evidence:** inferred (llvm-mc rejects a fourth operand on addiw; sibling encode_alu_imm has the same arity contract)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_alu_imm_w([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], funct3=0)
**Expected / Actual:** Err / Ok(Word(0x0000001b))
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed OP-IMM-32 word instead of diagnosing the line.
**Root cause:** base.rs:267-271 never checks operands.len(), so trailing operands after rd, rs1, imm are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:267`
```rust
pub(crate) fn encode_alu_imm_w(operands: &[Operand], funct3: u32) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs1 = get_reg(operands, 1)?;
    let imm = get_imm(operands, 2)? as i32;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm)))
}
```
**Suggested fix:** Reject anything other than exactly three operands before packing.
```rust
    if operands.len() != 3 {
        return Err("alu_imm_w: expected rd, rs1, imm".to_string());
    }
    let rd = get_reg(operands, 0)?;
```
**Bug report:** bug_reports/encode_alu_imm_w_extra_operand.md
**Repro seed:** cc 2901cc06d4a99364790cfef20686f824d8818cb63d81f2b79d9c5dbdd9d6e3d4
**Raw output:** Test failed: extra operand must Err for addiw x0, x0, 0 (llvm-mc rejects extra operands); got Ok(Word(27)). minimal failing input: rd = "x0", rs1 = "x0", imm = 0, extra = Imm(0)

### B2: encode_alu_imm_w wraps out-of-range immediates

**Formal:** ∀ rd, rs1 ∈ GPRNames, imm ∉ [-2048, 2047]. llvm-mc rejects "addiw rd, rs1, imm" ⇒ encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(imm)], 0) = Err(_)
**Contract evidence:** documented README.md:353 "I-type:  [    imm[11:0]  | rs1 | funct3 |  rd  | opcode]" plus llvm-mc "integer in the range [-2048, 2047]"
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_alu_imm_w([Reg("x0"), Reg("x0"), Imm(2048)], funct3=0)
**Expected / Actual:** Err / Ok(Word(0x8000001b)) which is addiw x0, x0, -2048
**Impact:** addiw with immediate 2048 is encoded as addiw with -2048; a too-large immediate silently becomes a different instruction.
**Root cause:** base.rs:270 casts get_imm to i32 with no range check; encode_i then masks with 0xFFF, wrapping 2048 to the signed-12 encoding of -2048.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:270`
```rust
    let imm = get_imm(operands, 2)? as i32;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm)))
```
**Suggested fix:** Reject immediates outside the signed 12-bit range before packing.
```rust
    let imm = get_imm(operands, 2)?;
    if !(-2048..=2047).contains(&imm) {
        return Err(format!("alu_imm_w: immediate {imm} out of [-2048, 2047]"));
    }
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm as i32)))
```
**Bug report:** bug_reports/encode_alu_imm_w_imm_oob.md
**Repro seed:** (deterministic; shrunk to imm=2048)
**Raw output:** Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483675)). minimal failing input: rd = "x0", rs1 = "x0", imm = 2048

### B3: encode_alu_imm_w rejects I-type %lo/%pcrel_lo/%tprel_lo

**Formal:** ∀ rd, rs1 ∈ GPRNames, s ∈ ident. encode_alu_imm_w([Reg(rd), Reg(rs1), Symbol("%lo(s)")], 0) = WordWithReloc { word = encode_alu_imm_w([Reg(rd), Reg(rs1), Imm(0)], 0), reloc_type=Lo12I, symbol=s, addend=0 }. Likewise %pcrel_lo → PcrelLo12I and %tprel_lo → TprelLo12I.
**Contract evidence:** documented encoder/mod.rs:81 "R_RISCV_LO12_I - for ADDI/LW/LD (absolute low 12 bits, I-type)"; llvm-mc accepts addiw ra, sp, %lo(foo) with fixup_riscv_lo12_i; sibling encode_alu_imm implements the same I-type reloc form
**Documentation conflict:** (none) — encode_alu_imm_w has no comment declaring Symbol invalid
**Severity:** medium
**Counterexample:** encode_alu_imm_w([Reg("x0"), Reg("x0"), Symbol("%lo(foo)")], funct3=0)
**Expected / Actual:** Ok(WordWithReloc { word: 0x0000001b, Lo12I, "foo", 0 }) / Err("expected immediate at operand 2, got Some(Symbol(\"%lo(foo)\"))")
**Impact:** Valid I-type relocation forms on addiw fail to assemble, so RV64 code that uses addiw with %lo/%pcrel_lo/%tprel_lo cannot be encoded.
**Root cause:** base.rs:270 always calls get_imm on operand 2; unlike encode_alu_imm there is no Symbol branch to emit WordWithReloc.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:270`
```rust
    let imm = get_imm(operands, 2)? as i32;
    Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, imm)))
```
**Suggested fix:** Accept Symbol %lo/%pcrel_lo/%tprel_lo the same way encode_alu_imm does, packing imm=0 with the matching I-type lo12 reloc.
```rust
    match &operands.get(2) {
        Some(Operand::Imm(imm)) => {
            Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, funct3, rs1, *imm as i32)))
        }
        Some(Operand::Symbol(s)) => { /* WordWithReloc Lo12I / PcrelLo12I / TprelLo12I */ }
        _ => Err("alu_imm_w: expected immediate".to_string()),
    }
```
**Bug report:** bug_reports/encode_alu_imm_w_missing_lo_reloc.md
**Repro seed:** (deterministic; shrunk to rd="x0", rs1="x0", s="foo", m="%lo")
**Raw output:** Test failed: SUT rejected %lo(foo): expected immediate at operand 2, got Some(Symbol("%lo(foo)")). minimal failing input: rd = "x0", rs1 = "x0", s = "foo", m = "%lo"

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs | 9 properties + 4 KAT + 3 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_w -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_w_neg_extra -- --test-threads=1
```

B2 out-of-range immediate:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_w_neg_imm_oob -- --test-threads=1
```

B3 missing lo reloc:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_alu_imm_w_reloc_lo -- --test-threads=1
```

## Output Directories

pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/PROPERTIES.md, pbt-out/PLAN.md, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/run/encode_alu_imm_w.log, pbt-out/bug_reports/encode_alu_imm_w_extra_operand.md, pbt-out/bug_reports/encode_alu_imm_w_extra_operand.html, pbt-out/bug_reports/encode_alu_imm_w_imm_oob.md, pbt-out/bug_reports/encode_alu_imm_w_imm_oob.html, pbt-out/bug_reports/encode_alu_imm_w_missing_lo_reloc.md, pbt-out/bug_reports/encode_alu_imm_w_missing_lo_reloc.html

Tier: standard. Closed because the one coverage_gaps-driven sweep round was spent (manual arm audit + encode_alu_imm_w_neg_invalid_name); remaining documented gaps are the three filed bugs.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 15:16 (campaign: coverage)
> Files: 12/12 scanned (100%) | Functions: 191/324 total | PBT candidates: 191 | Tested: 191 (100%) | 1 pass, 191 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 (100%) |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 191 |
| **Tested (of PBT candidates)** | **191 / 191 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 191 / -1 |
| **Overall (tested / all functions)** | **191 / 324 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 191 | 191 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 191 | 191 | 0 | 100% |

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
