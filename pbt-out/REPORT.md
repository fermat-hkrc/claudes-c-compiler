# PBT Campaign Report: encode_c_jr

## Summary

**Verdict:** 2 medium: encode_c_jr silently encodes `c.jr x0` as reserved halfword 0x8002, and extra operands are ignored instead of rejected.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_c_jr
**Tests:** 7 properties (plus 6 KAT + 2 regression witnesses)
**Result:** 5 passing, 2 bugs
**Change surface:** 1 changed function (encode_c_jr), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo tests are not the C++ reporter binaries) and listed encode_c_jr as NOT LINKED in those binaries. Manual audit of the documented C.JR surface (1-op CR-type / ABI / isolation / arity-FP / extra / rs1=x0) drove every contract; remaining gaps are the two filed bugs.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_c_jr | 7 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_c_jr encodes rs1=x0 as a reserved halfword

**Formal:** ∀ name ∈ {x0, zero}. llvm-mc rejects `c.jr name` ∧ encode_c_jr([Reg(name)])=Err
**Contract evidence:** inferred (RISC-V Unprivileged ISA: C.JR is only valid when rs1≠x0; the code point with rs1=x0 is reserved; llvm-mc rejects `c.jr x0`; one-operand mnemonic `c.jr rs1`; compress.rs:566 requires rs1 != 0)
**Documentation conflict:** (none) — compressed.rs:49 only states the mnemonic `c.jr rs1`; it does not declare rs1=x0 invalid on this function, nor admit a limitation
**Severity:** medium
**Counterexample:** encode_c_jr([Reg("x0")])
**Expected / Actual:** Err / Ok(Half(0x8002)) — reserved CR-type encoding (funct4=1000, rs1=0, rs2=0)
**Impact:** Handwritten or generated `c.jr x0` / `c.jr zero` becomes a reserved 16-bit encoding rather than an assembler error.
**Root cause:** compressed.rs:50-53 packs CR-type bits with no rs1≠0 check, so rs1=x0 falls into the reserved pattern (funct4=1000, rs1=0, rs2=0).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:51`
```rust
    let rs1 = get_reg(operands, 0)?;
    Ok(EncodeResult::Half(0b10 | ((rs1 as u16) << 7) | (0b100 << 13)))
```
**Suggested fix:** Reject rs1 == 0 before packing.
```rust
if rs1 == 0 {
    return Err("c.jr: rs1 cannot be x0 (that encoding is reserved)".into());
}
```
**Bug report:** bug_reports/encode_c_jr_rs1_x0.md
**Repro seed:** (deterministic regression; PBT shrunk to name="x0")
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::encode_c_jr_neg_rs1_x0' (2852478) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:253:1:
Test failed: rs1=x0 must Err (llvm-mc rejects; encoding is reserved); got Ok(Half(32770)) at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:313.
minimal failing input: name = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::test_encode_c_jr_regression_rs1_x0' (2852481) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:246:5:
c.jr x0 must Err (rs1=x0 is reserved); got Ok(Half(32770))
```

### B2: encode_c_jr ignores extra operands

**Formal:** ∀ rs1 ∈ GPR-names\{x0}, extra ∈ Operand. encode_c_jr([Reg(rs1), extra]) = Err
**Contract evidence:** inferred (one-operand mnemonic `c.jr rs1` at compressed.rs:49; llvm-mc rejects a second operand)
**Documentation conflict:** (none) — the comment names the one-operand form and does not declare extra operands invalid in so many words, nor admit a limitation
**Severity:** medium
**Counterexample:** encode_c_jr([Reg("x1"), Imm(0)])
**Expected / Actual:** Err / Ok(Half(0x8082)) — encoding of `c.jr x1` / `ret`
**Impact:** Accidental extra tokens are dropped, so a malformed instruction still assembles as a valid 16-bit jump.
**Root cause:** compressed.rs:50-53 reads only operands[0] via get_reg and never checks operands.len().
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:51`
```rust
    let rs1 = get_reg(operands, 0)?;
    Ok(EncodeResult::Half(0b10 | ((rs1 as u16) << 7) | (0b100 << 13)))
```
**Suggested fix:** Reject any operand list whose length is not exactly 1 before packing.
```rust
if operands.len() != 1 {
    return Err(format!("c.jr: expected 1 operand, got {}", operands.len()));
}
```
**Bug report:** bug_reports/encode_c_jr_extra_operand.md
**Repro seed:** cc 76196dcd79a75097626af2f2f10d1b194c259b99dee69d29bb8415247c6fc7ab
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_jr_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc 76196dcd79a75097626af2f2f10d1b194c259b99dee69d29bb8415247c6fc7ab

thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::encode_c_jr_neg_extra' (2852477) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:253:1:
Test failed: extra operand must Err for c.jr x1 (llvm-mc rejects extra operands); got Ok(Half(32898)) at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:325.
minimal failing input: rs1 = "x1", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_jr_pbt::test_encode_c_jr_regression_extra_operand' (2852480) panicked at src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs:235:5:
c.jr x1 with a second operand must Err; got Ok(Half(32898))
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs | 7 properties + 6 KAT + 2 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_jr -- --test-threads=1
```

B1 rs1=x0:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_jr_regression_rs1_x0 -- --test-threads=1
```

B2 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_jr_regression_extra_operand -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger (append encode_c_jr row)
- pbt-out/COVERAGE_STATUS.md — this-campaign coverage summary
- pbt-out/INVARIANTS.md — confirmed invariants for later campaigns
- pbt-out/report.json — machine-readable report
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/bug_reports/encode_c_jr_rs1_x0.md — B1
- pbt-out/bug_reports/encode_c_jr_rs1_x0.html — B1 HTML
- pbt-out/bug_reports/encode_c_jr_extra_operand.md — B2
- pbt-out/bug_reports/encode_c_jr_extra_operand.html — B2 HTML
- pbt-out/run/ — scratch dir for test CWD
- proptest-regressions/backend/riscv/assembler/encoder/encode_c_jr_pbt.txt — proptest failure cache

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 22:21 (campaign: coverage)
> Files: 15/15 scanned (100%) | Functions: 219/367 total | PBT candidates: 219 | Tested: 219 (100%) | 1 pass, 219 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 15 |
| Files scanned | 15 / 15 (100%) |
| Total functions (all files) | 367 |
| PBT candidates (from FUNCTION_INDEX) | 219 |
| **Tested (of PBT candidates)** | **219 / 219 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 219 / -1 |
| **Overall (tested / all functions)** | **219 / 367 (60%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 219 | 219 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 219 | 219 | 0 | 100% |

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
| encode_fcvt_fp | float.rs |
| encode_fmv_x_f | float.rs |
| encode_fmv_f_x | float.rs |
| encode_fma | float.rs |
| encode_c_lui | compressed.rs |
| encode_c_li | compressed.rs |
| encode_c_addi | compressed.rs |
| encode_c_mv | compressed.rs |
| encode_c_add | compressed.rs |
| encode_c_jr | compressed.rs |
