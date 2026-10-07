# PBT Campaign Report: encode_vid_v

## Summary

**Verdict:** 2 medium: encode_vid_v ignores extra operands (assembles `vid.v v0, 0` as unmasked vid.v v0) and ignores trailing v0.t (hardcodes vm=1, so `vid.v v1, v0.t` is the unmasked word and `vid.v v0, v0.t` is accepted despite mask overlap).
**Date:** 2026-10-07
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_vid_v
**Tests:** 6
**Result:** 4 passing, 2 bugs
**Change surface:** 1 changed function (encode_vid_v), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_vid_v NOT LINKED in C++ reporter binaries; the function ran under `cargo test --lib encode_vid_v`
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_vid_v | 6 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_vid_v ignores extra operands

**Formal:** ∀ vd ∈ {0..31}, extra ∈ Operand \ {Symbol("v0.t")}. encode_vid_v([Reg("v{vd}"), extra]) is Err
**Contract evidence:** inferred (llvm-mc `-triple=riscv64 -mattr=+v` rejects a second operand other than v0.t; public wrapper encoder/mod.rs:1015 passes operands through; no arity check)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_vid_v([Reg("v0"), Imm(0)])
**Expected / Actual:** Err / Ok(Word(0x5208a057)) — same encoding as `vid.v v0`
**Impact:** A stray second token is assembled into a valid-looking unmasked vid.v instead of an assembler error.
**Root cause:** vector.rs:183-186 reads only operand 0 via get_vreg and never checks operands.len(), so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:183`
```rust
    let vd = get_vreg(operands, 0)?;
    // vs2=0 (bits 24:20), funct6=010100, vm=1
    let word = (0b010100u32 << 26) | (1u32 << 25) | (0b10001u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly one vector register (unmasked form). Masked v0.t is B2.
```rust
    if operands.len() != 1 {
        return Err(format!("vid.v expects 1 operand, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```
**Bug report:** bug_reports/encode_vid_v_extra_operand.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vid_v_pbt::encode_vid_v_neg_extra' (2890497) panicked at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:210:1:
Test failed: extra operand Imm(0) must Err for vid.v (llvm-mc rejects extra except v0.t); got Ok(Word(1376297047)) at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:269.
minimal failing input: vd = 0, extra = Imm(
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_vid_v ignores trailing v0.t and hardcodes vm=1

**Formal:** ∀ vd ∈ {0..31}. let mc = llvm-mc("vid.v v{vd}, v0.t"); let sut = encode_vid_v([Reg("v{vd}"), Symbol("v0.t")]). (mc = Ok(w) ⇒ sut = Ok(w)) ∧ (mc = Err ⇒ sut = Err)
**Contract evidence:** documented limitation encoder/mod.rs:962 "TODO: masked variants (v0.t) are not yet supported; vm is hardcoded to 1 (unmasked)."
**Documentation conflict:** encoder/mod.rs:962 admits the gap on an input the public `"vid.v"` dispatcher accepts (operands passed through). It does not declare v0.t invalid; RISC-V V 1.0 and llvm-mc accept `vid.v vd, v0.t` for vd ∈ {1..31}. Severity stepped down one grade from high (wrong machine code on valid masked input) because the author documented the limitation.
**Severity:** medium (documented by the author)
**Counterexample:** encode_vid_v([Reg("v0"), Symbol("v0.t")])
**Expected / Actual:** Err (llvm-mc overlap) / Ok(Word(0x5208a057)). Also encode_vid_v([Reg("v1"), Symbol("v0.t")]) = Ok(Word(0x5208a0d7)) vs llvm-mc 0x5008a0d7 (vm=0).
**Impact:** A caller requesting masked element-index (`vid.v v1, v0.t`) gets unmasked machine code. `vid.v v0, v0.t` is silently accepted despite destination/mask overlap.
**Root cause:** vector.rs:183-186 reads only operand 0 and hardcodes `(1u32 << 25)` (vm=1). Trailing Symbol("v0.t") is never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:183`
```rust
    let vd = get_vreg(operands, 0)?;
    // vs2=0 (bits 24:20), funct6=010100, vm=1
    let word = (0b010100u32 << 26) | (1u32 << 25) | (0b10001u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Honour a trailing v0.t by clearing vm, reject vd=v0 when masked, and reject any other extra operand.
```rust
    let vd = get_vreg(operands, 0)?;
    let vm = match operands.get(1) {
        None => 1u32,
        Some(Operand::Symbol(s)) if s.eq_ignore_ascii_case("v0.t") => {
            if operands.len() != 2 {
                return Err(format!("vid.v masked form expects 2 operands, got {}", operands.len()));
            }
            if vd == 0 {
                return Err("vid.v vd cannot overlap mask register v0".into());
            }
            0u32
        }
        other => return Err(format!("unexpected operand 1 for vid.v: {:?}", other)),
    };
    let word = (0b010100u32 << 26) | (vm << 25) | (0b10001u32 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
```
**Bug report:** bug_reports/encode_vid_v_mask_v0t.md
**Repro seed:** cc 79435c78f73220722fadb2a5c82042fb48b2375f13aa515a908848a73c958895
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_vid_v_pbt::encode_vid_v_mask_v0t_diff_llvm_mc' (2890489) panicked at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:210:1:
Test failed: SUT encoded vid.v v0, v0.t as 0x5208a057 but llvm-mc rejected: llvm-mc error: <stdin>:1:7: error: The destination vector register group cannot overlap the mask register.
vid.v v0, v0.t
      ^
 at src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs:302.
minimal failing input: vd = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs | 6 properties + 4 KAT + 3 regression witnesses |

## Reproduction

Whole suite (includes 2 expected failures):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_vid_v -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vid_v_regression_extra_operand -- --test-threads=1
```

B2 mask v0.t overlap:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vid_v_regression_mask_v0_overlap -- --test-threads=1
```

B2 mask v0.t wrong vm (vd=1):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_vid_v_regression_mask_v0t -- --test-threads=1
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
- pbt-out/bug_reports/encode_vid_v_extra_operand.md
- pbt-out/bug_reports/encode_vid_v_extra_operand.html
- pbt-out/bug_reports/encode_vid_v_mask_v0t.md
- pbt-out/bug_reports/encode_vid_v_mask_v0t.html
- proptest-regressions/backend/riscv/assembler/encoder/encode_vid_v_pbt.txt (proptest failure corpus)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-07 01:44 (campaign: coverage)
> Files: 16/16 scanned (100%) | Functions: 232/383 total | PBT candidates: 232 | Tested: 232 (100%) | 1 pass, 232 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 (100%) |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 232 |
| **Tested (of PBT candidates)** | **232 / 232 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 232 / -1 |
| **Overall (tested / all functions)** | **232 / 383 (61%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 232 | 232 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 232 | 232 | 0 | 100% |

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
| encode_vid_v | vector.rs |
