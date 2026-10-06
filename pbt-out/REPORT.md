# PBT Campaign Report: encode_tbz

## Summary

**Verdict:** 5 medium: encode_tbz rejects gas-legal immediate PC offsets, ignores extra operands, and silently accepts SP, FP/SIMD Rt, and out-of-range bit numbers, so GNU-style TBZ/TBNZ that llvm-mc accepts is either refused or encoded as the wrong instruction.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_tbz
**Tests:** 12
**Result:** 7 passing, 5 bugs
**Change surface:** 1 changed function (encode_tbz), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test, C++ reporter listed unrelated binaries and claimed encode_tbz NOT LINKED). Sweep was a manual arm audit of the 16-line body.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_tbz | 12 | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_tbz rejects immediate PC-offset form

**Formal:** ∀ rt ∈ GPR_W ∪ GPR_X (n∈0..31 including ZR/LR), bit ∈ 0..31 (W) / 0..63 (X), imm ∈ {k·4 | k∈ℤ, −32768 ≤ k·4 ≤ 32764}, is_nz ∈ {false,true}. encode_tbz([Reg(rt), Imm(bit), Imm(imm)], is_nz) = Word(llvm-mc("tbz/tbnz rt, #bit, #imm"))
**Contract evidence:** inferred (README.md:12 gas-compat; ARM ARM Test and branch (immediate) encodes imm14; llvm-mc accepts `tbz x0, #0, #0`)
**Documentation conflict:** (none) — compare_branch.rs:261 states the encoding layout including imm14 but does not declare #imm invalid. README.md:458 describes deferred symbol relocations, not an exclusion of immediate offsets.
**Severity:** medium
**Counterexample:** encode_tbz([Reg("x0"), Imm(0), Imm(-32768)], false) (`tbz x0, #0, #-32768`)
**Expected / Actual:** Word(0x36040000) matching llvm-mc / Err("expected symbol at operand 2, got Some(Imm(-32768))")
**Impact:** Hand-written `.s` files that use an explicit PC offset fail to assemble; compiler output currently uses labels so the hole is latent for codegen.
**Root cause:** compare_branch.rs:257 calls get_symbol for the branch target and never matches Operand::Imm, so imm14 is never packed.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:257`
```rust
    let (sym, addend) = get_symbol(operands, 2)?;
```
**Suggested fix:** When operand 2 is Imm, range-check a 4-byte-aligned offset in [-32768, 32764], pack imm14, and return Word.
```rust
    if let Some(Operand::Imm(imm)) = operands.get(2) {
        if *imm % 4 != 0 || *imm < -32768 || *imm > 32764 {
            return Err(format!("tbz offset out of range: {}", imm));
        }
        let imm14 = ((*imm as i32) >> 2) as u32 & 0x3fff;
        let word = (b5 << 31) | (0b011011 << 25) | (op << 24) | (b40 << 19) | (imm14 << 5) | rt;
        return Ok(EncodeResult::Word(word));
    }
    let (sym, addend) = get_symbol(operands, 2)?;
```
**Bug report:** bug_reports/encode_tbz_imm_offset.md
**Repro seed:** cc f6e95b6455706c1bda3d8eb1af12fddb3a8ce6b18bdc43c82d8a14c4cd42c27c
**Raw output:**
```text
Test failed: SUT rejected valid tbz x0, #0, #-32768: Err("expected symbol at operand 2, got Some(Imm(-32768))").
minimal failing input: (rt, bit) = ("x0", 0), is_nz = false, imm = -32768
```

### B2: encode_tbz ignores extra operands

**Formal:** ∀ n ∈ 0..30, bit ∈ 0..63, is_nz ∈ {false,true}, extra ∈ Operand. encode_tbz([Reg(xn), Imm(bit), Symbol(s), extra], is_nz) is Err
**Contract evidence:** inferred (llvm-mc "invalid operand for instruction" on a fourth operand; gas-compat README.md:12)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tbz([Reg("x0"), Imm(0), Symbol("labl0"), Reg("x1")], false)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x36000000, reloc: TstBr14 symbol=labl0 addend=0 })
**Impact:** Invalid GNU-style assembly silently encodes as a three-operand TBZ.
**Root cause:** compare_branch.rs:254-257 reads only operands 0..2 and has no operands.len() upper bound.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:257`
```rust
    let (sym, addend) = get_symbol(operands, 2)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 3 {
        return Err(format!("tbz: expected 3 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_tbz_extra_operand.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: tbz x0, #0, label, extra (which=0) must Err (llvm-mc: invalid operand)
minimal failing input: n = 0, bit = 0, is_nz = false, suffix = 0, which = 0
```

### B3: encode_tbz accepts SP/WSP as Rt and encodes it as ZR

