# PBT Campaign Report: encode_neon_rev64

## Summary

**Verdict:** 4 medium/high bugs: encode_neon_rev64 silently accepts extra operands, reserved .1d/.2d arrangements, mismatched T, and a GPR source, emitting a well-formed REV64 word that llvm-mc and gas reject.
**Date:** 2026-10-05
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neon_rev64
**Tests:** 9 properties (plus 6 KAT + 5 regression witnesses)
**Result:** 5 passing, 4 bugs
**Change surface:** 1 changed function, 1 with a property, 0 error-handling changes with a failure-path property
**Coverage evidence:** file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual arm audit added uppercase-V alt-spellings.
**Tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neon_rev64 | 9 properties (5 pass / 4 fail) + 6 KAT + 5 regression | 4 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### B1: encode_neon_rev64 ignores a third operand

**Formal:** ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. llvm-mc("rev64 Vd.T, Vn.T, Vextra.T") fails ∧ encode_neon_rev64([Vd.T, Vn.T, Vextra.T]) = Err(_)
**Contract evidence:** inferred (arity-2 signature plus llvm-mc/gas rejection of a third operand; neon.rs:754 documents "rev64 requires 2 operands" as the short-arity error, not as permission to ignore extras)
**Documentation conflict:** (none)
**Severity:** medium
**Counterexample:** encode_neon_rev64([v0.8b, v0.8b, v0.8b])
**Expected / Actual:** Err / Ok(Word(0x0e200800))
**Impact:** The assembler silently encodes `rev64 Vd.T, Vn.T, Vextra.T` as `rev64 Vd.T, Vn.T`, dropping the extra operand instead of diagnosing invalid assembly
**Root cause:** neon.rs:753 uses `operands.len() < 2`, so any extra operands after the first two are ignored
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:753`
```rust
    if operands.len() < 2 {
        return Err("rev64 requires 2 operands".to_string());
    }
