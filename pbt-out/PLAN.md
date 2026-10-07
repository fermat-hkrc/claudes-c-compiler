# PBT Campaign: encode_bgtz

## Scan findings
- **Spec:** (none found beyond in-tree README/ISA comments) — RISC-V Unprivileged ISA BGTZ rs, offset = BLT x0, rs, offset; README.md:329 documents `blez/bgez/...` → corresponding `bge`/`blt` with x0 (two-operand form).
- **Test layout:** Cargo lib tests; sibling encoder PBT modules live as `src/backend/riscv/assembler/encoder/encode_*_pbt.rs`, registered with `#[cfg(test)] mod encode_*_pbt;` in `encoder/mod.rs`. Framework: proptest 1.11.0 (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_blez_kat_llvm_mc -- --test-threads=1` → PASS (1 passed). User build contract `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` pre-built successfully (pbt-out/build.log).
- **Harness placement:** extend existing Cargo lib test target — new file `src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs` + one `#[cfg(test)] mod encode_bgtz_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_bgtz (pseudo.rs:327) — sole campaign target.
- **Skipped modules:** all other functions in pseudo.rs and outside `src/backend/riscv/assembler/encoder/pseudo.rs` — HARD scope is encode_bgtz only; HEAD changes outside this path skipped.

## Module: encode_bgtz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Results: 11 properties passing, 1 failing (encode_bgtz_neg_extra) + KAT pass + regression fail witness.
- Serial reconfirm of extra-operand failure: FAIL with PBT_TEST_JOBS=1 / --test-threads=1.
- coverage_gaps: no Rust profraw; C++ binaries report encode_bgtz NOT LINKED. Sweep closed via manual documented-surface audit (tier standard: 1 round). Remaining gap is the filed extra-operand bug.
