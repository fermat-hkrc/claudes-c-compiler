# PBT Campaign: encode_fabs

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:223 Floating point lists `fabs`. encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words). Dispatch: encoder/mod.rs:408-411 scalar `fabs` (non-RegArrangement) => encode_fabs. Purpose comment fp_scalar.rs:98: "FABS: 0 00 11110 ftype 1 0000 01 10000 Rn Rd". ARM ARM Floating-point data-processing (1 source) FABS: M=0 S=0 11110 ftype 1 opcode=000001 10000 Rn Rd; ftype 00=S, 01=D, 11=H; two matching FP registers; extra operands rejected. Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. The producing statements at fp_scalar.rs:94-100 are not independent Doc evidence.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_fp_1src_kat` (unchanged project harness) → `7 passed; 0 failed; 0 ignored; 0 measured; 2448 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_fabs_pbt` at the bottom of `src/backend/arm/assembler/encoder/fp_scalar.rs` (same convention as neighbouring encode_fp_1src_pbt / encode_fp_arith_pbt). Not pbt-native: probe succeeded. Do not rewrite already-covered encode_*_pbt modules in this file.
- **Candidate modules:** encode_fabs (single-symbol campaign on fp_scalar.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_fabs; other fp_scalar.rs symbols stay out of scope in FUNCTION_INDEX except previously covered encode_* functions
- **State machine:** not applicable — encode_fabs is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_fabs):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree FABS decoder). Sibling encode_fneg / encode_fsqrt rejected as differential (same-job gate: different ARM opcodes 000010 / 000011). Sibling encode_fp_1src rejected (FRINT* opcodes). Sibling encode_neon_float_two_misc rejected (vector/SIMD form, dispatch only on RegArrangement). Weaker available: algebraic.metamorphic (Rd/Rn/ftype), algebraic.invariant (ARM fields), negative_error (arity / extra / wrong type / nonreg / invalid name). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar fabs; mapping `[Reg(Sd|Dd|Hd), Reg(Sn|Dn|Hn)]` <-> `fabs Sd|Dd|Hd, Sn|Dn|Hn`.
- **Seeds:** fp_scalar.rs encode_fp_1src_pbt (same encoding class, 2-operand llvm-mc differential, ARM fields, extra/mismatch/nonreg negatives). No dedicated encode_fabs unit tests.

## Module: encode_fabs
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_fabs (invalid names). Added encode_fabs_neg_invalid_name (passing). Closed: tier round spent and documented surface covered.
