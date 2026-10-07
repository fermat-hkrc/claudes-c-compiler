# PBT Campaign: encode_bgtu

## Scan findings
- **Spec:** (none found — no requirement doc for this symbol; contract from README.md:330 `bgt/ble/bgtu/bleu` swapped-operand `blt`/`bge` variants + RISC-V Unprivileged ISA BGTU rs, rt, offset = BLTU rt, rs, offset + inline `// bltu rs2, rs1` at pseudo.rs:361)
- **Test layout:** Rust crate `ccc`; inline `#[cfg(test)]` modules beside encoder sources under `src/backend/riscv/assembler/encoder/encode_*_pbt.rs`, registered via `#[cfg(test)] mod encode_*_pbt;` in `encoder/mod.rs`. Framework: proptest 1.11.0 (Cargo.toml). Runner: `cargo test --lib <filter> -- --test-threads=1`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) → pass (pbt-out/build.log). Sibling shape confirmed via encode_bgt_pbt / encode_ble_pbt.
- **Harness placement:** extend existing cargo lib tests — new file `src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs` + one `#[cfg(test)] mod encode_bgtu_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_bgtu (pseudo.rs:356) — sole `--func` / change-surface target
- **Skipped modules:** all other functions in pseudo.rs and outside scope (single-symbol campaign); HEAD changes outside `src/backend/riscv/assembler/encoder/pseudo.rs` skipped

## Module: encode_bgtu
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Contract-surface sweep
- [x] Round 1: `coverage_gaps` (file-level only; no profraw for Rust). Reporter listed unrelated OH C++ binaries and said encode_bgtu NOT LINKED there — expected. All documented behaviors already have properties exercised by `cargo test --lib encode_bgtu_`. Closed.
