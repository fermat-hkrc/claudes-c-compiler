# PBT Campaign: encode_cinv

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers conditional aliases (CINC/CINV/CNEG live there). Conditional-select table at README.md:219 lists `csinv`/`csetm` (the architectural forms CINV aliases).
  - Dispatch `encoder/mod.rs:899`: `"cinv" => encode_cinv(operands)`.
  - Function comment `compare_branch.rs:308`: `Encode CINV Rd, Rn, cond -> CSINV Rd, Rn, Rn, invert(cond)`.
  - Sibling `encode_csinv` (`compare_branch.rs:113-125`) encodes the 4-operand CSINV form; sibling `encode_csetm` (`compare_branch.rs:155-167`) is `CSINV Rd, XZR, XZR, invert(cond)`.
  - ARM ARM Conditional Invert (alias of CSINV): `CINV Rd, Rn, cond` is `CSINV Rd, Rn, Rn, invert(cond)` and is not a valid alias when cond is AL or NV. CSINV encoding: sf 1 0 11010100 Rm cond 0 0 Rn Rd. invert(cond) = cond XOR 1.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg at idx; `encode_cond` (`encoder/mod.rs:169-188`) maps 16 cond codes plus aliases hs/cs, lo/cc (including al=14, nv=15); `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes.
  - llvm-mc `-triple=aarch64` rejects: AL/NV, SP/WSP, mixed x/w, FP/SIMD, extra operands, too few operands, invalid names.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules (`encode_bl_pbt`, `encode_blr_pbt`, `encode_br_pbt`, `encode_branch_pbt`, `encode_cbz_pbt`, `encode_ccmp_ccmn_pbt`, `encode_cinc_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_cinc_kat_llvm_mc_x0_x1_eq -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 761 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_cinv_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_cinc_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_cinv (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_cinv):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `cinv Rd, Rn, cond` with same-width GPR and cond not AL/NV. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CINV decoder rejected: none exists. encode_csinv is the architectural alias target (same encoding, different mnemonic/arity) so it fails the same-job sibling gate as a *differential* reference; it is used as an algebraic metamorphic transform (CINV = CSINV with Rm=Rn and invert(cond)). encode_csetm is likewise a metamorphic special case (Rn=ZR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Cond(c)]` <-> asm text `cinv rd, rn, c`.
- **Seeds:** encode_cinc_pbt (same file): encode_cinc_diff_llvm_mc, encode_cinc_meta_vs_csinc, encode_cinc_meta_vs_cset, encode_cinc_neg_arity / _extra_operand / _wrong_reg.

## Module: encode_cinv
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / bad operand kinds — all three added properties passed; extra/AL-NV/SP/FP/mixed filed as bugs)
