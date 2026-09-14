# PBT Campaign: encode_neon_rbit

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README (GNU gas-compatible textual assembly), encoder/mod.rs (AArch64 32-bit words), neon.rs encode_neon_rbit (RBIT Vd.T, Vn.T; only .8b/.16b), dispatcher encoder/mod.rs rbit NEON vs scalar split, and llvm-mc AArch64 as the independent assembler reference.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_neon_shift_left_imm_kat_llvm_mc_v0_8b_v1_8b` (unchanged project harness, existing KAT in neon.rs) → `1 passed, 1674 filtered out (1 suite, 0.05s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_neon_rbit_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (same convention as neighbouring encoder PBT modules). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_neon_rbit (single-symbol campaign on neon.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_neon_rbit; other neon.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_neon_rbit is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_neon_rbit):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Weaker available: algebraic.metamorphic (Q bit, Rd/Rn fields), algebraic.invariant (ARM two-misc word layout), negative_error (arity, extra, T, mismatch, non-reg). Round-trip rejected (no in-tree RBIT decoder). Sibling encode_rbit (bitfield.rs) rejected (copied NEON formula, same-job/independence gate). SUT-boundary: internal-helper of the GNU-style assembler; mapping `[RegArrangement(Vd,T), RegArrangement(Vn,T)]` <-> `rbit Vd.T, Vn.T`.
- **Seeds:** (none) — no existing tests call encode_neon_rbit

## Module: encode_neon_rbit
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
