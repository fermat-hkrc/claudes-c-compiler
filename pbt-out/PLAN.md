# PBT Campaign: encode_umulh

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: built-in AArch64 assembler translates GNU-style assembly text; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:214` Data Processing lists `umulh` (alongside `smulh`, `umull`, `umaddl`).
  - Encoder module docstring `src/backend/arm/assembler/encoder/mod.rs:1-7`: encodes AArch64 instructions into 32-bit machine code words.
  - Dispatch `encoder/mod.rs:274`: `"umulh" => encode_umulh(operands)` (scalar only; no NEON arrangement path).
  - Body at `data_processing.rs:688-694`: comment "UMULH: 1 00 11011 1 10 Rm 0 11111 Rn Rd"; `is_64` from get_reg is discarded (`_`); extra operands beyond index 2 ignored (`get_reg` only reads 0..2); no width check.
  - ARM ARM Data-processing (3 source) UMULH: sf=1 op54=00 11011 op31=110 Rm o0=0 Ra=11111 Rn Rd. Syntax UMULH Xd, Xn, Xm. All three operands are 64-bit GPRs. Register 31 is XZR, never SP. No 32-bit (W) form. Ra is always 11111 (unused). SMULH is the signed sibling with op31=010 (U=0 at bit 23).
  - llvm-mc `-triple=aarch64` (probed): `umulh x0, x1, x2` = 0x9bc27c20; `umulh xzr, xzr, xzr` = 0x9bdf7fff; `umulh lr, x1, x30` = 0x9bde7c3e; `umulh x0, x1, xzr` = 0x9bdf7c20; `smulh x0, x1, x2` = 0x9b427c20 (XOR = 1<<23). W dest/src, extra operand, too few operands, SP/WSP, FP all error. Uppercase, `x31`, and `lr` accepted.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already hosts neighbouring PBT modules (`encode_umaddl_pbt`, `encode_smull_pbt`, …). New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1655 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_umulh_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_umulh (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_umulh):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GNU-style UMULH Xd, Xn, Xm. State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_smulh fails the same-job gate (U=0 signed vs U=1 unsigned). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style `umulh` text. Mapping: `encode_umulh([Reg(Xd), Reg(Xn), Reg(Xm)])` <-> `umulh Xd, Xn, Xm`.
- **Seeds:** README.md:214 Data Processing table; encoder/mod.rs:274 dispatch; ARM ARM UMULH / SMULH U-bit; llvm-mc KAT encodings; neighbouring encode_umaddl_pbt (same 3-source multiply family). No existing unit test of encode_umulh.
- **State machine:** not applicable — encode_umulh is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending a word.

## Module: encode_umulh
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of extra / width / SP / FP / non-Reg / invalid-name / alt-spellings — added encode_umulh_neg_fp, encode_umulh_neg_nonreg, encode_umulh_neg_invalid_name)
