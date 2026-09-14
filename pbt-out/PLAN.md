# PBT Campaign: encode_csinc

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:219`: Conditional select category lists `csel`, `csinc`, `csinv`, `csneg`, `cset`, `csetm`, `fcsel`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare, conditional select, branches, and conditional aliases.
  - Dispatch `encoder/mod.rs:309`: `"csinc" => encode_csinc(operands)`.
  - Codegen callers: none emit `csinc` directly; the mnemonic is listed in the assembler surface and dispatched. `codegen/asm_emitter.rs:60` notes a TODO that CSET/CSINC is needed for `=@cc`.
  - Comment at `compare_branch.rs:99-109`: CSINC Rd, Rn, Rm, cond with op2=01.
  - Comment at `compare_branch.rs:142`: `CSET Rd, cond -> CSINC Rd, XZR, XZR, invert(cond)`.
  - Comment at `compare_branch.rs:292`: `CINC Rd, Rn, cond -> CSINC Rd, Rn, Rn, invert(cond)`.
  - ARM ARM Conditional Select Increment (architectural): `CSINC <Wd>, <Wn>, <Wm>, <cond>` / `CSINC <Xd>, <Xn>, <Xm>, <cond>`. Encoding: `sf 0 0 11010100 Rm cond 01 Rn Rd`. AL/NV are valid (unlike CINC/CSET aliases). Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only. Same-width GPRs.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `encode_cond` (`encoder/mod.rs:169-190`) maps 16 cond names including al/nv and hs/lo aliases; `sf_bit` from Rd width only.
  - llvm-mc `-triple=aarch64` accepts `csinc x0, x1, x2, eq` as 0x9a820420; `csinc w0, w1, w2, ne` as 0x1a821420; `csinc x0, x1, x2, al` as 0x9a82e420; `csinc xzr, x0, x1, eq` as 0x9a81041f; `csinc lr, x0, x1, eq` as 0x9a81041e. Disassembles `csinc x0, x1, x1, ne` as `cinc x0, x1, eq` and `csinc x0, xzr, xzr, ne` as `cset x0, eq`. Rejects: too few operands, extra operands, SP/WSP, FP/SIMD, mixed x/w.
  - Sibling `encode_csel` (`compare_branch.rs:85`) is CSEL (op2=00). Sibling `encode_csneg` (`compare_branch.rs:127`) is CSNEG (op=1, op2=01). Different jobs; used as metamorphic XOR transforms (CSEL XOR CSINC = bit 10; CSNEG XOR CSINC = bit 30). Sibling `encode_cinc` / `encode_cset` are aliases of CSINC (different mnemonic/arity); used as metamorphic alias transforms.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules — not rewritten.
- **Buildability probe:** `cargo test --lib encode_csetm_kat_llvm_mc_x0_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 905 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_csinc_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_csel_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_csinc (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_csinc):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `csinc Rd, Rn, Rm, cond` with valid AArch64 CSINC operands (same-width GPR Rd/Rn/Rm including xzr/wzr/lr, cond in Cond16 including al/nv and hs/lo aliases). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CSINC decoder rejected: none exists. encode_csel / encode_csneg fail the same-job sibling gate as *differential* references (different op/op2); used as algebraic metamorphic XOR transforms. encode_cinc / encode_cset fail the same-job sibling gate as differential (different mnemonic/arity); used as algebraic metamorphic alias transforms (CINC == CSINC Rd,Rn,Rn,invert(cond); CSET == CSINC Rd,ZR,ZR,invert(cond)). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Reg(rm), Cond(c)]` <-> `csinc rd, rn, rm, c`.
- **Seeds:** encode_csel_pbt (same tree, CSEL twin of this CSINC): differential vs llvm-mc, metamorphic XOR vs architectural sibling, word-layout invariant, neg arity / extra / wrong-reg / unknown-cond / invalid-name / bad kind. encode_cinc_pbt::encode_cinc_meta_vs_csinc at compare_branch.rs:3815 and encode_cset_pbt::encode_cset_meta_vs_csinc at compare_branch.rs:8139.
- **State machine:** not applicable — encode_csinc is a pure function with no mutating operations or lifecycle.

## Module: encode_csinc
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / non-Reg-or-Cond — two properties added and passed; extra/SP/mixed/FP filed as bugs)
