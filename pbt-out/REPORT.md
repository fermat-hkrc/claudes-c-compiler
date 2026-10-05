# PBT Campaign Report: encode_neon_umov

## Summary

**Verdict:** 4 medium: encode_neon_umov silently encodes extra operands, out-of-range lanes, wrong dest width, and SP/WSP/FP dest names that llvm-mc rejects, so a GNU-style assembler typo becomes a different valid (or reserved) instruction instead of an error.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_umov
**Tests:** 10 properties (+ 1 KAT + 6 regression witnesses)
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_umov), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit added unsupported-elem-size and non-lane-src (both passing).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_umov | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_umov ignores a surplus third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. llvm-mc("umov gpr, Vn.Ts[i], extra") = Err ⇒ encode_neon_umov([Reg, RegLane, extra]) = Err
**Contract evidence:** documented neon.rs:463 "umov requires 2 operands"
**Documentation conflict:** neon.rs:463 "umov requires 2 operands" states the arity IS 2; the code implements `len() < 2` (at least 2). The comment is the contract the code violates.
**Severity:** medium
**Counterexample:** encode_neon_umov([w0, v0.b[0], w0])
**Expected / Actual:** Err / Ok(Word(0x0e013c00))
**Impact:** A typo or extra token silently produces a valid 2-operand UMOV instead of an assembler error.
**Root cause:** neon.rs:462 `if operands.len() < 2` only rejects too few operands; extras past index 1 are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:462`
```rust
    if operands.len() < 2 {
        return Err("umov requires 2 operands".to_string());
    }
