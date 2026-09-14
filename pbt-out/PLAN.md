# PBT Campaign: encode_sbc

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; always 4 bytes little-endian; subset emitted by codegen.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: translates GNU-style assembly as GCC's gas would consume into ELF `.o`; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:214`: lists `adc`, `adcs`, `sbc`, `sbcs` under Data Processing.
  - Dispatch `encoder/mod.rs:283-284`: `"sbc" => encode_sbc(operands, false)`, `"sbcs" => encode_sbc(operands, true)`.
  - Body at `data_processing.rs:784-791`: `word = (sf << 31) | (1 << 30) | (s << 29) | (0b11010000 << 21) | (rm << 16) | (rn << 5) | rd`.
  - ARM ARM Add/subtract (with carry) SBC: `sf op=1 S 11010000 Rm 000000 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd/Rn/Rm same width. No shifted-register form. NGC Rd, Rm is the documented alias of SBC Rd, ZR, Rm; NGCS of SBCS Rd, ZR, Rm.
  - Codegen caller `src/backend/arm/codegen/i128_ops.rs:73`: emits `sbc x1, x3, x5` as the high half of 128-bit subtract.
  - Sibling `encode_adc` is ADC (op=0), different job. Used only as an op-bit metamorphic companion (XOR = 1<<30).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has sibling PBT modules (encode_adc_pbt and others) — those are prior campaign artifacts living in the project source; new work still extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1479 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_sbc_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_sbc (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_sbc):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid three-GPR same-width SBC/SBCS with Rd/Rn/Rm in {x0–x30, xzr} or {w0–w30, wzr}. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_adc fails the same-job gate (ADC / op=0) — used only as an op-bit metamorphic companion. SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style AArch64 SBC/SBCS text / ARM ARM SBC. Mapping: `[Reg(Rd), Reg(Rn), Reg(Rm)]` + set_flags <-> `sbc`/`sbcs` Rd, Rn, Rm. NGC alias: encode_sbc(Rd, ZR, Rm) <-> llvm-mc `ngc Rd, Rm`.
- **Seeds:** README.md:214 Data Processing table lists sbc/sbcs; i128_ops.rs:73 emits `sbc x1, x3, x5`. No existing unit test of encode_sbc. Seed: i128_ops.rs:73.
- **State machine:** not applicable — encode_sbc is a pure function with no mutating operations or lifecycle.

## Module: encode_sbc
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg success / None / other / extra / mixed / SP / FP / lr — added lr alias property and expanded non-register kinds)
