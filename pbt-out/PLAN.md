# PBT Campaign: classify_cast_with_f128

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Module + function docstring at `src/backend/cast.rs:1-7` and `:57-66`: shared decision logic for all four backends; Ptr normalization (Ptr treated as U64) and F128 reduction (F128 treated as F64 on x86) happen before classification; `f128_is_native` is true where F128 is IEEE binary128 (softfloat libcalls) and false where F128 is x87 80-bit approximated as F64.
  - `CastKind` variant docs at `src/backend/cast.rs:16-54` (Noop = same type or Ptr <-> I64/U64 or F128 <-> F64; native F128 variants; float/int/widen/narrow kinds).
  - Backend README `src/backend/README.md:644-670`: pointer normalization Ptr as U64; x86 F128 approximated as F64; `f128_is_native` distinguishes ARM/RISC-V binary128 from x86 x87.
  - Ptr width: `src/common/types.rs:20-21` and `src/backend/cast.rs:88-89` (Ptr ≡ U64 on LP64, U32 on ILP32).
  - Caller: i686 uses `classify_cast_with_f128(..., true)` (`src/backend/i686/codegen/casts.rs:32-37`); ARM/RISC-V `classify_cast()` is the `false` wrapper (`src/backend/arm/codegen/cast_ops.rs:10`, `src/backend/cast.rs:151-154`) and asserts F128 libcall kinds are unreachable (`src/backend/arm/codegen/cast_ops.rs:128`).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Examples: `src/backend/asm_expr.rs`, `src/ir/constants.rs` (`mod cast_float_to_target_pbt`), `src/backend/arm/assembler/encoder/data_processing.rs`. No crate-root `tests/` directory. No existing tests in `src/backend/cast.rs`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`).
- **Buildability probe:** `cargo test --lib -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: FAILED. 505 passed; 11 failed; 6 ignored`. All 11 failures are pre-existing from prior campaigns (`encode_add_sub_pbt` in `data_processing.rs` and `cast_float_to_target_pbt` in `constants.rs`), unrelated to this target. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending `#[cfg(test)] mod classify_cast_with_f128_pbt` at the bottom of `src/backend/cast.rs` (inline layout; no test block exists yet). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs.
- **Candidate modules:** classify_cast_with_f128 (cast.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in cast.rs are indexed but not tested.

## Module: classify_cast_with_f128
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
