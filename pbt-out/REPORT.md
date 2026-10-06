# PBT Campaign Report: encode_store

## Summary

**Verdict:** 5 high: encode_store silently ignores extra operands, wraps out-of-range offsets (2048 → −2048), remaps %hi to Lo12S, accepts GOT/TLS MemSymbol modifiers as GotHi20, and emits I-type lo relocs (Lo12I/PcrelLo12I/TprelLo12I) on S-type stores that the ELF writer will patch with the wrong bit layout.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_store
**Tests:** 9
**Result:** 4 passing, 5 bugs
**Change surface:** 1 changed function (encode_store), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter, unrelated binaries, claimed encode_store NOT LINKED). Manual arm audit of the 26-line body plus one sweep property.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_store | 9 | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_store ignores extra operands

**Formal:** ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, off ∈ [-2048, 2047], extra ∈ Operand. llvm-mc rejects "mn rs2, off(rs1)" with a trailing operand ⇒ encode_store([Reg(rs2), Mem{rs1, off}, extra], f3) is Err
**Contract evidence:** inferred (llvm-mc rejects extra operands on sb/sh/sw/sd; encode_instruction passes the operand slice through)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_store([Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0)  // sb x0, 0(x0), 0
**Expected / Actual:** Err / Ok(Word(0x00000023))
**Impact:** A mistyped extra operand is silently dropped, so the assembler emits a well-formed store instead of diagnosing the line.
**Root cause:** base.rs:196 matches only `operands.get(1)` and never checks `operands.len()`, so any trailing operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:196`
```rust
    match &operands.get(1) {
```
**Suggested fix:** Reject a slice longer than two operands before matching.
```rust
    if operands.len() != 2 {
        return Err("store: expected rs2, offset(rs1)".to_string());
    }
    match &operands.get(1) {
```
**Bug report:** bug_reports/encode_store_extra_operand.md
**Repro seed:** cc 070aab56176ae6228c105ce28d882d5fa274aac125d844095d39134820f11bbb
**Raw output:**
```text
Test failed: extra operand must Err for sb x0, 0(x0) (llvm-mc rejects extra operands); got Ok(Word(35)) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:535.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", off = 0, extra = Imm(
    0,
)
```

### B2: encode_store wraps out-of-range store offsets

**Formal:** ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, imm ∉ [-2048, 2047]. llvm-mc rejects "mn rs2, imm(rs1)" ⇒ encode_store([Reg(rs2), Mem{rs1, imm}], f3) is Err
**Contract evidence:** inferred (llvm-mc: store offset must be an integer in [-2048, 2047]; README.md:354 S-type 12-bit immediate)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_store([Reg("x0"), Mem { base: "x0", offset: 2048 }], funct3=0)  // sb x0, 2048(x0)
**Expected / Actual:** Err / Ok(Word(0x80000023)) encoding of sb x0, -2048(x0)
**Impact:** Offsets such as 2048 are encoded as the wrapped 12-bit pattern, so a store that the source wrote as a large displacement silently hits the wrong address.
**Root cause:** base.rs:199 casts `*offset as i32` into encode_s, which keeps only imm[11:5]|imm[4:0] and never range-checks the 12-bit signed field.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:199`
```rust
            Ok(EncodeResult::Word(encode_s(OP_STORE, funct3, rs1, rs2, *offset as i32)))
```
**Suggested fix:** Reject immediates outside [-2048, 2047] before packing.
```rust
            if !(-2048..=2047).contains(&offset) {
                return Err("store: immediate out of range [-2048, 2047]".to_string());
            }
            Ok(EncodeResult::Word(encode_s(OP_STORE, funct3, rs1, rs2, *offset as i32)))
```
**Bug report:** bug_reports/encode_store_imm_oob.md
**Repro seed:** (deterministic regression: offset=2048)
**Raw output:**
```text
Test failed: oob imm 2048 must Err (llvm-mc range [-2048, 2047]); got Ok(Word(2147483683)) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:517.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", imm = 2048
```

### B3: encode_store accepts non-lo MemSymbol modifiers

**Formal:** ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, s ∈ ident, hi ∈ {%hi, %pcrel_hi, %tprel_hi}. llvm-mc rejects "mn rs2, hi(s)(rs1)" ⇒ encode_store([Reg(rs2), MemSymbol{rs1, "hi(s)"}], f3) is Err
**Contract evidence:** inferred (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on S-type store offsets)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_store([Reg("x0"), MemSymbol { base: "x0", symbol: "%hi(foo)" }], funct3=0); also %got_pcrel_hi(foo) → GotHi20
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x00000023, reloc_type: Lo12S, symbol: "foo" }) for %hi; GotHi20 for %got_pcrel_hi
**Impact:** A hi-type or GOT/TLS modifier is accepted and a store with imm=0 is emitted. The linker then patches the wrong reloc class, producing a wrong address.
**Root cause:** base.rs:204-209 remaps PcrelHi20/Hi20/TprelHi20 onto S-type lo reloc kinds and passes every other kind through, instead of rejecting modifiers llvm-mc does not accept on stores.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:204`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelHi20 => RelocType::TprelLo12S,
                other => other,
            };
```
**Suggested fix:** Return Err for non-lo modifiers; remap only I-type lo variants to S-type.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Lo12I | RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelLo12I | RelocType::TprelHi20 => RelocType::TprelLo12S,
                RelocType::PcrelLo12S | RelocType::Lo12S | RelocType::TprelLo12S => reloc_type,
                _ => return Err("store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```
**Bug report:** bug_reports/encode_store_hi_modifier.md
**Repro seed:** (deterministic regression: %hi(foo) and %got_pcrel_hi(foo))
**Raw output:**
```text
Test failed: hi-type modifier %hi(foo) must Err on store (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo); got Ok(WordWithReloc { word: 35, reloc: Relocation { reloc_type: Lo12S, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:604.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", s = "foo", hi = "%hi"
```

### B4: encode_store emits I-type lo relocs on S-type stores

**Formal:** ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, s ∈ ident. encode_store([Reg(rs2), MemSymbol{rs1, "%lo(s)"}]) = WordWithReloc{word = encode_store([Reg(rs2), Mem{rs1, 0}]), reloc_type = Lo12S, symbol = s, addend = 0} ∧ same with %pcrel_lo → PcrelLo12S ∧ %tprel_lo → TprelLo12S
**Contract evidence:** documented encoder/mod.rs:77 "R_RISCV_LO12_S - for SW/SD (absolute low 12 bits, S-type)" and encoder/mod.rs:71 "R_RISCV_PCREL_LO12_S - for SW/SD (low 12 bits of PC-relative, S-type)"
**Documentation conflict:** encoder/mod.rs:77 states Lo12S is for SW/SD; the code emits Lo12I/PcrelLo12I/TprelLo12I. The comment states the behavior IS handled (S-type lo reloc) — documented-and-violated.
**Severity:** high
**Counterexample:** encode_store([Reg("x0"), MemSymbol { base: "x0", symbol: "%pcrel_lo(foo)" }], funct3=0)  // sb x0, %pcrel_lo(foo)(x0)
**Expected / Actual:** PcrelLo12S / PcrelLo12I (and Lo12I for %lo, TprelLo12I for %tprel_lo)
**Impact:** The ELF writer applies I-type imm[31:20] patching to an S-type instruction, overwriting rs2 and the scattered imm field. Every relocatable sb/sh/sw/sd rs, %lo(sym)(base) links to a corrupted store.
**Root cause:** parse_reloc_modifier returns I-type lo variants. base.rs:204-209 only remaps Hi20 kinds onto S-type lo relocs and passes `other => other`, so the valid lo modifiers keep the I-type reloc.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:204`
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelHi20 => RelocType::TprelLo12S,
                other => other,
            };
```
**Suggested fix:** Remap the I-type lo variants that parse_reloc_modifier actually returns for %lo/%pcrel_lo/%tprel_lo.
```rust
            let reloc_type = match reloc_type {
                RelocType::PcrelLo12I | RelocType::PcrelHi20 => RelocType::PcrelLo12S,
                RelocType::Lo12I | RelocType::Hi20 => RelocType::Lo12S,
                RelocType::TprelLo12I | RelocType::TprelHi20 => RelocType::TprelLo12S,
                RelocType::PcrelLo12S | RelocType::Lo12S | RelocType::TprelLo12S => reloc_type,
                _ => return Err("store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
            };
```
**Bug report:** bug_reports/encode_store_lo_reloc_i_type.md
**Repro seed:** (deterministic regression: %lo(foo) must be Lo12S)
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `"PcrelLo12I"`,
 right: `"PcrelLo12S"` at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:473.
minimal failing input: (_mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", s = "foo"
```

### B5: encode_store accepts GOT/TLS/plain MemSymbol modifiers

**Formal:** ∀ mn ∈ {sb,sh,sw,sd}, rs2, rs1 ∈ GPR, s ∈ ident, mod ∈ {got_pcrel_hi, tls_ie_pcrel_hi, tls_gd_pcrel_hi, tprel_add, plain}. llvm-mc rejects the corresponding store mem operand ⇒ encode_store([Reg(rs2), MemSymbol{rs1, form(mod,s)}], f3) is Err
**Contract evidence:** inferred (llvm-mc only allows %lo/%pcrel_lo/%tprel_lo on S-type store offsets)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_store([Reg("x0"), MemSymbol{base:"x0", symbol:"%got_pcrel_hi(foo)"}], funct3=0) -> Ok(WordWithReloc{word:35, reloc_type:GotHi20, symbol:"foo"})
**Expected / Actual:** Err / Ok(WordWithReloc { word: 35, reloc_type: GotHi20, symbol: "foo" })
**Impact:** %got_pcrel_hi is accepted with reloc_type GotHi20 on an S-type store. The linker then patches the wrong reloc class.
**Root cause:** base.rs:208 passes unmatched reloc kinds through `other => other`, so GotHi20 from parse_reloc_modifier is stored on the S-type instruction.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/base.rs:208`
```rust
                other => other,
```
**Suggested fix:** Reject non-lo modifiers.
```rust
                _ => return Err("store: expected %lo/%pcrel_lo/%tprel_lo".to_string()),
```
**Bug report:** bug_reports/encode_store_other_modifier.md
**Repro seed:** (deterministic regression: %got_pcrel_hi(foo))
**Raw output:**
```text
Test failed: non-lo modifier %got_pcrel_hi(foo) must Err on store; got Ok(WordWithReloc { word: 35, reloc: Relocation { reloc_type: GotHi20, symbol: "foo", addend: 0 } }) at src/backend/riscv/assembler/encoder/encode_store_pbt.rs:641.
minimal failing input: (mn, f3) = (
    "sb",
    0,
), rs2 = "x0", rs1 = "x0", form = "%got_pcrel_hi(foo)"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_store_pbt.rs | 9 properties + 6 KAT + 5 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_extra -- --test-threads=1
```

B2 imm oob:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_imm_oob -- --test-threads=1
```

B3 hi modifier:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_hi_modifier -- --test-threads=1
```

B4 I-type lo reloc:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_reloc_lo -- --test-threads=1
```

B5 GOT/TLS/plain modifier:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_store_neg_other_modifier -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign phases
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/report.json — machine-readable report
- pbt-out/run/encode_store_test.log — first full test run
- pbt-out/run/encode_store_sweep.log — sweep property run
- pbt-out/bug_reports/encode_store_extra_operand.md and .html
- pbt-out/bug_reports/encode_store_imm_oob.md and .html
- pbt-out/bug_reports/encode_store_hi_modifier.md and .html
- pbt-out/bug_reports/encode_store_lo_reloc_i_type.md and .html
- pbt-out/bug_reports/encode_store_other_modifier.md and .html
- src/backend/riscv/assembler/encoder/encode_store_pbt.rs — harness
- proptest-regressions/backend/riscv/assembler/encoder/encode_store_pbt.txt — proptest failure cache from the run

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 14:25 (campaign: coverage)
> Files: 12/12 scanned (100%) | Functions: 188/324 total | PBT candidates: 188 | Tested: 188 (100%) | 1 pass, 188 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 12 |
| Files scanned | 12 / 12 (100%) |
| Total functions (all files) | 324 |
| PBT candidates (from FUNCTION_INDEX) | 188 |
| **Tested (of PBT candidates)** | **188 / 188 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 188 / -1 |
| **Overall (tested / all functions)** | **188 / 324 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 188 | 188 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 188 | 188 | 0 | 100% |

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
