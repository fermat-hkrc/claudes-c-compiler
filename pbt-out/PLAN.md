# PBT Campaign: encode_ldaxr_stlxr

## Scan findings
- **Spec:** (none found) as a dedicated requirement/PR doc. Contract taken from assembler README.md:11 ("accepts the same textual assembly that GCC's gas would consume"), encoder/mod.rs:1-7 (Encodes AArch64 instructions into 32-bit machine code words), encoder/mod.rs:354-359 dispatch (`ldaxr`/`stlxr`/`ldaxrb`/`stlxrb`/`ldaxrh`/`stlxrh` => encode_ldaxr_stlxr), README.md:221 Loads/Stores table listing those six mnemonics, and ARM ARM Load/Store Exclusive (size 001000 0 L 0 Rs o0 Rt2 Rn Rt with o0=1; Rt is Wt/Xt 31=ZR; Rn is Xn|SP; Ws is Wt 31=WZR; offset {,#0}). Independent assembler reference: llvm-mc -triple=aarch64 -show-encoding. Callers: src/backend/arm/codegen/inline_asm.rs:314-338 (Acquire => ldaxr, Release => stlxr, AcqRel/SeqCst => both). The producing statements at load_store.rs:573-599 are not independent Doc evidence.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod encode_*_pbt` blocks in `src/backend/arm/assembler/encoder/*.rs` (no `tests/` directory, no `[[test]]` in Cargo.toml). Runner: `cargo test --lib`. PBT framework: `proptest = "1.11.0"` in `[dev-dependencies]`. Historical `pbt-native/` / `pbt-out/` excluded from layout discovery.
- **Buildability probe:** `cargo test --lib encode_ldxr_stxr_kat_llvm_mc_ldxr_x0_x1` (unchanged project harness, existing KAT in load_store.rs encode_ldxr_stxr_pbt) → `1 passed, 1735 filtered out (1 suite, 0.02s)`. Rung 1 available.
- **Harness placement:** extend existing `cargo test --lib` target; add `#[cfg(test)] mod encode_ldaxr_stlxr_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (same convention as neighbouring encode_ldxr_stxr_pbt / encode_ldxp_stxp_pbt). Not pbt-native: probe succeeded.
- **Candidate modules:** encode_ldaxr_stlxr (single-symbol campaign on load_store.rs)
- **Skipped modules:** (none) — campaign is restricted to encode_ldaxr_stlxr; other load_store.rs symbols stay out of scope in FUNCTION_INDEX
- **State machine:** not applicable — encode_ldaxr_stlxr is a pure function (operands, is_load, forced_size -> Word/Err) with no lifecycle or mutating operations
- **Oracle (encode_ldaxr_stlxr):** Differential vs llvm-mc `-triple=aarch64 -show-encoding`. Stronger rejected: State machine (no lifecycle). Algebraic round-trip rejected (no in-tree exclusive-acquire decoder). Sibling encode_ldxr_stxr rejected as differential (same-job gate fails: o0=0 relaxed exclusive vs o0=1 acquire/release). Sibling encode_ldxp_stxp / encode_ldar_stlr rejected (exclusive-pair / ordered non-exclusive, different job). Weaker available: algebraic.metamorphic (L / size / o0 vs ldxr), algebraic.invariant (ARM bitfield field layout with o0=1), negative_error (arity / extra / SP-as-Rt / W-base / XZR-base / FP / X-Ws / offset / Ws-overlap / X-data-on-byte). SUT-boundary: internal-helper of the GNU-style assembler; mapping load `[Reg(Rt), Mem{Rn,0}]` <-> `{ldaxr|ldaxrb|ldaxrh} Rt, [Rn]`; store `[Reg(Ws), Reg(Rt), Mem{Rn,0}]` <-> `{stlxr|stlxrb|stlxrh} Ws, Rt, [Rn]`.
- **Seeds:** load_store.rs encode_ldxr_stxr_pbt (same exclusive-single class with o0=0; extra/SP/W-base/XZR/FP/X-Ws/offset/Ws-overlap negative contracts and ARM field unpack). No dedicated encode_ldaxr_stlxr unit tests.

## Module: encode_ldaxr_stlxr
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results
- Sweep round 1/1: coverage_gaps had no LLVM profraw; manual arm audit added encode_ldaxr_stlxr_diff_alt_spellings (passing) and encode_ldaxr_stlxr_neg_mem_index (passing). Closed: tier round spent and documented surface covered.
