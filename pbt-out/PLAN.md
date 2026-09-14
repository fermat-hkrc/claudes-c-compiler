# PBT Campaign: encode_bfi

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:216 Bit fields lists `bfi`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:892 `"bfi" => encode_bfi(operands)`. Purpose comment at bitfield.rs:103: Encode BFI Rd, Rn, #lsb, #width -> BFM Rd, Rn, #(-lsb mod width_reg), #(width-1). Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding (gas aarch64-linux-gnu-as agrees on `bfi w0, w1, #0, #1` = 0x33000020). Callers: encoder/mod.rs:892 only. The producing statements at bitfield.rs:105-115 are not independent Doc evidence; the alias comment at 103 is a purpose comment. ARM ARM Bitfield Move (BFM) / BFI alias: BFI Wd, Wn, #lsb, #width <=> BFM Wd, Wn, #(-lsb MOD 32), #(width-1) with 0 <= lsb < 32 and 1 <= width <= 32-lsb (64-bit analog with 64); encoding sf 01 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. llvm-mc rejects width 0, lsb/width out of range, mixed W/X, SP/WSP, FP, extra operands, and too few operands.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_smulh_kat_llvm_mc_x0_x1_x2` (unchanged project harness) → `1 passed; 1985 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_bfi_pbt` at the bottom of `src/backend/arm/assembler/encoder/bitfield.rs` (same convention as neighbouring encode_smulh_pbt / encode_neon_aes_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_bfi (single-symbol campaign on bitfield.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_bfi; other bitfield.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_bfi is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_bfi):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree BFI decoder). Sibling encode_bfm rejected as differential (same-job gate: BFM is the raw immr/imms form, different assembly syntax; used only as algebraic alias after ARM mapping). Sibling encode_ubfiz / encode_sbfiz rejected (UBFM/SBFM opc, different instruction). Weaker available: algebraic.metamorphic (BFI alias of BFM; Rd/Rn field independence), algebraic.invariant (ARM fields), negative_error (arity / extra / SP / mixed width / bounds / FP / non-reg / invalid name). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for `bfi`; mapping `[Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)]` <-> `bfi Rd, Rn, #lsb, #width`.
- **Seeds:** data_processing.rs encode_smulh_pbt (llvm-mc differential, ARM fields, extra/SP/width negative contracts). No dedicated encode_bfi unit tests. README.md:216 lists `bfi`.

## Module: encode_bfi
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_bfi (arity / extra / SP / mixed W-X / FP / lsb-width / nonreg / invalid-name / alt-spellings). Added encode_bfi_neg_invalid_name (passing). Closed: tier round spent and documented surface covered.
