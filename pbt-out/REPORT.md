# PBT Campaign Report: encode_pop16

## Summary

**Verdict:** 1 high + 1 high + 1 medium: `encode_pop16` omits the 0x66 prefix on segment-register `popw`, rejects all memory forms (`popw (%eax)`), and silently accepts r32/r8 GP names as r16 — three assembler correctness bugs vs llvm-mc/Intel POP.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_pop16
**Tests:** 8 properties (+ KAT + 3 regression witnesses)
**Result:** 4 passing, 3 failing, 1 retired (duplicate metamorphic of b1); 3 bugs
**Change surface:** 1 changed function (encode_pop16), 1 with properties, 0 error-handling-only changes without failure-path coverage (arity/cs/wrong-width negatives included)
**Coverage evidence:** none from native profraw (`coverage_gaps` found no .gcda/.profraw and listed unrelated OH binaries); execution evidenced by cargo lib tests printing SUT encodings from `encode_pop16` — record as file-level (symbol exercised via cargo test)
**Effort tier:** standard (proptest cases=1000; ≥1 metamorphic; 1 coverage_gaps sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_pop16 | 8 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_pop16 omits 0x66 on segment-register popw

**Formal:** ∀ s ∈ {es,ss,ds,fs,gs}. encode_pop16([s]) = llvm_mc("popw %s")
**Contract evidence:** inferred (llvm-mc i686 + Intel operand-size override on 16-bit mnemonic; sibling popl omits 0x66 by design) — author comment at system.rs:333 asserts the opposite for all segment pops
**Documentation conflict:** system.rs:333 `// Segment register pops don't use 0x66 prefix` — known limitation / incorrect claim for `popw` (true for `popl`); does not declare the input invalid; severity one step down from impact high → **medium (documented by the author)** would apply only if we treat it as admitted gap — here the comment states intended behavior that contradicts the public `popw` contract and llvm-mc; classify as **documented-and-violated** relative to the popw mnemonic contract, severity **high** (wrong machine code, not a disclosed "暂不支持")
**Severity:** high
**Counterexample:** `popw %es` then compare bytes — SUT `[0x07]`, expected `[0x66, 0x07]`
**Expected / Actual:** `[0x66, 0x07]` / `[0x07]`
**Impact:** Wrong encoding for every 16-bit segment pop; breaks gas/llvm-mc compatibility.
**Root cause:** system.rs:333-336 emits bare Sreg POP opcodes without 0x66 under the popw path.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:333`
```rust
                    // Segment register pops don't use 0x66 prefix
                    match reg.name.as_str() {
                        "es" => { self.bytes.push(0x07); Ok(()) }
```
**Suggested fix:** Push 0x66 before the Sreg opcode match in `encode_pop16` only.
```rust
                    self.bytes.push(0x66);
                    match reg.name.as_str() {
                        "es" => { self.bytes.push(0x07); Ok(()) }
```
**Bug report:** bug_reports/encode_pop16_sreg_missing_66.md
**Repro seed:** proptest minimal `sreg = "es"`
**Raw output:**
```text
left: `[7]`, right: `[102, 7]`: Sreg popw must match llvm-mc (incl. 0x66) for popw %es
```

### B2: encode_pop16 rejects memory operands (POP m16)

**Formal:** ∀ mem ∈ valid_i686_mem. encode_pop16([mem]) = llvm_mc("popw mem")
**Contract evidence:** inferred (Intel POP r/m16; llvm-mc `popw (%eax)` = `[66,8f,00]`; sibling `encode_pop` implements memory for popl)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `popw (%eax)` → SUT `Err("unsupported popw operand")`, expected `[0x66, 0x8f, 0x00]`
**Expected / Actual:** Ok`[0x66, 0x8f, 0x00]` / Err(`unsupported popw operand`)
**Impact:** Valid AT&T memory popw forms cannot be assembled.
**Root cause:** system.rs:348 blanket `_ => Err` with no `Operand::Memory` arm.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:348`
```rust
            _ => Err("unsupported popw operand".to_string()),
```
**Suggested fix:**
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0x66);
                self.bytes.push(0x8F);
                self.encode_modrm_mem(0, mem)
            }
```
**Bug report:** bug_reports/encode_pop16_mem_unsupported.md
**Repro seed:** `kind = 0, base = "eax"`
**Raw output:**
```text
SUT rejected `popw (%eax)`: unsupported popw operand
```

### B3: encode_pop16 accepts r32/r8 via reg_num aliasing

**Formal:** ∀ r ∈ R32 ∪ R8. encode_pop16([r]) is Err
**Contract evidence:** inferred (llvm-mc rejects `popw %eax`/`%al`; popw requires r16)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** `popw %eax` → Ok`[0x66, 0x58]` (same as `popw %ax`)
**Expected / Actual:** Err / Ok`[0x66, 0x58]`
**Impact:** Typos and wrong-width operands assemble silently as r16.
**Root cause:** system.rs:341 uses `reg_num` without `reg_size == 2` gate.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:340`
```rust
                } else {
                    let num = reg_num(&reg.name).ok_or("bad register")?;
                    self.bytes.push(0x66);
                    self.bytes.push(0x58 + num);
```
**Suggested fix:**
```rust
                    if reg_size(&reg.name) != 2 {
                        return Err(format!("popw requires r16, got {}", reg.name));
                    }
                    let num = reg_num(&reg.name).ok_or("bad register")?;
```
**Bug report:** bug_reports/encode_pop16_wrong_width_gp.md
**Repro seed:** `r32 = "eax"`
**Raw output:**
```text
popw %eax must be Err like llvm-mc, got Ok([66, 58])
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_pop16_pbt.rs | 8 properties + 5 KAT + 3 regression |
| src/backend/i686/assembler/encoder/mod.rs | +1 `mod encode_pop16_pbt` |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_pop16 -- --test-threads=1
cargo test --lib encode_pop16_diff_sreg -- --test-threads=1
cargo test --lib encode_pop16_diff_mem -- --test-threads=1
cargo test --lib encode_pop16_neg_r32 -- --test-threads=1
cargo test --lib test_encode_pop16_regression_sreg_missing_66 -- --test-threads=1
cargo test --lib test_encode_pop16_regression_mem_unsupported -- --test-threads=1
cargo test --lib test_encode_pop16_regression_r32_accepted -- --test-threads=1
```

Build contract (user-supplied): `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (swapped filter to `encode_pop16`).

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_pop16_sreg_missing_66.md (+ .html)
- pbt-out/bug_reports/encode_pop16_mem_unsupported.md (+ .html)
- pbt-out/bug_reports/encode_pop16_wrong_width_gp.md (+ .html)
- pbt-out/run/encode_pop16_test.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 03:31 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 266/397 total | PBT candidates: 266 | Tested: 266 (100%) | 1 pass, 266 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 397 |
| PBT candidates (from FUNCTION_INDEX) | 266 |
| **Tested (of PBT candidates)** | **266 / 266 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 266 / -1 |
| **Overall (tested / all functions)** | **266 / 397 (67%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 266 | 266 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 266 | 266 | 0 | 100% |

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
| encode_smsw | system.rs |
| encode_mov_cr | system.rs |
| encode_mov_seg | system.rs |
| encode_pop16 | system.rs |