```
**Suggested fix:** Reject arity other than 2
```rust
    if operands.len() != 2 {
        return Err("rev64 requires 2 operands".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_rev64_extra_operand.md
**Repro seed:** cc 0b7ffb7fcbaabb1f340fa0f7e9b8ffb12e96b8464c4f99aa079e2448250110c0
**Raw output:**
```text
Test failed: extra operand must Err (llvm-mc rejects rev64 v0.8b, v0.8b, v0.8b) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:301.
minimal failing input: rd = 0, rn = 0, extra = 0, t = "8b"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B2: encode_neon_rev64 encodes reserved .1d/.2d arrangements

**Formal:** ∀ rd,rn ∈ {0..31}, T ∈ {1d,2d,4b,8d,1s,2h,8s,32b}. llvm-mc("rev64 Vd.T, Vn.T") fails ∧ encode_neon_rev64([Vd.T, Vn.T]) = Err(_)
**Contract evidence:** inferred (ARM Advanced SIMD two-register miscellaneous REV64 reserves size=11; gas lists only {8b,16b,4h,8h,2s,4s}; llvm-mc rejects .1d/.2d)
**Documentation conflict:** (none) — neon.rs:751 does not restrict T; neon_arr_to_q_size maps 1d/2d to size=11
**Severity:** high
**Counterexample:** encode_neon_rev64([v0.1d, v0.1d])
**Expected / Actual:** Err / Ok(Word(0x0ee00800))
**Impact:** The assembler emits a reserved Advanced SIMD encoding for `rev64 v0.1d, v0.1d` / `rev64 v0.2d, v0.2d` that llvm-mc and gas reject
**Root cause:** neon.rs:759 calls neon_arr_to_q_size which maps "1d"/"2d" to size=11; encode_neon_rev64 never rejects the reserved size
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:759`
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
```
**Suggested fix:** Reject size=11 (and any T not in {8b,16b,4h,8h,2s,4s})
```rust
    let (q, size) = neon_arr_to_q_size(&arr_d)?;
    if size == 0b11 {
        return Err(format!("rev64: unsupported arrangement .{arr_d}"));
    }
```
**Bug report:** bug_reports/encode_neon_rev64_invalid_arrangement.md
**Repro seed:** cc 949e3e382680fe390f078f4918464a9eec5dfa9d4fb72088516bfc306a455d78
**Raw output:**
```text
Test failed: invalid T must Err (only .8b/.16b/.4h/.8h/.2s/.4s; llvm-mc rejects rev64 v0.1d, v0.1d) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:321.
minimal failing input: rd = 0, rn = 0, t = "1d"
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B3: encode_neon_rev64 ignores source arrangement mismatch

**Formal:** ∀ rd,rn ∈ {0..31}, Td,Tn ∈ {8b,16b,4h,8h,2s,4s}, Td ≠ Tn. llvm-mc("rev64 Vd.Td, Vn.Tn") fails ∧ encode_neon_rev64([Vd.Td, Vn.Tn]) = Err(_)
**Contract evidence:** inferred (neon.rs:761 "REV64 Vd.T, Vn.T" same T; llvm-mc/gas operand mismatch)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_rev64([v0.8b, v0.16b])
**Expected / Actual:** Err / Ok(Word(0x0e200800))
**Impact:** `rev64 v0.8b, v0.16b` is encoded as `rev64 v0.8b, v0.8b` (dest T only). Callers can assemble illegal mixed-width reverse and get a well-formed but wrong instruction
**Root cause:** neon.rs:757 discards the source arrangement (`let (rn, _)`), then Q/size come only from dest
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:757`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require source arrangement to equal dest
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n != arr_d {
        return Err(format!("rev64: operand mismatch .{arr_d} vs .{arr_n}"));
    }
```
**Bug report:** bug_reports/encode_neon_rev64_mismatch_t.md
**Repro seed:** (none — mismatch test did not emit a cc seed)
**Raw output:**
```text
Test failed: mismatched T must Err (llvm-mc rejects rev64 v0.8b, v0.16b) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:341.
minimal failing input: rd = 0, rn = 0, (td, tn) = (
    "8b",
    "16b",
)
	successes: 0
	local rejects: 0
	global rejects: 0
```

### B4: encode_neon_rev64 treats a GPR source as a NEON register

**Formal:** ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-v, d-dest, s-dest, q-dest, x-src}. gas rejects the corresponding `rev64` form ∧ encode_neon_rev64(ops(kind)) = Err(_)
**Contract evidence:** inferred (neon.rs:751 NEON REV64; gas: operand 2 must be a SIMD vector register; llvm-mc rejects `rev64 v0.8b, x0`)
**Documentation conflict:** (none)
**Severity:** high
**Counterexample:** encode_neon_rev64([v0.8b, x0])
**Expected / Actual:** Err / Ok(Word(0x0e200800))
**Impact:** `rev64 v0.8b, x0` is encoded as `rev64 v0.8b, v0.8b`. The assembler silently retargets a scalar register number into Vn
**Root cause:** get_neon_reg accepts Operand::Reg and returns an empty arrangement; encode_neon_rev64 then discards that arrangement and uses only the register number as Rn
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/arm/assembler/encoder/neon.rs:757`
```rust
    let (rn, _) = get_neon_reg(operands, 1)?;
```
**Suggested fix:** Require a RegArrangement source (non-empty arrangement matching dest)
```rust
    let (rn, arr_n) = get_neon_reg(operands, 1)?;
    if arr_n.is_empty() || arr_n != arr_d {
        return Err("rev64: source must be a SIMD vector register with matching arrangement".to_string());
    }
```
**Bug report:** bug_reports/encode_neon_rev64_gpr_src.md
**Repro seed:** cc 23296c796186e0d23174cd434bdbacc199a8a681e93b33e4d650d1799a2b2aca
**Raw output:**
```text
Test failed: non-arranged NEON / GPR / SP / FP must Err (gas rejects rev64 v0.8b, x0) at src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs:405.
minimal failing input: rd = 0, rn = 0, t = "8b", kind = 7
	successes: 10
	local rejects: 0
	global rejects: 0
```

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs | 9 properties + 6 KAT + 5 regression witnesses |
| src/backend/arm/assembler/encoder/mod.rs | one-line `#[cfg(test)] mod encode_neon_rev64_pbt` |

## Reproduction

Whole suite:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_neon_rev64 -- --test-threads=1
```

B1 extra operand:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_extra_operand -- --test-threads=1 --exact
```

B2 invalid arrangement:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_invalid_t -- --test-threads=1 --exact
```

B3 mismatch T:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_mismatch_t -- --test-threads=1 --exact
```

B4 GPR source:
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_neon_rev64_regression_gpr_src -- --test-threads=1 --exact
```

## Output Directories

- pbt-out/REPORT.md — this report
- pbt-out/REPORT.html — customer-facing overview (rendered from report.json)
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/PLAN.md — campaign checklist
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/FUNCTION_INDEX.md — function index (merged)
- pbt-out/report.json — machine-readable report
- pbt-out/bug_reports/encode_neon_rev64_extra_operand.md
- pbt-out/bug_reports/encode_neon_rev64_extra_operand.html
- pbt-out/bug_reports/encode_neon_rev64_invalid_arrangement.md
- pbt-out/bug_reports/encode_neon_rev64_invalid_arrangement.html
- pbt-out/bug_reports/encode_neon_rev64_mismatch_t.md
- pbt-out/bug_reports/encode_neon_rev64_mismatch_t.html
- pbt-out/bug_reports/encode_neon_rev64_gpr_src.md
- pbt-out/bug_reports/encode_neon_rev64_gpr_src.html
- pbt-out/run/encode_neon_rev64_test.log — first full test run

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-10-05 12:04 (campaign: coverage)
> Files: 10/10 scanned (100%) | Functions: 114/289 total | PBT candidates: 114 | Tested: 114 (100%) | 0 pass, 114 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 114 |
| **Tested (of PBT candidates)** | **114 / 114 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 114 / 0 |
| **Overall (tested / all functions)** | **114 / 289 (39%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 114 | 114 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 114 | 114 | 0 | 100% |

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
| neon.rs | 68 | 29 | 29 | 100% | covered |
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
