# PBT Campaign: encode_bnez

## Scan findings
- **Spec:** (none found beyond in-tree README) — README.md:328 documents `beqz/bnez` → `beq/bne rs, x0, label`. RISC-V Unprivileged ISA: BNEZ rs, offset = BNE rs, x0, offset. No separate requirement doc for this function.
- **Test layout:** Inline `#[cfg(test)]` modules under `src/backend/riscv/assembler/encoder/encode_*_pbt.rs`, registered via `mod encode_*_pbt;` in `encoder/mod.rs`. Framework: proptest 1.11.0 (Cargo.toml dev-dependencies). Runner: `cargo test --lib <filter>`.
- **Buildability probe:** `cargo test --lib encode_beqz_kat_llvm_mc -- --test-threads=1` → PASS (1 passed; sibling beqz harness green). Canonical build contract: `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1`.
- **Harness placement:** extend existing lib-test target (rung 1) — new file `src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs` + one `mod encode_bnez_pbt;` line in `encoder/mod.rs`, mirroring `encode_beqz_pbt`.
- **Candidate modules:** encode_bnez (pseudo.rs:291) — sole `--func` target
- **Skipped modules:** all other functions in pseudo.rs (out of `--func` scope); HEAD changes outside `src/backend/riscv/assembler/encoder/pseudo.rs`

## Module: encode_bnez
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Serial reconfirm of extra-operand failure: PASS as SUT bug (same shape as encode_beqz).
- Contract-surface sweep (1/1 standard): `coverage_gaps` reported no line-level data and C++-only "NOT LINKED" for encode_bnez; Rust cargo lib test binary executes the symbol (12 properties ran). Documented behaviors covered: llvm-mc differential, BNE expansion metamorphic, B-type layout, ABI/xN/fp alias, Symbol/Label/Reg/Imm targets, Imm-as-rs, arity/invalid rs/target, extra-operand negative.
- Strengthen round: added `encode_bnez_imm_as_rs` (get_reg bare Imm 0..31 path).
