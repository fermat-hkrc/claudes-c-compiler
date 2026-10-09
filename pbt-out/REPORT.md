# PBT Campaign Report: encode_mov_rr (i686)

## Summary

**Verdict:** 3 high findings (2 root causes): `encode_mov_rr` silently accepts size-mismatched GP pairs (and both-wrong-width) and non-GP names (xmm/mm/st) via `reg_num` aliasing, emitting 88/89 bytes that look like valid same-width MOV; same-width GP paths match llvm-mc.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mov_rr (gp_integer.rs)
**Tests:** 8 properties (+ 5 KAT + 3 deterministic regressions)
**Result:** 5 passing, 3 failing properties, 3 bug reports (2 root causes)
**Change surface:** 1 changed function (encode_mov_rr), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo not producing gcov/llvm-cov artifacts the tool reads); production symbol exercised by `cargo test --lib encode_mov_rr`. Tool NOT LINKED list pointed at unrelated OH C++ binaries.
**Effort tier:** standard (≥1000 proptest cases; 1 strengthen round; 1 contract-surface sweep; ≥1 metamorphic/differential)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mov_rr | 8 props (+KAT/regressions) | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_mov_rr accepts size-mismatched GP pairs

**Formal:** ∀ src ∈ GP_a, dst ∈ GP_b, mnemonic size s. (reg_size(src)≠s ∨ reg_size(dst)≠s) ∧ llvm-mc rejects ⇒ encode returns Err
**Contract evidence:** inferred (Intel SDM MOV same-size operands; AT&T suffix encodes width; llvm-mc `-triple=i686` rejects `movl %ax, %ebx` / `movb %eax, %bl`; same class as encode_mov_cr r32-only gate)
**Documentation conflict:** (none) — no comment declares mismatched width valid or out-of-domain
**Severity:** high
**Counterexample:** `movl %ax, %ebx` → `Ok([0x89, 0xc3])`
**Expected / Actual:** Err / Ok with aliased same-width encoding
**Impact:** Assembler accepts wrong-sized register names and emits machine code identical to the full-width form, so callers get no diagnostic and may assume a different width move occurred.
**Root cause:** gp_integer.rs:179-191 uses `reg_num` without `reg_size(src)==size && reg_size(dst)==size`; `reg_num` collapses ax/eax/al to one code.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:179`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;

        if size == 2 {
            self.bytes.push(0x66);
        }
        if size == 1 {
            self.bytes.push(0x88);
        } else {
            self.bytes.push(0x89);
        }
        self.bytes.push(self.modrm(3, src_num, dst_num));
        Ok(())
```
**Suggested fix:** Gate GP path on matching widths:
```rust
        if reg_size(&src.name) != size || reg_size(&dst.name) != size {
            return Err(format!(
                "mov register size mismatch: src={}, dst={}, expected size {}",
                src.name, dst.name, size
            ));
        }
```
**Bug report:** bug_reports/encode_mov_rr_mismatched_width.md
**Repro seed:** proptest cc 7b19cc18f6043dfb5ca479fd1de321557b945c55e208a9c6c2812cccbff1fe37 (both-wrong-width); mode=0,a_i=0,b_i=0 (mismatched)
**Raw output:**
```text
SUT accepted size-mismatched `movl %ax, %eax` → [89, c0]
movl %ax, %ebx must Err; got Ok(Ok([137, 195]))
```

### B3: encode_mov_rr accepts both-operands wrong width

