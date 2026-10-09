# PBT Campaign Report: encode_lsl (i686)

## Summary

**Verdict:** 3 root-cause high bugs in `encode_lsl` (4 ledger entries): (1) memory form omits segment-override prefixes, (2) 0x66 operand-size is keyed off the source register instead of the destination, (3) memory form never emits 0x66 for a 16-bit destination (base+disp and SIB/abs witnesses) — all produce wrong machine code vs Intel/llvm-mc.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_lsl (src/backend/i686/assembler/encoder/system.rs)
**Tests:** 8 properties (+ KAT + 4 regression witnesses)
**Result:** 4 passing, 4 failing properties; 4 bug reports (3 distinct root causes)
**Change surface:** 1 function (`encode_lsl`), 1 with properties, 0 error-handling-only changes (arity/shape negative paths covered)
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw and listed unrelated host C++ pbt binaries; Rust execution of `encode_lsl` is evidenced by cargo test counterexamples hitting system.rs:144–166. Recorded as file-level fallback per tool output.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_lsl | 8 properties (4 pass / 4 fail) | 4 | differential (llvm-mc), algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_lsl omits segment-override prefix on memory form

**Formal:** ∀ seg∈{es,cs,ss,ds,fs,gs}, base∈GP32, dst∈GP32. encode_lsl([Mem(seg:base), Reg(dst)]) = llvm_mc("lsl %seg:(%base), %dst")
**Contract evidence:** inferred (Intel/AT&T encoding + core.rs:31-42 `emit_segment_prefix` used by correct i686 encoders; llvm-mc reference)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `lsl %es:(%eax), %ebx` then compare bytes
**Expected / Actual:** `[0x26, 0x0f, 0x03, 0x18]` / `[0x0f, 0x03, 0x18]`
**Impact:** Segmented LSL forms assemble as DS-default; kernel/boot code loading limits via FS/GS/ES gets wrong encoding.
**Root cause:** system.rs:160-164 memory arm never calls `emit_segment_prefix(mem)` before the opcode.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:** Call `emit_segment_prefix` (and dest-driven 0x66) before the opcode:
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    let is_16 = matches!(dst.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
    self.emit_segment_prefix(mem);
    if is_16 { self.bytes.push(0x66); }
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Bug report:** bug_reports/encode_lsl_missing_segment_prefix.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
assertion `left == right` failed: lsl %es:(%eax), %ebx must be [26, 0f, 03, 18], got [0f, 03, 18]
```

### B2: encode_lsl keys 0x66 off source register, not destination

**Formal:** ∀ src∈GP, dst∈GP. encode_lsl([Reg(src), Reg(dst)]) = llvm_mc("lsl %src, %dst") (osize follows dest width)
**Contract evidence:** inferred (Intel SDM LSL r16/r32, r/m16; llvm-mc dest-driven osize)
**Documentation conflict:** (none) — doc comment only names opcode `0F 03 /r`
**Severity:** high
**Counterexample:** `lsl %eax, %ax` → SUT missing 0x66; `lsl %ax, %ebx` → SUT spurious 0x66
**Expected / Actual:** `[0x66,0x0f,0x03,0xc0]` / `[0x0f,0x03,0xc0]` (eax→ax); inverse for ax→ebx
**Impact:** Mixed-width AT&T LSL writes the limit at the wrong operand size.
**Root cause:** system.rs:152 `is_16` matches on `src.name` instead of `dst.name`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:152`
```rust
let is_16 = matches!(src.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
if is_16 {
    self.bytes.push(0x66);
}
```
**Suggested fix:**
```rust
let is_16 = matches!(dst.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
if is_16 {
    self.bytes.push(0x66);
}
```
**Bug report:** bug_reports/encode_lsl_osize_from_src_not_dst.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
minimal failing input: src = "eax", dst = "ax"
SUT=[0f, 03, c0] llvm-mc=[66, 0f, 03, c0]
```

### B3: encode_lsl memory form never emits 0x66 for 16-bit destination

**Formal:** ∀ mem, dst∈GP16. encode_lsl([Mem(mem), Reg(dst)]) = llvm_mc(...) including leading 0x66
**Contract evidence:** inferred (Intel LSL r16, m16; llvm-mc `lsl (%eax), %bx` → `[66,0f,03,18]`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `lsl (%eax), %bx`
**Expected / Actual:** `[0x66, 0x0f, 0x03, 0x18]` / `[0x0f, 0x03, 0x18]`
**Impact:** 16-bit dest memory LSL is assembled as 32-bit form, corrupting the high half of the destination register's enclosing r32.
**Root cause:** system.rs:160-164 memory arm has no dest-width check for 0x66 (unlike the register arm).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:** Same memory-arm fix as B1 (dest-driven 0x66 + segment prefix).
```rust
let is_16 = matches!(dst.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
self.emit_segment_prefix(mem);
if is_16 { self.bytes.push(0x66); }
self.bytes.extend_from_slice(&[0x0F, 0x03]);
self.encode_modrm_mem(dst_num, mem)
```
**Bug report:** bug_reports/encode_lsl_mem16_missing_66.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
assertion `left == right` failed: lsl (%eax), %bx must be [66, 0f, 03, 18], got [0f, 03, 18]
```

### B4: encode_lsl SIB/abs memory form omits 0x66 for 16-bit destination

**Formal:** ∀ valid SIB/abs mem, dst∈GP16. encode_lsl([Mem(mem), Reg(dst)]) = llvm_mc(...) including leading 0x66
**Contract evidence:** inferred (Intel LSL r16, m16; llvm-mc SIB form)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** base=None, index="eax", scale=1, disp=0, dst="ax"
**Expected / Actual:** `[0x66, 0x0f, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00]` / `[0x0f, 0x03, 0x04, 0x05, 0x00, 0x00, 0x00, 0x00]`
**Impact:** SIB-addressed 16-bit dest LSL assembled as 32-bit; same defective memory arm as B3.
**Root cause:** system.rs:160-164 memory arm never inspects destination width for 0x66.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:**
```rust
let is_16 = matches!(dst.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
self.emit_segment_prefix(mem);
if is_16 { self.bytes.push(0x66); }
self.bytes.extend_from_slice(&[0x0F, 0x03]);
self.encode_modrm_mem(dst_num, mem)
```
**Bug report:** bug_reports/encode_lsl_sib_mem16_missing_66.md
**Repro seed:** (deterministic proptest shrink)
**Raw output:**
```text
minimal failing input: base = None, index = "eax", scale = 1, disp = 0, dst = "ax"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_lsl_pbt.rs | 8 properties + 7 KAT + 4 regression |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_lsl_pbt;` registration |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lsl_ -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lsl_regression_missing_es_prefix -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lsl_regression_osize_from_src_eax_bx -- --test-threads=1
cargo test --lib test_encode_lsl_regression_osize_from_src_ax_ebx -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lsl_regression_mem16_missing_66 -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lsl_diff_sib -- --test-threads=1
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
- pbt-out/run/encode_lsl_kat.log, encode_lsl_full.log
- pbt-out/bug_reports/encode_lsl_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_lsl_osize_from_src_not_dst.md (+ .html)
- pbt-out/bug_reports/encode_lsl_mem16_missing_66.md (+ .html)
- pbt-out/bug_reports/encode_lsl_sib_mem16_missing_66.md (+ .html)

## Contract-surface sweep

- Round 1/`coverage_gaps`: no native line coverage for Rust; file-level fallback listed unrelated C++ binaries and marked encode_lsl NOT LINKED there. Documented behaviors already have properties (segment, osize/dest, mem16, opcode invariant, arity/shape). Sweep closed after one round (standard tier).

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 01:53 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 260/397 total | PBT candidates: 260 | Tested: 260 (100%) | 1 pass, 260 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 397 |
| PBT candidates (from FUNCTION_INDEX) | 260 |
| **Tested (of PBT candidates)** | **260 / 260 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 260 / -1 |
| **Overall (tested / all functions)** | **260 / 397 (65%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 260 | 260 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 260 | 260 | 0 | 100% |

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
