# PBT Campaign Report: encode_adr

## Summary

**Date:** 2026-09-11
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_adr
**Tests:** 12 properties (plus 2 KAT + 6 regression witnesses)
**Result:** 6 passing, 6 failing (5 distinct SUT bugs)
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 strengthening round, 1 coverage-surface sweep)
**Contract-surface sweep:** 1 round. `coverage_gaps` had no LLVM profraw (RUSTFLAGS/LLVM_PROFILE_FILE unset). Manual arm audit of encode_adr / get_symbol added encode_adr_symbol_misclassified (passing) and encode_adr_neg_modifier_offset (failing, same bug as :lo12: Modifier). Close reason: the tier's one sweep round is done.

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_adr | 12 properties (6 pass / 6 fail) | 5 | differential, algebraic.round_trip, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

1. **encode_adr accepts W registers as Rd.** Law: ADR takes Xd only; llvm-mc rejects `adr w0, #imm`. Minimal input: `[Reg("w0"), Imm(-1048576)]` → Ok(Word(0x10800000)) same as `adr x0, #-1048576`. Severity: high. Serial reconfirm: PBT_TEST_JOBS=1. Report: `pbt-out/bug_reports/encode_adr_w_reg.md`

2. **encode_adr treats SP as XZR.** Law: ADR Rd is Xd; register 31 is XZR not SP; llvm-mc rejects `adr sp, #0`. Minimal input: `[Reg("sp"), Imm(-1048576)]` → Ok(Word(0x1080001f)) same as `adr xzr, #-1048576`. Severity: high. Serial reconfirm: PBT_TEST_JOBS=1. Report: `pbt-out/bug_reports/encode_adr_sp_as_zr.md`

3. **encode_adr silently truncates immediates outside the 21-bit signed range.** Law: offset ∈ [-1048576, 1048575]; llvm-mc rejects `#-1048577`; SUT TODO at load_store.rs:697. Minimal input: `[Reg("x0"), Imm(-1048577)]` → Ok(Word(0x70ffffe0)) same as `adr x0, #-1`. Severity: high. Serial reconfirm: PBT_TEST_JOBS=1. Report: `pbt-out/bug_reports/encode_adr_imm_range.md`

4. **encode_adr accepts FP/SIMD register names as GPR Rd.** Law: ADR takes Xd only; llvm-mc rejects `adr d0, #0`. Minimal input: `[Reg("d0"), Imm(-1048576)]` encoded as x0. Severity: medium. Serial reconfirm: PBT_TEST_JOBS=1. Report: `pbt-out/bug_reports/encode_adr_fp_reg.md`

5. **encode_adr accepts :lo12:/:got: modifiers as a bare ADR reloc.** Law: llvm-mc rejects `adr x0, :lo12:foo` ("unexpected adr label"). Minimal input: `[Reg("x0"), Modifier{kind:"lo12", symbol:"foo"}]` → WordWithReloc AdrPrelLo21. Additional witness: ModifierOffset with offset=0. Severity: high. Serial reconfirm: PBT_TEST_JOBS=1. Report: `pbt-out/bug_reports/encode_adr_modifier.md`

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/load_store.rs (mod encode_adr_pbt) | 12 properties + 2 KAT + 6 regressions |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md (merged)
- pbt-out/INVARIANTS.md (appended)
- pbt-out/bug_reports/encode_adr_w_reg.md
- pbt-out/bug_reports/encode_adr_sp_as_zr.md
- pbt-out/bug_reports/encode_adr_imm_range.md
- pbt-out/bug_reports/encode_adr_fp_reg.md
- pbt-out/bug_reports/encode_adr_modifier.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 11:47 (campaign: coverage)
> Files: 4/4 scanned (100%) | Functions: 5/96 total | PBT candidates: 5 | Tested: 5 (100%) | 0 pass, 5 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 4 |
| Files scanned | 4 / 4 (100%) |
| Total functions (all files) | 96 |
| PBT candidates (from FUNCTION_INDEX) | 5 |
| **Tested (of PBT candidates)** | **5 / 5 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 5 / 0 |
| **Overall (tested / all functions)** | **5 / 96 (5%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 5 | 5 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 5 | 5 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 2 | 2 | 100% | covered |
| load_store.rs | 20 | 1 | 1 | 100% | covered |

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
