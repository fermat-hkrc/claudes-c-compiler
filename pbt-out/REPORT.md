# PBT Campaign Report: encode_vmv_v_i

## Summary

**Verdict:** 3 medium: encode_vmv_v_i silently ignores extra operands and trailing `v0.t`, and truncates out-of-range immediates, so invalid `vmv.v.i` assembly is encoded as a valid unmasked move instead of an assembler error.
**Date:** 2026-10-07
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_vmv_v_i
**Tests:** 8 properties (plus 5 KAT + 4 regression witnesses)
**Result:** 5 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_vmv_v_i NOT LINKED in C++ reporter binaries; the function did execute under `cargo test --lib encode_vmv_v_i`.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_vmv_v_i | 8 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_vmv_v_i ignores extra operands

**Formal:** ∀ vd ∈ {0..31}, simm ∈ {-16..15}, extra ∈ Operand. encode_vmv_v_i([Reg("v{vd}"), Imm(simm), extra]) is Err
**Contract evidence:** inferred (llvm-mc rejects a third operand with "invalid operand for instruction"; encoder/mod.rs:1008 passes operands through; RISC-V V 1.0 form is vd plus one simm5)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vmv_v_i([Reg("v0"), Imm(-16), Imm(0)])
**Expected / Actual:** Err / Ok(Word(0x5e083057)) — same as `vmv.v.i v0, -16`
**Impact:** Invalid assembly with a stray third operand is assembled into a valid-looking unmasked vmv.v.i word instead of an assembler error.
**Root cause:** vector.rs:173-177 encode_vmv_v_i reads only operands 0 and 1 via get_vreg/get_imm and never checks operands.len() == 2, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:173`
```rust
    let vd = get_vreg(operands, 0)?;
    let simm5 = get_imm(operands, 1)? as u32 & 0x1F;
    // funct6=010111, vm=1, vs2=0
    let word = (0b010111u32 << 26) | (1u32 << 25) | (simm5 << 15) | (0b011 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands.
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.i expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```
**Bug report:** bug_reports/encode_vmv_v_i_extra_operand.md
**Repro seed:** cc 2417367e6420684f3af120bde8d42903ccea5e59ba918e2874d4010cd8b93bc4
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_i_pbt::encode_vmv_v_i_neg_extra' (2888382) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:266:1:
Test failed: extra operand Imm(0) must Err for vmv.v.i (llvm-mc rejects extra); got Ok(Word(1577594967)) at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:370.
minimal failing input: vd = 0, simm = -16, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_vmv_v_i ignores trailing v0.t

**Formal:** ∀ vd ∈ {0..31}, simm ∈ {-16..15}. encode_vmv_v_i([Reg("v{vd}"), Imm(simm), Symbol("v0.t")]) is Err
**Contract evidence:** inferred (RISC-V V 1.0 vmv.v.i is unmasked-only with vm=1 and vs2=0; llvm-mc rejects `vmv.v.i v0, 0, v0.t` as "invalid operand for instruction"; encoder/mod.rs:1008 passes operands through)
**Documentation conflict:** vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" asserts the unmasked two-operand encoding; it does not declare v0.t invalid as an input-domain restriction, nor admit a masked form. encoder/mod.rs:956 TODO "masked variants (v0.t) are not yet supported" is a limitation on instructions that have a masked encoding; vmv.v.i has none.
**Severity:** medium
**Counterexample:** encode_vmv_v_i([Reg("v0"), Imm(-16), Symbol("v0.t")])
**Expected / Actual:** Err / Ok(Word(0x5e083057)) — same as `vmv.v.i v0, -16`
**Impact:** A trailing `v0.t` on vmv.v.i (which has no masked form) is assembled into a valid-looking unmasked word instead of an assembler error.
**Root cause:** vector.rs:173-177 encode_vmv_v_i reads only operands 0 and 1 via get_vreg/get_imm, hardcodes vm=1, and never inspects a third operand, so a trailing v0.t is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:173`
```rust
    let vd = get_vreg(operands, 0)?;
    let simm5 = get_imm(operands, 1)? as u32 & 0x1F;
    // funct6=010111, vm=1, vs2=0
    let word = (0b010111u32 << 26) | (1u32 << 25) | (simm5 << 15) | (0b011 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly two operands (vmv.v.i has no masked form).
```rust
    if operands.len() != 2 {
        return Err(format!("vmv.v.i expects 2 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```
**Bug report:** bug_reports/encode_vmv_v_i_mask_v0t.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_i_pbt::encode_vmv_v_i_neg_mask_v0t' (2888384) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:266:1:
Test failed: trailing v0.t must Err for vmv.v.i (llvm-mc rejects mask on vmv.v.i); got Ok(Word(1577594967)) at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:382.
minimal failing input: vd = 0, simm = -16
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_vmv_v_i truncates out-of-range immediates

**Formal:** ∀ vd ∈ {0..31}, imm ∈ ℤ \ {-16..15}. encode_vmv_v_i([Reg("v{vd}"), Imm(imm)]) is Err
**Contract evidence:** inferred (llvm-mc requires simm5 in [-16, 15] with "immediate must be an integer in the range [-16, 15]"; rustdoc names the field simm5; encoder/mod.rs:1008 passes the Imm through)
**Documentation conflict:** vector.rs:171 "vmv.v.i vd, simm5: OPIVI, funct6=010111, vm=1, vs2=0" names simm5 but does not declare values outside [-16, 15] invalid. It is not an input-domain restriction quote.
**Severity:** medium
**Counterexample:** encode_vmv_v_i([Reg("v0"), Imm(16)])
**Expected / Actual:** Err / Ok(Word(0x5e083057)) — same as `vmv.v.i v0, -16`
**Impact:** An immediate outside the signed 5-bit range is silently wrapped (`16` becomes `-16`, `-17` becomes `15`) and assembled as a different in-range vmv.v.i.
**Root cause:** vector.rs:174 packs `get_imm(operands, 1)? as u32 & 0x1F` with no range check, so values outside the signed 5-bit field wrap.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:174`
```rust
    let simm5 = get_imm(operands, 1)? as u32 & 0x1F;
```
**Suggested fix:** Reject immediates outside the signed 5-bit range [-16, 15] before packing.
```rust
    let imm = get_imm(operands, 1)?;
    if imm < -16 || imm > 15 {
        return Err(format!("simm5 out of range: {}", imm));
    }
    let simm5 = (imm as u32) & 0x1F;
```
**Bug report:** bug_reports/encode_vmv_v_i_imm_oob.md
**Repro seed:** (none — deterministic regression at bound+1)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vmv_v_i_pbt::encode_vmv_v_i_neg_imm_oob' (2888383) panicked at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:266:1:
Test failed: vmv.v.i v0, 16 must Err (llvm-mc immediate out of range [-16, 15]); got Ok(Word(1577594967)) at src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs:392.
minimal failing input: vd = 0, imm = 16
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs | 8 properties + 5 KAT + 4 regression witnesses |

## Reproduction

Whole suite (expected: 5 properties + 5 KAT passing; extra/v0.t/oob properties and regressions failing):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_vmv_v_i -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_i_regression_extra_operand -- --test-threads=1
```

B2 trailing v0.t:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_i_regression_mask_v0t -- --test-threads=1
```

B3 immediate out of range:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vmv_v_i_regression_simm_oob_16 -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this campaign report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/report.json — machine-readable report
- pbt-out/bug_reports/encode_vmv_v_i_extra_operand.md
- pbt-out/bug_reports/encode_vmv_v_i_extra_operand.html
- pbt-out/bug_reports/encode_vmv_v_i_mask_v0t.md
- pbt-out/bug_reports/encode_vmv_v_i_mask_v0t.html
- pbt-out/bug_reports/encode_vmv_v_i_imm_oob.md
- pbt-out/bug_reports/encode_vmv_v_i_imm_oob.html
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/CHANGE_SURFACE.md — encode_vmv_v_i
- proptest-regressions/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.txt — proptest failure corpus (framework-owned, same location as sibling encoder campaigns)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-07 01:28 (campaign: coverage)
> Files: 16/16 scanned (100%) | Functions: 231/383 total | PBT candidates: 231 | Tested: 231 (100%) | 1 pass, 231 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 (100%) |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 231 |
| **Tested (of PBT candidates)** | **231 / 231 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 231 / -1 |
| **Overall (tested / all functions)** | **231 / 383 (60%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 231 | 231 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 231 | 231 | 0 | 100% |

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
| encode_c_jalr | compressed.rs |
| encode_vsetvli | vector.rs |
| encode_vsetivli | vector.rs |
| encode_vsetvl | vector.rs |
| encode_vload | vector.rs |
| encode_vstore | vector.rs |
| encode_v_arith_vv | vector.rs |
| encode_v_arith_vx | vector.rs |
| encode_v_arith_vi | vector.rs |
| encode_vmv_v_v | vector.rs |
| encode_vmv_v_x | vector.rs |
| encode_vmv_v_i | vector.rs |
