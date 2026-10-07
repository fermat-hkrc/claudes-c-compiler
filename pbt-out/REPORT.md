# PBT Campaign Report: encode_v_crypto_vi

## Summary

**Verdict:** 1 high: encode_v_crypto_vi emits major opcode 1110111 (OP-P) for vsm3c.vi / vsm4k.vi, so objects will not decode as RISC-V Vector Crypto (OP-V 1010111); plus 3 medium: extra operands, trailing v0.t, and out-of-range uimm5 are silently accepted.
**Date:** 2026-10-07
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_v_crypto_vi
**Tests:** 8
**Result:** 4 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_v_crypto_vi NOT LINKED in C++ reporter binaries; the Rust `cargo test --lib encode_v_crypto_vi` run executed the symbol (KATs and 1000-case properties). Sweep: 1 round (standard), manual audit of documented 3-op / format / isolation / swap / arity / opcode / extra / uimm-oob / v0.t. Closed: tier round spent.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_v_crypto_vi | 8 | 4 | reference, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_v_crypto_vi uses OP-P instead of OP-V

**Formal:** ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, (mnem,funct6) ∈ {(vsm3c.vi,101011),(vsm4k.vi,100001)}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) = Word(w) ∧ (w & 0x7F) = 0b1010111
**Contract evidence:** documented encoder/mod.rs:445 "Vector crypto (Zvk*) — uses OP-P encoding space per RVV Crypto spec" — the SUT claims RISC-V Cryptography Extensions Volume II, which encodes these instructions in OP-V (1010111)
**Documentation conflict:** vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" and encoder/mod.rs:445 assert OP-P. The comment cites the RVV Crypto spec but states the wrong major opcode; it does not declare OP-V invalid. Mark (not independently verified) against every historical draft — ratified Volume II uses OP-V.
**Severity:** high
**Counterexample:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(0)], 0b101011)
**Expected / Actual:** opcode 0b1010111 (Word 0xae002057) / opcode 0b1110111 (Word 0xae002077)
**Impact:** Assembled vsm3c.vi / vsm4k.vi will not execute as SM3/SM4 crypto on a spec-compliant Zvksh/Zvksed core.
**Root cause:** vector.rs:195 ORs OP_V_CRYPTO, defined as 0b1110111 at encoder/mod.rs:445, instead of OP-V 0b1010111.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:195`
```rust
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
```
**Suggested fix:** Pack OP-V.
```rust
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V;
```
**Bug report:** bug_reports/encode_v_crypto_vi_spec_opcode.md
**Repro seed:** vd=0, vs2=0, uimm=0, kind=0
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `119`,
 right: `87`: opcode must be OP-V 1010111 per RISC-V Crypto Volume II; got 0b1110111
minimal failing input: vd = 0, vs2 = 0, uimm = 0, kind = 0
```

### B2: encode_v_crypto_vi ignores extra operands

**Formal:** ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, extra ∈ Operand, funct6 ∈ {vsm3c.vi, vsm4k.vi}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), extra], funct6) = Err(_)
**Contract evidence:** inferred (3-operand rustdoc form at vector.rs:190; encode_instruction at encoder/mod.rs:1018,1021 passes operands through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(0), Imm(0)], 0b101011)
**Expected / Actual:** Err / Ok(Word(0xae002077))
**Impact:** A stray fourth token is assembled as a complete vsm3c.vi / vsm4k.vi instead of an assembler error.
**Root cause:** vector.rs:192-196 reads only operands 0..2 and never checks operands.len() == 3.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:192`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let uimm5 = get_imm(operands, 2)? as u32 & 0x1F;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject arity other than 3.
```rust
    if operands.len() != 3 {
        return Err(format!("vsm3c.vi/vsm4k.vi expect 3 operands, got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```
**Bug report:** bug_reports/encode_v_crypto_vi_extra_operand.md
**Repro seed:** cc c0fd4fa51e8959a8eeb2bbd3c144f15f56e01c3983d6e0aeafc2fb3664f302e0
**Raw output:**
```text
Test failed: extra operand Imm(0) must Err for crypto VI; got Ok(Word(2919243895))
minimal failing input: vd = 0, vs2 = 0, uimm = 0, extra = Imm(0), kind = 0
```

### B3: encode_v_crypto_vi truncates out-of-range uimm5

**Formal:** ∀ vd, vs2 ∈ {0..31}, uimm ∉ {0..31}, funct6 ∈ {vsm3c.vi, vsm4k.vi}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) = Err(_)
**Contract evidence:** documented vector.rs:190 "uimm5" — the documented domain is [0, 31]
**Documentation conflict:** (none) — the rustdoc names uimm5; it does not declare values outside [0, 31] valid
**Severity:** medium
**Counterexample:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(-1)], 0b101011)
**Expected / Actual:** Err / Ok(Word(0xae0f8077)) encoding uimm=31
**Impact:** Out-of-range round numbers wrap (`-1` → 31, `32` → 0) instead of being rejected.
**Root cause:** vector.rs:194 `get_imm(...) as u32 & 0x1F` masks without a range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:194`
```rust
    let uimm5 = get_imm(operands, 2)? as u32 & 0x1F;
