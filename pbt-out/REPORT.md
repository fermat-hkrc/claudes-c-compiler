# PBT Campaign Report: encode_double_shift (i686)

## Summary

**Verdict:** 4 bugs (2 high, 1 medium, 1 high-width): memory-destination SHLD/SHRD rejected; Imm counts outside Imm8 silently truncated; non-GP and width-mismatched registers accepted via `reg_num` alias — silent wrong encodings on the public `shld`/`shrd` assembler surface.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_double_shift (src/backend/i686/assembler/encoder/gp_integer.rs)
**Tests:** 10 properties (+ KAT + regression witnesses)
**Result:** 6 passing, 4 failing → 4 bugs
**Change surface:** 1 function (`encode_double_shift`), 1 with properties, 0 error-handling-only changes without failure-path coverage (neg paths covered)
**Coverage evidence:** file-level (symbol presence) — no .gcda/.profraw (uninstrumented build tree); `coverage_gaps` reported NOT LINKED false-negative on mangled symbol; behavioral execution confirmed via KAT/PBT byte outputs and 4 SUT bugs; sweep round 1 closed all documented contract surfaces
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_double_shift | 10 props (+KAT/reg) | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: Memory-destination SHLD/SHRD rejected

**Formal:** ∀ mnem ∈ {shldl,shrdl}, src ∈ GP32, mem ∈ valid_i686_mem, form ∈ {Imm(c), CL}. encode(mnem, form, Reg(src), Mem(mem)) = llvm-mc(…)
**Contract evidence:** inferred (Intel SDM Vol.2 SHLD/SHRD r/m32 forms; assembler/README.md:171 lists shld/shrd; llvm-mc accepts)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `shldl $0, %eax, (%eax)` → Err("unsupported double shift operands"); llvm-mc Ok
**Expected / Actual:** Ok bytes matching llvm-mc / Err
**Impact:** Valid memory double-shifts cannot assemble; segment-override mem forms unreachable.
**Root cause:** gp_integer.rs:940 catch-all rejects Memory destinations; only Reg+Reg arms exist.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:940`
```rust
            _ => Err("unsupported double shift operands".to_string()),
```
**Suggested fix:** Add Imm/CL + Reg + Mem arms with `emit_segment_prefix` + `encode_modrm_mem`.
```rust
            (Operand::Immediate(ImmediateValue::Integer(count)), Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, opcode]);
                self.encode_modrm_mem(src_num, mem)?;
                self.bytes.push((*count).try_into().map_err(|_| "Imm8")?);
                Ok(())
            }
```
**Bug report:** bug_reports/encode_double_shift_mem_dst_unsupported.md
**Repro seed:** proptest cc 7bdec11d24a84e46467a5068ecd7bf0dc748b55fa5b8c91ced4df43558df7f1a
**Raw output:**
```text
SUT rejected valid mem-dst form `shldl $0, %eax, (%eax)`: unsupported double shift operands
```

### B2: Imm count outside Imm8 silently truncated

**Formal:** ∀ c outside llvm-mc Imm8 acceptance. SUT Err iff llvm Err (no silent `as u8` truncation)
**Contract evidence:** inferred (Intel Imm8; llvm-mc rejects `$256`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `shldl $256, %eax, %eax` → Ok([0x0f,0xa4,0xc0,0x00])
**Expected / Actual:** Err / Ok with count truncated to 0
**Impact:** Wrong shift amount in generated code without assembler error (i128 Imm shld/shrd path).
**Root cause:** gp_integer.rs:931 `self.bytes.push(*count as u8);` with no range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:931`
```rust
                self.bytes.push(*count as u8);
```
**Suggested fix:** Reject counts outside Imm8 before push.
```rust
                if !(-128..=255).contains(count) {
                    return Err(format!("double shift immediate out of Imm8 range: {count}"));
                }
                self.bytes.push(*count as u8);
```
**Bug report:** bug_reports/encode_double_shift_imm8_truncate.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
encode_double_shift must reject Imm count 256 (Imm8 domain), got Ok([15, 164, 194, 0])
```

### B3: Non-GP registers accepted via reg_num alias

**Formal:** ∀ mnem ∈ {shldl,shrdl}, x ∈ XMM. encode(mnem, … x …) = Err
**Contract evidence:** inferred (Intel SDM GP r/m32; llvm-mc rejects xmm; `is_xmm` exists in registers.rs)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `shldl $1, %eax, %xmm0` → Ok([0x0f,0xa4,0xc0,0x01]) (= %eax)
**Expected / Actual:** Err / Ok aliasing xmm0→eax encoding
**Impact:** Public AT&T with xmm silently becomes a different GP instruction.
**Root cause:** gp_integer.rs:927-928 uses `reg_num` without GP-class gate; xmm aliases to 0–7.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:927`
```rust
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:** Reject `is_xmm` / `is_mm` / st before encoding.
```rust
                if is_xmm(&src.name) || is_mm(&src.name) || is_xmm(&dst.name) || is_mm(&dst.name) {
                    return Err("double shift requires GP registers".into());
                }
