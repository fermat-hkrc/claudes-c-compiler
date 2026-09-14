# PBT Campaign: encode_div

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/arm/assembler/encoder/mod.rs:1-7`: Encodes AArch64 instructions into 32-bit machine code words; covers the subset emitted by codegen; always 4 bytes little-endian.
  - Assembler README `src/backend/arm/assembler/README.md:5-14`: accepts the same textual assembly that GCC's gas would consume; GNU-style assembly text as emitted by AArch64 codegen.
  - Assembler README `src/backend/arm/assembler/README.md:214`: Data Processing category lists `udiv`, `sdiv`.
  - Assembler README `src/backend/arm/assembler/README.md:506`: `data_processing.rs` covers multiply/divide among data-processing encodings.
  - Dispatch `encoder/mod.rs:272-273`: `"udiv" => encode_div(operands, true)`, `"sdiv" => encode_div(operands, false)`.
  - Body at `data_processing.rs:618-628`: Data-processing (2 source) `sf 0 S=0 11010110 Rm 00001 o1 Rn Rd` with `o1=0` for UDIV and `o1=1` for SDIV.
  - ARM ARM UDIV/SDIV (architectural): `UDIV/SDIV <Wd>, <Wn>, <Wm>` / `UDIV/SDIV <Xd>, <Xn>, <Xm>`. Encoding: `sf 0 0 11010110 Rm 00001 o1 Rn Rd` (opcode 000010=UDIV, 000011=SDIV). Register 31 is XZR/WZR, never SP/WSP. Wt/Xt only. Same-width GPRs. No shifted-register or immediate form.
  - `get_reg` (`encoder/mod.rs:956-966`) requires Operand::Reg; `parse_reg_num` (`encoder/mod.rs:131-148`) maps sp/wsp/xzr/wzr→31, lr→30, and x/w/d/s/q/v/h/b prefixes; `sf_bit` from Rd width only.
  - llvm-mc `-triple=aarch64` accepts `udiv x0, x1, x2` as 0x9ac20820; `sdiv x0, x1, x2` as 0x9ac20c20; `udiv w0, w1, w2` as 0x1ac20820; `sdiv w0, w1, w2` as 0x1ac20c20; `udiv xzr, x0, x1` as 0x9ac1081f; `udiv lr, x0, x1` as 0x9ac1081e. Rejects: too few operands, extra operands, SP/WSP, FP/SIMD, mixed x/w, immediate.
  - Codegen callers (`src/backend/arm/codegen/alu.rs:172-210`) emit `sdiv`/`udiv` with same-width w0-w3 or x0-x3 only.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `data_processing.rs` already has prior-campaign modules (`encode_add_sub_pbt`, `encode_adc_pbt`, `encode_bic_pbt`, `encode_bics_pbt`) — not rewritten.
- **Buildability probe:** `cargo test --lib encode_adc_kat_llvm_mc_adc_x0_x1_x2 -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 959 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_div_pbt` at the bottom of `src/backend/arm/assembler/encoder/data_processing.rs` (inline layout; sibling of `encode_adc_pbt`). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_div (data_processing.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in data_processing.rs are indexed but not tested.
- **Oracle (encode_div):** Differential (llvm-mc -triple=aarch64 -show-encoding) for `udiv`/`sdiv Rd, Rn, Rm` with valid AArch64 UDIV/SDIV operands (same-width GPR Rd/Rn/Rm including xzr/wzr/lr). State machine rejected: pure function, no lifecycle. Round-trip with an in-tree UDIV/SDIV decoder rejected: none exists. encode_adc / encode_mul fail the same-job sibling gate as differential references (different opcode class); UDIV vs SDIV used as algebraic metamorphic XOR transform (differ only by o1 at bit 10). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text. Mapping: `[Reg(rd), Reg(rn), Reg(rm)]` + `unsigned` <-> `udiv`/`sdiv rd, rn, rm`.
- **Seeds:** encode_adc_pbt (same file, 3-GPR data-processing twin): differential vs llvm-mc, metamorphic S-bit XOR, word-layout invariant, neg arity / extra / wrong-reg / mixed-width / SP / FP. encode_adc_pbt::encode_adc_diff_gpr_same_width at data_processing.rs:2007.
- **State machine:** not applicable — encode_div is a pure function with no mutating operations or lifecycle.

## Module: encode_div
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg None / parse_reg_num None / FP prefixes — invalid-name property passed; FP filed as a bug; extra/mixed/SP already filed)