```
**Suggested fix:** Reject immediates outside 0..=31.
```rust
    let imm = get_imm(operands, 2)?;
    if !(0..=31).contains(&imm) {
        return Err(format!("uimm5 out of range: {imm}"));
    }
    let uimm5 = imm as u32;
```
**Bug report:** bug_reports/encode_v_crypto_vi_uimm_oob.md
**Repro seed:** vd=0, vs2=0, uimm=-1, kind=0
**Raw output:**
```text
Test failed: vsm3c.vi v0, v0, -1 must Err (uimm5 domain [0, 31]); got Ok(Word(2920259703))
minimal failing input: vd = 0, vs2 = 0, uimm = -1, kind = 0
```

### B4: encode_v_crypto_vi ignores trailing v0.t

**Formal:** ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, funct6 ∈ {vsm3c.vi, vsm4k.vi}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), Symbol("v0.t")], funct6) = Err(_)
**Contract evidence:** documented vector.rs:190 "vm=1"; RISC-V Cryptography Extensions Volume II — Zvksh/Zvksed VI is not maskable
**Documentation conflict:** encoder/mod.rs:962 "TODO: masked variants (v0.t) are not yet supported; vm is hardcoded to 1 (unmasked)" — known limitation on an input the API accepts if the parser yields a fourth Symbol("v0.t"). Severity one step below impact (medium → low) would apply only if this were a documented limitation of crypto VI; the TODO is about RVV arithmetic masking, and crypto VI is not maskable. Keep medium.
**Severity:** medium
**Counterexample:** encode_v_crypto_vi([Reg("v0"), Reg("v0"), Imm(0), Symbol("v0.t")], 0b101011)
**Expected / Actual:** Err / Ok(Word(0xae002077)) with vm=1
**Impact:** `vsm3c.vi vd, vs2, uimm, v0.t` assembles as unmasked instead of being rejected.
**Root cause:** vector.rs:192-196 never inspects operands past index 2; vm is hardcoded to 1.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/riscv/assembler/encoder/vector.rs:192`
```rust
    let vd = get_vreg(operands, 0)?;
    let vs2 = get_vreg(operands, 1)?;
    let uimm5 = get_imm(operands, 2)? as u32 & 0x1F;
    let word = (funct6 << 26) | (1u32 << 25) | (vs2 << 20) | (uimm5 << 15) | (0b010 << 12) | (vd << 7) | OP_V_CRYPTO;
    Ok(EncodeResult::Word(word))
```
**Suggested fix:** Reject a fourth v0.t (and any extra operand). These instructions are not maskable.
```rust
    if operands.len() != 3 {
        return Err(format!("vsm3c.vi/vsm4k.vi expect 3 operands (not maskable), got {}", operands.len()));
    }
    let vd = get_vreg(operands, 0)?;
```
**Bug report:** bug_reports/encode_v_crypto_vi_mask_v0t.md
**Repro seed:** vd=0, vs2=0, uimm=0, kind=0
**Raw output:**
```text
Test failed: trailing v0.t must Err (Zvksh/Zvksed VI is not maskable); got Ok(Word(2919243895))
minimal failing input: vd = 0, vs2 = 0, uimm = 0, kind = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs | 8 properties + 4 passing KAT + 1 failing spec KAT + 5 failing regression witnesses |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_v_crypto_vi -- --test-threads=1
```

B1 opcode:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_spec_opcode -- --test-threads=1
```

B2 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_extra_operand -- --test-threads=1
```

B3 uimm oob:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_uimm_oob_m1 -- --test-threads=1
```

B4 mask v0.t:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_v_crypto_vi_regression_mask_v0t -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_v_crypto_vi_spec_opcode.md
- pbt-out/bug_reports/encode_v_crypto_vi_spec_opcode.html
- pbt-out/bug_reports/encode_v_crypto_vi_extra_operand.md
- pbt-out/bug_reports/encode_v_crypto_vi_extra_operand.html
- pbt-out/bug_reports/encode_v_crypto_vi_uimm_oob.md
- pbt-out/bug_reports/encode_v_crypto_vi_uimm_oob.html
- pbt-out/bug_reports/encode_v_crypto_vi_mask_v0t.md
- pbt-out/bug_reports/encode_v_crypto_vi_mask_v0t.html
- pbt-out/run/encode_v_crypto_vi.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-07 02:00 (campaign: coverage)
> Files: 16/16 scanned (100%) | Functions: 233/383 total | PBT candidates: 233 | Tested: 233 (100%) | 1 pass, 233 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 (100%) |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 233 |
| **Tested (of PBT candidates)** | **233 / 233 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 233 / -1 |
| **Overall (tested / all functions)** | **233 / 383 (61%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 233 | 233 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 233 | 233 | 0 | 100% |

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
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

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
