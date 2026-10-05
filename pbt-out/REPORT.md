# PBT Campaign Report: encode_neon_addv

## Summary

**Verdict:** 1 high, 3 medium: encode_neon_addv emits the wrong opcode bits for every ADDV (compiler-emitted `addv b0, v0.8b` included), and also silently encodes extra operands, reserved 2S/1D/2D arrangements, and GPR destinations.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_addv
**Tests:** 8 properties (plus 6 KAT + 4 regression witnesses)
**Result:** 2 passing, 4 failing, 2 retired (duplicate encoding oracles), 4 bugs
**Change surface:** 1 changed function (encode_neon_addv), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo lib tests executed encode_neon_addv. Sweep: 1/1 manual arm audit of the function body (arity Err, get_neon_reg Err, neon_arr_to_q_size Err, Ok Word all driven).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_addv | 8 properties (2 passing, 6 failing) | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_addv places ADDV opcode bits incorrectly

**Formal:** ∀ rd,rn ∈ {0..31}, (v,t) ∈ {(b,8b),(b,16b),(h,4h),(h,8h),(s,4s)}. encode_neon_addv([Reg(v{rd}), Vn.t]) = llvm-mc("addv v{rd}, v{rn}.t")
**Contract evidence:** documented neon.rs:433 "ADDV: 0 Q 0 01110 size 11000 11011 10 Rn Rd"
**Documentation conflict:** neon.rs:433 states opcode 11011 then 10; the code uses `0b110111 << 10`, which is bits[15:10]=110111 rather than bits[16:12]=11011 and bits[11:10]=10. The comment is the contract the code violates. (not independently verified)
**Severity:** high
**Counterexample:** encode_neon_addv([b0, v0.8b])
**Expected / Actual:** Word(0x0e31b800) / Word(0x0e30dc00)
**Impact:** Every ADDV, including compiler-emitted `addv b0, v0.8b`, assembles to the wrong machine word
**Root cause:** neon.rs:434-435 uses `0b110111 << 10` instead of `(0b11011 << 12) | (0b10 << 10)`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:434`
```rust
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (0b11000 << 17)
        | (0b110111 << 10) | (rn << 5) | rd;
```
**Suggested fix:** Place opcode and op2 at the ARM bit positions (same pattern as encode_neon_across)
```rust
    let word = (q << 30) | (0b001110 << 24) | (size << 22) | (0b11000 << 17)
        | (0b11011 << 12) | (0b10 << 10) | (rn << 5) | rd;
```
**Bug report:** bug_reports/encode_neon_addv_wrong_encoding.md
**Repro seed:** rd = 0, rn = 0, (v, t) = ("b", "8b")
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_diff_llvm_mc' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: assertion failed: `(left == right)`
  left: `238083072`,
 right: `238139392`: SUT vs llvm-mc for addv b0, v0.8b at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:257.
minimal failing input: rd = 0, rn = 0, (v, t) = (
    "b",
    "8b",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_addv ignores a third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, (v,t) ∈ ADDV_T. llvm-mc("addv v{rd}, v{rn}.t, v{extra}.t") is Err ∧ encode_neon_addv([dest, src, extra]) is Err
**Contract evidence:** documented neon.rs:426 "addv requires 2 operands"
**Documentation conflict:** neon.rs:426 "addv requires 2 operands" only fires when len < 2; it does not declare extra operands valid. The asserted form is two operands; extra is accepted. (not independently verified as an exclusion)
**Severity:** medium
**Counterexample:** encode_neon_addv([b0, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e30dc00))
**Impact:** Invalid three-operand ADDV is assembled as two-operand ADDV; the extra register is dropped
**Root cause:** neon.rs:425 `operands.len() < 2` ignores operands after the first two
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:425`
```rust
    if operands.len() < 2 {
        return Err("addv requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("addv requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_addv_extra_operand.md
**Repro seed:** rd = 0, rn = 0, extra = 0, (v, t) = ("b", "8b")
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: extra operand must Err (llvm-mc rejects addv b0, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:334.
minimal failing input: rd = 0, rn = 0, extra = 0, (v, t) = (
    "b",
    "8b",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_addv encodes reserved ADDV arrangements

**Formal:** ∀ rd,rn ∈ {0..31}, t ∈ {2s,1d,2d,4b,8d,2h,1s}. llvm-mc rejects addv with Vn.t ∧ encode_neon_addv([scalar, Vn.t]) is Err
**Contract evidence:** inferred (ARM Advanced SIMD ADDV T ∈ {8B,16B,4H,8H,4S}; size:Q=10:0 and size=11 reserved; README.md:12 gas-compat; llvm-mc rejects .2s)
**Documentation conflict:** (none) — the function comment names ADDV without restricting T; it does not declare .2s valid
**Severity:** medium
**Counterexample:** encode_neon_addv([s0, v0.2s])
**Expected / Actual:** Err / Ok(Word(0x0eb0dc00))
**Impact:** Illegal 2S (reserved encoding) is assembled instead of diagnosed
**Root cause:** neon.rs:431 calls neon_arr_to_q_size with no ADDV T filter
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:431`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
```
**Suggested fix:** Reject arrangements other than 8b/16b/4h/8h/4s before encoding
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
    if matches!((q, size), (0, 0b10) | (_, 0b11)) {
        return Err(format!("addv: unsupported arrangement: {}", arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_addv_reserved_t.md
**Repro seed:** rd = 0, rn = 0, t = "2s"
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: invalid T must Err (ARM reserved/unsupported; llvm-mc rejects addv s0, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:359.
minimal failing input: rd = 0, rn = 0, t = "2s"
	successes: 2
	local rejects: 0
	global rejects: 0
```

