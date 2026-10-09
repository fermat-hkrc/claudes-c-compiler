# PBT Campaign: encode_bsr_bsf_16 (i686)

## Scan findings
- **Spec:** (none found beyond in-source doc comment) — Intel SDM BSF/BSR r16, r/m16; AT&T `bsfw`/`bsrw`; llvm-mc `-triple=i686` is the independent reference. Doc at system.rs:352 `"Encode 16-bit BSF/BSR: bsfw/bsrw"`.
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; existing suite builds/runs).
- **Harness placement:** extend existing cargo lib-test target — new file `src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs` + one `#[cfg(test)] mod encode_bsr_bsf_16_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_bsr_bsf_16 (system.rs:353) — sole HARD-scope target.
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_out, encode_in, encode_invlpg, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_mov_seg, encode_pop16, and all other encoder symbols — HARD scope: test only encode_bsr_bsf_16.
- **Oracle (strongest):** differential vs llvm-mc `-triple=i686 -show-encoding`; algebraic.invariant (r16 form = 66 0F BC/BD /r); algebraic.metamorphic (bsfw/bsrw = 0x66 ‖ bsfl/bsrl); negative_error (arity / unknown mnemonic / r32 / r8 / wrong op kinds).
- **Doc contract:** system.rs:352 `"Encode 16-bit BSF/BSR: bsfw/bsrw"` — asserted fingerprint a7c3e91f. Body always emits 0x66 then 0F BC/BD; memory arm calls encode_modrm_mem without emit_segment_prefix; register arm uses reg_num without reg_size==2 gate.
- **Dispatch path:** `encode_mnemonic` routes `"bsrw" | "bsfw"` → `encode_bsr_bsf_16` (mod.rs:233). Sibling `encode_bsr_bsf` (gp_integer.rs:984) handles `bsr`/`bsf`/`bsrl`/`bsfl` without 0x66.
- **Seed:** (none) — no prior unit/PBT covering this symbol; pattern generalized from encode_pop16_pbt.rs / encode_lmsw_pbt.rs.
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run (owed).

## Module: encode_bsr_bsf_16
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round via coverage_gaps; file-level evidence)
