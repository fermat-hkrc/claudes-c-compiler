# PBT Campaign: encode_csel

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:219`: Conditional select category lists `csel`, `csinc`, `csinv`, `csneg`, `cset`, `csetm`, `fcsel`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare, conditional select, branches, and conditional aliases.
  - Dispatch `encoder/mod.rs:308`: `"csel" => encode_csel(operands)`.
  - Codegen caller `src/backend/arm/codegen/comparison.rs:71`: `csel x0, x2, x1, ne`.
  - Parser `parser.rs:48`: Condition code for csel etc.: eq, ne, lt, gt, ...
  - ARM ARM Conditional Select (architectural, not an alias): `CSEL <Wd>, <Wn>, <Wm>, <cond>` / `CSEL <Xd>, <Xn>, <Xm>, <cond>`; encoding `sf 0 0 11010100 Rm cond 00 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only. AL and NV are valid (unlike CINC/CINV/CNEG aliases).
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `encode_cond` (`encoder/mod.rs:169-190`) maps 16 cond names including al/nv and hs/lo aliases.
  - llvm-mc `-triple=aarch64` accepts `csel x0, x1, x2, eq` as 0x9a820020; `csel w0, w1, w2, ne` as 0x1a821020; `csel x0, x1, x2, al` as 0x9a82e020; `csel x0, x1, x2, nv` as 0x9a82f020. Rejects: too few operands, extra operands, SP/WSP, mixed x/w, FP/SIMD.
  - Sibling `encode_csinc` (`compare_branch.rs:99`) is CSINC (op2=01). Sibling `encode_csinv` (`compare_branch.rs:113`) is CSINV (op=1). Different jobs (different mnemonics); used as metamorphic transforms (CSEL XOR CSINC = 1<<10; CSEL XOR CSINV = 1<<30).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules (`encode_bl_pbt`, `encode_blr_pbt`, `encode_br_pbt`, `encode_branch_pbt`, `encode_cbz_pbt`, `encode_ccmp_ccmn_pbt`, `encode_cinc_pbt`, `encode_cinv_pbt`, `encode_cmn_pbt`, `encode_cmp_pbt`, `encode_cneg_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_cinc_kat_llvm_mc_x0_x1_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 850 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_csel_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_cinc_pbt` / `encode_cneg_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_csel (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_csel):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `csel Rd, Rn, Rm, cond` with valid AArch64 CSEL operands (same-width GPR, cond in Cond16 including al/nv and hs/lo). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CSEL decoder rejected: none exists. encode_csinc / encode_csinv fail the same-job sibling gate as *differential* references (different mnemonics/op2/op); used as algebraic metamorphic transforms (XOR bit 10 / bit 30). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Reg(rm), Cond(c)]` <-> `csel rd, rn, rm, c`.
- **Seeds:** encode_cinc_pbt / encode_cneg_pbt (same tree): differential vs llvm-mc, metamorphic vs architectural sibling, word-layout invariant, neg arity / extra / wrong-reg / bad kind.

## Module: encode_csel
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / non-Reg-or-Cond — two properties added and passed; extra/SP/mixed/FP filed as bugs)
