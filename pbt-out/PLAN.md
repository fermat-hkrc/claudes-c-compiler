# PBT Campaign: encode_logical

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README data-processing table lists `and`, `orr`, `eor`, `ands`. NEON three-same table lists `and`, `orr`, `eor`.
  - Dispatch `encoder/mod.rs:231-234`: `and` opc=00, `orr` opc=01, `eor` opc=10, `ands` opc=11.
  - Body at `data_processing.rs:455-511`: arity >= 3; NEON RegArrangement dest delegates to encode_neon_logical; GPR bitmask immediate via encode_bitmask_imm; GPR shifted-register with optional lsl/lsr/asr/ror. No arity upper bound. Unknown shift kinds default to LSL. Shift amount masked with 0x3F.
  - ARM ARM Logical (shifted register): `sf opc 01010 shift N Rm imm6 Rn Rd` with N=0. opc 00 AND / 01 ORR / 10 EOR / 11 ANDS. shift 00 LSL / 01 LSR / 10 ASR / 11 ROR. 32-bit imm6 in [0,31]; 64-bit [0,63]. Rd/Rn/Rm=31 is XZR/WZR, never SP.
  - ARM ARM Logical (immediate): `sf opc 100100 N immr imms Rn Rd`. Rd=31 is SP for AND/ORR/EOR and XZR for ANDS (TST). llvm-mc (gas-compatible) rejects SP as Rn.
  - ARM ARM Advanced SIMD logical three-same: `0 Q U 01110 size 1 Rm 000111 Rn Rd`. AND U=0 size=00; ORR U=0 size=10; EOR U=1 size=00. T in {8B,16B} only. ANDS is not a NEON instruction.
  - llvm-mc `-triple=aarch64` confirms: `and x0, x1, x2` = 0x8a020020; `orr x0, x1, x2` = 0xaa020020; `eor x0, x1, x2` = 0xca020020; `ands x0, x1, x2` = 0xea020020; `and x0, x1, #1` = 0x92400020; `and sp, x0, #1` = 0x9240001f; `and v0.16b, v1.16b, v2.16b` = 0x4e221c20. Rejects extra operand, too few operands, mixed X/W, SP in shifted-register form, SP as Rn in immediate form, FP/SIMD GPR names, shift amount out of range, invalid bitmask (0 / all-ones / non-run), NEON T other than 8b/16b, mismatched NEON arrangements, ANDS on NEON. `lr` aliases X30. Register 31 in shifted-register form is XZR/WZR.
  - Caller `encode_tst` (`compare_branch.rs:37-49`) rewrites TST as ANDS XZR, Rn, op.
  - Sibling encode_bic/encode_orn/encode_eon/encode_bics are N=1 (AND-NOT / OR-NOT / EOR-NOT / ANDS-NOT) — different job. encode_neon_logical is a callee, not a same-job sibling.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules `encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`, `encode_bics_pbt`, `encode_div_pbt`, `encode_eon_pbt` — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1145 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_logical_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_eon_pbt` / `encode_bic_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_logical (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_logical):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid AND/ORR/EOR/ANDS shifted-register, bitmask-immediate, and NEON 8b/16b. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree logical decoder rejected: none exists. encode_bic/encode_orn/encode_eon/encode_bics fail the same-job sibling gate (N=1 vs N=0). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `(operands, opc)` <-> `{and|orr|eor|ands} Rd, Rn, Rm{, shift}` / `{and|orr|eor|ands} Rd, Rn, #imm` / `{and|orr|eor} Vd.T, Vn.T, Vm.T` with T in {8b,16b}.
- **Seeds:** encode_eon_pbt / encode_bic_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / SP / mixed width / FP / shift-range / invalid-imm negatives. encode_tst caller. No existing unit test of encode_logical itself. Seed: encode_eon_pbt::encode_eon_diff_reg_llvm_mc / encode_eon_invariant_arm_fields.
- **State machine:** not applicable — encode_logical is a pure function with no mutating operations or lifecycle.

## Module: encode_logical
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of arity / NEON / Imm / Reg / unsupported-third / invalid-reg / sf — added 3 properties; all reached)
