# PBT Campaign: encode_mul

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README data-processing table `src/backend/arm/assembler/README.md:214` lists `mul` (and sibling `madd` / `msub` / `mneg`).
  - README NEON three-same table `src/backend/arm/assembler/README.md:224` lists vector `mul`.
  - Dispatch `encoder/mod.rs:238-244`: `"mul"` with a `RegArrangement` dest goes to `encode_neon_elem` / `encode_neon_three_same`; otherwise `encode_mul(operands)`.
  - Body at `data_processing.rs:584-595`: if dest is `RegArrangement`, `encode_neon_mul`; else `get_reg(0..2)` for Rd/Rn/Rm; `sf` from Rd only; word = `sf<<31 | 0b0011011000<<21 | Rm<<16 | 0b11111<<10 | Rn<<5 | Rd` (MADD with Ra=XZR). No arity upper bound. No same-width check. No SP/FP rejection.
  - Comment at `data_processing.rs:589`: "MUL Rd, Rn, Rm is MADD Rd, Rn, Rm, XZR".
  - Sibling `encode_madd` (`data_processing.rs:598-606`) is the 4-operand 3-source form with o0=0. Same encoding only when Ra=31. Same-job gate fails for the 4-operand public mnemonic; the Ra=ZR alias is documented on encode_mul itself.
  - Callers: `codegen/alu.rs:168,202` emit `mul w0, w1, w2` / `mul x0, x1, x2`; `codegen/i128_ops.rs:77` emits `mul x0, x2, x4`; `codegen/intrinsics.rs:187` emits `mul v0.16b, v0.16b, v1.16b` (that NEON form is dispatched outside encode_mul).
  - ARM ARM Data-processing (3 source) MUL (alias of MADD): `sf 00 11011 000 Rm 0 11111 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All three registers same width. Exactly three operands. o0 (bit 15) is 0. Ra (bits 14:10) is 31. Semantics: Rd := Rn * Rm.
  - ARM ARM Advanced SIMD MUL (vector): `0 Q 0 01110 size 1 Rm 10011 1 Rn Rd`. T in {8B,16B,4H,8H,2S,4S}. size==11 is UNDEFINED. Exactly three arrangement operands, matching T.
  - llvm-mc `-triple=aarch64` confirms the encodings, aliases `madd Rd, Rn, Rm, ZR` to `mul`, and rejects extra operand, mixed width, SP/WSP, FP dest, too few operands, and vector T in {1D,2D}.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules (`encode_msub_pbt`, `encode_madd_pbt`, …) — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1287 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_mul_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_msub_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_mul (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_mul):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid three-GPR same-width MUL and valid NEON vector MUL (T in {8b,16b,4h,8h,2s,4s}). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree MUL decoder rejected: none exists. encode_madd fails the same-job sibling gate for the 4-operand form; the documented Ra=XZR alias is checked differentially against llvm-mc of both `mul` and `madd ..., ZR`. SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `operands` <-> `mul Rd, Rn, Rm` or `mul Vd.T, Vn.T, Vm.T`.
- **Seeds:** encode_madd_pbt / encode_msub_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / mixed-width / SP / FP / arity negatives. Comment data_processing.rs:589 documents the MADD/XZR alias. No existing unit test of encode_mul itself. Seed: encode_madd_pbt::encode_madd_diff_gpr / encode_madd_diff_ra_zr_is_mul.
- **State machine:** not applicable — encode_mul is a pure function with no mutating operations or lifecycle.

## Module: encode_mul
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg 0..2 / sf / Ra=31 / neon_arr_to_q_size / source T — added encode_mul_neg_non_register, encode_mul_neg_neon_mismatch_t)
