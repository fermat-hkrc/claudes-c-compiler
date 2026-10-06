# Confirmed invariants (encode_fp_arith_d)

- Valid 3-operand `fadd.d`/`fsub.d`/`fmul.d`/`fdiv.d` with FP rd, rs1, rs2 matches llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). Default omitted rm is DYN (111). KAT pins fadd.d fa0, fa1, fa2 = 0x02c5f553, fsub.d ft0, ft1, ft2 = 0x0a20f053, fmul.d fs0, fs1, fs2 = 0x1324f453, fdiv.d ft11, ft0, fa0 = 0x1aa07fd3, fadd.d f0, f1, f2 = 0x0220f053.
- Valid 4th RoundingMode in {rne,rtz,rdn,rup,rmm,dyn} matches llvm-mc (1000 cases). KAT pins fadd.d fa0, fa1, fa2, rne = 0x02c58553 and ..., rtz = 0x02c59553.
- R-type OP-FP layout holds: opcode=0b1010011, rm in funct3[14:12], rd/rs1/rs2/funct7 as given, D fmt bits 26:25 == 01 (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd/rs1/rs2 as fN (1000 cases).
- Omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111 (1000 cases).
- Empty, 1-operand, 2-operand, GPR in an FP slot, and non-Reg rd return Err (1000 cases).
- A 5th operand and a 4th non-RoundingMode operand currently disagree with llvm-mc (see bugs): extra ignored; non-RM 4th mapped to rm=DYN.

## Environment (encode_fp_arith_d)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D arith; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs, cargo test --lib encode_fp_arith_d_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:749-752 fadd.d/fsub.d/fmul.d/fdiv.d => encode_fp_arith_d with funct7 0000001/0000101/0001001/0001101. encode_fp_arith_d (float.rs:77) is a one-line forward to encode_fp_arith. Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / rm / R-type+D-fmt / ABI / dyn-default / arity-GPR / extra / non-rm 4th. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fp_arith_d)

- llvm-mc prints `fadd.d f0, f1, f2` as ft0, ft1, ft2 (same encoding). `fadd.d ..., dyn` is printed without the dyn token; encoding still has rm=111.
- parse_rm lowercases and maps unknown strings to 0b111; the parser only constructs RoundingMode for the closed set {rne,rtz,rdn,rup,rmm,dyn}, so unknown RM strings are not caller-reachable through encode_instruction.
- encode_fp_arith_d does not range-check funct7; callers supply the ISA D-extension funct7 (fmt bit set).
- proptest 1.11 requires `#[test]` inside `proptest!`.
- Filter `cargo test --lib encode_fp_arith_d` also matches ARM/RISC-V `encode_fp_arith_diff_*` (substring `encode_fp_arith_d`); use `encode_fp_arith_d_pbt`.
