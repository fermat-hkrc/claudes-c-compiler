# PBT Campaign: encode_pop16 (i686)

## Scan findings
- **Spec:** (none found beyond in-source doc comment) — Intel SDM POP r/m16 / POP Sreg; AT&T `popw`; llvm-mc `-triple=i686` is the independent reference. Doc at system.rs:324 `"Encode popw (16-bit pop)"`.
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; existing suite builds/runs).
- **Harness placement:** extend existing cargo lib-test target — new file `src/backend/i686/assembler/encoder/encode_pop16_pbt.rs` + one `#[cfg(test)] mod encode_pop16_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_pop16 (system.rs:325) — sole HARD-scope target.
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_out, encode_in, encode_invlpg, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_mov_seg, encode_bsr_bsf_16 — HARD scope: test only encode_pop16; HEAD/prior-campaign symbols outside this function.
- **Oracle (strongest):** differential vs llvm-mc `-triple=i686 -show-encoding`; algebraic.invariant; algebraic.metamorphic (popw = 0x66 ‖ popl); negative_error (arity / cs / r32 / r8).
- **Doc contract:** system.rs:324 `"Encode popw (16-bit pop)"` — asserted fingerprint b2a11c22. Inline comment system.rs:333 `"Segment register pops don't use 0x66 prefix"` — incorrect for `popw` (llvm-mc emits 0x66); true for `popl`.
- **Dispatch path:** `encode_mnemonic` routes `"popw"` → `encode_pop16` (mod.rs:175). Sibling `encode_pop` handles `popl`/`pop` including memory 0x8F /0.
- **Findings:** 3 bugs — (1) Sreg omits 0x66; (2) memory unsupported; (3) r32/r8 accepted via reg_num.
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run — no line-level profraw; documented behaviors all have properties. Closed: tier round done.

## Module: encode_pop16
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
