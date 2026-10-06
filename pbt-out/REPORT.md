# PBT Campaign Report: encode_ldr_str

## Summary

**Verdict:** 8 bugs (worst medium): encode_ldr_str accepts SP as Rt (and classifies it as SIMD), treats XZR/W bases as Xn/SP, invents UXTW for a bare W index, encodes UNPREDICTABLE writeback when Rt==Rn, truncates out-of-range offsets, ignores extra operands, and mismatches llvm-mc on byte `lsl #0` (S bit).
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_ldr_str
**Tests:** 11 properties (8 passing, 3 failing) plus 5 passing KAT and 8 failing regression witnesses
**Result:** 8 passing, 8 bugs
**Change surface:** 1 changed function (encode_ldr_str), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). cargo test --lib encode_ldr_str executed the real symbol (5 KAT + 11 properties + 8 regressions).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_ldr_str | 11 properties + 5 KAT + 8 regressions | 8 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: byte LSL #0 encodes S=0 instead of S=1

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn,rm ∈ 0..31, option ∈ valid(size), s ∈ {0,1}. encode_ldr_str([Reg(gp_rt), MemRegOffset{Xn|SP, Rm, option, shift}], is_load, size, false, false) = llvm_mc(mnemonic Rt, [Xn|SP, Rm{, option #shift}]) as Word
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc encoding of `strb w0, [x0, x0, lsl #0]`)
**Documentation conflict:** (none)
**Severity:** low
**Counterexample:** `encode_ldr_str([Reg("w0"), MemRegOffset{base:"x0", index:"x0", extend:Some("lsl"), shift:Some(0)}], false, 0, false, false)`
**Expected / Actual:** 0x38207800 / 0x38206800
**Impact:** Object bytes diverge from gas/llvm-mc on byte-sized register-offset with explicit `lsl #0`.
**Root cause:** load_store.rs:179 sets S=1 only when shift_amount > 0, so Some(0) on a byte access is S=0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:179`
```rust
                    let s_val = if shift_amount > 0 { 1u32 } else { 0u32 };
                    (0b011u32, s_val)
```
**Suggested fix:** Set S=1 when a shift amount is present, including `#0` on byte accesses.
```rust
                    let s_val = if shift.is_some() { 1u32 } else { 0u32 };
                    (0b011u32, s_val)
```
**Bug report:** bug_reports/encode_ldr_str_byte_lsl0.md
**Repro seed:** cc 6adb6a7e2266c663403ef87f3d1aac2759f4b5eb73f1423dfbd67afa5555034c
**Raw output:**
```text
left: 941647872, right: 941651968: regoff mismatch for strb w0, [x0, x0, lsl #0]
minimal failing input: is_load = false, size = 0, rt = 0, rn = 0, rm = 0, w_index = false, ext_sel = 0, s_bit = 1
```

### B2: SP dest accepted and classified as SIMD

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31. encode_ldr_str with Rt=SP is Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects `strb sp, [x0]`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `encode_ldr_str([Reg("sp"), Mem{base:"x0", offset:0}], false, 0, false, false)`
**Expected / Actual:** Err / Ok(Word(0x3D00001F))
**Impact:** SP as Rt is assembled as a SIMD store because `is_fp_reg("sp")` is true.
**Root cause:** load_store.rs:39 calls is_fp_reg on Rt; names starting with `s` are SIMD; parse_reg_num("sp")=31; no SP-as-Rt check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:39`
```rust
    let fp = is_fp_reg(operands.first().map(|o| match o { Operand::Reg(r) => r.as_str(), _ => "" }).unwrap_or(""));
```
**Suggested fix:** Reject SP/WSP as Rt before setting V.
```rust
    if rt_name.eq_ignore_ascii_case("sp") || rt_name.eq_ignore_ascii_case("wsp") {
        return Err("ldr/str: SP is not a valid Rt".to_string());
    }
```
**Bug report:** bug_reports/encode_ldr_str_sp_dest.md
**Repro seed:** (none — deterministic)
**Raw output:**
```text
SP dest must Err (llvm-mc: invalid operand); got Ok(Word(1023410207))
minimal failing input: is_load = false, size = 0, rt = 0, rn = 0
```

### B3: XZR/x31 base encoded as SP

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31. encode_ldr_str with base in {XZR, x31} is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `ldr x0, [xzr]`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `encode_ldr_str([Reg("x0"), Mem{base:"xzr", offset:0}], true, 0b11, false, false)`
**Expected / Actual:** Err / Ok(Word(0xF94003E0)) (`ldr x0, [sp]`)
**Impact:** XZR as base becomes a stack-pointer load/store.
**Root cause:** parse_reg_num maps xzr/x31 and sp all to 31 with no base-name check at load_store.rs:50.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:50`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Reject XZR/WZR/x31/w31 as a memory base.
```rust
            if base.eq_ignore_ascii_case("xzr") || base.eq_ignore_ascii_case("x31") {
                return Err("ldr/str: base must be Xn or SP, not XZR".to_string());
            }
```
**Bug report:** bug_reports/encode_ldr_str_xzr_base.md
**Repro seed:** (none — deterministic)
**Raw output:**
```text
LDR X0, [XZR] must Err; Rn=31 is SP not XZR (llvm-mc rejects it)
```

### B4: W-register base accepted as Xn

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31. encode_ldr_str with W-prefixed base is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `ldr x0, [w0]`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `encode_ldr_str([Reg("x0"), Mem{base:"w0", offset:0}], true, 0b11, false, false)`
**Expected / Actual:** Err / Ok(Word(0xF9400000)) (`ldr x0, [x0]`)
**Impact:** A 32-bit base is encoded as 64-bit.
**Root cause:** parse_reg_num("w0")=0 with no width check at load_store.rs:50.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:50`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Require Xn/SP/LR as the base name.
```rust
            let b = base.to_ascii_lowercase();
            if !(b.starts_with('x') || b == "sp" || b == "lr") {
                return Err("ldr/str: base must be Xn or SP".to_string());
            }
```
**Bug report:** bug_reports/encode_ldr_str_w_base.md
**Repro seed:** (none — deterministic)
**Raw output:**
```text
LDR X0, [W0] must Err; base must be Xn|SP (llvm-mc rejects it)
```

### B5: W index without extend defaults to UXTW

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31. encode_ldr_str with W-index and extend=None is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `ldr x0, [x1, w2]`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `encode_ldr_str([Reg("x0"), MemRegOffset{base:"x1", index:"w2", extend:None, shift:None}], true, 0b11, false, false)`
**Expected / Actual:** Err / Ok(Word(0xF8624820)) (`ldr x0, [x1, w2, uxtw]`)
**Impact:** Missing extend is invented rather than rejected.
**Root cause:** load_store.rs:191-192 defaults W index + extend=None to UXTW.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:191`
```rust
                    if is_w_index {
                        (0b010u32, 0u32) // UXTW, no shift
```
**Suggested fix:** Reject a W index when no extend is specified.
```rust
                    if is_w_index {
                        return Err("ldr/str: W index requires uxtw or sxtw".to_string());
                    }
```
**Bug report:** bug_reports/encode_ldr_str_w_index.md
**Repro seed:** (none — deterministic)
**Raw output:**
```text
LDR X0, [X1, W2] must Err; W index requires uxtw/sxtw (llvm-mc rejects it)
```

### B6: writeback with Rt==Rn is encoded

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt ∈ 0..30. encode_ldr_str pre/post with Rt==Rn is Err
**Contract evidence:** inferred (README.md:12; ARM UNPREDICTABLE; llvm-mc rejects `ldr x0, [x0, #8]!`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `encode_ldr_str([Reg("x0"), MemPreIndex{base:"x0", offset:8}], true, 0b11, false, false)`
**Expected / Actual:** Err / Ok(Word(0xF8408C00))
**Impact:** An UNPREDICTABLE instruction is assembled.
**Root cause:** Pre/post paths at load_store.rs:96-104 encode without an Rt==Rn check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:96`
```rust
        Some(Operand::MemPreIndex { base, offset }) => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            let imm9 = (*offset as i32) & 0x1FF;
```
**Suggested fix:** Reject writeback when Rt equals Rn and Rn is not SP.
```rust
            if rt == rn && rn != 31 {
                return Err("ldr/str: writeback with Rt==Rn is unpredictable".to_string());
            }
```
**Bug report:** bug_reports/encode_ldr_str_writeback_overlap.md
**Repro seed:** (none — deterministic)
**Raw output:**
```text
LDR X0, [X0, #8]! must Err; writeback Rt==Rn is unpredictable (llvm-mc rejects it)
```

### B7: out-of-range offset truncated to imm9

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31, off ∉ valid_pimm ∪ [-256,255]. encode_ldr_str([Rt, Mem{base,off}], ...) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects `strb w0, [x0, #-257]`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `encode_ldr_str([Reg("w0"), Mem{base:"x0", offset:-257}], false, 0, false, false)`
**Expected / Actual:** Err / Ok(Word(0x380FF000))
**Impact:** Out-of-range offsets wrap into a different in-range unscaled store/load.
**Root cause:** load_store.rs:81 masks imm9 with 0x1FF with no range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:81`
```rust
            let imm9 = (*offset as i32) & 0x1FF;
```
**Suggested fix:** Reject offsets outside [-256, 255] for unscaled/pre/post.
```rust
            if *offset < -256 || *offset > 255 {
                return Err("ldr/str: offset out of range".to_string());
            }
```
**Bug report:** bug_reports/encode_ldr_str_offset_range.md
**Repro seed:** (none — deterministic)
**Raw output:**
```text
out-of-range Mem offset -257 must Err (llvm-mc range); got Ok(Word(940568576))
minimal failing input: is_load = false, size = 0, rt = 0, rn = 0, extra = Reg("x0"), prepost = 0, which_off = 0
```

### B8: extra operand ignored

**Formal:** ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31, extra ∈ Operand. encode_ldr_str([Rt, Mem{base,0}, extra], ...) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects a 3rd operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `encode_ldr_str([Reg("w0"), Mem{base:"x0", offset:0}, Reg("x0")], false, 0, false, false)`
**Expected / Actual:** Err / Ok(Word(0x39000000)) (`strb w0, [x0]`)
**Impact:** Malformed three-operand LDR/STR is accepted.
**Root cause:** load_store.rs:34 only checks `operands.len() < 2`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/load_store.rs:34`
```rust
    if operands.len() < 2 {
        return Err("ldr/str requires at least 2 operands".to_string());
    }
```
**Suggested fix:** Require exactly two operands.
```rust
    if operands.len() != 2 {
        return Err("ldr/str requires exactly 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_ldr_str_extra_operand.md
**Repro seed:** (none — deterministic)
**Raw output:**
```text
STRB W0, [X0], X0 must Err; extra operand is invalid (llvm-mc rejects it)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs | 11 properties + 5 KAT + 8 regressions |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_ldr_str_pbt` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_ldr_str -- --test-threads=1
```

Per-bug:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_ldr_str_regression_byte_lsl0 -- --test-threads=1
cargo test --lib test_encode_ldr_str_regression_sp_dest -- --test-threads=1
cargo test --lib test_encode_ldr_str_regression_xzr_base -- --test-threads=1
cargo test --lib test_encode_ldr_str_regression_w_base -- --test-threads=1
cargo test --lib test_encode_ldr_str_regression_w_index -- --test-threads=1
cargo test --lib test_encode_ldr_str_regression_writeback_overlap -- --test-threads=1
cargo test --lib test_encode_ldr_str_regression_imm9_range -- --test-threads=1
cargo test --lib test_encode_ldr_str_regression_extra_operand -- --test-threads=1
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
- pbt-out/bug_reports/encode_ldr_str_byte_lsl0.md (+ .html)
- pbt-out/bug_reports/encode_ldr_str_sp_dest.md (+ .html)
- pbt-out/bug_reports/encode_ldr_str_xzr_base.md (+ .html)
- pbt-out/bug_reports/encode_ldr_str_w_base.md (+ .html)
- pbt-out/bug_reports/encode_ldr_str_w_index.md (+ .html)
- pbt-out/bug_reports/encode_ldr_str_writeback_overlap.md (+ .html)
- pbt-out/bug_reports/encode_ldr_str_offset_range.md (+ .html)
- pbt-out/bug_reports/encode_ldr_str_extra_operand.md (+ .html)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 09:19 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 175/307 total | PBT candidates: 175 | Tested: 175 (100%) | 1 pass, 175 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 175 |
| **Tested (of PBT candidates)** | **175 / 175 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 175 / -1 |
| **Overall (tested / all functions)** | **175 / 307 (57%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 175 | 175 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 175 | 175 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 20 | 20 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 30 | 30 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 15 | 15 | 100% | covered |
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
