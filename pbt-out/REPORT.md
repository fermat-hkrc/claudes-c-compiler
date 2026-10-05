# PBT Campaign Report: encode_neon_elem

## Summary

**Verdict:** 6 medium: encode_neon_elem silently encodes illegal GNU-style by-element MUL/MLA/MLS/SQDMULH/SQRDMULH (extra operand, mismatched T, H-lane Rm v16–v31, X-prefixed dest, out-of-range index, mismatched lane elem_size) instead of returning Err, so gas-rejected assembly becomes a 32-bit word.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_elem
**Tests:** 12 properties (plus 2 KAT + 6 regression witnesses)
**Result:** 6 passing, 6 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). cargo test --lib encode_neon_elem_pbt executed the symbol (2 KAT + 6 passing properties at 1000 cases).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_elem | 12 properties (6 pass / 6 fail) + 2 KAT + 6 regression | 6 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_elem ignores a fourth operand

**Formal:** ∀ valid by-element triple + extra operand. llvm-mc rejects ⇒ encode_neon_elem(ops++[extra], U, opc) = Err
**Contract evidence:** inferred (public wrapper encode() at encoder/mod.rs:307-310 / 793-796 passes operands through; llvm-mc/gas reject a fourth operand; README.md:12 GNU-style assembly)
**Documentation conflict:** neon.rs:1592 "NEON by-element requires 3 operands" — the error fires only for len < 3; it does not declare extra invalid as an input-domain restriction (no maximum). Context, not an exclusion.
**Severity:** medium
**Counterexample:** encode_neon_elem([v0.4h, v0.4h, v0.h[0], v0.4h], u=0, opcode=0b1000)
**Expected / Actual:** Err / Ok(Word) — fourth operand ignored
**Impact:** Illegal assembly such as `mul v0.4h, v0.4h, v0.h[0], v0.4h` becomes a valid 32-bit word
**Root cause:** neon.rs:1592 checks only `operands.len() < 3` and then encodes, so any extra operand after the lane is dropped
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1592`
```rust
    if operands.len() < 3 { return Err("NEON by-element requires 3 operands".to_string()); }
```
**Suggested fix:** Reject when `operands.len() != 3`
```rust
    if operands.len() != 3 {
        return Err("NEON by-element requires 3 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_elem_extra_operand.md
**Repro seed:** cc 1e1f2d8cf733a5a159408b23c6356532c0b57ee21a62c80fa4fd2013bb107840
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:597:5:
mul v0.4h, v0.4h, v0.h[0], v0.4h must Err (llvm-mc/gas reject a fourth operand)
```

### B2: encode_neon_elem ignores a mismatched source arrangement

**Formal:** ∀ T, T' ≠ T ∈ {8b,16b,4h,8h,2s,4s,2d,1d}. llvm-mc rejects mul Vd.T, Vn.T', Vm.Ts[idx] ⇒ encode_neon_elem([Vd.T, Vn.T', Vm.Ts[idx]], 0, 0b1000) = Err
**Contract evidence:** inferred (ARM matching T; llvm-mc rejects; wrapper encode() passes operands through at encoder/mod.rs:307-310)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_elem([v0.4h, v0.8b, v0.h[0]], u=0, opcode=0b1000)
**Expected / Actual:** Err / Ok(Word) — source arrangement discarded
**Impact:** `mul v0.4h, v0.8b, v0.h[0]` encodes as a valid .4h by-element word
**Root cause:** neon.rs:1594 binds source as `let (rn, _) = get_neon_reg(operands, 1)?`, discarding the source arrangement; Q/size come only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1594`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Compare source arrangement against dest and return Err on mismatch
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("mismatched arrangement: dest {arr_d} src {arr_n}"));
    }
```
**Bug report:** bug_reports/encode_neon_elem_mismatch_t.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_mismatch_t' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:606:5:
mul v0.4h, v0.8b, v0.h[0] must Err (llvm-mc/gas require matching T)
```

### B3: encode_neon_elem truncates H-lane Rm v16-v31 to v0-v15

