# PBT Campaign: encode_neon_dup

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:12 ("accepts the same textual assembly that GCC's gas would consume"), README.md:234 NEON insert/move lists `dup` (element/GPR). encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words). Dispatch: encoder/mod.rs:670 `"dup" => encode_neon_dup(operands)`. Purpose comments neon.rs:496-527: GPR form `DUP Vd.T, Rn` encoding `0 Q 0 01110 000 imm5 0 0001 1 Rn Rd`; element form `DUP Vd.T, Vn.Ts[index]` encoding `0 Q 0 01110 000 imm5 0 0000 1 Rn Rd`. ARM ARM Advanced SIMD DUP (general) and DUP (element); T in {8B,16B,4H,8H,2S,4S,2D}; GPR source Wn for T!=2D and Xn for T=2D; element index ranges b[0-15], h[0-7], s[0-3], d[0-1] with dest T matching element size. Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. The producing statements at neon.rs:528-545 are not independent Doc evidence.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_neon_float_two_misc_kat_llvm_mc_fneg` (unchanged project harness) → `1 passed; 0 failed; 0 ignored; 0 measured; 2529 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_neon_dup_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (same convention as neighbouring encode_neon_float_two_misc_pbt / encode_neon_rbit_pbt). Not pbt-native: probe succeeded. Do not rewrite already-covered encode_*_pbt modules in this file.
- **Candidate modules:** encode_neon_dup (single-symbol campaign on neon.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_neon_dup; other neon.rs symbols stay out of scope in FUNCTION_INDEX except previously covered encode_* functions
- **State machine:** not applicable — encode_neon_dup is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_neon_dup):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree DUP decoder; UMOV is the reverse data-path not a bit-exact inverse of the encoding). Sibling encode_neon_umov / encode_neon_ins rejected as differential (same-job gate: UMOV opcode 001111, INS 000111/element-to-element, different ARM encodings). Weaker available: algebraic.metamorphic (Rd/Rn/Q/opcode-bit11), algebraic.invariant (ARM fields), negative_error (arity / extra / index / GPR width / T mismatch). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for `dup`; mapping `[RegArrangement(Vd,T), Reg(Wn|Xn)]` <-> `dup Vd.T, Wn|Xn` and `[RegArrangement(Vd,T), RegLane(Vn,Ts,i)]` <-> `dup Vd.T, Vn.Ts[i]`.
- **Seeds:** neon.rs encode_neon_float_two_misc_pbt (llvm-mc differential, ARM fields, extra/mismatch/non-v/invalid-name negatives). No dedicated encode_neon_dup unit tests.

## Module: encode_neon_dup
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_neon_dup (invalid RegLane names / unsupported elem_size). Added encode_neon_dup_neg_elem_invalid (passing). Closed: tier round spent and documented surface covered.
