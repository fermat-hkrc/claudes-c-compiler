# PBT Campaign Report: encode_mrs

## Summary

**Verdict:** 1 high: encode_mrs encodes `cntv_cval_el0` as S3_3_C14_C3_4 (op2=4) instead of ARM CNTV_CVAL_EL0 S3_3_C14_C3_2, so a virtual-timer compare-value read hits the wrong sysreg; plus 4 medium (write-only OSLAR_EL1 accepted, extra operands ignored, Wt dest accepted, out-of-range S-form masked).
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_mrs
**Tests:** 9 properties (plus 4 KAT + 5 regression witnesses)
**Result:** 4 passing, 5 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). cargo test --lib encode_mrs executed the production symbol (KAT + 1000-case properties). Sweep: tier round spent; documented behaviors have properties.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_mrs | 9 | 5 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_mrs encodes write-only OSLAR_EL1

**Formal:** ∀ name ∈ NAMED, xt ∈ Xt. let asm = "mrs "+xt+", "+name; let sut = encode_mrs([Reg(xt), Symbol(name)]); (llvm-mc(asm)=Ok(w) ∧ sut=Ok(Word(w))) ∨ (llvm-mc(asm)=Err ∧ sut=Err)
**Contract evidence:** inferred (llvm-mc / ARM MRS requires a readable system register; README.md:12 gas-compatible assembly)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mrs([Reg("x0"), Symbol("oslar_el1")])  (`mrs x0, oslar_el1`)
**Expected / Actual:** Err (llvm-mc: expected readable system register) / Ok(Word(0xd5301080))
**Impact:** A write-only OS-lock register is assembled as MRS; the object file contains a read the architecture does not define.
**Root cause:** system.rs:131 `"oslar_el1" => 0x8084` is in the MRS named table; OSLAR_EL1 is MSR-only.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:131`
```rust
        "oslar_el1" => 0x8084,
```
**Suggested fix:** Drop oslar_el1 from the MRS table so unknown-name handling returns Err.
```rust
        // oslar_el1 is write-only (MSR); do not match it here
```
**Bug report:** bug_reports/encode_mrs_oslar_el1_write_only.md
**Repro seed:** cc bcdcbef1e10d742ca3362a6c8cb12669a33a82305d4a6265718d96d9cb96118a
**Raw output:**
```text
Test failed: SUT encoded mrs x0, oslar_el1 as d5301080, llvm-mc rejected: llvm-mc error: <stdin>:1:9: error: expected readable system register
minimal failing input: name = "oslar_el1", xt = "x0"
```

### B2: encode_mrs encodes CNTV_CVAL_EL0 with the wrong sysreg field

**Formal:** ∀ name ∈ NAMED, xt ∈ Xt. llvm-mc("mrs "+xt+", "+name)=Ok(w) ⇒ encode_mrs([Reg(xt), Symbol(name)])=Ok(Word(w))
**Contract evidence:** inferred (ARM CNTV_CVAL_EL0 is S3_3_C14_C3_2; llvm-mc encoding 0xd53be340; README.md:12 gas-compatible)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_mrs([Reg("x0"), Symbol("cntv_cval_el0")])  (`mrs x0, cntv_cval_el0`)
**Expected / Actual:** Ok(Word(0xd53be340)) / Ok(Word(0xd53be380))
**Impact:** A virtual-timer compare-value read hits S3_3_C14_C3_4 (op2=4) instead of CNTV_CVAL_EL0, so sampled timer values are wrong.
**Root cause:** system.rs:125 `"cntv_cval_el0" => 0xdf1c` uses op2=4; ARM encoding is 0xdf1a (op2=2).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:125`
```rust
        "cntv_cval_el0" => 0xdf1c,
```
**Suggested fix:** Use the ARM encoding 0xdf1a.
```rust
        "cntv_cval_el0" => 0xdf1a,
```
**Bug report:** bug_reports/encode_mrs_cntv_cval_el0_encoding.md
**Repro seed:** cc e031e081d69a61ee97d20c3f25ccd16f53422009f1be332f3b287082b8e4dc3d
**Raw output:**
```text
Test failed: assertion failed: `(left == right)` left: `3577471872`, right: `3577471808`: SUT vs llvm-mc for mrs x0, cntv_cval_el0
minimal failing input: name = "cntv_cval_el0", xt = "x0"
```

### B3: encode_mrs ignores extra operands

