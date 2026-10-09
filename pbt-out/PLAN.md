# PBT Campaign: encode_pop (i686)

## Scan findings
- **Spec:** (none found — function-scoped campaign via `--func encode_pop`)
- **Test layout:** project-owned cargo lib tests; PBT modules live as `src/backend/i686/assembler/encoder/*_pbt.rs` registered via `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework proptest 1.11.0 (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — pre-campaign success, log at `pbt-out/build.log`.
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_pop_pbt.rs` + one `#[cfg(test)] mod encode_pop_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_pop (gp_integer.rs:402) — sole HARD-scope target.
- **Skipped modules:** all other gp_integer symbols (encode_push, encode_push16, encode_alu, …) — HARD scope is encode_pop only; HEAD changes outside `src/backend/i686/assembler/encoder/gp_integer.rs` skipped.
- **Doc contract:** no doc comment on encode_pop; module header lists PUSH/POP; inline comments `// Pop to segment register` and `// pop m32: 0x8F /0`.
- **Oracle class:** Differential (llvm-mc -triple=i686) primary; algebraic.invariant (r32=0x58+n; Sreg fixed opcodes); algebraic.metamorphic (segmented mem = seg_prefix ‖ bare); negative_error (arity, cs, r8, xmm).
- **Evidence:** Intel SDM Vol.2 POP; x86-64 sibling `encode_pop` calls `emit_segment_prefix` before 0x8F; core.rs:31-42 `emit_segment_prefix`; mod.rs:196 `"popl"|"pop" => encode_pop`.
- **Seed:** encode_push_pbt.rs / encode_pop16_pbt.rs patterns (same encoder family).

## Module: encode_pop
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
