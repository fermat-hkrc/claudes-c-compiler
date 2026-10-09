# PBT Campaign Report: encode_push (i686)

## Summary

**Verdict:** 6 failing properties / 6 bug reports (4 root-cause classes, worst severity **high**): `encode_push` drops segment overrides on memory operands (TLS/`%fs:`/`%gs:` silently wrong), accepts non-GP/r8 via `reg_num` alias as `pushl %eax`-class bytes, rejects valid Sreg PUSH forms, and encodes r16 without the required 0x66 prefix (stack delta wrong).
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_push (src/backend/i686/assembler/encoder/gp_integer.rs)
**Tests:** 12 properties + 6 KAT + 3 regression witnesses (proptest cases=1000)
**Result:** 6 properties passing, 6 failing (6 bug reports / 4 root causes); 4 KAT pass / 2 KAT fail; 3 regression fail
**Change surface:** 1 changed function (encode_push), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no Rust `.profraw` and C++-binary fallback marked encode_push NOT LINKED; cargo lib-test run exercised the real symbol (14 pass / 11 fail under `cargo test --lib encode_push_`). Recorded as file-level + cargo execution evidence.
**Effort tier:** standard
**Contract-surface sweep:** 1 round — added symbol-imm invariant + mixed-arity negative; both pass. Remaining fails are filed bugs, not untested branches.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_push | 12 props (+ KAT/regression) | 6 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_push omits segment-override prefix on memory operands

**Formal:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, b ∈ GP32. encode_push([Mem(seg:b)]) = llvm-mc("pushl %seg:(%b)")
**Contract evidence:** documented core.rs:31-42 emit_segment_prefix for all six segs; x86-64 sibling `encode_push` calls `emit_segment_prefix` before FF /6; Intel SDM 2.1.1
**Documentation conflict:** (none — code simply omits the call)
**Severity:** high
**Counterexample:** `pushl %fs:(%eax)` → expected `[0x64, 0xff, 0x30]`, actual `[0xff, 0x30]`
**Expected / Actual:** `[0x64, 0xff, 0x30]` / `[0xff, 0x30]`
**Impact:** Segmented memory PUSH (especially FS/GS TLS) silently uses the wrong segment at runtime.
**Root cause:** gp_integer.rs:374-377 pushes 0xFF then encode_modrm_mem without emit_segment_prefix.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:374`
```rust
            Operand::Memory(mem) => {
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Call emit_segment_prefix before the opcode.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Bug report:** bug_reports/encode_push_missing_segment_prefix.md
**Repro seed:** proptest cc f818f220e3b4019905d8b72a95279a923103d69a39a70ee313d391192c2b2a9a (seg=es,base=eax,disp=0)
**Raw output:**
```text
assertion failed: `(left == right)` left: `[255, 48]`, right: `[38, 255, 48]`: segment diff `pushl %es:(%eax)`
```

### B2: encode_push accepts non-GP / r8 registers via reg_num alias

**Formal:** ∀ x ∈ {xmm0..xmm7} ∪ {al..bh}. encode_push([Reg(x)]) = Err
**Contract evidence:** inferred (Intel SDM PUSH operand set is r/m32/imm/Sreg; llvm-mc rejects `pushl %xmm0` / `pushl %al`; sibling campaigns on encode_lea/encode_mov* same class)
**Documentation conflict:** registers.rs:4-15 intentionally lists xmm/mm aliases for ModRM numbering — that is a shared helper limitation, not a PUSH domain restriction; PUSH must still reject non-GP
**Severity:** high
**Counterexample:** `pushl %xmm0` → Ok([0x50]); `pushl %al` → Ok([0x50])
**Expected / Actual:** Err / Ok([0x50]) (same as pushl %eax)
**Impact:** Silent mis-assembly: invalid operands become PUSH EAX-class encodings with no diagnostic.
**Root cause:** gp_integer.rs:351-354 uses only reg_num, which maps xmm0/al/eax all to 0.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Gate on reg class/size; reject xmm/mm/r8.
```rust
                if is_xmm(&reg.name) || is_mm(&reg.name) || reg_size(&reg.name) == 1 {
                    return Err(format!("invalid push register {}", reg.name));
                }
```
**Bug report:** bug_reports/encode_push_accepts_non_gp_register.md
**Repro seed:** x="xmm0" / r8="al"
**Raw output:**
```text
encode_push must reject non-GP `xmm0`, got Ok([80])
```

### B3: encode_push rejects valid segment-register PUSH forms

**Formal:** ∀ s ∈ {es,cs,ss,ds,fs,gs}. encode_push([Reg(s)]) = llvm-mc("pushl %s")
**Contract evidence:** inferred (Intel SDM PUSH Sreg encodings; sibling encode_pop implements POP Sreg table at gp_integer.rs:408-416; llvm-mc accepts)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `pushl %es` → Err("bad register"); expected Ok([0x06])
**Expected / Actual:** Ok([0x06]) / Err("bad register")
**Impact:** Valid AT&T `push %es` / `%fs` / etc. cannot be assembled.
**Root cause:** Register arm only calls reg_num (no Sreg entries); does not use is_segment_reg like encode_pop.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Mirror encode_pop Sreg table with PUSH opcodes (ES=06, CS=0E, SS=16, DS=1E, FS=0F A0, GS=0F A8).
```rust
                if is_segment_reg(&reg.name) {
                    return match reg.name.as_str() {
                        "es" => { self.bytes.push(0x06); Ok(()) }
                        "cs" => { self.bytes.push(0x0E); Ok(()) }
                        "ss" => { self.bytes.push(0x16); Ok(()) }
                        "ds" => { self.bytes.push(0x1E); Ok(()) }
                        "fs" => { self.bytes.extend_from_slice(&[0x0F, 0xA0]); Ok(()) }
                        "gs" => { self.bytes.extend_from_slice(&[0x0F, 0xA8]); Ok(()) }
                        _ => Err(format!("cannot push {}", reg.name)),
                    };
                }
```
**Bug report:** bug_reports/encode_push_missing_sreg_forms.md
**Repro seed:** sreg="es"
**Raw output:**
```text
SUT rejected Sreg push `pushl %es`: bad register
```

### B4: encode_push encodes r16 as bare r32 short form (missing 0x66)

**Formal:** ∀ r16 ∈ {ax..di}. encode("push", [Reg(r16)]) = llvm-mc("push %r16") = [0x66, 0x50+n]
**Contract evidence:** inferred (Intel SDM operand-size override in 32-bit mode; llvm-mc `push %ax` → [0x66, 0x50]; mnemonic `push` routes to encode_push at mod.rs:191)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `push %ax` → sut=[0x50] mc=[0x66, 0x50]
**Expected / Actual:** [0x66, 0x50] / [0x50]
**Impact:** Wrong stack delta (4 bytes vs 2) — silent ABI/stack corruption for 16-bit pushes.
**Root cause:** reg_num aliases ax→same n as eax; no reg_size check / no 0x66. pushw only handles immediates.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Emit 0x66 when reg_size==2 for GP registers.
```rust
                if reg_size(&reg.name) == 2 {
                    self.bytes.push(0x66);
                }
                self.bytes.push(0x50 + num);
```
**Bug report:** bug_reports/encode_push_missing_r16_operand_size_prefix.md
**Repro seed:** r16="ax"
**Raw output:**
```text
left: `[80]`, right: `[102, 80]`: r16 push diff `push %ax`
```

### B5: encode_push metamorphic segment strip fails (missing override)

**Formal:** ∀ seg, m. encode(seg:m) starts with seg_prefix(seg) ∧ strip_seg(encode(seg:m)) = encode(m)
**Contract evidence:** documented core.rs:31-42 emit_segment_prefix (same root cause as B1)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** seg="es", base="eax", disp=0 → no 0x26 prefix, got [ff,30]
**Expected / Actual:** prefix 0x26 then bare body / [0xff, 0x30]
**Impact:** Reinforcing witness of B1 — wrong segment at runtime.
**Root cause:** gp_integer.rs:374-377 omits emit_segment_prefix (same statement as B1).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:374`
```rust
            Operand::Memory(mem) => {
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Call emit_segment_prefix before the opcode.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0xFF);
                self.encode_modrm_mem(6, mem)
            }