**Formal:** ∀ src,dst both wrong width for mnemonic. llvm-mc rejects ⇒ encode returns Err
**Contract evidence:** inferred (same as B1)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `movl %al, %al` → `Ok([0x89, 0xc0])`
**Expected / Actual:** Err / Ok([0x89, 0xc0])
**Impact:** Same as B1; strengthen-round witness where both operands are wrong-sized.
**Root cause:** gp_integer.rs:179-191 — same missing `reg_size` gate as B1.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:179`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;

        if size == 2 {
            self.bytes.push(0x66);
        }
        if size == 1 {
            self.bytes.push(0x88);
        } else {
            self.bytes.push(0x89);
        }
        self.bytes.push(self.modrm(3, src_num, dst_num));
        Ok(())
```
**Suggested fix:** Same width gate as B1:
```rust
        if reg_size(&src.name) != size || reg_size(&dst.name) != size {
            return Err(format!(
                "mov register size mismatch: src={}, dst={}, expected size {}",
                src.name, dst.name, size
            ));
        }
```
**Bug report:** bug_reports/encode_mov_rr_both_wrong_width.md
**Repro seed:** cc 7b19cc18f6043dfb5ca479fd1de321557b945c55e208a9c6c2812cccbff1fe37
**Raw output:**
```text
SUT accepted both-wrong-width `movl %al, %al` → [89, c0]
minimal failing input: pair = 0, a_i = 0, b_i = 0
```

### B2: encode_mov_rr accepts non-GP registers (xmm/mm/st)

**Formal:** ∀ r ∈ {xmm*,mm*,st*}, gp ∈ GP_w, m ∈ {movb,movw,movl}. llvm-mc rejects ⇒ encode(m, …)=Err
**Contract evidence:** inferred (Intel SDM 88/89 GP-only; registers.rs:4-15 maps xmm/mm/st into reg_num 0–7; llvm-mc rejects)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `movl %xmm0, %eax` → `Ok([0x89, 0xc0])`
**Expected / Actual:** Err / Ok aliased as GP0
**Impact:** Mistyped or mis-parsed SSE/x87 names assemble as GP MOV to/from the aliased register number with no error.
**Root cause:** Same GP path at gp_integer.rs:179 uses `reg_num` without excluding non-GP classes.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:179`
```rust
        let src_num = reg_num(&src.name).ok_or_else(|| format!("bad register: {}", src.name))?;
        let dst_num = reg_num(&dst.name).ok_or_else(|| format!("bad register: {}", dst.name))?;
```
**Suggested fix:**
```rust
        fn is_gp_name(name: &str) -> bool {
            !name.starts_with("xmm")
                && !name.starts_with("ymm")
                && !(name.starts_with("mm") && !name.starts_with("mmx"))
                && !name.starts_with("st")
        }
        if !is_gp_name(&src.name) || !is_gp_name(&dst.name) {
            return Err(format!("non-GP register in mov: {}, {}", src.name, dst.name));
        }
```
**Bug report:** bug_reports/encode_mov_rr_non_gp.md
**Repro seed:** ni=0, gi=0, width=1, non_gp_as_src=false
**Raw output:**
```text
SUT accepted non-GP MOV RR `movb %al, %xmm0` → [88, c0]
movl %xmm0, %eax must Err; got Ok(Ok([137, 192]))
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs | 8 properties + 5 KAT + 3 regressions |
| src/backend/i686/assembler/encoder/mod.rs | +1 `#[cfg(test)] mod encode_mov_rr_pbt;` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_rr -- --test-threads=1
cargo test --lib test_encode_mov_rr_regression_rejects_movl_ax_ebx -- --test-threads=1
cargo test --lib test_encode_mov_rr_regression_rejects_movb_eax_bl -- --test-threads=1
cargo test --lib test_encode_mov_rr_regression_rejects_xmm -- --test-threads=1
```

Build contract (unchanged form): `cargo test --lib encode_mov_rr -- --test-threads=1`

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_mov_rr_mismatched_width.md (+ .html)
- pbt-out/bug_reports/encode_mov_rr_non_gp.md (+ .html)
- pbt-out/bug_reports/encode_mov_rr_both_wrong_width.md (+ .html)
- pbt-out/run/encode_mov_rr_test2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 04:23 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 269/398 total | PBT candidates: 269 | Tested: 269 (100%) | 1 pass, 269 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 398 |
| PBT candidates (from FUNCTION_INDEX) | 269 |
| **Tested (of PBT candidates)** | **269 / 269 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 269 / -1 |
| **Overall (tested / all functions)** | **269 / 398 (68%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 269 | 269 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 269 | 269 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 30 | 3 | 3 | 100% | covered |
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
