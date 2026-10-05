# PBT Campaign Report: encode_neon_not

## Summary

**Verdict:** 1 high: encode_neon_not encodes illegal arrangements (e.g. `not v0.4h, v0.4h`) as NOT .8b (0x2e205800), so callers assembling invalid SIMD NOT get a silently wrong instruction; plus 2 more high (mismatched T, GPR/SP/bare-V/FP) and 1 medium (extra operand ignored).
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_not
**Tests:** 9 properties (plus 4 KAT + 6 regression witnesses)
**Result:** 5 passing, 4 failing (4 bugs)
**Change surface:** 1 changed function (encode_neon_not), 1 with a property, 0 error-handling changes
**Coverage evidence:** file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (Rust cargo test tree was not C++-instrumented; the C++ reporter listed unrelated binaries and claimed NOT LINKED). The cargo test binary did execute encode_neon_not (9 properties, 1000 cases each on the passing set). Sweep round 1/1: uppercase-V alt-spellings (passing). Closed: tier round spent.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_not | 9 properties (5 pass / 4 fail) + 4 KAT + 6 regressions | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_not ignores a third operand

**Formal:** ∀ rd, rn, extra ∈ {0..31}, T ∈ {8b,16b}. llvm-mc rejects "not Vd.T, Vn.T, Vextra.T" ⇒ encode_neon_not([Vd.T, Vn.T, Vextra.T]) = Err
**Contract evidence:** inferred (signature takes a slice; neon.rs:607 asserts NOT Vd.T, Vn.T (two operands); README.md:12 gas-compatible assembly; llvm-mc/gas reject a third operand)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_not([v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x2e205800))
**Impact:** The assembler silently encodes `not Vd.T, Vn.T, Vextra.T` as `not Vd.T, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly
**Root cause:** neon.rs:609 uses `operands.len() < 2`, so extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:609`
```rust
    if operands.len() < 2 {
        return Err("not requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("not requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_not_extra_operand.md
**Repro seed:** cc 8c55d5df0fc521f697b2a99640d5cbac78f1d0738726e91041942b8508868ed7
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects not v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:258.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
```

### B2: encode_neon_not encodes illegal arrangements as NOT .8b

**Formal:** ∀ rd, rn ∈ {0..31}, T ∉ {8b,16b}. llvm-mc rejects "not Vd.T, Vn.T" ⇒ encode_neon_not([Vd.T, Vn.T]) = Err
**Contract evidence:** inferred (ARM Advanced SIMD two-register miscellaneous NOT: T in {8B,16B} only; llvm-mc "invalid operand"; gas "operand mismatch"; README.md:225 lists not/mvn under NEON two-misc)
**Documentation conflict:** (none) — neon.rs:607 asserts NOT Vd.T, Vn.T but does not name the T set; sibling encode_neon_rbit does check .8b/.16b
**Severity:** high
**Counterexample:** encode_neon_not([v0.4h, v0.4h])
**Expected / Actual:** Err / Ok(Word(0x2e205800))
**Impact:** `not v0.4h, v0.4h` (and 8h/2s/4s/2d/1d) is assembled as `not v0.8b, v0.8b`, emitting the wrong instruction instead of an error
**Root cause:** neon.rs:615 sets Q=1 only for "16b" and Q=0 for every other arrangement, with no check that T is 8b or 16b
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:615`
```rust
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Require T in {8b,16b} before encoding Q
```rust
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("not: unsupported arrangement .{arr_d}, expected .8b or .16b"));
    }
    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Bug report:** bug_reports/encode_neon_not_invalid_t.md
**Repro seed:** (none — shrinks to rd=0, rn=0, t="4h")
**Raw output:**
```text
Test failed: invalid T must Err (only .8b/.16b; llvm-mc rejects not v0.4h, v0.4h) at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:278.
minimal failing input: rd = 0, rn = 0, t = "4h"
```

### B3: encode_neon_not ignores a mismatched source arrangement

**Formal:** ∀ rd, rn ∈ {0..31}, Td ≠ Tn ∈ {8b,16b}. llvm-mc rejects "not Vd.Td, Vn.Tn" ⇒ encode_neon_not([Vd.Td, Vn.Tn]) = Err
**Contract evidence:** inferred (neon.rs:607 NOT Vd.T, Vn.T uses one T; llvm-mc/gas reject mismatched arrangements)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_not([v0.8b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0x2e205800))
**Impact:** `not v0.8b, v0.16b` is assembled as `not v0.8b, v0.8b`, using only the destination arrangement
**Root cause:** neon.rs:613 discards the source arrangement (`let (rn, _) = get_neon_reg(operands, 1)?`)
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:613`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require the source arrangement to match the destination
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("not: arrangement mismatch .{arr_d} vs .{arr_n}"));
    }
