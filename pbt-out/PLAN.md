# PBT Campaign: encode_cinc

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers conditional aliases (CINC/CINV/CNEG live there). Conditional-select table at README.md:219 lists `csinc`/`cset` (the architectural forms CINC aliases).
  - Dispatch `encoder/mod.rs:898`: `"cinc" => encode_cinc(operands)`.
  - Function comment `compare_branch.rs:292`: `Encode CINC Rd, Rn, cond -> CSINC Rd, Rn, Rn, invert(cond)`.
  - Sibling `encode_csinc` (`compare_branch.rs:99-111`) encodes the 4-operand CSINC form; sibling `encode_cset` (`compare_branch.rs:141-153`) is `CSINC Rd, XZR, XZR, invert(cond)`.
  - ARM ARM Conditional Increment (alias of CSINC): `CINC Rd, Rn, cond` is `CSINC Rd, Rn, Rn, invert(cond)` and is not a valid alias when cond is AL or NV. CSINC encoding: sf 0 0 11010100 Rm cond 0 1 Rn Rd. invert(cond) = cond XOR 1.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg at idx; `encode_cond` (`encoder/mod.rs:169-188`) maps 16 cond codes plus aliases hs/cs, lo/cc (including al=14, nv=15); `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes.
  - llvm-mc `-triple=aarch64` rejects: AL/NV, SP/WSP, mixed x/w, FP/SIMD, extra operands, too few operands, invalid names.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules (`encode_bl_pbt`, `encode_blr_pbt`, `encode_br_pbt`, `encode_branch_pbt`, `encode_cbz_pbt`, `encode_ccmp_ccmn_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_br_kat_llvm_mc_br_x0 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 741 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_cinc_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_ccmp_ccmn_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_cinc (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_cinc):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `cinc Rd, Rn, cond` with same-width GPR and cond not AL/NV. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CINC decoder rejected: none exists. encode_csinc is the architectural alias target (same encoding, different mnemonic/arity) so it fails the same-job sibling gate as a *differential* reference; it is used as an algebraic metamorphic transform (CINC = CSINC with Rm=Rn and invert(cond)). encode_cset is likewise a metamorphic special case (Rn=ZR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Cond(c)]` <-> asm text `cinc rd, rn, c`.
- **Seeds:** encode_ccmp_ccmn_pbt / encode_blr_pbt (same file): encode_blr_diff_xn_llvm_mc, encode_ccmp_ccmn_meta_ccmp_vs_ccmn, encode_blr_neg_arity / _extra_operand / _wrong_reg_class.

## Module: encode_cinc
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / parse_reg_num None / bad operand kinds — all three added properties passed; extra/AL-NV/SP/FP/mixed filed as bugs)
