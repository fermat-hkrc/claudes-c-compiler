# PBT Campaign: encode_push (i686)

## Scan findings
- **Spec:** (none found — function-scoped campaign via `--func encode_push`)
- **Test layout:** project-owned Rust lib tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered via `#[cfg(test)] mod …_pbt` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` in Cargo.toml `[dev-dependencies]`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — PASS (log: pbt-out/build.log; re-probed this campaign: 2 passed, 5538 filtered out).
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_push_pbt.rs` + one `#[cfg(test)] mod encode_push_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_push (gp_integer.rs:346) — sole HARD-scope / `--func` target.
- **Skipped modules:** all other gp_integer symbols (encode_mov*, encode_lea, encode_push16, encode_pop, …) and every file outside `src/backend/i686/assembler/encoder/gp_integer.rs` — HARD scope is `encode_push` only. Prior-campaign ARM/AArch64 bug_reports are out of scope.
- **Oracle classification:** differential (llvm-mc i686) strongest applicable; algebraic.invariant (r32 short 0x50+n; imm8 0x6A / imm32 0x68; mem FF /6); algebraic.metamorphic (segment-stripped mem form shares ModRM/SIB with bare mem); negative_error (arity ≠1, non-GP/r8). State machine rejected (pure encoding). Round-trip rejected (no in-tree PUSH decoder).
- **Doc contract:** no doc comment on encode_push; module header lists PUSH/POP (gp_integer.rs:3); dispatch `pushl|push` → encode_push at encoder/mod.rs:191; Intel SDM Vol.2 PUSH; core.rs:31-42 emit_segment_prefix; x86-64 sibling calls emit_segment_prefix; sibling encode_pop has Sreg table.
- **Body summary:** arity-1; Register→0x50+reg_num (aliases non-GP; no Sreg; no 0x66 for r16); Imm Integer → 6A/68; Imm Symbol → 68+reloc; Memory → FF /6 **without** emit_segment_prefix; else Err.
- **Effort tier:** standard (5–8 props, ≥1000 cases, ≥1 metamorphic/differential, 1 coverage_gaps sweep round).
- **Bugs found:** 4 (missing segment prefix; non-GP/r8 accept; missing Sreg; missing r16 0x66).
- **Contract-surface sweep:** 1 round via coverage_gaps after first full run — see Review.

## Module: encode_push
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round): coverage_gaps (no Rust .profraw; cargo symbol exercised) + symbol-imm/mixed-arity arms covered — close
