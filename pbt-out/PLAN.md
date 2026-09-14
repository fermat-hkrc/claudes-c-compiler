# PBT Campaign: encode_cls

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:240 Bit manipulation lists `cls`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:570-572 dispatch: scalar `cls` -> encode_cls, NEON RegArrangement -> encode_neon_two_misc. Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `cls w0, w1` = 0x5ac01420). Callers: encoder/mod.rs:570-572 only. The producing statements at bitfield.rs:159-163 are not independent Doc evidence. ARM ARM Data-processing (1 source) CLS: CLS <Wd>, <Wn> / CLS <Xd>, <Xn>; encoding sf 1 0 11010110 00000 000101 Rn Rd; register 31 is ZR not SP. llvm-mc/gas reject extra operands, mixed W/X, SP/WSP, FP/SIMD, and too few operands.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_bfxil_kat_llvm_mc_w0_w1_lsb0_width1` (unchanged project harness) → `1 passed; 0 failed; 2059 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_cls_pbt` at the bottom of `src/backend/arm/assembler/encoder/bitfield.rs` (same convention as neighbouring encode_bfi_pbt / encode_bfxil_pbt). Not pbt-native: probe succeeded. Do not rewrite already-covered encode_bfi_pbt / encode_bfxil_pbt modules.
- **Candidate modules:** encode_cls (single-symbol campaign on bitfield.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_cls; other bitfield.rs symbols stay out of scope in FUNCTION_INDEX except previously covered encode_bfi / encode_bfxil
- **State machine:** not applicable — encode_cls is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_cls):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree CLS decoder). Sibling encode_clz rejected as differential (same-job gate: CLZ is count-leading-zeros, opcode 000100 not 000101). Weaker available: algebraic.invariant (ARM field unpack), algebraic.metamorphic (Rd/Rn field independence; W vs X sf bit), negative_error (arity / extra / SP / mixed width / FP / nonreg / invalid-name). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for scalar `cls`; mapping `[Reg(Rd), Reg(Rn)]` <-> `cls Rd, Rn`.
- **Seeds:** bitfield.rs encode_bfi_pbt / encode_bfxil_pbt (llvm-mc differential, ARM fields, extra/SP/mixed/FP negative contracts). No dedicated encode_cls unit tests. README.md:240 lists `cls`.

## Module: encode_cls
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_cls (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings). Added encode_cls_diff_alt_spellings, encode_cls_neg_nonreg, encode_cls_neg_invalid_name (all passing). Closed: tier round spent and documented surface covered.
