# PBT Campaign: encode_cneg

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:219`: Conditional select category lists `csel`, `csinc`, `csinv`, `csneg`, `cset`, `csetm`, `fcsel` (CNEG is the CSNEG alias, dispatched separately).
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare, conditional select, branches, and conditional aliases.
  - Dispatch `encoder/mod.rs:897`: `"cneg" => encode_cneg(operands)`.
  - Function comment `compare_branch.rs:275-276`: `CNEG Rd, Rn, cond -> CSNEG Rd, Rn, Rn, invert(cond)`.
  - Sibling `encode_csneg` (`compare_branch.rs:127-138`) is the architectural CSNEG form (4 operands, no invert). Sibling `encode_cinc` / `encode_cinv` are different jobs (CSINC / CSINV aliases).
  - ARM ARM Conditional Negate (alias of CSNEG): `CNEG <Wd>, <Wn>, <cond>` ≡ `CSNEG <Wd>, <Wn>, <Wn>, invert(<cond>)`; not valid for AL/NV. Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `encode_cond` (`encoder/mod.rs:169-190`) maps 16 cond names including al/nv and hs/lo aliases.
  - llvm-mc `-triple=aarch64` rejects: too few operands, extra operands, XZR/WZR vs SP as SP, mixed x/w, FP/SIMD, AL/NV. Accepts `cneg x0, x1, eq` as encoding 0xda811420 (same as `csneg x0, x1, x1, ne`).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules (`encode_bl_pbt`, `encode_blr_pbt`, `encode_br_pbt`, `encode_branch_pbt`, `encode_cbz_pbt`, `encode_ccmp_ccmn_pbt`, `encode_cinc_pbt`, `encode_cinv_pbt`, `encode_cmn_pbt`, `encode_cmp_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_cinc_kat_llvm_mc_x0_x1_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 831 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_cneg_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_cinc_pbt` / `encode_cinv_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_cneg (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_cneg):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `cneg Rd, Rn, cond` with valid AArch64 CNEG operands (same-width GPR, cond in Cond14). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CNEG decoder rejected: none exists. encode_csneg is the architectural alias target (same encoding, different mnemonic/arity) so it fails the same-job sibling gate as a *differential* reference; it is used as an algebraic metamorphic transform (CNEG = CSNEG with Rm=Rn and invert(cond)). encode_cinc / encode_cinv are different jobs (CSINC / CSINV). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Cond(c)]` <-> `cneg rd, rn, c`.
- **Seeds:** encode_cinc_pbt / encode_cinv_pbt (same tree): differential vs llvm-mc, metamorphic vs architectural form, word-layout invariant, neg arity / extra / AL-NV / wrong-reg.

## Module: encode_cneg
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / non-Reg-or-Cond — two properties added and passed; extra/AL-NV/SP/mixed/FP filed as bugs)
