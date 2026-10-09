# PBT Campaign Report: encode_bsr_bsf_16

## Summary

**Verdict:** 2 bugs — 1 high (segment override omitted on `bsfw`/`bsrw` memory forms, so segmented loads assemble to the wrong address) and 1 medium (r32/r8 accepted via `reg_num` aliasing and silently encoded as r16).
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_bsr_bsf_16
**Tests:** 7 active properties (5 passing, 2 failing) + KAT + regression witnesses
**Result:** 5 passing, 2 bugs
**Change surface:** 1 changed function (encode_bsr_bsf_16), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no line-level `.profraw` and a false NOT LINKED against unrelated OH binaries; cargo lib tests executed the SUT (KATs and shrunk counterexamples from the real encoder body). Recorded as file-level.
**Effort tier:** standard (≥1000 proptest cases; differential + metamorphic required; 1 contract-surface sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bsr_bsf_16 | 7 active + KAT/regression | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_bsr_bsf_16 omits segment-override prefix on memory forms

**Formal:** ∀ m ∈ {bsfw,bsrw}, ∀ seg ∈ SREG, ∀ base ∈ R32, ∀ d ∈ R16. encode(m, %seg:(%base), %d) = llvm-mc(m %seg:(%base), %d)
**Contract evidence:** inferred (Intel SDM / AT&T segment override ordering; independent reference llvm-mc `-triple=i686` emits seg then 0x66 then 0F BC/BD; same contract as sibling i686 encoders that call `emit_segment_prefix`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `bsfw %es:(%eax), %bx` then compare bytes to llvm-mc
**Expected / Actual:** `[0x26, 0x66, 0x0f, 0xbc, 0x18]` / `[0x66, 0x0f, 0xbc, 0x18]`
**Impact:** Segmented memory sources assemble without the override; the CPU reads DS (default) instead of the requested segment — silent wrong-address bit-scans in boot/kernel code.
**Root cause:** `system.rs:362-375` pushes `0x66` and opcode then `encode_modrm_mem` without `emit_segment_prefix(mem)` (and the shared `0x66` is emitted before the match, so a naive post-hoc prefix would still be out of order).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:362`
```rust
        self.bytes.push(0x66); // 16-bit operand size prefix
        match (&ops[0], &ops[1]) {
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Emit segment override before `0x66` on the memory arm:
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.emit_segment_prefix(mem); // before 0x66
                self.bytes.push(0x66);
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Bug report:** bug_reports/encode_bsr_bsf_16_missing_segment_prefix.md
**Repro seed:** proptest `cc 8e74ee16fc4a2c3ddacab95a3cd26558b2eb772b0ed18706653c37559c1fb850` (diff_mem_segment); deterministic regression also available
**Raw output:**
```text
assertion failed: `(left == right)`
  left: `[102, 15, 188, 0]`,
 right: `[38, 102, 15, 188, 0]`: segmented mem must include override before 0x66 for bsfw %es:(%eax), %ax
minimal failing input: mnemonic = "bsfw", mseg = "es", base = "eax", dst = "ax", disp = 0
```

### B2: encode_bsr_bsf_16 accepts r32/r8 via reg_num aliasing

**Formal:** ∀ m ∈ {bsfw,bsrw}, ∀ bad ∈ R32∪R8, ∀ good ∈ R16. encode(m, bad, good) = Err ∧ encode(m, good, bad) = Err
**Contract evidence:** inferred (function doc "16-bit BSF/BSR"; Intel SDM r16,r/m16; llvm-mc rejects `bsfw %eax, %bx` / `bsrw %ax, %al`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `bsfw %eax, %bx` (also `bsfw %ax, %eax`, `bsrw %ax, %al`)
**Expected / Actual:** `Err` / `Ok([0x66, 0x0f, 0xbc, 0xd8])` (same as `bsfw %ax, %bx`)
**Impact:** Typos and wrong-width operands assemble without diagnostic, producing a 16-bit bit-scan encoding under a 32/8-bit name.
**Root cause:** `system.rs:365-368` calls `reg_num` with no `reg_size(...) == 2` gate; `reg_num` aliases eax/al→0, ebx/bl→3.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:365`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```
**Suggested fix:**
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                if reg_size(&src.name) != 2 || reg_size(&dst.name) != 2 {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
```
**Bug report:** bug_reports/encode_bsr_bsf_16_wrong_width_accepted.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
`bsfw %ax, %eax` must be Err like llvm-mc, got Ok([66, 0f, bc, c0])
minimal failing input: mnemonic = "bsfw", bad = "eax", good = "ax", bad_is_src = false
regression: bsfw %eax, %bx must be Err, got Ok([66, 0f, bc, d8])
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs | 7 proptest properties + 5 KAT + 3 regression |
| src/backend/i686/assembler/encoder/mod.rs | +1 `#[cfg(test)] mod encode_bsr_bsf_16_pbt;` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bsr_bsf_16 -- --test-threads=1
cargo test --lib test_encode_bsr_bsf_16_regression_segment_es_missing -- --test-threads=1
cargo test --lib test_encode_bsr_bsf_16_regression_r32_src_accepted -- --test-threads=1
cargo test --lib test_encode_bsr_bsf_16_regression_r8_dst_accepted -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_bsr_bsf_16_missing_segment_prefix.md
- pbt-out/bug_reports/encode_bsr_bsf_16_missing_segment_prefix.html
- pbt-out/bug_reports/encode_bsr_bsf_16_wrong_width_accepted.md
- pbt-out/bug_reports/encode_bsr_bsf_16_wrong_width_accepted.html
- pbt-out/run/encode_bsr_bsf_16_test1.log

## Contract-surface sweep

1 round via `coverage_gaps` after first full run. No line-level data; file-level false NOT LINKED ignored (cargo executed SUT). All documented behaviors of `encode_bsr_bsf_16` (arity, opcode select, 0x66, r16-r16, mem-r16, segment, width gate) have properties. Closed: tier round done.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 03:45 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 267/397 total | PBT candidates: 267 | Tested: 267 (100%) | 1 pass, 267 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 397 |
| PBT candidates (from FUNCTION_INDEX) | 267 |
| **Tested (of PBT candidates)** | **267 / 267 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 267 / -1 |
| **Overall (tested / all functions)** | **267 / 397 (67%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 267 | 267 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 267 | 267 | 0 | 100% |

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
| encode_mov_seg | system.rs |
| encode_pop16 | system.rs |
| encode_bsr_bsf_16 | system.rs |
