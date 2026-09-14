# PBT Campaign: encode_clz

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:240 Bit manipulation lists `clz`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:573-575 dispatch: scalar `clz` -> encode_clz, NEON RegArrangement -> encode_neon_two_misc. Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `clz w0, w1` = 0x5ac01020). Callers: encoder/mod.rs:573-575 only. The producing statements at bitfield.rs:148-155 are not independent Doc evidence. ARM ARM Data-processing (1 source) CLZ: CLZ <Wd>, <Wn> / CLZ <Xd>, <Xn>; encoding sf 1 0 11010110 00000 000100 Rn Rd; register 31 is ZR not SP. llvm-mc/gas reject extra operands, mixed W/X, SP/WSP, FP/SIMD, and too few operands.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_cls_kat_llvm_mc_w0_w1` (unchanged project harness) → `1 passed; 0 failed; 2080 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_clz_pbt` at the bottom of `src/backend/arm/assembler/encoder/bitfield.rs` (same convention as neighbouring encode_cls_pbt / encode_bfi_pbt / encode_bfxil_pbt). Not pbt-native: probe succeeded. Do not rewrite already-covered encode_cls_pbt / encode_bfi_pbt / encode_bfxil_pbt modules.
- **Candidate modules:** encode_clz (single-symbol campaign on bitfield.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_clz; other bitfield.rs symbols stay out of scope in FUNCTION_INDEX except previously covered encode_bfi / encode_bfxil / encode_cls
- **State machine:** not applicable — encode_clz is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_clz):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree CLZ decoder). Sibling encode_cls rejected as differential (same-job gate: CLS is count-leading-sign-bits, opcode 000101 not 000100). Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (Rd/Rn field independence; W vs X sf bit), negative_error (arity / extra / SP / mixed width / FP / nonreg / invalid-name). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar `clz`; mapping `[Reg(Rd), Reg(Rn)]` <-> `clz Rd, Rn`.
- **Seeds:** bitfield.rs encode_cls_pbt (llvm-mc differential, ARM fields, extra/SP/mixed/FP negative contracts). No dedicated encode_clz unit tests. README.md:240 lists `clz`.

## Module: encode_clz
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_clz (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings). Added encode_clz_diff_alt_spellings, encode_clz_neg_nonreg, encode_clz_neg_invalid_name (all passing). Closed: tier round spent and documented surface covered.
