# PBT Campaign Report: encode_neon_pmul

## Summary

**Verdict:** 4 medium: encode_neon_pmul silently encodes extra operands, mismatched T, reserved non-byte T, and GPR/bare-V as 8B PMUL, so invalid GNU-style assembly assembles instead of failing.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_pmul
**Tests:** 10 properties + 1 KAT + 6 regression witnesses
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_pmul), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust campaign; C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of encode_neon_pmul body plus reserved-T / nonreg sweep.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_pmul | 10 properties (6 pass / 4 fail) | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_pmul ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, t ∈ {8b,16b}. llvm_mc("pmul Vd.t, Vn.t, Vm.t, Vextra.t") is Err ⇒ encode_neon_pmul([Vd.t,Vn.t,Vm.t,Vextra.t]) is Err
**Contract evidence:** documented neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — three-operand form
**Documentation conflict:** (none) — the comment states the three-operand form; it does not declare extra operands invalid, but the GNU/llvm-mc assembler contract does
**Severity:** medium
**Counterexample:** encode_neon_pmul([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x2e209c00))
**Impact:** A typo or extra operand is silently dropped instead of failing the assemble
**Root cause:** neon.rs:336-345 never checks operands.len(); get_neon_reg only reads indices 0..2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:345`
```rust
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand count other than 3
```rust
if operands.len() != 3 {
    return Err("pmul requires 3 operands".to_string());
}
```
**Bug report:** bug_reports/encode_neon_pmul_extra_operand.md
**Repro seed:** rd=0, rn=0, rm=0, extra=0, t=8b
**Raw output:**
```text
Test failed: 4 operands must Err (llvm-mc rejects pmul v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:301.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
```

### B2: encode_neon_pmul ignores source arrangement mismatch

**Formal:** ∀ rd,rn,rm ∈ {0..31}, td,tn,tm ∈ Arr. ¬(td∈{8b,16b} ∧ td=tn ∧ tn=tm) ⇒ llvm_mc("pmul Vd.td, Vn.tn, Vm.tm") is Err ⇒ encode_neon_pmul([Vd.td,Vn.tn,Vm.tm]) is Err
**Contract evidence:** documented neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T" — matching T
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_pmul([v0.8b, v0.8b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0x2e209c00))
**Impact:** A width typo assembles as dest-only 8B PMUL
**Root cause:** neon.rs:338-339 discard Vn/Vm arrangements (`_`); only dest arr_d is consulted
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:338`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require all three arrangements to be equal and in {8b,16b}
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_n != arr_m || (arr_d != "8b" && arr_d != "16b") {
        return Err(format!("pmul requires matching T in {{8b,16b}}, got {arr_d}/{arr_n}/{arr_m}"));
    }
```
**Bug report:** bug_reports/encode_neon_pmul_mismatched_t.md
**Repro seed:** rd=0, rn=0, rm=0, td=8b, tn=8b, tm=16b
**Raw output:**
```text
Test failed: invalid/mismatched/reserved T must Err (ARM PMUL T in {8B,16B} matching; llvm-mc rejects pmul v0.8b, v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:329.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"
```

### B3: encode_neon_pmul encodes reserved non-byte T as 8B

**Formal:** ∀ rd,rn,rm ∈ {0..31}, t ∈ {4h,8h,2s,4s,1d,2d,1q}. llvm_mc("pmul Vd.t, Vn.t, Vm.t") is Err ⇒ encode_neon_pmul([Vd.t,Vn.t,Vm.t]) is Err
**Contract evidence:** documented neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)"
**Documentation conflict:** neon.rs:335 "(polynomial multiply, bytes only)" asserts the ARM domain (T in {8B,16B}); it does not declare other T invalid at the API, and the assembler still encodes them as 8B. Classified as documented-and-violated (the comment states bytes only; the code does not reject non-byte T).
**Severity:** medium
**Counterexample:** encode_neon_pmul([v0.4h, v0.4h, v0.4h])
**Expected / Actual:** Err / Ok(Word(0x2e209c00)) — same as pmul v0.8b, v0.8b, v0.8b
**Impact:** Invalid SIMD widths assemble to a different instruction
**Root cause:** neon.rs:340 sets Q=1 only when arr_d == "16b", else Q=0, with size hardcoded to 00
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:340`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only 8b/16b and error otherwise
```rust
    let q: u32 = match arr_d.as_str() {
        "8b" => 0,
        "16b" => 1,
        _ => return Err(format!("pmul T must be 8b or 16b, got {arr_d}")),
    };
```
**Bug report:** bug_reports/encode_neon_pmul_reserved_t.md
**Repro seed:** rd=0, rn=0, rm=0, t=4h
**Raw output:**
```text
Test failed: reserved PMUL T=4h (ARM T in {8B,16B} only) must Err (llvm-mc rejects pmul v0.4h, v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:457.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h"
```

### B4: encode_neon_pmul encodes GPR and bare-V operands as NEON registers

**Formal:** ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr_dest, bare_vn, gpr_vm, bare_vd, xN_arr}. llvm_mc(kind) is Err ⇒ encode_neon_pmul(kind) is Err
**Contract evidence:** documented neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T"
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_pmul([Reg("x0"), v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x2e209c00))
**Impact:** A wrong register class or missing arrangement is silently encoded as v0.8b
**Root cause:** get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w/d/s/q/v/h/b prefixes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:337`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require Operand::RegArrangement whose register name starts with v/V
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if !matches!(operands.get(0), Some(Operand::RegArrangement { reg, .. }) if reg.starts_with('v') || reg.starts_with('V')) {
        return Err("pmul destination must be Vd.T".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_pmul_gpr_or_bare.md
**Repro seed:** rd=0, rn=0, rm=0, kind=0, fp_prefix=x
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=0 must Err (llvm-mc rejects pmul x0, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs:398.
minimal failing input: rd = 0, rn = 0, rm = 0, kind = 0, fp_prefix = "x"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs | 10 properties + 1 KAT + 6 regressions |
| src/backend/arm/assembler/encoder/mod.rs | one-line `mod encode_neon_pmul_pbt` registration |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_pmul_pbt -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_extra_operand -- --test-threads=1
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_mismatched_t -- --test-threads=1
```

B3 reserved T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_reserved_4h -- --test-threads=1
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_pmul_regression_gpr_dest -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_pmul_extra_operand.md
- pbt-out/bug_reports/encode_neon_pmul_extra_operand.html
- pbt-out/bug_reports/encode_neon_pmul_mismatched_t.md
- pbt-out/bug_reports/encode_neon_pmul_mismatched_t.html
- pbt-out/bug_reports/encode_neon_pmul_reserved_t.md
- pbt-out/bug_reports/encode_neon_pmul_reserved_t.html
- pbt-out/bug_reports/encode_neon_pmul_gpr_or_bare.md
- pbt-out/bug_reports/encode_neon_pmul_gpr_or_bare.html
- pbt-out/run/encode_neon_pmul.log
- pbt-out/run/encode_neon_pmul_pbt.log
- pbt-out/run/encode_neon_pmul_reserved.log
- pbt-out/run/encode_neon_pmul_nonreg.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 17:20 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 130/289 total | PBT candidates: 130 | Tested: 130 (100%) | 0 pass, 130 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 130 |
| **Tested (of PBT candidates)** | **130 / 130 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 130 / 0 |
| **Overall (tested / all functions)** | **130 / 289 (45%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 130 | 130 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 130 | 130 | 0 | 100% |

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
| neon.rs | 68 | 45 | 45 | 100% | covered |
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
