# PBT Campaign: encode_neon_shift_right

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README NEON shifts table `src/backend/arm/assembler/README.md:228` lists `sshr`, `ushr`, `srshr`, `urshr`, `ssra`, `usra`, `srsra`, `ursra`.
  - Dispatch `encoder/mod.rs:607-612`: `"srshr" => encode_neon_shift_right(operands, 0, 0b001001)`, `"urshr" => (1, 0b001001)`, `"ssra" => (0, 0b000101)`, `"usra" => (1, 0b000101)`, `"srsra" => (0, 0b001101)`, `"ursra" => (1, 0b001101)`.
  - Body at `neon.rs:1450-1468`: comment "NEON shift right accumulate (SSRA/USRA/SRSHR/URSHR)"; Format `0 Q U 01111 0 immh immb opcode 1 Rn Rd`; requires 3 operands; dest arrangement drives Q via `neon_arr_to_q_size` and element size `{8b/16b=>8, 4h/8h=>16, 2s/4s=>32, 2d=>64}`; source arrangement discarded; `shift = get_imm as u32`; rejects `shift == 0 || shift > element_bits`; `immhb = (element_bits * 2) - shift`; word = `q<<30 | u<<29 | 0b011110<<23 | (immhb>>3)<<19 | (immhb&7)<<16 | opcode<<10 | rn<<5 | rd`.
  - `neon_arr_to_q_size` (`neon.rs:44-55`) also accepts `1d` (Q=0, size=11); the later match does not, so `1d` Errs after Q lookup.
  - `get_neon_reg` (`neon.rs:7-21`) accepts `Operand::RegArrangement` and `Operand::Reg` (empty arrangement).
  - Siblings `encode_neon_ushr` / `encode_neon_sshr` (`neon.rs:1179-1228`) encode the same class with opcode `000001`; different job (SSHR/USHR vs SRSHR/SSRA/SRSRA family). Same-job gate fails for differential vs those siblings. Independence would also be weak (shared construction).
  - Callers: no codegen emission found; public assembler surface via dispatch + README table.
  - ARM ARM Advanced SIMD shift by immediate (vector) SSHR/USHR/SRSHR/URSHR/SSRA/USRA/SRSRA/URSRA: `0 Q U 011110 immh immb opcode Rn Rd`. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 (1D) is Reserved. Exactly three operands, matching T. shift = (2*esize) - UInt(immh:immb) in [1, esize]. opcode (bits 15:10): SRSHR/URSHR=001001, SSRA/USRA=000101, SRSRA/URSRA=001101. U=0 signed / U=1 unsigned.
  - llvm-mc `-triple=aarch64` confirms encodings, rejects extra operand, mismatched T, 1d, shift 0 / esize+1, and accepts scalar `srshr Dd, Dn, #imm` with a different encoding (out of this vector helper's claimed surface).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `neon.rs` already has prior-campaign modules (`encode_neon_qshrn_pbt`, `encode_neon_sli_pbt`, …) — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1343 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neon_shift_right_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (inline layout; sibling of `encode_neon_qshrn_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_neon_shift_right (neon.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in neon.rs are indexed but not tested.
- **Oracle (encode_neon_shift_right):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid three-operand vector SRSHR/URSHR/SSRA/USRA/SRSRA/URSRA with matching T in {8b,16b,4h,8h,2s,4s,2d} and shift in [1, esize]. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_neon_ushr/sshr fail the same-job gate (different opcode / mnemonic). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `operands,u_bit,opcode` <-> `{srshr|urshr|ssra|usra|srsra|ursra} Vd.T, Vn.T, #shift`.
- **Seeds:** encode_neon_qshrn_pbt (same file, same AdvSIMD shift-by-immediate class): differential vs llvm-mc, ARM field unpack, Q/U metamorphic, extra-operand / mismatched arrangement / GPR dest / arity / shift-range / i64-trunc negatives. No existing unit test of encode_neon_shift_right itself. Seed: encode_neon_qshrn_pbt::encode_neon_qshrn_diff_llvm_mc / encode_neon_qshrn_invariant_arm_fields.
- **State machine:** not applicable — encode_neon_shift_right is a pure function with no mutating operations or lifecycle.

## Module: encode_neon_shift_right
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of arity / T / get_imm as u32 / get_neon_reg Reg dest+source / extra / mismatched T — added encode_neon_shift_right_neg_shift_i64_trunc, encode_neon_shift_right_neg_reg_source)
