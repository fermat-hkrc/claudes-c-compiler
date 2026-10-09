# PBT Campaign: encode_mov_mem_reg (i686)

## Scan findings
- **Spec:** (none found beyond Intel SDM / AT&T conventions) — Intel SDM Vol.2 MOV r32/r16/r8, m32/m16/m8: opcode 8B/8A /r with optional 0x66; segment overrides 26/2E/36/3E/64/65. AT&T `movb`/`movw`/`movl mem, %dst`. llvm-mc `-triple=i686` reference. i686 `core.rs:31-42` `emit_segment_prefix` implements all six prefixes; `encode_mov_mem_reg` inlines fs/gs-only.
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; log pbt-out/build.log). Re-probed: `cargo test --lib encode_mov_rr_kat_llvm_mc_rr32 -- --test-threads=1` → 1 passed.
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_mov_mem_reg_pbt.rs` + one `#[cfg(test)] mod encode_mov_mem_reg_pbt;` in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_mov_mem_reg (gp_integer.rs:194) — sole HARD-scope / change-surface target.
- **Skipped modules:** encode_mov and all other gp_integer symbols, plus every other encoder — HARD scope: test only encode_mov_mem_reg.
- **Oracle (strongest):** differential vs llvm-mc; algebraic.invariant; algebraic.metamorphic (load vs store); negative_error (mismatched width; non-GP dest).
- **Doc contract:** (none) on function; body gp_integer.rs:198-203 fs/gs-only match is the producing statement under test.
- **Dispatch path:** encoder/mod.rs:161-165 movl/movw/movb → encode_mov → encode_mov_mem_reg for Memory,Register GP dest.
- **Seed:** encode_invlpg_pbt.rs memory/segment; encode_mov_rr_pbt.rs width/GP.
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run. Tool: no .gcda/.profraw (Rust cargo); file-level evidence listed unrelated OH C++ binaries as NOT LINKED for this symbol. Campaign evidence: `cargo test --lib encode_mov_mem_reg` compiles and executes production `InstructionEncoder::encode` → `encode_mov_mem_reg` (KAT + 1000-case properties). Documented behaviors covered: base/disp/SIB/edge differential, all-six segment differential (fails), opcode/modrm invariant, load/store metamorphic, mismatched-width negative (fails), non-GP negative (fails). No further documented branch left untargeted under HARD scope. Sweep closed: tier rounds done.

## Module: encode_mov_mem_reg
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round via coverage_gaps; file-level + cargo execution evidence)
