# PBT Campaign: encode_neon_aes

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), README.md:238 NEON crypto lists `aese`, `aesd`, `aesmc`, `aesimc`, encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:747-750 dispatch:
  - `"aese" => encode_neon_aes(operands, 0b00100)`
  - `"aesd" => encode_neon_aes(operands, 0b00101)`
  - `"aesmc" => encode_neon_aes(operands, 0b00110)`
  - `"aesimc" => encode_neon_aes(operands, 0b00111)`
  Purpose comments at neon.rs:1145-1157:
  - Encode NEON AES instructions (AESE, AESD, AESMC, AESIMC)
  - AES instructions: 0100 1110 0010 1000 opcode 10 Rn Rd
  - AESE opcode=00100, AESD=00101, AESMC=00110, AESIMC=00111
  Independent assembler reference: llvm-mc -triple=aarch64 -mattr=+aes -show-encoding (gas aarch64-linux-gnu-as -march=armv8-a+crypto agrees on `aese v0.16b, v1.16b` = 0x4e284820). Callers: encoder/mod.rs:747-750 only; no ARM codegen emitter of aese/aesd/aesmc/aesimc found. The producing statements at neon.rs:1147-1160 are not independent Doc evidence; the encoding comment at 1153-1157 is a purpose comment. ARM ARM Cryptographic AES: AESE/AESD/AESMC/AESIMC Vd.16B, Vn.16B; size must be 00 (otherwise unallocated); llvm-mc rejects non-.16b, extra operands, GPR prefixes, SP, and too few operands.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_neon_rbit_kat_llvm_mc_v0_8b_v1_8b` (unchanged project harness) → `1 passed; 1967 filtered out`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_neon_aes_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (same convention as neighbouring encode_neon_rbit_pbt / encode_neon_tbl_pbt / encode_neon_sli_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_neon_aes (single-symbol campaign on neon.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_neon_aes; other neon.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_neon_aes is a pure function (operands + opcode -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_neon_aes):** Differential vs llvm-mc `-triple=aarch64 -mattr=+aes -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree AES decoder). Sibling encode_neon_rbit rejected (same-job gate fails: Advanced SIMD two-misc RBIT, different class). Sibling encode_neon_eor3 rejected (SHA-3 EOR3 three-register). Sibling encode_neon_two_misc rejected (generic two-misc, different opcode map). x86 AES-NI encode rejected (different ISA). Weaker available: algebraic.metamorphic (Rd/Rn/opcode), algebraic.invariant (ARM fields), negative_error (arity / extra / T / mismatch / non-reg / invalid name / prefix). SUT-boundary: internal-helper of the GNU-style assembler, caller-reachable from encode_instruction for `aese`/`aesd`/`aesmc`/`aesimc`; mapping `[RegArrangement(Vd,16b), RegArrangement(Vn,16b)]` + opcode in {00100,00101,00110,00111} <-> `aese|aesd|aesmc|aesimc Vd.16b, Vn.16b`.
- **Seeds:** neon.rs encode_neon_rbit_pbt (llvm-mc differential, ARM fields, extra/wrong-T/prefix/SP negative contracts). No dedicated encode_neon_aes unit tests. README.md:238 lists the four AES mnemonics.

## Module: encode_neon_aes
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit of encode_neon_aes (arity / extra / T / mismatch / bare Reg / GPR prefix / SP / WSP / nonreg src / invalid-name). Added encode_neon_aes_neg_src_nonreg_dest_reg_wsp (nonreg src passing, WSP dest failing — same SP-as-V31 bug). Closed: tier round spent and documented surface covered.
