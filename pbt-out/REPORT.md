# PBT Campaign Report: encode_neon_scalar_two_misc

## Summary

**Verdict:** 2 medium: encode_neon_scalar_two_misc ignores a third operand and encodes mismatched dest/src SIMD class (including dest SP as Sd), so the GNU-style assembler emits words that gas/llvm-mc refuse.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_scalar_two_misc
**Tests:** 9
**Result:** 7 passing, 2 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus neg_unsupported_dest.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_scalar_two_misc | 9 | 2 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_scalar_two_misc silently encodes a third operand

**Formal:** ∀ rd, rn, extra ∈ {0..31}, pfx ∈ {b,h,s,d}, is_neg ∈ bool. llvm-mc rejects three-operand form ⇒ encode_neon_scalar_two_misc([Rd,Rn,extra], U, 00111) = Err
**Contract evidence:** inferred (README.md:12 GNU-style assembler contract + llvm-mc rejection of a third operand; encode() at encoder/mod.rs:674-682 passes extra operands through)
**Documentation conflict:** neon.rs:1820 "scalar two-misc requires 2 operands" — min-arity domain restriction, not a declaration that extra operands are valid. (none as an exclusion of the extra-operand input)
**Severity:** medium
**Counterexample:** encode_neon_scalar_two_misc([Reg("b0"), Reg("b0"), Reg("b0")], 0, 0b00111)
**Expected / Actual:** Err / Ok(Word(0x5e207800)) encoding `sqabs b0, b0`
**Impact:** Invalid assembly such as `sqabs b0, b0, b0` is assembled into a scalar SQABS word instead of an error. encode() routes any non-RegArrangement dest into this helper with no arity maximum, so the witness is caller-reachable.
**Root cause:** neon.rs:1820 checks only `operands.len() < 2`, so arity 3+ is treated as a 2-operand encode using the first two operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1820`
```rust
    if operands.len() < 2 { return Err("scalar two-misc requires 2 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 2
```rust
    if operands.len() != 2 { return Err("scalar two-misc requires 2 operands".to_string()); }
```
**Bug report:** bug_reports/encode_neon_scalar_two_misc_extra_operand.md
**Repro seed:** rd = 0, rn = 0, extra = 0, pfx = "b", is_neg = false
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_two_misc_pbt::encode_neon_scalar_two_misc_neg_extra_operand' (2502835) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:199:1:
Test failed: 3 operands must Err (llvm-mc rejects sqabs b0, b0, b0) at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:347.
minimal failing input: rd = 0, rn = 0, extra = 0, pfx = "b", is_neg = false
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_scalar_two_misc encodes mismatched dest/src SIMD class

**Formal:** ∀ rd, rn ∈ {0..31}, dest_pfx ∈ {b,h,s,d}, bad_pfx ∉ {dest_pfx} ∪ {valid matching}, is_neg ∈ bool, slot ∈ {dest, src}. llvm-mc rejects the mismatched-class form ⇒ encode_neon_scalar_two_misc = Err
**Contract evidence:** documented neon.rs:1818 "NEON scalar two-reg misc: SQABS/SQNEG Hd,Hn / Sd,Sn / Dd,Dn" (matching pairs) plus ARM/llvm-mc same-class requirement
**Documentation conflict:** neon.rs:1818 "NEON scalar two-reg misc: SQABS/SQNEG Hd,Hn / Sd,Sn / Dd,Dn" — states matching H/S/D pairs ARE the form (contract the code violates: source class is never checked; dest `sp` matches starts_with('s')). Mark (not independently verified) only as the body contradicts the comment.
**Severity:** medium
**Counterexample:** encode_neon_scalar_two_misc([Reg("h0"), Reg("b0")], 0, 0b00111)
**Expected / Actual:** Err / Ok(Word(0x5e607800)) encoding as if `sqabs h0, h0`
**Impact:** Invalid assembly such as `sqabs h0, b0` is assembled as SQABS H0, H0. Dest `sp` is packed as Sd (rd=31). encode() does not inspect register class beyond dest not being RegArrangement.
**Root cause:** neon.rs:1822 extracts Rn via parse_reg_num with no class check; neon.rs:1823-1827 derives size from dest starts_with('b'|'h'|'s'|'d'), so `sp` is treated as Sd and a B source with an H dest is packed with size=H
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1822`
```rust
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()) };
    let size = if rd_name.starts_with('b') { 0b00u32 }
        else if rd_name.starts_with('h') { 0b01 }
        else if rd_name.starts_with('s') { 0b10 }
        else if rd_name.starts_with('d') { 0b11 }
        else { return Err(format!("scalar two-misc: unsupported register type: {}", rd_name)); };
