# PBT Campaign: encode_prefetch_0f0d

## Scan findings
- **Spec:** (none found — no requirement doc for this symbol; contract from doc comment `Encode prefetchw (0F 0D /1)` at system.rs:24, dispatch in encoder/mod.rs:746 mapping `prefetchw` → `encode_prefetch_0f0d(ops, 1)`, Intel/AMD PREFETCHW = `0F 0D /1` + ModR/M memory, sibling x86-64 `encode_sse_mem_only(ops, &[0x0F, 0x0D], 1)` which emits segment override before opcode, and i686 `emit_segment_prefix` at core.rs:31-42)
- **Test layout:** Rust crate `ccc`; project-owned tests are inline `#[cfg(test)]` modules / `encode_*_pbt.rs` beside encoder sources, registered via `#[cfg(test)] mod encode_*_pbt;` in the encoder `mod.rs`. Framework: proptest 1.11.0 (Cargo.toml). Runner: `cargo test --lib <filter> -- --test-threads=1`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) → pass (pbt-out/build.log). proptest already a dev-dependency.
- **Harness placement:** extend existing cargo lib tests — new file `src/backend/i686/assembler/encoder/encode_prefetch_0f0d_pbt.rs` + one `#[cfg(test)] mod encode_prefetch_0f0d_pbt;` line in `src/backend/i686/assembler/encoder/mod.rs` (rung 1). Call path: public `InstructionEncoder::encode` → mnemonic dispatch → `encode_prefetch_0f0d` (pub(super); not visible to sibling test module directly).
- **Candidate modules:** encode_prefetch_0f0d (system.rs:25) — sole `--func` / change-surface target
- **Skipped modules:** encode_prefetch, encode_out, encode_in, encode_invlpg, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_mov_seg, encode_pop16, encode_bsr_bsf_16 (same file, outside single-symbol scope); HEAD changes outside `src/backend/i686/assembler/encoder/system.rs`; bare gas `prefetch` (0F 0D /0) — not dispatched by i686 encoder

## Module: encode_prefetch_0f0d
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Contract-surface sweep
- [x] Round 1: `coverage_gaps` (file-level only; no profraw for Rust). Reporter listed unrelated OH C++ binaries and said encode_prefetch_0f0d NOT LINKED there — expected. All documented behaviors of encode_prefetch_0f0d have properties (opcode/hint, mem forms, edges, arity, non-mem, segment, metamorphic). Closed after filing segment-prefix bug.
