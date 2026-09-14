# PBT Campaign: encode_neg

## Scan findings
- **Spec:** (none found as a standalone requirement/dev-task document). In-tree contract for this symbol:
  - Encoder module docstring at `src/backend/riscv/assembler/encoder/mod.rs:1-7`: Encodes RISC-V instructions into 32-bit machine code words; RV64GC + Zbb subset; base instructions always 4 bytes little-endian; six formats R/I/S/B/U/J.
  - Assembler README `src/backend/riscv/assembler/README.md:1-10`: translates textual assembly as emitted by RISC-V codegen into ELF `.o`; RV64GC.
  - Assembler README expansion table `src/backend/riscv/assembler/README.md:321`: `neg rd, rs` expands to `sub rd, x0, rs`.
  - Dispatch `encoder/mod.rs:749`: `"neg" => encode_neg(operands)`. Sibling `"negw" => encode_negw` is 32-bit `subw` (different job).
  - Body at `pseudo.rs:239-242`: `rd = get_reg(0); rs2 = get_reg(1); encode_r(OP_OP, rd, 0b000, 0, rs2, 0b0100000)` — comment `sub rd, x0, rs2`.
  - R-type layout `encoder/mod.rs:272-276`: `funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]`. OP_OP = 0b0110011. SUB funct7=0100000, funct3=000, rs1=x0.
  - `get_reg` (`encoder/mod.rs:347-355`): `Operand::Reg` via `reg_num` (ABI names + x0-x31, case-folded; `s0`|`fp` = 8); `Operand::Imm(n)` for `0 <= n <= 31` accepted (`GCC sometimes emits bare register numbers (0-31) in inline asm`).
  - Same-job sibling: `encode_alu_reg(operands, 0b000, 0b0100000)` dispatched as `"sub"` (`encoder/mod.rs:502`, `base.rs:260-264`). Independence is weak (shared `encode_r`/`get_reg`); still a documented expansion identity, not the primary oracle.
  - Callers: codegen `alu.rs:23` `neg t0, t0`; `atomics.rs:450` `neg t2, t2`; `intrinsics.rs:395,404,419,427` `neg t3/t5`.
  - RISC-V Unprivileged ISA pseudoinstruction table: `NEG rd, rs` = `SUB rd, x0, rs` (two's complement). SUB R-type opcode OP=0110011, funct3=000, funct7=0100000.
  - llvm-mc `-triple=riscv64 -show-encoding` confirms encodings, rejects extra operand, too few operands, FP regs, immediates-as-asm-text.
- **Test layout:** Project-owned tests are inline `#[cfg(test)] mod <name>` inside source files, discovered by `cargo test --lib`. Filename convention: `#[test] fn` inside the source file's test module. `proptest` is already a `[dev-dependencies]` entry (`Cargo.toml`). Historical `pbt-out/`/`pbt-native/` excluded from layout discovery. No crate-root `tests/` directory. `pseudo.rs` has no existing test module.
- **Buildability probe:** `cargo test --lib test_ascii_passthrough -- --test-threads=${PBT_TEST_JOBS}` in `/home/toan/github/claudes-c-compiler` → `test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1358 filtered out`. Rung 1 available; project cargo harness builds and runs.
- **Harness placement:** extend existing `cargo test --lib` target (rung 1) by appending a new `#[cfg(test)] mod encode_neg_pbt` at the bottom of `src/backend/riscv/assembler/encoder/pseudo.rs` (inline layout). Reuse the existing `proptest` dev-dependency. Not pbt-native: project cargo harness builds and runs. Historical `pbt-out/`/`pbt-native/` were excluded from layout discovery. Rung 1 was available (probe passed).
- **Candidate modules:** encode_neg (pseudo.rs)
- **Skipped modules:** (none) — campaign scoped to this single symbol; other functions in pseudo.rs are indexed but not tested.
- **Oracle (encode_neg):** Differential (llvm-mc -triple=riscv64 -show-encoding) for valid two-operand integer-register NEG. State machine rejected: pure function, no lifecycle. Round-trip with an in-tree decoder rejected: none exists. encode_negw fails the same-job gate (SUBW / OP-32). encode_alu_reg(sub) is the documented expansion but shares encode_r/get_reg (independence disclosed; used as a weaker metamorphic, not the primary differential). SUT-boundary: internal-helper of the RISC-V assembler; public contract is encoding codegen-emitted GNU-style RISC-V text / RISC-V ISA NEG pseudo. Mapping: `operands` <-> `neg rd, rs`.
- **Seeds:** README.md:321 expansion table; compress.rs tests of ADD/MV encodings (not NEG itself). No existing unit test of encode_neg. Seed: (none for this symbol).
- **State machine:** not applicable — encode_neg is a pure function with no mutating operations or lifecycle.

## Module: encode_neg
- [x] Scan: identify targets
- [x] Plan: formalize properties
- [x] Test: write and run
- [x] Review: triage results (1 coverage-sweep round: coverage_gaps had no profraw; manual arm audit of get_reg Reg/Imm/other/missing + extra operand + SUB expansion vs llvm-mc — added encode_neg_diff_llvm_mc_sub and SymbolOffset/MemSymbol invalid kinds)
