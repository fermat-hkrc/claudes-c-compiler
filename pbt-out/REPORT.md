# PBT Campaign Report: encode_neon_sri

## Summary

**Verdict:** 4 medium: encode_neon_sri silently encodes a fourth operand, mismatched dest/src arrangements, out-of-range `#0` shifts (reserved immh=0000; debug overflow on negatives), and a bare `Vn` source — all rejected by llvm-mc/gas.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_sri
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function (encode_neon_sri), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test; C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit of the encode_neon_sri body plus 1000-case properties covering arity / get_neon_reg / get_imm / neon_arr_to_q_size / match T / Ok Word.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_sri | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_sri ignores a fourth operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_sri([Vd.T, Vn.T, #shift, Vextra.T]) = Err ∧ llvm-mc rejects the matching 4-operand asm
**Contract evidence:** inferred (signature / neon.rs:1284 names exactly three operands SRI Vd.T, Vn.T, #shift; README.md:12 gas-compatible; llvm-mc rejects a fourth operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_sri([v0.8b, v0.8b, #1, v0.8b])
**Expected / Actual:** Err / Ok(Word) same as `sri v0.8b, v0.8b, #1`
**Impact:** The assembler silently encodes `sri Vd.T, Vn.T, #shift, extra` as three-operand SRI, dropping the extra operand instead of diagnosing invalid assembly
**Root cause:** neon.rs:1286 checks `operands.len() < 3` only, so extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1286`
```rust
    if operands.len() < 3 {
        return Err("sri requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("sri requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_sri_extra_operand.md
**Repro seed:** cc 5f625bb1c70efd9d4ab4aa106a98f0e7cf5bdcc0f6c4e5bd37ae3685b02c5204
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_extra_operand' (2357942) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: 4 operands must Err (llvm-mc rejects sri v0.8b, v0.8b, #1, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:370.
minimal failing input: rd = 0, rn = 0, extra = 0, t_shift = (
    "8b",
    1,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_sri encodes mismatched dest/src arrangements

**Formal:** ∀ rd,rn ∈ {0..31}, Td,Tn arrangements, shift ∈ [1,64]. (Td ≠ Tn ∨ Td ∉ {8b,16b,4h,8h,2s,4s,2d}) ⇒ encode_neon_sri([Vd.Td, Vn.Tn, #shift]) = Err ∧ llvm-mc rejects
**Contract evidence:** inferred (neon.rs:1284 SRI Vd.T, Vn.T matching T; ARM SRI T in {8B,16B,4H,8H,2S,4S,2D}; llvm-mc rejects sri v0.2s, v0.8b, #1)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_sri([v0.2s, v0.8b, #1])
**Expected / Actual:** Err / Ok(Word) encoded from dest arrangement `.2s` only
**Impact:** Invalid assembly `sri v0.2s, v0.8b, #1` is encoded as if both were `.2s`
**Root cause:** neon.rs:1290 discards the source arrangement (`let (rn, _) = get_neon_reg(operands, 1)?`), so Tn is never compared to Td
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1290`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the source arrangement to match the dest arrangement
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("sri arrangement mismatch: .{} vs .{}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_sri_mismatched_t.md
**Repro seed:** (none — first-case shrink)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_invalid_t' (2357977) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: invalid/mismatched/reserved T must Err (ARM SRI T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects sri v0.2s, v0.8b, #1) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:396.
minimal failing input: rd = 0, rn = 0, td = "2s", tn = "8b", shift = 1
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_sri encodes out-of-range shift immediates

**Formal:** ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∉ [1, esize(T)]. encode_neon_sri([Vd.T, Vn.T, #shift]) = Err
**Contract evidence:** inferred (ARM SRI shift in [1, esize]; llvm-mc "immediate must be an integer in range [1, 8]" for .8b; README.md:12 gas-compatible)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_sri([v0.8b, v0.8b, #0])
**Expected / Actual:** Err / Ok(Word) with reserved immh=0000
**Impact:** `sri v0.8b, v0.8b, #0` is assembled instead of diagnosed. Negative shifts overflow-panic in debug at `16 - shift`.
**Root cause:** neon.rs:1296-1301 computes `(2*esize - shift) & mask` with no range check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1297`
```rust
        "8b" | "16b" => (16 - shift) & 0xF,
        "4h" | "8h" => (32 - shift) & 0x1F,
        "2s" | "4s" => (64 - shift) & 0x3F,
        "2d" => (128 - shift) & 0x7F,
```
**Suggested fix:** Reject shift outside [1, esize] before encoding
```rust
        "8b" | "16b" if (1..=8).contains(&shift) => (16 - shift) & 0xF,
        "4h" | "8h" if (1..=16).contains(&shift) => (32 - shift) & 0x1F,
        "2s" | "4s" if (1..=32).contains(&shift) => (64 - shift) & 0x3F,
        "2d" if (1..=64).contains(&shift) => (128 - shift) & 0x7F,
        "8b" | "16b" | "4h" | "8h" | "2s" | "4s" | "2d" => {
            return Err(format!("sri shift {} out of range for {}", shift, arr_d));
        }
```
**Bug report:** bug_reports/encode_neon_sri_shift_oob.md
**Repro seed:** (none — first-case shrink)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_shift_oob' (2357992) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: shift 0 not in [1, 8] must Err (llvm-mc rejects sri v0.8b, v0.8b, #0) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:420.
minimal failing input: rd = 0, rn = 0, t_shift = (
    "8b",
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B4: encode_neon_sri accepts a bare V source without arrangement

**Formal:** ∀ rd,rn ∈ {0..31}, kind ∈ {Reg dest, bare V src, bare V dest, xN.8b dest, xN src}. encode_neon_sri(kind) = Err ∧ llvm-mc rejects the matching asm
**Contract evidence:** inferred (neon.rs:1284 SRI Vd.T, Vn.T; README.md:12 gas-compatible; llvm-mc rejects sri v0.8b, v0, #1)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_sri([v0.8b, Reg(v0), #1])
**Expected / Actual:** Err / Ok(Word) same as `sri v0.8b, v0.8b, #1`
**Impact:** `sri v0.8b, v0, #1` is encoded as `sri v0.8b, v0.8b, #1`. GPR destinations written as `x0.8b` are treated as `v0.8b`.
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement, which encode_neon_sri then discards (`let (rn, _)`)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1290`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require RegArrangement with a v-prefix on both operands
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n.is_empty() {
        return Err("sri requires Vn.T arrangement".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_sri_gpr_or_bare.md
**Repro seed:** (none — first-case shrink)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_sri_pbt::encode_neon_sri_neg_gpr_or_bare' (2357969) panicked at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:267:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects sri v0.8b, v0, #1) at src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs:486.
minimal failing input: rd = 0, rn = 0, kind = 1, fp_prefix = "x"
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_sri_pbt.rs | 10 properties + 1 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_sri_pbt;` |

## Reproduction

Serial reconfirmation: all four property failures reproduced with `--test-threads=1` (the recorded run).

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_sri -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_extra_operand -- --test-threads=1 --exact
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_mismatched_t -- --test-threads=1 --exact
```

B3 shift OOB:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_shift_oob -- --test-threads=1 --exact
```

B4 bare src:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_sri_regression_bare_src -- --test-threads=1 --exact
```

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/CHANGE_SURFACE.md
- pbt-out/run/encode_neon_sri.log
- pbt-out/run/encode_neon_sri_sweep.log
- pbt-out/bug_reports/encode_neon_sri_extra_operand.md
- pbt-out/bug_reports/encode_neon_sri_extra_operand.html
- pbt-out/bug_reports/encode_neon_sri_mismatched_t.md
- pbt-out/bug_reports/encode_neon_sri_mismatched_t.html
- pbt-out/bug_reports/encode_neon_sri_shift_oob.md
- pbt-out/bug_reports/encode_neon_sri_shift_oob.html
- pbt-out/bug_reports/encode_neon_sri_gpr_or_bare.md
- pbt-out/bug_reports/encode_neon_sri_gpr_or_bare.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 15:32 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 125/289 total | PBT candidates: 125 | Tested: 125 (100%) | 0 pass, 125 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 125 |
| **Tested (of PBT candidates)** | **125 / 125 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 125 / 0 |
| **Overall (tested / all functions)** | **125 / 289 (43%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 125 | 125 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 125 | 125 | 0 | 100% |

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
| neon.rs | 68 | 40 | 40 | 100% | covered |
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
