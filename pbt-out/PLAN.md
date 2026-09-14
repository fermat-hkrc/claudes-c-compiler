# PBT Campaign: encode_eon

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:214`: Data Processing category lists `eon`.
  - Assembler README `src/backend/arm/assembler/README.md:506`: `data_processing.rs` covers ORN/EON/BICS/BIC among data-processing encodings.
  - Dispatch `encoder/mod.rs:236`: `"eon" => encode_eon(operands)`.
  - Body at `data_processing.rs:952-980`: Logical (shifted register) `sf 10 01010 shift 1 Rm imm6 Rn Rd` (opc=10, N=1). Optional 4th operand Shift {lsl,lsr,asr,ror}; unknown kinds default to lsl=00; imm6 = amount & 0x3F. No immediate form. No NEON form.
  - ARM ARM EON (shifted register): `EON <Wd>, <Wn>, <Wm>{, <shift> #<amount>}` / `EON <Xd>, <Xn>, <Xm>{, <shift> #<amount>}`. Encoding: `sf opc=10 01010 shift N=1 Rm imm6 Rn Rd`. shift in {LSL,LSR,ASR,ROR}. amount in [0,31] (32-bit) or [0,63] (64-bit). Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only. Same-width GPRs.
  - ARM ARM / GNU as assembler alias: `EON Rd, Rn, #imm` is EOR Rd, Rn, #~imm (logical immediate). llvm-mc accepts `eon x0, x1, #1` as `eor x0, x1, #0xfffffffffffffffe`.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `sf_bit` from Rd width only.
  - llvm-mc `-triple=aarch64` accepts `eon x0, x1, x2` as 0xca220020; `eon w0, w1, w2` as 0x4a220020; `eon x0, x1, x2, lsr #1` as 0xca620420; `eon xzr, x0, x1` as 0xca21001f; `eon lr, x0, x1` as 0xca21001e. Rejects: too few operands, extra non-shift operand, SP/WSP, FP/SIMD, mixed x/w, shift amount 32 (W) / 64 (X), NEON arrangement. Accepts EON-immediate as EOR #~imm.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules (`encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`, `encode_bics_pbt`, `encode_div_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_adc_kat_llvm_mc_adc_x0_x1_x2 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 975 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_eon_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_div_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_eon (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_eon):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `eon Rd, Rn, Rm{, shift #amt}` with valid AArch64 EON operands (same-width GPR including xzr/wzr/lr; shift in {lsl,lsr,asr,ror}; amount in [0,31]/[0,63]) and for the EON-immediate assembler alias. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree EON decoder rejected: none exists. encode_orn / encode_bics / encode_logical(EOR) fail the same-job sibling gate as differential references (different opcodes); EOR used as algebraic metamorphic XOR transform (differ only by N at bit 21). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Reg(rm), optional Shift]` <-> `eon rd, rn, rm{, shift #amt}`; `[Reg(rd), Reg(rn), Imm(imm)]` <-> `eon rd, rn, #imm`.
- **Seeds:** encode_bics_pbt (same file, Logical shifted-register twin with N=1): differential vs llvm-mc (reg + optional shift), metamorphic opc XOR vs BIC, word-layout invariant, neg arity / extra / mixed-width / SP / FP / shift range / unknown shift. encode_bics_pbt::encode_bics_diff_reg_llvm_mc at data_processing.rs:3274.
- **State machine:** not applicable — encode_eon is a pure function with no mutating operations or lifecycle.

## Module: encode_eon
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of unknown-shift default and parse_reg_num None — unknown shift filed as a bug; invalid names passed)
