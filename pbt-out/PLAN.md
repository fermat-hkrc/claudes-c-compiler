# PBT Campaign: encode_fcmp

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:223 Floating point lists `fcmp`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:439 `"fcmp" => encode_fcmp(operands)`, and the purpose comments at fp_scalar.rs:164-175:
  - FCMP Dn, #0.0 (Imm(0) or missing second operand currently takes this path)
  - FCMP Dn, Dm: 0 00 11110 ftype 1 Rm 00 1000 Rn 00 000
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `fcmp s0, s1` = 0x1e212000). Callers: encoder/mod.rs:439; codegen/comparison.rs:15-19 emits `fcmp s0, s1` / `fcmp d0, d1`. The producing statements at fp_scalar.rs:166-175 are not independent Doc evidence; the encoding comment at 171 is a purpose comment.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_int_to_float_kat_llvm_mc` (unchanged project harness) → `8 passed; 0 failed; 1914 filtered out (0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_fcmp_pbt` at the bottom of `src/backend/arm/assembler/encoder/fp_scalar.rs` (same convention as neighbouring encode_int_to_float_pbt / encode_fcvt_rounding_pbt / encode_fp_1src_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_fcmp (single-symbol campaign on fp_scalar.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_fcmp; other fp_scalar.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_fcmp is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_fcmp):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree FCMP decoder). Sibling encode_fp_arith rejected (same-job gate fails: 3-operand FP arithmetic). Sibling encode_neon_float_cmp_zero rejected (vector/SIMD compare-to-zero, different class). fccmp is a different mnemonic (conditional compare with NZCV/cond). fcmpe is not dispatched. Weaker available: algebraic.metamorphic (ftype/Rn/Rm/opc), algebraic.invariant (ARM fields), negative_error (arity / extra / wrong type / nonzero imm). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar `fcmp`; mapping `[Reg(Sn|Dn), Reg(Sm|Dm)]` <-> `fcmp Sn|Dn, Sm|Dm` and `[Reg(Sn|Dn), Imm(0)]` <-> `fcmp Sn|Dn, #0.0` (Operand has no float-immediate variant; purpose comment documents Imm(0) as #0.0).
- **Seeds:** fp_scalar.rs encode_int_to_float_pbt / encode_fp_1src_pbt (llvm-mc differential, ARM fields, extra/wrong-type/half negative contracts). No dedicated encode_fcmp unit tests. Codegen seed: comparison.rs:15-19 `fcmp s0, s1` / `fcmp d0, d1`.

## Module: encode_fcmp
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_fcmp (arity / extra / mixed S-D / GPR / QVB / SP / half ftype / nonzero imm / nonreg / invalid-name). Added encode_fcmp_neg_nonzero_imm, encode_fcmp_neg_nonreg, and encode_fcmp_neg_invalid_name (all passing). Closed: tier round spent and documented surface covered.