**Formal:** ∀ name ∈ {sp,wsp}, is_nz ∈ {false,true}. encode_tbz([Reg(name), Imm(0), Symbol("L")], is_nz) is Err
**Contract evidence:** inferred (llvm-mc "invalid operand for instruction" on sp; ARM ARM Rt is a GPR not SP)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tbz([Reg("sp"), Imm(0), Symbol("L")], false)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x3600001f, reloc: TstBr14 symbol=L addend=0 })
**Impact:** A stack-pointer test-and-branch is silently retargeted at ZR.
**Root cause:** compare_branch.rs:255 calls get_reg, which maps SP and XZR both to 31; encode_tbz never distinguishes them.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:255`
```rust
    let (rt, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject SP/WSP as Rt.
```rust
    let (rt, _) = get_reg(operands, 0)?;
    if let Some(Operand::Reg(name)) = operands.get(0) {
        let l = name.to_ascii_lowercase();
        if l == "sp" || l == "wsp" {
            return Err(format!("tbz: SP is not a valid Rt: {}", name));
        }
    }
```
**Bug report:** bug_reports/encode_tbz_sp_as_zr.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: tbz sp , #0, L must Err (llvm-mc rejects SP/WSP)
minimal failing input: which = 0, is_nz = false
```

### B4: encode_tbz accepts FP/SIMD registers as Rt

**Formal:** ∀ prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31, is_nz ∈ {false,true}. encode_tbz([Reg(prefix+n), Imm(0), Symbol("L")], is_nz) is Err
**Contract evidence:** inferred (llvm-mc "invalid operand for instruction" on d0; ARM ARM Rt is a GPR)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tbz([Reg("d0"), Imm(0), Symbol("L")], false)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0x36000000, reloc: TstBr14 symbol=L addend=0 })
**Impact:** A floating-point test-and-branch is silently retargeted at the same-numbered GPR.
**Root cause:** compare_branch.rs:255 calls get_reg, which accepts prefixes d/s/q/v/h/b; encode_tbz never checks is_fp_reg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:255`
```rust
    let (rt, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject FP/SIMD Rt.
```rust
    if let Some(Operand::Reg(name)) = operands.get(0) {
        if is_fp_reg(name) {
            return Err(format!("tbz: FP/SIMD register not valid Rt: {}", name));
        }
    }
```
**Bug report:** bug_reports/encode_tbz_fp_reg.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: tbz d0 , #0, L must Err (llvm-mc rejects FP/SIMD Rt)
minimal failing input: which = 2, n = 0, is_nz = false
```

### B5: encode_tbz masks out-of-range bit numbers instead of rejecting them

**Formal:** ∀ rt ∈ GPR_W ∪ GPR_X, bit ∉ valid range (W: [0,31], X: [0,63]), is_nz ∈ {false,true}. encode_tbz([Reg(rt), Imm(bit), Symbol("L")], is_nz) is Err
**Contract evidence:** inferred (llvm-mc "immediate must be an integer in range [0, 31]" / "[0, 63]"; ARM ARM bit = b5:b40)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tbz([Reg("w0"), Imm(-1), Symbol("L")], false)
**Expected / Actual:** Err / Ok(WordWithReloc { word: 0xb6f80000, reloc: TstBr14 symbol=L addend=0 })
**Impact:** `tbz w0, #-1, L` encodes as `tbz x0, #63, L`; `tbz w0, #32, L` encodes as `tbz x0, #32, L`. A 32-bit test-and-branch can silently become a 64-bit one.
**Root cause:** compare_branch.rs:258-259 casts bit to u32 and masks to 6 bits with no range check; get_reg width is discarded.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/compare_branch.rs:258`
```rust
    let b5 = ((bit as u32) >> 5) & 1;
```
**Suggested fix:** Reject out-of-range bits using the register width.
```rust
    let (rt, is_64) = get_reg(operands, 0)?;
    let bit = get_imm(operands, 1)?;
    let max_bit = if is_64 { 63 } else { 31 };
    if bit < 0 || bit > max_bit {
        return Err(format!("tbz bit {} out of range 0..{}", bit, max_bit));
    }
```
**Bug report:** bug_reports/encode_tbz_bit_oor.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
Test failed: tbz w0, #-1, L is out of bit range and must Err
minimal failing input: n = 0, is_64 = false, is_nz = false, which = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_tbz_pbt.rs | 12 properties + 4 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `mod encode_tbz_pbt` registration |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_tbz -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_imm_offset -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_extra_operand -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_sp_as_zr -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_fp_reg -- --test-threads=1
```

B5:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tbz_regression_bit_oor -- --test-threads=1
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
- pbt-out/run/encode_tbz_round1.log
- pbt-out/run/encode_tbz_round2.log
- pbt-out/bug_reports/encode_tbz_imm_offset.md
- pbt-out/bug_reports/encode_tbz_imm_offset.html
- pbt-out/bug_reports/encode_tbz_extra_operand.md
- pbt-out/bug_reports/encode_tbz_extra_operand.html
- pbt-out/bug_reports/encode_tbz_sp_as_zr.md
- pbt-out/bug_reports/encode_tbz_sp_as_zr.html
- pbt-out/bug_reports/encode_tbz_fp_reg.md
- pbt-out/bug_reports/encode_tbz_fp_reg.html
- pbt-out/bug_reports/encode_tbz_bit_oor.md
- pbt-out/bug_reports/encode_tbz_bit_oor.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 06:34 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 168/307 total | PBT candidates: 168 | Tested: 168 (100%) | 1 pass, 168 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 168 |
| **Tested (of PBT candidates)** | **168 / 168 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 168 / -1 |
| **Overall (tested / all functions)** | **168 / 307 (55%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 168 | 168 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 168 | 168 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 14 | 14 | 100% | covered |
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
