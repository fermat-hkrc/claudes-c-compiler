# PBT Campaign Report: encode_movn

## Summary

**Date:** 2026-09-14
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** encode_movn
**Tests:** 12 properties + 1 KAT + 5 regression witnesses
**Result:** 7 passing, 5 bugs
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 coverage-driven sweep round)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| encode_movn | 12 properties (7 passing, 5 failing) + 1 KAT + 5 regressions | 5 | differential, algebraic.metamorphic, algebraic.invariant, negative_error |

## Bugs Found

### 1. encode_movn silently truncates immediates outside [0, 65535]
- **Law:** MOVN imm16 must be in [0, 65535]; out-of-range values must be rejected.
- **Shrunk counterexample:** `movn w0, #-1` (`rd=0, is_64=false, imm=-1`)
- **Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 65535]")
- **Actual:** Ok(Word) with imm16=0xFFFF (`(imm as u32) & 0xFFFF`)
- **Root cause:** encode_movn masks the immediate instead of range-checking.
- **Impact:** Invalid GNU as / ARM ARM input is accepted and encodes a different inverted immediate.
- **Severity:** medium
- **Fix:** Reject imm not in [0, 65535] before encoding.
- **Bug report:** pbt-out/bug_reports/encode_movn_imm_oob.md
- **Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1

### 2. encode_movn accepts non-lsl shifts and illegal shift amounts
- **Law:** Only `lsl` with {0,16} (W) or {0,16,32,48} (X) is valid.
- **Shrunk counterexample:** `movn w0, #0, lsr #0`
- **Expected:** Err (llvm-mc: "expected 'lsl' with optional integer 0 or 16")
- **Actual:** Ok(Word) with hw=0 (non-lsl defaults to 0)
- **Root cause:** `kind == "lsl"` else hw=0; lsl amount is `amount / 16` with no range check.
- **Impact:** Invalid assembly is encoded as a different MOVN; out-of-range hw can spill into opcode bits.
- **Severity:** medium
- **Fix:** Reject non-lsl kinds and amounts not in the documented set.
- **Bug report:** pbt-out/bug_reports/encode_movn_invalid_shift.md
- **Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1

### 3. encode_movn ignores extra operands
- **Law:** MOVN is `Rd, #imm16 [, lsl #N]`; a further operand must be rejected.
- **Shrunk counterexample:** `movn x0, #0, x0`
- **Expected:** Err (llvm-mc: "invalid operand for instruction")
- **Actual:** Ok(Word) encoding `movn x0, #0`
- **Root cause:** Operands beyond index 2 are ignored; a non-Shift at index 2 sets hw=0.
- **Impact:** Typos assemble instead of failing.
- **Severity:** medium
- **Fix:** Reject operands.len() > 2 when operand 2 is not a valid lsl, and reject len > 3 always.
- **Bug report:** pbt-out/bug_reports/encode_movn_extra_operand.md
- **Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1

### 4. encode_movn encodes SP/WSP as XZR/WZR
- **Law:** Rd=31 is XZR/WZR, never SP/WSP.
- **Shrunk counterexample:** `movn wsp, #0`
- **Expected:** Err (llvm-mc: "invalid operand for instruction")
- **Actual:** Ok(Word) with Rd=31 (WZR)
- **Root cause:** parse_reg_num maps "sp"/"wsp" to 31; encode_movn does not distinguish SP from ZR.
- **Impact:** Stack-pointer names silently target the zero register.
- **Severity:** medium
- **Fix:** Reject sp/wsp as MOVN Rd.
- **Bug report:** pbt-out/bug_reports/encode_movn_sp.md
- **Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1

### 5. encode_movn encodes FP/SIMD register names as GPRs
- **Law:** MOVN operands are GPRs; FP/SIMD names (d/s/q/v/h/b) must be rejected.
- **Shrunk counterexample:** `movn d0, #0`
- **Expected:** Err (llvm-mc: "invalid operand for instruction")
- **Actual:** Ok(Word) encoding 32-bit MOVN with Rd=0
- **Root cause:** parse_reg_num accepts d/s/q/v/h/b prefixes; is_64bit_reg is false for `d0`.
- **Impact:** SIMD names silently retarget a GPR.
- **Severity:** medium
- **Fix:** Reject FP/SIMD names (use is_fp_reg or restrict parse_reg_num callers).
- **Bug report:** pbt-out/bug_reports/encode_movn_fp_as_gpr.md
- **Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1

## Design Caveats

- `:abs_g*:` modifiers are documented for movz/movk only (`Doc evidence: data_processing.rs:179-181` — "Resolve `:abs_g0:`, `:abs_g1:`, etc. modifiers for movz/movk."). encode_movn has no Modifier path; `get_imm` returns Err. llvm-mc accepts `movn x0, :abs_g0:sym` as a relocation. Not filed as a bug: the in-tree comment asserts the helper is for movz/movk, and codegen emit.rs:873-902 never emits abs_g with movn.
- Contract-surface sweep closed after 1 round (standard tier). `coverage_gaps` had no LLVM profraw; sweep was a manual arm audit of get_reg / get_imm / Shift / extra / too-few / FP / invalid name. Documented error paths now have properties.

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/arm/assembler/encoder/data_processing.rs (mod encode_movn_pbt) | 12 properties + 1 KAT + 5 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign phases
- pbt-out/PROPERTIES.md — property ledger (encode_movn appended)
- pbt-out/REPORT.md — this report
- pbt-out/COVERAGE.md — coverage ledger row for encode_movn
- pbt-out/INVARIANTS.md — confirmed encode_movn invariants
- pbt-out/FUNCTION_INDEX.md — encode_movn reclassified as PBT candidate
- pbt-out/bug_reports/encode_movn_imm_oob.md
- pbt-out/bug_reports/encode_movn_invalid_shift.md
- pbt-out/bug_reports/encode_movn_extra_operand.md
- pbt-out/bug_reports/encode_movn_sp.md
- pbt-out/bug_reports/encode_movn_fp_as_gpr.md

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-14 08:29 (campaign: coverage)
> Files: 6/6 scanned (100%) | Functions: 39/184 total | PBT candidates: 39 | Tested: 39 (100%) | 0 pass, 39 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 6 |
| Files scanned | 6 / 6 (100%) |
| Total functions (all files) | 184 |
| PBT candidates (from FUNCTION_INDEX) | 39 |
| **Tested (of PBT candidates)** | **39 / 39 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 39 / 0 |
| **Overall (tested / all functions)** | **39 / 184 (21%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 39 | 39 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 39 | 39 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| compare_branch.rs | 21 | 17 | 17 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 10 | 10 | 100% | covered |
| load_store.rs | 20 | 5 | 5 | 100% | covered |
| neon.rs | 68 | 5 | 5 | 100% | covered |

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
