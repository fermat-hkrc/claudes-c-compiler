# PBT Campaign: encode_smull

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: built-in AArch64 assembler translates GNU-style assembly text; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:214`: Data Processing lists `smull` (alongside `umull`, `smaddl`, `umaddl`).
  - Encoder module docstring `src/backend/arm/assembler/encoder/mod.rs:1-7`: encodes AArch64 instructions into 32-bit machine code words.
  - Dispatch `encoder/mod.rs:247-256`: `"smull"` with `Operand::RegArrangement` goes to NEON (`encode_neon_elem_long` / `encode_neon_three_diff`); otherwise `encode_smull(operands)` (scalar SMULL). This campaign tests only the scalar helper.
  - Body at `data_processing.rs:630-639`: `/// Encode SMULL Xd, Wn, Wm -> SMADDL Xd, Wn, Wm, XZR`. Always sets bit 31; opcode bits `[30:21]=00 11011 001`; Ra=XZR (`[14:10]=11111`); o0=0 at bit 15. `is_64` from `get_reg` is discarded. Extra operands beyond index 2 are ignored (`get_reg` only reads 0..2).
  - ARM ARM Data-processing (3 source) SMADDL: `sf=1 op=0 U=0 11011 001 Rm o0=0 Ra Rn Rd`. SMULL Xd, Wn, Wm is the documented alias of SMADDL Xd, Wn, Wm, XZR. Dest is Xd (incl. XZR, never SP); sources are Wn/Wm (incl. WZR, never WSP). UMULL is the U=1 twin (bit 23).
  - Sibling `encode_smaddl` (`data_processing.rs:653-662`) encodes the same SMADDL format with a caller-supplied Ra. Same-job for the Ra=XZR restriction (ARM ARM alias). Sibling `encode_umull` is signed-vs-unsigned (U bit) — same-job gate fails for differential, but ARM ARM documents a 1-bit metamorphic relation.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already hosts neighbouring PBT modules (`encode_mul_pbt`, `encode_madd_pbt`, `encode_sbc_pbt`, …). New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1558 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_smull_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_smull (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_smull):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GNU-style SMULL Xd, Wn, Wm. State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_umull fails the same-job gate (U=1 unsigned long multiply). encode_smaddl with Ra=XZR is the ARM ARM alias (algebraic metamorphic, not an independent differential). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style `smull Xd, Wn, Wm` text. Mapping: `encode_smull([Reg(Xd), Reg(Wn), Reg(Wm)])` <-> `smull Xd, Wn, Wm`.
- **Seeds:** README.md:214 Data Processing table; encoder/mod.rs:247-256 dispatch; body docstring data_processing.rs:630 SMULL->SMADDL XZR alias; ARM ARM SMADDL/SMULL alias. No existing unit test of encode_smull.
- **State machine:** not applicable — encode_smull is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending a word.

## Module: encode_smull
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of arity, extra, width, SP, FP, non-Reg, invalid name — added encode_smull_diff_alt_spellings for x31/w31, XZR/LR, uppercase)
