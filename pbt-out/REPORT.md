# PBT Campaign Report: classify_cast_with_f128

## Summary

**Date:** 2026-09-11
**Repository:** /home/toan/github/claudes-c-compiler
**Modules tested:** classify_cast_with_f128 (src/backend/cast.rs)
**Tests:** 11 properties + 3 regression witnesses
**Result:** 8 passing, 3 failing properties (1 bug), 3 failing regression witnesses
**Effort tier:** standard (5–8 properties/target, ≥1000 cases, 1 strengthen/sweep round, ≥1 metamorphic)

## Modules Tested

| Module | Tests | Bugs | Oracles Used |
|--------|-------|------|-------------|
| classify_cast_with_f128 | 11 properties (8 pass / 3 fail) + 3 regression | 1 | algebraic.invariant, algebraic.metamorphic |

## Bugs Found

**Ptr is not treated as U64/U32 for float and F128 casts.**

- **Laws violated:** Ptr normalization happens before classification (Ptr ≡ U64 on LP64, U32 on ILP32); pointer-to-float must be unsigned; float-to-pointer on ILP32 must use the 32-bit unsigned path.
- **Failing properties:** `classify_cast_ptr_normalized_as_unsigned_int`, `classify_cast_native_f128_int_signedness`, `classify_cast_float_int_kinds`
- **Shrunk counterexamples (serial reconfirm, PBT_TEST_JOBS=1):**
  - `classify_cast_with_f128(Ptr, F32, false)` at ptr_size=4 → `SignedToFloat { to_f64: false, from_ty: Ptr }` (expected `UnsignedToFloat { to_f64: false, from_ty: U32 }`)
  - `classify_cast_with_f128(Ptr, F128, true)` at ptr_size=4 → `SignedToF128 { from_ty: Ptr }` (expected `UnsignedToF128 { from_ty: U32 }`)
  - `classify_cast_with_f128(F32, Ptr, false)` at ptr_size=4 → `FloatToUnsigned { from_f64: false, to_u64: true }` (expected `to_u64: false`)
- **Root cause:** F128 handling and the float↔int arms run before (and skip) the Ptr-normalization block, which is gated on `!from_ty.is_float() && !to_ty.is_float()`.
- **Impact:** High-bit-set pointers convert to negative floats via signed x87 `fild`; ILP32 float-to-ptr takes the 64-bit conversion path (i686 `emit_f32_to_i64` stores 8 bytes into a 4-byte pointer slot).
- **Severity:** high
- **Bug report:** pbt-out/bug_reports/classify_cast_ptr_not_normalized_for_float.md
- **Regression tests:** `test_classify_cast_with_f128_regression_ptr_to_f32_unsigned`, `test_classify_cast_with_f128_regression_ptr_to_f128_unsigned`, `test_classify_cast_with_f128_regression_f32_to_ptr_ilp32` in src/backend/cast.rs (all fail, as required while the bug remains)

## Design Caveats (if any)

(none)

## Test Files Created

| File | Tests |
|------|-------|
| src/backend/cast.rs (`mod classify_cast_with_f128_pbt`) | 11 properties + 3 regression witnesses |

## Output Directories

- pbt-out/PLAN.md — campaign checklist
- pbt-out/PROPERTIES.md — property ledger
- pbt-out/FUNCTION_INDEX.md — merged function index (cast.rs added)
- pbt-out/COVERAGE.md — coverage ledger
- pbt-out/COVERAGE_STATUS.md — coverage summary
- pbt-out/INVARIANTS.md — confirmed invariants
- pbt-out/REPORT.md — this report
- pbt-out/bug_reports/classify_cast_ptr_not_normalized_for_float.md

## Contract-surface sweep

Exactly 1 round (standard tier). `coverage_gaps` had no LLVM profraw in this session (`RUSTFLAGS`/`LLVM_PROFILE_FILE` unset); sweep was a manual arm audit of `classify_cast_with_f128` / `classify_f128_cast_native`. Added three documented-branch properties, all passing: F32↔F64 `widen` flag, non-native never emits F128 libcall kinds, Ptr ↔ pointer-width integer is Noop. Closed because the tier's one round is done.

## Build / harness

- Build contract: `cargo check --lib` (prebuilt; log pbt-out/build.log)
- Harness: rung 1 — `cargo test --lib`, inline `#[cfg(test)]` in src/backend/cast.rs, existing `proptest` dev-dependency, 1000 cases
- Probe: `cargo test --lib` → 505 passed, 11 failed, 6 ignored (pre-existing encode_add_sub / cast_float_to_target failures only)
- No target skipped

## Coverage Report

# PBT Coverage Status

> Last updated: 2026-09-11 11:06 (campaign: coverage)
> Files: 3/3 scanned (100%) | Functions: 3/76 total | PBT candidates: 3 | Tested: 3 (100%) | 0 pass, 3 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 3 |
| Files scanned | 3 / 3 (100%) |
| Total functions (all files) | 76 |
| PBT candidates (from FUNCTION_INDEX) | 3 |
| **Tested (of PBT candidates)** | **3 / 3 (100%)** |
| &nbsp;&nbsp;↳ Pass / Fail / Other | 0 / 3 / 0 |
| **Overall (tested / all functions)** | **3 / 76 (4%)** |
| Untested | 0 |
| Skipped | 0 |

## Module Breakdown

| Module | Scanned | Tested | Skipped | Coverage |
|--------|---------|--------|---------|----------|
|  | 3 | 3 | 0 | 100% |

## Oracle Type Distribution

| Oracle Type | Total | Covered | Skipped | Coverage |
|-------------|-------|---------|---------|----------|
| unknown | 3 | 3 | 0 | 100% |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| cast.rs | 6 | 1 | 1 | 100% | covered |
| constants.rs | 34 | 1 | 1 | 100% | covered |
| data_processing.rs | 36 | 1 | 1 | 100% | covered |

## Recommended Focus

> **Priority 1 — Fix failing tests**
> These functions have failing PBT properties — fix before adding new tests.

| Function | Source |
|----------|--------|
| encode_add_sub | data_processing.rs |
| cast_float_to_target | constants.rs |
| classify_cast_with_f128 | cast.rs |
