# PBT Campaign: encode_neon_tbl

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; always 4 bytes little-endian; subset emitted by codegen.
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: translates GNU-style assembly as GCC's gas would consume into ELF `.o`; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:233`: lists `tbl`, `tbx` under NEON permute.
  - Body docstring at `neon.rs:767-771`: "Encode NEON TBL: table vector lookup"; forms `TBL Vd.T, {Vn.T}, Vm.T` (1-reg table) through multi-register lists. Encoding comment `neon.rs:792-796`: `0 Q 00 1110 000 Rm 0 len 0 00 Rn Rd`; `len` 1 reg -> 00, 2 -> 01, 3 -> 10, 4 -> 11.
  - ARM ARM Advanced SIMD table lookup TBL: `0 Q 00 1110 00 0 Rm 0 len op 00 Rn Rd` with op=0. Ta in {8B,16B}. Table registers are `.16B`, 1 to 4 consecutive (wrapping at 31). Vm.Ta must match Vd.Ta. Q=1 iff Ta=16B.
  - llvm-mc `-triple=aarch64 -show-encoding` confirms encodings; rejects Ta not in {8b,16b}, table not `.16b`, non-sequential lists, 5+ vectors, extra operand, GPR dest, too few operands, missing register list, mismatched Vd/Vm T.
  - Dispatch `encoder/mod.rs:729`: `"tbl" => encode_neon_tbl(operands)`. Sibling `encode_neon_tbx` is TBX (op=1), different job.
  - Parser `parser.rs:2030-2072` builds `Operand::RegList` of `RegArrangement`; rejects empty lists. Range syntax `vN.16b-vM.16b` expands consecutive wrapping registers.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `neon.rs` already has sibling PBT modules — those are prior campaign artifacts living in the project source; new work still extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1409 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neon_tbl_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_neon_tbl (neon.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in neon.rs are indexed but not tested.
- **Oracle (encode_neon_tbl):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid vector TBL with Ta in {8b,16b}, Vd/Vm in v0–v31, 1–4 consecutive wrapping table registers all `.16B`. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_neon_tbx fails the same-job gate (TBX / op=1). SUT-boundary: internal-helper of the AArch64 assembler; public contract is encoding GNU-style AArch64 TBL text / ARM ARM TBL. Mapping: `operands` <-> `tbl Vd.Ta, {Vn.16B, ...}, Vm.Ta`.
- **Seeds:** README.md:233 NEON permute table lists tbl. No existing unit test of encode_neon_tbl. Seed: (none for this symbol).
- **State machine:** not applicable — encode_neon_tbl is a pure function with no mutating operations or lifecycle.

## Module: encode_neon_tbl
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of list[0] Reg/Imm and Vm Operand::Reg — added encode_neon_tbl_neg_list_and_vm_kinds)
