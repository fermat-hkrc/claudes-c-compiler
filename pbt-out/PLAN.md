# PBT Campaign: encode_ccmp_ccmn

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:218`: Compare lists `ccmp` (and `fccmp`); dispatch also encodes `ccmn`.
  - Dispatch `encoder/mod.rs:304-305`: `"ccmp" => encode_ccmp_ccmn(operands, true)`, `"ccmn" => encode_ccmp_ccmn(operands, false)`.
  - Function comment `compare_branch.rs:53-54`: `CCMP/CCMN Rn, #imm5, #nzcv, cond`; CCMP has bit 30 = 1, CCMN has bit 30 = 0.
  - Parser comment `parser.rs:1987`: `"ccmp x10, x13, 0, eq"` (register form with bare integer nzcv).
  - ARM ARM Conditional compare (immediate): bits[31]=sf, [30]=op (0=CCMN, 1=CCMP), [29]=S=1, [28:21]=11010010, [20:16]=imm5, [15:12]=cond, [11]=o2=1, [10]=0, [9:5]=Rn, [4]=0, [3:0]=nzcv. imm5 unsigned [0, 31]; nzcv unsigned [0, 15].
  - ARM ARM Conditional compare (register): same except [20:16]=Rm, [11]=o2=0.
  - Callers: `encoder/mod.rs:304-305` only. No codegen emitter of `ccmp`/`ccmn` found outside the assembler.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg at idx; `encode_cond` (`encoder/mod.rs:169-188`) maps 16 cond codes plus aliases hs/cs, lo/cc; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has `#[cfg(test)] mod encode_bl_pbt`, `mod encode_blr_pbt`, `mod encode_br_pbt`, `mod encode_branch_pbt`, `mod encode_cbz_pbt` from prior campaigns (not rewritten).
- **Buildability probe:** `cargo test --lib encode_br_kat_llvm_mc_br_x0 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 720 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_ccmp_ccmn_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_cbz_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_ccmp_ccmn (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_ccmp_ccmn):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `ccmp/ccmn Rn, #imm5, #nzcv, cond` and `ccmp/ccmn Rn, Rm, #nzcv, cond`. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CCMP decoder rejected: none exists. encode_cmp / encode_cmn are related encodings (SUBS/ADDS aliases) but different jobs so they fail the same-job sibling gate as a differential reference. CCMP vs CCMN (is_ccmp flag) is used only as a metamorphic transform (bit 30 XOR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rn), Imm(imm5), Imm(nzcv), Cond(c)]` <-> asm text `ccmp/ccmn rn, #imm5, #nzcv, c`; `[Reg(rn), Reg(rm), Imm(nzcv), Cond(c)]` <-> `ccmp/ccmn rn, rm, #nzcv, c`.
- **Seeds:** encode_cbz_pbt / encode_blr_pbt (same compare-and-branch file): encode_cbz_diff_imm_llvm_mc, encode_blr_diff_xn_llvm_mc, encode_cbz_meta_cbz_vs_cbnz, encode_blr_neg_arity / _extra_operand / _wrong_reg_class.

## Module: encode_ccmp_ccmn
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of encode_cond None / invalid rm / bad operand kinds — all three added properties passed; SP/FP/mixed/extra/OOR filed as bugs)
