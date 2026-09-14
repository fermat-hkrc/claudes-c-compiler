# PBT Campaign: encode_ldxr_stxr

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - README.md load/store table lists `ldxr`, `stxr`, `ldxrb`, `stxrb`, `ldxrh`, `stxrh`.
  - Dispatch `encoder/mod.rs:348-353`: `ldxr`/`stxr` with `forced_size=None`; `ldxrb`/`stxrb` with `Some(0b00)`; `ldxrh`/`stxrh` with `Some(0b01)`.
  - Body at `load_store.rs:547-576`: load reads Rt then Mem base; store reads Ws, Rt, then Mem base. Size is `forced_size` or auto from Rt width (X→11, W→10). Encoding: `size 001000 0 L 0 Rs o0=0 Rt2=11111 Rn Rt` with Rs=11111 on load. No arity upper bound; Mem offset ignored.
  - ARM ARM Load/Store Exclusive (single): `size 001000 0 L 0 Rs o0 Rt2 Rn Rt`. size=00 byte / 01 half / 10 word / 11 doubleword. L=1 load, L=0 store. o0=0 (LDXR/STXR; LDAXR/STLXR is o0=1). Rt2=11111. Offset absent or #0.
  - llvm-mc `-triple=aarch64` confirms: `ldxr x0, [x1]` = 0xc85f7c20; `ldxr w0, [x1]` = 0x885f7c20; `ldxrb w0, [x1]` = 0x085f7c20; `ldxrh w0, [x1]` = 0x485f7c20; `stxr w0, x1, [x2]` = 0xc8007c41; `stxr w0, w1, [x2]` = 0x88007c41. Rejects extra operand, nonzero offset, pre/post-index, SP as Rt, W-base, XZR-base, WSP-base, FP/SIMD Rt, X as STXR status, X data on byte/half, Ws aliasing Rt/Xn ("status is also a source"). Allows `stxr wzr, x0, [sp]` (WZR vs SP). `lr` aliases X30. `#0` offset is canonicalized away.
  - Codegen callers (`codegen/atomics.rs:24-72`, `codegen/inline_asm.rs:322-338`) emit `ldxr Rt, [Xn]` and `stxr Ws, Rt, [Xn]` with no offset.
  - Sibling `encode_ldaxr_stlxr` is acquire/release exclusive (o0=1) — different job. Sibling `encode_ldxp_stxp` is exclusive pair (o1=1) — different job. Sibling `encode_ldar_stlr` is ordered non-exclusive (bit23=1) — different job.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `load_store.rs` already has prior-campaign modules `encode_adr_pbt`, `encode_ldar_stlr_pbt`, `encode_ldur_stur_pbt`, `encode_ldxp_stxp_pbt` — not rewritten.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1121 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_ldxr_stxr_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (inline layout; sibling of `encode_ldxp_stxp_pbt` / `encode_ldar_stlr_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_ldxr_stxr (load_store.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in load_store.rs are indexed but not tested.
- **Oracle (encode_ldxr_stxr):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid exclusive single load/store. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree LDXR/STXR decoder rejected: none exists. encode_ldaxr_stlxr / encode_ldxp_stxp / encode_ldar_stlr fail the same-job sibling gate (acquire-release exclusive vs exclusive; exclusive-pair vs exclusive-single; ordered non-exclusive vs exclusive). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: load `(Reg(Rt), Mem{Rn, 0}, is_load=true, forced_size)` <-> `{ldxr|ldxrb|ldxrh} Rt, [Rn]`; store `(Reg(Ws), Reg(Rt), Mem{Rn, 0}, is_load=false, forced_size)` <-> `{stxr|stxrb|stxrh} Ws, Rt, [Rn]`.
- **Seeds:** encode_ldxp_stxp_pbt / encode_ldar_stlr_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / SP / W-base / XZR-base / FP / offset / Ws-overlap negatives. Codegen `atomics.rs:24-25` emits canonical `ldxr Rt, [x1]` / `stxr w4, Rt, [x1]`. No existing unit test of encode_ldxr_stxr. Seed: encode_ldxp_stxp_pbt::encode_ldxp_stxp (diff / extra / overlap).
- **State machine:** not applicable — encode_ldxr_stxr is a pure function with no mutating operations or lifecycle.

## Module: encode_ldxr_stxr
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of is_load / get_reg miss / Mem vs non-Mem / parse_reg_num None / forced_size / is_64 — all reached)
