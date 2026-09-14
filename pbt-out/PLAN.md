# PBT Campaign: encode_neon_sqshrun

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: built-in AArch64 assembler translates GNU-style assembly text; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:229`: NEON narrow lists `sqshrun`/`sqrshrun` (+ `2` variants).
  - Encoder module docstring `src/backend/arm/assembler/encoder/mod.rs:1-7`: encodes AArch64 instructions into 32-bit machine code words.
  - Dispatch `encoder/mod.rs:649-650`: `"sqrshrun" => encode_neon_sqshrun(operands, true, false)`, `"sqrshrun2" => ... true, true`. `encoder/mod.rs:841-842`: `"sqshrun" => encode_neon_sqshrun(operands, false, false)`, `"sqshrun2" => ... false, true`.
  - Body at `neon.rs:119-155`: Format `0 Q 1 011110 immh immb opcode Rn Rd`; source arrangement selects element_bits 16/32/64 for 8h/4s/2d; `immhb = (element_bits - shift) & 0x7F`; `immh = (immhb >> 3) | immh_base`; Q from `is_high`; U hardcoded 1; opcode 100001 (SQSHRUN) / 100011 (SQRSHRUN). Dest arrangement is discarded. Shift check is `shift == 0 || shift > element_bits` (source size, not dest). Extra operands beyond index 2 are ignored (`len < 3`).
  - ARM ARM Advanced SIMD shift by immediate SQSHRUN/SQRSHRUN: `0 Q 1 011110 immh immb 100001/100011 Rn Rd`. Tb/Ta pairs: Q=0: 8B←8H (shift 1..8), 4H←4S (1..16), 2S←2D (1..32); Q=1: 16B←8H, 8H←4S, 4S←2D. `shift = (2 * esize) - UInt(immh:immb)` with esize = dest element size. immh != 0000. U=1 always (signed-to-unsigned saturating narrow).
  - Sibling `encode_neon_shrn` (`neon.rs:1436-1449`) uses `half_bits = element_bits / 2` as the shift upper bound (dest esize). Different opcode/U (same-job gate fails) but documents the narrowing shift range.
  - Siblings encode_neon_qshrn (SQSHRN/UQSHRN signed/unsigned saturating narrow, opcodes 100101/100111) and encode_neon_scalar_qshrn (scalar) fail the same-job gate.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `neon.rs` already hosts neighbouring PBT modules (`encode_neon_qshrn_pbt`, `encode_neon_shll_pbt`, …). New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1540 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neon_sqshrun_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_neon_sqshrun (neon.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in neon.rs are indexed but not tested.
- **Oracle (encode_neon_sqshrun):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GNU-style SQSHRUN/SQRSHRUN(+2) forms. State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_neon_shrn / encode_neon_qshrn / encode_neon_scalar_qshrn fail the same-job gate (different opcode / saturation / scalar vs vector). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style `sqshrun`/`sqrshrun`/`sqshrun2`/`sqrshrun2` text. Mapping: `encode_neon_sqshrun([RegArrangement(Vd,Tb), RegArrangement(Vn,Ta), Imm(shift)], is_rounding, is_high)` <-> AT&T `{sq,sqr}shrun{2?} Vd.Tb, Vn.Ta, #shift`.
- **Seeds:** README.md:229 NEON narrow table; encoder/mod.rs:649-650,841-842 dispatch; sibling encode_neon_shrn neon.rs:1443-1444 half_bits = source/2. No existing unit test of encode_neon_sqshrun.
- **State machine:** not applicable — encode_neon_sqshrun is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending a word.

## Module: encode_neon_sqshrun
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of arity, Ta, *v as u32, get_neon_reg Reg dest+source, extra, mismatched Tb — added i64_trunc, reg_source)
