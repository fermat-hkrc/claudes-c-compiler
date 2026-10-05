# PBT Campaign Report: encode_neon_across

## Summary

**Verdict:** 3 medium: encode_neon_across silently encodes a third operand, reserved T (2S/1D/2D), and a GPR/mismatched dest that llvm-mc/gas reject, so invalid UMAXV/UMINV/SMAXV/SMINV assembly becomes a NEON word.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_across
**Tests:** 9 properties (plus 10 KAT + 3 failing regression witnesses)
**Result:** 6 passing, 3 bugs
**Change surface:** 1 changed function (encode_neon_across), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo lib tests executed encode_neon_across.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_across | 9 | 3 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_across ignores a third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, ∀ valid (v,T), ∀ caller (mnem,U,opc). llvm-mc("{mnem} {v}{rd}, v{rn}.{T}, v{extra}.{T}") is Err ⇒ encode_neon_across([Vd, Vn.T, Vextra.T], U, opc) is Err
**Contract evidence:** documented neon.rs:447 "NEON across-vector requires 2 operands"
**Documentation conflict:** neon.rs:447 "NEON across-vector requires 2 operands" — states the instruction requires 2 operands (exactly); the check is `len < 2`, so extras are accepted. The comment is the contract the code violates. (not independently verified)
**Severity:** medium
**Counterexample:** encode_neon_across([b0, v0.8b, v0.8b], 1, 0b01010)
**Expected / Actual:** Err / Ok(Word(0x2e30a800))
**Impact:** The assembler silently encodes `umaxv Vd, Vn.T, extra` as `umaxv Vd, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly
**Root cause:** neon.rs:446 uses `operands.len() < 2`, so extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:446`
```rust
    if operands.len() < 2 {
        return Err("NEON across-vector requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("NEON across-vector requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_across_extra_operand.md
**Repro seed:** rd = 0, rn = 0, extra = 0, (v, t) = ("b", "8b"), (mnem, u_bit, opcode) = ("umaxv", 1, 10)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_across_pbt::encode_neon_across_neg_extra' panicked at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:297:1:
Test failed: extra operand must Err (llvm-mc rejects umaxv b0, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:417.
minimal failing input: rd = 0, rn = 0, extra = 0, (v, t) = (
    "b",
    "8b",
), (mnem, u_bit, opcode) = (
    "umaxv",
    1,
    10,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_across encodes reserved across-lanes arrangements

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ T ∈ {2s,1d,2d,4b,8d,2h,1s}, ∀ caller (mnem,U,opc). llvm-mc("{mnem} {dest}{rd}, v{rn}.{T}") is Err ⇒ encode_neon_across([Reg(dest∥rd), RegArrangement(v∥rn,T)], U, opc) is Err
**Contract evidence:** inferred (ARM Advanced SIMD across lanes T in {8B,16B,4H,8H,4S}; size:Q=10:0 and size=11 reserved; llvm-mc/gas reject)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_across([s0, v0.2s], 1, 0b01010)
**Expected / Actual:** Err / Ok(Word(0x2eb0a800))
**Impact:** Illegal `umaxv s0, v0.2s` (and 1d/2d) is assembled into a reserved encoding instead of being diagnosed
**Root cause:** neon.rs:452 calls neon_arr_to_q_size on the source arrangement with no across-lanes T filter, so 2s/1d/2d become size/Q fields
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:452`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
```
**Suggested fix:** Reject arrangements other than 8b/16b/4h/8h/4s before encoding
```rust
    let (q, size) = neon_arr_to_q_size(&arr_n)?;
    if matches!((q, size), (0, 0b10) | (_, 0b11)) {
        return Err(format!("NEON across-vector: unsupported arrangement: {}", arr_n));
    }
```
**Bug report:** bug_reports/encode_neon_across_reserved_t.md
**Repro seed:** rd = 0, rn = 0, t = "2s", (mnem, u_bit, opcode) = ("umaxv", 1, 10)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_across_pbt::encode_neon_across_neg_invalid_t' panicked at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:297:1:
Test failed: invalid T must Err (ARM reserved/unsupported; llvm-mc rejects umaxv s0, v0.2s) at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:443.
minimal failing input: rd = 0, rn = 0, t = "2s", (mnem, u_bit, opcode) = (
    "umaxv",
    1,
    10,
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_across accepts a GPR destination

**Formal:** ∀ rd,rn ∈ {0..31}, ∀ valid (v,T), ∀ caller (mnem,U,opc), ∀ dest ∈ {GPR-x, GPR-w, sp, Vd.T arranged, mismatched scalar prefix}. llvm-mc rejects dest ⇒ encode_neon_across([bad_dest, Vn.T], U, opc) is Err
**Contract evidence:** inferred (ARM dest Bd/Hd/Sd matching T; llvm-mc/gas reject GPR/arranged/mismatch; public dispatch encoder/mod.rs:693-696 passes dest through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_across([x0, v0.8b], 1, 0b01010)
**Expected / Actual:** Err / Ok(Word(0x2e30a800))
**Impact:** `umaxv x0, v0.8b` is assembled as if dest were b0, producing a NEON word for invalid assembly
**Root cause:** neon.rs:449 takes only the register number from dest (`let (rd, _)`) and never checks that dest is a matching B/H/S SIMD scalar
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:449`
```rust
    let (rd, _) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a scalar SIMD dest whose prefix matches T
```rust
    let dest = match &operands[0] {
        Operand::Reg(r) if dest_prefix_matches(r, &arr_n) => parse_reg_num(r).ok_or("invalid dest")?,
        _ => return Err("NEON across-vector: dest must be Bd/Hd/Sd matching T".to_string()),
    };
```
**Bug report:** bug_reports/encode_neon_across_invalid_dest.md
**Repro seed:** rd = 0, rn = 0, (v, t) = ("b", "8b"), (mnem, u_bit, opcode) = ("umaxv", 1, 10), kind = 0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_across_pbt::encode_neon_across_neg_dest' panicked at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:297:1:
Test failed: invalid dest must Err (llvm-mc rejects umaxv x0, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs:491.
minimal failing input: rd = 0, rn = 0, (v, t) = (
    "b",
    "8b",
), (mnem, u_bit, opcode) = (
    "umaxv",
    1,
    10,
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
| src/backend/arm/assembler/encoder/encode_neon_across_pbt.rs | 9 properties + 10 KAT + 3 regression witnesses |

## Reproduction

Whole suite (filter avoids encode_neon_across_long):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_across_pbt -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_across_regression_extra_operand -- --test-threads=1 --exact
```

B2 reserved T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_across_regression_reserved_2s -- --test-threads=1 --exact
```

B3 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_across_regression_gpr_dest -- --test-threads=1 --exact
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/report.json
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_across_extra_operand.md
- pbt-out/bug_reports/encode_neon_across_extra_operand.html
- pbt-out/bug_reports/encode_neon_across_reserved_t.md
- pbt-out/bug_reports/encode_neon_across_reserved_t.html
- pbt-out/bug_reports/encode_neon_across_invalid_dest.md
- pbt-out/bug_reports/encode_neon_across_invalid_dest.html
- pbt-out/run/encode_neon_across.log
- pbt-out/run/encode_neon_across_pbt.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 12:55 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 117/289 total | PBT candidates: 117 | Tested: 117 (100%) | 0 pass, 117 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 117 |
| **Tested (of PBT candidates)** | **117 / 117 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 117 / 0 |
| **Overall (tested / all functions)** | **117 / 289 (40%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 117 | 117 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 117 | 117 | 0 | 100% |

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
| neon.rs | 68 | 32 | 32 | 100% | covered |
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
