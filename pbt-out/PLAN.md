# PBT Campaign: encode_lmsw (i686)

## Scan findings
- **Spec:** (none found — single-symbol campaign on `encode_lmsw`; contract from doc comment + Intel SDM LMSW 0F 01 /6 r/m16 + llvm-mc i686 reference)
- **Test layout:** project-owned Rust lib tests via `cargo test --lib`; i686 encoder PBT files live as `src/backend/i686/assembler/encoder/encode_*_pbt.rs` registered with `#[cfg(test)] mod` in `encoder/mod.rs`; framework = proptest 1.11.0 (dev-dependency)
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) → PASS (prebuilt; log pbt-out/build.log). Rebuilds reuse `cargo test --lib encode_lmsw -- --test-threads=1`.
- **Harness placement:** extend existing project test target (rung 1) — new `encode_lmsw_pbt.rs` beside sibling `encode_verw_pbt.rs`, register one `#[cfg(test)] mod encode_lmsw_pbt;` line in `encoder/mod.rs`
- **Candidate modules:** encode_lmsw (src/backend/i686/assembler/encoder/system.rs:203)
- **Skipped modules:** all other functions in system.rs (HARD scope: test only encode_lmsw); HEAD/ARM/x86-64 changes outside this path

## Module: encode_lmsw
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Results: 8 properties passing, 3 failing (segment, segment+SIB, non-r16); KAT segment + 4 regression witnesses fail (two root causes)
- Serial reconfirm: `--test-threads=1` reproduces both root-cause bugs
- Strengthening round: abs disp32 + segment+SIB + metamorphic vs lidt; segment+SIB is same missing-prefix root cause as B1 (filed as B3 witness)
- Contract-surface sweep: 1 round via coverage_gaps (file-level fallback; cargo test confirms execution; all documented behaviors already property-covered — close)
- Bugs: B1/B3 missing segment prefix on memory arm (base+disp and SIB); B2 accepts non-r16 registers via reg_num aliasing
