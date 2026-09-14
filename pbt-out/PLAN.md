# PBT Campaign: encode_smulh

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:214 Data Processing lists `smulh`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:275 (`"smulh" => encode_smulh(operands)`), data_processing.rs:697-703 purpose comment (`SMULH: 1 00 11011 0 10 Rm 0 11111 Rn Rd`), and ARM ARM Data-processing (3 source):
  - SMULH Xd, Xn, Xm: sf=1 op54=00 11011 op31=010 Rm o0=0 Ra=11111 Rn Rd
  - 64-bit only (no 32-bit W form); register 31 is XZR not SP
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. Callers: encoder/mod.rs:275 dispatch; no codegen emitter of smulh found (umulh is used in i128_ops.rs). The producing statements at data_processing.rs:698-702 are not independent Doc evidence; the encoding comment at 701 is a purpose comment.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_umulh_kat_llvm_mc_x0_x1_x2` (unchanged project harness) → `1 passed, 1837 filtered out (1 suite, 0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_smulh_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (same convention as neighbouring encode_umulh_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_smulh (single-symbol campaign on data_processing.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_smulh; other data_processing.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_smulh is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_smulh):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree SMULH decoder). Sibling encode_umulh rejected as differential (same-job gate fails: UMULH is unsigned op31=110, SMULH is signed op31=010). Weaker available: algebraic.metamorphic (U bit vs UMULH), algebraic.invariant (ARM bitfield layout), negative_error (arity / extra / wrong width / SP / FP / non-reg / invalid name). SUT-boundary: internal-helper of the GNU-style assembler; mapping `[Reg(Xd), Reg(Xn), Reg(Xm)]` <-> `smulh Xd, Xn, Xm`.
- **Seeds:** data_processing.rs encode_umulh_pbt (same Data-processing 3-source class: llvm-mc differential, ARM fields, extra/wrong-width/SP/FP negative contracts). No dedicated encode_smulh unit tests. encode_umulh_pbt calls encode_smulh only as a U-bit metamorphic sibling.

## Module: encode_smulh
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit added encode_smulh_neg_wzr (failing, same wrong-width bug). Closed: tier round spent and documented surface covered.
