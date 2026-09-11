# PBT Campaign: encode_neon_three_diff_narrow

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:230`: NEON widen/long lists `addhn`/`subhn` (+ `2` variants).
  - Dispatch `encoder/mod.rs:628-635`: `"addhn"`/`"addhn2"`/`"raddhn"`/`"raddhn2"`/`"subhn"`/`"subhn2"`/`"rsubhn"`/`"rsubhn2"` => `encode_neon_three_diff_narrow` with (U, opcode, is_high).
  - Function comment `neon.rs:1513`: Three-different narrowing high: Format `0 Q U 01110 size 1 Rm opcode 00 Rn Rd`.
  - ARM ARM Advanced SIMD three-different (ADDHN/RADDHN/SUBHN/RSUBHN): Q=0 writes lower half (Tb = 8B/4H/2S), Q=1 (`*2`) writes upper half (Tb = 16B/8H/4S); size from source Ta (8H=00, 4S=01, 2D=10); U=0 ADDHN/SUBHN, U=1 RADDHN/RSUBHN; opcode 0100 add-family, 0110 sub-family; size=11 reserved.
  - Callers: `encoder/mod.rs:628-635` only.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Existing PBT modules live in `data_processing.rs` (`encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`) and `load_store.rs`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `neon.rs` currently has no `#[cfg(test)]` block.
- **Buildability probe:** `cargo test --lib encode_bic_kat_llvm_mc_bic_x0_x1_x2 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 605 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neon_three_diff_narrow_pbt` at the bottom of `src/backend/arm/assembler/encoder/neon.rs` (inline layout; file has no test module yet, sibling PBT modules live in other encoder sources). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery.
- **Candidate modules:** encode_neon_three_diff_narrow (neon.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in neon.rs are indexed but not tested.
- **Oracle (encode_neon_three_diff_narrow):** Differential (llvm-mc -triple=aarch64 -show-encoding). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree ADDHN decoder rejected: none exists. encode_neon_three_diff is a widening/long sibling (source arrangements 8b/16b/4h/8h/2s/4s; different job) and fails the same-job sibling gate as a differential reference. SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[RegArrangement(Vd,Tb), RegArrangement(Vn,Ta), RegArrangement(Vm,Ta)]` <-> `{addhn|raddhn|subhn|rsubhn}[2] Vd.Tb, Vn.Ta, Vm.Ta` with (U, opcode, is_high) from the mnemonic.

## Module: encode_neon_three_diff_narrow
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_neon_three_diff_narrow error paths — invalid NEON register names; dest Tb / Rm Ta / extra operand / GPR dest filed as bugs)
