# PBT Campaign: encode_mov_rr (i686)

## Scan findings
- **Spec:** (none found beyond inline comments) — Intel SDM Vol.2 MOV r/m,r/r: opcode 88/89 /r with optional 0x66 operand-size override for 16-bit; AT&T `movb`/`movw`/`movl %src, %dst`. llvm-mc `-triple=i686` is the independent reference. Inline comment at gp_integer.rs:163–176 documents segment-register arms (`mov %r16, %sreg` 8E /r; `mov %sreg, %r16` 8C /r); GP path has no separate doc comment.
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; log pbt-out/build.log; re-probed this campaign: 2 passed).
- **Harness placement:** extend existing cargo lib-test target — new file `src/backend/i686/assembler/encoder/encode_mov_rr_pbt.rs` + one `#[cfg(test)] mod encode_mov_rr_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_mov_rr (gp_integer.rs:162) — sole HARD-scope / change-surface target.
- **Skipped modules:** encode_mov and all other gp_integer symbols, plus every other encoder — HARD scope: test only encode_mov_rr. (Dispatch path through `InstructionEncoder::encode("movl"|"movw"|"movb", …)` with two GP registers reaches the production symbol via encode_mov → encode_mov_rr. Segment/CR RR forms are routed away by encode_mov before encode_mov_rr and are covered by prior encode_mov_seg / encode_mov_cr campaigns.)
- **Oracle (strongest):** differential vs llvm-mc `-triple=i686 -show-encoding`; algebraic.invariant (opcode 88|89, optional 0x66, mod=3); algebraic.metamorphic (identity + swap); negative_error (mismatched width; non-GP aliases).
- **Doc contract:** gp_integer.rs:163 `"// Handle segment register moves"` — other fingerprint 28e3197a. GP path: no width/GP-class validation before reg_num.
- **Dispatch path:** encoder/mod.rs:161-163 `movl`→size 4, `movw`→2, `movb`→1 → encode_mov → encode_mov_rr for Register,Register when neither is CR/seg.
- **Seed:** encode_mov_infer_size_pbt.rs:175; encode_mov_cr_pbt width-rejection pattern.
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run — tool reported no .gcda/.profraw (Rust cargo; file-level). Campaign evidence: cargo test executes production symbol via InstructionEncoder::encode; NOT LINKED listing referred to unrelated OH C++ binaries, not this Rust lib test. Documented behaviors covered: same-width differential, invariant, metamorphic identity/swap, mismatched-width negative, non-GP negative. No further documented GP-path branch left untargeted.

## Module: encode_mov_rr
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round via coverage_gaps; file-level evidence + cargo execution)