```
**Bug report:** bug_reports/encode_neon_not_mismatch_t.md
**Repro seed:** (none — shrinks to rd=0, rn=0, td="8b")
**Raw output:**
```text
Test failed: mismatched T must Err (llvm-mc rejects not v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:299.
minimal failing input: rd = 0, rn = 0, td = "8b"
```

### B4: encode_neon_not accepts GPR, SP, bare V, and FP scalar operands

**Formal:** ∀ kind ∈ {x-gpr, w-gpr, sp, bare-v, d-fp, s-fp, q-fp}. llvm-mc rejects the corresponding `not` ⇒ encode_neon_not(ops(kind)) = Err
**Contract evidence:** inferred (neon.rs:607 NOT Vd.T, Vn.T; llvm-mc rejects `not x0, x0` / `not v0, v1` / `not sp, v0.8b`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_not([Reg("x0"), Reg("x0")])
**Expected / Actual:** Err / Ok(Word(0x2e205800))
**Impact:** `not x0, x0` (also `not sp, v0.8b`, `not v0, v1`, `not d0, d1`) encodes as NOT v0.8b, v0.8b, mapping GPRs/SP/bare V/FP scalars onto SIMD register numbers
**Root cause:** neon.rs:612-615 calls get_neon_reg, which accepts Operand::Reg via parse_reg_num (x/w/d/s/q/v/h/b/sp), then treats the empty arrangement as Q=0
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:612`
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, _) = get_neon_reg(operands, 1)?;

    let q: u32 = if arr_d == "16b" { 1 } else { 0 };
```
**Suggested fix:** Require RegArrangement with T in {8b,16b} on both operands
```rust
    let (rd, arr_d) = get_neon_reg(operands, 0)?;
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_d != "8b" && arr_d != "16b" {
        return Err(format!("not: unsupported arrangement .{arr_d}, expected .8b or .16b"));
    }
    if arr_n != arr_d {
        return Err(format!("not: arrangement mismatch .{arr_d} vs .{arr_n}"));
    }
```
**Bug report:** bug_reports/encode_neon_not_gpr_bare_sp.md
**Repro seed:** (none — shrinks to rd=0, rn=0, t="8b", kind=0)
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / FP must Err (llvm-mc rejects not x0, x0) at src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs:355.
minimal failing input: rd = 0, rn = 0, t = "8b", kind = 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_not_pbt.rs | 9 properties + 4 KAT + 6 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_not_pbt` |

## Reproduction

Whole suite (serial, as run):
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_not -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_not_regression_extra_operand -- --test-threads=1 --exact
```

B2 invalid T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_not_regression_invalid_t -- --test-threads=1 --exact
```

B3 mismatched T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_not_regression_mismatch_t -- --test-threads=1 --exact
```

B4 GPR dest:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_not_regression_gpr_dest -- --test-threads=1 --exact
```

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/REPORT.html (rendered from report.json)
- pbt-out/report.json
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/encode_neon_not_extra_operand.md
- pbt-out/bug_reports/encode_neon_not_extra_operand.html
- pbt-out/bug_reports/encode_neon_not_invalid_t.md
- pbt-out/bug_reports/encode_neon_not_invalid_t.html
- pbt-out/bug_reports/encode_neon_not_mismatch_t.md
- pbt-out/bug_reports/encode_neon_not_mismatch_t.html
- pbt-out/bug_reports/encode_neon_not_gpr_bare_sp.md
- pbt-out/bug_reports/encode_neon_not_gpr_bare_sp.html
- pbt-out/run/encode_neon_not_pbt.log
- pbt-out/run/encode_neon_not_pbt2.log
- proptest-regressions/backend/arm/assembler/encoder/encode_neon_not_pbt.txt (proptest failure seeds, written by the framework next to the crate)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 11:46 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 113/289 total | PBT candidates: 113 | Tested: 113 (100%) | 0 pass, 113 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 113 |
| **Tested (of PBT candidates)** | **113 / 113 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 113 / 0 |
| **Overall (tested / all functions)** | **113 / 289 (39%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 113 | 113 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 113 | 113 | 0 | 100% |

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
| neon.rs | 68 | 28 | 28 | 100% | covered |
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
