# PBT Campaign: encode_msub

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README data-processing table `src/backend/arm/assembler/README.md:214` lists `msub` (and sibling `madd` / `mneg` / `mul`).
  - Dispatch `encoder/mod.rs:246`: `"msub" => encode_msub(operands)`.
  - Body at `data_processing.rs:608-616`: `get_reg(0..3)` for Rd/Rn/Rm/Ra; `sf` from Rd only; word = `sf<<31 | 0b0011011000<<21 | Rm<<16 | 1<<15 | Ra<<10 | Rn<<5 | Rd`. No arity upper bound. No same-width check. No SP/FP rejection.
  - Sibling `encode_madd` (`data_processing.rs:598-606`) is the same 3-source layout with o0=0 (bit 15 clear). Same-job gate fails (MADD vs MSUB).
  - Sibling `encode_mneg` (`data_processing.rs:677-686`) documents the alias: "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR". Same encoding with Ra=31.
  - Callers: `codegen/alu.rs:178,183,207,211` emit `msub w0, w3, w2, w1` / `msub x0, x3, x2, x1` for remainder (SRem/URem).
  - ARM ARM Data-processing (3 source) MSUB: `sf 00 11011 000 Rm 1 Ra Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All four registers same width. Exactly four operands. o0 (bit 15) is 1 (MADD is 0). Semantics: Rd := Ra - (Rn * Rm).
  - llvm-mc `-triple=aarch64` confirms the encoding, aliases `msub Rd, Rn, Rm, ZR` to `mneg`, and rejects extra operand, mixed width, SP/WSP, FP dest, too few operands.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules (`encode_madd_pbt`, `encode_movk_pbt`, …) — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1268 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_msub_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_madd_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_msub (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_msub):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid four-GPR same-width MSUB. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree MSUB decoder rejected: none exists. encode_madd / encode_mneg fail the same-job sibling gate (o0=0 vs o0=1; 3-operand alias vs 4-operand MSUB). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `operands` <-> `msub Rd, Rn, Rm, Ra`.
- **Seeds:** encode_madd_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / mixed-width / SP / FP / arity negatives. Sibling encode_mneg documents Ra=XZR alias. No existing unit test of encode_msub itself. Seed: encode_madd_pbt::encode_madd_diff_gpr / encode_madd_diff_ra_zr_is_mul.
- **State machine:** not applicable — encode_msub is a pure function with no mutating operations or lifecycle.

## Module: encode_msub
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg 0..3 / sf / o0=1 / extra operand / mixed width / SP / FP — added encode_msub_neg_sp, encode_msub_neg_fp, encode_msub_neg_invalid_reg, encode_msub_neg_non_register)
