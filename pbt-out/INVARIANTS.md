# Confirmed invariants (encode_c_mv)

- Valid 2-operand C.MV with rd ∈ {x0..x31} and rs2 ∈ {x1..x31} matches llvm-mc `-triple=riscv64 -mattr=+c -show-encoding` (1000 cases). KAT pins c.mv t1, t0 = 0x8316, c.mv x1, x2 = 0x808a, c.mv a0, a1 = 0x852e, c.mv x31, x31 = 0x8ffe, c.mv x1, x1 = 0x8086, c.mv sp, ra = 0x8106.
- CR-type layout holds: op=10, funct4=1000, rd in bits[11:7], rs2 in bits[6:2] (1000 cases).
- ABI names (zero/ra/sp/a0/t6/fp/s0/…) encode the same halfword as xN (1000 cases).
- Field isolation: rd bits[11:7] independent of rs2; rs2/op/funct4 bits independent of rd (1000 cases).
- Empty/1-operand and FP dest/src return Err (1000 cases).
- Extra operand and rs2=x0 currently disagree with llvm-mc (see bugs): extra ignored; rs2=x0 encodes as C.JR.

## Environment (encode_c_mv)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+c -show-encoding (LLVM 15.0.6). Default riscv64 without +c rejects C.MV.
- Harness: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs, cargo test --lib encode_c_mv, proptest cases=1000.
- Dispatch: encoder/mod.rs:924 "c.mv" => encode_c_mv(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_c_mv NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / CR-type / ABI / isolation / arity-FP / extra / rs2=x0. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_c_mv)

- llvm-mc prints `c.mv x1, x2` as `mv ra, sp` with a 16-bit encoding ([0x8a,0x80]).
- llvm-mc encodes `c.mv x0, x1` as a HINT; SUT also encodes it (no rd=x0 rejection, unlike C.LUI). ISA: C.MV with rd=x0 and rs2≠x0 is HINT.
- C.MV allows rd=x2 (unlike C.LUI).
- llvm-mc rejects `c.mv x1, x0` because rs2=x0 is the C.JR encoding. SUT currently emits that halfword (bug).
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rd.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_c_addi)

- Valid 2-operand C.ADDI with rd ∈ {x0..x31} and imm ∈ [-32, 31] matches llvm-mc `-triple=riscv64 -mattr=+c -show-encoding` (1000 cases). KAT pins c.addi x1, 1 = 0x0085, c.addi x1, 0 = 0x0081, c.addi x1, 31 = 0x00fd, c.addi x1, -1 = 0x10fd, c.addi x1, -32 = 0x1081, c.addi a0, 5 = 0x0515.
- CI-type layout holds for signed 6-bit imm ∈ [-32, 31]: op=01, funct3=000, rd, imm[5] in bit 12, imm[4:0] in bits 6:2 (1000 cases).
- ABI names (zero/ra/sp/a0/t6/fp/s0/…) encode the same halfword as xN (1000 cases).
- Field isolation: rd bits[11:7] independent of imm; imm/op/funct3 bits independent of rd (1000 cases).
- Empty/1-operand and FP dest return Err (1000 cases).
- Extra operand and out-of-range imm currently disagree with llvm-mc (see bugs): extra ignored; 32 truncated to imm=-32.

## Environment (encode_c_addi)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+c -show-encoding (LLVM 15.0.6). Default riscv64 without +c rejects C.ADDI.
- Harness: src/backend/riscv/assembler/encoder/encode_c_addi_pbt.rs, cargo test --lib encode_c_addi, proptest cases=1000.
- Dispatch: encoder/mod.rs:921 "c.addi" => encode_c_addi(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_c_addi NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / CI-type / ABI / isolation / arity-FP / extra / oob. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_c_addi)

