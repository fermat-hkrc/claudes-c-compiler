# PBT Campaign Report: encode_tlbi

## Summary

**Verdict:** 3 medium, 1 low: encode_tlbi silently encodes missing Xt as XZR, extra Xt on no-Xt ops, and W/SP/SIMD as X registers; it also rejects ARM default-CPU ops (alle2/alle3/vae3/vale3) that llvm-mc/gas assemble.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_tlbi
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_tlbi), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_tlbi_neg_invalid_reg and encode_tlbi_neg_unknown_op. Closed: every documented behavior has a property; tier round spent.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_tlbi | 10 | 4 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_tlbi defaults a missing Xt to XZR

**Formal:** ∀ op ∈ XtRequired. llvm_mc("tlbi " + op) = Err ⇒ encode_tlbi([], op) = Err
**Contract evidence:** inferred (ARM ARM / llvm-mc require Xt for VA*/VALE*/VAAE*/VAALE*/ASIDE*/IPAS2*/R*; README.md:12 gas-compat)
**Documentation conflict:** (none) — system.rs:485 `31 // xzr` is a producing-statement default, not a domain restriction declaring missing Xt valid
**Severity:** medium
**Counterexample:** encode_tlbi([], "vale1is")
**Expected / Actual:** Err / Ok(Word(0xd50883bf))
**Impact:** Dropping the register still assembles; the invalidate uses XZR instead of being rejected as GNU gas / llvm-mc would
**Root cause:** system.rs:483-485 sets Rt=31 whenever no comma is present, without checking that the matched op requires Xt
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:483`
```rust
    } else {
        31 // xzr
    };
```
**Suggested fix:** Return Err on Xt-required ops with no register operand
```rust
    } else if needs_xt(op_name) {
        return Err(format!("tlbi: {} requires a register", op_name));
    } else {
        31 // xzr
    };
```
**Bug report:** bug_reports/encode_tlbi_missing_xt.md
**Repro seed:** (deterministic regression)
**Raw output:** `Test failed: Xt-required TLBI without register must Err (llvm-mc rejects tlbi vale1is); SUT raw "vale1is": Word(3574105023). minimal failing input: raw = "vale1is"`

### B2: encode_tlbi accepts a register on no-Xt ops

**Formal:** ∀ op ∈ NoXt, xt ∈ ValidXt. llvm_mc("tlbi " + op + ", " + xt) = Err ⇒ encode_tlbi([], op + ", " + xt) = Err
**Contract evidence:** inferred (ARM ARM / llvm-mc: "specified tlbi op does not use a register"; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tlbi([], "vmalle1is, x0")
**Expected / Actual:** Err / Ok(Word(0xd5088300))
**Impact:** `tlbi vmalle1is, x0` is assembled with Rt=x0 instead of rejected; the word is not the architectural VMALLE1IS encoding
**Root cause:** system.rs:480-485 parse optional Rt for every op; system.rs:537 patches bits[4:0] even when Rt must stay XZR
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:537`
```rust
    let word = (base & !0x1F) | rt;
```
**Suggested fix:** Reject a second operand on no-Xt ops
```rust
    if !needs_xt(op_name) && parts.len() > 1 {
        return Err(format!("tlbi: {} does not use a register", op_name));
    }
```
**Bug report:** bug_reports/encode_tlbi_extra_xt.md
**Repro seed:** (deterministic regression)
**Raw output:** `Test failed: no-Xt TLBI with register must Err (llvm-mc rejects tlbi vmalle1is, x0); SUT raw "vmalle1is, x0": Word(3574104832). minimal failing input: op = "vmalle1is", xt = "x0"`

### B3: encode_tlbi accepts W/SP/SIMD as Xt

**Formal:** ∀ op ∈ XtRequired, bad ∈ WrongRegClass. llvm_mc("tlbi " + op + ", " + bad) = Err ⇒ encode_tlbi([], op + ", " + bad) = Err
**Contract evidence:** inferred (ARM ARM TLBI Xt is a 64-bit GPR / XZR / LR; llvm-mc "invalid operand for instruction")
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_tlbi([], "vale1is, w0")
**Expected / Actual:** Err / Ok(Word(0xd50883a0)) (same as x0)
**Impact:** A W or SIMD operand is encoded as the corresponding X register; gas / llvm-mc refuse the same text
**Root cause:** system.rs:482 uses parse_reg_num, which accepts w/sp/simd prefixes, with no is_64bit_reg check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:482`
```rust
        parse_reg_num(reg_str).ok_or_else(|| format!("tlbi: invalid register '{}'", reg_str))?
