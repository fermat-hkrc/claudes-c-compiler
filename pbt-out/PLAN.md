# PBT Campaign: encode_bit_count (i686)

## Scan findings
- **Spec:** (none found — no requirement doc; Intel SDM Vol.2 LZCNT/TZCNT/POPCNT + llvm-mc i686 as reference)
- **Test layout:** Project-owned Rust inline tests: `src/backend/i686/assembler/encoder/*_pbt.rs` registered via `#[cfg(test)] mod ...` in `encoder/mod.rs`; runner `cargo test --lib <filter>`. Framework: proptest 1.11.0 (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — pre-campaign success (pbt-out/build.log). Neighbouring PBT modules (encode_bswap_pbt) compile under the same cargo test harness.
- **Harness placement:** extend existing cargo lib test target — new file `src/backend/i686/assembler/encoder/encode_bit_count_pbt.rs` + one `#[cfg(test)] mod encode_bit_count_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_bit_count (gp_integer.rs:959) — sole HARD-scope / change-surface target
- **Skipped modules:** all other functions in gp_integer.rs (HARD scope: test only encode_bit_count); files outside src/backend/i686/assembler/encoder/gp_integer.rs; dispatch-only gaps for unsuffixed `lzcnt`/`tzcnt`/`popcnt` and `*w` forms (never call encode_bit_count)

## Module: encode_bit_count
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Sweep
- [x] coverage_gaps round 1 (standard): no line-level data; file-level matcher missed Rust mangled `encode_bit_count` (false NOT LINKED); nm + KAT prove link/execution. Documented behaviors already have properties (rr success, mem, arity, width, non-GP, imm/label, opc invariant/meta). No additional gap properties required.
