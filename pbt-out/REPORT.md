# PBT Campaign Report: encode_neon_mul

## Summary

**Verdict:** 4 medium: encode_neon_mul silently encodes a fourth operand, mismatched T, reserved .1d/.2d, and a bare Vn / GPR dest instead of rejecting them as gas/llvm-mc do.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_mul
**Tests:** 10 properties + 1 KAT + 6 regression witnesses
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (build tree not instrumented); C++ reporter listed unrelated binaries and claimed encode_neon_mul NOT LINKED. Manual arm audit of neon.rs:323-332: dest/src get_neon_reg, neon_arr_to_q_size, Ok Word all driven by the suite.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_mul | 10 properties (6 passing, 4 failing) + 1 KAT + 6 regressions | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_mul ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. llvm-mc rejects "mul Vd.T, Vn.T, Vm.T, Vextra.T" ⇒ encode_neon_mul([Vd.T,Vn.T,Vm.T,Vextra.T]) = Err
**Contract evidence:** documented neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" (three operands); inferred (llvm-mc/gas reject a fourth operand; README gas-compatible assembler)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mul([v0.8b, v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e209c00))
**Impact:** Invalid four-operand MUL is assembled as the three-operand form; a typo is silently dropped
**Root cause:** neon.rs:323-332 never checks operands.len(); get_neon_reg reads only indices 0..2
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:323`
```rust
pub(crate) fn encode_neon_mul(operands: &[Operand]) -> Result<EncodeResult, String> {
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let (q, size) = neon_arr_to_q_size(&arr_d)?;

    // MUL (vector): 0 Q 0 01110 size 1 Rm 10011 1 Rn Rd
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (1 << 21)
        | (rm << 16) | (0b100111 << 10) | (rn << 5) | rd;
    Ok(EncodeResult::Word(word))
}
```
**Suggested fix:** Reject any operand count other than 3
```rust
if operands.len() != 3 {
    return Err("mul requires 3 operands".to_string());
}
```
**Bug report:** bug_reports/encode_neon_mul_extra_operand.md
**Repro seed:** rd=0, rn=0, rm=0, extra=0, t="8b"
**Raw output:** Test failed: 4 operands must Err (llvm-mc rejects mul v0.8b, v0.8b, v0.8b, v0.8b). minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b"

### B2: encode_neon_mul ignores source arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}. ¬(valid_mul_T(Td) ∧ Td=Tn ∧ Tn=Tm) ∧ llvm-mc rejects "mul Vd.Td, Vn.Tn, Vm.Tm" ⇒ encode_neon_mul([Vd.Td,Vn.Tn,Vm.Tm]) = Err
**Contract evidence:** documented neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" (same T); inferred (ARM matching T; llvm-mc rejects mul v0.8b, v0.8b, v0.16b)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mul([v0.8b, v0.8b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0x0e209c00))
**Impact:** A width mismatch is coerced to the destination arrangement, producing a different MUL than written
**Root cause:** neon.rs:325-326 bind source arrangements to `_`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:325`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn.T and Vm.T to match Vd.T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("MUL arrangement mismatch: {arr_d} vs {arr_n} vs {arr_m}"));
    }
```
**Bug report:** bug_reports/encode_neon_mul_mismatched_t.md
**Repro seed:** rd=0, rn=0, rm=0, td="8b", tn="8b", tm="16b"
**Raw output:** Test failed: invalid/mismatched/reserved T must Err ... mul v0.8b, v0.8b, v0.16b. minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b"

### B3: encode_neon_mul encodes reserved .1d/.2d MUL

**Formal:** ∀ rd,rn,rm ∈ {0..31}, T ∈ {1d,2d}. llvm-mc rejects "mul Vd.T, Vn.T, Vm.T" ⇒ encode_neon_mul([Vd.T,Vn.T,Vm.T]) = Err
**Contract evidence:** inferred (ARM Advanced SIMD MUL T in {8B,16B,4H,8H,2S,4S}; size:Q=11:x reserved; llvm-mc rejects .1d and .2d)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mul([v0.1d, v0.1d, v0.1d]); also .2d
**Expected / Actual:** Err / Ok(Word(0x0ee09c00)) for .1d; Ok(Word(0x4ee09c00)) for .2d
**Impact:** Unallocated encodings that gas/llvm-mc refuse; executing them is UNDEFINED
**Root cause:** neon.rs:327 calls neon_arr_to_q_size, which maps 1d/2d to size=11, with no MUL-specific rejection
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:327`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size=11 for vector MUL
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 {
        return Err(format!("MUL does not support arrangement {arr_d}"));
    }
