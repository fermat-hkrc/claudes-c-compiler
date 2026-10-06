# PBT Campaign Report: encode_lui

## Summary

**Verdict:** 4 bugs, worst high: encode_lui silently truncates out-of-range immediates (`lui x0, -1` encodes as `lui x0, 0xFFFFF`), accepts extra operands and non-%hi symbols as R_RISCV_HI20, and treats `%hi(foo+4)` as symbol `foo+4` with addend 0.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_lui
**Tests:** 12 properties (plus 5 KAT + 4 regression witnesses)
**Result:** 8 passing, 4 bugs
**Change surface:** 1 changed function (encode_lui), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual audit of the 22-line body drove every match arm.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_lui | 12 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_lui silently truncates immediates outside [0, 1048575]

**Formal:** ∀ rd ∈ GPR names, ∀ imm ∈ i64 \ [0, 1048575]. encode_lui([Reg(rd), Imm(imm)]) = Err(_)
**Contract evidence:** inferred (llvm-mc LUI operand diagnostic "integer in the range [0, 1048575]"; README.md:356 U-type imm[31:12] is 20 bits)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_lui([Reg("x0"), Imm(-1)])
**Expected / Actual:** Err / Ok(Word(0xFFFFF037))
**Impact:** Out-of-range LUI immediates assemble to a different instruction instead of being rejected, so callers get silent wrong machine code.
**Root cause:** base.rs:10 casts the i64 immediate to u32 and shifts, wrapping negatives and dropping bits above 20.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:10`
```rust
            Ok(EncodeResult::Word(encode_u(OP_LUI, rd, (*imm as u32) << 12)))
```
**Suggested fix:** Reject immediates outside the 20-bit unsigned range before shifting.
```rust
            let imm = *imm;
            if !(0..=1048575).contains(&imm) {
                return Err(format!("lui: immediate {imm} out of range [0, 1048575]"));
            }
            Ok(EncodeResult::Word(encode_u(OP_LUI, rd, (imm as u32) << 12)))
```
**Bug report:** bug_reports/encode_lui_imm_oob.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_imm_oob' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:333:5:
lui x0, -1 must Err (llvm-mc range [0, 1048575]); got Ok(Word(4294963255))
```

### B2: encode_lui ignores extra operands

**Formal:** ∀ rd, ∀ imm ∈ [0, 1048575], ∀ extra. encode_lui([Reg(rd), Imm(imm), extra]) = Err(_)
**Contract evidence:** inferred (README.md:304 two-operand U-type lui; llvm-mc "invalid operand for instruction" on a third operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_lui([Reg("x0"), Imm(0), Imm(0)])
**Expected / Actual:** Err / Ok(Word(0x37))
**Impact:** Typos and extra tokens are silently dropped, so the assembler accepts invalid syntax that llvm-mc rejects.
**Root cause:** base.rs:7 matches only operands[0] and operands[1]; operands.len() is never checked.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:7`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Require exactly two operands before encoding.
```rust
    if operands.len() != 2 {
        return Err("lui: invalid operands".to_string());
    }
    match &operands.get(1) {
```
**Bug report:** bug_reports/encode_lui_extra_operand.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_extra_operand' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:344:5:
lui x0, 0 with a third operand must Err; got Ok(Word(55))
```

### B3: encode_lui accepts plain symbols and non-%hi modifiers as R_RISCV_HI20

**Formal:** ∀ rd, ∀ s ∈ {bare ident, %pcrel_hi(ident), %lo(ident), %got_pcrel_hi(ident), %pcrel_lo(ident)}. encode_lui([Reg(rd), Symbol(s)]) = Err(_)
**Contract evidence:** inferred (llvm-mc "operand must be a symbol with %hi/%tprel_hi modifier or an integer in the range [0, 1048575]")
**Documentation conflict:** base.rs:12 "// %hi(symbol)" names the intended Symbol form; it does not declare other symbols invalid. The comment is asserted intent, not an exclusion.
**Severity:** high
**Counterexample:** encode_lui([Reg("x0"), Symbol("foo")])
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x37, Hi20, symbol: "foo", addend: 0 })
**Impact:** Bare symbols and `%pcrel_hi`/`%lo` emit R_RISCV_HI20. The linker applies the wrong reloc kind or looks up a non-existent name.
**Root cause:** base.rs:16-20 special-cases only `%tprel_hi(`; every other Symbol is classified as Hi20.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:16`
```rust
                    reloc_type: if s.starts_with("%tprel_hi(") {
                        RelocType::TprelHi20
                    } else {
                        RelocType::Hi20
                    },
