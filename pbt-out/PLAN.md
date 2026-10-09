# PBT Campaign: encode_mov_imm_mem (i686)

## Scan findings
- **Spec:** (none found beyond Intel SDM / AT&T conventions) — Intel SDM Vol.2 MOV m32/m16/m8, imm32/imm16/imm8: opcode C7/C6 /0 with optional 0x66; segment overrides 26/2E/36/3E/64/65. AT&T `movb`/`movw`/`movl $imm, mem`. llvm-mc `-triple=i686` reference. i686 `core.rs:31-42` `emit_segment_prefix` implements all six prefixes; x86-64 sibling `encode_mov_imm_mem` calls `emit_segment_prefix` first. i686 body emits neither.
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; log pbt-out/build.log). Project harness is cargo lib tests with proptest.
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_mov_imm_mem_pbt.rs` + one `#[cfg(test)] mod encode_mov_imm_mem_pbt;` in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_mov_imm_mem (gp_integer.rs:238) — sole HARD-scope / change-surface target.
- **Skipped modules:** encode_mov, encode_mov_imm_reg, encode_mov_rr, encode_mov_mem_reg, encode_mov_reg_mem, encode_mov_infer_size and all other gp_integer symbols, plus every other encoder — HARD scope: test only encode_mov_imm_mem. HEAD changes outside this path go under Skipped.
- **Oracle (strongest):** differential vs llvm-mc; algebraic.invariant (C6/C7 /0 + imm trail); algebraic.metamorphic (same-mem across imms); negative_error (SymbolMod/SymbolDiff).
- **Doc contract:** (none) on function; body gp_integer.rs:238-270 has no segment emission; symbol arm error string documents size==4 only (limitation fingerprint 073edab9).
- **Dispatch path:** encoder/mod.rs:167-169 movl/movw/movb → encode_mov → encode_mov_imm_mem for Immediate,Memory.
- **Seed:** encode_mov_reg_mem_pbt.rs / encode_mov_mem_reg_pbt.rs; encode_system_table_pbt.rs (llvm-mc `A` fixup); x86-64 gp_integer.rs:179 emit_segment_prefix sibling.
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run. Tool: no .gcda/.profraw (Rust cargo); file-level evidence from tool pointed at unrelated OH binaries (NOT LINKED false negative). Campaign evidence: `cargo test --lib encode_mov_imm_mem` compiles and executes production `InstructionEncoder::encode` → `encode_mov_imm_mem` (KAT + 1000-case properties). Documented behaviors covered: base/disp/SIB/edge differential, all-six segment differential (fails), opcode/modrm/imm invariant, same-mem metamorphic, symbol imm32 differential, narrow symbol differential (fails), SymbolMod/Diff negative. No further documented branch left untargeted under HARD scope. Sweep closed: tier rounds done.

## Module: encode_mov_imm_mem
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round via coverage_gaps; file-level + cargo execution evidence)
