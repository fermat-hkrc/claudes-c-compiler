# PBT Campaign: encode_mov_cr (i686)

## Scan findings
- **Spec:** (none found beyond in-source doc comment) — Intel SDM MOV to/from control registers: 0F 20 /r (CR→r32) and 0F 22 /r (r32→CR); doc at system.rs:249
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency)
- **Buildability probe:** `cargo test --lib encode_lmsw_kat_llvm_mc_ax -- --test-threads=1` → PASS (1 passed; existing suite builds/runs)
- **Harness placement:** extend existing cargo lib-test target — new file `src/backend/i686/assembler/encoder/encode_mov_cr_pbt.rs` + one `#[cfg(test)] mod encode_mov_cr_pbt;` line in `encoder/mod.rs` (rung 1)
- **Candidate modules:** encode_mov_cr (system.rs:250) — sole HARD-scope target
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_out, encode_in, encode_invlpg, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_seg, encode_pop16, encode_bsr_bsf_16 — HARD scope: test only encode_mov_cr; HEAD/prior-campaign symbols outside this function
- **Oracle (strongest):** differential vs llvm-mc `-triple=i686 -show-encoding` (independent assembler reference); plus algebraic.invariant (0F 20/22 + mod=3 + CR in ModRM.reg), algebraic.metamorphic (read↔write differ only in opcode byte 0x20↔0x22), negative_error (arity / non-CR-GP pairs / non-r32 GP)
- **Doc contract:** system.rs:249 `"Encode MOV to/from control register: 0F 20 /r (read) or 0F 22 /r (write)"` — asserted fingerprint 8d63ee6c
- **Dispatch path:** `encode_mov` (gp_integer.rs:18-19) routes when either operand is `is_control_reg` (cr0/cr2/cr3/cr4); public entry via mnemonic `movl`/`mov`/`movw`
- **Contract-surface sweep:** 1 round via `coverage_gaps` — tool reported no native .profraw (file-level evidence only; cargo tests still executed encode_mov_cr). Documented behaviors covered: CR→GP/GP→CR differential, opcode/ModRM invariant, read↔write metamorphic, arity error, non-r32/movw negative paths. No further documented branch without a property.

## Module: encode_mov_cr
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