```
**Suggested fix:** Accept only `%hi(` and `%tprel_hi(`; reject every other symbol form.
```rust
                    reloc_type: if s.starts_with("%tprel_hi(") {
                        RelocType::TprelHi20
                    } else if s.starts_with("%hi(") {
                        RelocType::Hi20
                    } else {
                        return Err("lui: expected %hi/%tprel_hi or integer".to_string());
                    },
```
**Bug report:** bug_reports/encode_lui_bad_modifier.md
**Repro seed:** cc b3e97c02a9431da9c8eebb88d7f71c4565d6943c13e200aea2166c2df6811a98
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_plain_symbol' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:355:5:
lui x0, foo must Err (llvm-mc requires %hi/%tprel_hi); got Ok(WordWithReloc { word: 55, reloc: Relocation { reloc_type: Hi20, symbol: "foo", addend: 0 } })
```

### B4: encode_lui treats %hi(foo+4) as symbol "foo+4" with addend 0

**Formal:** ∀ rd, ∀ sym, ∀ a ≠ 0. encode_lui([Reg(rd), Symbol("%hi("+sym+sign(a)+")")]) = Ok(WordWithReloc{Hi20, symbol=sym, addend=a})
**Contract evidence:** inferred (llvm-mc object `R_RISCV_HI20 foo 0x4` for `lui x1, %hi(foo+4)`; README.md:25 relocation modifiers)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_lui([Reg("x0"), Symbol("%hi(foo+4)")])
**Expected / Actual:** symbol="foo" addend=4 / symbol="foo+4" addend=0
**Impact:** The linker looks up the literal name `foo+4` instead of relocating `foo` with addend 4, so `symbol+offset` high-part addressing is wrong.
**Root cause:** base.rs:22-23 stores extract_modifier_symbol(s) (the full inner text) and hardcodes addend 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:22`
```rust
                    symbol: extract_modifier_symbol(s),
                    addend: 0,
```
**Suggested fix:** Split a trailing `+N`/`-N` off the extracted inner text into symbol and addend.
```rust
                    let inner = extract_modifier_symbol(s);
                    let (symbol, addend) = split_symbol_addend(&inner);
```
**Bug report:** bug_reports/encode_lui_hi_addend.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_lui_pbt::test_encode_lui_regression_hi_addend' panicked at src/backend/riscv/assembler/encoder/encode_lui_pbt.rs:368:13:
assertion `left == right` failed: %hi(foo+4) symbol must be foo, not foo+4
  left: "foo+4"
 right: "foo"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_lui_pbt.rs | 12 properties + 5 KAT + 4 regressions |
| src/backend/riscv/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_lui_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lui -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_imm_oob -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_extra_operand -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_plain_symbol -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lui_regression_hi_addend -- --test-threads=1
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
- pbt-out/bug_reports/encode_lui_imm_oob.md
- pbt-out/bug_reports/encode_lui_imm_oob.html
- pbt-out/bug_reports/encode_lui_extra_operand.md
- pbt-out/bug_reports/encode_lui_extra_operand.html
- pbt-out/bug_reports/encode_lui_bad_modifier.md
- pbt-out/bug_reports/encode_lui_bad_modifier.html
- pbt-out/bug_reports/encode_lui_hi_addend.md
- pbt-out/bug_reports/encode_lui_hi_addend.html
- proptest-regressions/backend/riscv/assembler/encoder/encode_lui_pbt.txt (proptest failure corpus, left in-tree)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 12:22 (campaign: coverage)
> Files: 12/12 scanned (100%) | Functions: 182/324 total | PBT candidates: 182 | Tested: 182 (100%) | 1 pass, 182 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 (100%) |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 182 |
| **Tested (of PBT candidates)** | **182 / 182 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 182 / -1 |
| **Overall (tested / all functions)** | **182 / 324 (56%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 182 | 182 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 182 | 182 | 0 | 100% |

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
