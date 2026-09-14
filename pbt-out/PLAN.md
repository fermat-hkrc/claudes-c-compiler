# PBT Campaign: encode_ldtr_sized

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:340-343 (`"ldtrh"|"sttrh"|"ldtrb"|"sttrb" => encode_ldtr_sized(...)`), and ARM ARM LDTRB/LDTRH/STTRB/STTRH (unprivileged unscaled immediate):
  - Encoding: size 111 V=0 00 opc 0 imm9 10 Rn Rt
  - size=00 byte (ldtrb/sttrb), size=01 half (ldtrh/sttrh)
  - opc=01 load, opc=00 store
  - bits[11:10]=10 distinguishes unprivileged from LDUR/STUR (00)
  - Syntax: LDTRB/LDTRH/STTRB/STTRH Wt, [Xn|SP{, #simm}]; Rt is Wt (31=WZR), never Xt/SP/WSP/SIMD; Rn is Xn|SP (not W, not XZR)
  - simm9 in [-256, 255]; only unscaled Mem form (no pre/post/reg-offset)
  Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. Callers: encoder/mod.rs dispatch only (no codegen sites emit ldtrb/ldtrh). The producing statements at load_store.rs:292-309 are not independent Doc evidence.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_ldur_stur_kat_llvm_mc_ldtr_x0_x1` (unchanged project harness, existing KAT in load_store.rs encode_ldur_stur_pbt) → `1 passed, 1789 filtered out (1 suite, 0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_ldtr_sized_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (same convention as neighbouring encode_ldur_stur_pbt / encode_ldrsw_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_ldtr_sized (single-symbol campaign on load_store.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_ldtr_sized; other load_store.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_ldtr_sized is a pure function (operands, is_load, size -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_ldtr_sized):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree LDTRB decoder). Sibling encode_ldur_stur rejected as differential (same-job gate fails: LDUR/STUR/LDTR/STTR auto-size from W/X/SIMD; this helper is explicit size=00/01 for ldtrb/ldtrh/sttrb/sttrh). Sibling encode_ldr_str rejected (LDRB/STRB unsigned/unscaled/pre/post, different op2). Weaker available: algebraic.metamorphic (size bit 30, opc bit 22, Rt/Rn/imm9 independence), algebraic.invariant (ARM bitfield layout), negative_error (arity / extra / Xt dest / SP-as-Rt / WSP / FP / W-base / XZR-base / offset range / pre/post/reg-offset). SUT-boundary: internal-helper of the GNU-style assembler; mapping `[Reg(Wt), Mem{Xn|SP, simm}]` <-> `{ldtrb|ldtrh|sttrb|sttrh} Wt, [Xn|SP{, #simm}]`.
- **Seeds:** load_store.rs encode_ldur_stur_pbt (offset range / extra / SP / W-base / XZR / SIMD-LDTR negative contracts and ARM field unpack). No dedicated encode_ldtr_sized unit tests.

## Module: encode_ldtr_sized
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit added encode_ldtr_sized_neg_bad_form (passing) and encode_ldtr_sized_diff_w31_alias (passing). Closed: tier round spent and documented surface covered.
