# PBT Campaign Report: encode_c_lui

## Summary

**Verdict:** 1 high: encode_c_lui silently truncates out-of-range immediates so `c.lui x3, 32` encodes as nzimm=-32 (halfword 0x7181, value 0xfffe0000) instead of Err; 1 medium: extra operands are ignored so `c.lui x3, 1, 0` encodes as `c.lui x3, 1`.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_c_lui
**Tests:** 9 properties (plus 6 KAT + 2 regression witnesses)
**Result:** 7 passing, 2 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (build not instrumented / reporter missing) and encode_c_lui NOT LINKED in C++ pbt binaries; Rust `cargo test --lib` is not those binaries. Sweep: 1 round (signed-vs-uimm20), then closed (tier round spent).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_c_lui | 9 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_c_lui ignores extra operands

**Formal:** ∀ rd ∈ GPR\{x0,x2}, ∀ imm ∈ [1,31], ∀ extra. encode_c_lui([Reg(rd), Imm(imm), extra]) = Err
**Contract evidence:** inferred (compressed.rs:5 two-operand form `c.lui rd, nzimm`; llvm-mc `-triple=riscv64 -mattr=+c` rejects a third token as `invalid operand for instruction`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_c_lui([Reg("x3"), Imm(1), Imm(0)])
**Expected / Actual:** Err / Ok(Half(0x6185)) (encoding of `c.lui x3, 1`)
**Impact:** Accidental extra tokens are dropped; the assembler emits a valid-looking 16-bit instruction instead of an error, so the extra operand never surfaces.
**Root cause:** compressed.rs:6-14 reads only operands[0] and operands[1] via get_reg/get_imm and never checks operands.len(), then returns Ok.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:6`
```rust
pub(crate) fn encode_c_lui(operands: &[Operand]) -> Result<EncodeResult, String> {
    let rd = get_reg(operands, 0)?;
    if rd == 0 || rd == 2 { return Err("c.lui: rd cannot be x0 or x2".into()); }
    let imm = get_imm(operands, 1)?;
    let nzimm = imm as i32;
    if nzimm == 0 { return Err("c.lui: nzimm must not be zero".into()); }
    let bit17 = ((nzimm >> 5) & 1) as u16;
    let bits16_12 = (nzimm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | ((bits16_12 & 0x1F) << 2) | ((rd as u16) << 7) | (bit17 << 12) | (0b011 << 13)))
}
```
**Suggested fix:** Reject any operand list whose length is not exactly 2 before packing.
```rust
if operands.len() != 2 {
    return Err(format!("c.lui: expected 2 operands, got {}", operands.len()));
}
```
**Bug report:** bug_reports/encode_c_lui_extra_operand.md
**Repro seed:** cc d61994242bf0b142134434df88fff935c338d29ed9044b435e1065818557a9d7
**Raw output:**
```text
proptest: Saving this and future failures in /home/toan/github/claudes-c-compiler/proptest-regressions/backend/riscv/assembler/encoder/encode_c_lui_pbt.txt
proptest: If this test was run on a CI system, you may wish to add the following line to your copy of the file. (You may need to create it.)
cc d61994242bf0b142134434df88fff935c338d29ed9044b435e1065818557a9d7

thread 'backend::riscv::assembler::encoder::encode_c_lui_pbt::encode_c_lui_neg_extra' (2838212) panicked at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:295:1:
Test failed: extra operand must Err for c.lui x3, 1 (llvm-mc rejects extra operands); got Ok(Half(24965)) at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:382.
minimal failing input: rd = "x3", imm = 1, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_c_lui truncates out-of-range immediates to 6 bits

