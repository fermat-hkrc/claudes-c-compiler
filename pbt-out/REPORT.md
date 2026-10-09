# PBT Campaign Report: encode_imul (i686)

## Summary

**Verdict:** 10 filed bugs (4 root-cause classes) in `encode_imul`: (1) high — 2/3-op `imulw` omits 0x66 and emits imm32 instead of imm16; (2) high — memory forms omit segment prefixes; (3) high — 1-op segmented memory omits segment via `encode_unary_rm`; (4) medium — non-GP/mismatched-width registers silently alias through `reg_num`. Bare 32-bit forms match llvm-mc.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_imul (i686 gp_integer)
**Tests:** 12 properties + 8 KAT + 5 regressions
**Result:** 2 properties passing, 10 failing; 10 bugs (4 root-cause classes)
**Change surface:** 1 changed function (encode_imul), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence / cargo test execution of encode_imul_pbt); native line coverage may appear under pbt-out/code-coverage/ when instrument-coverage data is collected
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_imul | 12 props (+ KAT/regressions) | 10 (4 root classes) | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_imul ignores size on 2/3-operand forms (missing 0x66 + wrong imm width)

**Formal:** ∀ width∈{2,4}, src,dst∈GP(width). encode(imul{w|l} %src, %dst) = llvm_mc(same) — falsified at width=2
**Contract evidence:** inferred (Intel SDM IMUL 0F AF /r and 6B/69 with operand-size override; llvm-mc `-triple=i686` reference; dispatch passes size=2 for `imulw`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `imulw %ax, %bx` → sut `[0f,af,d8]` vs mc `[66,0f,af,d8]`; `imulw $300, %ax` → sut `[69,c0,2c,01,00,00]` vs mc `[66,69,c0,2c,01]`
**Expected / Actual:** 66-prefixed 16-bit encoding / unprefixed 32-bit-shaped bytes
**Impact:** Any `imulw` 2/3-op assembly produces 32-bit IMUL machine code
**Root cause:** gp_integer.rs:716-771 never consults `size` on 2/3-op arms; 0x69 always appends i32 LE bytes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:719`
```rust
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
self.bytes.push(self.modrm(3, dst_num, src_num));
```
**Suggested fix:** Emit `0x66` when `size==2` before opcodes; use `(*val as i16).to_le_bytes()` for 0x69 when size==2
```rust
if size == 2 { self.bytes.push(0x66); }
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
// ...
if size == 2 {
    self.bytes.extend_from_slice(&(*val as i16).to_le_bytes());
} else {
    self.bytes.extend_from_slice(&(*val as i32).to_le_bytes());
}
```
**Bug report:** bug_reports/encode_imul_missing_operand_size_prefix.md
**Repro seed:** proptest minimal width=2, si=0, di=0
**Raw output:**
```text
assertion `left == right` failed: imulw RR must emit 0x66 operand-size prefix; sut=[0f, af, d8] mc=[66, 0f, af, d8]
```

### B2: encode_imul memory forms omit segment override prefix

**Formal:** ∀ seg, mem, dst. encode(seg:mem → dst) = [seg_prefix(seg)] ++ encode(mem → dst)
**Contract evidence:** documented core.rs:31-42 `emit_segment_prefix`; inferred from llvm-mc and sibling encoders
**Documentation conflict:** (none) — helper exists but is not called
**Severity:** high
**Counterexample:** `imull %es:(%eax), %ebx` → sut `[0f,af,18]` vs mc `[26,0f,af,18]`; `imull $5, %es:(%eax), %ebx` → sut `[6b,18,05]` vs `[26,6b,18,05]`
**Expected / Actual:** segment byte present / absent
**Impact:** Segmented IMUL addresses the wrong segment silently
**Root cause:** gp_integer.rs:723-726 and 761-771 call `encode_modrm_mem` without `emit_segment_prefix(mem)`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:723`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0xAF]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:**
```rust
self.emit_segment_prefix(mem);
if size == 2 { self.bytes.push(0x66); }
self.bytes.extend_from_slice(&[0x0F, 0xAF]);
self.encode_modrm_mem(dst_num, mem)
```
**Bug report:** bug_reports/encode_imul_missing_segment_prefix.md
**Repro seed:** si=0, bi=0, di=0, form=0
**Raw output:**
```text
metamorphic seg: expected [26]||[0f, af, 00] got [0f, af, 00]
```

### B3: encode_imul 1-op memory path omits segment prefix (via encode_unary_rm)

