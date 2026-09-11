# PBT Campaign: IrConst::cast_float_to_target

## Scan findings
- **Spec:** (none found for cast_float_to_target specifically). No requirement/dev-task document. In-tree contract: method docstring at src/ir/constants.rs:274-276 ("Cast a float value (as f64) to the target IR type"; unsigned targets convert via the unsigned type first; example `200.0 as u8 = 200`, not saturated to i8 max). Sibling constructor `from_i64` (src/ir/constants.rs:448-451) documents the IR storage convention: unsigned sub-64-bit types (U8, U16, U32) are stored as I64 with zero-extended values so `to_i64()` does not sign-extend. Same-job float-to-int folding: `src/passes/constant_fold.rs:606` uses `from_i64(val as i64, to_ty)`; `src/passes/simplify.rs:407` uses `cast_float_to_target`. TODO at `src/passes/constant_fold.rs:583-584` says the two paths should be unified. Seed: `src/passes/constant_fold.rs:1049` (`3.125` to I32 yields 3, truncation toward zero).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod tests` inside source files, discovered by `cargo test --lib`. Examples: `src/backend/asm_expr.rs`, `src/backend/asm_preprocess.rs`, `src/passes/constant_fold.rs`. No crate-root `tests/` directory. No existing tests in `src/ir/constants.rs`. Filename convention: test functions named `test_*` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry.
- **Buildability probe:** `cargo test --lib -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: FAILED. 499 passed; 6 failed; 6 ignored`. All 6 failures are pre-existing `encode_add_sub_pbt` cases from a prior campaign (`data_processing.rs`), unrelated to this target. Re-probe excluding those: `cargo test --lib -- --skip encode_add_sub` → `test result: ok. 493 passed; 0 failed; 6 ignored; 12 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending `#[cfg(test)] mod cast_float_to_target_pbt` at the bottom of `src/ir/constants.rs` (inline layout; no test block exists yet). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs.
- **Candidate modules:** cast_float_to_target (constants.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in constants.rs are indexed but not tested.

## Module: cast_float_to_target
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
