# PBT Campaign: encode_test (i686)

## Scan findings
- **Spec:** (none found — symbol-targeted campaign via `--func encode_test`)
- **Test layout:** Project-owned Rust inline/module tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered with `#[cfg(test)] mod ...` in `encoder/mod.rs`; runner is `cargo test --lib <filter>`. Framework: proptest 1.11.0 (dev-dep in Cargo.toml).
- **Buildability probe:** `cargo test --lib encode_alu_kat_llvm_mc_addl_rr -- --test-threads=1` → pass (1 passed, 5628 filtered). Canonical contract form: `cargo test --lib <target> -- --test-threads=1`.
- **Harness placement:** extend existing test target (rung 1) — new file `src/backend/i686/assembler/encoder/encode_test_pbt.rs` + one `#[cfg(test)] mod encode_test_pbt;` line in `mod.rs`.
- **Candidate modules:** encode_test in `src/backend/i686/assembler/encoder/gp_integer.rs:645` (HARD scope sole target)
- **Skipped modules:** all other functions in gp_integer.rs and outside `src/backend/i686/assembler/encoder/gp_integer.rs` — HARD scope is `encode_test` only; HEAD/prior campaign modules (ALU/MOV/system/ARM) not re-targeted

## Module: encode_test
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Review sweep: coverage_gaps round 1 (file-level; contract surface covered by 10 properties)
