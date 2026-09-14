# PBT Campaign: encode_orn

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; always 4 bytes little-endian; subset emitted by codegen.
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: translates GNU-style assembly as GCC's gas would consume into ELF `.o`; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:214`: lists `orn` under Data Processing; `README.md:224` lists `orn` under NEON three-same; `README.md:506` lists ORN in `data_processing.rs`.
  - Body docstring at `data_processing.rs:909`: "Encode ORN (logical OR NOT): ORN Rd, Rn, Rm (scalar or vector)". Encoding comment `data_processing.rs:933`: `sf 01 01010 shift 1 Rm imm6 Rn Rd`. NEON comment `data_processing.rs:922`: `0 Q 0 01110 11 1 Rm 000111 Rn Rd`; Q=1 iff arrangement is `16b`.
  - ARM ARM Logical (shifted register) ORN: `sf 01 01010 shift N=1 Rm imm6 Rn Rd`. opc=01, N=1. Register 31 is XZR/WZR, never SP/WSP. Rd/Rn/Rm same width. shift in {LSL,LSR,ASR,ROR}. imm6 0..31 (sf=0) or 0..63 (sf=1).
  - ARM ARM Advanced SIMD three-same ORN: `0 Q 0 01110 11 1 Rm 00011 1 Rn Rd`. T in {8B,16B}. Q=1 iff T=16B.
  - GNU as / llvm-mc alias: `orn Rd, Rn, #imm` encodes as `orr Rd, Rn, #~imm` (logical immediate). Confirmed with llvm-mc `-triple=aarch64 -show-encoding`.
  - Documented MVN alias at `data_processing.rs:753`: `MVN Rd, Rm [, shift #amount] -> ORN Rd, XZR, Rm [, shift #amount]`.
  - Dispatch `encoder/mod.rs:235`: `"orn" => encode_orn(operands)`. Sibling `encode_eon` is EON (opc=10), different job. Sibling `encode_logical(opc=01)` is ORR (N=0), different job. Sibling `encode_mvn` is the XZR-Rn alias of this instruction (same-job on that subset).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has sibling PBT modules — those are prior campaign artifacts living in the project source; new work still extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1429 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_orn_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_orn (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_orn):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GPR ORN with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr and optional LSL/LSR/ASR/ROR in range; for valid vector ORN with T in {8b,16b}; and for GNU `orn Rd, Rn, #imm` = `orr Rd, Rn, #~imm`. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_eon fails the same-job gate (EON / opc=10). encode_logical(opc=01) fails the same-job gate (ORR / N=0) — used only as N-bit metamorphic companion. encode_mvn is same-job on the Rn=XZR alias subset (doc at data_processing.rs:753). SUT-boundary: internal-helper of the AArch64 assembler; public contract is encoding GNU-style AArch64 ORN text / ARM ARM ORN. Mapping: `operands` <-> `orn Rd, Rn, Rm{, shift}` / `orn Rd, Rn, #imm` / `orn Vd.T, Vn.T, Vm.T`.
- **Seeds:** README.md:214 Data Processing table lists orn; README.md:224 NEON three-same lists orn. No existing unit test of encode_orn. Seed: (none for this symbol).
- **State machine:** not applicable — encode_orn is a pure function with no mutating operations or lifecycle.

## Module: encode_orn
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of Imm / Shift / NEON T / extra / get_reg kinds — added sf, Q, neon mismatch, trailing-after-shift, non-reg kinds)
