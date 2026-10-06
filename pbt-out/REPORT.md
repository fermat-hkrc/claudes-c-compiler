# PBT Campaign Report: encode_c_add

## Summary

**Verdict:** 1 high: encode_c_add silently encodes `c.add rd, x0` as C.JALR or C.EBREAK (so `c.add x1, x0` becomes `c.jalr x1`); 1 medium: extra operands are ignored instead of rejected.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_c_add
**Tests:** 7 properties (plus 6 KAT + 2 regression witnesses)
**Result:** 5 passing, 2 bugs
**Change surface:** 1 changed function (encode_c_add), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo tests are not the C++ reporter binaries) and listed encode_c_add as NOT LINKED in those binaries. Manual audit of the documented C.ADD surface (2-op CR-type / ABI / isolation / arity-FP / extra / rs2=x0) drove every contract; remaining gaps are the two filed bugs.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_c_add | 7 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_c_add encodes rs2=x0 as C.JALR / C.EBREAK

**Formal:** ∀ rd ∈ GPR-names. llvm-mc rejects `c.add rd, x0` ∧ encode_c_add([Reg(rd), Reg(x0)])=Err ∧ encode_c_add([Reg(rd), Reg(zero)])=Err
**Contract evidence:** inferred (RISC-V CR-type with funct4=1001 and rs2=x0 is C.JALR when rd≠0 and C.EBREAK when rd=0, not C.ADD; llvm-mc rejects `c.add x1, x0`; two-operand mnemonic `c.add rd, rs2`; compress.rs:200 requires rs2 != 0)
**Documentation conflict:** (none) — compressed.rs:42 only states the mnemonic `c.add rd, rs2`; it does not declare rs2=x0 invalid on this function, nor admit a limitation
**Severity:** high
**Counterexample:** encode_c_add([Reg("x1"), Reg("x0")]) then also the shrunk PBT witness encode_c_add([Reg("x0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Half(0x9082)) which is `c.jalr x1`; shrunk Ok(Half(0x9002)) C.EBREAK
**Impact:** Handwritten or generated `c.add rd, x0` / `c.add rd, zero` becomes a jump (C.JALR) or a breakpoint (C.EBREAK) instead of a register add, changing control flow.
**Root cause:** compressed.rs:43-47 packs CR-type bits with no rs2≠0 check, so rs2=x0 falls into the C.JALR / C.EBREAK pattern (funct4=1001, rs2=0).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:43`
```rust
pub(crate) fn encode_c_add(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Half(0b10 | ((rs2 as u16) << 2) | ((rd as u16) << 7) | (1 << 12) | (0b100 << 13)))
}
```
**Suggested fix:** Reject rs2 == 0 before packing.
```rust
if rs2 == 0 {
    return Err("c.add: rs2 cannot be x0 (that encoding is c.jalr / c.ebreak)".into());
}
```
**Bug report:** bug_reports/encode_c_add_rs2_x0.md
**Repro seed:** (deterministic regression; PBT shrunk to rd="x0")
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::encode_c_add_neg_rs2_x0' (2850635) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:268:1:
Test failed: rs2=x0 must Err (llvm-mc rejects; encoding is C.JALR/C.EBREAK); got Ok(Half(36866)) at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:335.
minimal failing input: rd = "x0"
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::test_encode_c_add_regression_rs2_x0' (2850645) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:261:5:
c.add x1, x0 must Err (rs2=x0 is C.JALR); got Ok(Half(36994))
```

### B2: encode_c_add ignores extra operands

**Formal:** ∀ rd ∈ GPR-names, rs2 ∈ GPR-names\{x0}, extra ∈ Operand. encode_c_add([Reg(rd), Reg(rs2), extra]) = Err
**Contract evidence:** inferred (two-operand mnemonic `c.add rd, rs2` at compressed.rs:42; llvm-mc rejects a third operand)
**Documentation conflict:** (none) — the comment names the two-operand form and does not declare extra operands invalid in so many words, nor admit a limitation
**Severity:** medium
**Counterexample:** encode_c_add([Reg("x0"), Reg("x1"), Imm(0)])
**Expected / Actual:** Err / Ok(Half(0x9006)) — encoding of `c.add x0, x1` (HINT)
**Impact:** Accidental extra tokens are dropped, so a malformed instruction still assembles as a valid 16-bit HINT/C.ADD.
**Root cause:** compressed.rs:43-47 reads only operands[0] and operands[1] via get_reg and never checks operands.len().
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:43`
```rust
pub(crate) fn encode_c_add(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    let rs2 = get_reg(operands, 1)?;
    Ok(EncodeResult::Half(0b10 | ((rs2 as u16) << 2) | ((rd as u16) << 7) | (1 << 12) | (0b100 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.add: expected 2 operands, got {}", operands.len()));
}
```
**Bug report:** bug_reports/encode_c_add_extra_operand.md
**Repro seed:** cc 90bcb15308bb091474f8c16e69b1127e88b3a129476d372024005ea6230c2d9a
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_add_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc 90bcb15308bb091474f8c16e69b1127e88b3a129476d372024005ea6230c2d9a

thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::encode_c_add_neg_extra' (2850634) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:268:1:
Test failed: extra operand must Err for c.add x0, x1 (llvm-mc rejects extra operands); got Ok(Half(36870)) at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:352.
minimal failing input: rd = "x0", rs2 = "x1", extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0

thread 'backend::riscv::assembler::encoder::encode_c_add_pbt::test_encode_c_add_regression_extra_operand' (2850644) panicked at src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs:250:5:
c.add x1, x2 with a third operand must Err; got Ok(Half(37002))
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs | 7 properties + 6 KAT + 2 regression witnesses |

## Reproduction

Whole suite (serial reconfirm of this campaign):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_add_pbt -- --test-threads=1
```

B1 (`c.add rd, x0` encodes as C.JALR / C.EBREAK):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_add_regression_rs2_x0 -- --test-threads=1
```

B2 (extra operand ignored):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_c_add_regression_extra_operand -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage summary
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/report.json — machine-readable report
- pbt-out/bug_reports/encode_c_add_rs2_x0.md — B1 markdown
- pbt-out/bug_reports/encode_c_add_rs2_x0.html — B1 HTML
- pbt-out/bug_reports/encode_c_add_extra_operand.md — B2 markdown
- pbt-out/bug_reports/encode_c_add_extra_operand.html — B2 HTML
- pbt-out/run/encode_c_add_pbt.log — serial test log
- pbt-out/run/kat.log — KAT gate log
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/CHANGE_SURFACE.md — change surface

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 22:11 (campaign: coverage)
> Files: 15/15 scanned (100%) | Functions: 218/367 total | PBT candidates: 218 | Tested: 218 (100%) | 1 pass, 218 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 15 |
| Files scanned | 15 / 15 (100%) |
| Total functions (all files) | 367 |
| PBT candidates (from FUNCTION_INDEX) | 218 |
| **Tested (of PBT candidates)** | **218 / 218 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 218 / -1 |
| **Overall (tested / all functions)** | **218 / 367 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 218 | 218 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 218 | 218 | 0 | 100% |

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
