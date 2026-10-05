# PBT Campaign Report: encode_cnt

## Summary

**Verdict:** 2 high, 2 medium: encode_cnt silently encodes illegal CNT forms (extra operand, T∉{8b,16b}, mismatched T, GPR/SP/bare/FP) as CNT .8b 0x0e205800 instead of rejecting them.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_cnt
**Tests:** 9 properties (plus 4 KAT + 6 regression witnesses)
**Result:** 5 passing, 4 bugs
**Change surface:** 1 changed function (encode_cnt), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test; C++ reporter listed unrelated binaries and claimed NOT LINKED). encode_cnt executed via `cargo test --lib encode_cnt`.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_cnt | 9 | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_cnt ignores a third operand

**Formal:** ∀ rd, rn, extra ∈ {0..31}, T ∈ {8b,16b}. llvm-mc("cnt Vd.T, Vn.T, Vextra.T") = Err ∧ encode_cnt([Vd.T, Vn.T, Vextra.T]) = Err
**Contract evidence:** inferred (README.md:12 gas-compatible assembly; llvm-mc/gas reject a third operand)
**Documentation conflict:** (none) — neon.rs:28 "cnt requires 2 operands" states a minimum, not an extra-operand exclusion
**Severity:** medium
**Counterexample:** encode_cnt([v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e205800))
**Impact:** Invalid three-operand CNT is assembled as two-operand CNT, dropping the extra operand without a diagnostic
**Root cause:** neon.rs:27 `if operands.len() < 2` ignores operands after the first two
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:27`
```rust
    if operands.len() < 2 {
        return Err("cnt requires 2 operands".to_string());
    }
```
**Suggested fix:** Require exact arity 2
```rust
    if operands.len() != 2 {
        return Err("cnt requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_cnt_extra_operand.md
**Repro seed:** cc 57dc163b54f78d916c8e2440e5fc9116d14895f3aa905613c639dd58db8118b8
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects cnt v0.8b, v0.8b, v0.8b)
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
```

### B2: encode_cnt encodes illegal arrangements as CNT .8b

**Formal:** ∀ rd, rn ∈ {0..31}, T ∈ {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s}. llvm-mc("cnt Vd.T, Vn.T") = Err ∧ encode_cnt([Vd.T, Vn.T]) = Err
**Contract evidence:** documented neon.rs:26 "Only valid for .8b (Q=0) and .16b (Q=1)"
**Documentation conflict:** neon.rs:26 "Only valid for .8b (Q=0) and .16b (Q=1)" — states the behavior IS handled (illegal T out of domain); the code encodes them as Q=0. Contract the code violates.
**Severity:** high
**Counterexample:** encode_cnt([v0.4h, v0.4h])
**Expected / Actual:** Err / Ok(Word(0x0e205800))
**Impact:** Illegal SIMD arrangements assemble as CNT .8b, emitting the wrong instruction
**Root cause:** neon.rs:33 sets Q=1 only for `"16b"` and Q=0 for every other string
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:33`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 }; // .8b -> Q=0, .16b -> Q=1
```
**Suggested fix:** Reject T other than 8b/16b
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("cnt: unsupported arrangement .{}, expected .8b or .16b", arr_d));
    }
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Bug report:** bug_reports/encode_cnt_invalid_t.md
**Repro seed:** (deterministic regression; proptest shrunk to t="4h")
**Raw output:**
```text
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects cnt v0.4h, v0.4h)
minimal failing input: rd = 0, rn = 0, t = "4h"
```

### B3: encode_cnt ignores a mismatched source arrangement

**Formal:** ∀ rd, rn ∈ {0..31}, Td ≠ Tn, {Td,Tn} ⊆ {8b,16b}. llvm-mc("cnt Vd.Td, Vn.Tn") = Err ∧ encode_cnt([Vd.Td, Vn.Tn]) = Err
**Contract evidence:** documented neon.rs:24 "CNT Vd.<T>, Vn.<T>"
**Documentation conflict:** neon.rs:24 "CNT Vd.<T>, Vn.<T>" — names the same T on both operands; the code ignores source T. Contract the code violates.
**Severity:** medium
**Counterexample:** encode_cnt([v0.8b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0x0e205800))
**Impact:** Mixed .8b/.16b CNT is encoded from dest T only; gas/llvm-mc reject it as operand mismatch
**Root cause:** neon.rs:32 binds `_arr_n` and never compares it to dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:32`
```rust
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require matching arrangements
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_d != arr_n {
        return Err(format!("cnt: arrangement mismatch .{arr_d} vs .{arr_n}"));
    }
```
**Bug report:** bug_reports/encode_cnt_mismatch_t.md
**Repro seed:** (deterministic regression; proptest shrunk to td="8b")
**Raw output:**
```text
Test failed: mismatched T must Err (llvm-mc rejects cnt v0.8b, v0.16b)
minimal failing input: rd = 0, rn = 0, td = "8b"
```

