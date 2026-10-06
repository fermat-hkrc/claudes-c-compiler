# PBT Campaign Report: encode_load

## Summary

**Verdict:** 2 high, 2 medium: encode_load silently wraps out-of-range offsets and drops extra operands (wrong machine code), treats %hi as %lo, and rejects `ld rd, symbol+addend` that llvm-mc accepts.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_load
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_load), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter, NOT LINKED on unrelated binaries). Manual arm audit of encode_load plus two sweep properties. Recorded as file-level because the SUT is Rust cargo-test, not a C++ binary.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_load | 10 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_load ignores extra operands

**Formal:** ∀ mn, rd, rs1, off∈[-2048,2047], extra. encode_load([Reg(rd), Mem{rs1,off}, extra], f3) = Err
**Contract evidence:** inferred (llvm-mc rejects extra operands; encode_instruction passes the operand slice through unchanged)
**Documentation conflict:** (none) — no comment declares extra operands valid or ignored
**Severity:** high
**Counterexample:** encode_load([Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0)  // lb x0, 0(x0), 0
**Expected / Actual:** Err / Ok(Word(0x00000003))
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed load instead of diagnosing the line.
**Root cause:** base.rs:149 matches only `operands.get(1)` and never checks `operands.len()`, so trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:149`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Reject a slice whose length is not 2 before matching.
```rust
    if operands.len() != 2 {
        return Err("load: expected rd, offset(rs1)".to_string());
    }
    match &operands.get(1) {
```
**Bug report:** bug_reports/encode_load_extra_operand.md
**Repro seed:** cc bda355ef8e7a7edc4c74e201a4a2cd946c69ecdaaf4af8a98662136648037d94
**Raw output:**
```text
Test failed: extra operand must Err for lb x0, 0(x0) (llvm-mc rejects extra operands); got Ok(Word(3))
minimal failing input: (mn, f3) = ("lb", 0), rd = "x0", rs1 = "x0", off = 0, extra = Imm(0)
```

### B2: encode_load wraps out-of-range I-type offsets

**Formal:** ∀ mn, rd, rs1, off ∉ [-2048,2047]. llvm-mc rejects `mn rd, off(rs1)` ⇒ encode_load([Reg(rd), Mem{rs1,off}], f3) = Err
**Contract evidence:** documented README.md:353 "I-type:  [    imm[11:0]  | rs1 | funct3 |  rd  | opcode]" plus llvm-mc "integer in the range [-2048, 2047]"
**Documentation conflict:** (none) — no comment declares out-of-range offsets valid
**Severity:** high
**Counterexample:** encode_load([Reg("x0"), Mem { base: "x0", offset: 2048 }], funct3=0)  // lb x0, 2048(x0)
**Expected / Actual:** Err / Ok(Word(0x80000003))  // lb x0, -2048(x0)
**Impact:** A too-large offset wraps through the 12-bit field and becomes a large negative address with no diagnostic.
**Root cause:** base.rs:152 casts `*offset as i32` with no range check; encode_i then masks with 0xFFF, wrapping 2048 to -2048.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:152`
```rust
            Ok(EncodeResult::Word(encode_i(OP_LOAD, rd, funct3, rs1, *offset as i32)))
```
**Suggested fix:** Reject offsets outside the documented I-type range before packing.
```rust
            if !(-2048..=2047).contains(offset) {
                return Err("load: offset out of signed-12 range".to_string());
            }
            Ok(EncodeResult::Word(encode_i(OP_LOAD, rd, funct3, rs1, *offset as i32)))
```
**Bug report:** bug_reports/encode_load_imm_oob.md
**Repro seed:** (none saved; deterministic regression test_encode_load_regression_imm_oob)
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483651))
minimal failing input: (mn, f3) = ("lb", 0), rd = "x0", rs1 = "x0", imm = 2048
```

### B3: encode_load accepts %hi/%pcrel_hi/%tprel_hi as Lo12I

**Formal:** ∀ mn, rd, rs1, s, hi ∈ {%hi,%pcrel_hi,%tprel_hi}. llvm-mc rejects `mn rd, %hi(s)(rs1)` ⇒ encode_load([Reg(rd), MemSymbol{rs1, "%hi(s)"}], f3) = Err
**Contract evidence:** inferred (llvm-mc: "operand must be a symbol with %lo/%pcrel_lo/%tprel_lo modifier or an integer in the range [-2048, 2047]")
**Documentation conflict:** base.rs:157 "Use Lo12I for load-type relocations" — purpose comment on the Hi20→Lo12I remap; it does not declare %hi a valid load operand. The comment is context, not an input-domain restriction.
**Severity:** medium
**Counterexample:** encode_load([Reg("x0"), MemSymbol { base: "x0", symbol: "%hi(foo)", modifier: "" }], funct3=0)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x00000003, reloc_type: Lo12I, symbol: "foo", addend: 0 })
**Impact:** `%hi(sym)` on a load is silently treated as `%lo(sym)`, so the linker patches the low 12 bits of a symbol the source asked to take the high 20 bits of.
**Root cause:** base.rs:161 remaps RelocType::Hi20 to Lo12I instead of rejecting hi-type modifiers on I-type loads.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:161`
```rust
                RelocType::Hi20 => RelocType::Lo12I,
```
**Suggested fix:** Accept only lo-type modifiers on loads.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::Lo12I | RelocType::TprelLo12I => reloc_type,
                _ => return Err("load: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```
**Bug report:** bug_reports/encode_load_hi_modifier.md
**Repro seed:** (none saved; deterministic regression test_encode_load_regression_hi_modifier)
**Raw output:**
```text
Test failed: hi-type modifier %hi(foo) must Err on load (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 3, reloc: Relocation { reloc_type: Lo12I, symbol: "foo", addend: 0 } })
minimal failing input: (mn, f3) = ("lb", 0), rd = "x0", rs1 = "x0", s = "foo", hi = "%hi"
```

### B4: encode_load rejects ld rd, symbol+addend

**Formal:** ∀ mn, rd, s, addend≠0. encode_load([Reg(rd), SymbolOffset(s, addend)], f3) = WordsWithRelocs[(auipc, PcrelHi20, s, addend), (load, PcrelLo12I, s, _)]
**Contract evidence:** inferred (llvm-mc accepts `ld x1, foo+4` as auipc+%pcrel_hi(foo+4) plus ld; parser.rs:29 documents Operand::SymbolOffset; base.rs:173 documents the bare-symbol expansion)
**Documentation conflict:** (none) — base.rs:173-175 describes Symbol/Label only and does not declare SymbolOffset invalid
**Severity:** medium
**Counterexample:** encode_load([Reg("x0"), SymbolOffset("foo", 1)], funct3=0)  // lb x0, foo+1
**Expected / Actual:** Ok(WordsWithRelocs) of auipc+lb with PcrelHi20 addend=1 / Err("load: expected memory operand")
**Impact:** Valid `ld rd, foo+4` fails to assemble, so objects llvm-mc would produce are rejected.
**Root cause:** base.rs:176 matches only Symbol and Label; SymbolOffset falls through to the default error at base.rs:190.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:190`
```rust
        _ => Err("load: expected memory operand".to_string()),
```
**Suggested fix:** Handle SymbolOffset like Symbol, copying the addend onto the AUIPC reloc.
```rust
        Some(Operand::SymbolOffset(s, addend)) => {
            Ok(EncodeResult::WordsWithRelocs(vec![
                (encode_u(OP_AUIPC, rd, 0), Some(Relocation {
                    reloc_type: RelocType::PcrelHi20,
                    symbol: s.clone(),
                    addend: *addend,
                })),
                (encode_i(OP_LOAD, rd, funct3, rd, 0), Some(Relocation {
                    reloc_type: RelocType::PcrelLo12I,
                    symbol: s.clone(),
                    addend: 0,
                })),
            ]))
        }
```
**Bug report:** bug_reports/encode_load_symbol_offset.md
**Repro seed:** (none saved; deterministic regression test_encode_load_regression_symbol_offset)
**Raw output:**
```text
Test failed: expected WordsWithRelocs for lb x0, foo+1, got Err("load: expected memory operand").
minimal failing input: (mn, f3) = ("lb", 0), rd = "x0", s = "foo", addend = 1
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_load_pbt.rs | 10 properties + 7 KAT + 4 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load_neg_extra -- --test-threads=1
```

B2 imm oob:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load_neg_imm_oob -- --test-threads=1
```

B3 hi modifier:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load_neg_hi_modifier -- --test-threads=1
```

B4 symbol offset:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_load_symbol_offset_addend -- --test-threads=1
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
- pbt-out/CHANGE_SURFACE.md
- pbt-out/bug_reports/encode_load_extra_operand.md
- pbt-out/bug_reports/encode_load_extra_operand.html
- pbt-out/bug_reports/encode_load_imm_oob.md
- pbt-out/bug_reports/encode_load_imm_oob.html
- pbt-out/bug_reports/encode_load_hi_modifier.md
- pbt-out/bug_reports/encode_load_hi_modifier.html
- pbt-out/bug_reports/encode_load_symbol_offset.md
- pbt-out/bug_reports/encode_load_symbol_offset.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 14:06 (campaign: coverage)
> Files: 12/12 scanned (100%) | Functions: 187/324 total | PBT candidates: 187 | Tested: 187 (100%) | 1 pass, 187 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 (100%) |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 187 |
| **Tested (of PBT candidates)** | **187 / 187 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 187 / -1 |
| **Overall (tested / all functions)** | **187 / 324 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 187 | 187 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 187 | 187 | 0 | 100% |

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
