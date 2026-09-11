# PBT Campaign: encode_adr

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:222`: Address category lists `adrp`, `adr`.
  - Assembler README `src/backend/arm/assembler/README.md:255`: `AdrPrelLo21` ELF 274 for `adr` (21-bit PC-relative).
  - `RelocType::AdrPrelLo21` comment `encoder/mod.rs:79-80`: R_AARCH64_ADR_PREL_LO21 - for ADR (21-bit PC-relative); elf_type 274 at `:103`.
  - Dispatch `encoder/mod.rs:373`: `"adr" => encode_adr(operands)`.
  - Function comment `load_store.rs:696-698`: immediate form `adr Rd, #imm`; TODO validate 21-bit signed immediate range; encoding `ADR: 0 immlo[1:0] 10000 immhi[18:0] Rd`.
  - ARM ARM ADR (PC-relative): `op=0 immlo 10000 immhi Rd`; 21-bit signed byte offset in [-1048576, 1048575]; Rd is Xd (register 31 is XZR, not SP).
  - Linker README `src/backend/arm/linker/README.md:402`: `R_AARCH64_ADR_PREL_LO21` | 274 | S + A - P | ADR instruction.
  - Callers: `encoder/mod.rs:373` only.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Existing PBT modules live in `data_processing.rs` (`encode_add_sub_pbt`, `encode_adc_pbt`). `load_store.rs` has no `#[cfg(test)]` block yet. No crate-root `tests/` directory. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_adc_kat_llvm_mc_adc_x0_x1_x2 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 560 filtered out`. Rung 1 available; project cargo harness builds and runs. (`cargo test --lib -- --list` reports 561 tests.)
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_adr_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (inline layout; file has no existing test module). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery.
- **Candidate modules:** encode_adr (load_store.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in load_store.rs are indexed but not tested.
- **Oracle (encode_adr):** Differential (llvm-mc -triple=aarch64 -show-encoding). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree ADR decoder rejected: none exists. Linker `reloc::encode_adr` rejected as differential sibling (different job: patches imm into an already-encoded word). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(Xd), Imm(imm)]` <-> `adr Xd, #imm`.

## Module: encode_adr
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: get_symbol Reg/Cond/Barrier + ModifierOffset; coverage_gaps had no profraw)
