# PBT Campaign: encode_sxth

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: built-in AArch64 assembler translates GNU-style assembly text; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:217`: Extensions lists `sxth` (alongside `sxtw`, `sxtb`, `uxtw`, `uxth`, `uxtb`).
  - Encoder module docstring `src/backend/arm/assembler/encoder/mod.rs:1-7`: encodes AArch64 instructions into 32-bit machine code words.
  - Dispatch `encoder/mod.rs:294`: `"sxth" => encode_sxth(operands)`.
  - Body at `data_processing.rs:863-869`: reads Rd (and its is_64) and Rn via `get_reg`; `sf`/`N` follow dest width; `immr=0`, `imms=15`; extra operands beyond index 1 are ignored.
  - ARM ARM C6 SXTH (alias of SBFM): `SXTH <Wd>, <Wn>` = `SBFM <Wd>, <Wn>, #0, #15`; `SXTH <Xd>, <Wn>` = `SBFM <Xd>, <Xn>, #0, #15`. SBFM layout: `sf 00 100110 N immr imms Rn Rd` with N=sf, immr=0, imms=15. Register 31 is WZR/XZR, never SP/WSP.
  - llvm-mc `-triple=aarch64` (probed): `sxth w0, w1` = 0x13003c20; `sxth x0, w1` = 0x93403c20; `sxth x0, x1` canonicalizes to `sxth x0, w1` (accepted); `sxth w0, x1`, extra operand, SP/WSP, FP, arity 0/1 all error.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already hosts neighbouring PBT modules (`encode_smull_pbt`, `encode_sbc_pbt`, …). New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1578 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_sxth_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_sxth (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_sxth):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GNU-style SXTH Wd,Wn / Xd,Wn (and Xd,Xn which llvm-mc canonicalizes). State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_sxtb / encode_uxth fail the same-job gate (imms=7 / UBFM opc). encode_sbfm with #0,#15 is the ARM ARM alias (algebraic metamorphic, not an independent differential — shared get_reg / same crate). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style `sxth` text. Mapping: `encode_sxth([Reg(Rd), Reg(Rn)])` <-> `sxth Rd, Rn`.
- **Seeds:** README.md:217 Extensions table; encoder/mod.rs:294 dispatch; ARM ARM SXTH = SBFM #0,#15; llvm-mc KAT encodings. No existing unit test of encode_sxth.
- **State machine:** not applicable — encode_sxth is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending a word.

## Module: encode_sxth
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of arity, extra, width, SP, FP, non-Reg, invalid name, x31, uppercase, LR, Xd/Xn — added encode_sxth_diff_alt_spellings, encode_sxth_neg_nonreg, encode_sxth_neg_invalid_name)
