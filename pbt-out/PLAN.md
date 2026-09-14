# PBT Campaign: encode_blr

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:220`: Branches lists `blr`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` encodes branches including BLR.
  - Dispatch `encoder/mod.rs:319`: `"blr" => encode_blr(operands)`.
  - Function comment `compare_branch.rs:219-224`: `BLR: 1101011 0001 11111 000000 Rn 00000`; word `0xd63f0000 | (rn << 5)`.
  - ARM ARM Unconditional branch (register) BLR: bits[31:25]=1101011, opc[24:21]=0001, op2[20:16]=11111, op3[15:10]=000000, Rn[9:5], op4[4:0]=00000. Rn is a 64-bit GPR (Xn / XZR / LR). SP is not a valid Rn.
  - Callers: `encoder/mod.rs:319` only. Codegen emits `blr x17` for indirect calls (`src/backend/arm/codegen/calls.rs:233`).
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg at the given index; `parse_reg_num` maps x/w/d/s/q/v/h/b 0-31, sp/wsp/xzr/wzr -> 31, lr -> 30.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has `#[cfg(test)] mod encode_bl_pbt` from a prior campaign (not rewritten).
- **Buildability probe:** `cargo test --lib encode_bl_kat_symbol_foo -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 657 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_blr_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_bl_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_blr (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_blr):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `blr Xn` / `blr xzr` / `blr lr`. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree BLR decoder rejected: none exists. encode_br is a related encoding sibling (BR vs BLR) but a different job (no link) so it fails the same-job sibling gate as a differential reference; it is used only as a metamorphic transform (bit 21 XOR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg("xN"|"xzr"|"lr")]` <-> asm text `blr xN`.

## Module: encode_blr
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg — invalid names passing; W/SP/FP/extra filed as bugs)
