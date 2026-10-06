# Confirmed invariants (encode_fp_cmp)

- Valid 3-operand FEQ/FLT/FLE .S/.D with GPR rd and FP rs1, rs2 match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). KAT pins feq.s a0, fa1, fa2 = 0xa0c5a553, flt.s t0, fs0, fs1 = 0xa09412d3, fle.s zero, ft0, ft1 = 0xa0100053, feq.d a0, fa1, fa2 = 0xa2c5a553, flt.d t6, ft11, ft10 = 0xa3ef9fd3, fle.d x0, f0, f1 = 0xa2100053, feq.s x31, f0, f31 = 0xa1f02fd3, feq.s fp, ft0, fa0 = 0xa0a02453, feq.s s0, f8, f10 = 0xa0a42453.
- R-type OP-FP layout holds: opcode=0b1010011, funct3 is the comparison (000 FLE, 001 FLT, 010 FEQ), rd/rs1/rs2/funct7 as given (1000 cases).
- GPR ABI names (a0/t0/fp/...) encode the same rd as xN; FP ABI names encode the same rs1/rs2 as fN (1000 cases).
- Same-register form `mn rd, rs, rs` matches llvm-mc (1000 cases).
- Empty, 1-operand, 2-operand, FP rd, GPR in an FP slot, non-Reg rd (excluding Imm 0..=31), and Imm as rs1/rs2 return Err (1000 cases).
- A 4th operand and a 4th RoundingMode currently disagree with llvm-mc (see bugs): extra ignored; rm does not overwrite funct3.

## Environment (encode_fp_cmp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs, cargo test --lib encode_fp_cmp, proptest cases=1000.
- Dispatch: encoder/mod.rs:737-739 feq.s/flt.s/fle.s; encoder/mod.rs:765-767 D counterparts. Operands passed through with ISA funct7/funct3.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / R-type / ABI / rs1=rs2 / arity-class / Imm-FP-slot / extra / rm-fourth. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fp_cmp)

- llvm-mc prints `feq.s x10, f11, f12` as a0, fa1, fa2 (same encoding).
- These instructions have no rounding-mode field (unlike FADD). llvm-mc rejects a 4th rne token.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number for rd (encoder/mod.rs:398-399). llvm-mc rejects numeric rd. get_freg does not accept Imm for rs1/rs2.
- encode_fp_cmp does not range-check funct7 or funct3; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fp_sgnj)

- Valid 3-operand FSGNJ.S/N/X, FMIN.S, FMAX.S and D counterparts with FP rd, rs1, rs2 match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). KAT pins fsgnj.s fa0, fa1, fa2 = 0x20c58553, fsgnjn.s ft0, ft1, ft2 = 0x20209053, fsgnjx.s fs0, fs1, fs2 = 0x2124a453, fmin.s fa0, fa1, fa2 = 0x28c58553, fmax.s fa0, fa1, fa2 = 0x28c59553, fsgnj.d fa0, fa1, fa2 = 0x22c58553, fsgnj.s f0, f1, f2 = 0x20208053, fmin.d ft0, ft1, ft2 = 0x2a208053, fsgnj.s ft11, ft0, ft1 = 0x20100fd3.
- R-type OP-FP layout holds: opcode=0b1010011, funct3 is the op (not rm), rd/rs1/rs2/funct7 as given (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd/rs1/rs2 as fN (1000 cases).
- Same-register form `mn rd, rs, rs` (README fmv/fabs/fneg expansion shape) matches llvm-mc for the six FSGNJ* mnemonics (1000 cases).
- Empty, 1-operand, 2-operand, GPR in an FP slot, and non-Reg rd return Err (1000 cases).
- A 4th operand and a 4th RoundingMode currently disagree with llvm-mc (see bugs): extra ignored; rm does not overwrite funct3.

## Environment (encode_fp_sgnj)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs, cargo test --lib encode_fp_sgnj, proptest cases=1000.
- Dispatch: encoder/mod.rs:730-734 fsgnj.s/fsgnjn.s/fsgnjx.s/fmin.s/fmax.s; encoder/mod.rs:758-762 D counterparts. Operands passed through with ISA funct7/funct3.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / R-type / ABI / rs1=rs2 / arity-GPR / extra / rm-fourth. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fp_sgnj)

- llvm-mc prints `fsgnj.s f0, f1, f2` as ft0, ft1, ft2 (same encoding).
- These instructions have no rounding-mode field (unlike FADD). llvm-mc rejects a 4th rne token.
- encode_fp_sgnj does not range-check funct7 or funct3; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fp_unary)

- Valid 2-operand `fsqrt.s`/`fsqrt.d` with FP rd, rs1 matches llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). Default omitted rm is DYN (111). rs2 is hardwired 00000. KAT pins fsqrt.s fa0, fa1 = 0x5805f553, fsqrt.s ft0, ft1 = 0x5800f053, fsqrt.s fs0, fs1 = 0x5804f453, fsqrt.s ft11, ft0 = 0x58007fd3, fsqrt.s f0, f1 = 0x5800f053, fsqrt.d fa0, fa1 = 0x5a05f553, fsqrt.d ft0, ft1 = 0x5a00f053.
- Valid 3rd RoundingMode in {rne,rtz,rdn,rup,rmm,dyn} matches llvm-mc (1000 cases). KAT pins fsqrt.s fa0, fa1, rne = 0x58058553 and ..., rtz = 0x58059553.
- R-type OP-FP layout holds: opcode=0b1010011, rm in funct3[14:12], rd/rs1/funct7 as given, rs2=0 (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd/rs1 as fN (1000 cases).
- Omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111 (1000 cases).
- Empty, 1-operand, GPR in an FP slot, and non-Reg rd return Err (1000 cases).
- A 4th operand and a 3rd non-RoundingMode operand currently disagree with llvm-mc (see bugs): extra ignored; non-RM 3rd mapped to rm=DYN.

## Environment (encode_fp_unary)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs, cargo test --lib encode_fp_unary_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:727 fsqrt.s => encode_fp_unary(operands, 0b0101100, 0); encoder/mod.rs:755 fsqrt.d => encode_fp_unary(operands, 0b0101101, 0). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / rm / R-type / ABI / dyn-default / arity-GPR / extra / non-rm 3rd. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fp_unary)

- llvm-mc prints `fsqrt.s f0, f1` as ft0, ft1 (same encoding). `fsqrt.s ..., dyn` is printed without the dyn token; encoding still has rm=111.
- parse_rm lowercases and maps unknown strings to 0b111; the parser only constructs RoundingMode for the closed set {rne,rtz,rdn,rup,rmm,dyn}, so unknown RM strings are not caller-reachable through encode_instruction.
- encode_fp_unary does not range-check funct7 or rs2; callers supply the ISA FSQRT funct7 and rs2=0.
- proptest 1.11 requires `#[test]` inside `proptest!`.