- llvm-mc prints `c.addi x1, 1` as `addi ra, ra, 1` with a 16-bit encoding ([0x85,0x00]). HINT/NOP forms `c.addi x1, 0` and `c.addi x0, 0` are printed as `c.addi`. Signed immediates are accepted as decimal (`c.addi x1, -1`).
- llvm-mc encodes `c.addi x0, 1` as a HINT and `c.addi x0, 0` as C.NOP; SUT also encodes them (no rd=x0 / imm=0 rejection, unlike C.LUI). ISA: C.ADDI with rd=x0 or nzimm=0 is HINT (C.NOP when both).
- C.ADDI allows rd=x2 and imm=0 (unlike C.LUI).
- The comment field name `nzimm` is the ISA packing name; llvm-mc still accepts 0. The llvm-mc diagnostic for out-of-range values says "immediate must be non-zero in the range [-32, 31]" even though 0 is accepted.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rd.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_c_li)

- Valid 2-operand C.LI with rd ∈ {x0..x31} and imm ∈ [-32, 31] matches llvm-mc `-triple=riscv64 -mattr=+c -show-encoding` (1000 cases). KAT pins c.li x1, 0 = 0x4081, c.li x1, 1 = 0x4085, c.li x1, 31 = 0x40fd, c.li x1, -1 = 0x50fd, c.li x1, -32 = 0x5081, c.li a0, 5 = 0x4515.
- CI-type layout holds for signed 6-bit imm ∈ [-32, 31]: op=01, funct3=010, rd, imm[5] in bit 12, imm[4:0] in bits 6:2 (1000 cases).
- ABI names (zero/ra/sp/a0/t6/fp/s0/…) encode the same halfword as xN (1000 cases).
- Field isolation: rd bits[11:7] independent of imm; imm/op/funct3 bits independent of rd (1000 cases).
- Empty/1-operand and FP dest return Err (1000 cases).
- Extra operand and out-of-range imm currently disagree with llvm-mc (see bugs): extra ignored; 32 truncated to imm=-32.

## Environment (encode_c_li)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+c -show-encoding (LLVM 15.0.6). Default riscv64 without +c rejects C.LI.
- Harness: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs, cargo test --lib encode_c_li, proptest cases=1000.
- Dispatch: encoder/mod.rs:918 "c.li" => encode_c_li(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_c_li NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / CI-type / ABI / isolation / arity-FP / extra / oob. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_c_li)

- llvm-mc prints `c.li x1, 0` as `li ra, 0` with a 16-bit encoding ([0x81,0x40]). Signed immediates are accepted as decimal (`c.li x1, -1`).
- llvm-mc encodes `c.li x0, 1` as a HINT; SUT also encodes it (no rd=x0 rejection, unlike C.LUI). ISA: C.LI with rd=x0 is HINT.
- C.LI allows rd=x2 and imm=0 (unlike C.LUI).
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rd.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_c_lui)

- Valid 2-operand C.LUI with rd ∉ {x0,x2} and imm in [1,31]∪[1048544,1048575] matches llvm-mc `-triple=riscv64 -mattr=+c -show-encoding` (1000 cases). KAT pins c.lui x1, 1 = 0x6085, c.lui x1, 31 = 0x60fd, c.lui x1, 1048544 = 0x7081, c.lui x1, 1048575 = 0x70fd, c.lui a0, 1 = 0x6505, c.lui x31, 1 = 0x6f85.
- CI-type layout holds for signed 6-bit nzimm ∈ [-32,-1]∪[1,31]: op=01, funct3=011, rd, nzimm[5] in bit 12, nzimm[4:0] in bits 6:2 (1000 cases).
- ABI names (ra/a0/t6/fp/s0/…) encode the same halfword as xN (1000 cases).
- Signed negative nzimm equals the 20-bit LUI-style form (nzimm & 0xfffff) and matches llvm-mc of that uimm20 (1000 cases).
- rd ∈ {x0,zero,x2,sp} returns Err (1000 cases). nzimm=0 returns Err and llvm-mc rejects (1000 cases). Empty/1-operand and FP dest return Err (1000 cases).
- Extra operand and out-of-range imm currently disagree with llvm-mc (see bugs): extra ignored; 32 truncated to nzimm=-32.

