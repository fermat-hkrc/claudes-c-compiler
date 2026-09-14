# PBT Campaign: encode_ret

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; always 4 bytes little-endian; subset emitted by codegen.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: translates GNU-style assembly as GCC's gas would consume into ELF `.o`; "accepts the same textual assembly that GCC's gas would consume".
  - README instruction table `src/backend/arm/assembler/README.md:220`: lists `ret` under Branches.
  - README file map `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers branches including RET.
  - Body comment at `compare_branch.rs:232`: `RET: 1101011 0010 11111 000000 Rn 00000`. Default at `compare_branch.rs:227-228`: empty operands → Rn=30 (LR / x30).
  - Dispatch `encoder/mod.rs:320`: `"ret" => encode_ret(operands)`.
  - ARM ARM Unconditional branch (register) RET: bits[31:25]=1101011, opc[24:21]=0010, op2[20:16]=11111, op3[15:10]=000000, Rn[9:5], op4[4:0]=00000. If Xn is omitted, X30 is used. Rn is a 64-bit GPR; register 31 is XZR, never SP.
  - Codegen caller `src/backend/arm/codegen/prologue.rs:319`: emits bare `ret` (no operand) as the function epilogue.
  - Peephole classifier `src/backend/arm/codegen/peephole.rs:289`: `trimmed == "ret"` → LineKind::Ret.
  - Sibling `encode_br` is BR (opc=0000), different job. Sibling `encode_blr` is BLR (opc=0001), different job.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has sibling PBT modules — those are prior campaign artifacts living in the project source; new work still extends the same `cargo test --lib` inline layout.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1462 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_ret_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_ret (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_ret):** Differential (llvm-mc -triple=aarch64 -show-encoding) for valid RET with omitted Rn (default x30) or Rn in {x0–x30, xzr, lr}. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_br fails the same-job gate (BR / opc=0000). encode_blr fails the same-job gate (BLR / opc=0001) — used only as opc-bit metamorphic companions. SUT-boundary: internal-helper of the GNU-style AArch64 assembler; public contract is encoding GNU-style AArch64 RET text / ARM ARM RET. Mapping: `[]` <-> `ret`; `[Reg("xN"|"xzr"|"lr")]` <-> `ret xN`.
- **Seeds:** README.md:220 Branches table lists ret; prologue.rs:319 emits bare `ret`; peephole.rs:1028 classifies `"    ret"`. No existing unit test of encode_ret. Seed: peephole.rs:1028 (bare `ret` is valid assembler text).
- **State machine:** not applicable — encode_ret is a pure function with no mutating operations or lifecycle.

## Module: encode_ret
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of empty-default / get_reg success / None / other / extra / W / SP / FP — added invalid-name and dedicated FP properties)
