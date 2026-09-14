# PBT Campaign: encode_br

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:220`: Branches lists `br`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` encodes branches including BR.
  - Dispatch `encoder/mod.rs:318`: `"br" => encode_br(operands)`.
  - Function comment `compare_branch.rs:212-216`: `BR: 1101011 0000 11111 000000 Rn 00000`; word `0xd61f0000 | (rn << 5)`.
  - ARM ARM Unconditional branch (register) BR: bits[31:25]=1101011, opc[24:21]=0000, op2[20:16]=11111, op3[15:10]=000000, Rn[9:5], op4[4:0]=00000. Rn is a 64-bit GPR (Xn / XZR / LR). SP is not a valid Rn.
  - Callers: `encoder/mod.rs:318` only. Codegen emits `br x0` for indirect jumps (`src/backend/arm/codegen/emit.rs:1760`) and `br x17` for jump tables (`emit.rs:1808`).
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg at the given index; `parse_reg_num` maps x/w/d/s/q/v/h/b 0-31, sp/wsp/xzr/wzr -> 31, lr -> 30.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has `#[cfg(test)] mod encode_bl_pbt` and `mod encode_blr_pbt` from prior campaigns (not rewritten).
- **Buildability probe:** `cargo test --lib encode_blr_kat_llvm_mc_blr_x0 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 672 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_br_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_blr_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_br (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_br):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `br Xn` / `br xzr` / `br lr`. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree BR decoder rejected: none exists. encode_blr is a related encoding sibling (BLR vs BR) but a different job (with link) so it fails the same-job sibling gate as a differential reference; it is used only as a metamorphic transform (bit 21 XOR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg("xN"|"xzr"|"lr")]` <-> asm text `br xN`.
- **Seeds:** encode_blr_pbt (same Unconditional branch (register) class): encode_blr_diff_xn_llvm_mc, encode_blr_word_layout, encode_blr_meta_vs_br, encode_blr_neg_arity / _w_reg / _extra_operand / _bad_operand / _wrong_reg_class.

## Module: encode_br
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg — invalid names passing; W/SP/FP/extra filed as bugs)
