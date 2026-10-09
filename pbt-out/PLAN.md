# PBT Campaign: encode_lsl (i686)

## Scan findings
- **Spec:** (none found — single-symbol campaign on `encode_lsl`; contract from doc comment + Intel SDM LSL 0F 03 /r + llvm-mc i686 reference)
- **Test layout:** project-owned Rust lib tests via `cargo test --lib`; i686 encoder PBT files live as `src/backend/i686/assembler/encoder/encode_*_pbt.rs` registered with `#[cfg(test)] mod` in `encoder/mod.rs`; framework = proptest 1.11.0 (dev-dependency)
- **Buildability probe:** `cargo test --lib encode_verw_kat_llvm_mc_ax -- --test-threads=1` → PASS (1 passed, 5242 filtered out). Build contract form confirmed.
- **Harness placement:** extend existing project test target (rung 1) — new `encode_lsl_pbt.rs` beside sibling `encode_verw_pbt.rs`, register one `#[cfg(test)] mod encode_lsl_pbt;` line in `encoder/mod.rs`
- **Candidate modules:** encode_lsl (src/backend/i686/assembler/encoder/system.rs:144)
- **Skipped modules:** all other functions in system.rs (HARD scope: test only encode_lsl); HEAD/ARM/x86-64 changes outside this path

## Module: encode_lsl
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Results: 4 properties passing, 4 failing (plus KAT/regression witnesses)
- Serial reconfirm: `--test-threads=1` reproduces all bugs
- Contract-surface sweep: 1 round via coverage_gaps (file-level fallback; documented behaviors already property-covered)
- Bugs: B1 missing segment prefix; B2 osize from src; B3/B4 mem16 missing 0x66 (base+disp and SIB)