```
**Suggested fix:** Require a 64-bit GPR name after parsing
```rust
        if !is_64bit_gpr(reg_str) {
            return Err(format!("tlbi: Xt must be a 64-bit GPR, got '{}'", reg_str));
        }
```
**Bug report:** bug_reports/encode_tlbi_wrong_reg_class.md
**Repro seed:** (deterministic regression)
**Raw output:** `Test failed: TLBI with non-X register must Err (llvm-mc rejects tlbi vale1is, w0); SUT raw "vale1is, w0": Word(3574104992). minimal failing input: op = "vale1is", bad = "w0"`

### B4: encode_tlbi rejects ARM default-CPU TLBI ops

**Formal:** ∀ op ∈ ArmDefaultTlbi, xt ∈ ValidXt∪{ε}. well_formed(op, xt) ⇒ encode_tlbi([], asm(op, xt)) = Word(llvm_mc("tlbi " + asm(op, xt)))
**Contract evidence:** documented limitation encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen."
**Documentation conflict:** encoder/mod.rs:4 admits a codegen subset on an input the assembler API accepts (README.md:12 gas-compat; README.md:239 lists tlbi). Limitation on accepted input, not a domain restriction — filed one step down
**Severity:** low (documented by the author)
**Counterexample:** encode_tlbi([], "vae3is, x0"); also alle2, alle3, alle3is, vae3, vale3, vale3is
**Expected / Actual:** Ok(Word(0xd50e8320)) / Err("unsupported tlbi operation: vae3is")
**Impact:** Valid GNU `tlbi alle2` / `tlbi vae3is, x0` fails to assemble
**Root cause:** system.rs:534 — the match table omits alle2 and all EL3 variants llvm-mc accepts on the default CPU
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:534`
```rust
        _ => return Err(format!("unsupported tlbi operation: {}", op_name)),
```
**Suggested fix:** Add the missing ARMv8.0 encodings
```rust
        "alle2"     => 0xd50c871f,
        "alle3is"   => 0xd50e831f,
        "alle3"     => 0xd50e871f,
        "vae3is"    => 0xd50e8320,
        "vae3"      => 0xd50e8720,
        "vale3is"   => 0xd50e83a0,
        "vale3"     => 0xd50e87a0,
```
**Bug report:** bug_reports/encode_tlbi_unimplemented_arm_ops.md
**Repro seed:** cc 75f53c0582588bfe0a9e8730a8931d2fba4e14c7fe82d9d69dde0e947fc900e1
**Raw output:** `Test failed: ARM TLBI op "vae3is" must encode (llvm-mc accepts tlbi vae3is, x0): "unsupported tlbi operation: vae3is". minimal failing input: kind = 1, i = 12, t = 0`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs | 10 properties + 6 KAT + 4 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_tlbi -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_missing_xt -- --test-threads=1
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_extra_xt -- --test-threads=1
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_w0 -- --test-threads=1
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_tlbi_regression_alle2 -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/report.json
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_tlbi_missing_xt.md
- pbt-out/bug_reports/encode_tlbi_missing_xt.html
- pbt-out/bug_reports/encode_tlbi_extra_xt.md
- pbt-out/bug_reports/encode_tlbi_extra_xt.html
- pbt-out/bug_reports/encode_tlbi_wrong_reg_class.md
- pbt-out/bug_reports/encode_tlbi_wrong_reg_class.html
- pbt-out/bug_reports/encode_tlbi_unimplemented_arm_ops.md
- pbt-out/bug_reports/encode_tlbi_unimplemented_arm_ops.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 03:58 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 162/307 total | PBT candidates: 162 | Tested: 162 (100%) | 1 pass, 162 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 162 |
| **Tested (of PBT candidates)** | **162 / 162 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 162 / -1 |
| **Overall (tested / all functions)** | **162 / 307 (53%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 162 | 162 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 162 | 162 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 10 | 11 | 110% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 11 | 11 | 100% | covered |
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
