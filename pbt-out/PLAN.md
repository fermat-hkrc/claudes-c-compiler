# PBT Campaign: encode_movzx (i686)

## Scan findings
- **Spec:** (none found — function-scoped campaign via `--func encode_movzx`)
- **Test layout:** project-owned Rust lib tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered via `#[cfg(test)] mod …_pbt` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` in Cargo.toml `[dev-dependencies]`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — PASS (log: pbt-out/build.log). Sibling `encode_movsx_pbt` already builds in the same tree.
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_movzx_pbt.rs` + one `#[cfg(test)] mod encode_movzx_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_movzx (gp_integer.rs:302) — sole HARD-scope / `--func` target.
- **Skipped modules:** encode_movsx and all other gp_integer symbols, plus every file outside `src/backend/i686/assembler/encoder/gp_integer.rs` — HARD scope is `encode_movzx` only. (encode_movsx was covered by a prior campaign.)
- **Oracle classification:** differential (llvm-mc i686) strongest applicable; algebraic.invariant (opcode/modrm); algebraic.metamorphic (movzx vs movsx); negative_error (arity, mismatched widths, non-GP). State machine rejected (pure encoding). Round-trip rejected (no in-tree MOVZX decoder).
- **Doc contract:** no doc comment on encode_movzx; contract from Intel SDM MOVZX + dispatch in mod.rs:179-181 + twin encode_movsx + core.rs emit_segment_prefix.
- **Dispatch path:** encoder/mod.rs:179-181 movzbl/movzwl/movzbw → encode_movzx(src,dst).
- **Effort tier:** standard (5–8 props, ≥1000 cases, ≥1 metamorphic/differential, 1 coverage_gaps sweep round).
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run.

## Module: encode_movzx
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round): coverage_gaps (file-level; Rust symbol exercised via cargo) + P12 encode_movzx_neg_unsupported_shape for gp_integer.rs:327 Err path — passing
