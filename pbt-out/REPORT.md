# PBT Campaign Report: encode_neon_ld1r

## Summary

**Verdict:** 1 high, 2 medium, 1 low: encode_neon_ld1r drops register post-index to a no-writeback load, ignores surplus operands, accepts W/XZR/FP bases, and encodes illegal post-index immediates as the legal form.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_ld1r
**Tests:** 13
**Result:** 9 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test, C++ reporter looking at unrelated binaries listed encode_neon_ld1r NOT LINKED). cargo test --lib encode_neon_ld1r executed the symbol (KAT + 13 properties). Sweep: manual arm audit of documented LD1R forms (register post-index, illegal #imm, alt spellings, invalid names, [Xn,#imm]).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_ld1r | 13 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_neon_ld1r ignores a surplus operand

**Formal:** ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30, extra ∈ {Cond, Shift, RegArrangement, Label}. encode_neon_ld1r([RegList, Mem, extra]) is Err.
**Contract evidence:** inferred (README.md:12 gas-compatible assembler; llvm-mc rejects a surplus operand after ld1r)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld1r([RegList({v0.8b}), Mem{x0, 0}, Cond("eq")])
**Expected / Actual:** Err / Ok(Word(0x0d40c000))
**Impact:** Trailing junk is dropped and a no-offset LD1R is emitted, so a mistyped extra operand assembles without a diagnostic.
**Root cause:** neon.rs:834 checks `operands.len() < 2` only, so a third Cond/Shift/Label is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:834`
```rust
    if operands.len() < 2 {
        return Err("ld1r requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject a surplus operand that is not a register post-index Xm.
```rust
    if operands.len() > 2 && !matches!(operands.get(2), Some(Operand::Reg(_))) {
        return Err("ld1r: unexpected extra operand".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_ld1r_extra_operand.md
**Repro seed:** t="8b", rt=0, rn=0, extra_kind=0
**Raw output:**
```text
Test failed: ld1r extra operand must Err (llvm-mc/gas reject a surplus operand)
minimal failing input: t = "8b", rt = 0, rn = 0, extra_kind = 0
```

### B2: encode_neon_ld1r accepts W, XZR, x31, and FP bases

**Formal:** ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, base ∈ {w0,wzr,wsp,xzr,x31,d0,s0,v0,q0}. encode_neon_ld1r([RegList, Mem{base}]) is Err.
**Contract evidence:** inferred (llvm-mc requires Xn|SP; README.md:12 gas-compatible)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld1r([RegList({v0.8b}), Mem{base:"w0", offset:0}])
**Expected / Actual:** Err / Ok(Word(0x0d40c000))
**Impact:** `[w0]` encodes as `[x0]`; `[xzr]`/`[x31]` encode as `[sp]`; `[d0]` encodes as `[x0]`. A mistyped base silently loads from the wrong register.
**Root cause:** neon.rs:868 calls parse_reg_num on the base with no Xn|SP check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:868`
```rust
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
```
**Suggested fix:** Accept only `sp` or `x0`–`x30`.
```rust
            let rn = parse_ld1r_base(base)?;
```
**Bug report:** bug_reports/encode_neon_ld1r_invalid_base.md
**Repro seed:** t="8b", rt=0, base="w0"
**Raw output:**
```text
Test failed: ld1r base [w0] must Err (llvm-mc requires Xn|SP)
minimal failing input: t = "8b", rt = 0, base = "w0"
```

### B3: encode_neon_ld1r encodes register post-index as no-offset

**Formal:** ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt ∈ 0..31, rn ∈ 0..30, rm ∈ 0..30. encode_neon_ld1r([RegList({Vt.T}), Mem{Xn}, Reg(Xm)]) = llvm-mc("ld1r {Vt.T}, [Xn], Xm").
**Contract evidence:** documented README.md:235 "ld1r/ld2r/ld3r/ld4r (with post-index)"
**Documentation conflict:** README.md:235 lists ld1r with post-index as supported; the function comment neon.rs:833 shows only `[Xn]`. The README is the assembler contract. The comment does not declare register post-index invalid.
**Severity:** high
**Counterexample:** encode_neon_ld1r([RegList({v0.8b}), Mem{x0, 0}, Reg("x0")]) then compared to llvm-mc `ld1r {v0.8b}, [x0], x0`
**Expected / Actual:** 0x0dc0c000 / 0x0d40c000
**Impact:** Valid register post-index is assembled as no-writeback LD1R, so the base is not updated.
**Root cause:** neon.rs:866 matches only operands[1] as Mem/MemPostIndex and never reads a trailing GPR.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:866`
```rust
        Operand::Mem { base, offset: 0 } => {
            let rn = parse_reg_num(base).ok_or("invalid base reg")?;
            // LD1R: 0 Q 0 01101 0 1 0 00000 110 0 size Rn Rt (no post-index)
            let word = (q << 30) | (0b001101 << 24) | (1 << 22) | (0b110 << 13)
                | (size << 10) | (rn << 5) | rt;
            Ok(EncodeResult::Word(word))
        }
```
**Suggested fix:** If operands[2] is Xm, set L=1 and Rm=Xm.
```rust
            if let Some(Operand::Reg(rm_name)) = operands.get(2) {
                let rm = parse_reg_num(rm_name).ok_or("invalid post-index reg")?;
                let word = (q << 30) | (0b001101 << 24) | (1 << 23) | (1 << 22)
                    | (rm << 16) | (0b110 << 13) | (size << 10) | (rn << 5) | rt;
                return Ok(EncodeResult::Word(word));
            }
```
**Bug report:** bug_reports/encode_neon_ld1r_reg_post.md
**Repro seed:** t="8b", rt=0, rn=0, rm=0
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `222347264`,
 right: `230735872`: mismatch for ld1r {v0.8b}, [x0], x0
minimal failing input: t = "8b", rt = 0, rn = 0, rm = 0
```

### B4: encode_neon_ld1r accepts illegal post-index immediates

**Formal:** ∀ T ∈ {8b,16b,4h,8h,2s,4s,1d,2d}, rt, rn, imm ∈ {-1,0,3,5,7,64,256} \ {esize(T)}. encode_neon_ld1r([RegList, MemPostIndex{imm}]) is Err.
**Contract evidence:** documented limitation neon.rs:876 "offset must match element size, not encoded separately"
**Documentation conflict:** neon.rs:876 states the offset must match element size, then discards it. That admits a gap on an input the API accepts (MemPostIndex with any i64), not an input-domain exclusion. Severity stepped down.
**Severity:** low (documented by the author)
**Counterexample:** encode_neon_ld1r([RegList({v0.8b}), MemPostIndex{x0, -1}])
**Expected / Actual:** Err / Ok(Word(0x0ddfc000))
**Impact:** Any illegal #imm encodes as the legal post-index-by-esize form, so the written immediate and the machine-code increment disagree.
**Root cause:** neon.rs:876 `let _ = offset` always encodes Rm=11111.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:876`
```rust
            let _ = offset; // offset must match element size, not encoded separately
```
**Suggested fix:** Compare offset to esize(T) and reject a mismatch.
```rust
            let esize = match size { 0b00 => 1i64, 0b01 => 2, 0b10 => 4, _ => 8 };
            if *offset != esize {
                return Err(format!("ld1r: post-index #{} must be #{}", offset, esize));
            }
```
**Bug report:** bug_reports/encode_neon_ld1r_bad_post_imm.md
**Repro seed:** t="8b", rt=0, rn=0, imm=-1
**Raw output:**
```text
Test failed: ld1r post-index #-1 (legal #1) must Err
minimal failing input: t = "8b", rt = 0, rn = 0, imm = -1
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_ld1r_pbt.rs | 13 properties + 1 KAT + 6 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_ld1r -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_extra_operand -- --test-threads=1
```

B2 invalid base:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_w_base -- --test-threads=1
```

B3 register post-index:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_reg_post -- --test-threads=1
```

B4 illegal post-index immediate:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld1r_regression_bad_post_imm -- --test-threads=1
```

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/bug_reports/encode_neon_ld1r_extra_operand.md
- pbt-out/bug_reports/encode_neon_ld1r_extra_operand.html
- pbt-out/bug_reports/encode_neon_ld1r_invalid_base.md
- pbt-out/bug_reports/encode_neon_ld1r_invalid_base.html
- pbt-out/bug_reports/encode_neon_ld1r_reg_post.md
- pbt-out/bug_reports/encode_neon_ld1r_reg_post.html
- pbt-out/bug_reports/encode_neon_ld1r_bad_post_imm.md
- pbt-out/bug_reports/encode_neon_ld1r_bad_post_imm.html
- pbt-out/run/encode_neon_ld1r_round1.log
- pbt-out/run/encode_neon_ld1r_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 08:09 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 103/289 total | PBT candidates: 103 | Tested: 103 (100%) | 0 pass, 103 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 103 |
| **Tested (of PBT candidates)** | **103 / 103 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 103 / 0 |
| **Overall (tested / all functions)** | **103 / 289 (36%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 103 | 103 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 103 | 103 | 0 | 100% |

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
| neon.rs | 68 | 18 | 18 | 100% | covered |
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