**Formal:** ∀ rd,rn ∈ {0..31}, rm ∈ {16..31}, idx ∈ {0..7}, T ∈ {4h,8h}. llvm-mc rejects mul Vd.T, Vn.T, Vm.h[idx] ⇒ encode_neon_elem(...) = Err
**Contract evidence:** inferred (ARM size=01 Rm v0-v15; llvm-mc rejects v16.h[0]; `rm & 0xF` is the producing mask, not an API exclusion)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_elem([v0.4h, v0.4h, v16.h[0]], u=0, opcode=0b1000)
**Expected / Actual:** Err / Ok(Word) encoding Rm=v0
**Impact:** `mul v0.4h, v0.4h, v16.h[0]` silently encodes as v0.h[0]
**Root cause:** neon.rs:1605 masks half-word Rm with `rm & 0xF` instead of rejecting rm >= 16
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1605`
```rust
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```
**Suggested fix:** Reject Rm v16-v31 when size is 01 (H)
```rust
    if size == 0b01 && rm > 15 {
        return Err(format!("H-lane Rm v{rm} out of range (v0-v15)"));
    }
    let rm_enc = if size == 0b01 { rm & 0xF } else { rm & 0x1F };
```
**Bug report:** bug_reports/encode_neon_elem_h_rm_hi.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_h_rm_hi' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:615:5:
mul v0.4h, v0.4h, v16.h[0] must Err (ARM size=01 Rm v0-v15; llvm-mc rejects)
```

### B4: encode_neon_elem accepts an X-prefixed arranged destination as a V register

