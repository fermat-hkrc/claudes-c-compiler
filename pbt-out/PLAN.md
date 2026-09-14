# PBT Campaign: encode_negs

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; always 4 bytes little-endian; subset emitted by codegen.
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: translates GNU-style assembly as GCC's gas would consume into ELF `.o`; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:214`: lists `neg`, `negs` under Data Processing.
  - DESIGN_DOC.md:338: AArch64 | ARM assembly syntax | Fixed 32-bit encoding.
  - Dispatch `encoder/mod.rs:279`: `"negs" => encode_negs(operands)`. Sibling `"neg"` is SUB (S=0), different job (does not set flags); NEG also has a NEON two-misc path that NEGS does not.
  - Body at `data_processing.rs:727-747`: comment `NEGS Rd, Rm [, shift #amount] -> SUBS Rd, XZR, Rm [, shift #amount]`. Encoding: sf at 31, op=1 at 30, S=1 at 29, 01011 at [28:24], shift at [23:22], Rm at [20:16], imm6=(amount&0x3F) at [15:10], Rn=31 at [9:5], Rd at [4:0]. Shift kinds: lsl/lsr/asr; unknown defaults to LSL. ROR is not in the match.
  - `get_reg` (`encoder/mod.rs:956-965`): `Operand::Reg` via `parse_reg_num` (x/w/d/s/q/v/h/b prefixes 0-31, sp/wsp/xzr/wzr=31, lr=30); `is_64bit_reg` is x*/sp/xzr/lr. No FP-reg rejection, no SP-vs-ZR distinction, no mixed-width check.
  - ARM ARM Add/subtract (shifted register) NEGS alias of SUBS: `sf 1 1 01011 shift 0 Rm imm6 11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd and Rm same width. Shift in {LSL,LSR,ASR} (not ROR). imm6 0..31 (sf=0) or 0..63 (sf=1). Exactly two registers plus optional shift.
  - llvm-mc `-triple=aarch64 -show-encoding` confirms encodings; `subs Rd, ZR, Rm` aliases to `negs`; rejects extra operand, too few operands, mixed width, SP, FP, ROR, out-of-range shift, unknown shift, extend.
  - Callers: no codegen emission of `negs` found (assembler table only). Public assembler contract still applies: GNU-style `negs` text.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has sibling PBT modules (`encode_mvn_pbt`, `encode_mul_pbt`, …) — those are prior campaign artifacts living in the project source; new work still extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1369 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_negs_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_negs (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_negs):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid two-operand same-width integer-register NEGS with optional LSL/LSR/ASR. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_neg fails the same-job gate (SUB / S=0, and NEG has a NEON path). encode_add_sub(is_sub=true, set_flags=true) is the documented SUBS expansion but shares get_reg/sf_bit (independence disclosed; used only as ARM-field metamorphic via llvm-mc SUBS, not in-tree encode_add_sub). SUT-boundary: internal-helper of the AArch64 assembler; public contract is encoding GNU-style AArch64 text / ARM ARM NEGS alias. Mapping: `operands` <-> `negs Rd, Rm{, shift}`.
- **Seeds:** README.md:214 instruction table; encode_mvn_pbt is a sibling alias-of-shifted-register pattern (not a test of this symbol). No existing unit test of encode_negs. Seed: (none for this symbol).
- **State machine:** not applicable — encode_negs is a pure function with no mutating operations or lifecycle.

## Module: encode_negs
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg Reg/invalid/non-Reg + extra operand + SUBS alias vs llvm-mc — added encode_negs_neg_invalid_name and encode_negs_neg_non_reg)
