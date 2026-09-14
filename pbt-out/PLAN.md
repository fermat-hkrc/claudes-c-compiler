# PBT Campaign: encode_csetm

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:219`: Conditional select category lists `csel`, `csinc`, `csinv`, `csneg`, `cset`, `csetm`, `fcsel`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare, conditional select, branches, and conditional aliases.
  - Dispatch `encoder/mod.rs:313`: `"csetm" => encode_csetm(operands)`.
  - Codegen callers: none emit `csetm` directly; the mnemonic is listed in the assembler surface and dispatched.
  - Comment at `compare_branch.rs:156`: `CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond)`.
  - ARM ARM Conditional Set Mask (alias of CSINV, not architectural): `CSETM <Wd>, <cond>` / `CSETM <Xd>, <cond>` is equivalent to `CSINV <Wd/Xd>, WZR/XZR, WZR/XZR, invert(<cond>)` and is not valid when `<cond>` is AL or NV. Encoding is CSINV: `sf 1 0 11010100 Rm=11111 cond 00 Rn=11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `encode_cond` (`encoder/mod.rs:169-190`) maps 16 cond names including al/nv and hs/lo aliases.
  - llvm-mc `-triple=aarch64` accepts `csetm x0, eq` as 0xda9f13e0; `csetm w0, ne` as 0x5a9f03e0; `csetm xzr, eq` as 0xda9f13ff; `csetm lr, eq` as 0xda9f13fe. Rejects: too few operands, extra operands, SP/WSP, FP/SIMD, AL, NV.
  - Sibling `encode_csinv` (`compare_branch.rs:113`) is 4-operand CSINV. Sibling `encode_cinv` (`compare_branch.rs:309`) is CINV Rd, Rn, cond. Different jobs (different mnemonics/arity); used as metamorphic alias transforms (CSETM == CSINV Rd,ZR,ZR,invert(cond); CSETM == CINV Rd,ZR,cond). Sibling `encode_cset` is CSET (CSINC alias) — different job (0/1 vs 0/all-ones); used only as a bit-field XOR metamorphic (op and op2).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules — not rewritten.
- **Buildability probe:** `cargo test --lib encode_cset_kat_llvm_mc_x0_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 886 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_csetm_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_cset_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_csetm (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_csetm):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `csetm Rd, cond` with valid AArch64 CSETM operands (same-width GPR Rd including xzr/wzr/lr, cond in Cond14 excluding al/nv, including hs/lo aliases). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CSETM decoder rejected: none exists. encode_csinv / encode_cinv fail the same-job sibling gate as *differential* references (different mnemonics/arity); used as algebraic metamorphic transforms (CSETM == CSINV Rd,ZR,ZR,invert(cond); CSETM == CINV Rd,ZR,cond). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Cond(c)]` <-> `csetm rd, c`.
- **Seeds:** encode_cset_pbt (same tree, CSINC twin of this CSINV alias): differential vs llvm-mc, metamorphic vs architectural sibling, word-layout invariant, neg arity / extra / AL-NV / wrong-reg / bad kind. encode_cinv_pbt::encode_cinv_kat_csetm_alias at compare_branch.rs:4466 and encode_cinv_meta_vs_csetm at compare_branch.rs:4562.
- **State machine:** not applicable — encode_csetm is a pure function with no mutating operations or lifecycle.

## Module: encode_csetm
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / non-Reg-or-Cond — three properties added and passed; extra/AL-NV/SP/FP filed as bugs)
