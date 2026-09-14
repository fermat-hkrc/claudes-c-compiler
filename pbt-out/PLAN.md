# PBT Campaign: encode_sxtw

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: built-in AArch64 assembler translates GNU-style assembly text; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:217`: Extensions lists `sxtw` (alongside `sxth`, `sxtb`, `uxtw`, `uxth`, `uxtb`).
  - Encoder module docstring `src/backend/arm/assembler/encoder/mod.rs:1-7`: encodes AArch64 instructions into 32-bit machine code words.
  - Dispatch `encoder/mod.rs:293`: `"sxtw" => encode_sxtw(operands)`.
  - Body at `data_processing.rs:855-861`: reads Rd and Rn via `get_reg`; hardcodes sf=1, N=1, immr=0, imms=31; extra operands beyond index 1 are ignored; dest/src width bits from `get_reg` are discarded.
  - ARM ARM C6 SXTW (alias of SBFM): `SXTW <Xd>, <Wn>` = `SBFM <Xd>, <Xn>, #0, #31`. There is no 32-bit (Wd) form. SBFM layout: `sf 00 100110 N immr imms Rn Rd` with sf=1, N=1, immr=0, imms=31. Register 31 is XZR/WZR, never SP/WSP.
  - llvm-mc `-triple=aarch64` (probed): `sxtw x0, w1` = 0x93407c20; `sxtw xzr, wzr` = 0x93407fff; `sxtw x0, x1` canonicalizes to `sxtw x0, w1` (accepted); `sbfm x0, x1, #0, #31` aliases to the same word; `sxtw w0, w1`, extra operand, SP/WSP, FP, arity 0/1 all error.
  - Callers emit GNU-style `sxtw xN, wM` (cast_ops.rs, atomics.rs, f128.rs, alu.rs, peephole.rs).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already hosts neighbouring PBT modules (`encode_sxth_pbt`, `encode_smull_pbt`, …). New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1597 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_sxtw_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_sxtw (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_sxtw):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GNU-style SXTW Xd,Wn (and Xd,Xn which llvm-mc canonicalizes). State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_sxth / encode_sxtb / encode_uxtw fail the same-job gate (imms=15 / imms=7 / UBFM-as-MOV). encode_sbfm with #0,#31 is the ARM ARM alias (algebraic metamorphic, not an independent differential — shared get_reg / same crate). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style `sxtw` text. Mapping: `encode_sxtw([Reg(Rd), Reg(Rn)])` <-> `sxtw Rd, Rn`.
- **Seeds:** README.md:217 Extensions table; encoder/mod.rs:293 dispatch; ARM ARM SXTW = SBFM #0,#31; llvm-mc KAT encodings; neighbouring encode_sxth_pbt (same alias family). No existing unit test of encode_sxtw.
- **State machine:** not applicable — encode_sxtw is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending a word.

## Module: encode_sxtw
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of arity, extra, W dest, SP, FP, non-Reg, invalid name, x31, uppercase, LR, Xd/Xn — added encode_sxtw_diff_alt_spellings, encode_sxtw_neg_nonreg, encode_sxtw_neg_invalid_name)
