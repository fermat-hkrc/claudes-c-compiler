# PBT Campaign Report: encode_neon_movi

## Summary

**Verdict:** 2 high, 2 medium: encode_neon_movi drops LSL #8 on .4h/.8h (encodes unshifted), truncates `#256` to `#0`, ignores extra operands, and encodes MSL as LSL #0 — any of those silently emit the wrong 32-bit word.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_movi
**Tests:** 9 properties (plus 1 KAT + 4 failing regression witnesses)
**Result:** 5 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_movi), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit added documented-error isolation (invalid T / illegal LSL amount / bad 2d byte), all passing.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_movi | 9 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_movi ignores LSL #8 on .4h/.8h

**Formal:** ∀ rd ∈ [0,31], T ∈ {4h,8h}, imm8 ∈ [0,255]. encode_neon_movi([Vd.T, #imm8, lsl #8]) = llvm-mc("movi Vd.T, #imm8, lsl #8")
**Contract evidence:** inferred (ARM AdvSIMD modified immediate cmode=1010; llvm-mc/gas accept `movi v0.4h, #0, lsl #8`; parser.rs:1887 emits Operand::Shift { kind: "lsl" })
**Documentation conflict:** neon.rs:708 "cmode=1000 for .4h/.8h with no shift" describes the no-shift encoding; it does not declare LSL #8 invalid.
**Severity:** high
**Counterexample:** encode_neon_movi([v0.4h, #0, lsl #8])
**Expected / Actual:** llvm-mc 0x0f00a400 / SUT 0x0f008400
**Impact:** 16-bit MOVI with LSL #8 silently produces the unshifted immediate, so each halfword is imm8 instead of imm8<<8.
**Root cause:** neon.rs:701-710 the .4h/.8h arm hard-codes cmode=1000 and never inspects a third Shift operand.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:701`
```rust
        "4h" | "8h" => {
            // MOVI Vd.4h/8h, #imm8
            let q: u32 = if arr_d == "8h" { 1 } else { 0 };
```
**Suggested fix:** Peek at a third LSL operand; LSL #0 → cmode=1000, LSL #8 → cmode=1010, other amounts Err.
```rust
            let cmode = if let Some(Operand::Shift { kind, amount }) = operands.get(2) {
                if kind == "lsl" {
                    match *amount {
                        0 => 0b1000u32,
                        8 => 0b1010,
                        _ => return Err(format!("movi: unsupported shift amount: {}", amount)),
                    }
                } else {
                    return Err(format!("movi: unsupported shift kind: {}", kind));
                }
            } else {
                0b1000
            };
```
**Bug report:** bug_reports/encode_neon_movi_h_lsl8.md
**Repro seed:** rd = 0, t = "4h", imm8 = 0
**Raw output:**
```text
assertion `left == right` failed: movi v0.4h, #0, lsl #8 must match llvm-mc (cmode=1010, 0x0f00a400)
  left: 251692032
 right: 251700224
```

### B2: encode_neon_movi encodes MSL as unshifted LSL #0

**Formal:** ∀ rd ∈ [0,31], T ∈ {2s,4s}, imm8 ∈ [0,255], n ∈ {8,16}. encode_neon_movi([Vd.T, #imm8, msl #n]) = llvm-mc("movi Vd.T, #imm8, msl #n")
**Contract evidence:** inferred (ARM cmode=110x; llvm-mc/gas accept MSL; sibling encode_neon_mvni implements MSL)
**Documentation conflict:** neon.rs:675 "Check for optional LSL shift operand" asserts LSL handling; it does not declare MSL invalid.
**Severity:** medium
**Counterexample:** encode_neon_movi([v0.2s, #0, msl #8])
**Expected / Actual:** llvm-mc 0x0f00c400 / SUT 0x0f000400
**Impact:** MSL #8 (ones shifted in from the right) encodes as unshifted #0. Parser does not currently tokenize `msl`, so the public assembler also cannot accept the form; the encoder still produces a silently wrong word for Operand::Shift { kind: "msl" }.
**Root cause:** neon.rs:678-687 only special-cases kind == "lsl"; any other Shift kind falls through to cmode=0000.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:678`
```rust
                    if kind == "lsl" {
                        match amount {
                            0 => (0b0000u32, 0),
```
**Suggested fix:** Handle MSL #8/#16 as cmode 1100/1101 (as encode_neon_mvni does) and reject unknown shift kinds.
```rust
                    } else if kind == "msl" {
                        match amount {
                            8 => (0b1100u32, 8),
                            16 => (0b1101, 16),
                            _ => return Err(format!("movi: unsupported MSL shift: {}", amount)),
                        }
                    } else {
                        return Err(format!("movi: unsupported shift kind: {}", kind));
                    }
```
**Bug report:** bug_reports/encode_neon_movi_s_msl.md
**Repro seed:** rd = 0, t = "2s", imm8 = 0, n = 8
**Raw output:**
```text
assertion `left == right` failed: movi v0.2s, #0, msl #8 must match llvm-mc (cmode=1100, 0x0f00c400)
  left: 251659264
 right: 251708416
```

### B3: encode_neon_movi ignores surplus operands

**Formal:** ∀ valid 2-operand movi plus (extra RegArrangement OR illegal shift for T). llvm-mc rejects the asm ∧ encode_neon_movi(ops) = Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembler; llvm-mc/gas reject a third non-shift operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_movi([v0.8b, #0, v0.8b])
**Expected / Actual:** Err / Ok encoded as `movi v0.8b, #0`
**Impact:** A typo or extra token silently produces a valid instruction instead of an assembler error.
**Root cause:** neon.rs:625 `if operands.len() < 2` only rejects too few operands; extras are never rejected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:625`
```rust
    if operands.len() < 2 {
        return Err("movi requires 2 operands".to_string());
    }
```
**Suggested fix:** Require exactly 2 operands, or 3 when the third is a legal Shift for that arrangement.
```rust
    if operands.len() != 2 && !(operands.len() == 3 && matches!(operands.get(2), Some(Operand::Shift { .. }))) {
        return Err("movi requires 2 operands (optional shift)".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_movi_extra_operand.md
**Repro seed:** rd = 0, extra = 0, t = "8b", imm8 = 0, kind = 0
**Raw output:**
```text
movi v0.8b, #0, v0.8b must Err (gas/llvm-mc reject a third non-shift operand)
```

### B4: encode_neon_movi truncates out-of-range 8-bit immediates

**Formal:** ∀ rd ∈ [0,31], T ∈ {8b,16b,4h,8h,2s,4s}, imm ∉ [0,255]. llvm-mc rejects ∧ encode_neon_movi([Vd.T, #imm]) = Err
**Contract evidence:** inferred (llvm-mc: immediate must be an integer in range [0, 255]; gas: out of range -128 to 255)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_movi([v0.8b, #256])
**Expected / Actual:** Err / Ok(Word(0x0f00e400)) same as `#0`
**Impact:** A mistyped immediate silently wraps (`imm as u32 & 0xFF`), so the object file contains a different constant than the source.
**Root cause:** neon.rs:637 (and the 2s/4s and 4h/8h copies) mask with `imm as u32 & 0xFF` instead of requiring 0..=255.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:637`
```rust
            let imm8 = imm as u32 & 0xFF;
```
**Suggested fix:** Reject immediates outside 0..=255 for 8-bit MOVI forms.
```rust
            if !(0..=255).contains(&imm) {
                return Err(format!("movi: immediate {} out of range [0, 255]", imm));
            }
            let imm8 = imm as u32;
```
**Bug report:** bug_reports/encode_neon_movi_imm_oor.md
**Repro seed:** rd = 0, kind = 0, t_ok = "8b", over = 1
**Raw output:**
```text
movi v0.8b, #256 must Err (llvm-mc: immediate must be in range [0, 255])
```

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_movi_pbt.rs | 9 properties + 1 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one additive `#[cfg(test)] mod encode_neon_movi_pbt;` |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_movi -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_h_lsl8 -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_s_msl -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_extra_operand -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_movi_regression_imm_oor -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md, pbt-out/REPORT.html
- pbt-out/PROPERTIES.md, pbt-out/PLAN.md
- pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_neon_movi_h_lsl8.md (+ .html)
- pbt-out/bug_reports/encode_neon_movi_s_msl.md (+ .html)
- pbt-out/bug_reports/encode_neon_movi_extra_operand.md (+ .html)
- pbt-out/bug_reports/encode_neon_movi_imm_oor.md (+ .html)
- pbt-out/run/kat.log, movi.log, regression.log, sweep.log
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_movi_pbt.txt (proptest shrinking corpus, sibling convention)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 10:53 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 110/289 total | PBT candidates: 110 | Tested: 110 (100%) | 0 pass, 110 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 110 |
| **Tested (of PBT candidates)** | **110 / 110 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 110 / 0 |
| **Overall (tested / all functions)** | **110 / 289 (38%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 110 | 110 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 110 | 110 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 11 | 11 | 100% | covered |
| neon.rs | 68 | 25 | 25 | 100% | covered |
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
