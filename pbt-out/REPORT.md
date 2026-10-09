# PBT Campaign Report: encode_in

## Summary

**Verdict:** 3 high/medium bugs in i686 `encode_in`: non-canonical register pairs encode as EC/ED, out-of-range port immediates silently truncate via `as u8`, and the AT&T `(%dx)` port form is rejected — same defect class as the sibling `encode_out`.
**Date:** 2026-10-09
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_in (src/backend/i686/assembler/encoder/system.rs)
**Tests:** 10 properties (+ 6 KAT + 3 regression witnesses)
**Result:** 7 passing, 3 failing properties → 3 bugs
**Change surface:** 1 changed function (encode_in), 1 with properties, 0 error-handling-only changes
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reports no native line-level profraw for this Rust cargo target; encode_in is linked into the lib test binary via encode_in_pbt.rs
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_in | 10 props (+KAT/regression) | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_in accepts non-canonical register pairs

**Formal:** ∀ m ∈ {inb,inw,inl}, src, dst. ¬(src=dx ∧ dst=data_reg(m)) ∧ llvm_mc rejects ⇒ encode_in(m,[src,dst]) = Err(_)
**Contract evidence:** inferred (Intel SDM IN fixed DX + AL/AX/EAX; llvm-mc rejects other pairs; AT&T syntax `inb %dx, %al`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `encode_in("inb", [%al, %al])` → Ok([0xec]); llvm-mc rejects
**Expected / Actual:** Err / Ok([0xec])
**Impact:** Invalid source operands assemble to a real IN DX-port instruction, so wrong I/O code is emitted silently.
**Root cause:** system.rs:94 matches any `(Register, Register)` and ignores names.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:94`
```rust
(Operand::Register(_src), Operand::Register(_dst)) => {
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xEC } else { 0xED });
    Ok(())
}
```
**Suggested fix:** Require `src.name == "dx"` and `dst.name` equal to the size-canonical data register; else Err.
```rust
(Operand::Register(src), Operand::Register(dst))
    if src.name == "dx"
        && matches!((size, dst.name.as_str()), (1, "al") | (2, "ax") | (4, "eax")) =>
{
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xEC } else { 0xED });
    Ok(())
}
```
**Bug report:** bug_reports/encode_in_wrong_registers.md
**Repro seed:** mnemonic="inb", src="al", dst="al"
**Raw output:**
```text
SUT accepted invalid IN `inb %al, %al` → [ec]; llvm-mc rejected
minimal failing input: mnemonic = "inb", src = "al", dst = "al"
```

### B2: encode_in silently truncates out-of-range port immediates

**Formal:** ∀ m ∈ {inb,inw,inl}, v ∉ imm8_accepted. llvm_mc rejects(m,$v) ⇒ encode_in(m,[imm(v),data_reg(m)]) = Err(_)
**Contract evidence:** inferred (Intel SDM imm8 port; llvm-mc rejects $256; signature accepts Immediate i64)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** `encode_in("inb", [$256, %al])` → Ok([0xe4, 0x00])
**Expected / Actual:** Err / Ok([0xe4, 0x00])
**Impact:** Port 256 becomes port 0; wrong I/O port selected without diagnostic.
**Root cause:** system.rs:102 `self.bytes.push(*val as u8)` truncates without range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:99`
```rust
(Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(_dst)) => {
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xE4 } else { 0xE5 });
    self.bytes.push(*val as u8);
    Ok(())
}
```
**Suggested fix:** Reject values outside the imm8 domain before casting.
```rust
if *val < -128 || *val > 255 {
    return Err(format!("IN port immediate out of imm8 range: {val}"));
}
self.bytes.push(*val as u8);
```
**Bug report:** bug_reports/encode_in_imm_truncation.md
**Repro seed:** mnemonic="inb", imm_v=256
**Raw output:**
```text
SUT silently encoded out-of-range port `inb $256, %al` → [e4, 00] (truncated?); llvm-mc rejected
minimal failing input: mnemonic = "inb", imm_v = 256
```

### B3: encode_in rejects AT&T (%dx) memory port form

**Formal:** ∀ m ∈ {inb,inw,inl}. encode_in(m, [mem(%dx), data_reg(m)]) = llvm_mc(m " (%dx), %" ++ data_reg(m))
**Contract evidence:** documented sibling x86/system.rs:75 "Also handle parenthesized form: inl (%dx), %eax"; llvm-mc accepts and emits EC/ED
**Documentation conflict:** (none for i686 — gap relative to sibling + llvm-mc)
**Severity:** medium
**Counterexample:** `encode_in("inb", [Memory(%dx), %al])` → Err("unsupported inb operands"); llvm-mc=[0xec]
**Expected / Actual:** Ok([0xec]) / Err("unsupported inb operands")
**Impact:** Valid AT&T forms used in kernel/boot code fail to assemble on the i686 backend.
**Root cause:** No `(Memory, Register)` match arm; falls through to `_ => Err`.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:93`
```rust
match (&ops[0], &ops[1]) {
    (Operand::Register(_src), Operand::Register(_dst)) => { ... }
    (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Register(_dst)) => { ... }
    _ => Err(format!("unsupported {} operands", mnemonic)),
}
```
**Suggested fix:** Add Memory,Register arm (mirror x86 sibling system.rs:83-87).
```rust
(Operand::Memory(_), Operand::Register(_)) => {
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.push(if size == 1 { 0xEC } else { 0xED });
    Ok(())
}
```
**Bug report:** bug_reports/encode_in_missing_dx_mem_form.md
**Repro seed:** mnemonic="inb"
**Raw output:**
```text
SUT rejected valid AT&T form `inb (%dx), %al`: unsupported inb operands; llvm-mc=[ec].
minimal failing input: mnemonic = "inb"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/i686/assembler/encoder/encode_in_pbt.rs | 10 properties + 6 KAT + 3 regressions |
| src/backend/i686/assembler/encoder/mod.rs | +1 `mod encode_in_pbt` registration |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_in_pbt -- --test-threads=1
# narrowed:
cargo test --lib encode_in_pbt::test_encode_in_regression_wrong_reg_al_al -- --test-threads=1
cargo test --lib encode_in_pbt::test_encode_in_regression_imm_256_truncated -- --test-threads=1
cargo test --lib encode_in_pbt::test_encode_in_regression_dx_mem_form -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_in_wrong_registers.md (+ .html)
- pbt-out/bug_reports/encode_in_imm_truncation.md (+ .html)
- pbt-out/bug_reports/encode_in_missing_dx_mem_form.md (+ .html)
- pbt-out/run/encode_in_pbt_serial.log
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-09 01:08 (campaign: coverage)
> Files: 16/17 scanned (94%) | Functions: 257/397 total | PBT candidates: 257 | Tested: 257 (100%) | 1 pass, 257 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 17 |
| Files scanned | 16 / 17 (94%) |
| Total functions (all files) | 397 |
| PBT candidates (from FUNCTION_INDEX) | 257 |
| **Tested (of PBT candidates)** | **257 / 257 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 257 / -1 |
| **Overall (tested / all functions)** | **257 / 397 (65%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 257 | 257 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 257 | 257 | 0 | 100% |

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
