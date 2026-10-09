# PBT Campaign: encode_mov_reg_mem (i686)

## Scan findings
- **Spec:** (none found beyond Intel SDM / AT&T conventions) — Intel SDM Vol.2 MOV m32/m16/m8, r32/r16/r8: opcode 89/88 /r with optional 0x66; segment overrides 26/2E/36/3E/64/65. AT&T `movb`/`movw`/`movl %src, mem`. llvm-mc `-triple=i686` reference. i686 `core.rs:31-42` `emit_segment_prefix` implements all six prefixes; `encode_mov_reg_mem` inlines fs/gs-only (same class as encode_mov_mem_reg).
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; log pbt-out/build.log). Project harness is cargo lib tests with proptest.
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_mov_reg_mem_pbt.rs` + one `#[cfg(test)] mod encode_mov_reg_mem_pbt;` in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_mov_reg_mem (gp_integer.rs:216) — sole HARD-scope / change-surface target.
- **Skipped modules:** encode_mov, encode_mov_mem_reg, encode_mov_rr, encode_mov_infer_size and all other gp_integer symbols, plus every other encoder — HARD scope: test only encode_mov_reg_mem.
- **Oracle (strongest):** differential vs llvm-mc; algebraic.invariant; algebraic.metamorphic (store vs load); negative_error (mismatched width; non-GP src).
- **Doc contract:** (none) on function; body gp_integer.rs:220-225 fs/gs-only match is the producing statement under test.
- **Dispatch path:** encoder/mod.rs:161-165 movl/movw/movb → encode_mov → encode_mov_reg_mem for Register,Memory GP src.
- **Seed:** encode_mov_mem_reg_pbt.rs (load twin); encode_invlpg_pbt.rs memory/segment.
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run. Tool: no .gcda/.profraw (Rust cargo); file-level evidence. Campaign evidence: `cargo test --lib encode_mov_reg_mem` compiles and executes production `InstructionEncoder::encode` → `encode_mov_reg_mem` (KAT + 1000-case properties). Documented behaviors covered: base/disp/SIB/edge differential, all-six segment differential (fails), opcode/modrm invariant, store/load metamorphic, mismatched-width negative (fails), non-GP negative (fails). No further documented branch left untargeted under HARD scope. Sweep closed: tier rounds done.

## Module: encode_mov_reg_mem
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round via coverage_gaps; file-level + cargo execution evidence)
