# PBT Campaign: encode_system_table (i686)

## Scan findings
- **Spec:** (none found — single-symbol campaign on `encode_system_table`; contract from doc comment + Intel SDM SGDT/SIDT/LGDT/LIDT 0F 01 /0../3 + llvm-mc i686 reference)
- **Test layout:** project-owned Rust lib tests via `cargo test --lib`; i686 encoder PBT files live as `src/backend/i686/assembler/encoder/encode_*_pbt.rs` registered with `#[cfg(test)] mod` in `encoder/mod.rs`; framework = proptest 1.11.0 (dev-dependency)
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) → PASS (prebuilt; log pbt-out/build.log). Rebuilds reuse `cargo test --lib encode_system_table -- --test-threads=1`.
- **Harness placement:** extend existing project test target (rung 1) — new `encode_system_table_pbt.rs` beside sibling `encode_invlpg_pbt.rs`, register one `#[cfg(test)] mod encode_system_table_pbt;` line in `encoder/mod.rs`
- **Candidate modules:** encode_system_table (src/backend/i686/assembler/encoder/system.rs:170)
- **Skipped modules:** all other functions in system.rs (HARD scope: test only encode_system_table); HEAD/ARM/x86-64 changes outside this path

## Module: encode_system_table
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Results: 10 properties passing, 2 failing (segment / segment+SIB); KAT segment + 3 regression witnesses fail (same root cause)
- Serial reconfirm: `--test-threads=1` reproduces the bug
- Strengthening round: abs disp32 + segment+SIB added; segment+SIB fails with same missing-prefix bug
- Contract-surface sweep: 1 round via coverage_gaps (file-level fallback; all documented behaviors already property-covered — close)
- Bugs: B1 missing segment prefix on memory arm (does not call emit_segment_prefix)
