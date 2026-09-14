# PBT Campaign: encode_cbz

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:220`: Branches lists `cbz`, `cbnz`.
  - Assembler README `src/backend/arm/assembler/README.md:267`: Relocation types: `CondBr19` ELF 280 for conditional branch, 19-bit.
  - Assembler README `src/backend/arm/assembler/README.md:456-464`: All branch-type relocations (B, BL, B.cond, CBZ/CBNZ, TBZ/TBNZ) are deferred and resolved after labels are known.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` encodes branches including CBZ/CBNZ.
  - Dispatch `encoder/mod.rs:321-322`: `"cbz" => encode_cbz(operands, false)`, `"cbnz" => encode_cbz(operands, true)`.
  - Function comment `compare_branch.rs:242-251`: `CBZ/CBNZ: sf 011010 op imm19 Rt`; `RelocType::CondBr19`.
  - RelocType comment `encoder/mod.rs:76`: `R_AARCH64_CONDBR19 - conditional branch, 19-bit offset`; `elf_type` 280 at `encoder/mod.rs:115`.
  - ARM ARM Compare and branch (immediate): bits[31]=sf, [30:25]=011010, [24]=op (0=CBZ, 1=CBNZ), [23:5]=imm19, [4:0]=Rt. Signed 19-bit PC offset, multiple of 4, range [-1048576, 1048572] (±1 MiB).
  - Callers: `encoder/mod.rs:321-322` only. Codegen emits `cbz xN, .Llabel` / `cbnz wN, .Llabel` (peephole.rs, emit.rs, memory.rs, atomics.rs, i128_ops.rs), not `cbz Rt, #imm`.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg at idx; `get_symbol` (`encoder/mod.rs:975-989`) accepts Symbol/Label/SymbolOffset/Modifier/ModifierOffset and parser-misclassified Reg/Cond/Barrier; Imm and other kinds return Err.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has `#[cfg(test)] mod encode_bl_pbt`, `mod encode_blr_pbt`, `mod encode_br_pbt`, `mod encode_branch_pbt` from prior campaigns (not rewritten).
- **Buildability probe:** `cargo test --lib encode_br_kat_llvm_mc_br_x0 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 702 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_cbz_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_branch_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_cbz (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_cbz):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `cbz/cbnz Rt, #imm` (aligned PC offset in ±1 MiB). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CBZ decoder rejected: none exists. encode_tbz / encode_cond_branch are related encodings (TstBr14 / B.cond) but different jobs so they fail the same-job sibling gate as a differential reference. CBZ vs CBNZ (is_nz flag) is used only as a metamorphic transform (bit 24 XOR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rt), Imm(imm)]` <-> asm text `cbz/cbnz rt, #imm`; `[Reg(rt), Symbol(s)|Label(s)|SymbolOffset(s,a)]` <-> `cbz/cbnz rt, s{+a}` (reloc form; llvm-mc does not emit a concrete word for unresolved labels).
- **Seeds:** encode_branch_pbt / encode_bl_pbt (same compare-and-branch class): encode_branch_diff_imm_llvm_mc, encode_branch_symbol_reloc, encode_branch_meta_vs_bl, encode_branch_neg_arity / _imm_unaligned_oor / _extra_operand / _bad_operand.

## Module: encode_cbz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_symbol — Reg/Cond/Barrier via encode_cbz_symbol_misclassified; Mem/Shift/Extend/Expr via encode_cbz_neg_bad_label_kind; Imm/extra/SP/FP filed as bugs)
