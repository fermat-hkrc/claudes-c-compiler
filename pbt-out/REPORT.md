# PBT Campaign Report: encode_mov_infer_size

## Summary

**Verdict:** 2 high bugs: unsuffixed `mov $imm, mem` silently defaults to movl, and mismatched-width GP `mov` silently encodes using the first register's size — both accepted where llvm-mc/GAS reject.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mov_infer_size (i686 gp_integer)
**Tests:** 9 properties (+ 7 KAT / strengthen unit tests + 2 regression witnesses)
**Result:** 7 passing properties, 2 failing properties, 2 bugs
**Change surface:** 1 changed function (encode_mov_infer_size), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence / cargo test execution) — `coverage_gaps` found no .gcda/.profraw (Rust build not gcov-instrumented in this tree) and listed unrelated OH binaries as NOT LINKED; campaign evidence is the lib-test binary executing `InstructionEncoder::encode("mov")` → `encode_mov_infer_size` (KATs + 1000-case proptest runs). Tier: standard.
**Effort tier:** standard (≥1000 proptest cases; 1 strengthen round; 1 contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mov_infer_size | 9 props (+KAT/regression) | 2 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: Unsuffixed `mov $imm, mem` silently defaults to 32-bit

**Formal:** ∀ imm, mem. llvm_mc rejects "mov $imm, mem" ⇒ encode_mov_infer_size([Imm,Mem]).is_err()
**Contract evidence:** inferred (GAS/llvm-mc ambiguous-suffix contract for unsuffixed mov without a register operand; doc claims size is inferred from operands)
**Documentation conflict:** (none) — gp_integer.rs:121 `_ => 4 // default to 32-bit` is the producing statement, not a domain restriction
**Severity:** high
**Counterexample:** `mov $0, (%eax)` → Ok([c7, 00, 00, 00, 00, 00])
**Expected / Actual:** Err(ambiguous) / Ok(movl encoding)
**Impact:** Assembler accepts ambiguous store-immediate and always emits dword width, risking memory corruption vs intended byte/word stores.
**Root cause:** gp_integer.rs:121 `_ => 4` defaults size when neither operand is a Register, then encode_mov emits C7.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:117`
```rust
        let size = match (&ops[0], &ops[1]) {
            (Operand::Register(r), _) => reg_size(&r.name),
            (_, Operand::Register(r)) => reg_size(&r.name),
            _ => 4, // default to 32-bit
        };
        self.encode_mov(ops, size)
```
**Suggested fix:** Reject the no-register case:
```rust
            _ => {
                return Err(
                    "ambiguous mov: no register operand to infer size (use movb/movw/movl)"
                        .to_string(),
                );
            }
```
**Bug report:** bug_reports/encode_mov_infer_size_ambiguous_imm_mem.md
**Repro seed:** cc d01348f11493b4893fbc4fbc4fd75495baa5f95445f568547559ec9cc02c1242
**Raw output:**
```text
Test failed: ambiguous imm→mem `mov $0, (%eax)` must Err; got Ok(Some([c7, 00, 00, 00, 00, 00]))
minimal failing input: bi = 0, imm = 0, disp = 0
```

### B2: Mismatched-width GP `mov` silently uses first-register size

**Formal:** ∀ src∈GP_w1, dst∈GP_w2, w1≠w2. llvm_mc rejects "mov %src, %dst" ⇒ encode_mov_infer_size.is_err()
**Contract evidence:** inferred (GAS/llvm-mc reject unsuffixed mov with unequal GP widths; doc claims size inference from operands implies a single consistent size)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `mov %ax, %al` → Ok([66, 89, c0])
**Expected / Actual:** Err(size mismatch) / Ok(16-bit mov encoding)
**Impact:** Wrong opcode/prefix for mismatched register pairs; code other assemblers reject is accepted with incorrect machine code.
**Root cause:** gp_integer.rs:118 takes only the first Register's `reg_size` and never compares widths when both operands are GP registers.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:117`
```rust
        let size = match (&ops[0], &ops[1]) {
            (Operand::Register(r), _) => reg_size(&r.name),
            (_, Operand::Register(r)) => reg_size(&r.name),
            _ => 4, // default to 32-bit
        };
        self.encode_mov(ops, size)
```
**Suggested fix:** When both operands are GP registers, require `reg_size(a) == reg_size(b)` (leave CR/Sreg pairs to encode_mov specialized paths):
```rust
        if let (Operand::Register(a), Operand::Register(b)) = (&ops[0], &ops[1]) {
            if !is_control_reg(&a.name) && !is_control_reg(&b.name)
                && !is_segment_reg(&a.name) && !is_segment_reg(&b.name)
                && reg_size(&a.name) != reg_size(&b.name)
            {
                return Err(format!(
                    "mov operand size mismatch: {} vs {}",
                    a.name, b.name
                ));
            }
        }
```
**Bug report:** bug_reports/encode_mov_infer_size_mismatched_width.md
**Repro seed:** (deterministic regression; proptest shrunk to w1=2,w2=1,si=0,di=0)
**Raw output:**
```text
Test failed: mismatched-width `mov %ax, %al` must Err; got Ok(Some([66, 89, c0]))
minimal failing input: w1 = 2, w2 = 1, si = 0, di = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs | 9 properties, 7 KAT/strengthen units, 2 regression witnesses |
| src/backend/i686/assembler/encoder/mod.rs | +1 `mod encode_mov_infer_size_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_infer_size -- --test-threads=1
```

B1 regression:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_infer_size_regression_ambiguous_imm_mem -- --test-threads=1
```

B2 regression:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_infer_size_regression_mismatched_width -- --test-threads=1
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
- pbt-out/bug_reports/encode_mov_infer_size_ambiguous_imm_mem.md (+ .html)
- pbt-out/bug_reports/encode_mov_infer_size_mismatched_width.md (+ .html)
- pbt-out/run/encode_mov_infer_size_test1.log
- pbt-out/run/encode_mov_infer_size_test2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 04:05 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 268/398 total | PBT candidates: 268 | Tested: 268 (100%) | 1 pass, 268 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 398 |
| PBT candidates (from FUNCTION_INDEX) | 268 |
| **Tested (of PBT candidates)** | **268 / 268 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 268 / -1 |
| **Overall (tested / all functions)** | **268 / 398 (67%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 268 | 268 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 268 | 268 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 30 | 2 | 2 | 100% | covered |
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
