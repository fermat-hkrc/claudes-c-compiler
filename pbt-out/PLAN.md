# PBT Campaign: encode_add_sub

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:208-214`: `add` / `adds` / `sub` / `subs` listed under Data Processing; NEON three-same also lists vector `add`/`sub`.
  - DESIGN_DOC.md:338: AArch64 | ARM assembly syntax | Fixed 32-bit encoding | imm12 auto-shift.
  - DESIGN_DOC.md:350: AArch64 relocs include ADD_ABS_LO12_NC.
  - Dispatch `src/backend/arm/assembler/encoder/mod.rs:223-230`: `"add"`/`"sub"` (non-scalar-d) and `"adds"`/`"subs"` call `encode_add_sub`.
  - ARM ARM ADD/SUB (immediate): `sf op S 10001 sh imm12 Rn Rd`; sh in {0,1} meaning LSL #0 / LSL #12; imm12 is 12 bits; register 31 is SP/WSP.
  - ARM ARM ADD/SUB (shifted register): `sf op S 01011 shift 0 Rm imm6 Rn Rd`; shift in {LSL,LSR,ASR} (not ROR); register 31 is XZR/WZR.
  - ARM ARM ADD/SUB (extended register): `sf op S 01011 00 1 Rm option imm3 Rn Rd`; imm3 in 0..=4; used when Rd or Rn is SP.
  - Callers: `encoder/mod.rs:223-230`; `compare_branch.rs:19` (CMP → SUBS XZR) and `:34` (CMN → ADDS XZR).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Existing module in this file: `encode_add_sub_pbt` at `data_processing.rs:1069` and sibling `encode_adc_pbt` at `:1602`. No crate-root `tests/` directory. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`).
- **Buildability probe:** `cargo test --lib encode_add_sub -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: FAILED. 6 passed; 6 failed; 0 ignored`. Failures are the pre-existing encode_add_sub_pbt contracts (imm12 lsl12 mask, ROR accepted, SP+LSL shifted form) plus their three regression witnesses. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending new properties inside `#[cfg(test)] mod encode_add_sub_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; do not rewrite existing property functions). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery.
- **Candidate modules:** encode_add_sub (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.

## Module: encode_add_sub
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: FP regs + ADDS Rd=SP + tprel modifiers; coverage_gaps had no profraw)
