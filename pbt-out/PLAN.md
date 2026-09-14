# PBT Campaign: encode_fp_1src

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:223 Floating point lists `frintn`/`p`/`m`/`z`/`a`/`x`/`i`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:414-434 dispatch of those 7 mnemonics to encode_fp_1src (vector RegArrangement goes to encode_neon_float_two_misc), and the purpose comment at fp_scalar.rs:115-116:
  - Encode FP 1-source ops: FRINTN/P/M/Z/A/X/I
  - Format: 0 00 11110 ftype 1 opcode 10000 Rn Rd
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `frintn s0, s1` = 0x1e244020). Callers: encoder/mod.rs:414-434. The producing statements at fp_scalar.rs:118-126 are not independent Doc evidence; the encoding comment at 115-116 is a purpose comment.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_smulh_kat` (unchanged project harness) → `4 passed, 1877 filtered out (1 suite, 0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_fp_1src_pbt` at the bottom of `src/backend/arm/assembler/encoder/fp_scalar.rs` (same convention as neighbouring encode_fcvt_rounding_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_fp_1src (single-symbol campaign on fp_scalar.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_fp_1src; other fp_scalar.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_fp_1src is a pure function (operands, opcode -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_fp_1src):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree FRINT decoder). Sibling encode_fneg / encode_fabs / encode_fsqrt rejected as differential (same-job gate fails: those are FNEG/FABS/FSQRT with hardcoded opcodes, not FRINT*). Sibling encode_neon_float_two_misc rejected (vector/SIMD-scalar form, different encoding). Sibling encode_fp_arith rejected (2-source FP arith). Weaker available: algebraic.metamorphic (Rd / Rn / ftype / opcode field independence), algebraic.invariant (ARM bitfield layout), negative_error (arity / extra / mixed S-D / GPR / QVB / SP / non-reg / invalid name / H vs fp16). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar `frintn` etc.; mapping `[Reg(Sd|Dd), Reg(Sn|Dn)]` + opcode <-> `frint* Sd|Dd, Sn|Dn`.
- **Seeds:** fp_scalar.rs encode_fcvt_rounding_pbt (llvm-mc differential, ARM fields, extra/wrong-type/SP/FP negative contracts). No dedicated encode_fp_1src unit tests.

## Module: encode_fp_1src
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_fp_1src (arity / extra / mixed S-D / GPR / QVB / SP / half ftype / nonreg / invalid-name / uppercase / S vs D / all 7 FRINT opcodes). Closed: tier round spent and documented surface covered.
