# PBT Campaign Report: encode_mov_cr (i686)

## Summary

**Verdict:** 2 medium (one root cause): `encode_mov_cr` accepts 8/16-bit GP names and `movw` and silently emits the r32 CR-move encoding (`movl %cr0, %ax` / `movw %cr0, %ax` → `0F 20 C0`), so invalid-width CR moves assemble instead of erroring.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mov_cr (src/backend/i686/assembler/encoder/system.rs)
**Tests:** 8 properties + 5 KAT + 2 regression witnesses
**Result:** 6 properties passing, 2 failing; 2 bug reports (shared root cause)
**Change surface:** 1 changed function (encode_mov_cr), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) / none native — `coverage_gaps` reported no .profraw/.gcda; execution proven by KAT/PBT byte assertions against the live symbol
**Effort tier:** standard (≥1000 proptest cases; ≥1 metamorphic; 1 strengthen round; 1 coverage_gaps sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mov_cr | 8 props + 5 KAT | 1 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_mov_cr accepts non-r32 GP operands

**Formal:** ∀ cr ∈ {cr0,cr2,cr3,cr4}, gp ∈ r8∪r16. llvm_mc rejects movl %cr,%gp (and symmetric write) ⇒ SUT must Err (not emit 0F 20/22 as if r32)
**Contract evidence:** inferred (Intel SDM MOV to/from control registers is r32 on IA-32; llvm-mc `-triple=i686` rejects `movl %cr0, %ax`; doc asserts 0F 20/22 /r without authorizing narrow GP)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `movl %cr0, %ax` → Ok([0x0f, 0x20, 0xc0]); also `movl %al, %cr0`
**Expected / Actual:** Err / Ok([0x0f, 0x20, 0xc0]) (same as `%eax`)
**Impact:** Accidental narrow-register CR moves assemble silently as r32 forms; disagrees with llvm-mc and Intel operand-size rules.
**Root cause:** system.rs:257-258 and 263-264 use `reg_num` which aliases al/ax/eax to one index, with no `reg_size == 4` check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:255`
```rust
            (Operand::Register(cr), Operand::Register(gp)) if is_control_reg(&cr.name) => {
                let cr_num = control_reg_num(&cr.name).ok_or("bad control register")?;
                let gp_num = reg_num(&gp.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x20]);
                self.bytes.push(self.modrm(3, cr_num, gp_num));
                Ok(())
            }
```
**Suggested fix:** Gate both arms with `reg_size(&gp.name) == 4` before encoding.
```rust
                if reg_size(&gp.name) != 4 {
                    return Err("mov cr requires 32-bit register".to_string());
                }
                let gp_num = reg_num(&gp.name).ok_or("bad register")?;
```
**Bug report:** bug_reports/encode_mov_cr_non_r32_gp.md
**Repro seed:** proptest cc 11bc540d850d59d89636a58f251f4d9027d380054df80d63af5d56346fa706d5 (neg_bad_operands); deterministic regressions need no seed
**Raw output:**
```text
SUT accepted invalid-width GP `movl %cr0, %ax` → [0f, 20, c0]; MOV CR requires r32 (Intel SDM; llvm-mc rejects).
minimal failing input: kind = 3, cr = "cr0", r16 = "ax"
movl %cr0, %ax must Err (r32 only); got Ok(Ok([15, 32, 192]))
```

### B2: encode_mov_cr accepts movw with control registers

**Formal:** ∀ cr, r16. llvm_mc rejects movw CR form ⇒ SUT Err
**Contract evidence:** inferred (Intel SDM r32-only; llvm-mc rejects `movw %cr0, %ax`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `movw %cr0, %ax` → Ok([0x0f, 0x20, 0xc0])
**Expected / Actual:** Err / Ok([0x0f, 0x20, 0xc0])
**Impact:** 16-bit-sized mov involving CR silently becomes a 32-bit CR move.
**Root cause:** `encode_mov` routes `movw` into `encode_mov_cr`; no `reg_size == 4` gate (same statements as B1).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:255`
```rust
            (Operand::Register(cr), Operand::Register(gp)) if is_control_reg(&cr.name) => {
                let cr_num = control_reg_num(&cr.name).ok_or("bad control register")?;
                let gp_num = reg_num(&gp.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x20]);
                self.bytes.push(self.modrm(3, cr_num, gp_num));
                Ok(())
            }
```
**Suggested fix:** Same r32 gate as B1.
```rust
                if reg_size(&gp.name) != 4 {
                    return Err("mov cr requires 32-bit register".to_string());
                }
```
**Bug report:** bug_reports/encode_mov_cr_movw_accepted.md
**Repro seed:** (deterministic proptest shrink; no separate seed required)
**Raw output:**
```text
SUT accepted `movw %cr0, %ax` → [0f, 20, c0]; MOV CR is r32-only
minimal failing input: write = false, cr = "cr0", r16 = "ax"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs | 8 properties + 5 KAT + 2 regressions |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_mov_cr_pbt;` registration |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_cr -- --test-threads=1
cargo test --lib test_encode_mov_cr_regression_rejects_ax -- --test-threads=1
cargo test --lib test_encode_mov_cr_regression_rejects_al -- --test-threads=1
cargo test --lib encode_mov_cr_neg_bad_operands -- --test-threads=1
cargo test --lib encode_mov_cr_neg_movw_width -- --test-threads=1
```

Build contract (immutable): `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` — rebuilt by swapping filter to `encode_mov_cr`.

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_mov_cr_non_r32_gp.md
- pbt-out/bug_reports/encode_mov_cr_non_r32_gp.html (from report.json)
- pbt-out/bug_reports/encode_mov_cr_movw_accepted.md
- pbt-out/bug_reports/encode_mov_cr_movw_accepted.html (from report.json)
- pbt-out/run/encode_mov_cr_full2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 03:00 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 264/397 total | PBT candidates: 264 | Tested: 264 (100%) | 1 pass, 264 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 397 |
| PBT candidates (from FUNCTION_INDEX) | 264 |
| **Tested (of PBT candidates)** | **264 / 264 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 264 / -1 |
| **Overall (tested / all functions)** | **264 / 397 (66%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 264 | 264 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 264 | 264 | 0 | 100% |

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
| pseudo.rs | 44 | 19 | 19 | 100% | covered |

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
| encode_v_crypto_vi | vector.rs |
| encode_v_crypto_vv | vector.rs |
| encode_v_crypto_vs | vector.rs |
| encode_li | pseudo.rs |
| encode_mv | pseudo.rs |
| encode_not | pseudo.rs |
| encode_negw | pseudo.rs |
| encode_sext_w | pseudo.rs |
| encode_seqz | pseudo.rs |
| encode_snez | pseudo.rs |
| encode_sltz | pseudo.rs |
| encode_sgtz | pseudo.rs |
| encode_beqz | pseudo.rs |
| encode_bnez | pseudo.rs |
| encode_blez | pseudo.rs |
| encode_bgez | pseudo.rs |
| encode_bltz | pseudo.rs |
| encode_bgtz | pseudo.rs |
| encode_bgt | pseudo.rs |
| encode_ble | pseudo.rs |
| encode_bgtu | pseudo.rs |
| encode_prefetch | system.rs |
| encode_prefetch_0f0d | system.rs |
| encode_out | system.rs |
| encode_in | system.rs |
| encode_invlpg | system.rs |
| encode_verw | system.rs |
| encode_lsl | system.rs |
| encode_system_table | system.rs |
| encode_lmsw | system.rs |
| encode_smsw | system.rs |
| encode_mov_cr | system.rs |
