# PBT Campaign: encode_seqz

## Scan findings
- **Spec:** README.md:324 `seqz rd, rs` → `sltiu rd, rs, 1`; inline comment pseudo.rs:260 `// sltiu rd, rs1, 1`; RISC-V Unprivileged ISA SEQZ = SLTIU rd, rs, 1. Dispatch: encoder/mod.rs:870 `"seqz" => encode_seqz(operands)`.
- **Test layout:** Project-owned Rust unit tests via `#[cfg(test)] mod …_pbt` under `src/backend/riscv/assembler/encoder/`, discovered by `cargo test --lib`. Framework: proptest 1.11 (dev-dependency). Filename convention: `encode_<name>_pbt.rs` beside the encoder modules.
- **Buildability probe:** `cargo test --lib encode_not_kat -- --test-threads=1` → PASS (1 passed). Build contract command family confirmed green.
- **Harness placement:** extend existing cargo lib-test target — new file `src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs` + one `mod encode_seqz_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_seqz (pseudo.rs:257) — sole `--func` target
- **Skipped modules:** all other functions in pseudo.rs (campaign HARD scope is encode_seqz only); HEAD changes outside src/backend/riscv/assembler/encoder/pseudo.rs

## Module: encode_seqz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
