# PBT Campaign Report: encode_mov_reg_mem

## Summary

**Verdict:** 3 high: `encode_mov_reg_mem` rejects es/cs/ss/ds segment overrides, silently accepts size-mismatched GP sources (`movl %ax, mem` → eax store bytes), and silently accepts non-GP sources (`movl %xmm0, mem` → eax store bytes) via `reg_num` aliasing.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mov_reg_mem (i686 gp_integer)
**Tests:** 8 properties (+ KAT gates + 3 regression witnesses)
**Result:** 5 passing, 3 failing (3 bugs)
**Change surface:** 1 changed function (encode_mov_reg_mem), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw and listed unrelated OH C++ binaries as NOT LINKED for this Rust symbol. Campaign execution evidence: `cargo test --lib encode_mov_reg_mem` compiles and runs production `InstructionEncoder::encode` → `encode_mov_reg_mem` (KAT + 1000-case proptest). Tier: standard.
**Effort tier:** standard (5–8 properties, ≥1000 cases, ≥1 strengthen/metamorphic, 1 coverage_gaps sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mov_reg_mem | 8 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_mov_reg_mem rejects es/cs/ss/ds segment overrides

**Formal:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base ∈ GP32, disp ∈ i32, width ∈ {1,2,4}, src ∈ GP(width). bytes(SUT) = llvm-mc(att with %seg:)
**Contract evidence:** inferred (core.rs:31-42 `emit_segment_prefix` implements all six overrides; Intel SDM 2.1.1; llvm-mc accepts; public MOV path via encode_mov)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `movb %al, %es:(%eax)` — SUT `Err("unsupported segment: es")`; llvm-mc `[0x26, 0x88, 0x00]`
**Expected / Actual:** `[0x26, 0x88, 0x00]` / `Err("unsupported segment: es")`
**Impact:** Valid AT&T forms used in OS/kernel code are rejected; fs/gs work, es/cs/ss/ds do not
**Root cause:** gp_integer.rs:220-225 inlines fs/gs-only match instead of `emit_segment_prefix`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:220`
```rust
        if let Some(ref seg) = mem.segment {
            match seg.as_str() {
                "fs" => self.bytes.push(0x64),
                "gs" => self.bytes.push(0x65),
                _ => return Err(format!("unsupported segment: {}", seg)),
            }
        }
```
**Suggested fix:** Call the shared helper:
```rust
        self.emit_segment_prefix(mem);
```
**Bug report:** bug_reports/encode_mov_reg_mem_missing_segment_prefix.md
**Repro seed:** proptest cc 28687c1cb77be470c1ea50a67dbb4ae3d376c5535467c7fe51f644388b94ae29; minimal seg="es", base="eax", disp=0, width=1, si=0
**Raw output:**
```text
Test failed: SUT rejected valid segment form `movb %al, %es:(%eax)`: unsupported segment: es; i686 MOV reg→mem must emit segment override (core.rs emit_segment_prefix; Intel SDM 2.1.1). Body only accepts fs/gs..
minimal failing input: seg = "es", base = "eax", disp = 0, width = 1, si = 0
```

### B2: encode_mov_reg_mem accepts size-mismatched GP source

**Formal:** ∀ mnemonic width W, src with reg_size≠W. llvm-mc rejects ⇒ SUT returns Err
**Contract evidence:** inferred (Intel SDM MOV size match; llvm-mc rejects; signature takes size from mnemonic)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `movl %ax, (%eax)` → `Ok([0x89, 0x00])` (same as `movl %eax, (%eax)`)
**Expected / Actual:** Err / Ok([0x89, 0x00])
**Impact:** Wrong-width assembly silently encodes as matching-width GP store
**Root cause:** gp_integer.rs:217 uses `reg_num` with no `reg_size(&src.name) == size` gate
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:217`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Suggested fix:**
```rust
        if reg_size(&src.name) != size {
            return Err(format!("register size mismatch for mov: {}", src.name));
        }
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Bug report:** bug_reports/encode_mov_reg_mem_mismatched_width.md
**Repro seed:** mode=0, bi=0, si=0, disp=0
**Raw output:**
```text
Test failed: SUT accepted size-mismatched reg→mem `movl %ax, (%eax)` → [89, 00]; MOV m,r requires matching operand size (Intel SDM; llvm-mc rejects). reg_num aliases widths so bytes look like a valid same-width form..
minimal failing input: mode = 0, bi = 0, si = 0, disp = 0
```

### B3: encode_mov_reg_mem accepts non-GP source via reg_num alias

**Formal:** ∀ non_gp ∈ {xmm*,mm*,st*,ymm*}, width. llvm-mc rejects ⇒ SUT returns Err
**Contract evidence:** inferred (Intel SDM 88/89 is GP-only; llvm-mc rejects; registers.rs aliases xmm/mm/st)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `movb %xmm0, (%eax)` → `Ok([0x88, 0x00])` (same as `movb %al, (%eax)`)
**Expected / Actual:** Err / Ok([0x88, 0x00])
**Impact:** Non-GP register names silently encode as GP stores
**Root cause:** `reg_num` maps xmm0/mm0/st/ymm0 to 0–7; no is_xmm/is_mm/st/ymm reject
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:217`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Suggested fix:**
```rust
        if is_xmm(&src.name) || is_mm(&src.name) || src.name.starts_with("st") || src.name.starts_with("ymm") {
            return Err(format!("non-GP register for mov: {}", src.name));
        }
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
```
**Bug report:** bug_reports/encode_mov_reg_mem_non_gp_src.md
**Repro seed:** ni=0, bi=0, width=1, disp=0
**Raw output:**
```text
Test failed: SUT accepted non-GP MOV reg→mem `movb %xmm0, (%eax)` → [88, 00]; 88/89 form is GP-only (Intel SDM). reg_num aliases xmm/mm/st to 0-7..
minimal failing input: ni = 0, bi = 0, width = 1, disp = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs | 8 properties + 6 KAT + 3 regressions |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_mov_reg_mem_pbt;` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_reg_mem -- --test-threads=1
cargo test --lib encode_mov_reg_mem_diff_llvm_mc_segment -- --test-threads=1
cargo test --lib encode_mov_reg_mem_neg_mismatched_width -- --test-threads=1
cargo test --lib encode_mov_reg_mem_neg_non_gp_src -- --test-threads=1
cargo test --lib encode_mov_reg_mem_regression_es_segment_prefix -- --test-threads=1
cargo test --lib encode_mov_reg_mem_regression_mismatched_width_ax -- --test-threads=1
cargo test --lib encode_mov_reg_mem_regression_non_gp_xmm0 -- --test-threads=1
```

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html (auto-rendered from report.json)
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_mov_reg_mem_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_mov_reg_mem_mismatched_width.md (+ .html)
- pbt-out/bug_reports/encode_mov_reg_mem_non_gp_src.md (+ .html)
- pbt-out/run/encode_mov_reg_mem_test.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 04:50 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 271/398 total | PBT candidates: 271 | Tested: 271 (100%) | 1 pass, 271 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 398 |
| PBT candidates (from FUNCTION_INDEX) | 271 |
| **Tested (of PBT candidates)** | **271 / 271 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 271 / -1 |
| **Overall (tested / all functions)** | **271 / 398 (68%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 271 | 271 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 271 | 271 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 30 | 5 | 5 | 100% | covered |
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