```
**Suggested fix:** Require exactly two operands.
```rust
    if operands.len() != 2 {
        return Err("umov requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_umov_extra_operand.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:569:9:
umov w0, v0.b[0], w0 must Err (llvm-mc rejects a third operand)
```

### B2: encode_neon_umov masks an out-of-range lane index instead of rejecting it

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ over ∈ {1..8}. let i = imax(ts)+over. llvm-mc("umov gpr, Vn.Ts[i]") = Err ⇒ encode_neon_umov(...) = Err
**Contract evidence:** inferred (ARM Advanced SIMD copy lane ranges b[0-15] h[0-7] s[0-3] d[0-1]; llvm-mc/gas reject OOR; README gas-compatibility)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_umov([w0, v0.b[16]])
**Expected / Actual:** Err / Ok(Word(0x0e013c00)) encoded as v0.b[0]
**Impact:** An out-of-range lane silently wraps and produces a different valid instruction.
**Root cause:** neon.rs:474-477 mask the index (`index & 0xF` etc.) instead of range-checking against imax(Ts).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:474`
```rust
                "b" => ((*index & 0xF) << 1) | 0b00001,
```
**Suggested fix:** Reject an index above the ARM maximum for that element size before encoding imm5.
```rust
            if *index > max_for(elem_size) {
                return Err(format!("umov lane index {} out of range for .{}", index, elem_size));
            }
```
**Bug report:** bug_reports/encode_neon_umov_index_oor.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_index_oor' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:578:9:
umov w0, v0.b[16] must Err (llvm-mc range for .b is [0, 15])
```

### B3: encode_neon_umov accepts the wrong GPR width for UMOV

**Formal:** ∀ rd ∈ {0..30}, ∀ rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)]. llvm-mc("umov {wrong-width gpr}, Vn.Ts[i]") = Err ⇒ encode_neon_umov([Reg(wrong), RegLane]) = Err
**Contract evidence:** inferred (ARM UMOV Wd+B/H/S and Xd+D; llvm-mc rejects the crossed pairing; README gas-compatibility)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_umov([x0, v0.b[0]])
**Expected / Actual:** Err / Ok(Word(0x4e013c00)) with Q=1
**Impact:** A width typo silently produces a reserved/invalid UMOV encoding llvm-mc and gas refuse to assemble.
**Root cause:** neon.rs:471 sets Q from dest GPR width (`is_64`) and never checks that Wd pairs with B/H/S and Xd pairs with D.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:471`
```rust
            let q = if is_64 { 1u32 } else { 0 };
```
**Suggested fix:** Derive Q from the element size and reject a dest width that does not match.
```rust
            let q = match elem_size.as_str() {
                "b" | "h" | "s" => {
                    if is_64 { return Err("umov B/H/S requires a W register".into()); }
                    0u32
                }
                "d" => {
                    if !is_64 { return Err("umov D requires an X register".into()); }
                    1u32
                }
                _ => return Err(format!("unsupported umov element size: {}", elem_size)),
            };
```
**Bug report:** bug_reports/encode_neon_umov_wrong_width.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_wrong_width' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:600:9:
umov x0, v0.b[0] must Err (llvm-mc requires Wd for Ts=B)
```

### B4: encode_neon_umov encodes SP/WSP and FP names as the UMOV GPR dest

**Formal:** ∀ n ∈ {0,1}, ∀ rd,rn ∈ {0..31}, ∀ ts ∈ {b,h,s,d}, ∀ i ∈ [0, imax(ts)], ∀ kind ∈ {arity, sp, wsp, fp, bad-name}. llvm-mc rejects the corresponding assembly ⇒ encode_neon_umov returns Err
**Contract evidence:** inferred (ARM UMOV dest is Wd/Xd/WZR/XZR; llvm-mc rejects SP/WSP/FP; README gas-compatibility)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_umov([sp, v0.b[0]])
**Expected / Actual:** Err / Ok(Word(0x4e013c1f)) for SP; also WSP → 0x0e013c1f, d0 → 0x0e013c00
**Impact:** A mistyped dest silently becomes XZR/WZR/W0.
**Root cause:** neon.rs:465 calls get_reg, which uses parse_reg_num (sp/wsp → 31; d/s/q/v/h/b prefixes accepted). encode_neon_umov never restricts dest to W/X/WZR/XZR.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:465`
```rust
    let (rd, is_64) = get_reg(operands, 0)?;
```
**Suggested fix:** After get_reg, reject SP/WSP and FP/SIMD dest names.
```rust
    let dest = match operands.get(0) {
        Some(Operand::Reg(name)) => name,
        _ => return Err("umov: expected GPR dest".into()),
    };
    let lower = dest.to_lowercase();
    if lower == "sp" || lower == "wsp" || matches!(lower.chars().next(), Some('d' | 's' | 'q' | 'v' | 'h' | 'b')) {
        return Err(format!("umov dest must be a W/X register, got {}", dest));
    }
```
**Bug report:** bug_reports/encode_neon_umov_sp_fp_dest.md
**Repro seed:** (none — deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_umov_pbt::test_encode_neon_umov_regression_sp_as_zr' panicked at src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs:615:9:
umov sp, v0.b[0] must Err (llvm-mc rejects SP as UMOV dest)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs | 10 properties + 1 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_umov_pbt` |

## Reproduction

Whole suite (serial, same as the campaign run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_umov -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_extra_operand -- --test-threads=1
```

B2 index OOR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_index_oor -- --test-threads=1
```

B3 wrong width:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_wrong_width -- --test-threads=1
```

B4 SP dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_umov_regression_sp_as_zr -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — per-function coverage table
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/report.json — machine-readable report
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/FUNCTION_INDEX.md — merged function index
- pbt-out/bug_reports/encode_neon_umov_extra_operand.md + .html
- pbt-out/bug_reports/encode_neon_umov_index_oor.md + .html
- pbt-out/bug_reports/encode_neon_umov_wrong_width.md + .html
- pbt-out/bug_reports/encode_neon_umov_sp_fp_dest.md + .html
- pbt-out/run/encode_neon_umov_test.log — first full test run
- src/backend/arm/assembler/encoder/encode_neon_umov_pbt.rs — harness

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 10:10 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 108/289 total | PBT candidates: 108 | Tested: 108 (100%) | 0 pass, 108 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 108 |
| **Tested (of PBT candidates)** | **108 / 108 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 108 / 0 |
| **Overall (tested / all functions)** | **108 / 289 (37%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 108 | 108 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 108 | 108 | 0 | 100% |

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
| neon.rs | 68 | 23 | 23 | 100% | covered |
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
