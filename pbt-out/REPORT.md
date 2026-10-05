# PBT Campaign Report: encode_neon_scalar_qshrn

## Summary

**Verdict:** 1 high: encode_neon_scalar_qshrn packs every valid scalar SQSHRN as the vector SQSHRN2 encoding (bit 28 clear: 0x4f0f9400 vs llvm-mc 0x5f0f9400), so assembled objects disagree with gas and execute the wrong instruction class; plus 2 medium: extra operand ignored, dest/src class pairing not checked.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_scalar_qshrn
**Tests:** 10
**Result:** 5 passing, 5 failing properties, 3 unique defects (encoding counted once)
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit plus sweep of documented dest x/w/q/v/d and Imm/Mem/Label paths.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_scalar_qshrn | 10 | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_scalar_qshrn packs scalar SQSHRN as vector SQSHRN2

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ (vd,vn,esize) ∈ {(b,h,8),(h,s,16),(s,d,32)}, ∀ shift ∈ {1..esize}, ∀ u ∈ {0,1}, ∀ round ∈ {false,true}. encode_neon_scalar_qshrn([Reg(vd||rd), Reg(vn||rn), Imm(shift)], u, round) = Word(llvm-mc(mnem(u,round) || " " || vd||rd || ", " || vn||rn || ", #" || shift))
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; ARM asisdshf 01 U 11111; llvm-mc 15.0.6 `sqshrn b0, h0, #1` = 0x5f0f9400)
**Documentation conflict:** neon.rs:1848 "01 U 11110 immh:immb opcode 1 Rn Rd" restates the producing packing (vector-style 11110) rather than declaring scalar input invalid or admitting a limitation on accepted input. ARM/llvm-mc scalar encodings use bits[31:24]=01 U 11111.
**Severity:** high
**Counterexample:** encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1)], u_bit=0, is_rounding=false)
**Expected / Actual:** 0x5f0f9400 / 0x4f0f9400
**Impact:** Every successful scalar SQSHRN/SQRSHRN/UQSHRN/UQRSHRN is emitted as the vector Q=1 encoding. Assembled objects disagree with gas/llvm-mc and execute the wrong instruction class. encode() routes non-RegArrangement `sqshrn` dest into this helper.
**Root cause:** neon.rs:1849 ORs vector asimdshf fixed bits `(0b011110 << 23)` onto `(0b01 << 30)`, leaving bit 28 = 0; ARM scalar asisdshf needs bits[28:24]=11111
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1849`
```rust
    let word = (0b01 << 30) | (u_bit << 29) | (0b011110 << 23) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
```
**Suggested fix:** Place scalar asisdshf fixed bits at [28:24]=11111
```rust
    let word = (0b01 << 30) | (u_bit << 29) | (0b11111 << 24) | ((immhb >> 3) << 19) | ((immhb & 7) << 16)
        | (opcode_bits << 10) | (rn << 5) | rd;
```
**Bug report:** bug_reports/encode_neon_scalar_qshrn_asisdshf_bit28.md
**Repro seed:** cc e6eeee35cb96efd2430f8574b661459cebb5b0fdd1cf7e3a68110087c18f5194
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `1326420992`,
 right: `1594856448`: mismatch for sqshrn b0, h0, #1
minimal failing input: rd = 0, rn = 0, case = ("b", "h", 8, 1), u = 0, round = false
```

### B2: encode_neon_scalar_qshrn silently encodes a fourth operand

**Formal:** ∀ valid 3-operand scalar-qshrn ops, ∀ extra Operand::Reg. llvm-mc rejects asm with a fourth operand ⇒ encode_neon_scalar_qshrn(ops||[extra], u, round) is Err
**Contract evidence:** documented neon.rs:1836 "scalar qshrn requires 3 operands"
**Documentation conflict:** neon.rs:1836 "scalar qshrn requires 3 operands" states the instruction needs 3 operands; the check is only `len < 3`, so arity 4+ is accepted. The comment/error does not declare extra operands valid.
**Severity:** medium
**Counterexample:** encode_neon_scalar_qshrn([Reg("b0"), Reg("h0"), Imm(1), Reg("b0")], 0, false)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Invalid assembly such as `sqshrn b0, h0, #1, b0` is assembled into a word that gas/llvm-mc refuse.
**Root cause:** neon.rs:1836 checks only `operands.len() < 3`, so arity 4+ is treated as a 3-operand encode using the first three operands
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1836`
```rust
    if operands.len() < 3 { return Err("scalar qshrn requires 3 operands".to_string()); }
```
**Suggested fix:** Reject any arity other than 3
```rust
    if operands.len() != 3 { return Err("scalar qshrn requires 3 operands".to_string()); }
