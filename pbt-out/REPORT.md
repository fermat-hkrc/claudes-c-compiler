# PBT Campaign Report: encode_neon_shl

## Summary

**Verdict:** 4 medium: encode_neon_shl silently encodes a fourth operand, mismatched Vn.T, an out-of-range shift (debug overflow on #-1; wrap on #8), and a bare source register (and GPR dest), so gas-incompatible assembly becomes a wrong 32-bit word instead of an error.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_shl
**Tests:** 10
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and claimed NOT LINKED against unrelated C++ binaries; manual arm audit of encode_neon_shl drove arity, get_neon_reg, get_imm, neon_arr_to_q_size, match T, and Ok Word. Sweep added alt-spellings and non-register negatives.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_shl | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_shl ignores a fourth operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1]. encode_neon_shl([Vd.T, Vn.T, #shift, Vextra.T]) = Err
**Contract evidence:** inferred (README.md:12 gas-compatible textual assembly; llvm-mc rejects a fourth operand; neon.rs:1241 three-operand form `SHL Vd.T, Vn.T, #shift`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_shl([v0.8b, v0.8b, #0, v0.8b])
**Expected / Actual:** Err / Ok(Word) same as `shl v0.8b, v0.8b, #0`
**Impact:** The assembler silently encodes `shl Vd.T, Vn.T, #shift, extra` as three-operand SHL, dropping the extra operand instead of diagnosing invalid assembly
**Root cause:** neon.rs:1232 checks `operands.len() < 3` only, so extra operands after the first three are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1232`
```rust
    if operands.len() < 3 {
        return Err("shl requires 3 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 3
```rust
    if operands.len() != 3 {
        return Err("shl requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_shl_extra_operand.md
**Repro seed:** cc 29b853fbb31c1319405191e8d23c256f449426e74b518ce18974cb185241e0a8
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shl_pbt::encode_neon_shl_neg_extra_operand' (2353101) panicked at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:265:1:
Test failed: 4 operands must Err (llvm-mc rejects shl v0.8b, v0.8b, #0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:368.
minimal failing input: rd = 0, rn = 0, extra = 0, t_shift = (
    "8b",
    0,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_shl ignores the source arrangement

**Formal:** ∀ rd,rn ∈ {0..31}, Td,Tn ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}, shift ∈ [0,64]. (Td ≠ Tn ∨ Td ∉ valid_SHL_T) ⇒ encode_neon_shl([Vd.Td, Vn.Tn, #shift]) = Err
**Contract evidence:** inferred (neon.rs:1241 `SHL Vd.T, Vn.T, #shift` matching T; ARM reserved 1D; llvm-mc rejects `shl v0.2s, v0.4s, #0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_shl([v0.2s, v0.4s, #0])
**Expected / Actual:** Err / Ok(Word) encoded as if both were .2s
**Impact:** Invalid assembly `shl v0.2s, v0.4s, #0` is encoded as `shl v0.2s, v0.2s, #0` (size and Q taken only from the destination)
**Root cause:** neon.rs:1236 discards the source arrangement (`let (rn, _)`), so Td/Tn are never compared
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1236`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require matching arrangements
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("shl arrangement mismatch: {} vs {}", arr_d, arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_shl_mismatched_t.md
**Repro seed:** cc 40417577166f44f0f46b514af39abf239ea1389702d6ef3d22a99f1d92cdb731
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shl_pbt::encode_neon_shl_neg_invalid_t' (2353139) panicked at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:265:1:
Test failed: invalid/mismatched/reserved T must Err (ARM SHL T in {8B,16B,4H,8H,2S,4S,2D} matching; llvm-mc rejects shl v0.2s, v0.4s, #0) at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:394.
minimal failing input: rd = 0, rn = 0, td = "2s", tn = "4s", shift = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_shl accepts a shift outside [0, esize-1] (debug overflow / wrap)

**Formal:** ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∉ [0, esize(T)-1]. encode_neon_shl([Vd.T, Vn.T, #shift]) = Err
**Contract evidence:** inferred (ARM SHL shift in [0, esize-1]; README.md:12 gas-compatible; llvm-mc rejects `shl v0.8b, v0.8b, #-1` and `#8`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_shl([v0.8b, v0.8b, #-1])
**Expected / Actual:** Err / debug panic "attempt to add with overflow"; for shift=8, Ok(Word) with reserved immh=0000
**Impact:** `shl v0.8b, v0.8b, #-1` panics in debug instead of diagnosing invalid assembly. `shl v0.8b, v0.8b, #8` is encoded (immh:immb wrapped to 0) instead of rejected
**Root cause:** neon.rs:1237 casts the i64 immediate to u32 then neon.rs:1245 computes `(8 + shift) & 0xF` with no range check
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1245`
```rust
        "8b" | "16b" => (8 + shift) & 0xF,
```
**Suggested fix:** Reject shift outside [0, esize-1] before encoding
```rust
        "8b" | "16b" => {
            if shift > 7 {
                return Err(format!("shl shift {} out of range [0, 7]", shift));
            }
            8 + shift
        }
```
**Bug report:** bug_reports/encode_neon_shl_shift_oob.md
**Repro seed:** (none — overflow panic, not a saved proptest cc)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shl_pbt::encode_neon_shl_neg_shift_oob' (2353161) panicked at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:265:1:
Test failed: attempt to add with overflow.
minimal failing input: rd = 0, rn = 0, t_shift = (
    "8b",
    -1,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B4: encode_neon_shl accepts a bare (non-arrangement) source register

**Formal:** ∀ rd,rn ∈ {0..31}, kind ∈ {GPR dest, bare Vn, bare Vd, xN.8b dest, GPR src}. encode_neon_shl(kind) = Err
**Contract evidence:** inferred (neon.rs:1241 `SHL Vd.T, Vn.T, #shift`; README.md:12; llvm-mc rejects `shl v0.8b, v0, #0` and `shl x0.8b, v0.8b, #0`)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_shl([v0.8b, Operand::Reg("v0"), #0])
**Expected / Actual:** Err / Ok(Word) encoded as `shl v0.8b, v0.8b, #0`
**Impact:** Invalid assembly `shl v0.8b, v0, #0` is encoded as `shl v0.8b, v0.8b, #0`. The same helper also accepts `shl x0.8b, v0.8b, #0` as NEON SHL with Rd taken from the X-register number
**Root cause:** neon.rs:1236 calls get_neon_reg, whose Operand::Reg arm returns an empty arrangement that the caller discards; parse_reg_num also accepts x/w prefixes
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1236`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require RegArrangement with a V-prefixed register on dest and src
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n.is_empty() {
        return Err("shl source must be Vn.T".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_shl_bare_src.md
**Repro seed:** (none)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_shl_pbt::encode_neon_shl_neg_gpr_or_bare' (2353121) panicked at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:265:1:
Test failed: GPR/bare/non-arrangement kind=1 must Err (llvm-mc rejects shl v0.8b, v0, #0) at src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs:484.
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
| src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs | 10 properties + 1 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_shl_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_shl_pbt -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shl_regression_extra_operand -- --test-threads=1 --exact
```

B2 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shl_regression_mismatched_t -- --test-threads=1 --exact
```

B3 shift OOB:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shl_regression_shift_oob -- --test-threads=1 --exact
```

B4 bare src:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_shl_regression_bare_src -- --test-threads=1 --exact
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
- pbt-out/bug_reports/encode_neon_shl_extra_operand.md
- pbt-out/bug_reports/encode_neon_shl_extra_operand.html
- pbt-out/bug_reports/encode_neon_shl_mismatched_t.md
- pbt-out/bug_reports/encode_neon_shl_mismatched_t.html
- pbt-out/bug_reports/encode_neon_shl_shift_oob.md
- pbt-out/bug_reports/encode_neon_shl_shift_oob.html
- pbt-out/bug_reports/encode_neon_shl_bare_src.md
- pbt-out/bug_reports/encode_neon_shl_bare_src.html
- pbt-out/run/encode_neon_shl_pbt.log
- pbt-out/run/kat.log
- pbt-out/run/sweep_alt.log
- pbt-out/run/sweep_nonreg.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 15:16 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 124/289 total | PBT candidates: 124 | Tested: 124 (100%) | 0 pass, 124 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 124 |
| **Tested (of PBT candidates)** | **124 / 124 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 124 / 0 |
| **Overall (tested / all functions)** | **124 / 289 (43%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 124 | 124 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 124 | 124 | 0 | 100% |

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
| neon.rs | 68 | 39 | 39 | 100% | covered |
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
