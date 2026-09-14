# PBT Campaign: encode_branch

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:220`: Branches lists `b`.
  - Assembler README `src/backend/arm/assembler/README.md:247-254`: Relocation types: `Jump26` ELF 282 for `b` (26-bit PC-relative jump).
  - Assembler README `src/backend/arm/assembler/README.md:456-464`: All branch-type relocations (B, BL, ...) are deferred and resolved after labels are known; JUMP26 encodes the imm26 field.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` encodes branches including B.
  - Dispatch `encoder/mod.rs:316`: `"b" => encode_branch(operands)`.
  - Function comment `compare_branch.rs:171-181`: `B: 000101 imm26` (filled by linker/assembler); `RelocType::Jump26`.
  - RelocType comment `encoder/mod.rs:51-52`: `R_AARCH64_JUMP26 - for B instruction (26-bit PC-relative)`.
  - ARM ARM Unconditional branch (immediate) B: bits[31:26]=000101, imm26=offset/4, signed 26-bit range ±128 MiB, multiple of 4.
  - Callers: `encoder/mod.rs:316` only. Codegen emits `b <label>` (peephole.rs, emit.rs), not `b #imm`.
  - `get_symbol` (`encoder/mod.rs:975-989`) accepts Symbol/Label/SymbolOffset/Modifier/ModifierOffset and parser-misclassified Reg/Cond/Barrier; Imm and other kinds return Err.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has `#[cfg(test)] mod encode_bl_pbt`, `mod encode_blr_pbt`, `mod encode_br_pbt` from prior campaigns (not rewritten).
- **Buildability probe:** `cargo test --lib encode_br_kat_llvm_mc_br_x0 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 687 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_branch_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_br_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_branch (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_branch):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `b #imm` (aligned PC offset in ±128 MiB). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree B decoder rejected: none exists. encode_bl is a related encoding sibling (BL vs B) but a different job (with link / Call26) so it fails the same-job sibling gate as a differential reference; it is used only as a metamorphic transform (bit 31 XOR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Imm(imm)]` <-> asm text `b #imm`; `[Symbol(s)|Label(s)|SymbolOffset(s,a)]` <-> `b s{+a}` (reloc form; llvm-mc does not emit a concrete word for unresolved labels).
- **Seeds:** encode_bl_pbt (same Unconditional branch (immediate) class): encode_bl_diff_imm_llvm_mc, encode_bl_symbol_reloc, encode_bl_meta_vs_b, encode_bl_neg_arity / _imm_unaligned_oor / _extra_operand / _bad_operand.

## Module: encode_branch
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_symbol — Reg/Cond/Barrier via encode_branch_symbol_misclassified; Imm/extra/modifier filed as bugs)
