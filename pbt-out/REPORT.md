# PBT Campaign Report: encode_bt (i686)

## Summary

**Verdict:** 3 unique high/medium defects in `encode_bt` (4 failing properties / bug reports): (1) high — memory forms omit segment-override prefixes so `%es:`/`%fs:` BT encodes as DS-default (diff + metamorphic witnesses); (2) high — out-of-range imm8 bit offsets truncate via `as u8` (`$256` -> `$0`); (3) medium — r16/r8 names are accepted via `reg_num` aliasing for *l forms.
**Date:** 2026-10-12
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_bt (src/backend/i686/assembler/encoder/gp_integer.rs)
**Tests:** 12 properties (+ 7 KAT + 4 regression witnesses)
**Result:** 9 passing properties, 4 failing properties, 4 bug reports (3 unique root causes)
**Change surface:** 1 function (`encode_bt`), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` returned no line-level data (build tree not re-instrumented); file-level matcher falsely reported NOT LINKED for Rust-mangled `encode_bt`; KAT + 1000-case proptest runs prove execution. Sweep round 1 complete: all documented behaviors have properties.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_bt | 13 props + 7 KAT + 4 regressions | 4 (3 unique RC) | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_bt omits segment override prefix on memory forms

**Formal:** ∀ mnem ∈ BT_L, ∀ form ∈ {reg_mem,imm_mem}, ∀ seg ∈ {es,cs,ss,ds,fs,gs}, ∀ bare_mem. encode_bt(..., seg:mem) = llvm-mc(... %seg:mem)
**Contract evidence:** inferred (core.rs:31-42 `emit_segment_prefix` documents all six overrides; x86-64 sibling `x87_misc.rs` encode_bt calls `emit_segment_prefix` on both mem arms; llvm-mc emits the prefix)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `btl %eax, %es:(%ebx)` then compare bytes — SUT `[0f,a3,03]`, expected `[26,0f,a3,03]`
**Expected / Actual:** `[26, 0f, a3, 03]` / `[0f, a3, 03]`
**Impact:** Segmented BT/BTS/BTR/BTC memory forms assemble without the override, so the instruction addresses the wrong segment (DS default) — silent wrong machine code.
**Root cause:** `gp_integer.rs:1033-1037` and `1045-1050` encode memory without calling `self.emit_segment_prefix(mem)` before the opcode.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1033`
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.encode_modrm_mem(src_num, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode on both memory arms.
```rust
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.encode_modrm_mem(src_num, mem)
```
**Bug report:** bug_reports/encode_bt_missing_segment_prefix.md
**Repro seed:** proptest cc f1147210be13e46d1e589d708267310c23cf835f4b10df820ac7664cc43e675a
**Raw output:**
```text
segment diff `btl %eax, %es:(%eax)`: sut=[0f, a3, 00] mc=[26, 0f, a3, 00]; expected prefix 0x26
```

### B2: encode_bt accepts r16/r8 GP names via reg_num aliasing

**Formal:** ∀ mnem ∈ {btl,btsl,btrl,btcl}, ∀ (src,dst) where a name is r16/r8 and llvm-mc rejects. encode_bt([%src,%dst], mnem) = Err(_)
**Contract evidence:** inferred (Intel SDM BT *l is r32,r/m32; llvm-mc rejects `btl %ax, %ebx`; registers.rs `reg_num` aliases widths)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `btl %ax, %ebx` → SUT `Ok([0f,a3,c3])` (same as `%eax,%ebx`)
**Expected / Actual:** `Err(...)` / `Ok([0f, a3, c3])`
**Impact:** Wrong-width register names silently assemble as r32 encodings with no diagnostic.
**Root cause:** `gp_integer.rs:1026-1031` calls `reg_num` with no `reg_size` gate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1026`
```rust
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, opcode_rr]);
                self.bytes.push(self.modrm(3, src_num, dst_num));
                Ok(())
            }
