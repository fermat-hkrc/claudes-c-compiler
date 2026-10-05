# PBT Campaign Report: encode_neon_three_same

## Summary

**Verdict:** 5 medium: encode_neon_three_same silently encodes invalid GNU three-same assembly — extra operands, mismatched T, reserved `.1d`, bare Vn, and GPR dest `x0.8b` — so the assembler emits a word llvm-mc rejects.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_three_same
**Tests:** 10 properties + 1 KAT + 5 regression witnesses
**Result:** 5 passing, 5 failing properties, 5 bugs
**Change surface:** 1 changed function (encode_neon_three_same), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of encode_neon_three_same: arity < 3, get_neon_reg dest/src, neon_arr_to_q_size, Ok Word all driven. Recorded as file-level because native line coverage was absent.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_three_same | 10 properties (5 pass / 5 fail) + 1 KAT + 5 regressions | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_neon_three_same ignores a fourth operand

**Formal:** ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ three_same_t, insn ∈ InsnTable. llvm-mc(mnemonic Vd.T,Vn.T,Vm.T,Vextra.T) = Err ⇒ encode_neon_three_same([Vd.T,Vn.T,Vm.T,Vextra.T], U, opcode) = Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembler; llvm-mc rejects a fourth operand; documented layout is three registers)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_three_same([v0.8b, v0.8b, v0.8b, v0.8b], u=1, opcode=0b10001) then `cmeq v0.8b, v0.8b, v0.8b, v0.8b`
**Expected / Actual:** Err / Ok(Word) for the first three operands
**Impact:** Invalid assembly is assembled; extra operands are dropped.
**Root cause:** neon.rs:66 checks only `operands.len() < 3`, so slots beyond index 2 are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:66`
```rust
    if operands.len() < 3 {
        return Err("NEON three-same requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject `operands.len() != 3`.
```rust
    if operands.len() != 3 {
        return Err("NEON three-same requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_three_same_extra_operand.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread '...test_encode_neon_three_same_regression_extra_operand' panicked at encode_neon_three_same_pbt.rs:489:5:
cmeq v0.8b, v0.8b, v0.8b, v0.8b must Err (gas/llvm-mc reject a fourth operand)
```

### B2: encode_neon_three_same ignores source arrangements

**Formal:** ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ Arr ∪ {1d,1q}, insn ∈ InsnTable. ¬(valid_T(Td) ∧ Td=Tn=Tm) ⇒ encode_neon_three_same([Vd.Td,Vn.Tn,Vm.Tm], U, opcode) = Err
**Contract evidence:** inferred (ARM three-same matching T; README.md:12; llvm-mc rejects `cmeq v0.8b, v0.16b, v0.8b`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_three_same([v0.8b, v0.16b, v0.8b], u=1, opcode=0b10001)
**Expected / Actual:** Err / Ok(Word) using dest T only
**Impact:** Mismatched T encodes as if all three matched Vd.T.
**Root cause:** neon.rs:70-71 bind `_arr_n` / `_arr_m` and never compare them to `arr_d`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:70`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require `arr_n == arr_d && arr_m == arr_d`.
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("NEON three-same arrangement mismatch: dest {arr_d}, src {arr_n}/{arr_m}"));
    }
```
**Bug report:** bug_reports/encode_neon_three_same_mismatched_t.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread '...test_encode_neon_three_same_regression_mismatched_t' panicked at encode_neon_three_same_pbt.rs:499:5:
cmeq v0.8b, v0.16b, v0.8b must Err (ARM/gas/llvm-mc require matching T)
```

### B3: encode_neon_three_same encodes reserved .1d

**Formal:** ∀ rd,rn,rm ∈ {0..31}, insn ∈ InsnTable. encode_neon_three_same([Vd.1d,Vn.1d,Vm.1d], U, opcode) = Err
**Contract evidence:** inferred (ARM ARM Advanced SIMD three-same size:Q=11:0 Reserved; llvm-mc rejects `.1d`; function doc lists CMEQ/UQSUB/SQSUB/CMHI which do not support 1D)
**Documentation conflict:** (none) — neon_arr_to_q_size lists `"1d"` as a general NEON arrangement, not a three-same domain restriction
**Severity:** medium
**Counterexample:** encode_neon_three_same([v0.1d, v0.1d, v0.1d], u=1, opcode=0b10001)
**Expected / Actual:** Err / Ok(Word) with Q=0 size=11
**Impact:** Reserved encodings are emitted for integer three-same mnemonics.
**Root cause:** neon.rs:73 uses neon_arr_to_q_size, which accepts `"1d"` with no Reserved check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:73`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject Q=0 && size=11.
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if q == 0 && size == 0b11 {
        return Err(format!("NEON three-same reserved arrangement: {arr_d}"));
    }
```
**Bug report:** bug_reports/encode_neon_three_same_reserved_1d.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread '...test_encode_neon_three_same_regression_reserved_1d' panicked at encode_neon_three_same_pbt.rs:509:5:
cmeq v0.1d, v0.1d, v0.1d must Err (ARM size:Q=11:0 reserved; llvm-mc rejects)
```

### B4: encode_neon_three_same accepts a bare Vn without arrangement

**Formal:** ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr_dest, bare_vn, bare_vm, fp_dest}. llvm-mc rejects ⇒ encode_neon_three_same(ops, U, opcode) = Err
**Contract evidence:** inferred (GNU/llvm-mc require Vn.T; parser.rs documents RegArrangement as v0.8b)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_three_same([v0.8b, Reg("v0"), v0.8b], u=1, opcode=0b10001)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Bare `v0` is treated as a numbered NEON register with dest T.
**Root cause:** neon.rs:70-71 call get_neon_reg (accepts Operand::Reg) then discard the empty arrangement.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:70`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
    let (rm, _arr_m) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Reject an empty source arrangement.
```rust
    if arr_n.is_empty() || arr_m.is_empty() {
        return Err("NEON three-same requires Vn.T and Vm.T".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_three_same_bare_src.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread '...test_encode_neon_three_same_regression_bare_src' panicked at encode_neon_three_same_pbt.rs:519:5:
cmeq v0.8b, v0, v0.8b must Err (gas/llvm-mc require Vn.T)
```

### B5: encode_neon_three_same encodes a GPR name as Vd

**Formal:** ∀ rd ∈ {0..31}. encode_neon_three_same([RegArrangement{x{rd}, 8b}, v0.8b, v0.8b], U, opcode) = Err
**Contract evidence:** inferred (three-same Vd is a NEON V register; llvm-mc rejects `cmeq x0.8b, …`; parse_reg_num accepts `x`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_three_same([RegArrangement{reg:"x0", arrangement:"8b"}, v0.8b, v0.8b], u=1, opcode=0b10001)
**Expected / Actual:** Err / Ok(Word) identical to `cmeq v0.8b, v0.8b, v0.8b`
**Impact:** GPR names encode as V registers with the same number.
**Root cause:** neon.rs:69 uses get_neon_reg → parse_reg_num, which accepts prefix `x`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:69`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a `v` prefix on three-same register names.
```rust
    // get_neon_reg / parse_reg_num must require a 'v' prefix for three-same Vd/Vn/Vm
```
**Bug report:** bug_reports/encode_neon_three_same_gpr_dest.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread '...test_encode_neon_three_same_regression_gpr_dest' panicked at encode_neon_three_same_pbt.rs:536:5:
cmeq x0.8b, v0.8b, v0.8b must Err (gas/llvm-mc require Vd.T)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs | 10 properties + 1 KAT + 5 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_three_same -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_extra_operand -- --test-threads=1
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_mismatched_t -- --test-threads=1
```

B3 reserved 1d:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_reserved_1d -- --test-threads=1
```

B4 bare src:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_bare_src -- --test-threads=1
```

B5 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_three_same_regression_gpr_dest -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_three_same_extra_operand.md
- pbt-out/bug_reports/encode_neon_three_same_extra_operand.html
- pbt-out/bug_reports/encode_neon_three_same_mismatched_t.md
- pbt-out/bug_reports/encode_neon_three_same_mismatched_t.html
- pbt-out/bug_reports/encode_neon_three_same_reserved_1d.md
- pbt-out/bug_reports/encode_neon_three_same_reserved_1d.html
- pbt-out/bug_reports/encode_neon_three_same_bare_src.md
- pbt-out/bug_reports/encode_neon_three_same_bare_src.html
- pbt-out/bug_reports/encode_neon_three_same_gpr_dest.md
- pbt-out/bug_reports/encode_neon_three_same_gpr_dest.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 18:12 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 133/289 total | PBT candidates: 133 | Tested: 133 (100%) | 0 pass, 133 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 133 |
| **Tested (of PBT candidates)** | **133 / 133 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 133 / 0 |
| **Overall (tested / all functions)** | **133 / 289 (46%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 133 | 133 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 133 | 133 | 0 | 100% |

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
| neon.rs | 68 | 48 | 48 | 100% | covered |
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
