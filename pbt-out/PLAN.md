# PBT Campaign: encode_in

## Scan findings
- **Spec:** (none found — no requirement doc for this symbol; contract from doc comment `Encode IN instruction: inb/inw/inl` at system.rs:74, dispatch in encoder/mod.rs:363 mapping `inb|inw|inl` → `encode_in(ops, mnemonic)`, Intel SDM IN encodings E4/E5/EC/ED (+66h for 16-bit), llvm-mc `-triple=i686`, sibling x86-64 `encode_in` at x86/.../system.rs:52 which documents AT&T forms `inb %dx, %al` / `inb $imm8, %al` / `inl (%dx), %eax`)
- **Test layout:** project-owned Cargo lib tests; PBT files live beside the encoder as `src/backend/i686/assembler/encoder/*_pbt.rs`, registered via `#[cfg(test)] mod ...` in `encoder/mod.rs`; framework = proptest 1.11.0 (Cargo.toml); runner = `cargo test --lib <filter> -- --test-threads=1`
- **Buildability probe:** `cargo test --lib encode_out_kat_llvm_mc_outb_dx -- --test-threads=1` → 1 passed, 5189 filtered out (0.02s). Rung 1 available.
- **Harness placement:** extend existing cargo lib tests — new file `src/backend/i686/assembler/encoder/encode_in_pbt.rs` + one `#[cfg(test)] mod encode_in_pbt;` line in `src/backend/i686/assembler/encoder/mod.rs` (rung 1). Call path: public `InstructionEncoder::encode` → mnemonic dispatch → `encode_in` (pub(super)).
- **Candidate modules:** encode_in (system.rs:75) — sole `--func` / change-surface target
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_out, encode_invlpg, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_mov_seg, encode_pop16, encode_bsr_bsf_16 (same file, outside single-symbol scope); HEAD changes outside `src/backend/i686/assembler/encoder/system.rs`

## Module: encode_in
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Round 1: `coverage_gaps` (file-level only; no profraw for Rust). Reporter listed unrelated OH C++ binaries and said encode_in NOT LINKED there — expected. All documented behaviors of encode_in have properties (DX/imm differential, opcodes, metamorphic inw=66|inl, arity, wrong regs, imm OOR, (%dx) form, DX-vs-imm families). Closed after filing 3 bugs.
