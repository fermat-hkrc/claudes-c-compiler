# PBT Campaign Report: encode_add_sub

## Summary

**Date:** 2026-09-11
**Repository:** claudes-c-compiler (/home/toan/github/claudes-c-compiler)
**Modules tested:** encode_add_sub (src/backend/arm/assembler/encoder/data_processing.rs)
**Tests:** 14 properties (plus KAT + 7 regression witnesses)
**Result:** 7 passing, 7 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, ≥1 metamorphic/differential, 1 contract-surface sweep)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_add_sub | 14 properties (7 passing, 7 failing) | 7 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### encode_add_sub_diff_extended_and_sp
- **Law:** SP/WSP as Rn with LSL #N (N≤4) must use extended-register encoding (bit 21=1) so register 31 is SP, not XZR.
- **Shrunk counterexample:** rd=0, rn=0, rm=0, is_64=false, is_sub=false, set_flags=false, use_lsl_alias=true → asm=`add w0, wsp, w0, lsl #1`
- **Expected:** Word(0x0b2047e0) (llvm-mc)
- **Actual:** Word(0x0b0007e0)
- **Severity:** high
- **Report:** pbt-out/bug_reports/encode_add_sub_sp_lsl_shifted_form.md

### encode_add_sub_neg_imm_out_of_range
- **Law:** Explicit `lsl #12` requires the unshifted immediate in 0..=4095; overflow must Err, not mask.
- **Shrunk counterexample:** rd=0, rn=0, is_64=false, is_sub=false, set_flags=false, imm=4097, explicit_lsl12=true
- **Expected:** Err
- **Actual:** Ok(Word) via `(imm_val as u32) & 0xFFF`
- **Severity:** medium
- **Report:** pbt-out/bug_reports/encode_add_sub_imm12_lsl12_mask.md

### encode_add_sub_neg_invalid_shift_extend
- **Law:** ADD/SUB shifted-register allows only LSL/LSR/ASR; ROR must Err.
- **Shrunk counterexample:** rd=0, rn=0, rm=0, is_64=false, is_sub=false, set_flags=false, class=0, extra=0 (ROR #0)
- **Expected:** Err
- **Actual:** Ok(Word) — unknown shift kind defaults to LSL
- **Severity:** medium
- **Report:** pbt-out/bug_reports/encode_add_sub_ror_accepted.md

### encode_add_sub_neg_imm_bad_shift
- **Law:** ADD/SUB immediate form allows only LSL #0 or LSL #12 after #imm; lsr/asr/ror must Err.
- **Shrunk counterexample:** rd=0, rn=0, is_64=false, is_sub=false, set_flags=false, imm=0, class=0, amt=0 (lsr #0)
- **Expected:** Err
- **Actual:** Ok(Word) — non-lsl#12 Shift on immediate form is ignored
- **Severity:** medium
- **Report:** pbt-out/bug_reports/encode_add_sub_imm_bad_shift_ignored.md

### encode_add_sub_neg_mixed_width
- **Law:** Immediate and shifted-register ADD/SUB require all registers the same width.
- **Shrunk counterexample:** rd=0, rn=0, rm=0, rd64=true, rn64=false, rm64=false, is_sub=false, set_flags=false, use_imm=false → ops=[x0, w0, w0]
- **Expected:** Err
- **Actual:** Ok(Word) — sf taken only from Rd
- **Severity:** medium
- **Report:** pbt-out/bug_reports/encode_add_sub_mixed_width.md

### encode_add_sub_neg_fp_reg
- **Law:** GPR ADD/SUB operands are W/X registers; FP/SIMD names must Err.
- **Shrunk counterexample:** which=0, is_sub=false, set_flags=false, prefix="d", n=0 → ops=[d0, x1, x2]
- **Expected:** Err
- **Actual:** Ok(Word) — parse_reg_num maps d0→0 as W0
- **Severity:** medium
- **Report:** pbt-out/bug_reports/encode_add_sub_fp_reg.md

### encode_add_sub_neg_adds_sp_rd
- **Law:** ADDS/SUBS Rd cannot be SP/WSP (that encoding is XZR/WZR = CMP/CMN).
- **Shrunk counterexample:** is_64=false, is_sub=false, rn=0, imm=0 → ops=[wsp, w0, #0], set_flags=true
- **Expected:** Err
- **Actual:** Ok(Word)
- **Severity:** high
- **Report:** pbt-out/bug_reports/encode_add_sub_adds_sp_rd.md

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_add_sub_pbt) | 14 properties + KAT + 7 regression witnesses (extended existing module; prior 8 properties not rewritten) |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — 14-property ledger
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage statistics
- pbt-out/FUNCTION_INDEX.md — function index (merged, unchanged)
- pbt-out/INVARIANTS.md — confirmed invariants (encode_add_sub section updated)
- pbt-out/bug_reports/encode_add_sub_sp_lsl_shifted_form.md
- pbt-out/bug_reports/encode_add_sub_imm12_lsl12_mask.md
- pbt-out/bug_reports/encode_add_sub_ror_accepted.md
- pbt-out/bug_reports/encode_add_sub_imm_bad_shift_ignored.md
- pbt-out/bug_reports/encode_add_sub_mixed_width.md
- pbt-out/bug_reports/encode_add_sub_fp_reg.md
- pbt-out/bug_reports/encode_add_sub_adds_sp_rd.md

Contract-surface sweep: 1 round (standard tier). `coverage_gaps` had no LLVM profraw; sweep was a manual arm audit of untested documented paths (FP regs, ADDS Rd=SP, :lo12:/:tprel: modifiers). Close reason: tier's one sweep round completed.

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 11:32 (campaign: coverage)
> Files: 3/3 scanned (100%) | Functions: 4/76 total | PBT candidates: 4 | Tested: 4 (100%) | 0 pass, 4 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 3 |
| Files scanned | 3 / 3 (100%) |
| Total functions (all files) | 76 |
| PBT candidates (from FUNCTION_INDEX) | 4 |
| **Tested (of PBT candidates)** | **4 / 4 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 4 / 0 |
| **Overall (tested / all functions)** | **4 / 76 (5%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 4 | 4 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 4 | 4 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 2 | 2 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
| classify_cast_with_f128 | cast.rs |
| encode_adc | data_processing.rs |
