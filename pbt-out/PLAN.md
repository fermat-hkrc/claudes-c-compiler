# PBT Campaign: encode_double_shift (i686)

## Scan findings
- **Spec:** (none found — symbol-targeted campaign via `--func encode_double_shift`)
- **Test layout:** Project-owned Rust module tests under `src/backend/i686/assembler/encoder/*_pbt.rs`, registered with `#[cfg(test)] mod ...` in `encoder/mod.rs`; runner is `cargo test --lib <filter>`. Framework: proptest (dev-dep in Cargo.toml).
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1 -- --test-threads=1` → pass (prebuilt SUT; log pbt-out/build.log). Canonical contract form: `cargo test --lib <target> -- --test-threads=1`.
- **Harness placement:** extend existing test target (rung 1) — new file `src/backend/i686/assembler/encoder/encode_double_shift_pbt.rs` + one `#[cfg(test)] mod encode_double_shift_pbt;` line in `mod.rs`.
- **Candidate modules:** encode_double_shift in `src/backend/i686/assembler/encoder/gp_integer.rs:920` (HARD scope sole target)
- **Skipped modules:** all other functions in gp_integer.rs and outside scope — HARD scope is `encode_double_shift` only. HEAD changes outside this path skipped.

### Target analysis
- Doc contract: (none on the function itself). Dispatch: `mod.rs:250-251` shldl/shld→0xA4, shrdl/shrd→0xAC size=4. README lists shld/shrd. Intel SDM Imm8|CL × r/m32 × r32.
- Body: Imm+Reg+Reg and CL+Reg+Reg only; `_size` unused; Imm `as u8` truncates; no GP/width gate; no memory arm.

### Oracle classification
- Differential primary (llvm-mc); algebraic.invariant + metamorphic; negative_error for arity/class/Imm8/mem.

### Contract-surface sweep
- [x] coverage_gaps round 1 (standard tier) — after first full run

## Module: encode_double_shift
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
