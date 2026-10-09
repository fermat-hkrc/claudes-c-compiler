# PBT Campaign: encode_lea (i686)

## Scan findings
- **Spec:** (none found — function-scoped campaign via `--func encode_lea`)
- **Test layout:** project-owned Rust lib tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered via `#[cfg(test)] mod …_pbt` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` in Cargo.toml `[dev-dependencies]`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — PASS (log: pbt-out/build.log; re-probed this campaign: 2 passed, 5523 filtered out).
- **Harness placement:** extend existing cargo lib-test target — `src/backend/i686/assembler/encoder/encode_lea_pbt.rs` + one `#[cfg(test)] mod encode_lea_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_lea (gp_integer.rs:332) — sole HARD-scope / `--func` target.
- **Skipped modules:** all other gp_integer symbols (encode_mov*, encode_push, …) and every file outside `src/backend/i686/assembler/encoder/gp_integer.rs` — HARD scope is `encode_lea` only. Prior-campaign ARM/AArch64 bug_reports are out of scope.
- **Oracle classification:** differential (llvm-mc i686) strongest applicable; algebraic.invariant (opcode 0x8D + ModRM.reg); algebraic.metamorphic (LEA vs MOV mem→reg share Mod+RM/SIB/disp); negative_error (arity, non-Mem/Reg shapes, non-GP dest, r8/r16 dest). State machine rejected (pure encoding). Round-trip rejected (no in-tree LEA decoder).
- **Doc contract:** no doc comment on encode_lea; module header lists LEA among data-movement ops (gp_integer.rs:3); dispatch `leal|lea` → encode_lea(ops, 4) at encoder/mod.rs:186; Intel SDM LEA r16/r32,m; core.rs:31-42 emit_segment_prefix for all six segs.
- **Body summary:** arity-2; Memory→Register only; pushes 0x8D then encode_modrm_mem(dst); **does not** call emit_segment_prefix; **ignores** `_size` (no 0x66 for r16); dest gated only by reg_num (aliases non-GP).
- **Effort tier:** standard (5–8 props, ≥1000 cases, ≥1 metamorphic/differential, 1 coverage_gaps sweep round).
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run — tool reported no Rust .profraw (file-level fallback pointed at unrelated C++ binaries); cargo evidence shows encode_lea exercised. Both documented Err arms (arity + bad shape) hit by P7/P8; no additional documented branch beyond the two filed bugs.

## Module: encode_lea
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round): coverage_gaps (no Rust line data; cargo symbol exercised) + arity/shape Err paths covered — close
