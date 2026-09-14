# PBT Campaign: encode_movz

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README data-processing table `src/backend/arm/assembler/README.md:214` lists `movz` (siblings `mov`/`movk`/`movn`).
  - Dispatch `encoder/mod.rs:220`: `"movz" => encode_movz(operands)`.
  - Body at `data_processing.rs:201-231`: get_reg(0) for Rd/sf; optional Modifier at 1 via resolve_abs_g_modifier (comment at 179-181: `:abs_g*:` modifiers are for movz/movk); else get_imm(1); optional Shift at 2 with kind=="lsl" => hw=amount/16 else hw=0; word = sf<<31 | 0b10100101<<23 | hw<<21 | (imm as u32 & 0xFFFF)<<5 | Rd. No arity upper bound. No imm16 range check. No hw range check.
  - Codegen `emit.rs:911-922` emits `movz Rd, #imm16` / `movz Rd, #imm16, lsl #N` (N in {16,32,48}) as the start of MOVZ+MOVK wide-immediate sequences when most halfwords are 0x0000.
  - ARM ARM Move wide (immediate) MOVZ: `sf 10 100101 hw imm16 Rd`. opc=10. Rd=31 is XZR/WZR, never SP. imm16 in [0, 65535]. hw in {0,1} when sf=0 (lsl #0/#16); hw in {0,1,2,3} when sf=1 (lsl #0/#16/#32/#48). Semantics: Rd := ZeroExtend(imm16) << (hw*16). Exactly Rd + imm16 + optional lsl.
  - llvm-mc `-triple=aarch64` confirms: `movz x0, #42` = 0xd2800540; `movz w0, #42` = 0x52800540; `movz x0, #42, lsl #16` = 0xd2a00540; `lr` aliases X30. Rejects SP/WSP, imm outside [0,65535], non-lsl shift, lsl not in {0,16,32,48} (W: {0,16}), extra operand, FP/SIMD names, too few operands.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules (`encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`, `encode_bics_pbt`, `encode_div_pbt`, `encode_eon_pbt`, `encode_logical_pbt`, `encode_madd_pbt`, `encode_movk_pbt`, `encode_movn_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1231 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_movz_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_movk_pbt` / `encode_movn_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_movz (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_movz):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GPR + imm16 + optional lsl. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree MOVZ decoder rejected: none exists. encode_movk / encode_movn fail the same-job sibling gate (opc 11/00 vs 10; MOVK keeps other halfwords, MOVN inverts). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `operands` <-> `movz Rd, #imm16 [, lsl #N]`.
- **Seeds:** encode_movk_pbt (same file, same Move-wide class): differential vs llvm-mc, ARM field unpack, extra-operand / SP / FP / imm-oob / invalid-shift negatives. Codegen emit.rs:911-922. No existing unit test of encode_movz itself. Seed: encode_movk_pbt::encode_movk_diff_imm_shift / encode_movk_invariant_arm_fields.
- **State machine:** not applicable — encode_movz is a pure function with no mutating operations or lifecycle.

## Module: encode_movz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg / Modifier abs_g / get_imm / Shift / extra / too-few / FP / invalid name — added encode_movz_neg_too_few, encode_movz_neg_fp, encode_movz_neg_invalid_name, encode_movz_neg_bad_second)
