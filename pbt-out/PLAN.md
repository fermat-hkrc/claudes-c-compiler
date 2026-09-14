# PBT Campaign: encode_movn

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README data-processing table `src/backend/arm/assembler/README.md:214` lists `movn` (siblings `mov`/`movz`/`movk`).
  - Dispatch `encoder/mod.rs:222`: `"movn" => encode_movn(operands)`.
  - Body at `data_processing.rs:266-289`: get_reg(0) for Rd/sf; get_imm(1); optional Shift at 2 with kind=="lsl" => hw=amount/16 else hw=0; word = sf<<31 | 0b00100101<<23 | hw<<21 | (imm as u32 & 0xFFFF)<<5 | Rd. No arity upper bound. No imm16 range check. No hw range check. Unlike encode_movz/encode_movk, no `:abs_g*:` Modifier path (comment at data_processing.rs:179-181 says those modifiers are for movz/movk).
  - Codegen `emit.rs:863-928` emits `movn Rd, #imm16` / `movn Rd, #imm16, lsl #N` (N in {16,32,48}) as the start of MOVN+MOVK wide-immediate sequences when most halfwords are 0xFFFF. All-ones uses `movn reg, #0`.
  - ARM ARM Move wide (immediate) MOVN: `sf 00 100101 hw imm16 Rd`. opc=00. Rd=31 is XZR/WZR, never SP. imm16 in [0, 65535]. hw in {0,1} when sf=0 (lsl #0/#16); hw in {0,1,2,3} when sf=1 (lsl #0/#16/#32/#48). Semantics: Rd := NOT(ZeroExtend(imm16) << (hw*16)). Exactly Rd + imm16 + optional lsl.
  - llvm-mc `-triple=aarch64` confirms: `movn x0, #42` = 0x92800540; `movn w0, #42` = 0x12800540; `movn x0, #42, lsl #16` = 0x92a00540; `lr` aliases X30. Rejects SP/WSP, imm outside [0,65535], non-lsl shift, lsl not in {0,16,32,48} (W: {0,16}), extra operand, FP/SIMD names, too few operands.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules (`encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`, `encode_bics_pbt`, `encode_div_pbt`, `encode_eon_pbt`, `encode_logical_pbt`, `encode_madd_pbt`, `encode_movk_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1213 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_movn_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_movk_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_movn (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_movn):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GPR + imm16 + optional lsl. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree MOVN decoder rejected: none exists. encode_movz / encode_movk fail the same-job sibling gate (opc 10/11 vs 00; MOVZ zeros other halfwords, MOVK keeps them). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `operands` <-> `movn Rd, #imm16 [, lsl #N]`.
- **Seeds:** encode_movk_pbt (same file, same Move-wide class): differential vs llvm-mc, ARM field unpack, extra-operand / SP / FP / imm-oob / invalid-shift negatives. Codegen emit.rs:873-902. No existing unit test of encode_movn itself. Seed: encode_movk_pbt::encode_movk_diff_imm_shift / encode_movk_invariant_arm_fields.
- **State machine:** not applicable — encode_movn is a pure function with no mutating operations or lifecycle.

## Module: encode_movn
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg / get_imm / Shift / extra / too-few / FP / invalid name — added encode_movn_neg_too_few, encode_movn_neg_fp, encode_movn_neg_invalid_name, encode_movn_neg_bad_second)
