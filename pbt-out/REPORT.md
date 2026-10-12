# PBT Campaign Report: encode_setcc (i686)

## Summary

**Verdict:** 2 high bugs in `encode_setcc`: (1) memory forms omit segment-override prefixes so `sete %es:(%eax)` drops 0x26; (2) non-r8 registers (`%ax`/`%eax`/`%xmm0`) are accepted via `reg_num` aliasing and encoded as the corresponding r8.
**Date:** 2026-10-12
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_setcc (src/backend/i686/assembler/encoder/gp_integer.rs)
**Tests:** 8 properties (+ KAT gates + 4 deterministic regressions); proptest cases=1000
**Result:** 6 passing, 2 failing properties; 2 bugs
**Change surface:** 1 changed function (encode_setcc), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (pre-campaign build tree); binary symbol scan said NOT LINKED for the mangled `pub(super)` method, but all properties exercise the production path via `InstructionEncoder::encode` → dispatch → `encode_setcc` (confirmed by differential byte disagreements). Recorded as file-level.
**Effort tier:** standard (≈30 min, ≥1000 cases, ≥1 metamorphic/differential, 1 contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_setcc | 8 properties (+KAT/regressions) | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_setcc omits segment-override prefix on memory operands

**Formal:** ∀ m ∈ DISPATCHED_SETCC, seg ∈ {es,cs,ss,ds,fs,gs}, base ∈ GP32, d ∈ edge_disp. llvm_mc(m " %"seg":" mem) = encode_setcc(m, [Mem(seg,base,d)])
**Contract evidence:** inferred (Intel SETcc memory form + llvm-mc `-triple=i686` + sibling `emit_segment_prefix` at core.rs:31; same class as encode_smsw/invlpg/prefetch)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `sete %es:(%eax)` then compare bytes — SUT `[0f,94,00]` vs llvm-mc `[26,0f,94,00]`
**Expected / Actual:** `[0x26,0x0f,0x94,0x00]` / `[0x0f,0x94,0x00]`
**Impact:** Segmented memory SETcc forms assemble without the override, so the CPU uses the wrong segment at run time.
**Root cause:** `gp_integer.rs:1089-1091` memory arm never calls `emit_segment_prefix(mem)` before `0F 90+cc`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1089`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.encode_modrm_mem(0, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` first.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.encode_modrm_mem(0, mem)
            }
```
**Bug report:** bug_reports/encode_setcc_missing_segment_prefix.md
**Repro seed:** proptest cc 745de6caff4af69b316bc2ee32328a0fa6a18adf428d0708cbce340852d76bf8 (segment base form)
**Raw output:**
```text
segment prefix diff for `sete %es:(%eax)`: SUT=[0f, 94, 00] llvm-mc=[26, 0f, 94, 00]
minimal failing input: m = "sete", seg = "es", base = "eax", disp = 0
```

### B2: encode_setcc accepts non-r8 register operands via reg_num aliasing

**Formal:** ∀ m ∈ DISPATCHED_SETCC, r ∈ R16∪R32∪NON_GP_ALIASED. llvm_mc rejects m "%" r ∧ encode_setcc(m,[Reg(r)]) = Err
**Contract evidence:** inferred (Intel SDM SETcc r/m8-only; llvm-mc rejects `sete %ax` / `sete %eax`; `reg_num` at registers.rs:4 aliases widths)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `sete %ax` → Ok([0f,94,c0]) (identical to `sete %al`); also `%eax`, `%xmm0`
**Expected / Actual:** Err(invalid operand) / Ok([0x0f,0x94,0xc0])
**Impact:** Accidental r16/r32/non-GP names silently assemble as the aliased r8, hiding bugs in callers.
**Root cause:** `gp_integer.rs:1083` uses bare `reg_num` with no `reg_size == 1` (and no non-GP reject).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1083`
```rust
            Operand::Register(reg) => {
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.bytes.push(self.modrm(3, 0, num));
                Ok(())
            }
```
**Suggested fix:** Gate on `reg_size(&reg.name) == 1` and reject xmm/mm/st/ymm prefixes before `reg_num`.
```rust
            Operand::Register(reg) => {
                if reg_size(&reg.name) != 1
                    || reg.name.starts_with("xmm")
                    || reg.name.starts_with("mm")
                    || reg.name.starts_with("st")
                    || reg.name.starts_with("ymm")
                {
                    return Err(format!("setcc requires r8 register, got {}", reg.name));
                }
                let num = reg_num(&reg.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.bytes.push(self.modrm(3, 0, num));
                Ok(())
            }
```
**Bug report:** bug_reports/encode_setcc_accepts_non_r8_register.md
**Repro seed:** (deterministic; minimal `r="ax"`, `m="sete"`)
**Raw output:**
```text
SUT accepted invalid-width `sete %ax` → [0f, 94, c0]; SETcc is r/m8 only (Intel SDM; llvm-mc rejects).
minimal failing input: r = "ax", m = "sete"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_setcc_pbt.rs | 8 properties + 7 KAT + 4 regressions |
| src/backend/i686/assembler/encoder/mod.rs | +1 `mod encode_setcc_pbt` registration |

## Reproduction

Whole suite (serial):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_setcc -- --test-threads=1
```

B1 only:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_setcc_regression_missing_es_prefix -- --test-threads=1 --exact
cargo test --lib encode_setcc_diff_segment_llvm_mc -- --test-threads=1 --exact
```

B2 only:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_setcc_regression_rejects_ax -- --test-threads=1 --exact
cargo test --lib encode_setcc_neg_wrong_width -- --test-threads=1 --exact
```

## Output Directories

- `pbt-out/REPORT.md` — this report
- `pbt-out/REPORT.html` — customer-facing overview (from report.json)
- `pbt-out/PROPERTIES.md` — property ledger
- `pbt-out/PLAN.md` — campaign checklist
- `pbt-out/COVERAGE.md` — coverage ledger row
- `pbt-out/COVERAGE_STATUS.md` — coverage status
- `pbt-out/report.json` — machine-readable report
- `pbt-out/bug_reports/encode_setcc_missing_segment_prefix.md` (+ `.html`)
- `pbt-out/bug_reports/encode_setcc_accepts_non_r8_register.md` (+ `.html`)
- `pbt-out/run/encode_setcc_test2.log` — full test log
- `pbt-out/INVARIANTS.md` — updated with encode_setcc notes
- `pbt-out/FUNCTION_INDEX.md` — encode_setcc marked PBT candidate yes

## Contract-surface sweep

One `coverage_gaps` round (standard tier). No line-level data; file-level note recorded. Documented behaviors already covered by the 8 properties (r8 differential, mem base/SIB/abs, segment, opcode invariant, alias metamorphic, arity/bad-op negatives, wrong-width negatives). Close reason: tier sweep round complete + documented surface has properties.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-12 10:41 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 288/401 total | PBT candidates: 288 | Tested: 288 (100%) | 1 pass, 288 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 401 |
| PBT candidates (from FUNCTION_INDEX) | 288 |
| **Tested (of PBT candidates)** | **288 / 288 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 288 / -1 |
| **Overall (tested / all functions)** | **288 / 401 (72%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 288 | 288 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 288 | 288 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 21 | 21 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 31 | 31 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 33 | 22 | 22 | 100% | covered |
| load_store.rs | 20 | 19 | 19 | 100% | covered |
| neon.rs | 68 | 63 | 63 | 100% | covered |
| pseudo.rs | 44 | 19 | 19 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_setcc | gp_integer.rs |
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
