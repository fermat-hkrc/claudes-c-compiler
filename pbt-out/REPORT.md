# PBT Campaign Report: encode_push16

## Summary

**Verdict:** 4 high bugs (3 root causes): `encode_push16` only encodes integer immediates and rejects valid `pushw` r16, Sreg, bare memory, and segmented-memory forms that llvm-mc accepts, so any 16-bit register/memory/segment push fails to assemble.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_push16 (gp_integer.rs)
**Tests:** 9 properties (+ KAT + regression witnesses)
**Result:** 5 passing, 4 failing, 4 bugs
**Change surface:** 1 changed function (encode_push16), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (cargo symbol execution) — no Rust .profraw/.gcda; coverage_gaps returned NOT LINKED against unrelated OH binaries (ignored). `cargo test --lib encode_push16` exercised the real symbol.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_push16 | 9 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_push16 rejects r16 register operands

**Formal:** ∀ r ∈ {ax,cx,dx,bx,sp,bp,si,di}. encode_push16([Reg(r)]) = llvm-mc(`pushw %r`) = [0x66, 0x50+reg_num(r)]
**Contract evidence:** inferred (Intel SDM Vol.2 PUSH r16; AT&T `pushw %r16`; llvm-mc `-triple=i686`; sibling encode_push short form + encode_pop16 r16 path)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `pushw %ax` → Operand::Register("ax")
**Expected / Actual:** `[0x66, 0x50]` / `Err("unsupported pushw operand")`
**Impact:** 16-bit GP stack pushes cannot be assembled under the public `pushw` mnemonic.
**Root cause:** gp_integer.rs:389-400 — match only handles Immediate(Integer); Register hits catch-all Err.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400`
```rust
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Add r16 Register arm with `reg_size==2`, emit `0x66` then `0x50+n`.
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 2 {
                    return Err(format!("pushw requires r16, got {}", reg.name));
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x66);
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Bug report:** bug_reports/encode_push16_r16_unsupported.md
**Repro seed:** r16 = "ax" (proptest minimal)
**Raw output:**
```text
Test failed: SUT rejected valid r16 form `pushw %ax`: unsupported pushw operand; ...
minimal failing input: r16 = "ax"
```

### B2: encode_push16 rejects segment-register pushw

**Formal:** ∀ s ∈ {es,cs,ss,ds,fs,gs}. encode_push16([Reg(s)]) = llvm-mc(`pushw %s`)
**Contract evidence:** inferred (Intel SDM PUSH Sreg; llvm-mc encodings 66 06 / 0E / 16 / 1E / 0F A0 / 0F A8)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `pushw %es` → Operand::Register("es")
**Expected / Actual:** `[0x66, 0x06]` / `Err("unsupported pushw operand")`
**Impact:** Segment save under operand-size override cannot be encoded.
**Root cause:** gp_integer.rs:400 — no Sreg table; Register falls through to Err.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400`
```rust
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Sreg arm: push 0x66 then classic PUSH Sreg opcodes (es=06, cs=0E, ss=16, ds=1E, fs=0F A0, gs=0F A8).
```rust
            Operand::Register(reg) if is_segment_reg(&reg.name) => {
                self.bytes.push(0x66);
                match reg.name.as_str() {
                    "es" => { self.bytes.push(0x06); Ok(()) }
                    "cs" => { self.bytes.push(0x0E); Ok(()) }
                    "ss" => { self.bytes.push(0x16); Ok(()) }
                    "ds" => { self.bytes.push(0x1E); Ok(()) }
                    "fs" => { self.bytes.extend_from_slice(&[0x0F, 0xA0]); Ok(()) }
                    "gs" => { self.bytes.extend_from_slice(&[0x0F, 0xA8]); Ok(()) }
                    _ => Err(format!("cannot push {}", reg.name)),
                }
            }
```
**Bug report:** bug_reports/encode_push16_sreg_unsupported.md
**Repro seed:** sreg = "es"
**Raw output:**
```text
Test failed: SUT rejected valid Sreg form `pushw %es`: unsupported pushw operand; ...
minimal failing input: sreg = "es"
```