## Environment (encode_c_lui)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+c -show-encoding (LLVM 15.0.6). Default riscv64 without +c rejects C.LUI.
- Harness: src/backend/riscv/assembler/encoder/encode_c_lui_pbt.rs, cargo test --lib encode_c_lui, proptest cases=1000.
- Dispatch: encoder/mod.rs:915 "c.lui" => encode_c_lui(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_c_lui NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit plus signed-vs-uimm20 metamorphic. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_c_lui)

- llvm-mc prints `c.lui x1, 1` as `lui ra, 1` with a 16-bit encoding ([0x85,0x60]). Negative nzimm is written as 20-bit unsigned (0xfffe0..0xfffff); `c.lui x1, -1` is a syntax error, `c.lui x1, 1048575` is accepted.
- llvm-mc encodes `c.lui x0, 1` as a HINT; SUT returns Err ("rd cannot be x0 or x2"), matching the ISA C.LUI constraint and compress.rs:30.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rd.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fma)

- Valid 4-operand FMADD/FMSUB/FNMSUB/FNMADD .S/.D with four FP registers match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). Default omitted rm is DYN (111). KAT pins fmadd.s fa0, fa1, fa2, fa3 = 0x68c5f543, fmadd.s ft0, ft1, ft2, ft3 = 0x1820f043, fmsub.s fs0, fs1, fs2, fs3 = 0x9924f447, fnmsub.s fa0, fa1, fa2, fa3 = 0x68c5f54b, fnmadd.s fa0, fa1, fa2, fa3 = 0x68c5f54f, fmadd.d fa0, fa1, fa2, fa3 = 0x6ac5f543, fmadd.s f0, f1, f2, f3 = 0x1820f043, fmadd.s f0, f0, f0, f0 = 0x00007043, fmadd.s f31, f31, f31, f31 = 0xf9ffffc3.
- Valid 5th RoundingMode in {rne,rtz,rdn,rup,rmm,dyn} matches llvm-mc (1000 cases). KAT pins fmadd.s fa0, fa1, fa2, fa3, rne = 0x68c58543 and ..., rtz = 0x68c59543.
- R4-type layout holds: opcode in {0b1000011,0b1000111,0b1001011,0b1001111}, fmt in bits 26:25 (00 S / 01 D), rs3 in bits 31:27, rm in bits 14:12, rd/rs1/rs2 as given (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd/rs1/rs2/rs3 as fN (1000 cases).
- Omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111 (1000 cases).
- Empty, 1-operand, 2-operand, 3-operand, GPR in any of the four FP slots, and non-Reg rd return Err (1000 cases).
- A 6th operand and a 5th non-RoundingMode operand currently disagree with llvm-mc (see bugs): extra ignored; non-RM 5th mapped to rm=DYN.

## Environment (encode_fma)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs, cargo test --lib encode_fma_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:797-804 fmadd/fmsub/fnmsub/fnmadd .s/.d => encode_fma with OP_FMADD/OP_FMSUB/OP_FNMSUB/OP_FNMADD and fmt 0b00 S / 0b01 D. Operands passed through with ISA opcode and fmt.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_fma NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 4-op / rm / R4 / ABI / dyn-default / arity-GPR / extra / non-rm 5th. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fma)

- llvm-mc prints `fmadd.s f0, f1, f2, f3` as ft0, ft1, ft2, ft3 (same encoding). `fmadd.s ..., dyn` is printed without the dyn token; encoding still has rm=111.
- parse_rm lowercases and maps unknown strings to 0b111; the parser only constructs RoundingMode for the closed set {rne,rtz,rdn,rup,rmm,dyn}, so unknown RM strings are not caller-reachable through encode_instruction.
- encode_fma does not range-check opcode or fmt; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fmv_f_x)

