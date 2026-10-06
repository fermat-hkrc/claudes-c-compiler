# PBT Campaign Report: encode_branch_instr

## Summary

**Verdict:** 2 high: encode_branch_instr silently truncates odd/out-of-range offsets (`beq x0, x0, 1` encodes as `beq x0, x0, 0`) and ignores a fourth operand; plus 1 medium: `beq rs1, rs2, foo+N` is rejected instead of emitting R_RISCV_BRANCH with that addend.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_branch_instr
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** requested encode_branch was unresolved in base.rs; tested encode_branch_instr (1 function, 1 with a property, 0 error-handling changes)
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Execution evidence is `cargo test --lib encode_branch_instr` (KAT + 1000-case proptest). Tier: standard.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_branch_instr | 9 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_branch_instr silently truncates odd and out-of-range immediates

**Formal:** ∀ rs1,rs2 ∈ GPR, imm ∈ ℤ. (imm odd ∨ imm ∉ [-4096,4094]) ∧ llvm-mc rejects "beq rs1, rs2, imm" ⇒ encode_branch_instr([Reg(rs1),Reg(rs2),Imm(imm)], 0) = Err
**Contract evidence:** documented src/backend/riscv/assembler/README.md:355 "B-type:  [imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode]" — 13-bit even signed field (bit 0 implicit 0); llvm-mc rejects values that are not a multiple of 2 in [-4096, 4094]
**Documentation conflict:** (none) — the B-type layout states a 13-bit even immediate; the code does not admit a limitation, it silently masks
**Severity:** high
**Counterexample:** encode_branch_instr([Reg("x0"), Reg("x0"), Imm(1)], 0)
**Expected / Actual:** Err / Ok(Word(0x00000063)) encoding of beq x0, x0, 0
**Impact:** An odd or out-of-range branch offset assembles to a different target instead of being rejected, so callers get silent wrong machine code.
**Root cause:** base.rs:131 casts the i64 immediate to i32 with no range or alignment check; encode_b then drops bit 0 via `(imm >> 1) & 0xF`, so odd 1 becomes 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:131`
```rust
            Ok(EncodeResult::Word(encode_b(OP_BRANCH, funct3, rs1, rs2, *imm as i32)))
```
**Suggested fix:** Reject immediates that are odd or outside the 13-bit even signed range before packing.
```rust
            let imm = *imm;
            if imm % 2 != 0 || !(-4096..=4094).contains(&imm) {
                return Err(format!(
                    "branch: immediate {imm} must be a multiple of 2 in [-4096, 4094]"
                ));
            }
            Ok(EncodeResult::Word(encode_b(OP_BRANCH, funct3, rs1, rs2, imm as i32)))
```
**Bug report:** bug_reports/encode_branch_instr_imm_oob_odd.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_branch_instr_pbt::test_encode_branch_instr_regression_imm_oob' (2737315) panicked at src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs:368:5:
beq x0, x1, 1 must Err (odd offset); got Ok(Word(1048675))
```

### B2: encode_branch_instr ignores a fourth operand