```
**Bug report:** bug_reports/encode_neon_scalar_qshrn_extra_operand.md
**Repro seed:** (deterministic; shrunk to rd=0, rn=0, extra=0, b<-h #1)
**Raw output:**
```text
Test failed: 4 operands must Err (llvm-mc rejects sqshrn b0, h0, #1, b0)
minimal failing input: rd = 0, rn = 0, extra = 0, case = ("b", "h", 8, 1), u = 0, round = false
```

### B3: encode_neon_scalar_qshrn ignores dest/src scalar class pairing

**Formal:** ∀ dest ∈ {b,h,s}, ∀ src_pfx such that (dest,src) is not a mandated pair, ∀ rd,rn,shift,u,round. llvm-mc rejects mnem dest||rd, src||rn, #shift ⇒ encode_neon_scalar_qshrn is Err
**Contract evidence:** inferred (ARM/llvm-mc mandated B<-H / H<-S / S<-D; neon.rs:1834 names `sqshrn Hd,Sn,#shift / sqshrn Sd,Dn,#shift`)
**Documentation conflict:** neon.rs:1834 "NEON scalar SQSHRN: sqshrn Hd,Sn,#shift / sqshrn Sd,Dn,#shift" names the valid H<-S and S<-D pairs (body also accepts B<-H). It does not declare mismatched pairs valid. llvm-mc rejects `sqshrn b0, s0, #8`.
**Severity:** medium
**Counterexample:** encode_neon_scalar_qshrn([Reg("b0"), Reg("s0"), Imm(8)], 0, false)
**Expected / Actual:** Err / Ok(Word)
**Impact:** Invalid assembly such as `sqshrn b0, s0, #8` is assembled instead of an error.
**Root cause:** neon.rs:1838 accepts any Operand::Reg for the source; only dest prefix selects element size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1838`
```rust
    let rn = match &operands[1] { Operand::Reg(r) => parse_reg_num(r).ok_or("invalid reg")?, _ => return Err("expected register".to_string()); };
```
**Suggested fix:** Require the source prefix to match the dest narrowing pair (B<-H, H<-S, S<-D)
```rust
    let (rn, rn_name) = match &operands[1] { Operand::Reg(r) => (parse_reg_num(r).ok_or("invalid reg")?, r.to_lowercase()), _ => return Err("expected register".to_string()) };
    let want_src = match rd_name.chars().next() {
        Some('b') => 'h',
        Some('h') => 's',
        Some('s') => 'd',
        _ => return Err(format!("scalar qshrn: unsupported dest: {}", rd_name)),
    };
    if !rn_name.starts_with(want_src) {
        return Err(format!("scalar qshrn: source {} incompatible with dest {}", rn_name, rd_name));
    }
```
**Bug report:** bug_reports/encode_neon_scalar_qshrn_wrong_reg_class.md
**Repro seed:** (deterministic; shrunk to rd=0, rn=0, vd=b, src=s, #8)
**Raw output:**
```text
Test failed: wrong class src=s must Err (llvm-mc rejects sqshrn b0, s0, #8)
minimal failing input: rd = 0, rn = 0, vd = "b", src = "s", u = 0, round = false
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_scalar_qshrn_pbt.rs | 10 properties + 1 KAT + 3 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `mod encode_neon_scalar_qshrn_pbt` registration |

## Reproduction

```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_scalar_qshrn -- --test-threads=1
cargo test --lib encode_neon_scalar_qshrn_diff_llvm_mc -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_asisdshf_bit28 -- --test-threads=1
cargo test --lib encode_neon_scalar_qshrn_neg_extra_operand -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_extra_operand -- --test-threads=1
cargo test --lib encode_neon_scalar_qshrn_neg_wrong_reg_class -- --test-threads=1
cargo test --lib test_encode_neon_scalar_qshrn_regression_wrong_reg_class -- --test-threads=1
```

## Output Directories

pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/PROPERTIES.md, pbt-out/PLAN.md, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/bug_reports/encode_neon_scalar_qshrn_asisdshf_bit28.md, pbt-out/bug_reports/encode_neon_scalar_qshrn_asisdshf_bit28.html, pbt-out/bug_reports/encode_neon_scalar_qshrn_asisdshf_fields.md, pbt-out/bug_reports/encode_neon_scalar_qshrn_asisdshf_fields.html, pbt-out/bug_reports/encode_neon_scalar_qshrn_asisdshf_alt_spellings.md, pbt-out/bug_reports/encode_neon_scalar_qshrn_asisdshf_alt_spellings.html, pbt-out/bug_reports/encode_neon_scalar_qshrn_extra_operand.md, pbt-out/bug_reports/encode_neon_scalar_qshrn_extra_operand.html, pbt-out/bug_reports/encode_neon_scalar_qshrn_wrong_reg_class.md, pbt-out/bug_reports/encode_neon_scalar_qshrn_wrong_reg_class.html, pbt-out/run/encode_neon_scalar_qshrn.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 23:02 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 147/289 total | PBT candidates: 147 | Tested: 147 (100%) | 0 pass, 147 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 147 |
| **Tested (of PBT candidates)** | **147 / 147 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 147 / 0 |
| **Overall (tested / all functions)** | **147 / 289 (51%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 147 | 147 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 147 | 147 | 0 | 100% |

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
| neon.rs | 68 | 62 | 62 | 100% | covered |
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
