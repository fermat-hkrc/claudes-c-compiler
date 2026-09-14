# PBT Campaign: encode_umull

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README ("accepts the same textual assembly that GCC's gas would consume"), encoder/mod.rs ("Encodes AArch64 instructions into 32-bit machine code words"), encoder/mod.rs:258-267 umull dispatch (scalar path vs NEON arrangement), data_processing.rs:641-648 docstring (UMULL Xd, Wn, Wm -> UMADDL Xd, Wn, Wm, XZR), README.md:214 Data Processing table listing umull, and ARM ARM Data-processing (3 source) UMULL alias of UMADDL with Ra=XZR. Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_smull_kat` (unchanged project harness, existing KATs in data_processing.rs) → `4 passed, 1692 filtered out (1 suite, 0.03s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_umull_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (same convention as neighbouring encoder PBT modules encode_smull_pbt / encode_umaddl_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_umull (single-symbol campaign on data_processing.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_umull; other data_processing.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_umull is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_umull):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree UMULL decoder). Sibling encode_smull rejected (same-job gate fails: U=0 signed vs U=1 unsigned). Sibling encode_umaddl rejected as independent differential (shared get_reg / same TU; alias equality is algebraic.metamorphic). Weaker available: algebraic.metamorphic (UMADDL Ra=XZR alias, U bit vs SMULL), algebraic.invariant (ARM 3-source field layout), negative_error (arity / extra / wrong width / SP / FP / non-reg / invalid name). SUT-boundary: internal-helper of the GNU-style assembler; mapping `[Reg(Xd), Reg(Wn), Reg(Wm)]` <-> `umull Xd, Wn, Wm`.
- **Seeds:** data_processing.rs encode_smull_pbt (same 3-source multiply-long class; extra/width/SP/FP negative contracts) and encode_umaddl_pbt alias_umull_xzr KAT. No dedicated encode_umull unit tests.

## Module: encode_umull
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
