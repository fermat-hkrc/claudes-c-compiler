# PBT Campaign: encode_ldrsw

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:333 (`"ldrw" | "ldrsw" => encode_ldrsw(operands)`), README.md:221 Loads/Stores table listing `ldrsw`, and ARM ARM LDRSW / LDURSW:
  - Unsigned offset: size=10 111 V=0 01 opc=10 imm12 Rn Rt; pimm = imm12*4 in [0, 16380]
  - Unscaled LDURSW: size=10 111 V=0 00 opc=10 0 imm9 00 Rn Rt; simm9 in [-256, 255]
  - Pre-index: bits[11:10]=11; post-index: bits[11:10]=01; simm9 in [-256, 255]
  - Register offset: size=10 111 V=0 00 opc=10 1 Rm option S 10 Rn Rt; Xm with lsl/sxtx #0|#2; Wm with uxtw/sxtw #0|#2
  - Literal: opc=10 011 000 imm19 Rt (RelocType::Ldr19 family)
  Syntax: LDRSW Xt, [Xn|SP{, #pimm}]; Rt is Xt (31=XZR), never SP/Wt/SIMD; Rn is Xn|SP (not XZR). Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. Callers: prologue.rs:334 (IrType::I32 => "ldrsw"), peephole.rs ldrsw forwarding, emit.rs:667/1190, variadic.rs ldrsw x2, [x1, #off]. The producing statements at load_store.rs:311-381 are not independent Doc evidence.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_ldaxr_stlxr_kat_llvm_mc_ldaxr_x0_x1` (unchanged project harness, existing KAT in load_store.rs encode_ldaxr_stlxr_pbt) → `1 passed, 1761 filtered out (1 suite, 0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_ldrsw_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (same convention as neighbouring encode_ldaxr_stlxr_pbt / encode_ldur_stur_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_ldrsw (single-symbol campaign on load_store.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_ldrsw; other load_store.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_ldrsw is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_ldrsw):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree LDRSW decoder). Sibling encode_ldr_str rejected as differential (same-job gate fails: unsigned/signed 32/64-bit LDR/STR vs 32-bit sign-extend to X). Sibling encode_ldrs rejected (LDRSB/LDRSH, different size/opc). Sibling encode_ldur_stur rejected (generic unscaled, different opc/size). Weaker available: algebraic.metamorphic (Rt/Rn/imm12 independence, unsigned vs unscaled bit 24, pre vs post bits[11:10]), algebraic.invariant (ARM bitfield layout), negative_error (arity / extra / W-dest / SP-as-Rt / FP / W-base / XZR-base / offset range / W-index-without-extend / unsupported kind). SUT-boundary: internal-helper of the GNU-style assembler; mapping `[Reg(Xt), Mem{Xn|SP, imm}]` <-> `ldrsw Xt, [Xn|SP{, #imm}]`; pre/post/reg-offset analogously.
- **Seeds:** load_store.rs encode_ldur_stur_pbt (offset range / extra / SP / W-base / XZR / FP negative contracts and ARM field unpack). No dedicated encode_ldrsw unit tests.

## Module: encode_ldrsw
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit added encode_ldrsw_diff_alt_spellings (passing), encode_ldrsw_neg_bad_extend_base (passing), and encode_ldrsw_literal_reloc (failing — missing LDRSW literal). Closed: tier round spent and documented surface covered.