```
**Bug report:** bug_reports/encode_double_shift_accepts_non_gp.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
encode_double_shift must reject non-GP xmm0 src, got Ok([15, 164, 194, 1])
```

### B4: Width-mismatched GP registers accepted

**Formal:** ∀ mnem ∈ {shldl,shrdl}, bad ∈ GP16∪GP8. encode(mnem, Imm(1), … bad …) = Err
**Contract evidence:** inferred (size=4 dispatch; llvm-mc rejects `shldl $1, %ax, %edx`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `shldl $1, %eax, %ax` → Ok([0x0f,0xa4,0xc0,0x01]) (= %eax dst)
**Expected / Actual:** Err / Ok with ax aliased to eax
**Impact:** Width-mismatched assembler text silently becomes 32-bit double-shift.
**Root cause:** No `reg_size == 4` check; `_size` unused; `reg_num` collapses ax/eax.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:927`
```rust
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:** Require `reg_size(name) == size` (and still reject non-GP).
```rust
                if reg_size(&src.name) != 4 || reg_size(&dst.name) != 4 {
                    return Err("double shift register width mismatch".into());
                }
```
**Bug report:** bug_reports/encode_double_shift_mismatched_width.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
encode_double_shift must reject width-mismatched `shldl $1, %eax, %ax`, got Ok([15, 164, 192, 1])
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs | 10 properties + 6 KAT + 4 regression |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_double_shift_pbt;` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_double_shift -- --test-threads=1
cargo test --lib encode_double_shift_regression_mem_dst_rejected -- --test-threads=1
cargo test --lib encode_double_shift_regression_imm_truncate -- --test-threads=1
cargo test --lib encode_double_shift_regression_xmm0_accepted -- --test-threads=1
cargo test --lib encode_double_shift_regression_mismatched_width -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_double_shift_mem_dst_unsupported.md (+ .html)
- pbt-out/bug_reports/encode_double_shift_imm8_truncate.md (+ .html)
- pbt-out/bug_reports/encode_double_shift_accepts_non_gp.md (+ .html)
- pbt-out/bug_reports/encode_double_shift_mismatched_width.md (+ .html)
- pbt-out/run/encode_double_shift_test2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 08:50 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 283/399 total | PBT candidates: 283 | Tested: 283 (100%) | 1 pass, 283 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 399 |
| PBT candidates (from FUNCTION_INDEX) | 283 |
| **Tested (of PBT candidates)** | **283 / 283 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 283 / -1 |
| **Overall (tested / all functions)** | **283 / 399 (71%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 283 | 283 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 283 | 283 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 31 | 17 | 17 | 100% | covered |
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
| encode_mov_seg | system.rs |
| encode_pop16 | system.rs |
| encode_bsr_bsf_16 | system.rs |
| encode_mov_infer_size | gp_integer.rs |
| encode_mov_rr | gp_integer.rs |
| encode_mov_mem_reg | gp_integer.rs |
| encode_mov_reg_mem | gp_integer.rs |
| encode_mov_imm_mem | gp_integer.rs |
| encode_movsx | gp_integer.rs |
| encode_movzx | gp_integer.rs |
| encode_lea | gp_integer.rs |
| encode_push | gp_integer.rs |
| encode_push16 | gp_integer.rs |
| encode_pop | gp_integer.rs |
| encode_alu | gp_integer.rs |
| encode_test | gp_integer.rs |
| encode_imul | gp_integer.rs |
| encode_inc_dec | gp_integer.rs |
| encode_double_shift | gp_integer.rs |
