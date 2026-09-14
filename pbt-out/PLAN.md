# PBT Campaign: encode_umaddl

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: built-in AArch64 assembler translates GNU-style assembly text; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:214` Data Processing lists `umaddl` (alongside `smull`, `umull`, `smaddl`).
  - Encoder module docstring `src/backend/arm/assembler/encoder/mod.rs:1-7`: encodes AArch64 instructions into 32-bit machine code words.
  - Dispatch `encoder/mod.rs:270`: `"umaddl" => encode_umaddl(operands)` (scalar only; no NEON arrangement path, unlike `umull`/`smull`).
  - Body at `data_processing.rs:664-676`: comment "Encode UMADDL Xd, Wn, Wm, Xa (unsigned multiply-add long)"; format `1 00 11011 101 Rm 0 Ra Rn Rd`; `is_64` from get_reg is discarded (`_`); extra operands beyond index 3 ignored (`get_reg` only reads 0..3); no width check.
  - ARM ARM Data-processing (3 source) UMADDL: sf=1 U=1 11011 101 Rm o0=0 Ra Rn Rd. Syntax UMADDL Xd, Wn, Wm, Xa. UMULL Xd, Wn, Wm is the alias of UMADDL Xd, Wn, Wm, XZR. Register 31 is XZR/WZR, never SP/WSP. Dest and accumulator are 64-bit; multiply sources are 32-bit.
  - llvm-mc `-triple=aarch64` (probed): `umaddl x0, w1, w2, x3` = 0x9ba20c20; `umaddl xzr, wzr, wzr, xzr` = 0x9bbf7fff (disassembles as umull); `umaddl lr, w1, w2, x30` = 0x9ba2783e; `umaddl x0, w1, w2, xzr` = `umull x0, w1, w2` = 0x9ba27c20; `smaddl x0, w1, w2, x3` = 0x9b220c20 (XOR = 1<<23). W dest, X sources, W accumulator, extra operand, too few operands, SP/WSP, FP all error. Uppercase and `lr` accepted.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already hosts neighbouring PBT modules (`encode_smull_pbt`, `encode_madd_pbt`, …). New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1635 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_umaddl_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_umaddl (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_umaddl):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GNU-style UMADDL Xd, Wn, Wm, Xa. State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_smaddl fails the same-job gate (U=0 signed vs U=1 unsigned). encode_umull is the Ra=XZR alias of this instruction (algebraic.metamorphic, not an independent implementation — shared get_reg / same TU). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style `umaddl` text. Mapping: `encode_umaddl([Reg(Xd), Reg(Wn), Reg(Wm), Reg(Xa)])` <-> `umaddl Xd, Wn, Wm, Xa`.
- **Seeds:** README.md:214 Data Processing table; encoder/mod.rs:270 dispatch; ARM ARM UMADDL / UMULL alias; llvm-mc KAT encodings; neighbouring encode_smull_pbt (same 3-source multiply-long family). No existing unit test of encode_umaddl.
- **State machine:** not applicable — encode_umaddl is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending a word.

## Module: encode_umaddl
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of extra / width / SP / FP / non-Reg / invalid-name / alt-spellings — added encode_umaddl_diff_alt_spellings, encode_umaddl_neg_fp, encode_umaddl_neg_nonreg, encode_umaddl_neg_invalid_name)
