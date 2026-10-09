# PBT Campaign: encode_mov_infer_size (i686)

## Scan findings
- **Spec:** (none found beyond in-source doc comment) — GNU AS unsuffixed `mov` infers operand size from register operands; ambiguous forms (imm→mem, size-mismatched regs) require an explicit suffix. llvm-mc `-triple=i686` is the independent reference. Doc at gp_integer.rs:111 `"Handle unsuffixed mov from inline asm - infer size from operands"`.
- **Test layout:** project-owned Rust module tests under `src/backend/i686/assembler/encoder/`; pattern `encode_*_pbt.rs` + `#[cfg(test)] mod` in `encoder/mod.rs`; runner `cargo test --lib <filter> -- --test-threads=1`; framework `proptest = "1.11.0"` (dev-dependency).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → PASS (prebuilt SUT contract; log pbt-out/build.log).
- **Harness placement:** extend existing cargo lib-test target — new file `src/backend/i686/assembler/encoder/encode_mov_infer_size_pbt.rs` + one `#[cfg(test)] mod encode_mov_infer_size_pbt;` line in `encoder/mod.rs` (rung 1).
- **Candidate modules:** encode_mov_infer_size (gp_integer.rs:112) — sole HARD-scope / change-surface target.
- **Skipped modules:** encode_mov and all other gp_integer symbols, plus every other encoder — HARD scope: test only encode_mov_infer_size. (Dispatch path through `InstructionEncoder::encode("mov", …)` is used so the production symbol is reached.)
- **Oracle (strongest):** differential vs llvm-mc `-triple=i686 -show-encoding`; algebraic.metamorphic (`mov` ≡ `movb`/`movw`/`movl` when size is unambiguous); algebraic.invariant (inferred size = `reg_size` of the chosen register operand); negative_error (arity ≠ 2; ambiguous imm→mem; size-mismatched GP regs).
- **Doc contract:** gp_integer.rs:111 `"Handle unsuffixed mov from inline asm - infer size from operands"` — asserted fingerprint 7b2e4c91. Body: arity check → size from first Register else second else 4 → `encode_mov(ops, size)`.
- **Dispatch path:** `encode_mnemonic` routes `"mov"` → `encode_mov_infer_size` (mod.rs:163). Suffixed `movb`/`movw`/`movl` go to `encode_mov` with fixed size.
- **Seed:** encode_mov_cr_pbt.rs:321 (`encode_mov_cr_diff_mnemonic_aliases` already checks unsuffixed `mov` for CR forms) — generalize to GP size inference.
- **Contract-surface sweep:** 1 round via `coverage_gaps` after first full run — file-level (Rust proptest suite; coverage_gaps reported no .gcda/.profraw and OH-style binaries NOT LINKED; campaign evidence is cargo test execution of the production symbol via InstructionEncoder::encode("mov")).

## Module: encode_mov_infer_size
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- [x] Contract-surface sweep (1 round via coverage_gaps; file-level evidence)