### B4: encode_cnt accepts GPR, SP, bare V, and FP scalar operands

**Formal:** ∀ kind ∈ {gpr-x, gpr-w, sp, bare-v, fp-d, fp-s, fp-q}, rd, rn ∈ {0..31}, T ∈ {8b,16b}. llvm-mc(asm(kind)) = Err ∧ encode_cnt(ops(kind)) = Err
**Contract evidence:** inferred (README.md:12 gas-compatible; neon.rs:24 CNT Vd.<T>, Vn.<T> requires arranged NEON registers)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_cnt([Reg("x0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x0e205800))
**Impact:** GPR/SP/bare-V/FP scalar CNT encodes as CNT v0.8b, mapping integer register numbers onto SIMD Rd/Rn
**Root cause:** neon.rs:31-33 uses get_neon_reg which accepts Operand::Reg via parse_reg_num, then treats empty arrangement as Q=0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:31`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _arr_n) = get_neon_reg(operands, 1)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 }; // .8b -> Q=0, .16b -> Q=1
```
**Suggested fix:** Require RegArrangement with T in {8b,16b} on both operands
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("cnt: unsupported arrangement .{arr_d}, expected .8b or .16b"));
    }
    if arr_n != arr_d {
        return Err(format!("cnt: arrangement mismatch .{arr_d} vs .{arr_n}"));
    }
```
**Bug report:** bug_reports/encode_cnt_gpr_bare_sp.md
**Repro seed:** (deterministic regression; proptest shrunk to kind=0, rd=0, rn=0)
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects cnt x0, x0)
minimal failing input: rd = 0, rn = 0, t = "8b", kind = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_cnt_pbt.rs | 9 properties + 4 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_cnt_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_cnt -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_extra_operand -- --test-threads=1 --exact
```

B2 invalid T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_invalid_t -- --test-threads=1 --exact
```

B3 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_mismatch_t -- --test-threads=1 --exact
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cnt_regression_gpr_dest -- --test-threads=1 --exact
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
- pbt-out/CHANGE_SURFACE.md
- pbt-out/run/encode_cnt_pbt.log
- pbt-out/run/encode_cnt_pbt_round2.log
- pbt-out/bug_reports/encode_cnt_extra_operand.md
- pbt-out/bug_reports/encode_cnt_extra_operand.html
- pbt-out/bug_reports/encode_cnt_invalid_t.md
- pbt-out/bug_reports/encode_cnt_invalid_t.html
- pbt-out/bug_reports/encode_cnt_mismatch_t.md
- pbt-out/bug_reports/encode_cnt_mismatch_t.html
- pbt-out/bug_reports/encode_cnt_gpr_bare_sp.md
- pbt-out/bug_reports/encode_cnt_gpr_bare_sp.html

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 11:29 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 112/289 total | PBT candidates: 112 | Tested: 112 (100%) | 0 pass, 112 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 112 |
| **Tested (of PBT candidates)** | **112 / 112 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 112 / 0 |
| **Overall (tested / all functions)** | **112 / 289 (39%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 112 | 112 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 112 | 112 | 0 | 100% |

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
| neon.rs | 68 | 27 | 27 | 100% | covered |
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
