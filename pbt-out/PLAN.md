# PBT Campaign: encode_bic

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:224`: NEON three-same lists `bic`.
  - Assembler README `src/backend/arm/assembler/README.md:506`: `data_processing.rs` covers ORN/EON/BICS/BIC.
  - Dispatch `encoder/mod.rs:674`: `"bic" => encode_bic(operands)`.
  - Function comment `data_processing.rs:1008-1012`: Scalar register `BIC Xd, Xn, Xm [, shift #amount]` (AND NOT, opc=00, N=1); scalar immediate `BIC Xd, Xn, #imm` -> AND `#~imm`; NEON `BIC Vd.T, Vn.T, Vm.T`.
  - ARM ARM BIC (shifted register): `sf 00 01010 shift 1 Rm imm6 Rn Rd`; Rd/Rn/Rm of 31 are XZR/WZR not SP; shift amount [0,31] (W) / [0,63] (X); shifts lsl/lsr/asr/ror.
  - ARM ARM BIC (immediate) is an alias of AND (immediate): `sf 00 100100 N immr imms Rn Rd` with inverted bitmask; Rd of 31 is SP/WSP not XZR; Rn of 31 is XZR.
  - ARM ARM BIC (vector, three-same): `0 Q 0 01110 01 1 Rm 000111 Rn Rd`; T is 8B or 16B only.
  - Callers: `encoder/mod.rs:674` only.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Existing PBT modules live in `data_processing.rs` (`encode_add_sub_pbt`, `encode_adc_pbt`). Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory.
- **Buildability probe:** `cargo test --lib encode_adc_kat_llvm_mc_adc_x0_x1_x2 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 580 filtered out`. Rung 1 available; project cargo harness builds and runs. (`cargo test --lib -- --list` reports 581 tests.)
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_bic_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; file already has sibling PBT modules). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery.
- **Candidate modules:** encode_bic (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_bic):** Differential (llvm-mc -triple=aarch64 -show-encoding). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree BIC decoder rejected: none exists. encode_logical (AND) is a different mnemonic / different N-bit job and fails the same-job sibling gate as a differential reference (kept as a metamorphic alias check for the documented BIC-imm = AND-~imm relation). encode_neon_bic is the helper encode_bic calls, not independent. SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(Rd), Reg(Rn), Reg(Rm), Shift?]` <-> `bic Rd, Rn, Rm {, shift #amt}`; `[Reg(Rd), Reg(Rn), Imm(imm)]` <-> `bic Rd, Rn, #imm`; `[RegArrangement, ...]` <-> `bic Vd.T, Vn.T, Vm.T`.

## Module: encode_bic
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_bic error paths — invalid bitmask, unsupported third operand, invalid rm, unknown shift kind)
