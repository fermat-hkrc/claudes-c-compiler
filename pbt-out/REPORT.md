# PBT Campaign Report: encode_lmsw (i686)

## Summary

**Verdict:** 2 root-cause bugs (3 reports): (1) high — `encode_lmsw` omits segment override prefixes on memory operands so `lmsw %es:(%eax)` / SIB forms drop 0x26; (2) medium — register arm accepts 32/8-bit names (`%eax`/`%al`) despite doc "16-bit register" and Intel LMSW r/m16.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_lmsw
**Tests:** 11 properties (+ 6 KAT + 4 regression witnesses)
**Result:** 8 properties passing, 3 failing; 2 root-cause bugs (3 reports)
**Change surface:** 1 changed function (encode_lmsw), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` returned no .gcda/.profraw and only unrelated OH C++ binaries; execution confirmed by `cargo test --lib encode_lmsw` (12 pass / 9 fail including KAT/regression)
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_lmsw | 11 properties | 2 root-cause (3 reports) | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_lmsw omits segment override prefix on memory operands

**Formal:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, base, disp. encode_lmsw([Mem seg:base+disp]) = llvm_mc(att with %seg:)
**Contract evidence:** inferred (x86 sibling `emit_rex_rm` before opcode at x86/.../system.rs:256; i686 `core.rs:31-42` `emit_segment_prefix`; llvm-mc i686 reference; same defect class as encode_invlpg/verw/prefetch/system_table)
**Documentation conflict:** (none) — doc states memory operand is accepted but does not name segment prefixes; contract from assembler encoding rules + sibling helpers
**Severity:** high
**Counterexample:** `lmsw %es:(%eax)` then compare bytes — SUT `[0f,01,30]`, llvm-mc `[26,0f,01,30]`
**Expected / Actual:** `[0x26,0x0f,0x01,0x30]` / `[0x0f,0x01,0x30]`
**Impact:** Segment-qualified LMSW assembles to default-DS addressing; privileged code that intentionally loads MSW from FS/ES/etc. gets wrong machine code.
**Root cause:** `system.rs:214-216` memory arm never calls `emit_segment_prefix(mem)` before emitting `0F 01`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(6, mem)
            }
```
**Bug report:** bug_reports/encode_lmsw_missing_segment_prefix.md
**Repro seed:** proptest `cc fd7824514f31a10119584b0a7394ddd6e0080ce571361bbd5a6698929f08078b` (seg=es, base=eax, disp=0)
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 1, 48]`,
 right: `[38, 15, 1, 48]`: segment prefix diff for `lmsw %es:(%eax)`: SUT=[0f, 01, 30] llvm-mc=[26, 0f, 01, 30]
minimal failing input: seg = "es", base = "eax", disp = 0
```

### B3: encode_lmsw omits segment override prefix on SIB memory operands

**Formal:** ∀ seg, base, index≠esp, scale, disp. encode_lmsw([Mem seg:SIB]) = llvm_mc(att)
**Contract evidence:** inferred (same as B1 — emit_segment_prefix / llvm-mc / x86 sibling)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `lmsw %es:(%eax,%eax,1)` — SUT `[0f,01,34,00]`, llvm-mc `[26,0f,01,34,00]`
**Expected / Actual:** `[0x26,0x0f,0x01,0x34,0x00]` / `[0x0f,0x01,0x34,0x00]`
**Impact:** Same as B1 for SIB forms.
**Root cause:** Same statement as B1 (`system.rs:214-216`).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Same as B1 — call `emit_segment_prefix` before opcode.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(6, mem)
            }
