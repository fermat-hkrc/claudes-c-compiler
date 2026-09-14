# PBT Campaign Report: encode_neg

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_neg (src/backend/riscv/assembler/encoder/pseudo.rs)
**Tests:** 9 properties + 1 KAT + 1 regression witness
**Result:** 8 passing, 1 bug
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 strengthening/sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_neg | 9 properties (8 passing, 1 failing) + 1 KAT + 1 regression | 1 | differential (llvm-mc), algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### encode_neg silently ignores extra operands
- **Law:** `neg` is a two-operand pseudoinstruction (`neg rd, rs`). Extra operands must be rejected.
- **Shrunk counterexample:** `[Reg("zero"), Reg("zero"), Reg("zero")]`
- **Expected:** Err (llvm-mc: "invalid operand for instruction")
- **Actual:** Ok(Word(0x40000033)) — encodes as `neg zero, zero`
- **Root cause:** `encode_neg` only calls `get_reg` on indices 0 and 1; trailing operands are never examined.
- **Impact:** Assembler accepts `neg rd, rs, extra` and emits the two-operand encoding, silently disagreeing with llvm-mc / the documented form.
- **Severity:** medium
- **Fix:** Reject `operands.len() != 2` (or `operands.get(2).is_some()`).
- **Bug report:** pbt-out/bug_reports/encode_neg_extra_operand.md
- **Regression test:** `encode_neg_pbt::test_encode_neg_regression_extra_operand` (fails until fixed)
- **Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1`

## Design Caveats (if any)

- **Imm 0..=31 as a register number.** `get_reg` accepts `Operand::Imm(n)` for `0 <= n <= 31`. llvm-mc rejects textual `neg 10, 11`. This is an in-tree documented extension, not a bug.
  - Doc evidence: `src/backend/riscv/assembler/encoder/mod.rs:351-352` — quote: `// GCC sometimes emits bare register numbers (0-31) in inline asm`

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/riscv/assembler/encoder/pseudo.rs (mod encode_neg_pbt) | 9 properties + 1 KAT + 1 regression |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger (encode_neg row appended)
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/FUNCTION_INDEX.md — merged index including pseudo.rs
- pbt-out/INVARIANTS.md — encode_neg invariants prepended
- pbt-out/bug_reports/encode_neg_extra_operand.md — extra-operand bug
- pbt-out/build.log — pre-campaign `cargo check --lib` log

**Contract-surface sweep:** 1 round (standard). `coverage_gaps` had no LLVM profraw; manual arm audit of `get_reg` (Reg / Imm 0..=31 / Imm OOB / other kinds / missing index) plus documented SUB expansion vs llvm-mc. Added `encode_neg_diff_llvm_mc_sub` and SymbolOffset/MemSymbol invalid kinds. Closed because the tier's one sweep round is done.

**Skipped targets:** (none). In-scope symbol only.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 10:13 (campaign: coverage)
> Files: 7/7 scanned (100%) | Functions: 46/229 total | PBT candidates: 46 | Tested: 46 (100%) | 0 pass, 46 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 7 |
| Files scanned | 7 / 7 (100%) |
| Total functions (all files) | 229 |
| PBT candidates (from FUNCTION_INDEX) | 46 |
| **Tested (of PBT candidates)** | **46 / 46 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 46 / 0 |
| **Overall (tested / all functions)** | **46 / 229 (20%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 46 | 46 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 46 | 46 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 14 | 14 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 7 | 7 | 100% | covered |
| pseudo.rs | 44 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
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