- Valid 2-operand FMV.W.X / FMV.S.X / FMV.D.X with FP rd and GPR rs1 match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). KAT pins fmv.w.x fa0, a1 = 0xf0058553, fmv.s.x fa0, a1 = 0xf0058553 (alias), fmv.w.x ft0, t0 = 0xf0028053, fmv.w.x ft0, zero = 0xf0000053, fmv.d.x fa0, a1 = 0xf2058553, fmv.d.x ft11, t6 = 0xf20f8fd3, fmv.d.x f0, x0 = 0xf2000053, fmv.w.x f31, x0 = 0xf0000fd3, fmv.w.x ft0, fp = 0xf0040053, fmv.w.x f8, s0 = 0xf0040453.
- R-type OP-FP layout holds: opcode=0b1010011, funct3 hardwired 000, rs2 hardwired 0, rd/rs1/funct7 as given (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd as fN; GPR ABI names encode the same rs1 as xN; fp aliases s0/x8 (1000 cases).
- FMV.W.X vs FMV.D.X differ only in funct7 bit 0 (1 << 25) (1000 cases).
- fmv.w.x and fmv.s.x share funct7=0b1111000 and match llvm-mc identically (1000 cases).
- Empty, 1-operand, GPR rd, FP rs1, non-Reg rd (including Imm), and Imm as rd return Err (1000 cases).
- A 3rd operand and a 3rd RoundingMode currently disagree with llvm-mc (see bugs): extra ignored; rm does not overwrite funct3=000.

## Environment (encode_fmv_f_x)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs, cargo test --lib encode_fmv_f_x, proptest cases=1000.
- Dispatch: encoder/mod.rs:764 fmv.w.x | fmv.s.x => encode_fmv_f_x(operands, 0b1111000, 0b00); encoder/mod.rs:794 fmv.d.x => encode_fmv_f_x(operands, 0b1111001, 0b00). Operands passed through with ISA funct7. `_fmt` is unused; fmt lives in funct7.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_fmv_f_x NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / R-type / ABI / S-vs-D / w-vs-s alias / arity-class / extra / rm-third. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fmv_f_x)

- llvm-mc prints `fmv.w.x f10, x11` as fa0, a1 and `fmv.s.x` as `fmv.w.x` (same encoding).
- These instructions have no rounding-mode field (unlike FCVT). llvm-mc rejects a 3rd rne token.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number for rs1 (encoder/mod.rs:412-413). llvm-mc rejects numeric rs1. get_freg does not accept Imm for rd.
- encode_fmv_f_x does not range-check funct7; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fmv_x_f)

- Valid 2-operand FMV.X.W / FMV.X.S / FMV.X.D with GPR rd and FP rs1 match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). KAT pins fmv.x.w a0, fa1 = 0xe0058553, fmv.x.s a0, fa1 = 0xe0058553 (alias), fmv.x.w t0, fs0 = 0xe00402d3, fmv.x.w zero, ft0 = 0xe0000053, fmv.x.d a0, fa1 = 0xe2058553, fmv.x.d t6, ft11 = 0xe20f8fd3, fmv.x.d x0, f0 = 0xe2000053, fmv.x.w x31, f0 = 0xe0000fd3, fmv.x.w fp, ft0 = 0xe0000453, fmv.x.w s0, f8 = 0xe0040453.
- R-type OP-FP layout holds: opcode=0b1010011, funct3 hardwired 000, rs2 hardwired 0, rd/rs1/funct7 as given (1000 cases).
- GPR ABI names (a0/t0/fp/...) encode the same rd as xN; FP ABI names encode the same rs1 as fN (1000 cases).
- FMV.X.W vs FMV.X.D differ only in funct7 bit 0 (1 << 25) (1000 cases).
- fmv.x.w and fmv.x.s share funct7=0b1110000 and match llvm-mc identically (1000 cases).
- Empty, 1-operand, FP rd, GPR in the FP slot, non-Reg rd (excluding Imm 0..=31), and Imm as rs1 return Err (1000 cases).
- A 3rd operand and a 3rd RoundingMode currently disagree with llvm-mc (see bugs): extra ignored; rm does not overwrite funct3=000.

## Environment (encode_fmv_x_f)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs, cargo test --lib encode_fmv_x_f, proptest cases=1000.
- Dispatch: encoder/mod.rs:759 fmv.x.w | fmv.x.s => encode_fmv_x_f(operands, 0b1110000, 0b00); encoder/mod.rs:789 fmv.x.d => encode_fmv_x_f(operands, 0b1110001, 0b00). Operands passed through with ISA funct7. `_fmt` is unused; fmt lives in funct7.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_fmv_x_f NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / R-type / ABI / S-vs-D / w-vs-s alias / arity-class / extra / rm-third. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fmv_x_f)

