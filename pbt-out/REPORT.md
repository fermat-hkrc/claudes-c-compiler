# PBT Campaign Report: encode_neon_ins

## Summary

**Verdict:** 4 medium: encode_neon_ins silently encodes invalid INS (extra operand, out-of-range lane, wrong GPR width, SP/FP/mismatched Ts) instead of returning Err, so a GNU-style assembler typo becomes a different legal instruction.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_ins
**Tests:** 10 properties (plus 1 KAT + 6 regression witnesses)
**Result:** 6 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test, C++ reporter listed unrelated binaries and claimed NOT LINKED). The libtest binary ran encode_neon_ins (KAT plus 10 properties). Sweep was a manual arm audit (arity < 2, uppercase V/W/X).
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_ins | 10 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_ins ignores a surplus third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. llvm-mc("ins Vd.ts[i], R, extra") fails ⇒ encode_neon_ins(gpr_ops ++ [extra]) = Err
**Contract evidence:** inferred (README.md:12 gas-compat; llvm-mc rejects a third operand; neon.rs:551 documents the 2-operand arity but the check is `len < 2`)
**Documentation conflict:** neon.rs:551 "ins requires 2 operands" — asserted arity, implemented as `len < 2`, so extras are not excluded. (not independently verified as an exclusion)
**Severity:** medium
**Counterexample:** encode_neon_ins([v0.b[0], w0, w0]) then Ok(Word(0x4e011c00))
**Expected / Actual:** Err / Ok(Word(0x4e011c00)) encoded as `ins v0.b[0], w0`
**Impact:** A typo extra token is assembled as a valid INS instead of an assembler error.
**Root cause:** neon.rs:550 `if operands.len() < 2` only rejects too few operands; extras past index 1 are never inspected.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:550`
```rust
    if operands.len() < 2 {
        return Err("ins requires 2 operands".to_string());
    }
