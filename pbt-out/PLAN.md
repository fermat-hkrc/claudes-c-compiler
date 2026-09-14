# PBT Campaign: encode_csinv

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:219`: Conditional select category lists `csel`, `csinc`, `csinv`, `csneg`, `cset`, `csetm`, `fcsel`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare, conditional select, branches, and conditional aliases.
  - Dispatch `encoder/mod.rs:310`: `"csinv" => encode_csinv(operands)`.
  - Comment at `compare_branch.rs:113-125`: CSINV Rd, Rn, Rm, cond with op=1 and op2=00.
  - Comment at `compare_branch.rs:155`: `CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond)`.
  - Comment at `compare_branch.rs:308`: `CINV Rd, Rn, cond -> CSINV Rd, Rn, Rn, invert(cond)`.
  - ARM ARM Conditional Select Invert (architectural): `CSINV <Wd>, <Wn>, <Wm>, <cond>` / `CSINV <Xd>, <Xn>, <Xm>, <cond>`. Encoding: `sf 1 0 11010100 Rm cond 00 Rn Rd`. AL/NV are valid (unlike CINV/CSETM aliases). Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only. Same-width GPRs.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `encode_cond` (`encoder/mod.rs:169-190`) maps 16 cond names including al/nv and hs/lo aliases; `sf_bit` from Rd width only.
  - llvm-mc `-triple=aarch64` accepts `csinv x0, x1, x2, eq` as 0xda820020; `csinv w0, w1, w2, ne` as 0x5a821020; `csinv x0, x1, x2, al` as 0xda82e020; `csinv xzr, x0, x1, eq` as 0xda81001f; `csinv lr, x0, x1, eq` as 0xda81001e. Disassembles `csinv x0, x1, x1, ne` as `cinv x0, x1, eq` and `csinv x0, xzr, xzr, ne` as `csetm x0, eq`. Rejects: too few operands, extra operands, SP/WSP, FP/SIMD, mixed x/w, unknown cond.
  - Sibling `encode_csel` (`compare_branch.rs:85`) is CSEL (op=0, op2=00). Sibling `encode_csneg` (`compare_branch.rs:127`) is CSNEG (op=1, op2=01). Different jobs; used as metamorphic XOR transforms (CSEL XOR CSINV = bit 30; CSNEG XOR CSINV = bit 10). Sibling `encode_cinv` / `encode_csetm` are aliases of CSINV (different mnemonic/arity); used as metamorphic alias transforms.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules — not rewritten.
- **Buildability probe:** `cargo test --lib encode_csinc_kat_llvm_mc_x0_x1_x2_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 923 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_csinv_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_csinc_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_csinv (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_csinv):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `csinv Rd, Rn, Rm, cond` with valid AArch64 CSINV operands (same-width GPR Rd/Rn/Rm including xzr/wzr/lr, cond in Cond16 including al/nv and hs/lo aliases). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CSINV decoder rejected: none exists. encode_csel / encode_csneg fail the same-job sibling gate as *differential* references (different op/op2); used as algebraic metamorphic XOR transforms. encode_cinv / encode_csetm fail the same-job sibling gate as differential (different mnemonic/arity); used as algebraic metamorphic alias transforms (CINV == CSINV Rd,Rn,Rn,invert(cond); CSETM == CSINV Rd,ZR,ZR,invert(cond)). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Reg(rm), Cond(c)]` <-> `csinv rd, rn, rm, c`.
- **Seeds:** encode_csinc_pbt (same tree, CSINC twin of this CSINV): differential vs llvm-mc, metamorphic XOR vs architectural sibling, word-layout invariant, neg arity / extra / wrong-reg / unknown-cond / invalid-name / bad kind. encode_cinv_pbt::encode_cinv_meta_vs_csinv at compare_branch.rs:4152 and encode_csetm_pbt::encode_csetm_meta_vs_csinv at compare_branch.rs:8423.
- **State machine:** not applicable — encode_csinv is a pure function with no mutating operations or lifecycle.

## Module: encode_csinv
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / non-Reg-or-Cond — two properties added and passed; extra/SP/mixed/FP filed as bugs)