**Formal:** ∀ mem with segment. encode(imull mem) = llvm_mc(same)
**Contract evidence:** inferred (llvm-mc; emit_segment_prefix contract; encode_imul 1-op delegates to encode_unary_rm)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `imull %es:(%eax)` → sut `[f7,28]` vs mc `[26,f7,28]`
**Expected / Actual:** `[26,f7,28]` / `[f7,28]`
**Impact:** Unary IMUL with segment override is wrong
**Root cause:** encode_unary_rm memory arm (gp_integer.rs:794-796) never calls `emit_segment_prefix`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:794`
```rust
Operand::Memory(mem) => {
    self.bytes.push(if size == 1 { 0xF6 } else { 0xF7 });
    self.encode_modrm_mem(op_ext, mem)
}
```
**Suggested fix:**
```rust
self.emit_segment_prefix(mem);
if size == 2 { self.bytes.push(0x66); }
self.bytes.push(if size == 1 { 0xF6 } else { 0xF7 });
self.encode_modrm_mem(op_ext, mem)
```
**Bug report:** bug_reports/encode_imul_unary_missing_segment_prefix.md
**Repro seed:** width=2, form=2, si=0, bi=0
**Raw output:**
```text
diff unary `imulw %es:(%eax)`: sut=[66, f7, 28] mc=[26, 66, f7, 28]
```

### B4: encode_imul accepts non-GP / mismatched-width registers

**Formal:** ∀ non-GP or width-mismatched register pair. encode → Err (when llvm-mc rejects)
**Contract evidence:** inferred (Intel SDM IMUL r/m forms are GP; llvm-mc rejects `imull %xmm0, %eax`; `reg_num` is not a width/class gate)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `imull %xmm0, %eax` → Ok(`[0f,af,c0]`) (aliases as eax,eax)
**Expected / Actual:** Err / Ok with GP-aliased bytes
**Impact:** Invalid asm silently produces wrong GP encoding
**Root cause:** gp_integer.rs:717 uses `reg_num` without `reg_size`/GP class checks
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:717`
```rust
let src_num = reg_num(&src.name).ok_or("bad register")?;
let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:** Require `reg_size(name) == size` and GP-only names before encoding
```rust
if reg_size(&src.name) != size || reg_size(&dst.name) != size {
    return Err("imul register width mismatch".into());
}
```
**Bug report:** bug_reports/encode_imul_accepts_non_gp.md
**Repro seed:** kind=2, ni=0 (xmm0)
**Raw output:**
```text
unsupported shape kind=2 must Err, got Ok(Some([0f, af, c0]))
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_imul_pbt.rs | 12 properties + 8 KAT + 5 regressions |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_imul_pbt;` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_ -- --test-threads=1
# Single bugs:
cargo test --lib encode_imul_regression_imulw_rr_missing_66 -- --test-threads=1
cargo test --lib encode_imul_regression_segment_mem_reg -- --test-threads=1
cargo test --lib encode_imul_regression_unary_segment -- --test-threads=1
cargo test --lib encode_imul_neg_unsupported_shape -- --test-threads=1
# Green path:
cargo test --lib encode_imul_diff_bare32_all_forms -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md, pbt-out/REPORT.html
- pbt-out/PROPERTIES.md, pbt-out/PLAN.md
- pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_imul_missing_operand_size_prefix.md (+ .html)
- pbt-out/bug_reports/encode_imul_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_imul_unary_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_imul_accepts_non_gp.md (+ .html)
- pbt-out/bug_reports/encode_imul_diff_mem_reg_size_or_seg.md (+ .html)
- pbt-out/bug_reports/encode_imul_diff_imm_reg_size.md (+ .html)
- pbt-out/bug_reports/encode_imul_diff_imm_reg_reg_size.md (+ .html)
- pbt-out/bug_reports/encode_imul_diff_imm_mem_reg_seg.md (+ .html)
- pbt-out/bug_reports/encode_imul_invariant_rr_missing_66.md (+ .html)
- pbt-out/bug_reports/encode_imul_neg_mismatched_width.md (+ .html)
- pbt-out/run/encode_imul_*.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 07:42 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 281/399 total | PBT candidates: 281 | Tested: 281 (100%) | 1 pass, 281 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 399 |
| PBT candidates (from FUNCTION_INDEX) | 281 |
| **Tested (of PBT candidates)** | **281 / 281 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 281 / -1 |
| **Overall (tested / all functions)** | **281 / 399 (70%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 281 | 281 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 281 | 281 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 31 | 15 | 15 | 100% | covered |
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
