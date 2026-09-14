# PBT Campaign: encode_madd

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README data-processing table `src/backend/arm/assembler/README.md:214` lists `madd` (and sibling `mul`/`msub`).
  - Dispatch `encoder/mod.rs:245`: `"madd" => encode_madd(operands)`.
  - Body at `data_processing.rs:598-605`: four GPR operands via get_reg(0..3); sf from Rd only; word = sf<<31 | 0b0011011000<<21 | Rm<<16 | Ra<<10 | Rn<<5 | Rd. No arity upper bound. Rn/Rm/Ra widths discarded.
  - Sibling comment `data_processing.rs:589`: `MUL Rd, Rn, Rm is MADD Rd, Rn, Rm, XZR`. encode_msub is o0=1 (different job). encode_smaddl/umaddl are long multiply-add (different encoding).
  - ARM ARM Data-processing (3 source) MADD: `sf 00 11011 000 Rm 0 Ra Rn Rd`. Rd/Rn/Rm/Ra=31 is XZR/WZR, never SP. All four registers same width (W or X). Exactly four operands. o0 (bit 15) is 0.
  - llvm-mc `-triple=aarch64` confirms: `madd x0, x1, x2, x3` = 0x9b020c20; `madd w0, w1, w2, w3` = 0x1b020c20; `madd x0, x1, x2, xzr` aliases `mul x0, x1, x2` = 0x9b027c20; `lr` aliases X30. Rejects 3 operands, 5 operands, mixed X/W, SP/WSP in any slot, FP/SIMD names, extra shift.
  - Callers: `i128_ops.rs:79-80` emit `madd x1, x3, x4, x1` / `madd x1, x2, x5, x1`.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules `encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`, `encode_bics_pbt`, `encode_div_pbt`, `encode_eon_pbt`, `encode_logical_pbt` — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1175 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_madd_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_div_pbt` / `encode_logical_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_madd (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_madd):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid four-GPR same-width MADD. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree MADD decoder rejected: none exists. encode_msub fails the same-job sibling gate (o0=1 vs o0=0). encode_mul is the Ra=XZR alias of MADD (documented at data_processing.rs:589) — used as a llvm-mc MUL differential on encode_madd(Ra=ZR), not by calling encode_mul (campaign is single-symbol). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `operands` <-> `madd Rd, Rn, Rm, Ra`.
- **Seeds:** encode_div_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / SP / mixed width / FP negatives. encode_mul comment (data_processing.rs:589). No existing unit test of encode_madd itself. Seed: encode_div_pbt::encode_div_diff_gpr_same_width / encode_div_invariant_arm_fields.
- **State machine:** not applicable — encode_madd is a pure function with no mutating operations or lifecycle.

## Module: encode_madd
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg 0..3 / sf / parse_reg_num lr — added encode_madd_diff_lr, passing)
