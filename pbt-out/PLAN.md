# PBT Campaign: encode_fcvt_precision

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:223 Floating point lists `fcvt`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:460 `"fcvt" => encode_fcvt_precision(operands)`, and the purpose comments at fp_scalar.rs:236-239:
  - FCVT: float precision conversion (e.g., FCVT Dd, Sn or FCVT Sd, Dn)
  - Encoding: 0 00 11110 ftype 1 0001 opc 10000 Rn Rd
  - ftype: source precision (00=S, 01=D, 11=H)
  - opc: dest precision (00=S, 01=D, 11=H)
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `fcvt d0, s1` = 0x1e22c020). Callers: encoder/mod.rs:460; codegen/cast_ops.rs:74-78 emits `fcvt d0, s0` / `fcvt s0, d0`. The producing statements at fp_scalar.rs:241-268 are not independent Doc evidence; the encoding comment at 237-239 is a purpose comment. ARM ARM Floating-point data-processing (1 source) FCVT: ftype==opc is unallocated; llvm-mc rejects same-precision (`fcvt s0, s1` / `d,d` / `h,h`) as invalid operand.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_fcmp_kat_llvm_mc_s0_s1` (unchanged project harness) → `1 passed; 1944 filtered out (0.01s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_fcvt_precision_pbt` at the bottom of `src/backend/arm/assembler/encoder/fp_scalar.rs` (same convention as neighbouring encode_fcmp_pbt / encode_fcvt_rounding_pbt / encode_fp_1src_pbt / encode_int_to_float_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_fcvt_precision (single-symbol campaign on fp_scalar.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_fcvt_precision; other fp_scalar.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_fcvt_precision is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_fcvt_precision):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree FCVT precision decoder). Sibling encode_fcvt_rounding rejected (same-job gate fails: float-to-integer with rmode/opcode). Sibling encode_int_to_float rejected (integer-to-float). Sibling encode_neon_fcvtl / encode_neon_fcvtn rejected (vector widen/narrow, different class). RISC-V encode_fcvt_fp rejected (different ISA). Weaker available: algebraic.metamorphic (ftype/opc/Rn/Rd), algebraic.invariant (ARM fields), negative_error (arity / extra / same-precision / wrong type / nonreg / invalid name). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar `fcvt`; mapping `[Reg(Sd|Dd|Hd), Reg(Sn|Dn|Hn)]` with dest precision != src precision <-> `fcvt Sd|Dd|Hd, Sn|Dn|Hn`.
- **Seeds:** fp_scalar.rs encode_fcvt_rounding_pbt / encode_fcmp_pbt (llvm-mc differential, ARM fields, extra/wrong-type/half negative contracts). No dedicated encode_fcvt_precision unit tests. Codegen seed: cast_ops.rs:74-78 `fcvt d0, s0` / `fcvt s0, d0`.

## Module: encode_fcvt_precision
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_fcvt_precision (arity / extra / same-precision S,D,H / mixed S-D-H valid / GPR / QVB / SP dest+src / WSP / half ftype / nonreg / invalid-name). Added encode_fcvt_precision_neg_gpr_qvb (passing) and SP-src regression (failing, same SP-as-S bug). Closed: tier round spent and documented surface covered.
