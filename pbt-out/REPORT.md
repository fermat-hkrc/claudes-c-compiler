# PBT Campaign Report: encode_bit_count (i686)

## Summary

**Verdict:** 3 high bugs: `encode_bit_count` rejects valid memory sources (`lzcntl (%eax), %eax`), and silently accepts r16/r8 and xmm/mm/st via `reg_num` aliasing as if they were GP r32 — wrong machine code for invalid assembly.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_bit_count (src/backend/i686/assembler/encoder/gp_integer.rs)
**Tests:** 9 properties (+ 4 KAT + 3 regression witnesses)
**Result:** 6 passing, 3 failing properties → 3 bugs
**Change surface:** 1 changed function (encode_bit_count), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — no .gcda/.profraw; `coverage_gaps` matcher reported false NOT LINKED for Rust-mangled `encode_bit_count`; nm shows the real symbol in `ccc-70d56e2a1978a8d3` and KAT/differential runs execute it
**Effort tier:** standard (≥1000 cases, strengthen round done, 1 contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bit_count | 9 props (+4 KAT +3 reg) | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: Memory source rejected (r32, r/m32)

**Formal:** ∀ m ∈ {lzcntl,tzcntl,popcntl}, ∀ base,d ∈ GP32. encode(m, Mem(base), Reg(d)) = llvm-mc("m (%base), %d")
**Contract evidence:** inferred (Intel SDM LZCNT/TZCNT/POPCNT form r32, r/m32; llvm-mc accepts; x86 sibling encode_bit_count and i686 encode_bsr_bsf encode Memory)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `lzcntl (%eax), %eax` → Err("unsupported lzcntl operands"); llvm-mc = [f3, 0f, bd, 00]
**Expected / Actual:** Ok([f3,0f,bd,00]) / Err("unsupported lzcntl operands")
**Impact:** Any AT&T source using a memory operand for bit-count instructions fails to assemble.
**Root cause:** gp_integer.rs:971-980 match only arms (Register, Register); Memory falls through to default Err.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:971`
```rust
        match (&ops[0], &ops[1]) {
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(prefix);
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
            _ => Err(format!("unsupported {} operands", mnemonic)),
        }
```
**Suggested fix:** Add Memory arm like encode_bsr_bsf:
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.push(prefix);
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Bug report:** bug_reports/encode_bit_count_mem_src.md
**Repro seed:** proptest cc 9ab739bcb2eca664adae6bb9e25942b1c5f04082c9a88a64f84fcb79e93d31fd
**Raw output:**
```text
SUT rejected valid mem-source `lzcntl (%eax), %eax`: unsupported lzcntl operands; got mc=[f3, 0f, bd, 00]
minimal failing input: m = "lzcntl", base = "eax", d = "eax"
```

### B2: Wrong-width r16/r8 accepted

**Formal:** ∀ m ∈ {lzcntl,tzcntl,popcntl}, r16/r8 operands must Err (Intel *l forms are r32-only; llvm-mc rejects)
**Contract evidence:** inferred (Intel SDM; llvm-mc rejects `lzcntl %ax, %eax`; registers.rs aliases widths)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `lzcntl %ax, %eax` → Ok([f3, 0f, bd, c0])
**Expected / Actual:** Err / Ok([f3,0f,bd,c0]) (same as eax,eax)
**Impact:** Wrong-width assembly silently produces r32 encodings.
**Root cause:** gp_integer.rs:972-973 uses reg_num only; registers.rs maps ax/al to same codes as eax with no width check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:972`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:**
```rust
                if reg_size(&src.name) != 4 || reg_size(&dst.name) != 4 {
                    return Err(format!("{} requires 32-bit GP registers", mnemonic));
                }
```
**Bug report:** bug_reports/encode_bit_count_wrong_width.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
SUT accepted invalid-width `lzcntl %ax, %eax` → [f3, 0f, bd, c0]
minimal failing input: m = "lzcntl", s = "ax", d = "eax", flip = false
```

### B3: Non-GP xmm/mm/st/ymm accepted via reg_num

**Formal:** ∀ m, non-GP aliased names must Err for bit-count
**Contract evidence:** inferred (Intel SDM GP-only; llvm-mc rejects; registers.rs aliases xmm0→0)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `lzcntl %xmm0, %eax` → Ok([f3, 0f, bd, c0])
**Expected / Actual:** Err / Ok([f3,0f,bd,c0])
**Impact:** Invalid non-GP assembly encodes as GP instruction.
**Root cause:** Same reg_num alias path as B2 without is_xmm/is_mm/GP-only gate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:972`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
```
**Suggested fix:**
```rust
fn is_gp32(name: &str) -> bool {
    matches!(name, "eax"|"ecx"|"edx"|"ebx"|"esp"|"ebp"|"esi"|"edi")
}
if !is_gp32(&src.name) || !is_gp32(&dst.name) {
    return Err(format!("{} requires GP r32 registers", mnemonic));
}
```
**Bug report:** bug_reports/encode_bit_count_non_gp.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
SUT accepted non-GP `lzcntl %xmm0, %eax` → [f3, 0f, bd, c0]
minimal failing input: m = "lzcntl", bad = "xmm0", d = "eax", flip = false
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs | 9 proptest properties + 4 KAT + 3 regression |
| src/backend/i686/assembler/encoder/mod.rs | +1 `mod encode_bit_count_pbt` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bit_count -- --test-threads=1
cargo test --lib test_encode_bit_count_regression_mem_src_eax_eax -- --test-threads=1
cargo test --lib test_encode_bit_count_regression_rejects_ax_eax -- --test-threads=1
cargo test --lib test_encode_bit_count_regression_rejects_xmm0_eax -- --test-threads=1
```

Serial reconfirm (PBT_TEST_JOBS=1): all three property failures reproduced.

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_bit_count_mem_src.md (+ .html)
- pbt-out/bug_reports/encode_bit_count_wrong_width.md (+ .html)
- pbt-out/bug_reports/encode_bit_count_non_gp.md (+ .html)
- pbt-out/run/encode_bit_count_test*.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 09:39 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 285/399 total | PBT candidates: 285 | Tested: 285 (100%) | 1 pass, 285 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 399 |
| PBT candidates (from FUNCTION_INDEX) | 285 |
| **Tested (of PBT candidates)** | **285 / 285 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 285 / -1 |
| **Overall (tested / all functions)** | **285 / 399 (71%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 285 | 285 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 285 | 285 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 31 | 19 | 19 | 100% | covered |
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
| encode_bswap | gp_integer.rs |
| encode_bit_count | gp_integer.rs |
