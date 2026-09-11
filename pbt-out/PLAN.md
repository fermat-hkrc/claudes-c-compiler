# PBT Campaign: encode_bics

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:214`: Data Processing lists `bics`.
  - Dispatch `encoder/mod.rs:237`: `"bics" => encode_bics(operands)`.
  - Function comment `data_processing.rs:980-1010`: BICS Rd, Rn, Rm [, shift #amount]: `sf 11 01010 shift 1 Rm imm6 Rn Rd`.
  - ARM ARM Logical (shifted register) BICS: opc=11, N=1; shift in {LSL,LSR,ASR,ROR}; imm6 in [0,31] (W) / [0,63] (X); register 31 is XZR/WZR, never SP/WSP. GNU/llvm-mc also accept `bics Rd, Rn, #imm` as an alias of `ands Rd, Rn, #~imm` when ~imm is a valid bitmask.
  - Callers: `encoder/mod.rs:237` only.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Existing PBT modules live in `data_processing.rs` (`encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`), `load_store.rs`, and `neon.rs`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory.
- **Buildability probe:** `cargo test --lib encode_bic_kat_llvm_mc_bic_x0_x1_x2 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 622 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_bics_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling PBT modules already live in this file). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery.
- **Candidate modules:** encode_bics (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_bics):** Differential (llvm-mc -triple=aarch64 -show-encoding). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree BICS decoder rejected: none exists. encode_bic is a related encoding sibling (opc=00 vs 11) but a different job (does not set flags) so it fails the same-job sibling gate as a differential reference; it is used only as a metamorphic transform (opc-field XOR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(Rd), Reg(Rn), Reg(Rm){, Shift}]` <-> `bics Rd, Rn, Rm{, shift #amt}` and `[Reg(Rd), Reg(Rn), Imm]` <-> `bics Rd, Rn, #imm` (llvm-mc alias of ANDS #~imm).

## Module: encode_bics
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_bics error paths — invalid Rm names passing; extra 4th operand filed as a bug)
