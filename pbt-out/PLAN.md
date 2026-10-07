# PBT Campaign: encode_bltz

## Scan findings
- **Spec:** (none found as a separate requirement doc) — contract from README.md:329 (`blez/bgez/...` → corresponding `bge`/`blt` with x0), inline comment `// blt rs, x0` at pseudo.rs:322, RISC-V Unprivileged ISA BLTZ = BLT rs, x0, offset, and llvm-mc as independent assembler reference.
- **Test layout:** Project-owned Rust lib tests via `#[cfg(test)]` modules under `src/backend/riscv/assembler/encoder/`, discovered by `cargo test --lib`. Sibling pattern: `encode_bgez_pbt.rs` / `encode_blez_pbt.rs` registered in `encoder/mod.rs`. Filename convention: `encode_<mnemonic>_pbt.rs`. Framework: proptest 1.11 (`Cargo.toml`).
- **Buildability probe:** `cargo test --lib encode_bgez_kat_llvm_mc -- --test-threads=1` → pass (1 passed). User build contract: `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (prebuilt OK, log: pbt-out/build.log).
- **Harness placement:** extend existing Cargo lib-test target (rung 1) — new file `src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs` + one `#[cfg(test)] mod encode_bltz_pbt;` line in `encoder/mod.rs`.
- **Candidate modules:** encode_bltz (pseudo.rs:318) — sole `--func` / change-surface target.
- **Skipped modules:** (none within HARD scope). Other functions in pseudo.rs are out of `--func` scope by campaign instruction. HEAD changes outside `src/backend/riscv/assembler/encoder/pseudo.rs` skipped.

## Module: encode_bltz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
