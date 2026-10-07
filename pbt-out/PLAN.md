# PBT Campaign: encode_snez

## Scan findings
- **Spec:** README.md:325 `snez rd, rs` → `sltu rd, x0, rs`; inline comment pseudo.rs:266 `// sltu rd, x0, rs2`; RISC-V Unprivileged ISA SNEZ = SLTU rd, x0, rs. No rustdoc on the function.
- **Test layout:** Project-owned Rust lib tests via `#[cfg(test)] mod …_pbt` under `src/backend/riscv/assembler/encoder/`, discovered by `cargo test --lib`. Filename convention `encode_<op>_pbt.rs`. Framework: proptest 1.11 (Cargo.toml dev-dependency).
- **Buildability probe:** `cargo test --lib encode_seqz_kat_llvm_mc -- --test-threads=1` → PASS (1 passed). Build contract command shape confirmed.
- **Harness placement:** extend existing lib-test target (rung 1) — add `src/backend/riscv/assembler/encoder/encode_snez_pbt.rs` and one `mod encode_snez_pbt;` line in `encoder/mod.rs` beside `encode_seqz_pbt`.
- **Candidate modules:** encode_snez (pseudo.rs:263) — sole `--func` / change-surface target.
- **Skipped modules:** all other functions in pseudo.rs (encode_li, encode_mv, encode_not, encode_neg, encode_negw, encode_sext_w, encode_seqz, encode_sltz, encode_sgtz, branch/jump pseudos, …) — outside HARD scope `encode_snez` only. HEAD changes outside `src/backend/riscv/assembler/encoder/pseudo.rs` — out of campaign scope.

## Module: encode_snez
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
