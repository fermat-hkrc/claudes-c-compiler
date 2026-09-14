# PBT Campaign: encode_int_to_float

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:223 Floating point lists `ucvtf`/`scvtf`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:454-459 dispatch of scalar `ucvtf`/`scvtf` to encode_ucvtf/encode_scvtf which wrap encode_int_to_float (vector RegArrangement goes to encode_neon_float_two_misc), and the purpose comment at fp_scalar.rs:211-215:
  - SCVTF/UCVTF: integer-to-float conversion
  - Encoding: sf 00 11110 ftype 1 00 opcode 000000 Rn Rd
  - sf: 0=W source, 1=X source; ftype: 00=S dest, 01=D dest; opcode: 010=signed, 011=unsigned
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `scvtf s0, w1` = 0x1e220020). Callers: encoder/mod.rs:454-459; codegen/cast_ops.rs:53-66 emits `scvtf`/`ucvtf` d0/s0, x0. The producing statements at fp_scalar.rs:217-231 are not independent Doc evidence; the encoding comment at 211-215 is a purpose comment.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_fp_1src_kat_llvm_mc_frintn_s0_s1` (unchanged project harness) → `1 passed; 0 failed; 1899 filtered out (0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_int_to_float_pbt` at the bottom of `src/backend/arm/assembler/encoder/fp_scalar.rs` (same convention as neighbouring encode_fcvt_rounding_pbt / encode_fp_1src_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_int_to_float (single-symbol campaign on fp_scalar.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_int_to_float; other fp_scalar.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_int_to_float is a pure function (operands, is_signed -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_int_to_float):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree SCVTF/UCVTF decoder). Sibling encode_fcvt_rounding rejected as differential (same-job gate fails: float-to-integer, opposite conversion). Sibling encode_scvtf / encode_ucvtf rejected (thin wrappers that call this function — not independent). Sibling encode_neon_float_two_misc rejected (vector/SIMD-scalar form, different encoding class 0x5e21d820 for `scvtf s0, s1`). Sibling encode_fcvt_precision rejected (float-to-float precision). Weaker available: algebraic.metamorphic (sf/ftype/opcode/Rd/Rn), algebraic.invariant (ARM fields), negative_error (arity / extra / SP / wrong type). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar `scvtf`/`ucvtf`; mapping `[Reg(Sd|Dd), Reg(Wn|Xn)]` + is_signed <-> `scvtf|ucvtf Sd|Dd, Wn|Xn`.
- **Seeds:** fp_scalar.rs encode_fcvt_rounding_pbt (llvm-mc differential, ARM fields, extra/wrong-type/SP/FP negative contracts). No dedicated encode_int_to_float unit tests.

## Module: encode_int_to_float
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_int_to_float (arity / extra / SP src / wrong types / half ftype / nonreg / invalid-name / uppercase / W vs X / S vs D / signed vs unsigned). Added encode_int_to_float_neg_nonreg and encode_int_to_float_neg_invalid_name (both passing). Closed: tier round spent and documented surface covered.
