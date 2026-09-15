# PBT Campaign: encode_ldrs

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:12 ("accepts the same textual assembly that GCC's gas would consume"), README.md:221 Loads/Stores table lists ldrsb, ldrsh. encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words). Dispatch: encoder/mod.rs:334 `"ldrsb" => encode_ldrs(operands, 0b00)`; encoder/mod.rs:335 `"ldrsh" => encode_ldrs(operands, 0b01)`. Purpose comment load_store.rs:383 "LDRSB/LDRSH: sign-extending byte/halfword loads". ARM ARM LDRSB/LDRSH unsigned: size 111 V=0 01 opc imm12 Rn Rt (size=00 byte / 01 half; opc=10 Xt / 11 Wt; pimm=imm12*scale, scale=1 or 2). Unscaled LDURSB/LDURSH: bits[25:24]=00 bits[11:10]=00 simm9 in [-256,255]. Pre bits[11:10]=11, post bits[11:10]=01. Register offset: bit21=1 option S bits[11:10]=10; shift 0 (byte) or 0/1 (half). Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. The producing statements at load_store.rs:384-449 are not independent Doc evidence.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_ldrsw_kat_llvm_mc_x0_x1` (unchanged project harness) → `2 passed; 0 failed; 0 ignored; 0 measured; 2544 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_ldrs_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (same convention as neighbouring encode_ldrsw_pbt / encode_cas_pbt). Not pbt-native: probe succeeded. Do not rewrite already-covered encode_*_pbt modules in this file.
- **Candidate modules:** encode_ldrs (single-symbol campaign on load_store.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_ldrs; other load_store.rs symbols stay out of scope in FUNCTION_INDEX except previously covered encode_* functions
- **State machine:** not applicable — encode_ldrs is a pure function (operands, size -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_ldrs):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree LDRSB/LDRSH decoder). Sibling encode_ldrsw / encode_ldr_str / encode_ldur_stur rejected as differential (same-job gate: LDRSW is 32-bit sign-extend to Xt only; LDR/STR different opc/size; LDUR/STUR is generic unscaled). Weaker available: algebraic.metamorphic (Rt/Rn/imm12/opc/size), algebraic.invariant (ARM fields), negative_error (arity / extra / SP / FP / W-base / XZR-base / offset range / W-index / writeback overlap). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for `ldrsb`/`ldrsh`; mapping `[Reg(Wt|Xt), Mem{Xn|SP, imm}]` <-> `ldrsb|ldrsh Wt|Xt, [Xn|SP{, #imm}]`.
- **Seeds:** load_store.rs encode_ldrsw_pbt (llvm-mc differential, ARM fields, extra/range/invalid-reg negatives). No dedicated encode_ldrs unit tests.

## Module: encode_ldrs
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_ldrs (invalid dest/base names). Added encode_ldrs_neg_invalid_name (passing). Closed: tier round spent and documented surface covered.
