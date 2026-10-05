# PBT Campaign Report: encode_neon_ext

## Summary

**Verdict:** 5 medium: encode_neon_ext accepts invalid EXT assembly (extra operand, non-{8b,16b} T, out-of-range index, mismatched T, GPR/bare dest) and silently emits a 32-bit word, so a typo becomes the wrong or UNALLOCATED extract instead of an assembler error.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_ext
**Tests:** 10
**Result:** 5 passing, 5 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit covered mismatched T and GPR/bare dest.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_ext | 10 | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_ext ignores a surplus fifth operand

**Formal:** ∀ valid 4-operand EXT ops, extra ∈ Operand. encode_neon_ext(ops ++ [extra]) = Err
**Contract evidence:** inferred (README.md:12 gas compatibility; llvm-mc and gas reject a fifth operand; the arity string only guards too few)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ext([v0.8b, v0.8b, v0.8b, #0, v0.8b]) then Ok(Word(0x2e000000))
**Expected / Actual:** Err / Ok(Word(0x2e000000)) encoded as ext v0.8b, v0.8b, v0.8b, #0
**Impact:** A typo or extra token silently produces a valid EXT instead of an assembler error.
**Root cause:** neon.rs:406 `if operands.len() < 4` only rejects too few operands; extras past index 3 are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:406`
```rust
    if operands.len() < 4 {
        return Err("ext requires 4 operands".to_string());
    }
```
**Suggested fix:** Require exactly four operands.
```rust
    if operands.len() != 4 {
        return Err("ext requires 4 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_ext_extra_operand.md
**Repro seed:** rd = 0, rn = 0, rm = 0, extra = 0, (t, i) = ("8b", 0)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:533:5:
ext v0.16b, v1.16b, v2.16b, #3, v3.16b must Err (gas/llvm-mc reject a fifth operand)
```

### B2: encode_neon_ext encodes invalid arrangements as 8B EXT

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∉ {8b,16b} among ARM arrangement tokens, i ∈ [0,15]. encode_neon_ext([Vd.T, Vn.T, Vm.T, #i]) = Err
**Contract evidence:** inferred (ARM Advanced SIMD extract T ∈ {8B,16B}; gas/llvm-mc reject .8h/.4s/.2d; README.md:12 gas compatibility)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ext([v0.8h, v0.8h, v0.8h, #0])
**Expected / Actual:** Err / Ok(Word(0x2e000000)) encoded as ext v0.8b, v0.8b, v0.8b, #0
**Impact:** An illegal permute (.8h/.4s/.2d) silently becomes an 8B extract.
**Root cause:** neon.rs:414 sets Q=1 only for arr_d=="16b" and otherwise Q=0 with no arrangement whitelist.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:414`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only 8b and 16b.
```rust
    let q: u32 = match arr_d.as_str() {
        "16b" => 1,
        "8b" => 0,
        _ => return Err(format!("unsupported EXT arrangement: {}", arr_d)),
    };
```
**Bug report:** bug_reports/encode_neon_ext_invalid_t.md
**Repro seed:** rd = 0, rn = 0, rm = 0, t = "8h", i = 0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:542:5:
ext v0.8h, v1.8h, v2.8h, #1 must Err (gas/llvm-mc accept only .8b/.16b)
```

### B3: encode_neon_ext masks an out-of-range EXT index instead of rejecting it

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, i > imax(T) ∨ i < 0. encode_neon_ext([Vd.T, Vn.T, Vm.T, #i]) = Err
**Contract evidence:** inferred (gas "immediate value out of range 0 to 7/15"; ARM Q=0 and imm4<3>!=0 is UNALLOCATED; README.md:12 gas compatibility)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ext([v0.8b, v0.8b, v0.8b, #8])
**Expected / Actual:** Err / Ok(Word(0x2e004000)) with imm4=8 (UNALLOCATED for Q=0)
**Impact:** A typo or computed index silently produces a different or illegal instruction. llvm-mc wraps; gas rejects; the SUT encodes UNALLOCATED.
**Root cause:** neon.rs:412 casts the immediate as u32 with no range check; neon.rs:418 then does `index & 0xF`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:418`
```rust
        | (rm << 16)) | ((index & 0xF) << 11)) | (rn << 5) | rd;
```
**Suggested fix:** Reject index outside the arrangement's byte count before encoding.
```rust
    let max = if arr_d == "16b" { 15 } else { 7 };
    if index > max {
        return Err(format!("EXT index {} out of range 0 to {}", index, max));
    }
```
**Bug report:** bug_reports/encode_neon_ext_index_oor.md
**Repro seed:** rd = 0, rn = 0, rm = 0, t = "8b", over = 1, neg = 1
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_index_oor' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:559:5:
ext v0.8b, v1.8b, v2.8b, #8 must Err (gas range for .8b is [0, 7])
```

### B4: encode_neon_ext ignores mismatched source arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b}, i ∈ [0,7]. (Td ≠ Tn ∨ Td ≠ Tm) ⇒ encode_neon_ext([Vd.Td, Vn.Tn, Vm.Tm, #i]) = Err
**Contract evidence:** inferred (comment neon.rs:415 "EXT Vd.T, Vn.T, Vm.T, #index" names one T; gas/llvm-mc reject mixed T)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ext([v0.8b, v0.8b, v0.16b, #0])
**Expected / Actual:** Err / Ok(Word(0x2e000000)) encoded as 8B EXT
**Impact:** A mixed 64-bit/128-bit permute silently becomes an 8B extract using only dest T.
**Root cause:** neon.rs:410-411 bind source arrangements to `_` and never compare them to arr_d.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:410`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require matching arrangements on all three registers.
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("EXT arrangement mismatch: {} vs {} vs {}", arr_d, arr_n, arr_m));
    }
```
**Bug report:** bug_reports/encode_neon_ext_mismatched_t.md
**Repro seed:** rd = 0, rn = 0, rm = 0, t_d = "8b", t_n = "8b", t_m = "16b", i = 0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_mismatched_t' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:582:5:
ext v0.16b, v1.8b, v2.16b, #3 must Err (gas/llvm-mc require matching T)
```

### B5: encode_neon_ext encodes a GPR or bare-V dest as 8B EXT

**Formal:** ∀ kind ∈ {GPR dest, bare Vn, GPR Vm, bare V dest, non-Imm index}. encode_neon_ext(ops) = Err
**Contract evidence:** inferred (EXT requires Vd.T; get_neon_reg Operand::Reg path returns empty arrangement; gas/llvm-mc reject `ext x0, …`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ext([x0, v0.8b, v0.8b, #0])
**Expected / Actual:** Err / Ok(Word(0x2e000000)) encoded as ext v0.8b, v0.8b, v0.8b, #0
**Impact:** A GPR/FP/bare-V typo silently becomes a different vector extract (x0 maps to Rd=0, empty T ⇒ Q=0).
**Root cause:** get_neon_reg neon.rs:14 accepts Operand::Reg and returns an empty arrangement; encode_neon_ext treats any non-"16b" arrangement as Q=0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Reject Operand::Reg in EXT (require RegArrangement on Vd/Vn/Vm).
```rust
        Some(Operand::Reg(name)) => {
            Err(format!("expected NEON register with arrangement, got {}", name))
        }
```
**Bug report:** bug_reports/encode_neon_ext_gpr_dest.md
**Repro seed:** rd = 0, rn = 0, rm = 0, (t, i) = ("8b", 0), kind = 0, fp_prefix = "x"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ext_pbt::test_encode_neon_ext_regression_gpr_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs:597:5:
ext x0, v1.16b, v2.16b, #3 must Err (gas/llvm-mc reject GPR dest)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs | 10 properties + 1 KAT + 6 regression witnesses |

## Reproduction

Valid-domain suite (5 passing properties + KAT):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_ext_diff_llvm_mc -- --test-threads=1
cargo test --lib encode_neon_ext_kat_llvm_mc -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_extra_operand -- --test-threads=1
```

B2 invalid T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_invalid_t -- --test-threads=1
```

B3 index OOR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_index_oor -- --test-threads=1
```

B4 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_mismatched_t -- --test-threads=1
```

B5 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ext_regression_gpr_dest -- --test-threads=1
```

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_ext -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/report.json — machine-readable report
- pbt-out/bug_reports/encode_neon_ext_extra_operand.md (+ .html)
- pbt-out/bug_reports/encode_neon_ext_invalid_t.md (+ .html)
- pbt-out/bug_reports/encode_neon_ext_index_oor.md (+ .html)
- pbt-out/bug_reports/encode_neon_ext_mismatched_t.md (+ .html)
- pbt-out/bug_reports/encode_neon_ext_gpr_dest.md (+ .html)
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/run/ — test scratch (empty for cargo)
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_ext_pbt.txt — proptest failure cache (framework artifact)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 10:29 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 109/289 total | PBT candidates: 109 | Tested: 109 (100%) | 0 pass, 109 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 109 |
| **Tested (of PBT candidates)** | **109 / 109 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 109 / 0 |
| **Overall (tested / all functions)** | **109 / 289 (38%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 109 | 109 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 109 | 109 | 0 | 100% |

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
| neon.rs | 68 | 24 | 24 | 100% | covered |
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
