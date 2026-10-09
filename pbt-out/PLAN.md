# PBT Campaign: encode_push16 (i686)

## Scan findings
- **Spec:** (none found — function-scoped campaign via `--func encode_push16`)
- **Test layout:** project-owned Rust lib tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered via `#[cfg(test)] mod …_pbt` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` in Cargo.toml `[dev-dependencies]`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — PASS (2 passed, 5563 filtered out).
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_push16_pbt.rs` + one `#[cfg(test)] mod encode_push16_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_push16 (gp_integer.rs:382) — sole HARD-scope / `--func` target.
- **Skipped modules:** all other gp_integer symbols (encode_push, encode_pop, encode_mov*, …) and every file outside `src/backend/i686/assembler/encoder/gp_integer.rs` — HARD scope is `encode_push16` only. Prior-campaign ARM/AArch64 bug_reports are out of scope.
- **Oracle classification:** differential (llvm-mc i686) strongest applicable; algebraic.invariant (imm8 66 6A / imm16 66 68); algebraic.metamorphic (pushw imm8 = 0x66 ‖ pushl imm8); negative_error (arity ≠1, r32/r8). State machine rejected (pure encoding). Round-trip rejected (no in-tree PUSH decoder).
- **Doc contract:** no doc comment on encode_push16; module header lists PUSH/POP (gp_integer.rs:3); dispatch `pushw` → encode_push16 at encoder/mod.rs:196; sibling encode_push (gp_integer.rs:346) handles r32/imm/mem/symbol; sibling encode_pop16 handles r16/sreg/mem with 0x66; Intel SDM Vol.2 PUSH; core.rs emit_segment_prefix.
- **Body summary:** arity-1; Imm Integer → 0x66 + 6A/68 (i16 LE for non-i8); **all other operands → Err** (no r16, no Sreg, no mem, no symbol).
- **Effort tier:** standard (5–8 props, ≥1000 cases, ≥1 metamorphic/differential, 1 coverage_gaps sweep round).
- **Bugs found:** 4 (r16 unsupported; sreg unsupported; bare mem unsupported; segmented mem unsupported — 3 root causes).
- **Contract-surface sweep:** 1 round via coverage_gaps (no Rust .profraw; cargo symbol exercised) — documented missing arms already covered by failing props; close.

## Module: encode_push16
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round): coverage_gaps (no line-level; file-level cargo evidence) + missing r16/sreg/mem arms covered by failing properties — close
