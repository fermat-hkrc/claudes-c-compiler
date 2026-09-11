# PBT Campaign Report: IrConst::cast_float_to_target

## Summary

**Date:** 2026-09-11
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** cast_float_to_target (src/ir/constants.rs)
**Tests:** 9 properties (1000 cases each) + 2 deterministic regression witnesses
**Result:** 6 passing, 3 failing (2 distinct SUT bugs)
**Effort tier:** standard
**Contract-surface sweep:** 1 round (manual arm audit; `coverage_gaps` had no instrumented profraw). Closed after adding Ptr-vs-from_i64 (passing).

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| cast_float_to_target | 9 properties + 2 regressions | 2 | differential, algebraic.invariant, algebraic.metamorphic, negative_error |

## Bugs Found

### 1. Unsigned U8/U16 stored as I8/I16 so to_i64() sign-extends

- **Laws:** Differential vs `from_i64` (same-job float-to-int constructor); algebraic invariant that `to_i64()` of a U8/U16 result equals the unsigned truncated value.
- **Shrunk input:** `cast_float_to_target(128.0, IrType::U8)` → `I8(-128)`, `to_i64() = Some(-128)` (expected `I64(128)` / `Some(128)`). Also `0.0`/`U8` → `I8(0)` vs `from_i64(0, U8) = I64(0)`.
- **Expected vs actual:** Docstring `200.0 as u8 = 200`; `from_i64` stores U8/U16/U32 as I64 zero-extended. Actual U8 arm is `IrConst::I8(fv as u8 as i8)`.
- **Root cause:** U8/U16 arms use the signed storage variants after an unsigned `as` conversion. High bit set → `to_i64()` sign-extends. U32 already uses I64 and is consistent.
- **Impact:** simplify/const_eval/coerce_to fold `(unsigned char)128.0` to a negative i64, disagreeing with constant_fold's `from_i64` path.
- **Severity:** medium
- **Serial reconfirmation:** yes (`PBT_TEST_JOBS=1`)
- **Bug report:** pbt-out/bug_reports/cast_float_to_target_unsigned_storage.md
- **Regression:** `test_cast_float_to_target_regression_u8_high_bit` (fails as witness)

### 2. F128 target panics on f64 subnormals

- **Law:** `cast_float_to_target(fv, F128)` returns `Some(LongDouble(fv, bytes))` for every f64, including subnormals.
- **Shrunk input:** bits `9223372036854775809` (`0x8000000000000001`, negative subnormal).
- **Expected vs actual:** Some(LongDouble) with matching approx bits. Actual: debug panic `attempt to subtract with overflow` at `src/common/long_double.rs:1040` (`biased_exp as u128 - 1023` with biased_exp=0). Release wrap would emit garbage exponent bits.
- **Root cause:** `f64_to_f128_bytes_lossless` handles zero and inf/NaN but falls into the normal path for subnormals.
- **Severity:** high
- **Serial reconfirmation:** yes (`PBT_TEST_JOBS=1`)
- **Bug report:** pbt-out/bug_reports/cast_float_to_target_f128_subnormal.md
- **Regression:** `test_cast_float_to_target_regression_f128_neg_subnormal` (fails as witness)

## Design Caveats

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/ir/constants.rs (mod cast_float_to_target_pbt) | 9 properties + 2 regression witnesses |

## Output Directories

- pbt-out/PLAN.md
- pbt-out/PROPERTIES.md
- pbt-out/REPORT.md
- pbt-out/COVERAGE.md
- pbt-out/COVERAGE_STATUS.md
- pbt-out/FUNCTION_INDEX.md
- pbt-out/INVARIANTS.md
- pbt-out/bug_reports/cast_float_to_target_unsigned_storage.md
- pbt-out/bug_reports/cast_float_to_target_f128_subnormal.md
- pbt-out/code-coverage/ (no profraw this session)
- proptest-regressions/ir/constants.txt (framework shrunk-failure cache)

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 10:50 (campaign: coverage)
> Files: 2/2 scanned (100%) | Functions: 2/70 total | PBT candidates: 2 | Tested: 2 (100%) | 0 pass, 2 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 2 |
| Files scanned | 2 / 2 (100%) |
| Total functions (all files) | 70 |
| PBT candidates (from FUNCTION_INDEX) | 2 |
| **Tested (of PBT candidates)** | **2 / 2 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 2 / 0 |
| **Overall (tested / all functions)** | **2 / 70 (3%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 2 | 2 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 2 | 2 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
