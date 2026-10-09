# PBT Campaign Report: encode_test (i686)

## Summary

**Verdict:** 4 high/medium bugs in `encode_test`: missing Reg→Mem form (high), missing segment override on Imm→Mem (high), accepts size-mismatched GP pairs (medium), accepts non-GP via reg_num (medium). Same-width RR / Imm→Reg / bare Imm→Mem match llvm-mc.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_test (i686 gp_integer)
**Tests:** 10 properties (+ 6 KAT + 4 regression witnesses)
**Result:** 5 passing, 5 failing properties; 4 distinct root-cause bugs (5 bug entries; b2/b2b share segment-prefix root cause)
**Change surface:** 1 changed function (encode_test), 1 with properties, 0 error-handling-only changes without failure-path coverage (arity/neg paths covered)
**Coverage evidence:** file-level (symbol presence / cargo execution) — `coverage_gaps` reported no .gcda/.profraw for this Rust build and listed unrelated C++ binaries; real SUT execution evidenced by cargo test outcomes
**Effort tier:** standard (proptest cases=1000; ≥1 metamorphic; 1 coverage sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_test | 10 props + 6 KAT + 4 reg | 4 root causes (5 entries) | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: Missing Reg→Mem TEST form

**Formal:** ∀ width∈{1,2,4}, src∈GP(width), base∈GP32, disp. bytes(encode_test(test*, %src, mem)) = llvm-mc_i686("test* %src, mem")
**Contract evidence:** inferred (Intel SDM TEST r/m,r; AT&T `test %reg, mem`; x86-64 sibling `src/backend/x86/assembler/encoder/gp_integer.rs:608` implements the arm; llvm-mc accepts)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `testb %al, (%eax)` → Err("unsupported test operands"); llvm-mc = `[84, 00]`
**Expected / Actual:** Ok([0x84,0x00]) / Err("unsupported test operands")
**Impact:** Memory-destination TEST forms cannot be assembled on i686.
**Root cause:** `gp_integer.rs:708` default match arm — only RR, Imm→Reg, Imm→Mem are handled; no `(Register, Memory)`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:708`
```rust
            _ => Err("unsupported test operands".to_string()),
```
**Suggested fix:** Add Reg→Mem arm with emit_segment_prefix + 84/85 + encode_modrm_mem (mirror x86-64 sibling).
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                self.emit_segment_prefix(mem);
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x84 } else { 0x85 });
                self.encode_modrm_mem(src_num, mem)
            }
```
**Bug report:** bug_reports/encode_test_missing_reg_mem.md
**Repro seed:** proptest cc 4dfa37673f3b27f191e944dfd8eb9953927af4762d55fbbd54d62e48f131b44c
**Raw output:**
```text
SUT rejected valid Reg→Mem TEST `testb %al, (%eax)`: unsupported test operands
```

### B2: Imm→Mem omits segment override prefix

**Formal:** ∀ seg∈{es,cs,ss,ds,fs,gs}, width, base, imm. bytes(encode_test(test*, $imm, seg:mem)) = llvm-mc_i686(...) ∧ = [seg_prefix(seg)] ‖ bare
**Contract evidence:** documented `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/core.rs:31` "Emit segment override prefix if the memory operand has a segment."; x86-64 sibling calls emit_segment_prefix on Imm→Mem
**Documentation conflict:** (none) — helper documents the required behavior; encode_test fails to call it
**Severity:** high
**Counterexample:** `testb $5, %es:(%eax)` → sut=`[f6,00,05]` mc=`[26,f6,00,05]`
**Expected / Actual:** `[0x26,0xf6,0x00,0x05]` / `[0xf6,0x00,0x05]`
**Impact:** Segmented TEST imm forms encode against the wrong segment (silent wrong address).
**Root cause:** Imm→Mem arm `gp_integer.rs:689-706` never calls `emit_segment_prefix(mem)`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:689`
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Memory(mem)) => {
                let val = *val;
                if size == 2 { self.bytes.push(0x66); }
                if size == 1 {
                    self.bytes.push(0xF6);
                } else {
                    self.bytes.push(0xF7);
                }
                self.encode_modrm_mem(0, mem)?;
```
**Suggested fix:**
```rust
                self.emit_segment_prefix(mem);
                if size == 2 { self.bytes.push(0x66); }
```
**Bug report:** bug_reports/encode_test_missing_segment_prefix.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
segment diff `testb $5, %es:(%eax)`: sut=[f6, 00, 05] mc=[26, f6, 00, 05]
```

### B2b: Metamorphic seg‖bare fails (same root cause as B2)

**Formal:** ∀ seg, width, base, imm. encode(test*, $imm, seg:mem) = [seg_prefix(seg)] ‖ encode(test*, $imm, bare_mem)
**Contract evidence:** documented core.rs:31 emit_segment_prefix
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `testb $0 %es:(%eax)` → got `[f6,00,00]` expect `[26,f6,00,00]`
**Expected / Actual:** `[0x26,0xf6,0x00,0x00]` / `[0xf6,0x00,0x00]`
**Impact:** Same as B2 — segmented TEST imm forms omit override prefix.
**Root cause:** Same as B2 — Imm→Mem never calls emit_segment_prefix (`gp_integer.rs:689`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:689`
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Memory(mem)) => {
                let val = *val;
                if size == 2 { self.bytes.push(0x66); }
```
**Suggested fix:**
```rust
self.emit_segment_prefix(mem);
if size == 2 { self.bytes.push(0x66); }
```
**Bug report:** bug_reports/encode_test_metamorphic_segment_prefix.md
**Repro seed:** (deterministic)
**Raw output:**
```text
metamorphic seg||bare for testb $0 %es:(%eax): got [f6, 00, 00] expect [26, f6, 00, 00]
```

### B3: Accepts size-mismatched GP pairs

**Formal:** ∀ mismatched (src,dst,mnem) where llvm-mc rejects. encode_test → Err
**Contract evidence:** inferred (Intel SDM same-width TEST; llvm-mc rejects `testl %ax, %ebx`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `testl %ax, %eax` → Ok([0x85,0xc0]) (aliases as eax,eax)
**Expected / Actual:** Err / Ok([0x85,0xc0])
**Impact:** Mixed-width names silently encode as same-width GP forms.
**Root cause:** RR arm uses `reg_num` without `reg_size` vs mnemonic size gate (`gp_integer.rs:654-660`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:654`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
```
**Suggested fix:** Require `reg_size(src)==size && reg_size(dst)==size` before encoding.
```rust
                if reg_size(&src.name) != Some(size) || reg_size(&dst.name) != Some(size) {
                    return Err("size-mismatched test operands".into());
                }
```
**Bug report:** bug_reports/encode_test_accepts_mismatched_width.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
SUT accepted size-mismatched `testl %ax, %eax` → [85, c0]
```

### B4: Accepts non-GP register names

**Formal:** ∀ non_gp∈{xmm*,mm*,st*,ymm*}, gp∈GP. encode_test(test*, …) = Err
**Contract evidence:** inferred (Intel SDM TEST is GP r/m; llvm-mc rejects `testb %al, %xmm0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `testb %al, %xmm0` → Ok([0x84,0xc0]) (aliases as al,al)
**Expected / Actual:** Err / Ok([0x84,0xc0])
**Impact:** Invalid SIMD/x87 names produce plausible GP encodings.
**Root cause:** `reg_num` maps xmm/mm/st/ymm onto 0–7; RR arm does not reject non-GP (`gp_integer.rs:654-660`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:654`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
```
**Suggested fix:** Reject non-GP names before `reg_num`.
```rust
                if !is_gp_reg(&src.name) || !is_gp_reg(&dst.name) {
                    return Err("test requires GP registers".into());
                }
```
**Bug report:** bug_reports/encode_test_accepts_non_gp.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
SUT accepted non-GP TEST RR `testb %al, %xmm0` → [84, c0]
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_test_pbt.rs | 10 properties + 6 KAT + 4 regressions |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_test_pbt;` registration |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_test_ -- --test-threads=1
# single bugs:
cargo test --lib encode_test_regression_reg_mem_bare -- --test-threads=1
cargo test --lib encode_test_regression_es_segment_imm_mem -- --test-threads=1
cargo test --lib encode_test_regression_mismatched_width_testl_ax_ebx -- --test-threads=1
cargo test --lib encode_test_regression_non_gp_xmm -- --test-threads=1
```

## Output Directories

- `pbt-out/REPORT.md`, `pbt-out/REPORT.html` (rendered from report.json)
- `pbt-out/PROPERTIES.md`, `pbt-out/PLAN.md`
- `pbt-out/COVERAGE.md`, `pbt-out/COVERAGE_STATUS.md`
- `pbt-out/report.json`
- `pbt-out/INVARIANTS.md` (encode_test section appended)
- `pbt-out/bug_reports/encode_test_missing_reg_mem.md` (+ .html)
- `pbt-out/bug_reports/encode_test_missing_segment_prefix.md` (+ .html)
- `pbt-out/bug_reports/encode_test_accepts_mismatched_width.md` (+ .html)
- `pbt-out/bug_reports/encode_test_accepts_non_gp.md` (+ .html)
- `pbt-out/bug_reports/encode_test_metamorphic_segment_prefix.md` (+ .html)
- `pbt-out/run/encode_test_pbt.log`

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 07:22 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 280/399 total | PBT candidates: 280 | Tested: 280 (100%) | 1 pass, 280 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 399 |
| PBT candidates (from FUNCTION_INDEX) | 280 |
| **Tested (of PBT candidates)** | **280 / 280 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 280 / -1 |
| **Overall (tested / all functions)** | **280 / 399 (70%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 280 | 280 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 280 | 280 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 31 | 14 | 14 | 100% | covered |
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
