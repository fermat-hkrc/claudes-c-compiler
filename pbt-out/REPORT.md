# PBT Campaign Report: encode_jalr

## Summary

**Verdict:** 1 high: encode_jalr silently wraps out-of-range immediates (`jalr x0, x1, 2048` encodes as `jalr x0, x1, -2048`); plus 2 medium: 1-operand `jalr 8(x2)` and `jalr ra, %pcrel_lo(foo)(ra)` are rejected.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_jalr
**Tests:** 10
**Result:** 7 passing, 3 bugs
**Change surface:** 1 changed function (encode_jalr), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_jalr NOT LINKED). Execution evidence is `cargo test --lib encode_jalr` (KAT + 1000-case proptest). Tier: standard.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_jalr | 10 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_jalr silently truncates out-of-range immediates

**Formal:** ∀ rd, rs1 ∈ GPR, ∀ off ∈ ℤ \ [-2048, 2047]. llvm-mc("jalr rd, rs1, off") errors ∧ encode_jalr([Reg(rd), Reg(rs1), Imm(off)]) = Err(_)
**Contract evidence:** documented src/backend/riscv/assembler/README.md:353 "I-type:  [    imm[11:0]  | rs1 | funct3 |  rd  | opcode]" — 12-bit signed field; llvm-mc rejects values outside [-2048, 2047]
**Documentation conflict:** (none) — the I-type layout states a 12-bit immediate; the code does not admit a limitation, it silently masks
**Severity:** high
**Counterexample:** encode_jalr([Reg("x0"), Reg("x1"), Imm(2048)])
**Expected / Actual:** Err / Ok(Word(0x80008067)) encoding of jalr x0, x1, -2048
**Impact:** An out-of-range JALR offset assembles to a jump in the opposite direction instead of being rejected, so callers get silent wrong machine code.
**Root cause:** base.rs:119 casts the i64 immediate to i32 with no range check; encode_i then masks with 0xFFF, so 2048 becomes the 12-bit pattern of -2048.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:119`
```rust
            Ok(EncodeResult::Word(encode_i(OP_JALR, rd, 0, rs1, imm as i32)))
```
**Suggested fix:** Reject immediates outside the signed 12-bit range before packing.
```rust
            let imm = get_imm(operands, 2)?;
            if !(-2048..=2047).contains(&imm) {
                return Err(format!(
                    "jalr: immediate {imm} must be in [-2048, 2047]"
                ));
            }
            Ok(EncodeResult::Word(encode_i(OP_JALR, rd, 0, rs1, imm as i32)))
```
**Bug report:** bug_reports/encode_jalr_imm_oob.md
**Repro seed:** cc 0be9b7e62c879361fca694df444b990ff4923b0e01c60034a330862f9800b7c9
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jalr_pbt::test_encode_jalr_regression_imm_oob' (2734246) panicked at src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs:311:5:
jalr x0, x1, 2048 must Err (imm12 range); got Ok(Word(2147516519))
```

### B2: encode_jalr rejects 1-operand mem form jalr off(rs1)

**Formal:** ∀ rs1 ∈ GPR, ∀ off ∈ [-2048, 2047]. encode_jalr([Mem{base: rs1, offset: off}]) = encode_jalr([Reg("ra"), Reg(rs1), Imm(off)]) = Word(llvm-mc("jalr off(rs1)"))
**Contract evidence:** inferred (llvm-mc accepts `jalr off(rs1)` as jalr ra, off(rs1); analogous to documented 1-operand `jalr rs1` with implicit rd=ra; parser produces Mem for that text)
**Documentation conflict:** (none) — base.rs:96 documents `jalr rs1 (rd = ra, offset = 0)` but does not declare Mem invalid
**Severity:** medium
**Counterexample:** encode_jalr([Mem { base: "x2", offset: 8 }])
**Expected / Actual:** Ok(Word) equal to jalr ra, x2, 8 / Err("expected register at operand 0, got Some(Mem { base: \"x2\", offset: 8 })")
**Impact:** Valid textual assembly `jalr 8(x2)` cannot be encoded.
**Root cause:** base.rs:97 the 1-operand arm always calls get_reg; Mem is not matched.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:97`
```rust
            let rs1 = get_reg(operands, 0)?;
```
**Suggested fix:** Accept a Mem operand in the 1-operand arm with implicit rd=ra.
```rust
                Operand::Mem { base, offset } => {
                    let rs1 = reg_num(base).ok_or("invalid base register")?;
                    Ok(EncodeResult::Word(encode_i(OP_JALR, 1, 0, rs1, *offset as i32)))
                }
