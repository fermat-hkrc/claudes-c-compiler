# PBT Campaign: encode_fcvt_rounding

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:223 Floating point lists `fcvtzs`, `fcvtzu`, `fcvtas`/`au`/`ns`/`nu`/`ms`/`mu`/`ps`/`pu`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:440-453 dispatch of those 10 mnemonics to encode_fcvt_rounding, and the purpose comment at fp_scalar.rs:179-183:
  - Encoding: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd
  - sf: 0=W dest, 1=X dest
  - ftype: 00=S source, 01=D source
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `fcvtzs w0, s1` = 0x1e380020). Callers: encoder/mod.rs:440-453; codegen/cast_ops.rs emits `fcvtzs`/`fcvtzu`. The producing statements at fp_scalar.rs:196-201 are not independent Doc evidence; the encoding comment at 180-183 is a purpose comment.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_smulh_kat_llvm_mc_x0_x1_x2` (unchanged project harness) → `1 passed; 0 failed; 1859 filtered out (0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_fcvt_rounding_pbt` at the bottom of `src/backend/arm/assembler/encoder/fp_scalar.rs` (same convention as neighbouring encode_smulh_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_fcvt_rounding (single-symbol campaign on fp_scalar.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_fcvt_rounding; other fp_scalar.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_fcvt_rounding is a pure function (operands, rmode, opcode -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_fcvt_rounding):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree FCVT* integer decoder). Sibling encode_int_to_float / encode_scvtf / encode_ucvtf rejected as differential (same-job gate fails: those are integer-to-float, opcode 010/011, different ARM class). Sibling encode_fcvt_precision rejected (float-to-float precision, not float-to-integer). NEON encode_neon_float_two_misc rejected (vector/SIMD-scalar form, different encoding). Weaker available: algebraic.metamorphic (sf / ftype / rmode / opcode / Rd / Rn field independence), algebraic.invariant (ARM bitfield layout), negative_error (arity / extra / SP dest / GP source / FP dest / QVB / non-reg / invalid name / H vs fp16). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar `fcvtzs` etc.; mapping `[Reg(Wd|Xd), Reg(Sn|Dn)]` + (rmode,opcode) <-> `fcvt* Wd|Xd, Sn|Dn`.
- **Seeds:** data_processing.rs encode_smulh_pbt (llvm-mc differential, ARM fields, extra/wrong-type/SP/FP negative contracts). No dedicated encode_fcvt_rounding unit tests.

## Module: encode_fcvt_rounding
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit added encode_fcvt_rounding_neg_nonreg and encode_fcvt_rounding_neg_invalid_name (both passing). Closed: tier round spent and documented surface covered.