- llvm-mc prints `fmv.x.w x10, f11` as a0, fa1 and `fmv.x.s` as `fmv.x.w` (same encoding).
- These instructions have no rounding-mode field (unlike FCVT). llvm-mc rejects a 3rd rne token.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number for rd (encoder/mod.rs:410-411). llvm-mc rejects numeric rd. get_freg does not accept Imm for rs1.
- encode_fmv_x_f does not range-check funct7; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fcvt_fp)

- Valid 2-operand FCVT.S.D with FP rd and FP rs1 matches llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). Default omitted rm is DYN (111). rs2 encodes source format (00001 D). KAT pins fcvt.s.d fa0, fa1 = 0x4015f553, fcvt.s.d ft0, ft1 = 0x4010f053, fcvt.s.d fs0, fs1 = 0x4014f453, fcvt.s.d ft11, ft0 = 0x40107fd3, fcvt.s.d f0, f1 = 0x4010f053.
- Valid 3rd RoundingMode in {rne,rtz,rdn,rup,rmm,dyn} matches llvm-mc for FCVT.S.D (1000 cases). KAT pins fcvt.s.d fa0, fa1, rne = 0x40158553 and ..., rtz = 0x40159553.
- R-type OP-FP layout holds for both mnemonics including FCVT.D.S: opcode=0b1010011, rm in funct3[14:12], rd/rs1/rs2/funct7 as given (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd/rs1 as fN (1000 cases).
- Omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111 for both mnemonics (1000 cases).
- Empty, 1-operand, GPR rd, GPR rs1, and non-Reg rd return Err (1000 cases).
- A 4th operand and a 3rd non-RoundingMode operand currently disagree with llvm-mc (see bugs): extra ignored; non-RM 3rd mapped to rm=DYN.
- 2-operand FCVT.D.S currently disagrees with llvm-mc (see bugs): SUT omitted-rm=DYN, llvm-mc omitted-rm=RNE.

## Environment (encode_fcvt_fp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs, cargo test --lib encode_fcvt_fp, proptest cases=1000.
- Dispatch: encoder/mod.rs:785 fcvt.s.d => encode_fcvt_fp(operands, 0b0100000, 0b00001); encoder/mod.rs:786 fcvt.d.s => encode_fcvt_fp(operands, 0b0100001, 0b00000). Operands passed through with ISA funct7 and rs2.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_fcvt_fp NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / rm / R-type / ABI / dyn-default / arity-GPR / extra / non-rm 3rd. Closed: tier round spent; remaining documented gaps are the three filed bugs.

## Quirks (encode_fcvt_fp)

- llvm-mc prints `fcvt.s.d f0, f1` as ft0, ft1 (same encoding). `fcvt.s.d ..., dyn` is printed without the dyn token; encoding still has rm=111.
- llvm-mc 15 special-cases FCVT.D.S (float32→double is exact): it hardwires rm=RNE (000) and rejects an rm operand. This encoder uses omitted-rm=DYN. Filed as B3 (encoding mismatch vs llvm-mc).
- parse_rm lowercases and maps unknown strings to 0b111; the parser only constructs RoundingMode for the closed set {rne,rtz,rdn,rup,rmm,dyn}, so unknown RM strings are not caller-reachable through encode_instruction.
- encode_fcvt_fp does not range-check funct7 or rs2; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fcvt_from_int)

