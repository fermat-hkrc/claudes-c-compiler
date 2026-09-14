# PBT Campaign: encode_fmadd_fmsub

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:223 Floating point lists `fmadd`, `fmsub`. encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words). Dispatch: encoder/mod.rs:435-436 `"fmadd" => encode_fmadd_fmsub(operands, false)`, `"fmsub" => encode_fmadd_fmsub(operands, true)`. Purpose comment fp_scalar.rs:127-128: "Encode FMADD/FMSUB: Rd = Ra +/- (Rn * Rm)" / "Format: 0 00 11111 ftype 0 Rm o1 Ra Rn Rd". ARM ARM Floating-point data-processing (3 source): M=0 S=0 11111 ftype o1 Rm o0 Ra Rn Rd; FMADD o1=0 o0=0, FMSUB o1=0 o0=1; ftype 00=S, 01=D, 11=H; four matching FP registers; extra operands rejected. Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. The producing statements at fp_scalar.rs:131-141 are not independent Doc evidence.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_fabs_kat` (unchanged project harness) → `6 passed; 0 failed; 0 ignored; 0 measured; 2467 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_fmadd_fmsub_pbt` at the bottom of `src/backend/arm/assembler/encoder/fp_scalar.rs` (same convention as neighbouring encode_fp_arith_pbt / encode_fabs_pbt). Not pbt-native: probe succeeded. Do not rewrite already-covered encode_*_pbt modules in this file.
- **Candidate modules:** encode_fmadd_fmsub (single-symbol campaign on fp_scalar.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_fmadd_fmsub; other fp_scalar.rs symbols stay out of scope in FUNCTION_INDEX except previously covered encode_* functions
- **State machine:** not applicable — encode_fmadd_fmsub is a pure function (operands, is_sub -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_fmadd_fmsub):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree FMADD/FMSUB decoder). Sibling encode_fnmadd_fnmsub rejected as differential (same-job gate: o1=1 negated fused class, different mnemonics). Sibling encode_fp_arith rejected (2-source FP class). Sibling encode_madd rejected (integer MADD). Weaker available: algebraic.metamorphic (Rd/Rn/Rm/Ra/ftype/o0), algebraic.invariant (ARM fields), negative_error (arity / extra / wrong type / nonreg / invalid name). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar fmadd/fmsub; mapping `[Reg(Sd|Dd|Hd), Reg(Sn|Dn|Hn), Reg(Sm|Dm|Hm), Reg(Sa|Da|Ha)]` + is_sub <-> `fmadd|fmsub Sd|Dd|Hd, Sn|Dn|Hn, Sm|Dm|Hm, Sa|Da|Ha`.
- **Seeds:** fp_scalar.rs encode_fp_arith_pbt (2-source FP llvm-mc differential, ARM fields, extra/mismatch/nonreg negatives) and encode_fabs_pbt (1-source FP, half-precision ftype). No dedicated encode_fmadd_fmsub unit tests.

## Module: encode_fmadd_fmsub
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_fmadd_fmsub (invalid names). Added encode_fmadd_fmsub_neg_invalid_name (passing). Closed: tier round spent and documented surface covered.