```
**Suggested fix:** Require matching B/H/S/D prefixes on dest and source; reject aliases such as sp
```rust
    let rn_name = match &operands[1] {
        Operand::Reg(r) => r.to_lowercase(),
        _ => return Err("expected register".to_string()),
    };
    let size = match rd_name.as_str() {
        n if n.starts_with('b') && n[1..].parse::<u32>().ok().map_or(false, |v| v <= 31) => 0b00u32,
        n if n.starts_with('h') && n[1..].parse::<u32>().ok().map_or(false, |v| v <= 31) => 0b01,
        n if n.starts_with('s') && n[1..].parse::<u32>().ok().map_or(false, |v| v <= 31) => 0b10,
        n if n.starts_with('d') && n[1..].parse::<u32>().ok().map_or(false, |v| v <= 31) => 0b11,
        _ => return Err(format!("scalar two-misc: unsupported register type: {}", rd_name)),
    };
    if !rn_name.starts_with(&rd_name[..1]) {
        return Err(format!("scalar two-misc: dest/src class mismatch: {} vs {}", rd_name, rn_name));
    }
```
**Bug report:** bug_reports/encode_neon_scalar_two_misc_wrong_reg_class.md
**Repro seed:** rd = 0, rn = 0, dest_pfx = "b", bad = "h", is_neg = false, slot = 0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_scalar_two_misc_pbt::encode_neon_scalar_two_misc_neg_wrong_reg_class' (2503869) panicked at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:199:1:
Test failed: wrong class slot=0 bad=h must Err (llvm-mc rejects sqabs h0, b0) at src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs:379.
minimal failing input: rd = 0, rn = 0, dest_pfx = "b", bad = "h", is_neg = false, slot = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_scalar_two_misc_pbt.rs | 9 properties + 1 KAT + 3 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_scalar_two_misc_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_two_misc -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_two_misc_neg_extra_operand -- --test-threads=1
```

B2 wrong register class:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_two_misc_neg_wrong_reg_class -- --test-threads=1
```

## Output Directories

pbt-out/PLAN.md, pbt-out/PROPERTIES.md, pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/report.json, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/INVARIANTS.md, pbt-out/CHANGE_SURFACE.md, pbt-out/run/encode_neon_scalar_two_misc_test.log, pbt-out/run/encode_neon_scalar_two_misc_test2.log, pbt-out/run/sweep_unsupported_dest.log, pbt-out/bug_reports/encode_neon_scalar_two_misc_extra_operand.md, pbt-out/bug_reports/encode_neon_scalar_two_misc_extra_operand.html, pbt-out/bug_reports/encode_neon_scalar_two_misc_wrong_reg_class.md, pbt-out/bug_reports/encode_neon_scalar_two_misc_wrong_reg_class.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 22:28 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 146/289 total | PBT candidates: 146 | Tested: 146 (100%) | 0 pass, 146 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 146 |
| **Tested (of PBT candidates)** | **146 / 146 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 146 / 0 |
| **Overall (tested / all functions)** | **146 / 289 (51%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 146 | 146 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 146 | 146 | 0 | 100% |

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
| neon.rs | 68 | 61 | 61 | 100% | covered |
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
