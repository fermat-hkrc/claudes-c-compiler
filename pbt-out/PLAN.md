# PBT Campaign: encode_movsx (i686)

## Scan findings
- **Spec:** (none found beyond Intel SDM / AT&T conventions) — Intel SDM Vol.2 MOVSX: `0F BE /r` (r16/r32 ← r/m8), `0F BF /r` (r32 ← r/m16); operand-size `0x66` for 16-bit dest; segment overrides Group 2 before `0x66`. AT&T: `movsbl`/`movsbw`/`movswl`. llvm-mc `-triple=i686` reference. Dispatch: encoder/mod.rs:174-176.
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; log pbt-out/build.log). Project harness is cargo lib tests with proptest.
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_movsx_pbt.rs` + one `#[cfg(test)] mod encode_movsx_pbt;` in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_movsx (gp_integer.rs:272) — sole HARD-scope / change-surface target.
- **Skipped modules:** encode_movzx and all other gp_integer symbols, plus every other encoder — HARD scope: test only encode_movsx. HEAD changes outside this path go under Skipped.
- **Oracle (strongest):** differential vs llvm-mc; algebraic.invariant (0F BE/BF + optional 66 + ModRM); algebraic.metamorphic (movsx vs movzx same operands, opcode nibble only); negative_error (arity, mismatched widths, non-GP).
- **Doc contract:** (none) on function body; no doc comment at gp_integer.rs:272.
- **Dispatch path:** encoder/mod.rs:174-176 movsbl/movswl/movsbw → encode_movsx(src_size, dst_size).
- **Seed:** encode_mov_mem_reg_pbt.rs / encode_mov_rr_pbt.rs (llvm-mc helpers, segment differential, ModRM invariant).
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run. Tool: no .gcda/.profraw (Rust cargo); file-level evidence from tool pointed at unrelated OH binaries (NOT LINKED false negative). Campaign evidence: `cargo test --lib encode_movsx` compiles and executes production `InstructionEncoder::encode` → `encode_movsx` (KAT + 1000-case properties). Documented behaviors covered: RR/base/disp/SIB/edge differential, all-six segment differential (fails), segment+SIB strengthen (fails), opcode/modrm invariant, movsx↔movzx metamorphic, arity/width/non-GP negative (width+non-GP fail). Sweep closed: tier rounds done.

## Module: encode_movsx
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round via coverage_gaps; file-level + cargo execution evidence)
