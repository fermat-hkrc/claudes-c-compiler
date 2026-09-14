# PBT Campaign: encode_ldar_stlr

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:221`: Loads/Stores category lists `ldar`, `stlr`, `ldarb`, `stlrb`, `ldarh`, `stlrh`.
  - Assembler README `src/backend/arm/assembler/README.md:508`: `load_store.rs` covers exclusive/acquire/release encodings.
  - Dispatch `encoder/mod.rs:360-365`: `"ldar"`/`"stlr"` → `encode_ldar_stlr(operands, is_load, None)`; `"ldarb"`/`"stlrb"` → `Some(0b00)`; `"ldarh"`/`"stlrh"` → `Some(0b01)`.
  - Body at `load_store.rs:636-650`: `LDAR/STLR: size 001000 1 L 0 11111 1 11111 Rn Rt`. `size` from `forced_size` or Rt width (X→0b11, W→0b10). `L=1` load / `L=0` store. No arity check. Memory operand `Mem { base, .. }` ignores offset. No SP-vs-ZR distinction.
  - ARM ARM LDAR/STLR (Load-Acquire / Store-Release Register): `LDAR <Wt>, [<Xn|SP>{,#0}]` / `LDAR <Xt>, [<Xn|SP>{,#0}]` and STLR dual. Byte/halfword forms take `<Wt>` only. Encoding: `size 001000 1 L 0 Rs=11111 o0=1 Rt2=11111 Rn Rt`. size 00=byte, 01=half, 10=32-bit, 11=64-bit. Rt is Wt/Xt (31=WZR/XZR, never SP). Rn is Xn|SP (31=SP, never XZR). Offset absent or #0.
  - Codegen `src/backend/arm/codegen/atomics.rs:93-128` emits `ldarb/ldarh/ldar/stlrb/stlrh/stlr` with `w0`/`x0` and `[x0]`.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes with num<=31.
  - llvm-mc `-triple=aarch64` accepts `ldar x0, [x1]` as 0xc8dffc20; `ldar w0, [x1]` as 0x88dffc20; `stlr x0, [x1]` as 0xc89ffc20; `ldarb w0, [x1]` as 0x08dffc20; `ldarh w0, [x1]` as 0x48dffc20; `ldar xzr, [sp]` as 0xc8dfffff; `ldar x0, [x1, #0]` as the no-offset form. Rejects: extra operand, SP/WSP as Rt, W/WSP/XZR/WZR as base, nonzero offset, pre/post-index, register-offset, FP/SIMD Rt, Xt for ldarb/ldarh/stlrb/stlrh.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `load_store.rs` already has prior-campaign module `encode_adr_pbt` — not rewritten.
- **Buildability probe:** `cargo test --lib encode_adr_kat_llvm_mc_adr_x0_imm0 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 995 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_ldar_stlr_pbt` at the bottom of `src/backend/arm/assembler/encoder/load_store.rs` (inline layout; sibling of `encode_adr_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_ldar_stlr (load_store.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in load_store.rs are indexed but not tested.
- **Oracle (encode_ldar_stlr):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid LDAR/STLR/LDARB/STLRB/LDARH/STLRH with Wt/Xt Rt (31=ZR) and Xn|SP base, offset absent or #0. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree LDAR decoder rejected: none exists. encode_ldaxr_stlxr / encode_ldxr_stxr fail the same-job sibling gate (exclusive forms; different Rs/o0/L layout). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rt), Mem{base, offset:0}]` + (is_load, forced_size) <-> `{ldar|stlr|ldarb|stlrb|ldarh|stlrh} rt, [rn]`.
- **Seeds:** encode_adr_pbt (same file): differential vs llvm-mc, ARM field unpack, extra-operand / invalid-operand negatives. Codegen atomics.rs:93-128 emits `ldar w0, [x0]` / `stlr x1, [x0]` family. No existing unit test of encode_ldar_stlr. Seed: encode_adr_pbt::encode_adr_diff_imm_llvm_mc at load_store.rs encode_adr_pbt.
- **State machine:** not applicable — encode_ldar_stlr is a pure function with no mutating operations or lifecycle.

## Module: encode_ldar_stlr
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg non-Reg and parse_reg_num None — extended arity/shape generators; those paths Err as specified)
