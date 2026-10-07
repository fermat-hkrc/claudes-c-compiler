# PBT Campaign: encode_bgez

## Scan findings
- **Spec:** (none found beyond in-tree README) — README.md:329 documents `blez/bgez/...` → corresponding `bge`/`blt` with x0. RISC-V Unprivileged ISA: BGEZ rs, offset = BGE rs, x0, offset. Inline comment at pseudo.rs:313 `// bge rs, x0`. No separate requirement doc for this function.
- **Test layout:** Inline `#[cfg(test)]` modules under `src/backend/riscv/assembler/encoder/encode_*_pbt.rs`, registered via `mod encode_*_pbt;` in `encoder/mod.rs`. Framework: proptest 1.11.0 (Cargo.toml dev-dependencies). Runner: `cargo test --lib <filter>`.
- **Buildability probe:** `cargo test --lib encode_blez_kat_llvm_mc -- --test-threads=1` → PASS (sibling blez harness green). Canonical build contract: `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1`.
- **Harness placement:** extend existing lib-test target (rung 1) — new file `src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs` + one `mod encode_bgez_pbt;` line in `encoder/mod.rs`, mirroring `encode_blez_pbt` / `encode_beqz_pbt` / `encode_bnez_pbt`.
- **Candidate modules:** encode_bgez (pseudo.rs:309) — sole `--func` target
- **Skipped modules:** all other functions in pseudo.rs (out of `--func` scope); HEAD changes outside `src/backend/riscv/assembler/encoder/pseudo.rs`

## Module: encode_bgez
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results

## Review notes
- Serial reconfirm of extra-operand failure: PASS as SUT bug (same shape as encode_blez/encode_beqz/encode_bnez). Command: `PBT_TEST_JOBS=1 cargo test --lib test_encode_bgez_regression_extra_operand -- --test-threads=1`.
- Contract-surface sweep (1/1 standard): `coverage_gaps` reported no line-level data and C++-only "NOT LINKED" for encode_bgez; Rust cargo lib test binary executes the symbol (14 tests ran). Documented behaviors covered: llvm-mc differential, BGE expansion metamorphic, B-type layout, ABI/xN/fp alias, Symbol/Label/Reg/Imm targets, Imm-as-rs, arity/invalid rs/target, extra-operand negative.
- Strengthen angle already present: `encode_bgez_imm_as_rs` (get_reg bare Imm 0..31 path) + dual differential (bgez and bge rs,x0).
- Closed: tier round spent; remaining documented gap is the extra-operand bug.
