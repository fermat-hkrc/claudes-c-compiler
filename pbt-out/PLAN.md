# PBT Campaign: encode_cmp

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:218`: Compare category lists `cmp`.
  - Assembler README `src/backend/arm/assembler/README.md:507`: `compare_branch.rs` covers Compare (CMP/CMN/TST).
  - Dispatch `encoder/mod.rs:301`: `"cmp" => encode_cmp(operands)`.
  - Function comment `compare_branch.rs:7`: `CMP Rn, op -> SUBS XZR, Rn, op`. Width of ZR is chosen from the first operand (`is_32bit_reg` → WZR else XZR).
  - Sibling `encode_cmn` (`compare_branch.rs:22-32`) is the ADDS-XZR alias (different job). Sibling `encode_add_sub(..., is_sub=true, set_flags=true)` is the architectural SUBS form the alias expands to.
  - ARM ARM Compare (alias of SUBS): `CMP <Xn|SP>, #<imm>` ≡ `SUBS XZR, <Xn|SP>, #<imm>`; `CMP <Xn>, <Xm>{, shift}` ≡ `SUBS XZR, <Xn>, <Xm>{, shift}`; extended-register form likewise. Immediate-form Rn=31 is SP (XZR is not a valid CMP-immediate Rn). Shifted-register Rm=31 is XZR (SP as Rm requires extend).
  - Codegen `codegen/emit.rs:492-571`: emits `cmp wN/xN, #imm12` for unsigned 0..=4095 and `cmp wN/xN, wM/xM` for register compares.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `is_32bit_reg` (`encoder/mod.rs:157-160`) is w-prefix / wsp / wzr.
  - llvm-mc `-triple=aarch64` rejects: too few operands, XZR/WZR as immediate-form Rn, mixed x/w without extend, FP/SIMD, SP as Rm without extend, extra operands, ROR, shift out of range, imm that does not fit imm12/{imm12,lsl#12}. Accepts `cmp #-N` as `cmn #N` (gas rewrite).
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `compare_branch.rs` already has prior-campaign modules (`encode_bl_pbt`, `encode_blr_pbt`, `encode_br_pbt`, `encode_branch_pbt`, `encode_cbz_pbt`, `encode_ccmp_ccmn_pbt`, `encode_cinc_pbt`, `encode_cinv_pbt`, `encode_cmn_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_cmn_kat_llvm_mc_x0_imm42 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 806 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_cmp_pbt` at the bottom of `src/backend/arm/assembler/encoder/compare_branch.rs` (inline layout; sibling of `encode_cmn_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_cmp (compare_branch.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in compare_branch.rs are indexed but not tested.
- **Oracle (encode_cmp):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `cmp Rn, #imm` / `cmp Rn, Rm{, shift}` / `cmp Rn, Rm, extend` with valid AArch64 CMP operands. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree CMP decoder rejected: none exists. encode_cmn is a different job (ADDS-XZR / CMN). encode_add_sub is the architectural alias target (same encoding, different mnemonic/arity) so it fails the same-job sibling gate as a *differential* reference; it is used as an algebraic metamorphic transform (CMP = SUBS with Rd=XZR/WZR). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rn), Imm(imm)]` <-> `cmp rn, #imm`; `[Reg(rn), Reg(rm)]` <-> `cmp rn, rm`; optional `Shift`/`Extend` as a third operand.
- **Seeds:** encode_cmn_pbt / encode_add_sub_pbt (same tree): differential vs llvm-mc, metamorphic vs architectural form, word-layout invariant, neg arity / extra / wrong-reg.

## Module: encode_cmp
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of non-Reg-first / extended-register / negative-imm — all three added properties passed; extra/XZR-imm/mixed/FP/SP-Rm/i64::MIN filed as bugs)