**Formal:** ∀ name ∈ NAMED, xt ∈ Xt, extra ∈ Operand. llvm-mc("mrs "+xt+", "+name+", "+extra)=Err ⇒ encode_mrs([Reg(xt), Symbol(name), extra])=Err
**Contract evidence:** inferred (llvm-mc rejects arity>2; system.rs:55 "MRS Xt, system_reg" is a two-operand form; README.md:12)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mrs([Reg("x0"), Symbol("sp_el0"), Reg("x0")])  (`mrs x0, sp_el0, x0`)
**Expected / Actual:** Err (llvm-mc: invalid operand) / Ok(Word(0xd5384100))
**Impact:** Trailing typos assemble as a silent two-operand MRS.
**Root cause:** system.rs:56–57 read only operands[0] and operands[1]; len is never checked.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:56`
```rust
    let (rt, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Reject a slice whose length is not 2.
```rust
    if operands.len() != 2 {
        return Err("mrs: expected Xt, system_reg".to_string());
    }
```
**Bug report:** bug_reports/encode_mrs_extra_operand.md
**Repro seed:** (none — first generated case)
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects mrs x0, sp_el0, x0)
minimal failing input: name = "sp_el0", xt = "x0", extra = Reg("x0")
```

### B4: encode_mrs accepts a 32-bit Wt destination

**Formal:** ∀ dest ∈ {w0..w30, wzr, sp, wsp, d0, s0, q0, v0, h0, b0}, name ∈ NAMED. llvm-mc("mrs "+dest+", "+name)=Err ⇒ encode_mrs([Reg(dest), Symbol(name)])=Err
**Contract evidence:** documented system.rs:55 "MRS Xt, system_reg" (Xt, not Wt/SP/FP); llvm-mc rejects `mrs w0, sp_el0`
**Documentation conflict:** system.rs:55 states the form is MRS Xt; the code discards is_64. The comment states the behavior IS handled as Xt — contract the code violates.
**Severity:** medium
**Counterexample:** encode_mrs([Reg("w0"), Symbol("sp_el0")])  (`mrs w0, sp_el0`)
**Expected / Actual:** Err (llvm-mc: invalid operand) / Ok(Word(0xd5384100))
**Impact:** A width typo is rewritten to the 64-bit Xt encoding; SP and FP dests are accepted the same way.
**Root cause:** system.rs:56 `let (rt, _) = get_reg(operands, 0)?` discards is_64; parse_reg_num accepts w/sp/d/s/q/v/h/b.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:56`
```rust
    let (rt, _) = get_reg(operands, 0)?;
```
**Suggested fix:** Require a 64-bit GPR that is not SP.
```rust
    let (rt, is_64) = get_reg(operands, 0)?;
    if !is_64 {
        return Err("mrs: destination must be Xt".to_string());
    }
```
**Bug report:** bug_reports/encode_mrs_w_dest.md
**Repro seed:** (none — first generated case)
**Raw output:**
```text
Test failed: non-Xt dest must Err (llvm-mc rejects mrs w0, sp_el0), got Ok(Word(3577233664))
minimal failing input: dest = "w0", name = "sp_el0"
```

### B5: encode_mrs masks out-of-range generic sysreg fields

**Formal:** ∀ bad ∈ UnknownName ∪ Empty ∪ MissingSysreg ∪ OobGeneric ∪ OobNumbered. llvm-mc(asm(bad))=Err ⇒ encode_mrs(ops(bad))=Err
**Contract evidence:** inferred (llvm-mc rejects op0>3 / op1>7 / CRn>15 / CRm>15 / op2>7; ARM MRS field widths)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_mrs([Reg("x0"), Symbol("s4_0_c1_c0_1")])  (`mrs x0, s4_0_c1_c0_1`)
**Expected / Actual:** Err (llvm-mc: expected readable system register) / Ok(Word) with op0 masked to 0
**Impact:** An out-of-range S-form silently becomes a different system register.
**Root cause:** system.rs:185 sysreg_encoding masks `op0 & 3` (and the other fields) instead of rejecting; encode_mrs reaches this via parse_generic_sysreg.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/system.rs:185`
```rust
    ((op0 & 3) << 14) | ((op1 & 7) << 11) | ((crn & 0xF) << 7) | ((crm & 0xF) << 3) | (op2 & 7)
```
**Suggested fix:** Range-check fields in parse_generic_sysreg before encoding.
```rust
        if op0 > 3 || op1 > 7 || crn > 15 || crm > 15 || op2 > 7 {
            return Err(format!("unsupported system register: {}", name));
        }
```
**Bug report:** bug_reports/encode_mrs_oob_generic.md
**Repro seed:** cc 5cb784c4cdba23f2241aeebc409e8f1b6554a1162bfd5a6d2e412a3b256c493e
**Raw output:**
```text
Test failed: oob generic sysreg must Err (llvm-mc rejects mrs x0, s4_0_c1_c0_1)
minimal failing input: kind = 3, unknown = "foo", oob_g = "s4_0_c1_c0_1", oob_n = "dbgbcr16_el1", xt = "x0"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_mrs_pbt.rs | 9 properties + 4 KAT + 5 regressions |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mrs -- --test-threads=1
```

B1:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_oslar_el1 -- --test-threads=1 --nocapture
```

B2:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_cntv_cval_el0 -- --test-threads=1 --nocapture
```

B3:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_extra_x0 -- --test-threads=1 --nocapture
```

B4:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_w0_dest -- --test-threads=1 --nocapture
```

B5:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mrs_regression_oob_generic_s4 -- --test-threads=1 --nocapture
```

## Output Directories

- pbt-out/REPORT.md, pbt-out/REPORT.html
- pbt-out/PROPERTIES.md, pbt-out/PLAN.md
- pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/bug_reports/encode_mrs_oslar_el1_write_only.md (+ .html)
- pbt-out/bug_reports/encode_mrs_cntv_cval_el0_encoding.md (+ .html)
- pbt-out/bug_reports/encode_mrs_extra_operand.md (+ .html)
- pbt-out/bug_reports/encode_mrs_w_dest.md (+ .html)
- pbt-out/bug_reports/encode_mrs_oob_generic.md (+ .html)
- pbt-out/run/encode_mrs_kat.log, encode_mrs_full.log, encode_mrs_full2.log, encode_mrs_unknown.log
- src/backend/arm/assembler/encoder/encode_mrs_pbt.rs

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 00:17 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 151/307 total | PBT candidates: 151 | Tested: 151 (100%) | 0 pass, 151 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 151 |
| **Tested (of PBT candidates)** | **151 / 151 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 151 / 0 |
| **Overall (tested / all functions)** | **151 / 307 (49%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 151 | 151 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 151 | 151 | 0 | 100% |

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
