# PBT Campaign: encode_neon_shll

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Assembler README `src/backend/arm/assembler/README.md:1-14`: built-in AArch64 assembler translates GNU-style assembly text; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:230`: NEON widen/long lists `sshll`/`ushll`/`sxtl`/`uxtl` (+ `2` variants).
  - Encoder module docstring `src/backend/arm/assembler/encoder/mod.rs:1-7`: encodes AArch64 instructions into 32-bit machine code words.
  - Dispatch `encoder/mod.rs:614-617`: `"ushll" => encode_neon_shll(operands, 1, false)`, `"ushll2" => ... true`, `"sshll" => ... 0, false`, `"sshll2" => ... 0, true`.
  - Body at `neon.rs:1470-1485`: Format `0 Q U 011110 immh immb 101001 Rn Rd`; source arrangement selects base esize 8/16/32; `immhb = base_val + shift`; Q from `is_high`; U from `u_bit`. Dest arrangement is discarded. No shift-range check. Extra operands beyond index 2 are ignored (`len < 3`).
  - ARM ARM Advanced SIMD shift by immediate SSHLL/USHLL: `0 Q U 011110 immh immb 101001 Rn Rd`. Tb/Ta pairs: Q=0: 8B→8H (shift 0..7), 4H→4S (0..15), 2S→2D (0..31); Q=1: 16B→8H, 8H→4S, 4S→2D. `immh:immb = esize + shift`. immh != 0000.
  - Sibling `encode_neon_xtl` (`neon.rs:160-184`) is documented as "aliases for USHLL/SSHLL with shift #0" (same job at shift=0). Dispatch `encoder/mod.rs:844-848` uxtl/sxtl(+2). llvm-mc prints `sxtl` as `sshll ..., #0`.
  - Siblings encode_neon_shl / encode_neon_shift_left_imm (same-width SHL/SQSHL) and README two-misc `shll`/`shll2` (shift by element size, two-register miscellaneous) fail the same-job gate.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `neon.rs` already hosts neighbouring PBT modules (`encode_neon_qshrn_pbt`, `encode_neon_tbl_pbt`, …). New work extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1521 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neon_shll_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_neon_shll (neon.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in neon.rs are indexed but not tested.
- **Oracle (encode_neon_shll):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid GNU-style SSHLL/USHLL(+2) forms. State machine rejected: single encoding call, no lifecycle/state enum. Round-trip with an in-tree decoder rejected: none exists. encode_neon_shl / encode_neon_shift_left_imm / two-misc SHLL fail the same-job gate. encode_neon_xtl is the same job at shift #0 (documented alias) and is used as a differential companion. SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style `sshll`/`ushll`/`sshll2`/`ushll2` text. Mapping: `encode_neon_shll([RegArrangement(Vd,Ta), RegArrangement(Vn,Tb), Imm(shift)], u_bit, is_high)` <-> AT&T `{sshll|ushll}{2?} Vd.Ta, Vn.Tb, #shift`.
- **Seeds:** README.md:230 NEON widen/long table; encoder/mod.rs:614-617 dispatch. No existing unit test of encode_neon_shll. Seed: README.md:230.
- **State machine:** not applicable — encode_neon_shll is a single-call encoder with no mutating operation alphabet whose order can corrupt encoder-owned instruction state beyond appending a word.

## Module: encode_neon_shll
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of arity < 3, unsupported Tb, kinds, invalid names, GPR dest, Q vs Tb — added gpr_dest, src_vs_high, arity_kinds)