```
**Bug report:** bug_reports/encode_lmsw_missing_segment_prefix_sib.md
**Repro seed:** (deterministic first shrink: seg=es, base=eax, index=eax, scale=1, disp=0)
**Raw output:**
```text
seg+SIB diff for `lmsw %es:(%eax,%eax,1)`: SUT=[0f, 01, 34, 00] llvm-mc=[26, 0f, 01, 34, 00]
minimal failing input: seg = "es", base = "eax", index = "eax", scale = 1, disp = 0
```

### B2: encode_lmsw accepts 32-bit and 8-bit registers (r/m16 only)

**Formal:** ∀ bad ∈ {Reg32, Reg8}. encode_lmsw([bad]) = Err(...)
**Contract evidence:** documented `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:202` "Accepts a 16-bit register or memory operand." + Intel SDM LMSW r/m16 + llvm-mc rejects `%eax`/`%al`
**Documentation conflict:** system.rs:202 "Accepts a 16-bit register or memory operand." — asserts 16-bit register form; code accepts any name `reg_num` knows (eax/al alias to same rm). Class: contract the code violates.
**Severity:** medium
**Counterexample:** `lmsw %eax` → SUT Ok(`[0f,01,f0]`) (same as `%ax`); llvm-mc rejects
**Expected / Actual:** Err / Ok(`[0x0f,0x01,0xf0]`)
**Impact:** Width mistakes on LMSW are silently accepted as 16-bit encodings, diverging from gas/llvm-mc and the function's own doc.
**Root cause:** `system.rs:208-212` uses `reg_num` without gating `reg_size(&reg.name) == 2`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:208`
```rust
            Operand::Register(reg) => {
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.bytes.push(self.modrm(3, 6, rm));
                Ok(())
            }
```
**Suggested fix:** Reject non-16-bit register names.
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 2 {
                    return Err(format!("lmsw requires 16-bit register, got {}", reg.name));
                }
                let rm = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.bytes.push(self.modrm(3, 6, rm));
                Ok(())
            }
```
**Bug report:** bug_reports/encode_lmsw_accepts_non_r16_register.md
**Repro seed:** proptest `cc 8bfd22a7ff0ba7e4839c0fc93eb78bd3f244d48d4db157628933e60a841645e1` (kind=2, bad_reg=eax)
**Raw output:**
```text
Test failed: SUT accepted invalid-width register `lmsw %eax` → [0f, 01, f0]; LMSW requires r/m16 (doc: 16-bit register; llvm-mc rejects).
minimal failing input: kind = 2, bad_reg = "eax", imm = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs | 11 properties + 6 KAT + 4 regression |
| src/backend/i686/assembler/encoder/mod.rs | +1 `#[cfg(test)] mod encode_lmsw_pbt;` |

## Reproduction

Whole suite (serial, build-contract form):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lmsw -- --test-threads=1
```

B1 segment prefix:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lmsw_diff_segment -- --test-threads=1
cargo test --lib test_encode_lmsw_regression_missing_es_prefix -- --test-threads=1
```

B2 non-r16 register:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lmsw_neg_bad_operand -- --test-threads=1
cargo test --lib test_encode_lmsw_regression_rejects_eax -- --test-threads=1
```

## Output Directories

- `pbt-out/REPORT.md` — this report
- `pbt-out/REPORT.html` — customer-facing overview (from report.json)
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/PLAN.md` — campaign phases
- `pbt-out/COVERAGE.md` — coverage ledger row for encode_lmsw
- `pbt-out/COVERAGE_STATUS.md` — this-campaign coverage status
- `pbt-out/report.json` — machine-readable report
- `pbt-out/bug_reports/encode_lmsw_missing_segment_prefix.md` (+ .html)
- `pbt-out/bug_reports/encode_lmsw_missing_segment_prefix_sib.md` (+ .html)
- `pbt-out/bug_reports/encode_lmsw_accepts_non_r16_register.md` (+ .html)
- `pbt-out/run/encode_lmsw_test.log` — full test log
- `pbt-out/INVARIANTS.md` — updated with encode_lmsw notes

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 02:27 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 262/397 total | PBT candidates: 262 | Tested: 262 (100%) | 1 pass, 262 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 397 |
| PBT candidates (from FUNCTION_INDEX) | 262 |
| **Tested (of PBT candidates)** | **262 / 262 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 262 / -1 |
| **Overall (tested / all functions)** | **262 / 397 (66%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 262 | 262 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 262 | 262 | 0 | 100% |

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