```
**Suggested fix:** Require exactly two operands.
```rust
    if operands.len() != 2 {
        return Err("ins requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_ins_extra_operand.md
**Repro seed:** rd=0, rn=0, extra=0, ts=b, i=0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ins_pbt::test_encode_neon_ins_regression_extra_operand' panicked at src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs:576:5:
ins v0.b[0], w1, w2 must Err (llvm-mc rejects a third operand)
```

### B2: encode_neon_ins masks an out-of-range lane index

**Formal:** ∀ rd,rn ∈ {0..31}, ts ∈ {b,h,s,d}, i ∈ {imax(ts)+1 .. imax(ts)+8}. encode_neon_ins([Vd.ts[i], R]) = Err ∧ encode_neon_ins([Vd.ts[0], Vn.ts[i]]) = Err
**Contract evidence:** inferred (ARM INS lane ranges B[0-15] H[0-7] S[0-3] D[0-1]; llvm-mc rejects bound+1; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ins([v0.b[16], w0]) then Ok(Word(0x4e011c00))
**Expected / Actual:** Err / Ok(Word(0x4e011c00)) encoded as `ins v0.b[0], w0`
**Impact:** An out-of-range lane silently becomes a different legal insert.
**Root cause:** neon.rs:559-563 mask the index (`*index & 0xF` and friends) instead of rejecting values outside the ARM range.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:559`
```rust
            let imm5 = match elem_size.as_str() {
                "b" => ((*index & 0xF) << 1) | 0b00001,
                "h" => ((*index & 0x7) << 2) | 0b00010,
                "s" => ((*index & 0x3) << 3) | 0b00100,
                "d" => ((*index & 0x1) << 4) | 0b01000,
```
**Suggested fix:** Reject `index > max(Ts)` before packing imm5/imm4.
```rust
            if *index > max {
                return Err(format!("ins: lane index {} out of range [0, {}]", index, max));
            }
```
**Bug report:** bug_reports/encode_neon_ins_index_oor.md
**Repro seed:** rd=0, rn=0, ts=b, over=1
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ins_pbt::test_encode_neon_ins_regression_index_oor' panicked at src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs:585:5:
ins v0.b[16], w1 must Err (llvm-mc range for .b is [0, 15])
```

### B3: encode_neon_ins accepts the wrong GPR width

**Formal:** ∀ rd,rn ∈ {0..30}, ts ∈ {b,h,s,d}, i ∈ [0, imax(ts)]. encode_neon_ins([Vd.ts[i], wrong_width(ts,rn)]) = Err
**Contract evidence:** inferred (ARM INS general: Wn for B/H/S, Xn for D; llvm-mc rejects the swap; README.md:12 gas-compat)
**Documentation conflict:** neon.rs:549 "INS Vd.Ts[index], Xn" names Xn informally and does not declare Wn illegal for B/H/S. Not an input-domain restriction.
**Severity:** medium
**Counterexample:** encode_neon_ins([v0.b[0], x0]) then Ok(Word(0x4e011c00))
**Expected / Actual:** Err / Ok(Word(0x4e011c00)) encoded as `ins v0.b[0], w0`
**Impact:** A 64-bit source on a byte insert (or a 32-bit source on a doubleword insert) silently becomes the other width.
**Root cause:** neon.rs:555-557 calls parse_reg_num, which treats xN and wN as the same number; no width check against Ts.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:555`
```rust
        (Operand::RegLane { reg, elem_size, index }, Operand::Reg(rn_name)) => {
            let rd = parse_reg_num(reg).ok_or("invalid NEON register")?;
            let rn = parse_reg_num(rn_name).ok_or("invalid register")?;
```
**Suggested fix:** Require Wn for B/H/S and Xn for D.
```rust
            if want_x && !is_x || !want_x && !is_w {
                return Err(format!("ins: GPR source {} has wrong width for .{}", rn_name, elem_size));
            }
```
**Bug report:** bug_reports/encode_neon_ins_wrong_width_gpr.md
**Repro seed:** rd=0, rn=0, ts=b, i=0
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ins_pbt::test_encode_neon_ins_regression_wrong_width_gpr' panicked at src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs:607:5:
ins v0.b[0], x1 must Err (llvm-mc requires Wn for Ts=B)
```

### B4: encode_neon_ins encodes SP as XZR and ignores mismatched Ts / FP names

**Formal:** ∀ rd,rn ∈ {0..31}, ts ≠ ts2 ∈ {b,h,s,d}, i ∈ [0, imax(ts)], j ∈ [0, imax(ts2)]. encode_neon_ins([Vd.ts[i], Vn.ts2[j]]) = Err ∧ encode_neon_ins([Vd.ts[i], SP|WSP|dN|sN|qN|vN]) = Err
**Contract evidence:** inferred (ARM INS general source is Wn/Xn/WZR/XZR; llvm-mc rejects SP/WSP/dN and mismatched Ts; README.md:12 gas-compat)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_ins([v0.b[0], sp]) then Ok(Word(0x4e011fe0))
**Expected / Actual:** Err / Ok(Word(0x4e011fe0)) encoded as `ins v0.b[0], wzr`
**Impact:** SP is assembled as WZR; `d1` as w1; `ins v0.b[0], v1.h[0]` as a matching-B insert. Related regressions: test_encode_neon_ins_regression_fp_as_gpr, test_encode_neon_ins_regression_size_mismatch.
**Root cause:** neon.rs:557 parse_reg_num("sp") yields 31 and accepts d/s/q/v prefixes; neon.rs:574 binds `_src_size` and never compares it to the destination Ts.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:555`
```rust
        (Operand::RegLane { reg, elem_size, index }, Operand::Reg(rn_name)) => {
            let rd = parse_reg_num(reg).ok_or("invalid NEON register")?;
            let rn = parse_reg_num(rn_name).ok_or("invalid register")?;
```
**Suggested fix:** Restrict the general-form source to W/X/WZR/XZR, and require matching Ts on the element form.
```rust
            if n == "sp" || n == "wsp" || !(is_w || is_x) {
                return Err(format!("ins: GPR source must be Wn/Xn, got {}", rn_name));
            }
            if dst_size != src_size {
                return Err(format!("ins: element size mismatch {} vs {}", dst_size, src_size));
            }
```
**Bug report:** bug_reports/encode_neon_ins_sp_as_zr.md
**Repro seed:** rd=0, rn=0, ts=b, i=0, kind=1
**Raw output:**
```text
thread 'backend::arm::assembler::encoder::encode_neon_ins_pbt::test_encode_neon_ins_regression_sp_as_zr' panicked at src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs:632:5:
ins v0.b[0], sp must Err (llvm-mc rejects SP as INS GPR source)
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_ins_pbt.rs | 10 properties + 1 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_ins_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_ins -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ins_regression_extra_operand -- --test-threads=1
```

B2 index out of range:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ins_regression_index_oor -- --test-threads=1
```

B3 wrong GPR width:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ins_regression_wrong_width_gpr -- --test-threads=1
```

B4 SP as ZR:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_ins_regression_sp_as_zr -- --test-threads=1
```

## Output Directories

- pbt-out/REPORT.md
- pbt-out/REPORT.html
- pbt-out/PROPERTIES.md
- pbt-out/PLAN.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/report.json
- pbt-out/bug_reports/encode_neon_ins_extra_operand.md
- pbt-out/bug_reports/encode_neon_ins_extra_operand.html
- pbt-out/bug_reports/encode_neon_ins_index_oor.md
- pbt-out/bug_reports/encode_neon_ins_index_oor.html
- pbt-out/bug_reports/encode_neon_ins_wrong_width_gpr.md
- pbt-out/bug_reports/encode_neon_ins_wrong_width_gpr.html
- pbt-out/bug_reports/encode_neon_ins_sp_as_zr.md
- pbt-out/bug_reports/encode_neon_ins_sp_as_zr.html
- pbt-out/run/encode_neon_ins_pbt.log
- pbt-out/run/encode_neon_ins_pbt2.log
- pbt-out/run/encode_neon_ins_pbt3.log

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 09:49 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 107/289 total | PBT candidates: 107 | Tested: 107 (100%) | 0 pass, 107 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 107 |
| **Tested (of PBT candidates)** | **107 / 107 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 107 / 0 |
| **Overall (tested / all functions)** | **107 / 289 (37%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 107 | 107 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 107 | 107 | 0 | 100% |

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
| neon.rs | 68 | 22 | 22 | 100% | covered |
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