### B3: encode_push16 rejects memory operands (PUSH m16)

**Formal:** ∀ mem ∈ valid i686 memory forms. encode_push16([Mem(mem)]) = llvm-mc(`pushw mem`) = optional seg + 0x66 + FF /6 + ModR/M
**Contract evidence:** inferred (Intel SDM PUSH r/m16; sibling encode_push FF /6; core.rs emit_segment_prefix; llvm-mc)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `pushw (%ebx)` → Err; also `pushw %es:(%eax)` → Err
**Expected / Actual:** `[0x66, 0xff, 0x33]` / Err; segmented `[0x26, 0x66, 0xff, 0x30]` / Err
**Impact:** All 16-bit memory pushes (including segment overrides) fail to assemble.
**Root cause:** gp_integer.rs:400 — Memory falls through to catch-all Err; no FF /6 path and no emit_segment_prefix.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400`
```rust
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Memory arm with segment prefix, 0x66, FF /6:
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0x66);
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Bug report:** bug_reports/encode_push16_mem_unsupported.md
**Repro seed:** pushw (%eax)
**Raw output:**
```text
SUT rejected valid mem form `pushw (%eax)`: unsupported pushw operand
regression: encode_push16 must accept memory, got Err(unsupported pushw operand)
```

### B4: encode_push16 rejects segmented memory pushw

**Formal:** ∀ seg ∈ SREGS, base ∈ GP32, d ∈ i64. encode_push16([Mem(seg:base+d)]) = llvm-mc(`pushw %seg:d(%base)`)
**Contract evidence:** inferred (Intel SDM PUSH r/m16 + segment override; core.rs emit_segment_prefix; llvm-mc)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `pushw %es:(%eax)` → Err; expected `[0x26, 0x66, 0xff, 0x30]`
**Expected / Actual:** `[0x26, 0x66, 0xff, 0x30]` / `Err("unsupported pushw operand")`
**Impact:** Segment-overridden 16-bit memory pushes cannot be assembled.
**Root cause:** gp_integer.rs:400 — same missing Memory arm as B3; fix must call emit_segment_prefix before 0x66 + FF /6.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:400`
```rust
            _ => Err("unsupported pushw operand".to_string()),
```
**Suggested fix:** Same Memory arm as B3 with `emit_segment_prefix`.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0x66);
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Bug report:** bug_reports/encode_push16_mem_segment_unsupported.md
**Repro seed:** seg=es base=eax
**Raw output:**
```text
regression: encode_push16 must accept segmented mem, got Err(unsupported pushw operand)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_push16_pbt.rs | 9 properties + 6 KAT + 4 regression |
| src/backend/i686/assembler/encoder/mod.rs | `#[cfg(test)] mod encode_push16_pbt;` registration |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push16 -- --test-threads=1
# Single-case serial confirms:
cargo test --lib encode_push16_diff_r16 -- --test-threads=1
cargo test --lib encode_push16_diff_sreg -- --test-threads=1
cargo test --lib encode_push16_diff_mem -- --test-threads=1
cargo test --lib test_encode_push16_regression_r16_unsupported -- --test-threads=1
cargo test --lib test_encode_push16_regression_sreg_unsupported -- --test-threads=1
cargo test --lib test_encode_push16_regression_mem_unsupported -- --test-threads=1
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
- pbt-out/bug_reports/encode_push16_r16_unsupported.md (+ .html)
- pbt-out/bug_reports/encode_push16_sreg_unsupported.md (+ .html)
- pbt-out/bug_reports/encode_push16_mem_unsupported.md (+ .html)
- pbt-out/bug_reports/encode_push16_mem_segment_unsupported.md (+ .html)
- pbt-out/run/encode_push16_test.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 06:30 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 277/399 total | PBT candidates: 277 | Tested: 277 (100%) | 1 pass, 277 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 399 |
| PBT candidates (from FUNCTION_INDEX) | 277 |
| **Tested (of PBT candidates)** | **277 / 277 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 277 / -1 |
| **Overall (tested / all functions)** | **277 / 399 (69%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 277 | 277 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 277 | 277 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 31 | 11 | 11 | 100% | covered |
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
