# PBT Campaign Report: encode_ldnp_stnp

## Summary

**Verdict:** 4 medium: encode_ldnp_stnp silently encodes extra operands, SP dest / XZR-or-W base / mixed-width pairs, out-of-range offsets, and SIMD S/D/Q as integer V=0, so mistyped or FP pair loads/stores assemble to the wrong 32-bit word.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldnp_stnp
**Tests:** 9
**Result:** 5 passing, 4 bugs
**Change surface:** 1 changed function (encode_ldnp_stnp), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo tests executed encode_ldnp_stnp directly.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldnp_stnp | 9 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_ldnp_stnp ignores a fourth operand

**Formal:** ∀ is_load, is_64, rt1,rt2,rn ∈ 0..31, extra ∈ Operand. encode_ldnp_stnp([Reg(Rt1), Reg(Rt2), Mem{Xn|SP, 0}, extra], is_load) is Err
**Contract evidence:** inferred (llvm-mc/gas reject a fourth operand; signature is `&[Operand]` with no upper bound)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ldnp_stnp([Reg("w0"), Reg("w0"), Mem{base:"x0", offset:0}, Reg("x0")], is_load=false)
**Expected / Actual:** Err / Ok(Word(0x28000000))
**Impact:** A mistyped extra operand is dropped and a 32-bit pair store is still emitted
**Root cause:** load_store.rs:519 checks only `operands.len() < 3` and then reads operands[0..2]
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:519`
```rust
    if operands.len() < 3 {
        return Err("ldnp/stnp requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject `operands.len() != 3` before encoding
```rust
    if operands.len() != 3 {
        return Err("ldnp/stnp requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_ldnp_stnp_extra_operand.md
**Repro seed:** is_load=false, is_64=false, rt1=0, rt2=0, rn=0, extra=Reg("x0")
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects a fourth operand); got Ok(Word(671088640))
minimal failing input: is_load = false, is_64 = false, rt1 = 0, rt2 = 0, rn = 0, extra = Reg("x0")
```

### B2: encode_ldnp_stnp accepts invalid register forms that llvm-mc rejects

**Formal:** ∀ is_load, is_64, rt,rt2,rn ∈ 0..30. encode_ldnp_stnp of each of {SP as Rt1, SP as Rt2, XZR base, x31 base, W base, WSP base, mixed X/W pair} is Err
**Contract evidence:** inferred (ARM ARM C6 LDNP/STNP register classes; llvm-mc/gas reject SP dest, XZR/W/WSP/x31 base, mixed width)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ldnp_stnp([Reg("sp"), Reg("w0"), Mem{base:"x0", offset:0}], is_load=false) — also XZR/x31/W/WSP base, mixed X/W
**Expected / Actual:** Err / Ok(Word) for each form
**Impact:** SP dest encodes as XZR, XZR/x31 base encodes as SP, W/WSP base encodes as Xn, mixed X/W uses Rt1 width
**Root cause:** load_store.rs:523-524 get_reg / parse_reg_num maps SP and XZR both to 31, accepts W-prefixed bases, and discards Rt2 width
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:523`
```rust
    let (rt1, is_64) = get_reg(operands, 0)?;
    let (rt2, _) = get_reg(operands, 1)?;
```
**Suggested fix:** Reject SP as Rt, XZR/W/WSP/x31 as base, and mixed widths before packing the word
```rust
    if name_is_sp(operands[0]) || name_is_sp(operands[1]) {
        return Err("ldnp/stnp Rt cannot be SP".to_string());
    }
```
**Bug report:** bug_reports/encode_ldnp_stnp_invalid_regs.md
**Repro seed:** is_load=false, is_64=false, rt=0, rt2=0, rn=0
**Raw output:**
```text
Test failed: accepted invalid register forms (llvm-mc rejects): ["SP as Rt1", "SP as Rt2", "XZR base", "x31 base", "W base", "WSP base", "mixed X/W pair"]
minimal failing input: is_load = false, is_64 = false, rt = 0, rt2 = 0, rn = 0
```

### B3: encode_ldnp_stnp masks out-of-range and unaligned offsets instead of rejecting them

**Formal:** ∀ is_load, is_64, rt1,rt2,rn ∈ 0..31, off ∈ {min-1, max+1, 1, i64::MIN, i64::MAX}. encode_ldnp_stnp([Reg(Rt1), Reg(Rt2), Mem{Xn|SP, off}], is_load) is Err where min/max are the ARM range for that width
**Contract evidence:** inferred (ARM imm7 range; llvm-mc "index must be a multiple of 4/8 in range [-256,252]/[-512,504]")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_ldnp_stnp([Reg("w0"), Reg("w0"), Mem{base:"x0", offset:-257}], is_load=false)
**Expected / Actual:** Err / Ok(Word(0x281F0000))
**Impact:** An out-of-range or unaligned offset is shifted and masked into a different in-range imm7, so the assembled instruction accesses the wrong address
**Root cause:** load_store.rs:533 computes `imm7 = ((*offset >> shift) as i32) & 0x7F` with no range or alignment check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:533`
```rust
            let imm7 = ((*offset >> shift) as i32) & 0x7F;
```
**Suggested fix:** Require `offset` divisible by the scale and in the ARM range before encoding
```rust
            let scale = 1i64 << shift;
            if offset % scale != 0 || offset < -(64 * scale) || offset > (63 * scale) {
                return Err("ldnp/stnp offset out of range".to_string());
            }
```
**Bug report:** bug_reports/encode_ldnp_stnp_imm7_range.md
**Repro seed:** is_load=false, is_64=false, rt1=0, rt2=0, rn=0, which_off=0
**Raw output:**
```text
Test failed: out-of-range/unaligned offset -257 must Err (llvm-mc range); got Ok(Word(673153024))
minimal failing input: is_load = false, is_64 = false, rt1 = 0, rt2 = 0, rn = 0, which_off = 0
```

### B4: encode_ldnp_stnp encodes SIMD S/D/Q pairs as integer (V=0)

**Formal:** ∀ is_load ∈ Bool, kind ∈ {s,d,q}, rt1,rt2,rn ∈ 0..31, imm7 ∈ [-64,63]. encode_ldnp_stnp([Reg(kind+rt1), Reg(kind+rt2), Mem{Xn|SP, imm7·scale(kind)}], is_load) = llvm-mc("ldnp/stnp Sk/Dk/Qk, …")
**Contract evidence:** documented limitation load_store.rs:517 "TODO: Only handles integer registers (V=0). FP/SIMD register support needed for V=1."
**Documentation conflict:** load_store.rs:517 "TODO: Only handles integer registers (V=0). FP/SIMD register support needed for V=1." — admits a limitation on an input get_reg accepts (s/d/q); not an input-domain exclusion
**Severity:** medium (documented by the author)
**Counterexample:** encode_ldnp_stnp([Reg("s0"), Reg("s0"), Mem{base:"x0", offset:-256}], is_load=false)
**Expected / Actual:** llvm-mc 0x2C1F8000 (V=1) / Ok(Word(0x281F8000)) (V=0)
**Impact:** FP pair load/store is assembled as a 32-bit integer pair with the wrong opc/V/imm7
**Root cause:** load_store.rs:535 packs the word without V; is_fp_reg is never consulted so s/d/q go through as 32-bit integer
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:535`
```rust
            let word = (opc << 30) | (0b101 << 27) | (l << 22)
                | ((imm7 as u32 & 0x7F) << 15) | (rt2 << 10) | (rn << 5) | rt1;
```
**Suggested fix:** Detect FP/SIMD Rt (s/d/q), set V=1, opc and scale from the prefix, matching encode_ldp_stp
```rust
    let fp = is_fp_reg(rt1_name);
    let v = if fp { 1u32 } else { 0u32 };
    // opc/shift from s/d/q as in encode_ldp_stp; OR V into the word
```
**Bug report:** bug_reports/encode_ldnp_stnp_simd.md
**Repro seed:** is_load=false, kind=0, rt1=0, rt2=0, rn=0, imm7=-64
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `673185792`,
 right: `740294656`: SIMD mismatch for stnp s0, s0, [x0, #-256]
minimal failing input: is_load = false, kind = 0, rt1 = 0, rt2 = 0, rn = 0, imm7 = -64
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_ldnp_stnp_pbt.rs | 9 properties + 5 KAT + 9 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_ldnp_stnp -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_extra_operand -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_sp_dest -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_imm7_range -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldnp_stnp_regression_simd -- --test-threads=1
```

## Output Directories

pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/PROPERTIES.md, pbt-out/PLAN.md, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/bug_reports/encode_ldnp_stnp_extra_operand.md, pbt-out/bug_reports/encode_ldnp_stnp_extra_operand.html, pbt-out/bug_reports/encode_ldnp_stnp_invalid_regs.md, pbt-out/bug_reports/encode_ldnp_stnp_invalid_regs.html, pbt-out/bug_reports/encode_ldnp_stnp_imm7_range.md, pbt-out/bug_reports/encode_ldnp_stnp_imm7_range.html, pbt-out/bug_reports/encode_ldnp_stnp_simd.md, pbt-out/bug_reports/encode_ldnp_stnp_simd.html, pbt-out/run/encode_ldnp_stnp.log, pbt-out/run/encode_ldnp_stnp_regression.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 10:12 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 177/307 total | PBT candidates: 177 | Tested: 177 (100%) | 1 pass, 177 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 177 |
| **Tested (of PBT candidates)** | **177 / 177 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 177 / -1 |
| **Overall (tested / all functions)** | **177 / 307 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 177 | 177 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 177 | 177 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 30 | 30 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 17 | 17 | 100% | covered |
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
