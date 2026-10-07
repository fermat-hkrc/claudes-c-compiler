# PBT Campaign: encode_bgt

## Scan findings
- **Spec:** (none found — no requirement doc for this symbol; contract from README.md:330 + RISC-V Unprivileged ISA pseudoinstruction table + inline `// blt rs2, rs1` at pseudo.rs:341)
- **Test layout:** Rust crate `ccc`; inline `#[cfg(test)]` modules beside encoder sources under `src/backend/riscv/assembler/encoder/encode_*_pbt.rs`, registered via `#[cfg(test)] mod encode_*_pbt;` in `encoder/mod.rs`. Framework: proptest 1.11.0 (Cargo.toml). Runner: `cargo test --lib <filter> -- --test-threads=1`.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` (user build contract) — PASS (log: pbt-out/build.log). Sibling probe path works for riscv encoder PBT modules.
- **Harness placement:** extend existing cargo lib-test target (rung 1) — new `src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs` + one `#[cfg(test)] mod encode_bgt_pbt;` line in `encoder/mod.rs`.
- **Candidate modules:** encode_bgt (pseudo.rs:336) — sole campaign target
- **Skipped modules:** all other functions in pseudo.rs (HARD scope: test only encode_bgt); HEAD changes outside src/backend/riscv/assembler/encoder/pseudo.rs; historical ARM bug_reports/* (out of scope)

## Module: encode_bgt
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Results: 11 passed, 1 failed (encode_bgt_neg_extra) + deterministic regression witness
- Serial reconfirm (PBT_TEST_JOBS=1): encode_bgt_neg_extra and test_encode_bgt_regression_extra_operand both FAIL — SUT bug
- Contract-surface sweep: 1× coverage_gaps → no line-level data; documented behaviors already have properties (incl. extra-operand error path)
- Strengthen: differential blt expansion + ISA unpack + boundary imm targets already beyond first shallow batch
