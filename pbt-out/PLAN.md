# PBT Campaign: encode_csneg

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:219`: Conditional select category lists `csel`, `csinc`, `csinv`, `csneg`, `cset`, `csetm`, `fcsel`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare, conditional select, branches, and conditional aliases.
  - Dispatch `encoder/mod.rs:311`: `"csneg" => encode_csneg(operands)`.
  - Body at `compare_branch.rs:127-139`: CSNEG Rd, Rn, Rm, cond with op=1 and op2=01.
  - Comment at `compare_branch.rs:275`: `CNEG Rd, Rn, cond -> CSNEG Rd, Rn, Rn, invert(cond)`.
  - Comment at `compare_branch.rs:286`: CSNEG encoding `sf 1 0 11010100 Rm cond 0 1 Rn Rd`.
  - ARM ARM Conditional Select Negate (architectural): `CSNEG <Wd>, <Wn>, <Wm>, <cond>` / `CSNEG <Xd>, <Xn>, <Xm>, <cond>`. Encoding: `sf 1 0 11010100 Rm cond 01 Rn Rd`. AL/NV are valid (unlike CNEG alias). Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only. Same-width GPRs.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `encode_cond` (`encoder/mod.rs:169-190`) maps 16 cond names including al/nv and hs/lo aliases; `sf_bit` from Rd width only.
  - llvm-mc `-triple=aarch64` accepts `csneg x0, x1, x2, eq` as 0xda820420; `csneg w0, w1, w2, ne` as 0x5a821420; `csneg x0, x1, x2, al` as 0xda82e420; `csneg xzr, x0, x1, eq` as 0xda81041f; `csneg lr, x0, x1, eq` as 0xda81041e. Disassembles `csneg x0, x1, x1, ne` as `cneg x0, x1, eq`. Rejects: too few operands, extra operands, SP/WSP, FP/SIMD, mixed x/w, unknown cond.
  - Sibling `encode_csinc` (`compare_branch.rs:99`) is CSINC (op=0, op2=01). Sibling `encode_csinv` (`compare_branch.rs:113`) is CSINV (op=1, op2=00). Sibling `encode_csel` (`compare_branch.rs:85`) is CSEL (op=0, op2=00). Different jobs; used as metamorphic XOR transforms (CSINC XOR CSNEG = bit 30; CSINV XOR CSNEG = bit 10). Sibling `encode_cneg` is the 3-operand alias of CSNEG (different mnemonic/arity); used as metamorphic alias transform (CNEG == CSNEG Rd,Rn,Rn,invert(cond)).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules — not rewritten.
- **Buildability probe:** `cargo test --lib encode_csinv_kat_llvm_mc_x0_x1_x2_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 941 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_csneg_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_csinv_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_csneg (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_csneg):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `csneg Rd, Rn, Rm, cond` with valid AArch64 CSNEG operands (same-width GPR Rd/Rn/Rm including xzr/wzr/lr, cond in Cond16 including al/nv and hs/lo aliases). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CSNEG decoder rejected: none exists. encode_csinc / encode_csinv / encode_csel fail the same-job sibling gate as *differential* references (different op/op2); used as algebraic metamorphic XOR transforms. encode_cneg fails the same-job sibling gate as differential (different mnemonic/arity); used as algebraic metamorphic alias transform (CNEG == CSNEG Rd,Rn,Rn,invert(cond)). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Reg(rm), Cond(c)]` <-> `csneg rd, rn, rm, c`.
- **Seeds:** encode_csinc_pbt / encode_csinv_pbt (same tree, CSINC/CSINV twins of this CSNEG): differential vs llvm-mc, metamorphic XOR vs architectural sibling, word-layout invariant, neg arity / extra / wrong-reg / unknown-cond / invalid-name / bad kind. encode_cneg_pbt::encode_cneg_meta_vs_csneg at compare_branch.rs:6884.
- **State machine:** not applicable — encode_csneg is a pure function with no mutating operations or lifecycle.

## Module: encode_csneg
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / non-Reg-or-Cond — two properties added and passed; extra/SP/mixed/FP filed as bugs)
