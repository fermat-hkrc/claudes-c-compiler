# PBT Campaign: encode_invlpg

## Scan findings
- **Spec:** (none found — no requirement doc for this symbol; contract from doc comment `Encode INVLPG: 0F 01 /7 (memory operand)` at system.rs:109, dispatch in encoder/mod.rs:331 mapping `invlpg` → `encode_invlpg(ops)`, Intel SDM INVLPG = 0F 01 /7 + ModR/M memory, llvm-mc `-triple=i686`, sibling x86-64 `encode_mem_only(ops, &[0x0F, 0x01], 7)` at x86/.../system.rs:1513 / encode_mem_only at system.rs:321 which emits REX/segment via `emit_rex_rm` before opcode)
- **Test layout:** project-owned Cargo lib tests; PBT files live beside the encoder as `src/backend/i686/assembler/encoder/*_pbt.rs`, registered via `#[cfg(test)] mod ...` in `encoder/mod.rs`; framework = proptest 1.11.0 (Cargo.toml); runner = `cargo test --lib <filter> -- --test-threads=1`
- **Buildability probe:** `cargo test --lib encode_in_kat -- --test-threads=1` → 5 passed, 1 failed (pre-existing encode_in (%dx) bug, unrelated), 5203 filtered. Cargo lib harness builds and runs. Rung 1 available.
- **Harness placement:** extend existing cargo lib tests — new file `src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs` + one `#[cfg(test)] mod encode_invlpg_pbt;` line in `src/backend/i686/assembler/encoder/mod.rs` (rung 1). Call path: public `InstructionEncoder::encode` → mnemonic dispatch → `encode_invlpg` (pub(super)).
- **Candidate modules:** encode_invlpg (system.rs:110) — sole `--func` / change-surface target
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_out, encode_in, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_mov_seg, encode_pop16, encode_bsr_bsf_16 (same file, outside single-symbol scope); HEAD changes outside `src/backend/i686/assembler/encoder/system.rs`

## Module: encode_invlpg
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Round 1: `coverage_gaps` (file-level only; no profraw for Rust). Reporter listed unrelated OH C++ binaries and said encode_invlpg NOT LINKED there — expected. All documented behaviors of encode_invlpg have properties (base/SIB/abs differential, opcode/ext7, metamorphic vs lidt, arity, non-memory, segment, segment+SIB). Closed after filing 1 bug.
