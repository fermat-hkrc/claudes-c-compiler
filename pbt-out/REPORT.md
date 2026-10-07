# PBT Campaign Report: encode_v_arith_vx

## Summary

**Verdict:** 1 medium and 1 low: encode_v_arith_vx ignores extra operands, so `vadd.vx v0, v0, x0, 0` encodes as unmasked `vadd.vx v0, v0, x0`, and a trailing `v0.t` is dropped so a masked OPIVX is silently encoded unmasked (vm=1).
**Date:** 2026-10-07
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_v_arith_vx
**Tests:** 8
**Result:** 6 passing, 2 bugs
**Change surface:** 1 changed function (encode_v_arith_vx), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_v_arith_vx NOT LINKED in C++ reporter binaries; Rust cargo tests are not those binaries. Manual audit of the documented 3-operand / format / isolation / vs2-rs1-swap / ABI / arity / extra / mask-v0.t surface.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_v_arith_vx | 8 | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_v_arith_vx ignores extra operands

**Formal:** ∀ vd, vs2, rs1 ∈ 0..31, extra ∉ {v0.t mask}, (mnem, funct6) ∈ opivx_family. encode_v_arith_vx([v{vd}, v{vs2}, x{rs1}, extra], funct6) is Err
**Contract evidence:** inferred (rustdoc three-operand OPIVX form at vector.rs:133; llvm-mc `-triple=riscv64 -mattr=+v` rejects a fourth non-mask token; encoder/mod.rs:976-994 passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_v_arith_vx([Reg("v0"), Reg("v0"), Reg("x0"), Imm(0)], funct6=0b000000)
**Expected / Actual:** Err / Ok(Word(0x02004057)) — same encoding as `vadd.vx v0, v0, x0`
**Impact:** Invalid assembly with a stray fourth operand is assembled into a valid-looking unmasked OPIVX word instead of an assembler error.
**Root cause:** vector.rs:135-140 reads only operands 0, 1, and 2 via get_vreg/get_reg and never checks operands.len() == 3, so extra tokens are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:135`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let rs1 = get_reg(operands, 2)?;
    let vm: u32 = 1;
    let word = (funct6 << 26) | (vm << 25) | (vs2 << 20) | (rs1 << 15) | (0b100 << 12) | (vd << 7) | OP_V;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject any operand list that is not exactly three operands (mask token v0.t is a separate 4-operand form).
```rust
    if operands.len() != 3 {
        return Err(format!("OPIVX expects 3 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```
**Bug report:** bug_reports/encode_v_arith_vx_extra_operand.md
**Repro seed:** (none — shrunk to Imm(0) on the first case; replay via the regression test)
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_arith_vx_pbt::encode_v_arith_vx_neg_extra' (2880308) panicked at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:317:1:
Test failed: extra operand Imm(0) must Err for OPIVX (llvm-mc rejects extra); got Ok(Word(33570903)) at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:478.
minimal failing input: vd = 0, vs2 = 0, rs1 = 0, extra = Imm(
    0,
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_v_arith_vx drops v0.t and encodes unmasked

**Formal:** ∀ vd ∈ {1..31}, vs2, rs1 ∈ {0..31}, (mnem, funct6) ∈ opivx_family with not (slide mnemonic and vd = vs2). encode_v_arith_vx([Reg(v{vd}), Reg(v{vs2}), Reg(x{rs1}), Symbol("v0.t")], funct6) = llvm-mc(mnem v{vd}, v{vs2}, x{rs1}, v0.t)
**Contract evidence:** documented limitation encoder/mod.rs:952 "TODO: masked variants (v0.t) are not yet supported; vm is hardcoded to 1 (unmasked)." — known limitation on an input the public wrapper accepts (operands passed through at encoder/mod.rs:976-994). RISC-V V 1.0 and llvm-mc encode `, v0.t` as vm=0.
**Documentation conflict:** encoder/mod.rs:952 "TODO: masked variants (v0.t) are not yet supported; vm is hardcoded to 1 (unmasked)." — admits a gap on an input the API accepts (limitation, not an exclusion). vector.rs:138 `let vm: u32 = 1;` describes the emission, not that v0.t is invalid.
**Severity:** low (documented by the author)
**Counterexample:** encode_v_arith_vx([Reg("v1"), Reg("v0"), Reg("x0"), Symbol("v0.t")], funct6=0b000000)
**Expected / Actual:** Ok(Word(0x000040d7)) / Ok(Word(0x020040d7)) — llvm-mc `vadd.vx v1, v0, x0, v0.t` vs unmasked `vadd.vx v1, v0, x0`
**Impact:** A masked OPIVX instruction is silently encoded as unmasked. Inactive elements that should be left undisturbed are overwritten.
**Root cause:** vector.rs:138 hardcodes `vm = 1` and never inspects operand 3, so a v0.t mask token cannot clear bit 25.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:138`
```rust
    let vm: u32 = 1;
```
**Suggested fix:** Accept an optional fourth operand `v0.t` and set vm=0; reject any other extra operand.
```rust
    let vm: u32 = match operands.get(3) {
        None => 1,
        Some(Operand::Symbol(s)) | Some(Operand::Reg(s)) if s == "v0.t" => 0,
        Some(other) => return Err(format!("operand 3 must be v0.t, got {:?}", other)),
    };
    if operands.len() > 4 {
        return Err(format!("OPIVX expects 3 or 4 operands, got {}", operands.len()));
    }
```
**Bug report:** bug_reports/encode_v_arith_vx_mask_v0t.md
**Repro seed:** cc 26e6d7b1921522136d997ec987cd64251d210f32de45ce4dde8137a70f3bd208
**Raw output:**
```text
thread 'backend::riscv::assembler::encoder::encode_v_arith_vx_pbt::encode_v_arith_vx_mask_v0t' (2880287) panicked at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:317:1:
Test failed: assertion failed: `(left == right)` 
  left: `33571031`, 
 right: `16599`: SUT 0x020040d7 != llvm-mc 0x000040d7 for masked vadd.vx v1, v0, x0, v0.t at src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs:502.
minimal failing input: vd = 1, vs2 = 0, rs1 = 0, kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs | 8 properties + 9 KAT + 2 regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_v_arith_vx -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_arith_vx_regression_extra_operand -- --test-threads=1
```

B2 mask v0.t:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_arith_vx_regression_mask_v0t -- --test-threads=1
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
- pbt-out/bug_reports/encode_v_arith_vx_extra_operand.md
- pbt-out/bug_reports/encode_v_arith_vx_extra_operand.html
- pbt-out/bug_reports/encode_v_arith_vx_mask_v0t.md
- pbt-out/bug_reports/encode_v_arith_vx_mask_v0t.html
- pbt-out/run/ — scratch directory for test CWD

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-07 00:28 (campaign: coverage)
> Files: 16/16 scanned (100%) | Functions: 227/383 total | PBT candidates: 227 | Tested: 227 (100%) | 1 pass, 227 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 (100%) |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 227 |
| **Tested (of PBT candidates)** | **227 / 227 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 227 / -1 |
| **Overall (tested / all functions)** | **227 / 383 (59%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 227 | 227 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 227 | 227 | 0 | 100% |

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
