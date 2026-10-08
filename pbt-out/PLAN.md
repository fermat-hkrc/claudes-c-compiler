# PBT Campaign: encode_out

## Scan findings
- **Spec:** (none found — no requirement doc for this symbol; contract from doc comment `Encode OUT instruction: outb/outw/outl` at system.rs:38, dispatch in encoder/mod.rs:360 mapping `outb|outw|outl` → `encode_out(ops, mnemonic)`, Intel SDM OUT encodings E6/E7/EE/EF (+66h for 16-bit), llvm-mc `-triple=i686`, sibling x86-64 `encode_out` at x86/.../system.rs:6 which documents AT&T forms and accepts `(%dx)`)
- **Test layout:** Rust crate `ccc`; project-owned tests are inline `#[cfg(test)]` modules / `encode_*_pbt.rs` beside encoder sources, registered via `#[cfg(test)] mod encode_*_pbt;` in the encoder `mod.rs`. Framework: proptest 1.11.0 (Cargo.toml). Runner: `cargo test --lib <filter> -- --test-threads=1`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) → pass (pbt-out/build.log). proptest already a dev-dependency.
- **Harness placement:** extend existing cargo lib tests — new file `src/backend/i686/assembler/encoder/encode_out_pbt.rs` + one `#[cfg(test)] mod encode_out_pbt;` line in `src/backend/i686/assembler/encoder/mod.rs` (rung 1). Call path: public `InstructionEncoder::encode` → mnemonic dispatch → `encode_out` (pub(super)).
- **Candidate modules:** encode_out (system.rs:39) — sole `--func` / change-surface target
- **Skipped modules:** encode_prefetch, encode_prefetch_0f0d, encode_in, encode_invlpg, encode_verw, encode_lsl, encode_system_table, encode_lmsw, encode_smsw, encode_mov_cr, encode_mov_seg, encode_pop16, encode_bsr_bsf_16 (same file, outside single-symbol scope); HEAD changes outside `src/backend/i686/assembler/encoder/system.rs`

## Module: encode_out
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Contract-surface sweep
- [x] Round 1: `coverage_gaps` (file-level only; no profraw for Rust). Reporter listed unrelated OH C++ binaries and said encode_out NOT LINKED there — expected. All documented behaviors of encode_out have properties (DX/imm differential, opcodes, metamorphic outw=66|outl, arity, wrong regs, imm OOR, (%dx) form). Closed after filing 3 bugs.