### B4: encode_neon_addv accepts a GPR destination

**Formal:** ∀ rd,rn ∈ {0..31}, (v,t) ∈ ADDV_T, dest ∈ {GPR, SP, Vn.t arrangement, mismatched scalar}. llvm-mc rejects ∧ encode_neon_addv is Err
**Contract evidence:** inferred (ARM ADDV dest is Bd/Hd/Sd matching T; codegen/intrinsics.rs:190 emits `addv b0, v0.8b`; README.md:12 gas-compat; llvm-mc rejects `addv x0, v0.8b`)
**Documentation conflict:** (none) — dest type is discarded; nothing declares GPR dest valid
**Severity:** medium
**Counterexample:** encode_neon_addv([x0, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e30dc00))
**Impact:** GPR dest is encoded as if it were a matching SIMD scalar
**Root cause:** neon.rs:428 `let (rd, _)` keeps only the register number
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:428`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a scalar SIMD dest whose prefix matches T
```rust
    let dest = match &operands[0] {
        Operand::Reg(r) if dest_prefix_matches(r, &arr_n) => parse_reg_num(r).ok_or("invalid dest")?,
        _ => return Err("addv: dest must be Bd/Hd/Sd matching T".to_string()),
    };
```
**Bug report:** bug_reports/encode_neon_addv_invalid_dest.md
**Repro seed:** rd = 0, rn = 0, (v, t) = ("b", "8b"), kind = 0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_addv_pbt::encode_neon_addv_neg_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:245:1:
Test failed: invalid dest must Err (llvm-mc rejects addv x0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs:406.
minimal failing input: rd = 0, rn = 0, (v, t) = (
    "b",
    "8b",
), kind = 0
	successes: 0
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs | 8 properties + 6 KAT + 4 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_addv_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_addv -- --test-threads=1
```

B1 encoding:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_encoding -- --test-threads=1 --exact
```

B2 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_extra_operand -- --test-threads=1 --exact
```

B3 reserved T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_reserved_2s -- --test-threads=1 --exact
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_addv_regression_gpr_dest -- --test-threads=1 --exact
```

## Output Directories

pbt-out/REPORT.md, pbt-out/REPORT.html, pbt-out/PROPERTIES.md, pbt-out/PLAN.md, pbt-out/COVERAGE.md, pbt-out/COVERAGE_STATUS.md, pbt-out/report.json, pbt-out/INVARIANTS.md, pbt-out/FUNCTION_INDEX.md, pbt-out/run/encode_neon_addv_round1.log, pbt-out/bug_reports/encode_neon_addv_wrong_encoding.md, pbt-out/bug_reports/encode_neon_addv_wrong_encoding.html, pbt-out/bug_reports/encode_neon_addv_extra_operand.md, pbt-out/bug_reports/encode_neon_addv_extra_operand.html, pbt-out/bug_reports/encode_neon_addv_reserved_t.md, pbt-out/bug_reports/encode_neon_addv_reserved_t.html, pbt-out/bug_reports/encode_neon_addv_invalid_dest.md, pbt-out/bug_reports/encode_neon_addv_invalid_dest.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 12:38 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 116/289 total | PBT candidates: 116 | Tested: 116 (100%) | 0 pass, 116 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 116 |
| **Tested (of PBT candidates)** | **116 / 116 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 116 / 0 |
| **Overall (tested / all functions)** | **116 / 289 (40%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 116 | 116 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 116 | 116 | 0 | 100% |

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
| neon.rs | 68 | 31 | 31 | 100% | covered |
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
