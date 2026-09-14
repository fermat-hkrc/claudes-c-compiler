# PBT Campaign: encode_prfm

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:243 Prefetch `prfm`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:917 (`"prfm" => encode_prfm(operands)`), and ARM ARM PRFM:
  - PRFM (immediate): size=11 V=0 opc=10; 1111 1001 10 imm12 Rn Rt; pimm = imm12*8 in [0, 32760]; syntax PRFM <prfop>|#imm5, [Xn|SP{, #pimm}]
  - PRFM (register): size=11 V=0 opc=10; 11 111 0 00 10 1 Rm option S 10 Rn Rt; option UXTW/LSL/SXTW/SXTX; S amount 0 or 3
  - PRFM (literal): opc=11; 11 011 0 00 imm19 Rt — function comment at load_store.rs:759-761 says "not yet supported"
  - Rt is the 5-bit prefetch operation (named prfop or #0..31), not a GPR dest; Rn is Xn|SP (not W, not XZR)
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. Callers: encoder/mod.rs:917 dispatch; inline_asm.rs mentions prfm/prefetch. The producing statements at load_store.rs:726-782 are not independent Doc evidence; the encoding comments at 722-725 and 755/764 are purpose comments.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_ldtr_sized_kat_llvm_mc_ldtrb_w0_x1` (unchanged project harness) → `1 passed, 1811 filtered out (1 suite, 0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_prfm_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (same convention as neighbouring encode_ldtr_sized_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_prfm (single-symbol campaign on load_store.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_prfm; other load_store.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_prfm is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_prfm):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree PRFM decoder). Sibling encode_ldr_str rejected as differential (same-job gate fails: LDR/STR auto-size GPR/SIMD dest; PRFM uses Rt as prfop and opc=10). encode_prfop is a name-to-imm5 helper called by encode_prfm, not a same-job sibling. Weaker available: algebraic.invariant (ARM bitfield layout), algebraic.metamorphic (Rt/Rn/imm12 independence), negative_error (arity / extra / unknown prfop / W-base / XZR-base / FP / offset range / pre/post / W-index / bad shift). SUT-boundary: internal-helper of the GNU-style assembler; mapping `[Symbol(prfop)|Imm(0..31), Mem{Xn|SP, pimm}]` <-> `prfm <prfop>|#imm5, [Xn|SP{, #pimm}]`.
- **Seeds:** load_store.rs encode_ldtr_sized_pbt / encode_ldrsw_pbt (offset range / extra / W-base / XZR / SIMD negative contracts, ARM field unpack, llvm-mc differential). No dedicated encode_prfm unit tests.

## Module: encode_prfm
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit added encode_prfm_neg_w_index (failing), encode_prfm_neg_bad_shift (failing), encode_prfm_neg_bad_prfop_and_name (passing). Closed: tier round spent and documented surface covered.
