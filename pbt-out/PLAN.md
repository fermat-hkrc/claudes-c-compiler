# PBT Campaign: encode_bl

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:220`: Branches lists `bl`.
  - Assembler README `src/backend/arm/assembler/README.md:247-253`: When an instruction references an external symbol (e.g. `bl printf`), the encoder returns WordWithReloc; `Call26` ELF 283 for `bl` (26-bit PC-relative call).
  - Dispatch `encoder/mod.rs:317`: `"bl" => encode_bl(operands)`.
  - Function comment `compare_branch.rs:184-194`: `BL: 100101 imm26`; reloc `RelocType::Call26`.
  - ARM ARM Unconditional branch (immediate) BL: bits[31:26]=100101, bits[25:0]=imm26 (signed, offset = SignExtend(imm26)<<2); range ±128 MB; offset multiple of 4. Immediate form `bl #imm` is accepted by gas/llvm-mc.
  - RelocType comment `encoder/mod.rs:49`: `R_AARCH64_CALL26 - for BL instruction (26-bit PC-relative)`.
  - Callers: `encoder/mod.rs:317` only. Codegen emits `bl <name>` (symbol), not `bl #imm`.
  - `get_symbol` (`encoder/mod.rs:975-989`) documents parser-misclassified Reg/Cond/Barrier names as valid symbols in context.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Existing PBT modules live in `data_processing.rs`, `load_store.rs`, and `neon.rs`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` has no existing `#[cfg(test)]` block.
- **Buildability probe:** `cargo test --lib encode_adr_kat_llvm_mc_adr_x0_imm0 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 642 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_bl_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling PBT modules live in neighbouring encoder files). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_bl (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_bl):** Differential (llvm-mc -triple=aarch64 -show-encoding) for the immediate PC-offset form. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree BL decoder rejected: none exists. encode_branch is a related encoding sibling (B/Jump26 vs BL/Call26) but a different job (no link / different ELF reloc) so it fails the same-job sibling gate as a differential reference; it is used only as a metamorphic transform (bit 31 XOR plus reloc-type pair). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Symbol(s)|Label(s)|SymbolOffset(s,addend)]` <-> `bl s{+addend}` (WordWithReloc Call26, word=0x94000000) and `[Imm(imm)]` <-> `bl #imm` (Word with imm26 filled).

## Module: encode_bl
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_symbol arms — Reg/Cond/Barrier passing; Modifier/extra/Imm filed as bugs)