**Formal:** ∀ rs1,rs2 ∈ GPR, off ∈ even[-4096,4094], extra ∈ Operand. encode_branch_instr([Reg(rs1),Reg(rs2),Imm(off), extra], 0) = Err
**Contract evidence:** inferred (B-type is three-operand rs1, rs2, offset; llvm-mc rejects `beq x1, x2, 4, x3` as invalid operand; encode_instruction passes operands through)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_branch_instr([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], 0)
**Expected / Actual:** Err / Ok(Word(0x00000063)) encoding of beq x0, x0, 0
**Impact:** Extra tokens in a branch instruction are silently dropped, so typos assemble to a valid branch without an error.
**Root cause:** base.rs:129 uses operands.get(2) and never checks operands.len() == 3, so a fourth operand is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:129`
```rust
    match &operands.get(2) {
```
**Suggested fix:** Reject arity other than 3 before matching the offset operand.
```rust
    if operands.len() != 3 {
        return Err("branch: wrong number of operands".to_string());
    }
    match &operands[2] {
```
**Bug report:** bug_reports/encode_branch_instr_extra_operand.md
**Repro seed:** cc c8367c59bdfac9d62538e8ff3bc78142a27a713d363a3581270ff38c02fb4de6
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_branch_instr_pbt::test_encode_branch_instr_regression_extra_operand' (2737314) panicked at src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs:379:5:
beq x0, x1, 0 with a fourth operand must Err; got Ok(Word(1048675))
```

### B3: encode_branch_instr rejects SymbolOffset (symbol+addend) as a branch target

**Formal:** ∀ rs1,rs2 ∈ GPR, s ∈ ident, addend ∈ ℤ\{0}. encode_branch_instr([Reg(rs1),Reg(rs2),SymbolOffset(s,addend)], 0) = WordWithReloc{word=llvm-mc("beq rs1, rs2, 0"), reloc_type=Branch, symbol=s, addend=addend}
**Contract evidence:** inferred (parser.rs:29 SymbolOffset is the parsed form of symbol+offset; llvm-mc accepts `beq x1, x2, foo+4` as a branch fixup with value foo+4; Reloc.addend is i64)
**Documentation conflict:** (none) — base.rs:143 says "expected offset or label as 3rd operand" and does not declare symbol+addend invalid
**Severity:** medium
**Counterexample:** encode_branch_instr([Reg("x0"), Reg("x0"), SymbolOffset("foo", 1)], 0)
**Expected / Actual:** Ok(WordWithReloc { word: 0x00000063, reloc_type: Branch, symbol: "foo", addend: 1 }) / Err("branch: expected offset or label as 3rd operand")
**Impact:** Valid textual assembly `beq rs1, rs2, foo+N` cannot be encoded.
**Root cause:** base.rs:133 matches only Symbol, Label, and Reg; SymbolOffset falls through to the error arm at base.rs:143.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:133`
```rust
        Some(Operand::Symbol(s)) | Some(Operand::Label(s)) | Some(Operand::Reg(s)) => {
```
**Suggested fix:** Accept SymbolOffset as a reloc target and preserve the addend.
```rust
        Some(Operand::SymbolOffset(s, addend)) => {
            Ok(EncodeResult::WordWithReloc {
                word: encode_b(OP_BRANCH, funct3, rs1, rs2, 0),
                reloc: Relocation {
                    reloc_type: RelocType::Branch,
                    symbol: s.clone(),
                    addend: *addend,
                },
            })
        }
```
**Bug report:** bug_reports/encode_branch_instr_symbol_offset.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_branch_instr_pbt::test_encode_branch_instr_regression_symbol_offset' (2737316) panicked at src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs:403:18:
expected WordWithReloc for foo+4, got Err("branch: expected offset or label as 3rd operand")
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs | 9 properties + 8 KAT + 3 regression witnesses |

## Reproduction

Whole suite (expect 6 property failures plus 3 regression failures; 6 properties and 8 KAT pass):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_branch_instr -- --test-threads=1
```

B1 odd/out-of-range immediate:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_branch_instr_regression_imm_oob -- --test-threads=1
```

B2 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_branch_instr_regression_extra_operand -- --test-threads=1
```

B3 SymbolOffset:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_branch_instr_regression_symbol_offset -- --test-threads=1
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
- pbt-out/bug_reports/encode_branch_instr_imm_oob_odd.md
- pbt-out/bug_reports/encode_branch_instr_imm_oob_odd.html
- pbt-out/bug_reports/encode_branch_instr_extra_operand.md
- pbt-out/bug_reports/encode_branch_instr_extra_operand.html
- pbt-out/bug_reports/encode_branch_instr_symbol_offset.md
- pbt-out/bug_reports/encode_branch_instr_symbol_offset.html
- pbt-out/run/encode_branch_instr.log
- src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- proptest-regressions/backend/riscv/assembler/encoder/encode_branch_instr_pbt.txt

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 13:43 (campaign: coverage)
> Files: 12/12 scanned (100%) | Functions: 186/324 total | PBT candidates: 186 | Tested: 186 (100%) | 1 pass, 186 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 (100%) |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 186 |
| **Tested (of PBT candidates)** | **186 / 186 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 186 / -1 |
| **Overall (tested / all functions)** | **186 / 324 (57%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 186 | 186 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 186 | 186 | 0 | 100% |

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