- Valid 2-operand FCVT.S.{W,WU,L,LU} and FCVT.D.{L,LU} with FP rd and GPR rs1 match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). Default omitted rm is DYN (111). rs2 encodes src width/sign (00000 W, 00001 WU, 00010 L, 00011 LU). KAT pins fcvt.s.w fa0, a1 = 0xd005f553, fcvt.s.wu ft0, t0 = 0xd012f053, fcvt.s.l fs0, s0 = 0xd0247453, fcvt.s.lu fa0, a1 = 0xd035f553, fcvt.d.l ft11, t6 = 0xd22fffd3, fcvt.s.w f0, x31 = 0xd00ff053, fcvt.s.w ft0, fp = 0xd0047053, fcvt.s.w f8, s0 = 0xd0047453.
- Valid 3rd RoundingMode in {rne,rtz,rdn,rup,rmm,dyn} matches llvm-mc for those six mnemonics (1000 cases). KAT pins fcvt.s.w fa0, a1, rne = 0xd0058553 and ..., rtz = 0xd0059553.
- R-type OP-FP layout holds for all 8 mnemonics including FCVT.D.W/WU: opcode=0b1010011, rm in funct3[14:12], rd/rs1/rs2/funct7 as given (1000 cases).
- FP ABI names (ft0/fa0/fs0/...) encode the same rd as fN; GPR ABI names encode the same rs1 as xN; fp aliases s0/x8 (1000 cases).
- Omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111 for all 8 mnemonics (1000 cases).
- Empty, 1-operand, GPR rd, FP rs1, and non-Reg rd (including Imm) return Err (1000 cases).
- A 4th operand and a 3rd non-RoundingMode operand currently disagree with llvm-mc (see bugs): extra ignored; non-RM 3rd mapped to rm=DYN.

## Environment (encode_fcvt_from_int)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs, cargo test --lib encode_fcvt_from_int, proptest cases=1000.
- Dispatch: encoder/mod.rs:751-754 fcvt.s.w/wu/l/lu; encoder/mod.rs:779-782 D counterparts. Operands passed through with ISA funct7 and rs2.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_fcvt_from_int NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / rm / R-type / ABI / dyn-default / arity-class / extra / non-rm 3rd. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fcvt_from_int)

- llvm-mc prints `fcvt.s.w f10, x11` as fa0, a1 (same encoding). `fcvt.s.w ..., dyn` is printed without the dyn token; encoding still has rm=111. `fcvt.d.l ..., dyn` is printed with the dyn token.
- llvm-mc 15 special-cases FCVT.D.W/WU (int32→double is exact): it hardwires rm=RNE (000) and rejects an rm operand. This encoder uses omitted-rm=DYN. Filed as B3 (encoding mismatch vs llvm-mc).
- parse_rm lowercases and maps unknown strings to 0b111; the parser only constructs RoundingMode for the closed set {rne,rtz,rdn,rup,rmm,dyn}, so unknown RM strings are not caller-reachable through encode_instruction.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number for rs1 (encoder/mod.rs:388-389). llvm-mc rejects numeric rs1. get_freg does not accept Imm for rd.
- encode_fcvt_from_int does not range-check funct7 or rs2; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fcvt_int)

- Valid 2-operand FCVT.{W,WU,L,LU}.{S,D} with GPR rd and FP rs1 match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). Default omitted rm is DYN (111). rs2 encodes dest width/sign (00000 W, 00001 WU, 00010 L, 00011 LU). KAT pins fcvt.w.s a0, fa1 = 0xc005f553, fcvt.wu.s t0, fs0 = 0xc01472d3, fcvt.l.s zero, ft0 = 0xc0207053, fcvt.lu.s a0, fa1 = 0xc035f553, fcvt.w.d a0, fa1 = 0xc205f553, fcvt.l.d t6, ft11 = 0xc22fffd3, fcvt.w.s x31, f0 = 0xc0007fd3, fcvt.w.s fp, ft0 = 0xc0007453, fcvt.w.s s0, f8 = 0xc0047453.
- Valid 3rd RoundingMode in {rne,rtz,rdn,rup,rmm,dyn} matches llvm-mc (1000 cases). KAT pins fcvt.w.s a0, fa1, rne = 0xc0058553 and ..., rtz = 0xc0059553.
- R-type OP-FP layout holds: opcode=0b1010011, rm in funct3[14:12], rd/rs1/rs2/funct7 as given (1000 cases).
- GPR ABI names (a0/t0/fp/...) encode the same rd as xN; FP ABI names encode the same rs1 as fN (1000 cases).
- Omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111 (1000 cases).
- Empty, 1-operand, FP rd, GPR in the FP slot, and non-Reg rd (excluding Imm 0..=31) return Err (1000 cases).
- A 4th operand and a 3rd non-RoundingMode operand currently disagree with llvm-mc (see bugs): extra ignored; non-RM 3rd mapped to rm=DYN.

