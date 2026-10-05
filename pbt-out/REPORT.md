# PBT Campaign Report: encode_neon_ushr

## Summary

**Verdict:** 4 medium: encode_neon_ushr accepts extra operands, mismatched arrangements, GPR destinations, and shift 0, so invalid USHR assembly is encoded instead of rejected.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_ushr
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed encode_neon_ushr NOT LINKED). Manual arm audit of the function body plus alt-spellings/nonreg sweep.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_ushr | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_ushr ignores a fourth operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. llvm-mc rejects "ushr Vd.T, Vn.T, #shift, v{extra}.T" ⇒ encode_neon_ushr([Vd.T,Vn.T,#shift,Vextra.T]) = Err
**Contract evidence:** documented neon.rs:1181 "ushr requires 3 operands"
**Documentation conflict:** neon.rs:1181 "ushr requires 3 operands" states the instruction requires three operands; the check is `len < 3`, so extras are accepted. The comment is the contract the code violates.
**Severity:** medium
**Counterexample:** encode_neon_ushr([v0.8b, v0.8b, #1, v0.8b])
**Expected / Actual:** Err / Ok(Word) same as `ushr v0.8b, v0.8b, #1`
**Impact:** Invalid four-operand USHR is assembled as three-operand USHR; extra text is silently dropped.
**Root cause:** neon.rs:1180 checks `operands.len() < 3` only, so operands after the first three are ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1180`
```rust
    if operands.len() < 3 {
        return Err("ushr requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("ushr requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_ushr_extra_operand.md
**Repro seed:** rd=0, rn=0, extra=0, t="8b", shift=1
**Raw output:**
```text
Test failed: 4 operands must Err (llvm-mc rejects ushr v0.8b, v0.8b, #1, v0.8b)
minimal failing input: rd = 0, rn = 0, extra = 0, t_shift = ("8b", 1)
```

### B2: encode_neon_ushr ignores a mismatched source arrangement

**Formal:** ∀ rd,rn ∈ {0..31}, Td,Tn ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}, shift ∈ [1, esize(Td) if valid else 1..64]. ¬(valid_ushr_t(Td) ∧ Td=Tn) ⇒ encode_neon_ushr([Vd.Td,Vn.Tn,#shift]) = Err ∧ llvm-mc rejects
**Contract evidence:** documented neon.rs:1189 "USHR Vd.T, Vn.T, #shift"
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ushr([v0.8b, v0.16b, #1])
**Expected / Actual:** Err / Ok(Word) same as `ushr v0.8b, v0.8b, #1`
**Impact:** Mismatched dest/src arrangements are encoded using only dest T; gas/llvm-mc reject the same text.
**Root cause:** neon.rs:1184 discards the source arrangement (`let (rn, _)`), so Vn.T is never compared with Vd.T.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1184`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require matching arrangements
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("ushr arrangement mismatch: {} vs {}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_ushr_mismatched_t.md
**Repro seed:** rd=0, rn=0, td="8b", tn="16b", shift=1
**Raw output:**
```text
Test failed: invalid/mismatched/reserved T must Err (ARM USHR T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects ushr v0.8b, v0.16b, #1)
minimal failing input: rd = 0, rn = 0, td = "8b", tn = "16b", shift = 1
```

### B3: encode_neon_ushr accepts a shift of 0

**Formal:** ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ ℤ \ [1, esize(T)]. llvm-mc rejects "ushr Vd.T, Vn.T, #shift" ⇒ encode_neon_ushr([Vd.T,Vn.T,#shift]) = Err
**Contract evidence:** inferred (ARM Advanced SIMD USHR shift in [1, esize]; llvm-mc "immediate must be an integer in range [1, 8]" for .8b; README.md:12 gas compatibility)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ushr([v0.8b, v0.8b, #0])
**Expected / Actual:** Err / Ok(Word) with immh:immb=0 (reserved). Debug also panics at neon.rs:1192 when `16 - shift` underflows.
**Impact:** Out-of-range shifts are masked into a (sometimes reserved) encoding instead of being rejected; debug builds can abort.
**Root cause:** neon.rs:1192 computes `(16 - shift) & 0xF` with no range check.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1192`
```rust
        "8b" | "16b" => (16 - shift) & 0xF,
```
**Suggested fix:** Reject shift outside [1, esize] before encoding
```rust
        "8b" | "16b" => {
            if !(1..=8).contains(&shift) {
                return Err(format!("ushr shift {} out of range [1, 8]", shift));
            }
            16 - shift
        }
```
**Bug report:** bug_reports/encode_neon_ushr_shift_oob.md
**Repro seed:** rd=0, rn=0, t="8b", shift=0
**Raw output:**
```text
Test failed: shift 0 not in [1, 8] must Err (llvm-mc rejects ushr v0.8b, v0.8b, #0)
minimal failing input: rd = 0, rn = 0, t_shift = ("8b", 0)
```

### B4: encode_neon_ushr accepts a GPR destination

**Formal:** ∀ rd,rn ∈ {0..31}, kind ∈ {gpr_dest, bare_src, x_arr_dest, bare_dest, fp_dest}. llvm-mc rejects the corresponding ushr ⇒ encode_neon_ushr(ops(kind)) = Err
**Contract evidence:** documented neon.rs:1189 "USHR Vd.T, Vn.T, #shift"
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ushr([x0.8b, v0.8b, #1])
**Expected / Actual:** Err / Ok(Word) encoded as `ushr v0.8b, v0.8b, #1`
**Impact:** A GPR with a fake arrangement is treated as Vd; invalid assembly becomes a SIMD USHR.
**Root cause:** neon.rs:1183 calls get_neon_reg, which accepts any parse_reg_num prefix including x/w.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1183`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a V-prefixed register on dest and src
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    if !operands_is_v_reg(operands, 0) {
        return Err("ushr destination must be Vd.T".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_ushr_gpr_dest.md
**Repro seed:** rd=0, rn=0, kind=3, fp_prefix="x"
**Raw output:**
```text
Test failed: GPR/bare/non-arrangement kind=3 must Err (llvm-mc rejects ushr x0.8b, v0.8b, #1)
minimal failing input: rd = 0, rn = 0, kind = 3, fp_prefix = "x"
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs | 10 properties + 1 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_ushr_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_ushr -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ushr_regression_extra_operand -- --test-threads=1 --exact
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ushr_regression_mismatched_t -- --test-threads=1 --exact
```

B3 shift OOB:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ushr_regression_shift_oob -- --test-threads=1 --exact
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ushr_regression_gpr_dest -- --test-threads=1 --exact
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
- pbt-out/bug_reports/encode_neon_ushr_extra_operand.md
- pbt-out/bug_reports/encode_neon_ushr_extra_operand.html
- pbt-out/bug_reports/encode_neon_ushr_mismatched_t.md
- pbt-out/bug_reports/encode_neon_ushr_mismatched_t.html
- pbt-out/bug_reports/encode_neon_ushr_shift_oob.md
- pbt-out/bug_reports/encode_neon_ushr_shift_oob.html
- pbt-out/bug_reports/encode_neon_ushr_gpr_dest.md
- pbt-out/bug_reports/encode_neon_ushr_gpr_dest.html
- pbt-out/run/encode_neon_ushr.log
- pbt-out/run/encode_neon_ushr_round2.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 14:35 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 122/289 total | PBT candidates: 122 | Tested: 122 (100%) | 0 pass, 122 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 122 |
| **Tested (of PBT candidates)** | **122 / 122 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 122 / 0 |
| **Overall (tested / all functions)** | **122 / 289 (42%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 122 | 122 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 122 | 122 | 0 | 100% |

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
| neon.rs | 68 | 37 | 37 | 100% | covered |
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
