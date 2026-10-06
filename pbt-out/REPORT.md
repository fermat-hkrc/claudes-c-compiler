# PBT Campaign Report: encode_cond_branch

## Summary

**Verdict:** 1 high, 2 medium: encode_cond_branch rejects every immediate PC offset (`b.eq #0` is Err), ignores extra operands, and treats `:lo12:` modifiers as plain symbols, so gas-compatible `b.eq`/`b.ne`/… assembly is wrong or silently accepted.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_cond_branch
**Tests:** 11
**Result:** 8 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual audit of the 14-line body plus three targeted properties.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cond_branch | 11 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_cond_branch rejects a valid immediate PC offset

**Formal:** ∀ cond ∈ {eq,ne,cs,hs,cc,lo,mi,pl,vs,vc,hi,ls,ge,lt,gt,le,al,nv}, ∀ imm ∈ {k·4 | k ∈ ℤ, −1048576 ≤ k·4 ≤ 1048572}. encode_cond_branch(cond, [Imm(imm)]) = Word(w) ∧ w = llvm-mc(`b.{cond} #imm`)
**Contract evidence:** documented compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" plus README.md:12 gas contract — ARM/gas/llvm-mc all accept a 19-bit immediate displacement
**Documentation conflict:** (none) — the encoding comment states imm19 is part of the instruction; the code never handles Imm
**Severity:** high
**Counterexample:** encode_cond_branch("eq", [Imm(-1048576)]) — `b.eq #-1048576`
**Expected / Actual:** Word matching llvm-mc 0x54800000 / Err("expected symbol at operand 0, got Some(Imm(-1048576))")
**Impact:** Immediate-form conditional branches that GNU as and llvm-mc assemble (`b.eq #0`, `b.ne #4`, …) cannot be produced by this encoder.
**Root cause:** compare_branch.rs:199 always calls get_symbol, which has no Operand::Imm arm, so every immediate form is rejected instead of encoding 01010100 imm19 0 cond.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:199`
```rust
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Suggested fix:** Handle Operand::Imm: require a 4-byte-aligned offset in [-1048576, 1048572] and emit Word with imm19 = imm/4.
```rust
    if let Some(Operand::Imm(imm)) = operands.get(0) {
        if operands.len() != 1 {
            return Err(format!("b.{}: extra operand", cond));
        }
        if imm % 4 != 0 || *imm < -1_048_576 || *imm > 1_048_572 {
            return Err(format!("b.{} offset {} unaligned or out of range", cond, imm));
        }
        let imm19 = ((*imm as i32) >> 2) as u32 & 0x7ffff;
        return Ok(EncodeResult::Word((0b01010100 << 24) | (imm19 << 5) | cond_val));
    }
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Bug report:** bug_reports/encode_cond_branch_imm_offset.md
**Repro seed:** cc c4c8a4a11772a960febcea7927a7b6864b93d2b36271b77ac170ddb03c77b231
**Raw output:** Test failed: SUT rejected valid B.cond b.eq #-1048576: Err("expected symbol at operand 0, got Some(Imm(-1048576))"). minimal failing input: cond = "eq", imm = -1048576

### B2: encode_cond_branch ignores extra operands

**Formal:** ∀ cond ∈ 18 names, ∀ s, ∀ extra ∈ {Reg,Imm,Symbol,Mem}. encode_cond_branch(cond, [Symbol(s), extra]) is Err
**Contract evidence:** inferred (README.md:12 gas contract; llvm-mc/gas reject a second operand on b.eq)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_cond_branch("eq", [Symbol("labl0"), Reg("x0")])
**Expected / Actual:** Err / Ok(WordWithReloc CondBr19 labl0)
**Impact:** `b.eq foo, x0` is assembled as `b.eq foo`; invalid assembly is silently accepted.
**Root cause:** compare_branch.rs:199-209 calls get_symbol(operands, 0) and returns success without checking operands.len().
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:199`
```rust
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Suggested fix:** Reject any operand list whose length is not exactly 1.
```rust
    if operands.len() != 1 {
        return Err(format!("b.{}: expected 1 operand, got {}", cond, operands.len()));
    }
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Bug report:** bug_reports/encode_cond_branch_extra_operand.md
**Repro seed:** (deterministic; fails on first case cond="eq", suffix=0, which=0)
**Raw output:** Test failed: b.eq label, extra (which=0) must Err (llvm-mc: invalid operand). minimal failing input: cond = "eq", suffix = 0, which = 0

### B3: encode_cond_branch accepts :lo12: modifiers as branch targets

**Formal:** ∀ cond ∈ 18 names. encode_cond_branch(cond, [Modifier{lo12, foo}|ModifierOffset{lo12, foo, 8}]) is Err
**Contract evidence:** inferred (README.md:12 gas contract; llvm-mc/gas reject b.eq :lo12:foo)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_cond_branch("eq", [Modifier { kind: "lo12", symbol: "foo" }])
**Expected / Actual:** Err / Ok(WordWithReloc CondBr19 symbol foo addend 0)
**Impact:** `:lo12:` is dropped and the inner symbol is used as a CondBr19 target, which is not a valid B.cond addressing mode.
**Root cause:** compare_branch.rs:199 calls get_symbol, whose Modifier/ModifierOffset arms return the inner symbol and drop the modifier kind.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:199`
```rust
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Suggested fix:** Reject Operand::Modifier and Operand::ModifierOffset before calling get_symbol.
```rust
    match operands.get(0) {
        Some(Operand::Modifier { .. }) | Some(Operand::ModifierOffset { .. }) => {
            return Err(format!("b.{} does not take a relocation modifier", cond));
        }
        _ => {}
    }
    let (sym, addend) = get_symbol(operands, 0)?;
```
**Bug report:** bug_reports/encode_cond_branch_modifier.md
**Repro seed:** (deterministic; fails on first case cond="eq", which=0)
**Raw output:** Test failed: b.eq :lo12:foo must Err (which=0); llvm-mc/gas reject modifiers. minimal failing input: cond = "eq", which = 0

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs | 11 properties + 4 KAT + 3 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `mod encode_cond_branch_pbt` registration |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cond_branch -- --test-threads=1
```

B1 immediate offset:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cond_branch_diff_imm_llvm_mc -- --test-threads=1
```

B2 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cond_branch_neg_extra_operand -- --test-threads=1
```

B3 modifier:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cond_branch_neg_modifier -- --test-threads=1
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
- pbt-out/bug_reports/encode_cond_branch_imm_offset.md
- pbt-out/bug_reports/encode_cond_branch_imm_offset.html
- pbt-out/bug_reports/encode_cond_branch_extra_operand.md
- pbt-out/bug_reports/encode_cond_branch_extra_operand.html
- pbt-out/bug_reports/encode_cond_branch_modifier.md
- pbt-out/bug_reports/encode_cond_branch_modifier.html
- pbt-out/build.log
- pbt-out/run/ (scratch; cargo used the repo target/ dir)

Tier: standard. Sweep round 1/1 spent (coverage_gaps file-level NOT LINKED; manual body audit + neg_bad_operand / symbol_misclassified / neg_modifier). Closed: every documented behavior of encode_cond_branch has a property; remaining gaps are the three filed bugs.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 11:31 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 180/307 total | PBT candidates: 180 | Tested: 180 (100%) | 1 pass, 180 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 180 |
| **Tested (of PBT candidates)** | **180 / 180 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 180 / -1 |
| **Overall (tested / all functions)** | **180 / 307 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 180 | 180 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 180 | 180 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 18 | 18 | 100% | covered |
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
