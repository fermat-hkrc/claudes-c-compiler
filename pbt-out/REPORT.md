# PBT Campaign Report: encode_neon_mls

## Summary

**Verdict:** 4 medium: encode_neon_mls silently encodes extra operands, mismatched T, reserved .1d/.2d, and bare/GPR registers that llvm-mc/gas reject, so invalid GNU-style MLS assembles instead of failing.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_mls
**Tests:** 10 properties (plus 1 KAT + 6 regression witnesses)
**Result:** 6 passing, 4 failing (4 bugs)
**Change surface:** 1 changed function (encode_neon_mls), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_neon_mls NOT LINKED). The cargo test --lib encode_neon_mls run executed the real symbol; sweep was a manual arm audit plus reserved-T and nonreg properties.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_mls | 10 properties (6 pass / 4 fail) + 1 KAT + 6 regressions | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_mls ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. llvm-mc rejects 4-operand mls ⇒ encode_neon_mls([Vd.t,Vn.t,Vm.t,Vextra.t]) is Err
**Contract evidence:** documented neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — three operands; assembler README.md:12 gas-compatible
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mls([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x2e209400))
**Impact:** Invalid assembly `mls Vd.T, Vn.T, Vm.T, Vextra.T` is assembled as if the extra register were absent
**Root cause:** neon.rs:369 returns Ok without checking operands.len(); get_neon_reg only reads indices 0..2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:369`
```rust
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand count other than 3
```rust
if operands.len() != 3 {
    return Err("mls requires 3 operands".to_string());
}
```
**Bug report:** bug_reports/encode_neon_mls_extra_operand.md
**Repro seed:** rd=0, rn=0, rm=0, extra=0, t=8b
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:303:1:
Test failed: 4 operands must Err (llvm-mc rejects mls v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:322.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"
```

### B2: encode_neon_mls ignores source arrangement mismatch

**Formal:** ∀ rd,rn,rm ∈ {0..31}, td,tn,tm ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}. ¬(valid_mls(td) ∧ td=tn=tm) ∧ llvm-mc rejects ⇒ encode_neon_mls([Vd.td,Vn.tn,Vm.tm]) is Err
**Contract evidence:** documented neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — same T on all three
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mls([v0.8b, v0.16b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x2e209400))
**Impact:** A width typo such as `mls v0.8b, v0.16b, v0.8b` assembles as dest-only 8B MLS instead of failing
**Root cause:** neon.rs:363 discards Vn arrangement (`_`); only dest arr_d is passed to neon_arr_to_q_size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:363`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require all three arrangements to be equal and in the MLS T set
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_d != arr_n || arr_n != arr_m {
        return Err(format!("mls requires matching T, got {arr_d}/{arr_n}/{arr_m}"));
    }
```
**Bug report:** bug_reports/encode_neon_mls_mismatched_t.md
**Repro seed:** rd=0, rn=0, rm=0, td=8b, tn=16b, tm=8b
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:330:1:
Test failed: invalid/mismatched/reserved T must Err (ARM MLS T in {8B,16B,4H,8H,2S,4S} matching; llvm-mc rejects mls v0.8b, v0.16b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:350.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "16b", tm = "8b"
```

### B3: encode_neon_mls encodes reserved .1d/.2d MLS

**Formal:** ∀ rd,rn,rm ∈ {0..31}, t ∈ {1d,2d}. llvm-mc rejects mls Vd.t,Vn.t,Vm.t ⇒ encode_neon_mls is Err
**Contract evidence:** inferred (ARM Advanced SIMD three-same MLS T in {8B,16B,4H,8H,2S,4S}; size:Q=11:x reserved; llvm-mc rejects)
**Documentation conflict:** (none) — neon.rs:365 documents the encoding layout but does not declare 1d/2d invalid
**Severity:** medium
**Counterexample:** encode_neon_mls([v0.1d, v0.1d, v0.1d])
**Expected / Actual:** Err / Ok(Word(0x2ee09400)) for .1d; Ok(Word(0x6ee09400)) for .2d
**Impact:** `mls v0.1d` / `mls v0.2d` assemble to unallocated encodings; executing them is UNDEFINED
**Root cause:** neon.rs:365 calls neon_arr_to_q_size, which maps 1d/2d to size=11, with no MLS-specific rejection
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:365`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size=11 (1d/2d) for vector MLS
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 {
        return Err(format!("MLS does not support arrangement {arr_d}"));
    }
```
**Bug report:** bug_reports/encode_neon_mls_reserved_t.md
**Repro seed:** rd=0, rn=0, rm=0, t=1d
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_reserved_t' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:428:1:
Test failed: reserved MLS T=1d (ARM size:Q=11:x) must Err (llvm-mc rejects mls v0.1d, v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:513.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "1d"
```

### B4: encode_neon_mls accepts bare V / GPR as NEON MLS operands

**Formal:** ∀ rd,rn,rm ∈ {0..31}, kind ∈ {0..4}. llvm-mc rejects the corresponding non-arrangement form ⇒ encode_neon_mls(ops_kind) is Err
**Contract evidence:** documented neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — arrangement operands
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mls([v0.8b, Operand::Reg("v0"), v0.8b])
**Expected / Actual:** Err / Ok(Word(0x2e209400))
**Impact:** `mls v0.8b, v0, v0.8b` and `mls x0.8b, v0.8b, v0.8b` assemble as vector MLS instead of failing
**Root cause:** neon.rs:363 calls get_neon_reg, which accepts Operand::Reg (empty arrangement) and parse_reg_num maps x/w prefixes to the same 0..31 index
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:363`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require RegArrangement with a V-prefixed register on every operand
```rust
    match operands.get(idx) {
        Some(Operand::RegArrangement { reg, arrangement }) if reg.to_lowercase().starts_with('v') => { /* parse */ }
        other => return Err(format!("expected NEON Vn.T at operand {idx}, got {:?}", other)),
    }
```
**Bug report:** bug_reports/encode_neon_mls_gpr_or_bare.md
**Repro seed:** rd=0, rn=0, rm=0, kind=1, fp_prefix=x
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_mls_pbt::encode_neon_mls_neg_gpr_or_bare' panicked at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:358:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects mls v0.8b, v0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs:419.
minimal failing input: rd = 0, rn = 0, rm = 0, kind = 1, fp_prefix = "x"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs | 10 properties + 1 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_mls_pbt` registration |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mls -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_extra_operand -- --test-threads=1
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_mismatched_t -- --test-threads=1
```

B3 reserved T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_reserved_1d -- --test-threads=1
```

B4 bare src:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mls_regression_bare_src -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_neon_mls_extra_operand.md
- pbt-out/bug_reports/encode_neon_mls_extra_operand.html
- pbt-out/bug_reports/encode_neon_mls_mismatched_t.md
- pbt-out/bug_reports/encode_neon_mls_mismatched_t.html
- pbt-out/bug_reports/encode_neon_mls_reserved_t.md
- pbt-out/bug_reports/encode_neon_mls_reserved_t.html
- pbt-out/bug_reports/encode_neon_mls_gpr_or_bare.md
- pbt-out/bug_reports/encode_neon_mls_gpr_or_bare.html
- pbt-out/run/encode_neon_mls_round1.log
- pbt-out/run/encode_neon_mls_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 17:54 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 132/289 total | PBT candidates: 132 | Tested: 132 (100%) | 0 pass, 132 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 132 |
| **Tested (of PBT candidates)** | **132 / 132 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 132 / 0 |
| **Overall (tested / all functions)** | **132 / 289 (46%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 132 | 132 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 132 | 132 | 0 | 100% |

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
| neon.rs | 68 | 47 | 47 | 100% | covered |
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
| encode_neon_mla | neon.rs |
| encode_neon_mls | neon.rs |
