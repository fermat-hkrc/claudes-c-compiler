# PBT Campaign: encode_adc

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:208-214`: `adc` / `adcs` listed under Data Processing mnemonics handled by `encode_instruction()`.
  - Dispatch `src/backend/arm/assembler/encoder/mod.rs:281-282`: `"adc" => encode_adc(operands, false)`, `"adcs" => encode_adc(operands, true)`.
  - ARM ARM ADC (register) form: `ADC{S} <Wd>, <Wn>, <Wm>` / `ADC{S} <Xd>, <Xn>, <Xm>`; encoding `sf 0 S 11010000 Rm 000000 Rn Rd`; register 31 is WZR/XZR not WSP/SP; no shift/extend form.
  - Callers: `src/backend/arm/codegen/i128_ops.rs:46` (`adc x1, x1, xzr`) and `:68` (`adc x1, x3, x5`).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Existing module in this file: `encode_add_sub_pbt` at `data_processing.rs:1069`. No crate-root `tests/` directory. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`).
- **Buildability probe:** `cargo test --lib -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: FAILED. 513 passed; 17 failed; 6 ignored`. All 17 failures are pre-existing from prior campaigns (`encode_add_sub_pbt`, `cast_float_to_target_pbt`, `classify_cast_with_f128_pbt`), unrelated to encode_adc. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending `#[cfg(test)] mod encode_adc_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; do not edit `encode_add_sub_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs.
- **Candidate modules:** encode_adc (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.

## Module: encode_adc
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: invalid names + FP regs)