**Formal:** ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-V, x-prefix arrangement, non-lane third, s-dest}. llvm-mc rejects ⇒ encode_neon_elem(ops, 0, 0b1000) = Err
**Contract evidence:** inferred (GNU-style by-element requires Vd.T; llvm-mc rejects `mul x0.4h, ...`; parse_reg_num accepting x is not this function's contract)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_elem([x0.4h, v0.4h, v0.h[0]], u=0, opcode=0b1000)
**Expected / Actual:** Err / Ok(Word) encoding Rd=0 as if dest were v0.4h
**Impact:** `mul x0.4h, v0.4h, v0.h[0]` encodes as `mul v0.4h, ...`
**Root cause:** neon.rs:1593 extracts dest via get_neon_reg → parse_reg_num, which accepts prefix x/w/d/s/q/h/b as well as v
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1593`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
```
**Suggested fix:** Require a `v` prefix on arranged NEON registers before encoding
```rust
    let dest_name = match &operands[0] {
        Operand::RegArrangement { reg, .. } => reg,
        _ => return Err("expected NEON register".to_string()),
    };
    if !dest_name.to_lowercase().starts_with('v') {
        return Err(format!("expected V register, got {dest_name}"));
    }
```
**Bug report:** bug_reports/encode_neon_elem_x_prefix.md
**Repro seed:** cc a86789e549634cbd2b9ab29ecdf1403a36128cd05ac44e8c49ff91fd558214cd
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::test_encode_neon_elem_regression_x_prefix' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:631:5:
mul x0.4h, v0.4h, v0.h[0] must Err (llvm-mc/gas require Vd)
```

### B5: encode_neon_elem wraps an out-of-range lane index instead of rejecting it

**Formal:** ∀ T ∈ {4h,8h,2s,4s}, idx > imax(T). llvm-mc rejects mul Vd.T, Vn.T, Vm.Ts[idx] ⇒ encode_neon_elem(...) = Err
**Contract evidence:** inferred (ARM H:L:M 0..7 / H:L 0..3; llvm-mc "vector lane must be an integer in range [0, 7]"; sibling encode_neon_elem_long range-checks)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_elem([v0.4h, v0.4h, v0.h[8]], u=0, opcode=0b1000)
**Expected / Actual:** Err / Ok(Word) encoding index 0
**Impact:** `mul v0.4h, v0.4h, v0.h[8]` silently encodes as index 0
**Root cause:** neon.rs:1599-1603 takes only the bits that fit in H:L:M / H:L and never range-checks `index`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1599`
```rust
    let (h, l, m_bit) = match size {
        0b01 => ((index >> 2) & 1, (index >> 1) & 1, index & 1),
        0b10 => ((index >> 1) & 1, index & 1, (rm >> 4) & 1),
        _ => return Err("unsupported element size for by-element".to_string()),
    };
```
**Suggested fix:** Range-check index the same way encode_neon_elem_long does
```rust
            if index > 7 {
                return Err(format!("element index {index} out of range for .h"));
            }
```
**Bug report:** bug_reports/encode_neon_elem_index_oob.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::encode_neon_elem_neg_index_oob' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:223:1:
Test failed: index 8 out of range for .h must Err (llvm-mc rejects mul v0.4h, v0.4h, v0.h[8])
```

### B6: encode_neon_elem ignores RegLane elem_size

**Formal:** ∀ T ∈ {4h,8h,2s,4s}, wrong ≠ Ts(T). llvm-mc rejects mul Vd.T, Vn.T, Vm.wrong[idx] ⇒ encode_neon_elem(...) = Err
**Contract evidence:** inferred (llvm-mc rejects Vm.b[idx] for T=.4h; wrapper encode() passes RegLane through)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_elem([v0.4h, v0.4h, v0.b[0]], u=0, opcode=0b1000)
**Expected / Actual:** Err / Ok(Word) — elem_size discarded
**Impact:** `mul v0.4h, v0.4h, v0.b[0]` encodes as a valid .h-lane by-element word
**Root cause:** neon.rs:1595-1596 matches `Operand::RegLane { reg, index, .. }`, dropping `elem_size`
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:1595`
```rust
        Operand::RegLane { reg, index, .. } => (parse_reg_num(reg).ok_or("invalid reg")?, *index),
```
**Suggested fix:** Bind and check elem_size against dest size
```rust
        Operand::RegLane { reg, elem_size, index } => {
            let rm = parse_reg_num(reg).ok_or("invalid reg")?;
            (rm, elem_size.clone(), *index)
        }
```
**Bug report:** bug_reports/encode_neon_elem_lane_elem_mismatch.md
**Repro seed:** (deterministic regression)
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_elem_pbt::encode_neon_elem_neg_lane_elem_mismatch' panicked at src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs:223:1:
Test failed: lane elem_size b must match arrangement h (llvm-mc rejects mul v0.4h, v0.4h, v0.b[0])
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_elem_pbt.rs | 12 properties + 2 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_elem_pbt;` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_elem_pbt -- --test-threads=1
```

Per-bug:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_elem_regression_extra_operand -- --test-threads=1
cargo test --lib test_encode_neon_elem_regression_mismatch_t -- --test-threads=1
cargo test --lib test_encode_neon_elem_regression_h_rm_hi -- --test-threads=1
cargo test --lib test_encode_neon_elem_regression_x_prefix -- --test-threads=1
cargo test --lib test_encode_neon_elem_regression_index_oob -- --test-threads=1
cargo test --lib test_encode_neon_elem_regression_lane_elem_mismatch -- --test-threads=1
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
- pbt-out/bug_reports/encode_neon_elem_extra_operand.md
- pbt-out/bug_reports/encode_neon_elem_extra_operand.html
- pbt-out/bug_reports/encode_neon_elem_mismatch_t.md
- pbt-out/bug_reports/encode_neon_elem_mismatch_t.html
- pbt-out/bug_reports/encode_neon_elem_h_rm_hi.md
- pbt-out/bug_reports/encode_neon_elem_h_rm_hi.html
- pbt-out/bug_reports/encode_neon_elem_x_prefix.md
- pbt-out/bug_reports/encode_neon_elem_x_prefix.html
- pbt-out/bug_reports/encode_neon_elem_index_oob.md
- pbt-out/bug_reports/encode_neon_elem_index_oob.html
- pbt-out/bug_reports/encode_neon_elem_lane_elem_mismatch.md
- pbt-out/bug_reports/encode_neon_elem_lane_elem_mismatch.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 19:57 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 138/289 total | PBT candidates: 138 | Tested: 138 (100%) | 0 pass, 138 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 138 |
| **Tested (of PBT candidates)** | **138 / 138 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 138 / 0 |
| **Overall (tested / all functions)** | **138 / 289 (48%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 138 | 138 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 138 | 138 | 0 | 100% |

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
| neon.rs | 68 | 53 | 53 | 100% | covered |
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
