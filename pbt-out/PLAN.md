# PBT Campaign: encode_neon_shift_imm

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; always 4 bytes little-endian; subset emitted by codegen.
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: translates GNU-style assembly as GCC's gas would consume into ELF `.o`; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:228`: lists `sshr`, `ushr` under NEON shifts.
  - Body docstring at `neon.rs:372`: "Encode NEON USHR Vd.T, Vn.T, #shift (unsigned shift right immediate)". Encoding comment `neon.rs:383-400`: `0 Q 1 011110 immh:immb 00000 1 Rn Rd`; `immh:immb = (element_size * 2 - shift)`.
  - ARM ARM Advanced SIMD shift by immediate USHR: `0 Q U 011110 immh immb 00000 1 Rn Rd` with U=1. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 (1D) is Reserved. shift = (2*esize)-UInt(immh:immb) in [1, esize]. immh != 0000.
  - llvm-mc `-triple=aarch64 -show-encoding` confirms encodings; rejects shift 0 / esize+1, T=1d, mismatched T, extra operand, GPR dest/src, too few operands, bare V without arrangement.
  - Dispatch `encoder/mod.rs:658-659`: `"ushr" => encode_neon_ushr(operands)`, `"sshr" => encode_neon_sshr(operands)`. This symbol is not on the dispatch table (dead `pub(crate)` helper; module has `#![allow(dead_code)]`). Documented job remains USHR encoding.
  - `_is_unsigned` is unused (Rust `_` prefix); U is hardcoded to 1. Sibling `encode_neon_ushr` is a near-copy of this body (same-job, shared logic — not an independent differential). Sibling `encode_neon_sshr` is SSHR (U=0), different job.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `neon.rs` already has sibling PBT modules (`encode_neon_shift_right_pbt`, `encode_neon_sli_pbt`, …) — those are prior campaign artifacts living in the project source; new work still extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1394 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neon_shift_imm_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_neon_shift_imm (neon.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in neon.rs are indexed but not tested.
- **Oracle (encode_neon_shift_imm):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid vector USHR with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [1, esize]. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_neon_ushr fails the independence gate (near-copy of this body). encode_neon_sshr fails the same-job gate (SSHR / U=0). SUT-boundary: internal-helper of the AArch64 assembler; public contract is encoding GNU-style AArch64 USHR text / ARM ARM USHR. Mapping: `operands` + `_is_unsigned=true` <-> `ushr Vd.T, Vn.T, #shift`.
- **Seeds:** README.md:228 NEON shifts table; encode_neon_shift_right_pbt is a sibling SIMD-shift-immediate pattern (not a test of this symbol). No existing unit test of encode_neon_shift_imm. Seed: (none for this symbol).
- **State machine:** not applicable — encode_neon_shift_imm is a pure function with no mutating operations or lifecycle.

## Module: encode_neon_shift_imm
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_imm as u32 + get_neon_reg Reg source — added encode_neon_shift_imm_neg_shift_i64_trunc and encode_neon_shift_imm_neg_reg_source)
