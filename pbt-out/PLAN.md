# PBT Campaign: encode_sgtz

## Scan findings
- **Spec:** README.md:327 `sgtz rd, rs` → `slt rd, x0, rs`; inline comment pseudo.rs:278 `// slt rd, x0, rs2`; RISC-V Unprivileged ISA SGTZ = SLT rd, x0, rs. No rustdoc on the function.
- **Test layout:** Project-owned Rust lib tests via `#[cfg(test)] mod …_pbt` under `src/backend/riscv/assembler/encoder/`, discovered by `cargo test --lib`. Filename convention `encode_<op>_pbt.rs`. Framework: proptest 1.11 (Cargo.toml dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (user prebuilt; build.log). Sibling probe shape confirmed for encoder lib tests (`encode_sltz_pbt`, `encode_snez_pbt`).
- **Harness placement:** extend existing lib-test target (rung 1) — add `src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs` and one `mod encode_sgtz_pbt;` line in `encoder/mod.rs` beside `encode_sltz_pbt`.
- **Candidate modules:** encode_sgtz (pseudo.rs:275) — sole `--func` / change-surface target.
- **Skipped modules:** all other functions in pseudo.rs (encode_li, encode_mv, encode_not, encode_neg, encode_negw, encode_sext_w, encode_seqz, encode_snez, encode_sltz, branch/jump pseudos, …) — outside HARD scope `encode_sgtz` only. HEAD changes outside `src/backend/riscv/assembler/encoder/pseudo.rs` — out of campaign scope.

## Module: encode_sgtz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Results: 9 property tests run (8 PBT + KAT context); 8 passing properties, 1 failing (`encode_sgtz_neg_extra`) + regression witness.
- Serial reconfirm: `PBT_TEST_JOBS=1 cargo test --lib test_encode_sgtz_regression_extra_operand -- --test-threads=1` reproduces Ok(Word(8243)).
- Contract-surface sweep (1 round, standard tier): `coverage_gaps` had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries; encode_sgtz NOT LINKED there). Manual audit of documented behaviors: llvm-mc/slt/R-type/ABI/isolation/arity-invalid all covered and passing; extra-operand is the remaining documented gap (filed bug). Closed: tier round spent.