```
**Suggested fix:** Require `reg_size(&name) == 4` on both GP operands before `reg_num`.
```rust
                if reg_size(&src.name) != 4 || reg_size(&dst.name) != 4 {
                    return Err(format!("{mnemonic} requires 32-bit registers"));
                }
```
**Bug report:** bug_reports/encode_bt_wrong_width_alias.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
SUT accepted btl %ax, %ebx → Ok([15, 163, 195]); r16 not valid for btl
```

### B3: encode_bt truncates out-of-range bit-offset immediates

**Formal:** ∀ mnem ∈ BT_L, ∀ imm ∉ imm8_domain that llvm-mc rejects, ∀ dst ∈ GP32. encode_bt([$imm,%dst], mnem) = Err(_)
**Contract evidence:** inferred (Intel 0F BA /ext ib; llvm-mc rejects `$256`; SUT uses `*val as u8`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `btl $256, %eax` → SUT `Ok([0f,ba,e0,00])` (encodes as `$0`)
**Expected / Actual:** `Err(...)` / `Ok([0f, ba, e0, 00])`
**Impact:** An out-of-range bit index is silently truncated, so the wrong bit is tested/set/reset/complemented.
**Root cause:** `gp_integer.rs:1041` (and Imm→Mem/Label arms) push `*val as u8` with no range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1038`
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0xBA]);
                self.bytes.push(self.modrm(3, ext, dst_num));
                self.bytes.push(*val as u8);
                Ok(())
            }
```
**Suggested fix:** Reject values outside the imm8 domain before casting.
```rust
                if !(-128..=255).contains(val) {
                    return Err(format!("{mnemonic} immediate out of imm8 range: {val}"));
                }
                self.bytes.push(*val as u8);
```
**Bug report:** bug_reports/encode_bt_imm_oor_truncates.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
SUT accepted btl $256, %eax → Ok([15, 186, 224, 0]); imm must fit imm8
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_bt_pbt.rs | 12 properties + 7 KAT + 4 regressions |
| src/backend/i686/assembler/encoder/mod.rs | +1 `#[cfg(test)] mod encode_bt_pbt;` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt -- --test-threads=1
```

B1 segment:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt_regression_segment_es_reg_mem -- --test-threads=1
cargo test --lib encode_bt_diff_segment -- --test-threads=1
```

B2 wrong width:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt_regression_wrong_width_ax_ebx -- --test-threads=1
```

B3 imm OOR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_bt_regression_imm_oor_256 -- --test-threads=1
```

## Output Directories

- `pbt-out/REPORT.md` — this report
- `pbt-out/REPORT.html` — customer-facing overview (from report.json)
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/PLAN.md` — campaign phases
- `pbt-out/COVERAGE.md` — coverage ledger row for encode_bt
- `pbt-out/COVERAGE_STATUS.md` — coverage evidence summary
- `pbt-out/INVARIANTS.md` — encode_bt section appended
- `pbt-out/report.json` — machine-readable report
- `pbt-out/bug_reports/encode_bt_missing_segment_prefix.md` (+ .html)
- `pbt-out/bug_reports/encode_bt_wrong_width_alias.md` (+ .html)
- `pbt-out/bug_reports/encode_bt_imm_oor_truncates.md` (+ .html)
- `pbt-out/bug_reports/encode_bt_meta_segment_prefix.md` (+ .html)
- `pbt-out/run/encode_bt_test2.log` — full test log
- `src/backend/i686/assembler/encoder/encode_bt_pbt.rs` — harness

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-12 10:27 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 287/401 total | PBT candidates: 287 | Tested: 287 (100%) | 1 pass, 287 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 401 |
| PBT candidates (from FUNCTION_INDEX) | 287 |
| **Tested (of PBT candidates)** | **287 / 287 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 287 / -1 |
| **Overall (tested / all functions)** | **287 / 401 (72%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 287 | 287 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 287 | 287 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 33 | 21 | 21 | 100% | covered |
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
| encode_bsr_bsf | gp_integer.rs |
| encode_bt | gp_integer.rs |