**Formal:** ∀ rd ∈ GPR\{x0,x2}, ∀ imm ∉ [-32,-1]∪[1,31]∪[1048544,1048575]. llvm-mc rejects "c.lui rd, imm" ⇒ encode_c_lui([Reg(rd), Imm(imm)]) = Err
**Contract evidence:** inferred (RISC-V C.LUI CI-type nzimm is signed 6-bit; compress.rs:41 `So nzimm must fit in signed 6-bit range: -32..31 (but not 0)`; llvm-mc range `[0xfffe0, 0xfffff] or [1, 31]`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_c_lui([Reg("x3"), Imm(32)])
**Expected / Actual:** Err / Ok(Half(0x7181)) (encoding of `c.lui x3, -32`)
**Impact:** `c.lui x3, 32` is packed as nzimm=-32, so the register is loaded with 0xfffe0000 instead of the assembler rejecting an unencodable immediate. A caller that meant LUI 32 (0x00020000) gets a completely different value.
**Root cause:** compressed.rs:12-13 take bit 5 and bits 4:0 of nzimm with no range check, so 32 (0b100000) is packed as the 6-bit pattern of -32.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/compressed.rs:12`
```rust
    let bit17 = ((nzimm >> 5) & 1) as u16;
    let bits16_12 = (nzimm & 0x1F) as u16;
    Ok(EncodeResult::Half(0b01 | ((bits16_12 & 0x1F) << 2) | ((rd as u16) << 7) | (bit17 << 12) | (0b011 << 13)))
```
**Suggested fix:** Reject nzimm values that do not fit in signed 6 bits and are not the 20-bit LUI-style form of those values, before packing.
```rust
if !(-32..=31).contains(&nzimm) && !(0xfffe0..=0xfffff).contains(&imm) {
    return Err("c.lui: nzimm out of range".into());
}
```
**Bug report:** bug_reports/encode_c_lui_imm_oob.md
**Repro seed:** (none — deterministic counterexample rd="x3", imm=32)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_c_lui_pbt::encode_c_lui_neg_imm_oob' (2838213) panicked at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:295:1:
Test failed: oob imm 32 must Err (llvm-mc range [1,31]∪[0xfffe0,0xfffff]; ISA simm6); got Ok(Half(29057)) at src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs:370.
minimal failing input: rd = "x3", imm = 32
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs | 9 properties + 6 KAT + 2 regression witnesses |
| src/backend/riscv/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_c_lui_pbt;` |

## Reproduction

Whole suite (includes two expected property failures and two expected regression failures):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_lui -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_lui_neg_extra -- --test-threads=1
```

B2 oob immediate:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_c_lui_neg_imm_oob -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger (encode_c_lui row appended)
- pbt-out/COVERAGE_STATUS.md — this campaign's coverage summary
- pbt-out/INVARIANTS.md — confirmed invariants for encode_c_lui
- pbt-out/FUNCTION_INDEX.md — merged function index including compressed.rs
- pbt-out/report.json — machine-readable report
- pbt-out/bug_reports/encode_c_lui_extra_operand.md — B1
- pbt-out/bug_reports/encode_c_lui_extra_operand.html — B1 HTML
- pbt-out/bug_reports/encode_c_lui_imm_oob.md — B2
- pbt-out/bug_reports/encode_c_lui_imm_oob.html — B2 HTML
- pbt-out/run/kat.log, pbt-out/run/pbt.log, pbt-out/run/regression.log, pbt-out/run/sweep.log — test logs
- proptest-regressions/backend/riscv/assembler/encoder/encode_c_lui_pbt.txt — saved extra-operand seed

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 21:20 (campaign: coverage)
> Files: 15/15 scanned (100%) | Functions: 214/367 total | PBT candidates: 214 | Tested: 214 (100%) | 1 pass, 214 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 15 |
| Files scanned | 15 / 15 (100%) |
| Total functions (all files) | 367 |
| PBT candidates (from FUNCTION_INDEX) | 214 |
| **Tested (of PBT candidates)** | **214 / 214 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 214 / -1 |
| **Overall (tested / all functions)** | **214 / 367 (58%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 214 | 214 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 214 | 214 | 0 | 100% |

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
