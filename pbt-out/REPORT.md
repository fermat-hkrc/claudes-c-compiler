# PBT Campaign Report: encode_fnmadd_fnmsub

## Summary

**Verdict:** 1 high, 2 medium: encode_fnmadd_fnmsub encodes H-register FNMADD/FNMSUB as single-precision (ftype=00 instead of 11) and silently accepts a fifth operand plus mixed S/D, GPR, Q/V/B, and SP, so invalid GNU-style assembly becomes a wrong machine-code word.
**Date:** 2026-10-06
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_fnmadd_fnmsub
**Tests:** 9
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). The cargo test binary executed the symbol (7 KATs + 9 properties). Do not treat NOT LINKED as untested.
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_fnmadd_fnmsub | 9 | 3 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### B1: encode_fnmadd_fnmsub ignores extra operands

**Formal:** ∀ rd,rn,rm,ra ∈ {0..31}, is_d ∈ {S,D}, is_sub ∈ {false,true}, extra ∈ ExtraOperand. encode_fnmadd_fnmsub([Rd,Rn,Rm,Ra,extra], is_sub) is Err
**Contract evidence:** inferred (signature names four registers Rd, Rn, Rm, Ra; llvm-mc/gas reject a fifth operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fnmadd_fnmsub([Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0")], false)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Invalid GNU-style `fnmadd s0, s0, s0, s0, s0` is assembled instead of rejected, so a typo extra operand becomes a silent 32-bit encoding.
**Root cause:** fp_scalar.rs:147-150 four get_reg calls with no operands.len() == 4 check, so a fifth operand is never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:147`
```rust
    let (rd, _) = get_reg(operands, 0)?;
    let (rn, _) = get_reg(operands, 1)?;
    let (rm, _) = get_reg(operands, 2)?;
    let (ra, _) = get_reg(operands, 3)?;
```
**Suggested fix:** Reject extra operands before encoding.
```rust
    if operands.len() != 4 {
        return Err("fnmadd/fnmsub requires 4 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_fnmadd_fnmsub_extra_operand.md
**Repro seed:** (none — deterministic)
**Raw output:** Test failed: FNMADD/FNMSUB has no 5th operand; extra must Err. minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, is_d = false, is_sub = false, extra = Reg("s0")

### B2: encode_fnmadd_fnmsub accepts mixed S/D, GPR, Q/V/B, and SP operands

**Formal:** ∀ (dest,n,m,a) ∈ WrongTypeQuad, is_sub ∈ {false,true}. encode_fnmadd_fnmsub([dest,n,m,a], is_sub) is Err
**Contract evidence:** inferred (ARM 3-source FNMADD requires matching Sd/Dd/Hd; llvm-mc rejects mixed S/D, GPR, Q/V/B, SP)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_fnmadd_fnmsub([Reg("d0"), Reg("s0"), Reg("s0"), Reg("s0")], false)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Mixed-class or GPR/SP operands assemble as scalar fused multiply-add, so `fnmadd d0, s0, s0, s0` and `fnmadd x0, s1, s2, s3` become wrong-class encodings.
**Root cause:** fp_scalar.rs:151-153 get_reg/parse_reg_num accept any prefix; ftype is taken only from dest `starts_with('d')`, with no matching-class check on Rn/Rm/Ra.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:151`
```rust
    let rd_name = match &operands[0] { Operand::Reg(r) => r.to_lowercase(), _ => String::new() };
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01u32 } else { 0b00 };
```
**Suggested fix:** Require all four operands to be the same FP class (S, D, or H) and reject GPR/SP/Q/V/B.
```rust
    // reject unless all four names share prefix s, d, or h
```
**Bug report:** bug_reports/encode_fnmadd_fnmsub_wrong_types.md
**Repro seed:** (none — deterministic)
**Raw output:** Test failed: FNMADD/FNMSUB requires matching Sd/Dd/Hd quadruples; dest=d0 n=s0 m=s0 a=s0 must Err. minimal failing input: (dest, src_n, src_m, src_a) = ("d0", "s0", "s0", "s0"), is_sub = false

### B3: encode_fnmadd_fnmsub encodes H registers as ftype=00 (single) instead of ftype=11 (half)

**Formal:** ∀ rd,rn,rm,ra ∈ {0..31}, is_sub ∈ {false,true}. encode_fnmadd_fnmsub([h_rd,h_rn,h_rm,h_ra], is_sub) = llvm-mc -mattr=+fullfp16 ("fnmadd|fnmsub Hd, Hn, Hm, Ha")
**Contract evidence:** inferred (ARM ARM ftype=11 half; format comment names ftype; llvm-mc +fullfp16)
**Documentation conflict:** (none) — fp_scalar.rs:144 names ftype but does not declare H invalid or claim it is handled
**Severity:** high
**Counterexample:** encode_fnmadd_fnmsub([Reg("h0"), Reg("h0"), Reg("h0"), Reg("h0")], false)
**Expected / Actual:** 0x1fe00000 / 0x1f200000
**Impact:** Half-precision `fnmadd h0, h0, h0, h0` is emitted as the single-precision encoding, so the object file executes the wrong FP operation.
**Root cause:** fp_scalar.rs:152-153 sets ftype from `rd_name.starts_with('d')` only, so H (and every non-D prefix) collapses to ftype=00.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/fp_scalar.rs:152`
```rust
    let is_double = rd_name.starts_with('d');
    let ftype = if is_double { 0b01u32 } else { 0b00 };
```
**Suggested fix:** Map H to ftype=11.
```rust
    let ftype = if rd_name.starts_with('d') {
        0b01u32
    } else if rd_name.starts_with('h') {
        0b11u32
    } else {
        0b00
    };
```
**Bug report:** bug_reports/encode_fnmadd_fnmsub_half_ftype.md
**Repro seed:** (none — deterministic)
**Raw output:** Test failed: assertion failed: `(left == right)` left: `522190848`, right: `534773760`: FNMADD/FNMSUB half-precision mismatch for fnmadd h0, h0, h0, h0. minimal failing input: rd = 0, rn = 0, rm = 0, ra = 0, is_sub = false

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs | 9 properties + 7 KAT + 5 regression witnesses |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_fnmadd_fnmsub -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fnmadd_fnmsub_regression_extra_operand -- --test-threads=1
```

B2 wrong types:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fnmadd_fnmsub_regression_mixed_sd -- --test-threads=1
```

B3 half ftype:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_fnmadd_fnmsub_regression_half_ftype -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_fnmadd_fnmsub_extra_operand.md
- pbt-out/bug_reports/encode_fnmadd_fnmsub_extra_operand.html
- pbt-out/bug_reports/encode_fnmadd_fnmsub_wrong_types.md
- pbt-out/bug_reports/encode_fnmadd_fnmsub_wrong_types.html
- pbt-out/bug_reports/encode_fnmadd_fnmsub_half_ftype.md
- pbt-out/bug_reports/encode_fnmadd_fnmsub_half_ftype.html
- pbt-out/run/encode_fnmadd_fnmsub.log
- pbt-out/run/encode_fnmadd_fnmsub_invalid_name.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-06 05:43 (campaign: coverage)
> Files: 11/11 scanned (100%) | Functions: 166/307 total | PBT candidates: 166 | Tested: 166 (100%) | 1 pass, 166 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 166 |
| **Tested (of PBT candidates)** | **166 / 166 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 1 / 166 / -1 |
| **Overall (tested / all functions)** | **166 / 307 (54%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 166 | 166 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 166 | 166 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 18 | 18 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 25 | 25 | 100% | covered |
| fp_scalar.rs | 13 | 11 | 12 | 109% | covered |
| gp_integer.rs | 29 | 1 | 1 | 100% | covered |
| load_store.rs | 20 | 14 | 14 | 100% | covered |
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