## Environment (encode_fcvt_int)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs, cargo test --lib encode_fcvt_int, proptest cases=1000.
- Dispatch: encoder/mod.rs:745-748 fcvt.w/wu/l/lu.s; encoder/mod.rs:773-776 D counterparts. Operands passed through with ISA funct7 and rs2.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / rm / R-type / ABI / dyn-default / arity-class / extra / non-rm 3rd. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fcvt_int)

- llvm-mc prints `fcvt.w.s x10, f11` as a0, fa1 (same encoding). `fcvt.w.s ..., dyn` is printed without the dyn token; encoding still has rm=111.
- parse_rm lowercases and maps unknown strings to 0b111; the parser only constructs RoundingMode for the closed set {rne,rtz,rdn,rup,rmm,dyn}, so unknown RM strings are not caller-reachable through encode_instruction.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number for rd (encoder/mod.rs:398-399). llvm-mc rejects numeric rd. get_freg does not accept Imm for rs1.
- encode_fcvt_int does not range-check funct7 or rs2; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_fclass)

- Valid 2-operand FCLASS.S/D with GPR rd and FP rs1 match llvm-mc `-triple=riscv64 -mattr=+f,+d -show-encoding` (1000 cases). KAT pins fclass.s a0, fa1 = 0xe0059553, fclass.s t0, fs0 = 0xe00412d3, fclass.s zero, ft0 = 0xe0001053, fclass.d a0, fa1 = 0xe2059553, fclass.d t6, ft11 = 0xe20f9fd3, fclass.d x0, f0 = 0xe2001053, fclass.s x31, f0 = 0xe0001fd3, fclass.s fp, ft0 = 0xe0001453, fclass.s s0, f8 = 0xe0041453.
- R-type OP-FP layout holds: opcode=0b1010011, funct3 hardwired 001, rs2 hardwired 0, rd/rs1/funct7 as given (1000 cases).
- GPR ABI names (a0/t0/fp/...) encode the same rd as xN; FP ABI names encode the same rs1 as fN (1000 cases).
- FCLASS.S vs FCLASS.D differ only in funct7 bit 0 (1 << 25) (1000 cases).
- Empty, 1-operand, FP rd, GPR in the FP slot, non-Reg rd (excluding Imm 0..=31), and Imm as rs1 return Err (1000 cases).
- A 3rd operand and a 3rd RoundingMode currently disagree with llvm-mc (see bugs): extra ignored; rm does not overwrite funct3=001.

## Environment (encode_fclass)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+f,+d -show-encoding (LLVM 15.0.6). Default riscv64 without +f,+d rejects F/D; RV64GC includes both (README.md:13).
- Harness: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs, cargo test --lib encode_fclass, proptest cases=1000.
- Dispatch: encoder/mod.rs:742 fclass.s => encode_fclass(operands, 0b1110000); encoder/mod.rs:770 fclass.d => encode_fclass(operands, 0b1110001). Operands passed through with ISA funct7.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / R-type / ABI / S-vs-D / arity-class / extra / rm-third. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_fclass)

- llvm-mc prints `fclass.s x10, f11` as a0, fa1 (same encoding).
- These instructions have no rounding-mode field (unlike FSQRT). llvm-mc rejects a 3rd rne token.
- get_reg accepts Imm(0..=31) as a GCC bare GPR number for rd (encoder/mod.rs:398-399). llvm-mc rejects numeric rd. get_freg does not accept Imm for rs1.
- encode_fclass does not range-check funct7; callers supply the ISA values.
- proptest 1.11 requires `#[test]` inside `proptest!`.

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
