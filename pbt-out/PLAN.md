# PBT Campaign: encode_cset

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:219`: Conditional select category lists `csel`, `csinc`, `csinv`, `csneg`, `cset`, `csetm`, `fcsel`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare, conditional select, branches, and conditional aliases.
  - Dispatch `encoder/mod.rs:312`: `"cset" => encode_cset(operands)`.
  - Codegen callers: `src/backend/arm/codegen/comparison.rs:29` and `:40` emit `cset x0, <cond>`; `i128_ops.rs:296/298/312/315` emit `cset x0, ne/eq/<cond>`.
  - Parser `parser.rs:48`: Condition code for csel etc.: eq, ne, lt, gt, ...
  - ARM ARM Conditional Set (alias of CSINC, not architectural): `CSET <Wd>, <cond>` / `CSET <Xd>, <cond>` is equivalent to `CSINC <Wd/Xd>, WZR/XZR, WZR/XZR, invert(<cond>)` and is not valid when `<cond>` is AL or NV. Encoding is CSINC: `sf 0 0 11010100 Rm=11111 cond 01 Rn=11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only.
  - Comment at `compare_branch.rs:142`: `CSET Rd, cond -> CSINC Rd, XZR, XZR, invert(cond)`.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `encode_cond` (`encoder/mod.rs:169-190`) maps 16 cond names including al/nv and hs/lo aliases.
  - llvm-mc `-triple=aarch64` accepts `cset x0, eq` as 0x9a9f17e0; `cset w0, ne` as 0x1a9f07e0; `cset xzr, eq` as 0x9a9f17ff; `cset lr, eq` as 0x9a9f17fe. Rejects: too few operands, extra operands, SP/WSP, FP/SIMD, AL, NV.
  - Sibling `encode_csinc` (`compare_branch.rs:99`) is 4-operand CSINC. Sibling `encode_cinc` (`compare_branch.rs:293`) is CINC Rd, Rn, cond. Different jobs (different mnemonics/arity); used as metamorphic alias transforms (CSET == CSINC Rd,ZR,ZR,invert(cond); CSET == CINC Rd,ZR,cond).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules (`encode_bl_pbt`, `encode_blr_pbt`, `encode_br_pbt`, `encode_branch_pbt`, `encode_cbz_pbt`, `encode_ccmp_ccmn_pbt`, `encode_cinc_pbt`, `encode_cinv_pbt`, `encode_cmn_pbt`, `encode_cmp_pbt`, `encode_cneg_pbt`, `encode_csel_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_cinc_kat_llvm_mc_x0_x1_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 867 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_cset_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_cinc_pbt` / `encode_csel_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_cset (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_cset):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `cset Rd, cond` with valid AArch64 CSET operands (same-width GPR Rd including xzr/wzr/lr, cond in Cond14 excluding al/nv, including hs/lo aliases). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CSET decoder rejected: none exists. encode_csinc / encode_cinc fail the same-job sibling gate as *differential* references (different mnemonics/arity); used as algebraic metamorphic transforms (CSET == CSINC Rd,ZR,ZR,invert(cond); CSET == CINC Rd,ZR,cond). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Cond(c)]` <-> `cset rd, c`.
- **Seeds:** encode_cinc_pbt (same tree): `encode_cinc_kat_cset_alias` at compare_branch.rs:3757 and `encode_cinc_meta_vs_cset` at compare_branch.rs:3853; differential vs llvm-mc, metamorphic vs architectural sibling, word-layout invariant, neg arity / extra / AL-NV / wrong-reg / bad kind.
- **State machine:** not applicable — encode_cset is a pure function with no mutating operations or lifecycle.

## Module: encode_cset
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / non-Reg-or-Cond — three properties added and passed; extra/AL-NV/SP/FP filed as bugs)
