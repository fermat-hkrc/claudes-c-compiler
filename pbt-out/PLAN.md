# PBT Campaign: encode_imul (i686)

## Scan findings
- **Spec:** (none found — symbol-targeted campaign via `--func encode_imul`)
- **Test layout:** Project-owned Rust module tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered with `#[cfg(test)] mod ...` in `encoder/mod.rs`; runner is `cargo test --lib <filter>`. Framework: proptest (dev-dep in Cargo.toml).
- **Buildability probe:** `cargo test --lib encode_test_kat_llvm_mc_testl_rr -- --test-threads=1` → pass (1 passed, 5648 filtered). Canonical contract form: `cargo test --lib <target> -- --test-threads=1`.
- **Harness placement:** extend existing test target (rung 1) — new file `src/backend/i686/assembler/encoder/encode_imul_pbt.rs` + one `#[cfg(test)] mod encode_imul_pbt;` line in `mod.rs`.
- **Candidate modules:** encode_imul in `src/backend/i686/assembler/encoder/gp_integer.rs:711` (HARD scope sole target)
- **Skipped modules:** all other functions in gp_integer.rs and outside scope — HARD scope is `encode_imul` only. `imulb` not dispatched in mod.rs (size=1 multi-op unreachable via public encode).

## Module: encode_imul
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Review sweep: coverage_gaps round 1 (standard tier; strengthen bare32 + mismatched/non-GP)
