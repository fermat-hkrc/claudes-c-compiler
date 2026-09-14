# PBT Campaign: encode_uxtw

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README ("accepts the same textual assembly that GCC's gas would consume"), encoder/mod.rs ("Encodes AArch64 instructions into 32-bit machine code words"), encoder/mod.rs:296 `"uxtw" => encode_uxtw(operands)`, README.md:217 Extensions table listing `uxtw`, and ARM ARM C6 UXTW alias of UBFM Xd, Xn, #0, #31 (sf=1 opc=10 N=1 immr=0 imms=31 Rn Rd; assembler syntax UXTW Xd, Wn only). Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. The producing comment at data_processing.rs:882-886 ("UXTW is MOV Wd, Wn ... Or: UBFM Xd, Xn, #0, #31") is the producing statement, not independent Doc evidence; gas/llvm-mc emit UBFM not ORR/MOV.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_sxtw_kat` (unchanged project harness, existing KATs in data_processing.rs) → `4 passed, 1712 filtered out (1 suite, 0.03s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_uxtw_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (same convention as neighbouring encoder PBT modules encode_sxtw_pbt / encode_sxth_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_uxtw (single-symbol campaign on data_processing.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_uxtw; other data_processing.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_uxtw is a pure function (operands -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_uxtw):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree UXTW decoder). Sibling encode_sxtw rejected (same-job gate fails: SBFM signed vs UBFM unsigned). Sibling encode_uxth / encode_uxtb rejected (imms=15 / imms=7). Sibling encode_ubfm rejected as independent differential (shared get_reg / same crate; alias equality is algebraic.metamorphic). Weaker available: algebraic.metamorphic (UBFM #0,#31 alias), algebraic.invariant (ARM bitfield field layout), negative_error (arity / extra / Wd / SP / FP / non-reg / invalid name). SUT-boundary: internal-helper of the GNU-style assembler; mapping `[Reg(Xd), Reg(Wn)]` <-> `uxtw Xd, Wn`.
- **Seeds:** data_processing.rs encode_sxtw_pbt (same extension class; extra/Wd/SP/FP negative contracts and SBFM alias). No dedicated encode_uxtw unit tests.

## Module: encode_uxtw
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
