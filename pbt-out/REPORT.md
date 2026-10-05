# PBT Campaign Report: encode_neon_zip_uzp

## Summary

**Verdict:** 4 medium: encode_neon_zip_uzp silently encodes extra operands, reserved 1D, mismatched arrangements, and bare/GPR sources that gas and llvm-mc reject, so invalid ZIP/UZP/TRN assembly becomes a 32-bit word.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_zip_uzp
**Tests:** 9
**Result:** 5 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps found no .gcda/.profraw (build tree not instrumented / C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of encode_neon_zip_uzp: arity Err, get_neon_reg Err, neon_arr_to_q_size Err, Ok Word all driven.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_zip_uzp | 9 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_zip_uzp ignores a fourth operand

**Formal:** ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm,extra ∈ {0..31}. llvm-mc(mnemonic Vd.T,Vn.T,Vm.T,Vextra.T)=Err ⇒ encode_neon_zip_uzp([Vd.T,Vn.T,Vm.T,Vextra.T], opc(mnemonic), false)=Err
**Contract evidence:** inferred (README.md:12 gas-compatibility; llvm-mc/gas reject a fourth operand; neon.rs:1096 "uzp/zip requires 3 operands")
**Documentation conflict:** neon.rs:1096 `return Err("uzp/zip requires 3 operands".to_string());` — the check is `operands.len() < 3`, so extra operands are not rejected. The comment does not declare extra valid; gas/README require exactly 3.
**Severity:** medium
**Counterexample:** encode_neon_zip_uzp([v0.8b, v0.8b, v0.8b, v0.8b], 0b011, false)
**Expected / Actual:** Err / Ok(Word(0x0e003800))
**Impact:** Invalid four-operand ZIP/UZP/TRN is assembled as the three-operand form, dropping the extra operand.
**Root cause:** neon.rs:1095 uses `operands.len() < 3`, so operands after the first three are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1095`
```rust
    if operands.len() < 3 {
        return Err("uzp/zip requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("uzp/zip requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_zip_uzp_extra_operand.md
**Repro seed:** cc 9b67a6acb5b9b4868037f953a8ad0e2ded32eb53d3ba488ec80820016f5bf472
**Raw output:**
```text
Test failed: 4 operands must Err (llvm-mc rejects zip1 v0.8b, v0.8b, v0.8b, v0.8b)
minimal failing input: rd = 0, rn = 0, rm = 0, extra = 0, t = "8b", m = "zip1"
```

### B2: encode_neon_zip_uzp encodes reserved 1D arrangement

**Formal:** ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, rd,rn,rm ∈ {0..31}. llvm-mc(mnemonic Vd.1d,Vn.1d,Vm.1d)=Err ⇒ encode_neon_zip_uzp([Vd.1d,Vn.1d,Vm.1d], opc(mnemonic), false)=Err
**Contract evidence:** inferred (ARM Advanced SIMD permute size:Q=11:0 reserved; README.md:12 gas-compatibility; llvm-mc/gas reject `*.1d`)
**Documentation conflict:** (none) — neon_arr_to_q_size accepts "1d"; the function comment does not list 1D as valid
**Severity:** medium
**Counterexample:** encode_neon_zip_uzp([v0.1d, v0.1d, v0.1d], 0b011, false)
**Expected / Actual:** Err / Ok(Word(0x0ec03800))
**Impact:** Reserved 1D permute encodings are emitted for assembly that gas and llvm-mc reject.
**Root cause:** neon.rs:1101 uses neon_arr_to_q_size which maps "1d" to (Q=0, size=11) with no reserved-pair check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1101`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size:Q = 11:0 after decoding Q/size
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 && q == 0 {
        return Err("uzp/zip: 1d arrangement is reserved".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_zip_uzp_reserved_1d.md
**Repro seed:** (deterministic; rd=rn=rm=0, m=zip1)
**Raw output:**
```text
Test failed: reserved 1d must Err (llvm-mc/gas reject zip1 v0.1d, v0.1d, v0.1d)
minimal failing input: rd = 0, rn = 0, rm = 0, m = "zip1"
```

### B3: encode_neon_zip_uzp ignores mismatched source arrangements

**Formal:** ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, Td,Tn,Tm ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm ∈ {0..31}. (Td≠Tn ∨ Td≠Tm) ⇒ llvm-mc(mnemonic Vd.Td,Vn.Tn,Vm.Tm)=Err ∧ encode_neon_zip_uzp([Vd.Td,Vn.Tn,Vm.Tm], opc(mnemonic), false)=Err
**Contract evidence:** inferred (ARM permute matching arrangements; README.md:12 gas-compatibility; llvm-mc/gas reject mismatched T)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_zip_uzp([v0.8b, v0.8b, v0.16b], 0b011, false)
**Expected / Actual:** Err / Ok(Word(0x0e003800))
**Impact:** `zip1 v0.8b, v0.8b, v0.16b` is assembled as `zip1 v0.8b, v0.8b, v0.8b`.
**Root cause:** neon.rs:1099-1100 discard source arrangements (`let (rn, _)`, `let (rm, _)`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1099`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
    let (rm, _) = get_neon_reg(operands, 2)?;
```
**Suggested fix:** Require source arrangements to match dest T
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    let (rm, arr_m) = get_neon_reg(operands, 2)?;
    if arr_n != arr_d || arr_m != arr_d {
        return Err(format!("uzp/zip: mismatched arrangements {arr_d}/{arr_n}/{arr_m}"));
    }
```
**Bug report:** bug_reports/encode_neon_zip_uzp_mismatched_t.md
**Repro seed:** (deterministic; td=8b, tn=8b, tm=16b, m=zip1)
**Raw output:**
```text
Test failed: mismatched T must Err (llvm-mc/gas reject zip1 v0.8b, v0.8b, v0.16b)
minimal failing input: rd = 0, rn = 0, rm = 0, td = "8b", tn = "8b", tm = "16b", m = "zip1"
```

### B4: encode_neon_zip_uzp encodes a bare V or GPR source

**Formal:** ∀ mnemonic ∈ {zip1,zip2,uzp1,uzp2,trn1,trn2}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, rd,rn,rm ∈ {0..31}, kind ∈ {gpr_x, gpr_w, bare_v, fp_d, fp_s}. llvm-mc(non-arranged asm)=Err ⇒ encode_neon_zip_uzp(non-arranged ops, opc(mnemonic), false)=Err
**Contract evidence:** inferred (README.md:12 gas-compatibility; llvm-mc/gas require Vn.T / Vm.T)
**Documentation conflict:** (none) — dest as Operand::Reg already Errs via empty arrangement; source as Operand::Reg encodes because arrangements are discarded
**Severity:** medium
**Counterexample:** encode_neon_zip_uzp([v0.8b, Reg("v0"), v0.8b], 0b011, false)
**Expected / Actual:** Err / Ok(Word(0x0e003800))
**Impact:** `zip1 v0.8b, v0, v0.8b` (and `xN` as Rm) is assembled as a valid three-register permute.
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement; encode_neon_zip_uzp discards source arrangements.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:14`
```rust
        Some(Operand::Reg(name)) => {
            let num = parse_reg_num(name)
                .ok_or_else(|| format!("invalid register: {}", name))?;
            Ok((num, String::new()))
        }
```
**Suggested fix:** Require RegArrangement on every ZIP/UZP/TRN operand
```rust
        Some(Operand::Reg(_)) => {
            Err(format!("expected NEON register with arrangement at operand {}", idx))
        }
```
**Bug report:** bug_reports/encode_neon_zip_uzp_bare_src.md
**Repro seed:** cc 54bd80ef4ef3b6b1d2f0d671f69f2967a737d47ca19202db0b7c3c3c3ef0e386
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects zip1 v0.8b, v0, v0.8b)
minimal failing input: rd = 0, rn = 0, rm = 0, t = "8b", m = "zip1", kind = 1, fp_prefix = "x"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.rs | 9 properties + 1 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_neon_zip_uzp_pbt;` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_zip_uzp -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_extra_operand -- --test-threads=1 --exact
```

B2 reserved 1d:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_reserved_1d -- --test-threads=1 --exact
```

B3 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_mismatched_t -- --test-threads=1 --exact
```

B4 bare source:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_zip_uzp_regression_bare_src -- --test-threads=1 --exact
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
- pbt-out/bug_reports/encode_neon_zip_uzp_extra_operand.md
- pbt-out/bug_reports/encode_neon_zip_uzp_extra_operand.html
- pbt-out/bug_reports/encode_neon_zip_uzp_reserved_1d.md
- pbt-out/bug_reports/encode_neon_zip_uzp_reserved_1d.html
- pbt-out/bug_reports/encode_neon_zip_uzp_mismatched_t.md
- pbt-out/bug_reports/encode_neon_zip_uzp_mismatched_t.html
- pbt-out/bug_reports/encode_neon_zip_uzp_bare_src.md
- pbt-out/bug_reports/encode_neon_zip_uzp_bare_src.html
- pbt-out/run/encode_neon_zip_uzp.log
- pbt-out/run/encode_neon_zip_uzp_round2.log
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_zip_uzp_pbt.txt (proptest failure corpus; left in-tree as the framework writes it)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 13:14 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 118/289 total | PBT candidates: 118 | Tested: 118 (100%) | 0 pass, 118 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 118 |
| **Tested (of PBT candidates)** | **118 / 118 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 118 / 0 |
| **Overall (tested / all functions)** | **118 / 289 (41%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 118 | 118 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 118 | 118 | 0 | 100% |

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
| neon.rs | 68 | 33 | 33 | 100% | covered |
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