```
**Bug report:** bug_reports/encode_jalr_one_operand_mem.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jalr_pbt::test_encode_jalr_regression_one_operand_mem' (2734247) panicked at src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs:342:18:
expected Word for jalr 8(x2), got Err("expected register at operand 0, got Some(Mem { base: \"x2\", offset: 8 })")
```

### B3: encode_jalr rejects %pcrel_lo/%lo mem-symbol operands

**Formal:** ∀ rd, rs1 ∈ GPR, ∀ s ∈ ident. encode_jalr([Reg(rd), MemSymbol{base: rs1, symbol: "%pcrel_lo(s)"}]) = WordWithReloc{word: encode_jalr([Reg(rd), Mem{rs1,0}]), type: PcrelLo12I, symbol: s, addend: 0} and likewise %lo → Lo12I
**Contract evidence:** documented src/backend/riscv/assembler/README.md:334 "`call sym` → `auipc ra, %pcrel_hi(sym)` + `jalr ra, %pcrel_lo(sym)(ra)`"; llvm-mc accepts the same syntax
**Documentation conflict:** README.md:334 states the jalr %pcrel_lo form as the expansion of call — the comment states the behavior IS handled at assembler level; encode_jalr (the jalr mnemonic) rejects it. (not independently verified that call's pseudo path is the only producer)
**Severity:** medium
**Counterexample:** encode_jalr([Reg("ra"), MemSymbol { base: "ra", symbol: "%pcrel_lo(foo)", modifier: "" }])
**Expected / Actual:** Ok(WordWithReloc { PcrelLo12I, symbol: "foo", addend: 0 }) / Err("jalr: invalid operands")
**Impact:** Hand-written `jalr ra, %pcrel_lo(foo)(ra)` cannot be assembled even though the assembler documents that instruction sequence.
**Root cause:** base.rs:112 the 2-operand match handles only Reg and Mem; MemSymbol falls through to the invalid-operands error.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:112`
```rust
                _ => Err("jalr: invalid operands".to_string()),
```
**Suggested fix:** Handle MemSymbol like encode_load's I-type reloc arm (PcrelLo12I / Lo12I / TprelLo12I).
```rust
                Operand::MemSymbol { base, symbol, .. } => {
                    let rs1 = reg_num(base).ok_or("invalid base register")?;
                    let (reloc_type, sym) = parse_reloc_modifier(symbol);
                    let reloc_type = match reloc_type {
                        RelocType::PcrelHi20 => RelocType::PcrelLo12I,
                        RelocType::Hi20 => RelocType::Lo12I,
                        RelocType::TprelHi20 => RelocType::TprelLo12I,
                        other => other,
                    };
                    Ok(EncodeResult::WordWithReloc {
                        word: encode_i(OP_JALR, rd, 0, rs1, 0),
                        reloc: Relocation {
                            reloc_type,
                            symbol: sym,
                            addend: 0,
                        },
                    })
                }
```
**Bug report:** bug_reports/encode_jalr_pcrel_lo.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_jalr_pbt::test_encode_jalr_regression_pcrel_lo' (2734248) panicked at src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs:369:18:
expected WordWithReloc for jalr ra, %pcrel_lo(foo)(ra), got Err("jalr: invalid operands")
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs | 10 properties + 7 KAT + 3 failing regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_jalr -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jalr_regression_imm_oob -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jalr_regression_one_operand_mem -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_jalr_regression_pcrel_lo -- --test-threads=1
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
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_jalr_imm_oob.md
- pbt-out/bug_reports/encode_jalr_imm_oob.html
- pbt-out/bug_reports/encode_jalr_one_operand_mem.md
- pbt-out/bug_reports/encode_jalr_one_operand_mem.html
- pbt-out/bug_reports/encode_jalr_pcrel_lo.md
- pbt-out/bug_reports/encode_jalr_pcrel_lo.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 13:22 (campaign: coverage)
> Files: 12/12 scanned (100%) | Functions: 185/324 total | PBT candidates: 185 | Tested: 185 (100%) | 1 pass, 185 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 (100%) |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 185 |
| **Tested (of PBT candidates)** | **185 / 185 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 185 / -1 |
| **Overall (tested / all functions)** | **185 / 324 (57%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 185 | 185 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 185 | 185 | 0 | 100% |

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
