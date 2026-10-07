# PBT Campaign: encode_beqz

## Scan findings
- **Spec:** README.md:328 `beqz/bnez` → `beq/bne rs, x0, label`; inline section comment pseudo.rs:281 `// Branch pseudo-instructions`; body expands via `encode_b(OP_BRANCH, 0b000, rs1, 0, 0)` + `RelocType::Branch`; RISC-V Unprivileged ISA BEQZ rs, offset = BEQ rs, x0, offset. No rustdoc on the function.
- **Test layout:** Project-owned Rust lib tests via `#[cfg(test)] mod encode_<op>_pbt` under `src/backend/riscv/assembler/encoder/`, discovered by `cargo test --lib`. Filename convention `encode_<op>_pbt.rs`. Framework: proptest 1.11 (Cargo.toml dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (user prebuilt; pbt-out/build.log). Sibling probe shape confirmed for encoder lib tests (`encode_sgtz_pbt`, `encode_branch_instr_pbt`).
- **Harness placement:** extend existing lib-test target (rung 1) — add `src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs` and one `mod encode_beqz_pbt;` line in `encoder/mod.rs` beside `encode_sgtz_pbt`.
- **Candidate modules:** encode_beqz (pseudo.rs:282) — sole `--func` / change-surface target.
- **Skipped modules:** all other functions in pseudo.rs (encode_li, encode_mv, encode_not, encode_neg, encode_negw, encode_sext_w, encode_seqz, encode_snez, encode_sltz, encode_sgtz, encode_bnez, other branch/jump pseudos, get_branch_target, …) — outside HARD scope `encode_beqz` only. HEAD changes outside `src/backend/riscv/assembler/encoder/pseudo.rs` — out of campaign scope.

## Module: encode_beqz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Results: 13 test items run (10 properties + KAT + regression); 9 properties passing, 1 failing (`encode_beqz_neg_extra`) + regression witness.
- Serial reconfirm: `PBT_TEST_JOBS=1 cargo test --lib test_encode_beqz_regression_extra_operand -- --test-threads=1` reproduces Ok(WordWithReloc word=327779).
- Strengthen round: added `encode_beqz_diff_llvm_mc_beq` (beq rs,x0,0 differential) and `encode_beqz_imm_target` (Imm target stringification + zero-imm word).
- Contract-surface sweep (1 round, standard tier): `coverage_gaps` had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries; encode_beqz NOT LINKED there). Manual audit of documented behaviors: llvm-mc/beq/B-type/ABI/target-forms/imm/arity-invalid all covered and passing; extra-operand is the remaining documented gap (filed bug). Closed: tier round spent.
