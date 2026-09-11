# PBT Campaign: encode_add_sub

## Scan findings
- **Spec:** (none found for encode_add_sub specifically). Campaign contract is AArch64 ADD/SUB encoding. Evidence: encoder module docs (`src/backend/arm/assembler/encoder/mod.rs` "Encodes AArch64 instructions into 32-bit machine code words"); assembler README (`src/backend/arm/assembler/README.md`) "accepts the same textual assembly that GCC's gas would consume"; DESIGN_DOC.md table "AArch64 | ARM assembly syntax | Fixed 32-bit encoding | imm12 auto-shift".
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod tests` inside source files, discovered by `cargo test --lib`. Examples: `src/backend/asm_expr.rs`, `src/backend/asm_preprocess.rs`, `src/backend/arm/codegen/peephole.rs`. No crate-root `tests/` directory. No existing tests in `data_processing.rs`. Filename convention: test functions named `test_*` inside the source file's test module.
- **Buildability probe:** `cargo test --lib -- --test-threads=64` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 493 passed; 0 failed; 6 ignored; 0 measured; 0 filtered out; finished in 0.14s`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending `#[cfg(test)] mod encode_add_sub_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; no test block exists yet). Add `proptest` as a Cargo `[dev-dependencies]` entry. Not pbt-native: project cargo harness builds and runs.
- **Candidate modules:** encode_add_sub (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.

## Module: encode_add_sub
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
