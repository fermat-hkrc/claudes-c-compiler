# PBT Campaign Report: encode_neon_ld_st_single

## Summary

**Verdict:** 1 high, 5 medium, 1 low: encode_neon_ld_st_single silently drops register post-index (emits no-offset instead of Rm=Xm), ignores extra operands, accepts W/XZR/FP bases, wraps out-of-range lanes, ignores illegal post-index #imm, rejects uppercase V0.B, and accepts non-consecutive lists.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_ld_st_single
**Tests:** 14 properties (plus 1 KAT + 7 regression witnesses)
**Result:** 7 passing, 7 bugs
**Change surface:** 1 changed function (encode_neon_ld_st_single), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported NOT LINKED); cargo test --lib executed the symbol (KAT + 14 properties). Sweep round 1/1: manual arm audit added alt-spellings, index-oor, nonconsecutive, register post-index, illegal post-imm, and [Xn,#imm]. Closed: tier round spent.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_ld_st_single | 14 | 7 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: extra operand ignored

**Formal:** ∀ valid (n,sz,idx,rt,rn,load), extra ∈ {Cond, Shift, Label, RegArrangement}. encode_neon_ld_st_single([list, mem, extra], load, n) is Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; llvm-mc/gas reject a surplus operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_single([RegListIndexed({v0.b}[0]), Mem{x0,0}, Cond("eq")], false, 1)
**Expected / Actual:** Err / Ok(Word(0x0d000000))
**Impact:** A malformed line with a trailing junk operand is assembled as a valid st1 instead of being rejected.
**Root cause:** neon.rs:905 only rejects operands.len() < 2; extra non-Imm operands are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:905`
```rust
    if operands.len() < 2 {
        return Err(format!("ld/st{} single element requires at least 2 operands", num_structs));
    }
```
**Suggested fix:** Reject operands.len() > 2 unless operands[2] is a legal post-index Imm or Rm.
```rust
    if operands.len() > 2 {
        match &operands[2] {
            Operand::Imm(_) | Operand::Reg(_) => {}
            _ => return Err("unexpected extra operand".to_string()),
        }
    }
```
**Bug report:** bug_reports/encode_neon_ld_st_single_extra_operand.md
**Repro seed:** n=1, sz=b, idx=0, rt=0, rn=0, load=false, extra_kind=0
**Raw output:**
```text
st1 {v0.b}[0], [x0], eq must Err
```

### B2: W/XZR/x31/FP base accepted

**Formal:** ∀ base ∈ {w0,w31,wsp,xzr,x31,s0,d0,v0,q0}. encode_neon_ld_st_single([list, Mem(base)], load, n) is Err
**Contract evidence:** inferred (README.md:12; llvm-mc rejects W/XZR/x31/FP base)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_single([RegListIndexed({v0.b}[0]), Mem{base:"w0", offset:0}], false, 1)
**Expected / Actual:** Err / Ok(Word) encoded as [x0]
**Impact:** Invalid bases assemble as Xn or SP, producing the wrong addressing mode.
**Root cause:** neon.rs:931 calls parse_reg_num with no Xn|SP check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:931`
```rust
            let rn = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
```
**Suggested fix:** Require a 64-bit X register or SP, and reject xzr/x31.
```rust
            if !is_64bit_reg(base) || base.eq_ignore_ascii_case("xzr") || base.eq_ignore_ascii_case("x31") {
                return Err(format!("invalid base register: {}", base));
            }
```
**Bug report:** bug_reports/encode_neon_ld_st_single_invalid_base.md
**Repro seed:** n=1, sz=b, idx=0, rt=0, load=false, base=w0
**Raw output:**
```text
st1 {v0.b}[0], [w0] must Err
```

### B3: uppercase arrangement rejected

**Formal:** ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0,max(sz)], rt,rn ∈ 0..30, load ∈ {0,1}. encode_neon_ld_st_single([{Vrt.SZ..}[idx], [Xrn]], load, n) = llvm-mc(`ldN|stN {Vrt.SZ..}[idx], [Xrn]`)
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; llvm-mc accepts V0.B)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_single([RegListIndexed({V0.B}[0]), Mem{base:"X0", offset:0}], false, 1)
**Expected / Actual:** Ok(Word(0x0d000000)) / Err("unsupported element size for ld/st single: B")
**Impact:** Valid gas assembly with uppercase arrangement is rejected.
**Root cause:** neon.rs:960 matches elem_size only as lowercase "b"/"h"/"s"/"d".
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:960`
```rust
    let (opcode, s_bit, q_bit, size_field) = match elem_size.as_str() {
        "b" => {
```
**Suggested fix:** Lowercase the arrangement before the match.
```rust
    let elem_size = elem_size.to_ascii_lowercase();
```
**Bug report:** bug_reports/encode_neon_ld_st_single_alt_spellings.md
**Repro seed:** n=1, sz=b, idx=0, rt=0, rn=0, load=false
**Raw output:**
```text
st1 {V0.B}[0], [X0] must encode: "unsupported element size for ld/st single: B"
```

### B4: out-of-range lane index accepted

**Formal:** ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx > max_lane(sz). encode_neon_ld_st_single([{Vt.sz..}[idx], [Xn]], load, n) is Err
**Contract evidence:** inferred (llvm-mc "vector lane must be an integer in range")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_single([RegListIndexed({v0.b}[16]), Mem{x0,0}], false, 1)
**Expected / Actual:** Err / Ok(Word) with index bits masked to lane 0
**Impact:** A typo in the lane index silently stores/loads a different element.
**Root cause:** neon.rs:964-967 mask index bits with no range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:964`
```rust
            let q = (index >> 3) & 1;
            let s = (index >> 2) & 1;
            let sz = index & 3;
```
**Suggested fix:** Reject index above the documented lane maximum for the element size.
```rust
            if index > 15 {
                return Err(format!("vector lane must be in range [0, 15], got {}", index));
            }
```
**Bug report:** bug_reports/encode_neon_ld_st_single_index_oor.md
**Repro seed:** n=1, sz=b, rt=0, rn=0, load=false, over=1 (idx=16)
**Raw output:**
```text
st1 {v0.b}[16], [x0] must Err
```

### B5: non-consecutive register list accepted

**Formal:** ∀ n ∈ {2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0,max(sz)], list not consecutive-wrapping. encode_neon_ld_st_single([list, [Xn]], load, n) is Err
**Contract evidence:** documented limitation neon.rs:918 "TODO: validate that registers in the list are consecutive (ARM ISA requirement)"
**Documentation conflict:** neon.rs:918 "TODO: validate that registers in the list are consecutive (ARM ISA requirement)" — admits a gap on an input the API accepts; this entry stays, severity one step down.
**Severity:** low (documented by the author)
**Counterexample:** encode_neon_ld_st_single([RegListIndexed({v0.b, v2.b}[0]), Mem{x0,0}], false, 2)
**Expected / Actual:** Err / Ok(Word) using only v0 as Rt
**Impact:** A non-sequential list encodes as if the missing registers were consecutive, storing/loading the wrong vectors.
**Root cause:** neon.rs:918 TODO; the encoder reads only regs[0] for Rt and arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:918`
```rust
    // TODO: validate that registers in the list are consecutive (ARM ISA requirement)
```
**Suggested fix:** After the length check, require wrapping consecutiveness of the named registers.
```rust
    for i in 1..regs.len() {
        let prev = parse_reg_num(reg_name(&regs[i - 1])).ok_or("invalid register in list")?;
        let cur = parse_reg_num(reg_name(&regs[i])).ok_or("invalid register in list")?;
        if cur != (prev + 1) % 32 {
            return Err("registers must be sequential".to_string());
        }
    }
```
**Bug report:** bug_reports/encode_neon_ld_st_single_nonconsecutive.md
**Repro seed:** n=2, sz=b, idx=0, rt=0, rn=0, load=false
**Raw output:**
```text
st2 {v0.b, v2.b}[0], [x0] must Err
```

### B6: register post-index encoded as no-offset

**Formal:** ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, idx ∈ [0,max(sz)], rt,rn,rm ∈ 0..30, load ∈ {0,1}. encode_neon_ld_st_single([list, [Xn], Xm], load, n) = llvm-mc(`ldN|stN {..}[idx], [Xn], Xm`)
**Contract evidence:** inferred (README.md:235 with post-index; ARM Rm=Xm bit23=1; llvm-mc `ld1 {v0.s}[0], [x1], x2` = 0x0dc28020)
**Documentation conflict:** neon.rs:903 "TODO: add post-index form [Xn], #imm" — known limitation on the immediate form; register post-index is the same addressing class and is not excluded.
**Severity:** high
**Counterexample:** encode_neon_ld_st_single([RegListIndexed({v0.s}[0]), Mem{x1,0}, Reg("x2")], true, 1)
**Expected / Actual:** Ok(Word(0x0dc28020)) / Ok(Word(0x0d408020))
**Impact:** Requested post-index writeback is dropped; the emitted instruction does not update Xn.
**Root cause:** neon.rs:933-936 only promotes operands[2] when it is Imm; a trailing Reg is dropped. The post-index encoder always writes Rm=11111.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:933`
```rust
            let pi = if operands.len() > 2 {
                match &operands[2] {
                    Operand::Imm(off) => Some(*off),
                    _ => None,
                }
            } else {
                None
            };
```
**Suggested fix:** Treat a trailing GPR as register post-index and encode Rm=Xm with bit23=1.
```rust
                    Operand::Reg(rm) => Some(PostIndex::Reg(parse_reg_num(rm).ok_or("invalid Rm")?)),
```
**Bug report:** bug_reports/encode_neon_ld_st_single_reg_post.md
**Repro seed:** n=1, sz=b, idx=0, rt=0, rn=0, rm=0, load=false (property); regression ld1 {v0.s}[0], [x1], x2
**Raw output:**
```text
assertion `left == right` failed: ld1 {v0.s}[0], [x1], x2 must be 0x0dc28020
  left: 222330912
  right: 230850592
```

### B7: illegal post-index immediate accepted

**Formal:** ∀ n ∈ {1,2,3,4}, sz ∈ {b,h,s,d}, imm ≠ n*esize(sz). encode_neon_ld_st_single([list, MemPostIndex(Xn, imm)], load, n) is Err
**Contract evidence:** inferred (README.md:235; llvm-mc rejects illegal post-index #imm)
**Documentation conflict:** neon.rs:903 "TODO: add post-index form [Xn], #imm" — the body already encodes the form but ignores the immediate value; the TODO does not declare illegal #imm valid.
**Severity:** medium
**Counterexample:** encode_neon_ld_st_single([RegListIndexed({v0.b}[0]), MemPostIndex{x0, 0}], false, 1)
**Expected / Actual:** Err / Ok(Word) with Rm=11111
**Impact:** A wrong writeback amount is assembled as the legal n*esize post-index encoding.
**Root cause:** neon.rs:994 binds Some(_offset) and ignores the value.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:994`
```rust
    if let Some(_offset) = post_index {
        // Post-index form: Q 0011011 L R 11111 opcode S size Rn Rt
        // (Rm=11111 means immediate post-index, the amount is implicit from element size)
        let word = (q_bit << 30) | (0b0011011 << 23) | (l_bit << 22) | (r_bit << 21)
            | (0b11111 << 16) | (opcode << 13) | (s_bit << 12) | (size_field << 10) | (rn << 5) | rt;
```
**Suggested fix:** Require offset == num_structs * esize before encoding Rm=11111.
```rust
        let legal = (num_structs as i64) * esize_of(&elem_size);
        if offset != legal {
            return Err(format!("post-index immediate must be #{}, got #{}", legal, offset));
        }
```
**Bug report:** bug_reports/encode_neon_ld_st_single_bad_post_imm.md
**Repro seed:** n=1, sz=b, idx=0, rt=0, rn=0, load=false, imm=0
**Raw output:**
```text
st1 {v0.b}[0], [x0], #0 must Err
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_ld_st_single_pbt.rs | 14 properties + 1 KAT + 7 regressions |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_ld_st_single -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_extra_operand -- --test-threads=1
```

B2 invalid base:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_w_base -- --test-threads=1
```

B3 alt spellings:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_alt_spellings -- --test-threads=1
```

B4 index OOR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_index_oor -- --test-threads=1
```

B5 nonconsecutive:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_nonconsecutive -- --test-threads=1
```

B6 register post-index:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_reg_post -- --test-threads=1
```

B7 illegal post-imm:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_single_regression_bad_post_imm -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/run/encode_neon_ld_st_single_round1.log
- pbt-out/run/encode_neon_ld_st_single_round2.log
- pbt-out/bug_reports/encode_neon_ld_st_single_extra_operand.md
- pbt-out/bug_reports/encode_neon_ld_st_single_extra_operand.html
- pbt-out/bug_reports/encode_neon_ld_st_single_invalid_base.md
- pbt-out/bug_reports/encode_neon_ld_st_single_invalid_base.html
- pbt-out/bug_reports/encode_neon_ld_st_single_alt_spellings.md
- pbt-out/bug_reports/encode_neon_ld_st_single_alt_spellings.html
- pbt-out/bug_reports/encode_neon_ld_st_single_index_oor.md
- pbt-out/bug_reports/encode_neon_ld_st_single_index_oor.html
- pbt-out/bug_reports/encode_neon_ld_st_single_nonconsecutive.md
- pbt-out/bug_reports/encode_neon_ld_st_single_nonconsecutive.html
- pbt-out/bug_reports/encode_neon_ld_st_single_reg_post.md
- pbt-out/bug_reports/encode_neon_ld_st_single_reg_post.html
- pbt-out/bug_reports/encode_neon_ld_st_single_bad_post_imm.md
- pbt-out/bug_reports/encode_neon_ld_st_single_bad_post_imm.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 08:36 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 104/289 total | PBT candidates: 104 | Tested: 104 (100%) | 0 pass, 104 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 104 |
| **Tested (of PBT candidates)** | **104 / 104 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 104 / 0 |
| **Overall (tested / all functions)** | **104 / 289 (36%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 104 | 104 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 104 | 104 | 0 | 100% |

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
| neon.rs | 68 | 19 | 19 | 100% | covered |
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
