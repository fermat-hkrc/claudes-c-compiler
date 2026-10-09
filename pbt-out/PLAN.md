# PBT Campaign: encode_verw

## Scan findings
- **Spec:** (none found — no requirement doc for this symbol; contract from doc comment `Encode VERW: 0F 00 /5` at system.rs:123, dispatch in encoder/mod.rs:334 mapping `verw` → `encode_verw(ops)`, Intel SDM VERW = 0F 00 /5 + r/m16, llvm-mc `-triple=i686`, sibling x86-64 `encode_verw` at x86/.../system.rs:115 which calls `emit_rex_rm` before opcode for memory form)
- **Test layout:** project-owned Cargo lib tests; PBT files live beside the encoder as `src/backend/i686/assembler/encoder/*_pbt.rs`, registered via `#[cfg(test)] mod ...` in `encoder/mod.rs`; framework = proptest 1.11.0 (Cargo.toml); runner = `cargo test --lib <filter> -- --test-threads=1`
- **Buildability probe:** `cargo test --lib encode_invlpg_kat_llvm_mc_eax -- --test-threads=1` → 1 passed; 5225 filtered. Cargo lib harness builds and runs. Rung 1 available.
- **Harness placement:** extend existing cargo lib tests — new file `src/backend/i686/assembler/encoder/encode_verw_pbt.rs` + one `#[cfg(test)] mod encode_verw_pbt;` line in `src/backend/i686/assembler/encoder/mod.rs` (rung 1). Call path: public `InstructionEncoder::encode` → mnemonic dispatch → `encode_verw` (pub(super)).
- **Candidate modules:** encode_verw (system.rs:124) — sole `--func` / change-surface target
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_out, encode_in, encode_invlpg, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_mov_seg, encode_pop16, encode_bsr_bsf_16 (same file, outside single-symbol scope); HEAD changes outside `src/backend/i686/assembler/encoder/system.rs`

## Module: encode_verw
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Round 1: `coverage_gaps` (file-level only; no profraw for Rust). Reporter listed unrelated OH C++ binaries and said encode_verw NOT LINKED there — expected. All documented behaviors of encode_verw have properties (r16/base/SIB/abs differential, opcode/ext5, arity, non-r16, segment, segment+SIB). Closed after filing 2 bugs.
