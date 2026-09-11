# PBT Campaign Report: encode_add_sub

## Summary

**Date:** 2026-09-11
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_add_sub (src/backend/arm/assembler/encoder/data_processing.rs)
**Tests:** 8 properties (plus 1 KAT and 3 deterministic regression witnesses)
**Result:** 5 passing, 3 bugs
**Effort tier:** standard

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_add_sub | 8 | 3 | differential, algebraic.metamorphic, negative_error |

## Bugs Found

### 1. SP/WSP + LSL #N encoded as shifted-register (XZR) instead of extended-register (SP)

- **Law:** When Rd or Rn is SP/WSP, ADD/SUB must use extended-register form; LSL #0..4 is the UXTX/UXTW alias.
- **Shrunk input:** `add w0, wsp, w0, lsl #1`
- **Expected:** `0x0b2047e0` (llvm-mc)
- **Actual:** `0x0b0007e0` (shifted form; Rn=31 means WZR)
- **Root cause:** SP special case is gated on `operands.len() <= 3`, so a following Shift operand skips it.
- **Impact:** stack-pointer operand is assembled as the zero register — high severity
- **Fix:** treat LSL #0..4 with SP/WSP as extended UXTW/UXTX, matching llvm-mc/gas
- **Bug report:** pbt-out/bug_reports/encode_add_sub_sp_lsl_shifted_form.md
- **Serial reconfirmation:** failed under `--test-threads=1`

### 2. Explicit `lsl #12` masks an overflow imm12 instead of rejecting it

- **Law:** unshifted imm12 must be in 0..=4095; llvm-mc rejects `#4097, lsl #12`
- **Shrunk input:** Imm(4097) + Shift { lsl, 12 } on `w0, w1`
- **Expected:** Err
- **Actual:** Ok (4097 & 0xFFF = 1 → `add w0, w1, #1, lsl #12`)
- **Root cause:** explicit-shift arm uses `((imm_val as u32) & 0xFFF, 1)` instead of a range check
- **Impact:** silent wrong immediate — medium
- **Bug report:** pbt-out/bug_reports/encode_add_sub_imm12_lsl12_mask.md
- **Serial reconfirmation:** failed under `--test-threads=1`

### 3. Invalid shift/extend (ROR) is encoded instead of rejected

- **Law:** ADD/SUB shifted-register allows only LSL/LSR/ASR; ROR is invalid; llvm-mc rejects it
- **Shrunk input:** `add w0, w1, w2, ror #0`
- **Expected:** Err
- **Actual:** Ok (unknown shift kind defaults to LSL via `_ => 0b00`)
- **Root cause:** catch-all shift mapping; related masking `& 0x3F` / `& 0x7` for out-of-range amounts
- **Impact:** invalid asm becomes a different valid instruction — medium
- **Bug report:** pbt-out/bug_reports/encode_add_sub_ror_accepted.md
- **Serial reconfirmation:** failed under `--test-threads=1`

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (#[cfg(test)] mod encode_add_sub_pbt) | 8 properties + KAT + 3 regressions |
| Cargo.toml [dev-dependencies] proptest 1.11.0 | harness |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/INVARIANTS.md
- pbt-out/REPORT.md
- pbt-out/bug_reports/encode_add_sub_sp_lsl_shifted_form.md
- pbt-out/bug_reports/encode_add_sub_imm12_lsl12_mask.md
- pbt-out/bug_reports/encode_add_sub_ror_accepted.md
- pbt-out/build.log (pre-campaign user build)
- pbt-out/code-coverage/ (no rust profraw; coverage_gaps had no instrumented data)

## Contract-surface sweep

Exactly 1 round as required by the standard tier. `coverage_gaps` was called after the first full test run and reported no instrumented coverage data (no LLVM profraw / gcda under the campaign workspace). Sweep closed on that result; documented reloc-modifier arms (`:lo12:`, `:tprel_*`) remain unpropertied and are listed in COVERAGE_STATUS.md.

## Notes

- Harness: rung 1 — inline `#[cfg(test)]` in data_processing.rs, run via `cargo test --lib`.
- Buildability probe: `cargo test --lib -- --test-threads=64` → 493 passed before new tests.
- One test-bug was fixed during Test: immediate-form generator used XZR as Rn=31; llvm-mc rejects that syntax (register 31 is SP in immediate form). Not a SUT bug; generator corrected. The three remaining failures reproduced serially.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 10:07 (campaign: coverage)
> Files: 1/1 scanned (100%) | Functions: 1/36 total | PBT candidates: 1 | Tested: 1 (100%) | 0 pass, 1 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 1 |
| Files scanned | 1 / 1 (100%) |
| Total functions (all files) | 36 |
| PBT candidates (from FUNCTION_INDEX) | 1 |
| **Tested (of PBT candidates)** | **1 / 1 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 1 / 0 |
| **Overall (tested / all functions)** | **1 / 36 (3%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 1 | 1 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 1 | 1 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| data_processing.rs | 36 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_add_sub | data_processing.rs |
