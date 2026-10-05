# PBT Campaign Report: encode_neon_ld_st_multi

## Summary

**Verdict:** 1 high and 5 medium and 1 low: encode_neon_ld_st_multi silently encodes illegal LD2/3/4 list lengths and non-consecutive register lists as if they were the first register's consecutive form, so a caller that round-trips GNU assembly through this encoder gets the wrong vectors; extra operands, W/XZR/FP bases, illegal post-index immediates, .1d on LD2/3/4, and uppercase arrangements also disagree with llvm-mc/gas.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_ld_st_multi
**Tests:** 14 properties + 1 KAT + 7 regression witnesses
**Result:** 7 passing, 7 bugs
**Change surface:** 1 changed function (encode_neon_ld_st_multi), 1 with a property, 0 error-handling changes
**Coverage evidence:** none — coverage_gaps reported no .gcda/.profraw (the cargo test tree was configured before this campaign; the C++ reporter listed unrelated binaries). File-level fallback incorrectly claimed encode_neon_ld_st_multi is NOT LINKED. Manual evidence: `cargo test --lib encode_neon_ld_st_multi` executed the production symbol.
**Effort tier:** standard (5–8 properties, ≥1000 cases, one strengthening/sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_ld_st_multi | 14 | 7 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: LD2/ST2/LD3/ST3/LD4/ST4 encoded with the wrong list length

**Formal:** ∀ n∈{1,2,3,4}. unsupported T | invalid name ∈{foo,v32,x32,r0,ε} | LD1 n_regs∉{1,2,3,4} | LD2/3/4 n_regs≠n ⇒ encode_neon_ld_st_multi = Err
**Contract evidence:** inferred (llvm-mc/gas reject `ld2 {v0.16b}, [x0]`; neon.rs:1043 comment names "LD2/ST2: 2 reg=1000")
**Documentation conflict:** neon.rs:1043 "LD2/ST2: 2 reg=1000" names the 2-register opcode; it does not declare other lengths invalid / out of domain. The mismatch is a missing check, not a documented exclusion.
**Severity:** high
**Counterexample:** encode_neon_ld_st_multi([RegList({v0.16b}), Mem{x0,0}], is_load=true, num_structs=2)
**Expected / Actual:** Err / Ok(Word(0x4c408000))
**Impact:** A 1-register list is assembled as LD2, so the wrong number of structures is loaded.
**Root cause:** neon.rs:1056-1058 assigns a fixed opcode for num_structs∈{2,3,4} and never checks num_regs.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1056`
```rust
        2 => 0b1000u32,
        3 => 0b0100,
        4 => 0b0000,
```
**Suggested fix:** Require num_regs == num_structs for n in {2,3,4}.
```rust
        2 if num_regs == 2 => 0b1000u32,
        3 if num_regs == 3 => 0b0100,
        4 if num_regs == 4 => 0b0000,
        2 | 3 | 4 => return Err(format!("ld{}/st{}: expected {} registers, got {}", num_structs, num_structs, num_structs, num_regs)),
```
**Bug report:** bug_reports/encode_neon_ld_st_multi_wrong_reg_count.md
**Repro seed:** cc 826492188fd82275f92f3667941628350006fa5556d2932409ff349c894a1138
**Raw output:** Test failed: ld2 with 1 regs must Err (llvm-mc rejects wrong vector count). minimal failing input: n = 2, wrong_len = 1, t_idx = 0, name_idx = 0, rn = 0

### B2: Surplus non-post-index operand ignored

**Formal:** ∀ valid no-offset ops, extra∈{Cond,Shift,RegArrangement,Label}. encode_neon_ld_st_multi(ops++[extra], load, n) = Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembler; llvm-mc rejects `st1 {v0.8b}, [x0], eq`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_multi([RegList({v0.8b}), Mem{x0,0}, Cond("eq")], is_load=false, num_structs=1)
**Expected / Actual:** Err / Ok(Word(0x0c007000)) encoded as `st1 {v0.8b}, [x0]`
**Impact:** A trailing token is dropped and a valid instruction is emitted.
**Root cause:** neon.rs:1083 `_ => {}` ignores a third operand that is not Imm or Reg, then falls through to no-offset encoding.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1083`
```rust
            _ => {}
```
**Suggested fix:** Return Err for any extra operand that is not a post-index Imm or Xm.
```rust
            other => return Err(format!("ld{}/st{}: unexpected extra operand {:?}", num_structs, num_structs, other)),
```
**Bug report:** bug_reports/encode_neon_ld_st_multi_extra_operand.md
**Repro seed:** (deterministic regression)
**Raw output:** st1 {v0.8b}, [x0], eq must Err

### B3: W/XZR/x31/FP accepted as the memory base

**Formal:** ∀ n,T,rt,load, base∈{w0,w31,wsp,xzr,x31,s0,d0,v0,q0}. encode_neon_ld_st_multi(list, Mem{base}, load, n) = Err
**Contract evidence:** inferred (ARM base is Xn|SP; llvm-mc rejects W/XZR/x31/FP)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_multi([RegList({v0.8b}), Mem{base:"w0", offset:0}], is_load=false, num_structs=1)
**Expected / Actual:** Err / Ok(Word(0x0c007000)) encoded as `[x0]`
**Impact:** A W or FP base is silently rewritten as Xn; xzr/x31 become SP.
**Root cause:** neon.rs:1032 calls parse_reg_num, which accepts w/d/s/q/v/h/b and maps xzr/x31 to 31, with no Xn|SP check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1032`
```rust
            let r = parse_reg_num(base).ok_or_else(|| format!("invalid base register: {}", base))?;
```
**Suggested fix:** Reject a base that is not an X register or SP.
```rust
            if !base.eq_ignore_ascii_case("sp") && !base.to_ascii_lowercase().starts_with('x')
                || base.eq_ignore_ascii_case("xzr") || base.eq_ignore_ascii_case("x31") {
                return Err(format!("ld{}/st{}: base must be Xn or SP, got {}", num_structs, num_structs, base));
            }
```
**Bug report:** bug_reports/encode_neon_ld_st_multi_invalid_base.md
**Repro seed:** (deterministic regression)
**Raw output:** st1 {v0.8b}, [w0] must Err

### B4: Uppercase arrangement rejected while llvm-mc/gas accept it

**Formal:** ∀ valid inputs. encode_neon_ld_st_multi(RegList({Vrt.T_upper..}), Mem[XN], load, n) = llvm-mc(uppercase spelling)
**Contract evidence:** inferred (README.md:12 accepts the same textual assembly that GCC's gas would consume)
**Documentation conflict:** (none)
**Severity:** low
**Counterexample:** encode_neon_ld_st_multi([RegList({V0.8B}), Mem{X0,0}], is_load=true, num_structs=1)
**Expected / Actual:** Ok(Word(0x0c407000)) / Err("unsupported NEON arrangement: 8B")
**Impact:** Uppercase GCC assembly cannot be consumed.
**Root cause:** neon.rs:1028 passes the arrangement to neon_arr_to_q_size, whose match arms are lowercase-only.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1028`
```rust
    let (q, size) = neon_arr_to_q_size(&arr)?;
```
**Suggested fix:** Case-fold the arrangement before lookup.
```rust
    let (q, size) = neon_arr_to_q_size(&arr.to_ascii_lowercase())?;
```
**Bug report:** bug_reports/encode_neon_ld_st_multi_uppercase_arrangement.md
**Repro seed:** (deterministic regression)
**Raw output:** left: Err("unsupported NEON arrangement: 8B") right: Ok(205549568)

### B5: Non-consecutive register list encoded from the first register only

**Formal:** ∀ n∈{2,3,4}, T∈valid(n), rt∈0..28, rn∈0..30, load∈𝔹. list with regs[1] skipped ⇒ encode_neon_ld_st_multi = Err
**Contract evidence:** inferred (llvm-mc "registers must be sequential"; ARM ISA consecutive wrapping lists)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_ld_st_multi([RegList({v0.8b, v2.8b}), Mem{x0,0}], is_load=false, num_structs=2)
**Expected / Actual:** Err / Ok(Word(0x0c008000)) encoded as st2 from v0 (implies v1)
**Impact:** The second listed register is dropped; the wrong pair is stored.
**Root cause:** neon.rs:1016-1022 reads only regs[0] for Rt; later registers are never checked.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1016`
```rust
            let (first_reg, arrangement) = match &regs[0] {
                Operand::RegArrangement { reg, arrangement } => {
                    (parse_reg_num(reg).ok_or("invalid reg")?, arrangement.clone())
                }
                _ => return Err(format!("ld{}/st{}: expected RegArrangement in list", num_structs, num_structs)),
            };
            (first_reg, arrangement, regs.len() as u32)
```
**Suggested fix:** Require each subsequent list entry to be (first+i) mod 32 with the same arrangement.
```rust
            for (i, r) in regs.iter().enumerate().skip(1) {
                match r {
                    Operand::RegArrangement { reg, arrangement: a } => {
                        let num = parse_reg_num(reg).ok_or("invalid reg")?;
                        if num != (first_reg + i as u32) % 32 || a != &arrangement {
                            return Err("ld/st multi: registers must be sequential".into());
                        }
                    }
                    _ => return Err("ld/st multi: expected RegArrangement in list".into()),
                }
            }
```
**Bug report:** bug_reports/encode_neon_ld_st_multi_nonconsecutive.md
**Repro seed:** (deterministic regression)
**Raw output:** st2 {v0.8b, v2.8b}, [x0] must Err

### B6: Illegal post-index immediate encoded as the legal #imm form

**Formal:** ∀ valid inputs, bad≠legal_imm. encode_neon_ld_st_multi(RegList, MemPostIndex(Xn, #bad), load, n) = Err
**Contract evidence:** inferred (ARM immediate post-index amount is implicit n_regs*(Q?16:8); llvm-mc rejects other #imm)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_multi([RegList({v0.8b}), MemPostIndex{x0, 0}], is_load=false, num_structs=1)
**Expected / Actual:** Err / Ok(Word(0x0c9f7000)) encoded as `[x0], #8`
**Impact:** A wrong writeback amount is assembled as the legal one.
**Root cause:** neon.rs:1064 binds `_imm` and always writes Rm=11111.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1064`
```rust
    if let Some(_imm) = post_index {
        // Post-index with immediate: use Rm=11111 (0x1F)
        let word = ((q << 30) | (0b001100 << 24) | (1 << 23) | (l_bit << 22)) | (0b11111 << 16) | (opcode << 12) | (size << 10) | (rn << 5) | rt;
```
**Suggested fix:** Require the immediate to equal n_regs * (if q==1 {16} else {8}).
```rust
    if let Some(imm) = post_index {
        let legal = n_regs as i64 * if q == 1 { 16 } else { 8 };
        if imm != legal {
            return Err(format!("ld{}/st{}: post-index #{} != #{}", num_structs, num_structs, imm, legal));
        }
```
**Bug report:** bug_reports/encode_neon_ld_st_multi_bad_post_imm.md
**Repro seed:** (deterministic regression)
**Raw output:** st1 {v0.8b}, [x0], #0 must Err

### B7: .1d accepted for LD2/ST2/LD3/ST3/LD4/ST4

**Formal:** ∀ n∈{2,3,4}, rt,rn,load. encode_neon_ld_st_multi(RegList({v.1d}×n), Mem[Xn], load, n) = Err
**Contract evidence:** inferred (llvm-mc rejects `ld2 {v0.1d, v1.1d}, [x0]`; .1d is LD1/ST1-only)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ld_st_multi([RegList({v0.1d, v1.1d}), Mem{x0,0}], is_load=false, num_structs=2)
**Expected / Actual:** Err / Ok(Word(0x0c008c00))
**Impact:** An undocumented/unpredictable encoding is emitted for a form gas rejects.
**Root cause:** neon.rs:1028 uses neon_arr_to_q_size which accepts "1d" for every mnemonic; no check that Q=0 size=11 is illegal for n≥2.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1028`
```rust
    let (q, size) = neon_arr_to_q_size(&arr)?;
```
**Suggested fix:** Reject .1d when num_structs is 2, 3, or 4.
```rust
    if num_structs >= 2 && q == 0 && size == 0b11 {
        return Err(format!("ld{}/st{}: .1d is not a valid arrangement", num_structs, num_structs));
    }
```
**Bug report:** bug_reports/encode_neon_ld_st_multi_1d_ldn.md
**Repro seed:** (deterministic regression)
**Raw output:** st2 {v0.1d, v1.1d}, [x0] must Err

## Design Caveats

(none)}

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_ld_st_multi_pbt.rs | 14 properties + 1 KAT + 7 regressions |
| src/backend/arm/assembler/encoder/mod.rs | one additive `#[cfg(test)] mod encode_neon_ld_st_multi_pbt;` |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_ld_st_multi -- --test-threads=1
```

Per-bug regressions:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ld_st_multi_regression_ld2_one_reg -- --test-threads=1
cargo test --lib test_encode_neon_ld_st_multi_regression_extra_operand -- --test-threads=1
cargo test --lib test_encode_neon_ld_st_multi_regression_w_base -- --test-threads=1
cargo test --lib test_encode_neon_ld_st_multi_regression_uppercase_arr -- --test-threads=1
cargo test --lib test_encode_neon_ld_st_multi_regression_nonconsecutive -- --test-threads=1
cargo test --lib test_encode_neon_ld_st_multi_regression_bad_post_imm -- --test-threads=1
cargo test --lib test_encode_neon_ld_st_multi_regression_1d_ld2 -- --test-threads=1
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
- pbt-out/run/encode_neon_ld_st_multi_round1.log
- pbt-out/run/encode_neon_ld_st_multi_round2.log
- pbt-out/run/encode_neon_ld_st_multi_regressions.log
- pbt-out/bug_reports/encode_neon_ld_st_multi_wrong_reg_count.md
- pbt-out/bug_reports/encode_neon_ld_st_multi_extra_operand.md
- pbt-out/bug_reports/encode_neon_ld_st_multi_invalid_base.md
- pbt-out/bug_reports/encode_neon_ld_st_multi_uppercase_arrangement.md
- pbt-out/bug_reports/encode_neon_ld_st_multi_nonconsecutive.md
- pbt-out/bug_reports/encode_neon_ld_st_multi_bad_post_imm.md
- pbt-out/bug_reports/encode_neon_ld_st_multi_1d_ldn.md
- pbt-out/bug_reports/encode_neon_ld_st_multi_wrong_reg_count.html
- pbt-out/bug_reports/encode_neon_ld_st_multi_extra_operand.html
- pbt-out/bug_reports/encode_neon_ld_st_multi_invalid_base.html
- pbt-out/bug_reports/encode_neon_ld_st_multi_uppercase_arrangement.html
- pbt-out/bug_reports/encode_neon_ld_st_multi_nonconsecutive.html
- pbt-out/bug_reports/encode_neon_ld_st_multi_bad_post_imm.html
- pbt-out/bug_reports/encode_neon_ld_st_multi_1d_ldn.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 09:04 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 105/289 total | PBT candidates: 105 | Tested: 105 (100%) | 0 pass, 105 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 105 |
| **Tested (of PBT candidates)** | **105 / 105 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 105 / 0 |
| **Overall (tested / all functions)** | **105 / 289 (36%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 105 | 105 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 105 | 105 | 0 | 100% |

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
| neon.rs | 68 | 20 | 20 | 100% | covered |
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
