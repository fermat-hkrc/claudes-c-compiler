# PBT Campaign: encode_mvn

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README data-processing table `src/backend/arm/assembler/README.md:214` lists `mvn`.
  - README NEON two-misc table `src/backend/arm/assembler/README.md:225` lists `not`/`mvn`.
  - Dispatch `encoder/mod.rs:280`: `"mvn" => encode_mvn(operands)` (NEON dest is not pre-routed; encode_mvn itself branches).
  - Body at `data_processing.rs:748-771`: if dest is `RegArrangement`, `encode_neon_not`; else `get_reg(0..1)` for Rd/Rm; optional `Shift` at index 2 (lsl/lsr/asr/ror, else st=0); `sf` from Rd only; word = `sf<<31 | 0b01<<29 | 0b01010<<24 | shift<<22 | 1<<21 | Rm<<16 | (imm6&0x3F)<<10 | 0b11111<<5 | Rd` (ORN with Rn=XZR). No arity upper bound. No same-width check. No SP/FP rejection.
  - Comment at `data_processing.rs:753`: "MVN Rd, Rm [, shift #amount] -> ORN Rd, XZR, Rm [, shift #amount]".
  - Comment at `data_processing.rs:749`: "NEON vector form: MVN Vd.T, Vn.T (alias of NOT)".
  - Sibling `encode_orn` (`data_processing.rs:910-949`) is the 3-operand logical form; same encoding only when Rn=31. Same-job gate holds for the documented ZR-Rn alias; independence is weak (shared construction), so the campaign uses llvm-mc of both `mvn` and `orn Rd, ZR, Rm` as the independent reference.
  - `encode_neon_not` (`neon.rs:608-621`): requires 2 operands; Q=1 iff dest arrangement is `16b` else Q=0; encoding `0 Q 1 01110 00 10000 00101 10 Rn Rd`. Does not validate T in {8b,16b} or matching source T.
  - Callers: `codegen/alu.rs:26` `mvn x0, x0`; `codegen/i128_ops.rs:43-51` `mvn x0, x0` / `mvn x1, x1`; `codegen/inline_asm.rs:354` `mvn dest, dest`.
  - ARM ARM Logical (shifted register) MVN (alias of ORN): `sf 01 01010 shift 1 Rm imm6 11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd and Rm same width. Exactly two registers plus optional shift. shift in {LSL,LSR,ASR,ROR}. imm6 in 0..31 (sf=0; imm6<5>==1 is UNALLOCATED) or 0..63 (sf=1).
  - ARM ARM Advanced SIMD NOT (vector, alias MVN): `0 Q 1 01110 00 10000 00101 10 Rn Rd`. T in {8B,16B} only. Exactly two arrangement operands, matching T.
  - llvm-mc `-triple=aarch64` confirms the encodings, aliases `orn Rd, ZR, Rm` to `mvn`, aliases `not Vd.T, Vn.T` to `mvn`, and rejects extra operand, mixed width, SP/WSP, FP dest, too few operands, vector T not in {8B,16B}, mismatched T, and out-of-range shift.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules (`encode_mul_pbt`, `encode_eon_pbt`, …) — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1312 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_mvn_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_mul_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_mvn (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_mvn):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid two-GPR same-width MVN (optional LSL/LSR/ASR/ROR) and valid NEON vector MVN (T in {8b,16b}). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree MVN decoder rejected: none exists. encode_orn fails the independence gate as a primary differential (shared construction); the documented Rn=XZR alias is checked differentially against llvm-mc of both `mvn` and `orn ..., ZR`. SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `operands` <-> `mvn Rd, Rm{, shift #amt}` or `mvn Vd.T, Vn.T`.
- **Seeds:** encode_eon_pbt / encode_mul_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / mixed-width / SP / FP / arity / shift-range negatives. Comment data_processing.rs:753 documents the ORN/XZR alias. No existing unit test of encode_mvn itself. Seed: encode_eon_pbt::encode_eon_diff_reg_llvm_mc / encode_eon_invariant_arm_fields.
- **State machine:** not applicable — encode_mvn is a pure function with no mutating operations or lifecycle.

## Module: encode_mvn
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg 0..1 / sf / Shift at 2 / shift-kind / imm6 / Rn=31 / neon Q / neon extra — added encode_mvn_neg_bad_shift_kind, encode_mvn_neg_trailing_after_shift, encode_mvn_neg_neon_extra)
