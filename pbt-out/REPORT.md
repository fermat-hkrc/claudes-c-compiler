# PBT Campaign Report: encode_fp_arith_d

## Summary

**Verdict:** 2 medium: encode_fp_arith_d silently accepts a 5th operand and a non-rounding-mode 4th operand, so malformed `fadd.d`/`fsub.d`/`fmul.d`/`fdiv.d` still assembles as a valid OP-FP D-extension word instead of returning Err.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fp_arith_d
**Tests:** 8
**Result:** 6 passing, 2 bugs
**Change surface:** 1 changed function (encode_fp_arith_d), 1 with a property, 0 error-handling changes
**Coverage evidence:** none — no .gcda/.profraw (the build was not instrumented / reporter is missing). File-level reporter listed unrelated C++ binaries and marked encode_fp_arith_d NOT LINKED; that is a reporter gap, not an untested function — `cargo test --lib encode_fp_arith_d_pbt` executed the symbol (5 KAT + 6 passing properties + 2 failing). Sweep: 1 round, manual audit of 3-op / rm / R-type+D-fmt / ABI / dyn-default / arity-GPR / extra / non-rm 4th. Closed: tier round spent; remaining documented gaps are the two filed bugs.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fp_arith_d | 8 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_fp_arith_d ignores a 5th operand

**Formal:** ∀ mn ∈ {fadd.d,fsub.d,fmul.d,fdiv.d}, ∀ rd,rs1,rs2 ∈ FPRegs, ∀ extra. encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2),RoundingMode("rne"), extra], funct7(mn)) is Err
**Contract evidence:** inferred (llvm-mc rejects extra operands as "invalid operand for instruction"; encoder/mod.rs:749-752 passes the operand slice through unchanged; encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fp_arith_d([Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 1)
**Expected / Actual:** Err / Ok(Word(33554515)) = 0x02000053 (fadd.d f0, f0, f0, rne)
**Impact:** Malformed `fadd.d f0, f0, f0, rne, 0` still assembles as `fadd.d f0, f0, f0, rne`. A typo or extra token is silently dropped.
**Root cause:** float.rs:66 (callee encode_fp_arith, forwarded by encode_fp_arith_d) only tests `operands.len() > 3` to read optional rm and never rejects `operands.len() > 4`, so line 74 still returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:66`
```rust
    let rm = if operands.len() > 3 {
```
**Suggested fix:** Reject more than four operands before packing the R-type word.
```rust
    if operands.len() > 4 {
        return Err("fp arith: unexpected extra operand".to_string());
    }
    let rm = if operands.len() > 3 {
```
**Bug report:** bug_reports/encode_fp_arith_d_trailing_operand.md
**Repro seed:** cc 612dbcdaff62455438fde85085475af720926c10e7be08b849684daf860bd803
**Raw output:**
```text
Test failed: 5th operand must Err for fadd.d f0, f0, f0, rne (llvm-mc rejects extra operands); got Ok(Word(33554515)) at src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs:504.
minimal failing input: (mn, f7) = (
    "fadd.d",
    1,
), rd = "f0", rs1 = "f0", rs2 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_fp_arith_d treats a non-rounding-mode 4th operand as DYN

**Formal:** ∀ rd,rs1,rs2 ∈ FPRegs, ∀ extra ∉ RoundingMode, ∀ funct7 ∈ {1,5,9,13}. encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2), extra], funct7) is Err
**Contract evidence:** inferred (llvm-mc "operand must be a valid floating point rounding mode mnemonic"; parser.rs:41 closed RM set {rne,rtz,rdn,rup,rmm,dyn}; float.rs:65 "Check for optional rounding mode" documents an optional RM, not an arbitrary 4th operand)
**Documentation conflict:** (none) — float.rs:69 `_ => 0b111, // dynamic` is the producing statement, not a contract that a non-RM 4th operand is valid
**Severity:** medium
**Counterexample:** encode_fp_arith_d([Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 1)
**Expected / Actual:** Err / Ok(Word(33583187)) = 0x02007053 (fadd.d f0, f0, f0 with rm=DYN)
**Impact:** `fadd.d f0, f0, f0, 0` still encodes as `fadd.d f0, f0, f0` with rm=DYN. A mistaken 4th token is silently reinterpreted as dynamic rounding.
**Root cause:** float.rs:69 (callee encode_fp_arith, forwarded by encode_fp_arith_d) maps every non-RoundingMode 4th operand to rm=0b111 (dynamic) instead of returning Err, then line 74 still packs a valid OP-FP word.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/float.rs:69`
```rust
            _ => 0b111, // dynamic
```
**Suggested fix:** Return Err on a 4th operand that is not a RoundingMode.
```rust
            other => {
                return Err(format!(
                    "fp arith: expected rounding mode, got {:?}",
                    other
                ))
            }
```
**Bug report:** bug_reports/encode_fp_arith_d_non_rm_fourth.md
**Repro seed:** (deterministic regression; proptest seed cc 612dbcdaff62455438fde85085475af720926c10e7be08b849684daf860bd803 also shrinks the extra-operand case)
**Raw output:**
```text
Test failed: 4th non-RoundingMode operand must Err for fadd.d f0, f0, f0 (optional rm only); got Ok(Word(33583187)) at src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs:523.
minimal failing input: (mn, f7) = (
    "fadd.d",
    1,
), rd = "f0", rs1 = "f0", rs2 = "f0", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs | 8 properties + 5 KAT + 2 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fp_arith_d_pbt -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fp_arith_d_regression_extra_operand -- --test-threads=1
```

B2 non-RM fourth:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fp_arith_d_regression_non_rm_fourth -- --test-threads=1
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
- pbt-out/bug_reports/encode_fp_arith_d_trailing_operand.md
- pbt-out/bug_reports/encode_fp_arith_d_trailing_operand.html
- pbt-out/bug_reports/encode_fp_arith_d_non_rm_fourth.md
- pbt-out/bug_reports/encode_fp_arith_d_non_rm_fourth.html
- pbt-out/run/encode_fp_arith_d_pbt.log
- pbt-out/run/encode_fp_arith_d_test.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 18:28 (campaign: coverage)
> Files: 14/14 scanned (100%) | Functions: 203/351 total | PBT candidates: 203 | Tested: 203 (100%) | 1 pass, 203 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Files scanned | 14 / 14 (100%) |
| Total functions (all files) | 351 |
| PBT candidates (from FUNCTION_INDEX) | 203 |
| **Tested (of PBT candidates)** | **203 / 203 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 203 / -1 |
| **Overall (tested / all functions)** | **203 / 351 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 203 | 203 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 203 | 203 | 0 | 100% |

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
