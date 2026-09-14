# PBT Campaign: encode_neon_across_long

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:232`: NEON reduce category lists `saddlv` (alongside addv/umaxv/uminv/smaxv/sminv).
  - Dispatch `encoder/mod.rs:679-680`: `"saddlv"` → `encode_neon_across_long(operands, 0, 0b00011)`; `"uaddlv"` → `encode_neon_across_long(operands, 1, 0b00011)`.
  - Body at `neon.rs:1721-1736`: `SADDLV/UADDLV (signed/unsigned add long across vector)`. Format `0 Q U 01110 size 11000 00011 10 Rn Rd`. Comment: destination is a scalar register (e.g. s16), source is a vector arrangement. Arity check is `len < 2`. Dest accepts `Operand::Reg` or `Operand::RegArrangement` (arrangement ignored). Q/size from `neon_arr_to_q_size(arr_n)` which maps 8b/16b/4h/8h/2s/4s/1d/2d.
  - ARM ARM SADDLV/UADDLV (Advanced SIMD across lanes): `SADDLV <V><d>, <Vn>.<T>` / `UADDLV <V><d>, <Vn>.<T>`. Encoding: `0 Q U 01110 size 11000 opcode=00011 10 Rn Rd`. U=0 signed / U=1 unsigned. Valid T: 8B (Q=0,size=00,V=H), 16B (Q=1,size=00,V=H), 4H (Q=0,size=01,V=S), 8H (Q=1,size=01,V=S), 4S (Q=1,size=10,V=D). Reserved: size=11 (1D/2D) and size=10 with Q=0 (2S). Dest is a scalar SIMD register of twice the source element width, never a vector arrangement and never a GPR.
  - Codegen `src/backend/arm/codegen/alu.rs:65` and codegen README:594 emit `uaddlv h0, v0.8b`.
  - `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes with num<=31.
  - llvm-mc `-triple=aarch64` accepts `saddlv h0, v1.8b` as 0x0e303820; `uaddlv h0, v1.8b` as 0x2e303820; `saddlv h0, v1.16b` as 0x4e303820; `saddlv s0, v1.4h` as 0x0e703820; `saddlv s0, v1.8h` as 0x4e703820; `saddlv d0, v1.4s` as 0x4eb03820; `uaddlv h0, v0.8b` as 0x2e303800. Rejects: extra operand, dest type mismatch (saddlv s0, v1.8b / h0, v1.4h / b0 / x0 / q0 / v0.8h), reserved T (2s/1d/2d), invalid names (v32, h32).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `neon.rs` already has prior-campaign module `encode_neon_three_diff_narrow_pbt` — not rewritten.
- **Buildability probe:** `cargo test --lib encode_neon_three_diff_narrow_kat_llvm_mc_addhn_v0_v1_v2 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 1012 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neon_across_long_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (inline layout; sibling of `encode_neon_three_diff_narrow_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_neon_across_long (neon.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in neon.rs are indexed but not tested.
- **Oracle (encode_neon_across_long):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid SADDLV/UADDLV with scalar dest V in {H,S,D} matching T in {8B,16B,4H,8H,4S} and Vn in v0–v31. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree SADDLV decoder rejected: none exists. encode_neon_across / encode_neon_addv fail the same-job sibling gate (same-width reduce; different opcode and dest width). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(Vd), RegArrangement(Vn, T)]` + (u, opcode=0b00011) <-> `{saddlv|uaddlv} Vd, Vn.T`.
- **Seeds:** encode_neon_three_diff_narrow_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / dest-type / invalid-arrangement negatives. Codegen alu.rs:65 emits `uaddlv h0, v0.8b`. No existing unit test of encode_neon_across_long. Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_diff_llvm_mc at neon.rs encode_neon_three_diff_narrow_pbt.
- **State machine:** not applicable — encode_neon_across_long is a pure function with no mutating operations or lifecycle.

## Module: encode_neon_across_long
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of dest `_` non-Reg arm and parse_reg_num None on dest RegArrangement — extended arity/shape generators; those paths Err as specified)