```
**Bug report:** bug_reports/encode_neon_mul_reserved_t.md
**Repro seed:** rd=0, rn=0, rm=0, t="1d"
**Raw output:** Test failed: reserved MUL T=1d (ARM size:Q=11:x) must Err (llvm-mc rejects mul v0.1d, v0.1d, v0.1d). minimal failing input: rd = 0, rn = 0, rm = 0, t = "1d"

### B4: encode_neon_mul accepts a bare Vn / GPR dest as a NEON register

**Formal:** ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr-dest, bare-Vn, x-Rm, bare-Vd, xN.8b-dest}. llvm-mc rejects the corresponding asm ⇒ encode_neon_mul(ops(kind)) = Err
**Contract evidence:** documented neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T"; inferred (llvm-mc requires arrangement on every operand; parse_reg_num x/w is not a V register)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_mul([v0.8b, Reg("v0"), v0.8b]); also mul x0.8b, v0.8b, v0.8b
**Expected / Actual:** Err / Ok(Word(0x0e209c00))
**Impact:** Missing arrangement or a GPR name is treated as vN.T, assembling a different instruction than written
**Root cause:** get_neon_reg accepts Operand::Reg (empty arrangement) at neon.rs:14-18; encode_neon_mul discards source T; parse_reg_num accepts x/w prefixes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require a non-empty V-prefixed arrangement on every operand
```rust
        Some(Operand::Reg(_)) => {
            Err(format!("expected NEON register with arrangement at operand {idx}"))
        }
```
**Bug report:** bug_reports/encode_neon_mul_bare_src.md
**Repro seed:** rd=0, rn=0, rm=0, kind=1, fp_prefix="x"
**Raw output:** Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects mul v0.8b, v0, v0.8b). minimal failing input: rd = 0, rn = 0, rm = 0, kind = 1, fp_prefix = "x"

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs | 10 properties + 1 KAT + 6 regressions |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_mul_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_mul -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mul_regression_extra_operand -- --test-threads=1
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mul_regression_mismatched_t -- --test-threads=1
```

B3 reserved T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mul_regression_reserved_1d -- --test-threads=1
```

B4 bare src:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_mul_regression_bare_src -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_mul_extra_operand.md
- pbt-out/bug_reports/encode_neon_mul_extra_operand.html
- pbt-out/bug_reports/encode_neon_mul_mismatched_t.md
- pbt-out/bug_reports/encode_neon_mul_mismatched_t.html
- pbt-out/bug_reports/encode_neon_mul_reserved_t.md
- pbt-out/bug_reports/encode_neon_mul_reserved_t.html
- pbt-out/bug_reports/encode_neon_mul_bare_src.md
- pbt-out/bug_reports/encode_neon_mul_bare_src.html
- pbt-out/run/encode_neon_mul_pbt.log
- pbt-out/run/encode_neon_mul_pbt_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 16:58 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 129/289 total | PBT candidates: 129 | Tested: 129 (100%) | 0 pass, 129 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 129 |
| **Tested (of PBT candidates)** | **129 / 129 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 129 / 0 |
| **Overall (tested / all functions)** | **129 / 289 (45%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 129 | 129 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 129 | 129 | 0 | 100% |

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
| neon.rs | 68 | 44 | 44 | 100% | covered |
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
