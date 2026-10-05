# PBT Campaign Report: encode_neon_bitwise_insert

## Summary

**Verdict:** 4 medium: encode_neon_bitwise_insert silently encodes a fourth operand, T outside {8B,16B}, mismatched source T, and GPR/SP/bare-V/FP as if they were valid BIT/BIF, so invalid assembly becomes wrong machine code.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_bitwise_insert
**Tests:** 9
**Result:** 5 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). The cargo test run executed encode_neon_bitwise_insert via encode_neon_bitwise_insert_pbt.rs.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_bitwise_insert | 9 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_bitwise_insert ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}. llvm-mc(mnem size four-ops) is Err ⇒ encode_neon_bitwise_insert([Vd.t,Vn.t,Vm.t,Vextra.t], size) is Err
**Contract evidence:** inferred (llvm-mc/gas reject a fourth operand; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_bitwise_insert([v0.8b, v0.8b, v0.8b, v0.8b], size=0b10)
**Expected / Actual:** Err / Ok(Word(0x2ea01c00))
**Impact:** Trailing junk after BIT/BIF is silently dropped, so a mistyped extra register does not fail the assemble
**Root cause:** neon.rs:1668 uses `operands.len() < 3`, so extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1668`
```rust
    if operands.len() < 3 {
        return Err("bit/bif requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("bit/bif requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_bitwise_insert_extra_operand.md
**Repro seed:** rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", size = 2
**Raw output:** Test failed: extra operand must Err (llvm-mc rejects bit v0.8b, v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:305.
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", size = 2

### B2: encode_neon_bitwise_insert encodes T outside {8B,16B}

**Formal:** ∀ rd,rn,rm ∈ {0..31}, ∀ t ∈ {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s}, ∀ size ∈ {0b10,0b11}. llvm-mc rejects mnem Vd.t,Vn.t,Vm.t ⇒ encode_neon_bitwise_insert([Vd.t,Vn.t,Vm.t], size) is Err
**Contract evidence:** inferred (ARM Advanced SIMD three-same BIT/BIF admit only T in {8B,16B}; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_bitwise_insert([v0.4h, v0.4h, v0.4h], size=0b10)
**Expected / Actual:** Err / Ok(Word) with Q=0 as if T were 8B
**Impact:** Invalid assembly such as `bit v0.4h, v0.4h, v0.4h` encodes as 8B BIT, producing wrong machine code instead of an assemble error
**Root cause:** neon.rs:1674 sets Q=1 iff dest arrangement is exactly "16b", else 0, with no check that T is 8b or 16b
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1674`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Accept only 8b/16b
```rust
    let q: u32 = match arr_d.as_str() {
        "8b" => 0,
        "16b" => 1,
        _ => return Err(format!("bit/bif: unsupported arrangement: {}", arr_d)),
    };
```
**Bug report:** bug_reports/encode_neon_bitwise_insert_invalid_t.md
**Repro seed:** rd = 0, rn = 0, rm = 0, t = "4h", size = 2
**Raw output:** Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects bit v0.4h, v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:327.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "4h", size = 2

### B3: encode_neon_bitwise_insert ignores mismatched source arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, ∀ td,tn,tm ∈ {8b,16b} not all equal, ∀ size ∈ {0b10,0b11}. llvm-mc rejects mismatched T ⇒ encode_neon_bitwise_insert([Vd.td,Vn.tn,Vm.tm], size) is Err
**Contract evidence:** inferred (llvm-mc/gas require matching T on Vd, Vn, Vm; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_bitwise_insert([v0.16b, v0.8b, v0.8b], size=0b10)
**Expected / Actual:** Err / Ok(Word) with Q from dest "16b" only
**Impact:** A width mismatch such as `bit v0.16b, v0.8b, v0.8b` encodes as 16B BIT, so the assemble does not fail
**Root cause:** neon.rs:1672–1673 discard source arrangements; only dest T is read for Q
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1672`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require Vn.T and Vm.T to equal Vd.T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err("bit/bif: operand arrangement mismatch".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_bitwise_insert_mismatch_t.md
**Repro seed:** rd = 0, rn = 0, rm = 0, td = "16b", tn = "8b", tm = "8b", size = 2
**Raw output:** Test failed: mismatched T must Err (llvm-mc rejects bit v0.16b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:352.
minimal failing input: rd = 0, rn = 0, rm = 0, td = "16b", tn = "8b", tm = "8b", size = 2

### B4: encode_neon_bitwise_insert accepts GPR, SP, bare V, and FP scalars

**Formal:** ∀ rd,rn,rm ∈ {0..31}, ∀ t ∈ {8b,16b}, ∀ size ∈ {0b10,0b11}, ∀ kind ∈ {x-gpr, w-dest, sp, bare-v, d-scalar, s-dest, q-dest}. llvm-mc rejects that form ⇒ encode_neon_bitwise_insert(ops(kind), size) is Err
**Contract evidence:** inferred (llvm-mc/gas require arranged NEON Vd.T, Vn.T, Vm.T; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_bitwise_insert([x0, x0, x0], size=0b10)
**Expected / Actual:** Err / Ok(Word) — empty arrangement yields Q=0
**Impact:** `bit x0, x0, x0` (and SP / bare V / scalar FP) encodes as 8B BIT, so a wrong register class is not diagnosed
**Root cause:** neon.rs:1671 calls get_neon_reg, which accepts Operand::Reg; empty arrangement then takes the Q=0 arm at neon.rs:1674
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1671`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Require RegArrangement with T in {8b,16b} on every operand
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("bit/bif: expected Vd.8b or Vd.16b, got {}", arr_d));
    }
```
**Bug report:** bug_reports/encode_neon_bitwise_insert_gpr_bare_sp.md
**Repro seed:** rd = 0, rn = 0, rm = 0, t = "8b", size = 2, kind = 0
**Raw output:** Test failed: non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects bit x0, x0, x0) at src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs:411.
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", size = 2, kind = 0

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_bitwise_insert_pbt.rs | 9 properties + 4 KAT + 6 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_bitwise_insert -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_extra_operand -- --test-threads=1 --exact
```

B2 invalid T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_invalid_t -- --test-threads=1 --exact
```

B3 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_mismatch_t -- --test-threads=1 --exact
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_bitwise_insert_regression_gpr_dest -- --test-threads=1 --exact
```

## Output Directories

pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/PROPERTIES.md, pbt-out/PLAN.md, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/bug_reports/encode_neon_bitwise_insert_extra_operand.md, pbt-out/bug_reports/encode_neon_bitwise_insert_extra_operand.html, pbt-out/bug_reports/encode_neon_bitwise_insert_invalid_t.md, pbt-out/bug_reports/encode_neon_bitwise_insert_invalid_t.html, pbt-out/bug_reports/encode_neon_bitwise_insert_mismatch_t.md, pbt-out/bug_reports/encode_neon_bitwise_insert_mismatch_t.html, pbt-out/bug_reports/encode_neon_bitwise_insert_gpr_bare_sp.md, pbt-out/bug_reports/encode_neon_bitwise_insert_gpr_bare_sp.html, pbt-out/run/ (test logs).

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 21:09 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 142/289 total | PBT candidates: 142 | Tested: 142 (100%) | 0 pass, 142 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 142 |
| **Tested (of PBT candidates)** | **142 / 142 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 142 / 0 |
| **Overall (tested / all functions)** | **142 / 289 (49%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 142 | 142 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 142 | 142 | 0 | 100% |

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
| neon.rs | 68 | 57 | 57 | 100% | covered |
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
