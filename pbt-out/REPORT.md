# PBT Campaign Report: encode_cmovcc

## Summary

**Verdict:** 3 bugs (2 high, 1 medium): `encode_cmovcc` drops memory segment-override prefixes, omits 0x66 for unsuffixed r16 forms, and accepts r8/non-GP/mismatched-width registers via `reg_num` aliasing — any of these yields wrong machine code or silent acceptance of invalid operands.
**Date:** 2026-10-12
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_cmovcc (i686 gp_integer encoder)
**Tests:** 8 properties (5 passing, 3 failing) + KAT gates + deterministic regressions
**Result:** 5 passing, 3 bugs
**Change surface:** 1 changed function (encode_cmovcc), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` found no .gcda/.profraw (uninstrumented build tree); binary scan missed Rust-mangled encode_cmovcc, but cargo lib tests in encode_cmovcc_pbt exercise it via InstructionEncoder::encode (KAT/PBT byte oracles confirm). One standard sweep round closed after strengthen mem_w16 PASS.
**Effort tier:** standard (≥1000 proptest cases; ≥1 differential; 1 coverage_gaps sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cmovcc | 8 properties | 3 | differential (llvm-mc), algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_cmovcc omits memory segment-override prefix

**Formal:** ∀ m, seg, base, dst, disp. encode_cmovcc([Mem(seg:base+disp),Reg(dst)], m) = llvm-mc(...)
**Contract evidence:** inferred (sibling encoders call `emit_segment_prefix` at core.rs:31; Intel SDM segment override; llvm-mc emits 0x26/0x64/…)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `cmove %es:(%eax), %ebx` then compare bytes to llvm-mc
**Expected / Actual:** `[26, 0f, 44, 18]` / `[0f, 44, 18]`
**Impact:** FS/GS/ES memory cmovs assemble without the override → wrong segment at run time (silent data corruption).
**Root cause:** gp_integer.rs:1128-1133 memory arm never calls `emit_segment_prefix(mem)` before `0F 40+cc`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1128`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                if is_16bit { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&[0x0F, 0x40 + cc]);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode:
```rust
                self.emit_segment_prefix(mem);
                if is_16bit { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&[0x0F, 0x40 + cc]);
```
**Bug report:** bug_reports/encode_cmovcc_missing_segment_prefix.md
**Repro seed:** (deterministic regression; proptest shrunk to any seg/base)
**Raw output:**
```text
assertion failed: cmove %es:(%eax), %ebx must be [26, 0f, 44, 18], got [0f, 44, 18]
```

### B2: unsuffixed cmovcc with r16 omits 0x66

**Formal:** ∀ m ∈ CMOV_UNSUFFIXED, s,d ∈ GP16. encode_cmovcc([Reg(s),Reg(d)], m) = llvm-mc("m %s, %d")
**Contract evidence:** inferred (llvm-mc/GAS size-infer from r16; Intel CMOVcc r16 requires operand-size override)
**Documentation conflict:** gp_integer.rs:1103 "unsuffixed = 32-bit default" describes mnemonic-suffix default only — does not declare r16 invalid; SUT still accepts r16 and emits wrong (no 0x66) bytes. Not an input-domain exclusion.
**Severity:** high
**Counterexample:** `cmove %ax, %ax` → SUT `[0f,44,c0]` vs llvm-mc `[66,0f,44,c0]`
**Expected / Actual:** `[66, 0f, 44, c0]` / `[0f, 44, c0]`
**Impact:** Unsuffixed 16-bit cmovs encode as 32-bit ops — wrong-width conditional moves.
**Root cause:** gp_integer.rs:1104-1120 sets `is_16bit` only from explicit `w` suffix; register arm does not consult `reg_size`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1118`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                if is_16bit { self.bytes.push(0x66); }
```
**Suggested fix:** Infer 0x66 from matching r16 GP sizes (or reject unsuffixed r16):
```rust
                let use_16 = is_16bit || reg_size(&dst.name) == 2;
                if use_16 { self.bytes.push(0x66); }
```
**Bug report:** bug_reports/encode_cmovcc_unsuffixed_r16_no_66.md
**Repro seed:** m="cmove", src="ax", dst="ax"
**Raw output:**
```text
unsuffixed r16 size diff for `cmove %ax, %ax`: SUT=[0f, 44, c0] llvm-mc=[66, 0f, 44, c0]
```

### B3: encode_cmovcc accepts r8, non-GP, and mismatched-width registers

**Formal:** ∀ invalid ops ∈ {r8 pairs, non-GP, width mismatch}. encode_cmovcc(ops, m) = Err(_)
**Contract evidence:** inferred (Intel SDM CMOVcc r16/r32 only; llvm-mc rejects; `reg_num` aliasing is not a contract)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `cmove %al, %al` → Ok(`[0f,44,c0]`); `cmove %eax, %xmm0` → Ok; `cmove %ax, %eax` → Ok
**Expected / Actual:** Err / Ok with GP-aliased encoding
**Impact:** Invalid operands assemble without diagnostic as if the aliased GP register were used.
**Root cause:** gp_integer.rs:1119-1120 uses only `reg_num` with no `reg_size`/GP-class check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1119`
```rust
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:** Require matching r16/r32 GP; reject xmm/mm/st/ymm prefixes (see bug report for full gate).
```rust
                if reg_size(&src.name) != reg_size(&dst.name)
                    || !matches!(reg_size(&src.name), 2 | 4) { return Err(...); }
```
**Bug report:** bug_reports/encode_cmovcc_accepts_invalid_regs.md
**Repro seed:** src="al", dst="al", m="cmove"
**Raw output:**
```text
SUT accepted invalid-width r8 `cmove %al, %al` → [0f, 44, c0]
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs | 8 properties + KATs + 6 regressions |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_cmovcc_pbt;` (one line) |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cmovcc -- --test-threads=1
# narrowed:
cargo test --lib test_encode_cmovcc_regression_missing_es_prefix -- --test-threads=1 --nocapture
cargo test --lib test_encode_cmovcc_regression_unsuffixed_r16_prefix -- --test-threads=1 --nocapture
cargo test --lib test_encode_cmovcc_regression_rejects_al_bl -- --test-threads=1 --nocapture
```

## Output Directories

- pbt-out/REPORT.md, pbt-out/REPORT.html
- pbt-out/PROPERTIES.md, pbt-out/PLAN.md
- pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_cmovcc_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_cmovcc_unsuffixed_r16_no_66.md (+ .html)
- pbt-out/bug_reports/encode_cmovcc_accepts_invalid_regs.md (+ .html)
- pbt-out/run/kat.log, pbt-out/run/full.log
- pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-12 11:00 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 289/401 total | PBT candidates: 289 | Tested: 289 (100%) | 1 pass, 289 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 401 |
| PBT candidates (from FUNCTION_INDEX) | 289 |
| **Tested (of PBT candidates)** | **289 / 289 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 289 / -1 |
| **Overall (tested / all functions)** | **289 / 401 (72%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 289 | 289 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 289 | 289 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 33 | 23 | 23 | 100% | covered |
| load_store.rs | 20 | 19 | 19 | 100% | covered |
| neon.rs | 68 | 63 | 63 | 100% | covered |
| pseudo.rs | 44 | 19 | 19 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_setcc | gp_integer.rs |
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
| encode_bswap | gp_integer.rs |
| encode_bit_count | gp_integer.rs |
| encode_bsr_bsf | gp_integer.rs |
| encode_bt | gp_integer.rs |
| encode_cmovcc | gp_integer.rs |