```
**Bug report:** bug_reports/encode_push_meta_missing_segment_prefix.md
**Repro seed:** seg="es", base="eax", disp=0
**Raw output:**
```text
segmented push must start with 0x26 for %es:, got [ff, 30]
```

### B6: encode_push accepts r8 via reg_num alias

**Formal:** ∀ r8 ∈ {al..bh}. encode_push([Reg(r8)]) = Err
**Contract evidence:** inferred (Intel SDM PUSH operand set; llvm-mc rejects pushl %al; same root class as B2)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** r8="al" → Ok([0x50])
**Expected / Actual:** Err / Ok([0x50])
**Impact:** Silent mis-assembly of 8-bit register PUSH.
**Root cause:** gp_integer.rs:351-354 reg_num maps al→0 same as eax.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:351`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.push(0x50 + num);
                Ok(())
            }
```
**Suggested fix:** Reject reg_size==1.
```rust
                if reg_size(&reg.name) == 1 {
                    return Err(format!("invalid push register {}", reg.name));
                }
```
**Bug report:** bug_reports/encode_push_accepts_r8_register.md
**Repro seed:** r8="al"
**Raw output:**
```text
encode_push must reject r8 `al` when llvm-mc does, got Ok([80])
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_push_pbt.rs | 12 properties + 6 KAT + 3 regression (+ 2 sweep KAT) |
| src/backend/i686/assembler/encoder/mod.rs | +1 `#[cfg(test)] mod encode_push_pbt;` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_push_ -- --test-threads=1
# single bug witnesses:
cargo test --lib encode_push_regression_missing_fs_prefix -- --test-threads=1 --exact
cargo test --lib encode_push_regression_non_gp_xmm0 -- --test-threads=1 --exact
cargo test --lib encode_push_regression_sreg_es -- --test-threads=1 --exact
cargo test --lib encode_push_diff_r16_via_push -- --test-threads=1
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
- pbt-out/bug_reports/encode_push_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_push_accepts_non_gp_register.md (+ .html)
- pbt-out/bug_reports/encode_push_missing_sreg_forms.md (+ .html)
- pbt-out/bug_reports/encode_push_missing_r16_operand_size_prefix.md (+ .html)
- pbt-out/bug_reports/encode_push_meta_missing_segment_prefix.md (+ .html)
- pbt-out/bug_reports/encode_push_accepts_r8_register.md (+ .html)
- pbt-out/run/encode_push_test.log
- pbt-out/run/encode_push_full2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 06:16 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 276/398 total | PBT candidates: 276 | Tested: 276 (100%) | 1 pass, 276 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 398 |
| PBT candidates (from FUNCTION_INDEX) | 276 |
| **Tested (of PBT candidates)** | **276 / 276 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 276 / -1 |
| **Overall (tested / all functions)** | **276 / 398 (69%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 276 | 276 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 276 | 276 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 30 | 10 | 10 | 100% | covered |
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
