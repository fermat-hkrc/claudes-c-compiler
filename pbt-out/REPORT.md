# PBT Campaign Report: encode_fcvt_from_int

## Summary

**Verdict:** 2 medium, 1 low: encode_fcvt_from_int silently ignores a 4th operand, maps a non-rounding-mode 3rd operand to rm=DYN, and encodes omitted rm as DYN on FCVT.D.W/WU where llvm-mc encodes RNE.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fcvt_from_int
**Tests:** 8
**Result:** 5 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and encode_fcvt_from_int as NOT LINKED). Rust `cargo test --lib encode_fcvt_from_int` executed the production symbol; sweep was a manual audit of documented 2-op / rm / R-type / ABI / dyn-default / arity-class / extra / non-rm-3rd behaviors. Tier: standard.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fcvt_from_int | 8 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_fcvt_from_int ignores a 4th operand

**Formal:** ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs, ∀ extra ∈ Operand. encode_fcvt_from_int([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], f7, rs2) is Err
**Contract evidence:** inferred (llvm-mc rejects extra operands with "invalid operand for instruction"; encode_instruction at encoder/mod.rs:751-754 and 779-782 passes the operand slice through; RISC-V FCVT.from_int form is at most three operands)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fcvt_from_int([Reg("f0"), Reg("x0"), RoundingMode("rne"), Imm(0)], 0b1101000, 0)
**Expected / Actual:** Err / Ok(Word(0xd0000053)) — encoding of fcvt.s.w f0, x0, rne
**Impact:** Malformed `fcvt.s.w f0, x0, rne, 0` still assembles; a typo or extra token is silently dropped.
**Root cause:** float.rs:135 only tests `operands.len() > 2` to read an optional rm and never rejects `operands.len() > 3`, so a 4th operand is ignored and line 142 still returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:135`
```rust
    let rm = if operands.len() > 2 {
```
**Suggested fix:** Reject more than three operands before packing the R-type word.
```rust
    if operands.len() > 3 {
        return Err("fcvt from int: unexpected extra operand".to_string());
    }
    let rm = if operands.len() > 2 {
```
**Bug report:** bug_reports/encode_fcvt_from_int_extra_operand.md
**Repro seed:** cc b24ad28eb08d23cee68be7724b1d6d22f31cc2327fcbc8e063ebba97152d052c
**Raw output:**
```text
Test failed: 4th operand must Err for fcvt.s.w f0, x0, rne (llvm-mc rejects extra operands); got Ok(Word(3489661011)) at src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs:541.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.s.w",
    104,
    0,
), rd = "f0", rs1 = "x0", extra = Imm(
    0,
)
```

### B2: encode_fcvt_from_int treats a non-rounding-mode 3rd operand as DYN

**Formal:** ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs, ∀ extra ∈ Operand \ RoundingMode. encode_fcvt_from_int([Reg(rd), Reg(rs1), extra], f7, rs2) is Err
**Contract evidence:** inferred (parser.rs:41 closed set of rounding modes; llvm-mc rejects a non-rm 3rd with "operand must be a valid floating point rounding mode mnemonic"; optional 3rd is only rm)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fcvt_from_int([Reg("f0"), Reg("x0"), Imm(0)], 0b1101000, 0)
**Expected / Actual:** Err / Ok(Word(0xd0007053)) — encoding of fcvt.s.w f0, x0 with rm=DYN
**Impact:** `fcvt.s.w f0, x0, 0` still encodes as `fcvt.s.w f0, x0` with rm=DYN. A mistaken 3rd token is silently reinterpreted as dynamic rounding.
**Root cause:** float.rs:137 maps every non-RoundingMode 3rd operand to rm=0b111 (dynamic) instead of returning Err, then line 142 still packs a valid OP-FP word.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:137`
```rust
            _ => 0b111,
```
**Suggested fix:** Return Err on a 3rd operand that is not a RoundingMode.
```rust
            other => {
                return Err(format!(
                    "fcvt from int: expected rounding mode, got {:?}",
                    other
                ))
            }
```
**Bug report:** bug_reports/encode_fcvt_from_int_non_rm_third.md
**Repro seed:** (deterministic regression; proptest also shrinks to Imm(0))
**Raw output:**
```text
Test failed: 3rd non-RoundingMode operand must Err for fcvt.s.w f0, x0 (optional rm only); got Ok(Word(3489689683)) at src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs:559.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.s.w",
    104,
    0,
), rd = "f0", rs1 = "x0", extra = Imm(
    0,
)
```

### B3: encode_fcvt_from_int encodes omitted rm as DYN on FCVT.D.W/WU; llvm-mc encodes RNE

**Formal:** ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs. llvm-mc(mnemonic rd, rs1) = encode_fcvt_from_int([Reg(rd), Reg(rs1)], funct7(mnemonic), rs2(mnemonic)) as Word
**Contract evidence:** inferred (llvm-mc 2-operand encoding of the same mnemonic is the independent assembler reference for this encoder; encode_instruction at encoder/mod.rs:779-780 dispatches fcvt.d.w/wu through this function)
**Documentation conflict:** (none)
**Severity:** low
**Counterexample:** encode_fcvt_from_int([Reg("f0"), Reg("x0")], 0b1101001, 0) vs llvm-mc("fcvt.d.w f0, x0")
**Expected / Actual:** 0xd2000053 (llvm-mc, rm=RNE) / 0xd2007053 (SUT, rm=DYN)
**Impact:** `fcvt.d.w f0, x0` object encoding disagrees with llvm-mc. int32→double is exact, so the numerical result is unchanged.
**Root cause:** float.rs:141 always uses 0b111 (DYN) when operands.len() <= 2, including FCVT.D.W/WU where llvm-mc hardwires RNE.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:141`
```rust
        0b111
```
**Suggested fix:** For FCVT.D.W/WU (funct7=1101001 and rs2 in {0,1}), encode omitted rm as RNE to match llvm-mc.
```rust
        if funct7 == 0b1101001 && (rs2 == 0 || rs2 == 1) {
            0b000
        } else {
            0b111
        }
```
**Bug report:** bug_reports/encode_fcvt_from_int_dw_omitted_rm.md
**Repro seed:** cc 5ebc5e119cbe7373544e0fc0dcb566833ba31e285dcc60e42c365a825051fddd
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `3523244115`,
 right: `3523215443`: SUT d2007053 != llvm-mc d2000053 for fcvt.d.w f0, x0 at src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs:412.
minimal failing input: (mn, f7, rs2) = (
    "fcvt.d.w",
    105,
    0,
), rd = "f0", rs1 = "x0"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs | 8 properties + 4 KAT + 3 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fcvt_from_int -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fcvt_from_int_regression_extra_operand -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fcvt_from_int_regression_non_rm_third -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fcvt_from_int_regression_dw_omitted_rm -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/report.json — machine-readable report
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/FUNCTION_INDEX.md — function index
- pbt-out/bug_reports/encode_fcvt_from_int_extra_operand.md
- pbt-out/bug_reports/encode_fcvt_from_int_extra_operand.html
- pbt-out/bug_reports/encode_fcvt_from_int_non_rm_third.md
- pbt-out/bug_reports/encode_fcvt_from_int_non_rm_third.html
- pbt-out/bug_reports/encode_fcvt_from_int_dw_omitted_rm.md
- pbt-out/bug_reports/encode_fcvt_from_int_dw_omitted_rm.html
- pbt-out/run/ — scratch directory for test runs
- proptest-regressions/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.txt — saved failing seed

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 20:03 (campaign: coverage)
> Files: 14/14 scanned (100%) | Functions: 209/351 total | PBT candidates: 209 | Tested: 209 (100%) | 1 pass, 209 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 14 / 14 (100%) |
| Total functions (all files) | 351 |
| PBT candidates (from FUNCTION_INDEX) | 209 |
| **Tested (of PBT candidates)** | **209 / 209 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 209 / -1 |
| **Overall (tested / all functions)** | **209 / 351 (60%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 209 | 209 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 209 | 209 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 19 | 19 | 100% | covered |
| neon.rs | 68 | 63 | 63 | 100% | covered |
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
| encode_fnmadd_fnmsub | fp_scalar.rs |
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
| encode_neon_faddp | neon.rs |
| encode_neon_scalar_three_same | neon.rs |
| encode_neon_scalar_addp | neon.rs |
| encode_neon_scalar_two_misc | neon.rs |
| encode_neon_scalar_qshrn | neon.rs |
| encode_neon_two_misc_narrow | neon.rs |
| encode_dmb | system.rs |
| encode_dsb | system.rs |
| encode_mrs | system.rs |
| encode_msr | system.rs |
| encode_svc | system.rs |
| encode_hvc | system.rs |
| encode_brk | system.rs |
| encode_hint | system.rs |
| encode_bti | system.rs |
| encode_ic | system.rs |
| encode_dc | system.rs |
| encode_sys | system.rs |
| encode_at | system.rs |
| encode_tlbi | system.rs |
| encode_swp | load_store.rs |
| encode_ldop | load_store.rs |
| encode_stop | load_store.rs |
| encode_tst | compare_branch.rs |
| encode_tbz | compare_branch.rs |
| encode_crc32 | bitfield.rs |
| encode_smaddl | data_processing.rs |
| encode_mneg | data_processing.rs |
| encode_sxtb | data_processing.rs |
| encode_uxth | data_processing.rs |
| encode_uxtb | data_processing.rs |
| encode_ldr_str | load_store.rs |
| encode_ldp_stp | load_store.rs |
| encode_ldnp_stnp | load_store.rs |
| encode_adrp | load_store.rs |
| encode_mov | data_processing.rs |
| encode_cond_branch | compare_branch.rs |
| encode_ldr_str_auto | load_store.rs |
| encode_lui | base.rs |
| encode_auipc | base.rs |
| encode_jal | base.rs |
| encode_jalr | base.rs |
| encode_branch_instr | base.rs |
| encode_load | base.rs |
| encode_store | base.rs |
| encode_alu_imm | base.rs |
| encode_alu_reg | base.rs |
| encode_alu_imm_w | base.rs |
| encode_alu_reg_w | base.rs |
| encode_csr | system.rs |
| encode_fence | system.rs |
| encode_amo | atomics.rs |
| encode_lr | atomics.rs |
| encode_sc | atomics.rs |
| encode_sfence_vma | system.rs |
| encode_csri | system.rs |
| encode_float_load | float.rs |
| encode_float_store | float.rs |
| encode_fp_arith | float.rs |
| encode_fp_arith_d | float.rs |
| encode_fp_unary | float.rs |
| encode_fp_sgnj | float.rs |
| encode_fp_cmp | float.rs |
| encode_fclass | float.rs |
| encode_fcvt_int | float.rs |
| encode_fcvt_from_int | float.rs |
