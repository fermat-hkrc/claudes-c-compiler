# Confirmed invariants (encode_sgtz)

- Valid 2-operand `sgtz rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `sgtz rd, rs` and of `slt rd, x0, rs` byte-for-byte (1000 cases). KAT pins sgtz a0, a1 = slt a0, x0, a1 = 0x00b02533; sgtz zero, zero = 0x00002033; sgtz t6, ra = 0x00102fb3; sgtz fp, s0 = 0x00802433.
- R-type OP layout: opcode 0110011, funct3=010 (SLT), funct7=0000000, rs1=x0, rd and rs2 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_sgtz(rd, rs) equals encode_alu_reg SLT on [rd, x0, rs] (1000 cases).
- Field isolation: rd bits independent of rs2; non-rd bits independent of rd (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_sgtz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15) assembling `sgtz rd, rs` and `slt rd, x0, rs`.
- Harness: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs, cargo test --lib encode_sgtz_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:879 "sgtz" => encode_sgtz(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries and reported encode_sgtz NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of llvm-mc/slt/R-type/ABI/isolation/arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_sgtz)

- encode_sgtz has no rustdoc; contract is README.md:327 plus the inline SLT comment at pseudo.rs:278.
- RISC-V Unprivileged ISA SGTZ pseudo is SLT rd, x0, rs; this assembler matches that expansion (and llvm-mc).
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_sltz)

- Valid 2-operand `sltz rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `sltz rd, rs` and of `slt rd, rs, x0` byte-for-byte (1000 cases). KAT pins sltz a0, a1 = slt a0, a1, x0 = 0x0005a533; sltz zero, zero = 0x00002033; sltz t6, ra = 0x0000afb3; sltz fp, s0 = 0x00042433.
- R-type OP layout: opcode 0110011, funct3=010 (SLT), funct7=0000000, rs2=x0, rd and rs1 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_sltz(rd, rs) equals encode_alu_reg SLT on [rd, rs, x0] (1000 cases).
- Field isolation: rd bits independent of rs1; non-rd bits independent of rd (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_sltz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15) assembling `sltz rd, rs` and `slt rd, rs, x0`.
- Harness: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs, cargo test --lib encode_sltz_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:876 "sltz" => encode_sltz(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries and reported encode_sltz NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of llvm-mc/slt/R-type/ABI/isolation/arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_sltz)

- encode_sltz has no rustdoc; contract is README.md:326 plus the inline SLT comment at pseudo.rs:272.
- RISC-V Unprivileged ISA SLTZ pseudo is SLT rd, rs, x0; this assembler matches that expansion (and llvm-mc).
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_snez)

- Valid 2-operand `snez rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `snez rd, rs` and of `sltu rd, x0, rs` byte-for-byte (1000 cases). KAT pins snez a0, a1 = sltu a0, x0, a1 = 0x00b03533; snez zero, zero = 0x00003033; snez t6, ra = 0x00103fb3; snez fp, s0 = 0x00803433.
- R-type OP layout: opcode 0110011, funct3=011, funct7=0000000, rs1=x0, rd and rs2 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_snez(rd, rs) equals encode_alu_reg SLTU on [rd, x0, rs] (1000 cases).
- Field isolation: rd bits independent of rs2; non-rd bits independent of rd (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_snez)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15) assembling `snez rd, rs` and `sltu rd, x0, rs`.
- Harness: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs, cargo test --lib encode_snez_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:873 "snez" => encode_snez(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries and reported encode_snez NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of llvm-mc/sltu/R-type/ABI/isolation/arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_snez)

- encode_snez has no rustdoc; contract is README.md:325 plus the inline SLTU comment at pseudo.rs:266.
- RISC-V Unprivileged ISA SNEZ pseudo is SLTU rd, x0, rs; this assembler matches that expansion (and llvm-mc).
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_seqz)

- Valid 2-operand `seqz rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `seqz rd, rs` and of `sltiu rd, rs, 1` byte-for-byte (1000 cases). KAT pins seqz a0, a1 = sltiu a0, a1, 1 = 0x0015b513; seqz zero, zero = 0x00103013; seqz t6, ra = 0x0010bf93; seqz fp, s0 = 0x00143413.
- I-type OP-IMM layout: opcode 0010011, funct3=011, imm12=1, rd and rs1 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_seqz(rd, rs) equals encode_alu_imm SLTIU on [rd, rs, Imm(1)] (1000 cases).
- Field isolation: rd bits independent of rs1; opcode/funct3/rs1/imm bits independent of rd (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_seqz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding assembling `seqz rd, rs` and `sltiu rd, rs, 1`.
- Harness: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs, cargo test --lib encode_seqz_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:870 "seqz" => encode_seqz(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw for this Rust target (C++ reporter listed unrelated binaries and reported encode_seqz NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of llvm-mc/sltiu/I-type/ABI/isolation/arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_seqz)

- encode_seqz has no rustdoc; contract is README.md:324 plus the inline SLTIU comment at pseudo.rs:260.
- RISC-V Unprivileged ISA SEQZ pseudo is SLTIU rd, rs, 1; this assembler matches that expansion (and llvm-mc).
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_sext_w)

- Valid 2-operand `sext.w rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `sext.w rd, rs` and of `addiw rd, rs, 0` byte-for-byte (1000 cases). KAT pins sext.w a0, a1 = addiw a0, a1, 0 = 0x0005851b; sext.w zero, zero = 0x0000001b; sext.w t6, ra = 0x00008f9b; sext.w fp, s0 = 0x0004041b; sext.w x1, x2 = 0x0001009b.
- I-type OP-IMM-32 layout: opcode 0011011, funct3=000, imm12=0, rd and rs1 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_sext_w(rd, rs) equals encode_alu_imm_w ADDIW on [rd, rs, Imm(0)] (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_sext_w)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6) assembling `sext.w rd, rs` and `addiw rd, rs, 0`. llvm-mc pretty-prints `addiw rd, rs, 0` as `sext.w`.
- Harness: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs, cargo test --lib encode_sext_w_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:867 "sext.w" => encode_sext_w(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_sext_w NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of llvm-mc/addiw/I-type/ABI/arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_sext_w)

- encode_sext_w has no rustdoc; contract is README.md:323 `sext.w rd, rs` → `addiw rd, rs, 0` plus the inline comment at pseudo.rs:254.
- RISC-V Unprivileged ISA SEXT.W pseudo is ADDIW rd, rs, 0; this assembler matches that expansion (and llvm-mc).
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_negw)

- Valid 2-operand `negw rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `negw rd, rs` and of `subw rd, x0, rs` byte-for-byte (1000 cases). KAT pins negw a0, a1 = subw a0, x0, a1 = 0x40b0053b; negw zero, zero = 0x4000003b; negw t6, ra = 0x40100fbb; negw fp, s0 = 0x4080043b.
- R-type OP-32 layout: opcode 0111011, funct3=000, funct7=0100000, rs1=x0, rd and rs2 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_negw(rd, rs) equals encode_alu_reg_w SUBW on [rd, x0, rs] (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_negw)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6) assembling `negw rd, rs` and `subw rd, x0, rs`. llvm-mc pretty-prints `subw rd, x0, rs` as `negw`.
- Harness: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs, cargo test --lib encode_negw_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:864 "negw" => encode_negw(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_negw NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of llvm-mc/subw/R-type/ABI/arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_negw)

- encode_negw has no rustdoc or inline comment; contract is README.md:322 `negw rd, rs` → `subw rd, x0, rs`.
- RISC-V Unprivileged ISA NEGW pseudo is SUBW rd, x0, rs; this assembler matches that expansion (and llvm-mc).
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.
- encode_neg (OP / SUB) is a different job from encode_negw (OP-32 / SUBW); encodings differ in opcode only when rd/rs match.

# Confirmed invariants (encode_not)

- Valid 2-operand `not rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `not rd, rs` and of `xori rd, rs, -1` byte-for-byte (1000 cases). KAT pins not a0, a1 = xori a0, a1, -1 = 0xfff5c513; not zero, zero = 0xfff04013; not t6, ra = 0xfff0cf93; not fp, s0 = 0xfff44413.
- I-type layout: opcode 0010011, funct3=100, imm12=0xFFF (-1), rd and rs1 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_not(rd, rs) equals encode_alu_imm XORI on [rd, rs, Imm(-1)] (1000 cases).
- Field isolation: rd bits independent of rs1; opcode/funct3/rs1/imm bits independent of rd (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_not)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6) assembling `not rd, rs` and `xori rd, rs, -1`.
- Harness: src/backend/riscv/assembler/encoder/encode_not_pbt.rs, cargo test --lib encode_not_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:860 "not" => encode_not(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_not NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of llvm-mc/xori/I-type/ABI/isolation/arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_not)

- encode_not has no rustdoc; contract is README.md:320 plus the inline XORI comment at pseudo.rs:236.
- RISC-V Unprivileged ISA NOT pseudo is XORI rd, rs, -1; this assembler matches that expansion (and llvm-mc).
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_mv)

- Valid 2-operand `mv rd, rs` with rd, rs GPR matches llvm-mc `-triple=riscv64 -show-encoding` of `add rd, x0, rs` byte-for-byte (1000 cases). KAT pins mv a0, a1 = add a0, x0, a1 = 0x00b00533; mv zero, zero = 0x00000033; mv t6, ra = 0x00100fb3; mv fp, s0 = 0x00800433.
- Semantic copy: executing the SUT ADD word leaves init[rs] in rd for rd ∈ {x1..x31} and matches llvm-mc `mv` (ADDI expansion) simulated result (1000 cases).
- R-type layout: opcode 0110011, funct3=000, funct7=0000000, rs1=x0, rd and rs2 in their fields, including bounds 0 and 31 (1000 cases).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd/rs matches xN (get_reg GCC bare-number path). encode_mv(rd, rs) equals encode_alu_reg ADD on [rd, x0, rs] (1000 cases).
- Field isolation: rd bits independent of rs; non-rd bits independent of rd (1000 cases).
- Too few operands and invalid/FP/non-GPR names return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_mv)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6) assembling `add rd, x0, rs`. llvm-mc `mv rd, rs` is ADDI (`addi rd, rs, 0`); this assembler documents ADD (README.md:319, pseudo.rs:228-229).
- Harness: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs, cargo test --lib encode_mv_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:857 "mv" | "move" => encode_mv(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_mv NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of ADD encoding / semantic mv / R-type / ABI / isolation / ADD-x0 / arity-invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_mv)

- encode_mv has no rustdoc; contract is README.md:319 plus the inline ADD-vs-ADDI comment at pseudo.rs:228-229.
- RISC-V Unprivileged ISA MV pseudo is ADDI; this project uses ADD so the word is eligible for C.MV compression. Encoding mismatch with uncompressed llvm-mc `mv` is documented, not a SUT bug. Semantic copy still agrees.
- get_reg accepts Imm(0..=31) as a bare register number.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_li)

- Valid 2-operand `li rd, imm` with rd a GPR and imm in i32 matches llvm-mc `-triple=riscv64 -show-encoding` byte-for-byte (1000 cases). KAT pins li a0, 0 = 0x00000513; li a0, 1 = 0x00100513; li a0, 2047 = 0x7ff00513; li a0, -2048 = 0x80000513; li a0, 2048 = lui+addiw [0x00001537, 0x8005051b]; li a0, 2147483647 = [0x80000537, 0xfff5051b]; li x0, 0 = 0x00000013.
- Full i64 domain: executing the SUT expansion (RISC-V LUI/ADDI/ADDIW/SLLI/SRLI interpreter) leaves imm in rd for rd ∈ {x1..x31}, and matches llvm-mc's simulated result (1000 cases, sequence length ≤ 16).
- 12-bit immediates [-2048, 2047] encode as a single ADDI rd, x0, imm (opcode 0010011, funct3=000, rs1=x0).
- ABI names, xN, fp/s0, and zero/x0 aliases produce the same encoding. Imm(0..=31) as rd matches xN (get_reg GCC bare-number path).
- Too few operands and invalid rd / non-Imm second operand return Err (1000 cases).
- Extra operand currently disagrees with the README two-operand form and llvm-mc (see bugs).

## Environment (encode_li)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_li_pbt.rs, cargo test --lib encode_li, proptest cases=1000.
- Dispatch: encoder/mod.rs:854 "li" => encode_li(operands). Operands passed through. No arity check.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_li NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 12-bit Word/addi, 32-bit llvm-mc encoding, 64-bit semantic, ABI/xN, Imm-rd, arity/invalid (passing) and extra operand (filed bug). Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_li)

- encode_li has no rustdoc; contract is README.md:318 plus encode_li_32bit / encode_li_immediate comments.
- 12-bit path is addi (OP_OP_IMM); 32-bit path is lui + addiw (OP_OP_IMM_32) to match GAS on RV64; lo==0 omits addiw.
- 64-bit expansions may differ from llvm-mc (llvm-mc uses srli for i64::MAX); semantic agreement is the shared contract.
- README "up to 3-instruction sequences for 64-bit constants" is inaccurate vs both SUT and llvm-mc (dense immediates use more); not treated as a SUT bound.
- get_reg accepts Imm(0..=31) as a bare register number; get_imm accepts only Operand::Imm.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_v_crypto_vs)

- Valid 2-operand unmasked vsm4r.vs with vd, vs2 ∈ {v0..v31} packs funct6, vm=1, vs2, vs1=10000, funct3=010, vd (1000 cases). Rustdoc KAT pins vsm4r.vs v0, v0 = 0xa6082077; vsm4r.vs v1, v2 = 0xa62820f7; vsm4r.vs v31, v30 = 0xa7e82ff7 (these words use OP_V_CRYPTO=1110111).
- Format layout holds for vd/funct3/vs1=10000/vs2/vm/funct6 (1000 cases). Opcode currently 1110111, not Volume II OP-V 1010111 (see bugs).
- Field isolation: vd/vs2/funct6 bits independent of the other fields; vs1 stays 10000 (1000 cases).
- Operand swap: swapping operands 0 and 1 swaps bits [11:7] and [24:20]; other bits preserved including vs1=10000 (1000 cases).
- Too few operands and non-vector vd/vs2 return Err (1000 cases).
- Extra operand and trailing v0.t currently disagree with the claimed RISC-V Crypto Volume II contract (see bugs).

## Environment (encode_v_crypto_vs)

- Reference: RISC-V Cryptography Extensions Volume II (Zvksed vsm4r.vs funct6=101001, opcode OP-V=1010111, funct3=010, vm=1, vs1=10000, not maskable). llvm-mc 15.0.6 does not recognize this mnemonic.
- Harness: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs, cargo test --lib encode_v_crypto_vs, proptest cases=1000.
- Dispatch: encoder/mod.rs:1026 "vsm4r.vs" => encode_v_crypto_vs(operands, 0b101001). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_v_crypto_vs NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / format / isolation / operand-swap / arity-bad-regs (passing) and opcode / extra / mask-v0.t (filed bugs). Closed: tier round spent; remaining documented gaps are the three filed bugs.

## Quirks (encode_v_crypto_vs)

- vreg_num lowercases; get_vreg does not accept Imm(0..=31) as a bare vector register number.
- vm is hardcoded to 1. Zvksed VS is not maskable; a trailing v0.t is currently ignored (bug).
- Assembly order is vd, vs2. vs1 is hardcoded 10000.
- OP_V_CRYPTO is 0b1110111 (OP-P). Volume II uses OP-V 0b1010111 (bug).
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_v_crypto_vv)

- Valid 3-operand unmasked vsm3me.vv with vd, vs2, vs1 ∈ {v0..v31} packs funct6, vm=1, vs2, vs1, funct3=010, vd (1000 cases). Rustdoc KAT pins vsm3me.vv v0, v0, v0 = 0x82002077; vsm3me.vv v1, v2, v3 = 0x8221a0f7; vsm3me.vv v31, v30, v29 = 0x83eeaff7 (these words use OP_V_CRYPTO=1110111).
- Format layout holds for vd/funct3/vs1/vs2/vm/funct6 (1000 cases). Opcode currently 1110111, not Volume II OP-V 1010111 (see bugs).
- Field isolation: vd/vs2/vs1/funct6 bits independent of the other fields (1000 cases).
- Operand swap: swapping operands 0 and 1 swaps bits [11:7] and [24:20]; swapping operands 1 and 2 swaps bits [24:20] and [19:15]; other bits preserved (1000 cases).
- Too few operands and non-vector vd/vs2/vs1 return Err (1000 cases).
- Extra operand and trailing v0.t currently disagree with the claimed RISC-V Crypto Volume II contract (see bugs).

## Environment (encode_v_crypto_vv)

- Reference: RISC-V Cryptography Extensions Volume II (Zvksh vsm3me.vv funct6=100000, opcode OP-V=1010111, funct3=010, vm=1, not maskable). llvm-mc 15.0.6 does not recognize this mnemonic.
- Harness: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs, cargo test --lib encode_v_crypto_vv, proptest cases=1000.
- Dispatch: encoder/mod.rs:1021 "vsm3me.vv" => encode_v_crypto_vv(operands, 0b100000). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_v_crypto_vv NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / format / isolation / operand-swap / arity-bad-regs (passing) and opcode / extra / mask-v0.t (filed bugs). Closed: tier round spent; remaining documented gaps are the three filed bugs.

## Quirks (encode_v_crypto_vv)

- vreg_num lowercases; get_vreg does not accept Imm(0..=31) as a bare vector register number.
- vm is hardcoded to 1. Zvksh VV is not maskable; a trailing v0.t is currently ignored (bug).
- Assembly order is vd, vs2, vs1.
- OP_V_CRYPTO is 0b1110111 (OP-P). Volume II uses OP-V 0b1010111 (bug).
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_v_crypto_vi)

- Valid 3-operand unmasked vsm3c.vi / vsm4k.vi with vd, vs2 ∈ {v0..v31} and uimm5 ∈ [0, 31] packs funct6, vm=1, vs2, uimm5, funct3=010, vd (1000 cases). Rustdoc KAT pins vsm3c.vi v0, v0, 0 = 0xae002077; vsm3c.vi v1, v2, 3 = 0xae21a0f7; vsm3c.vi v31, v30, 31 = 0xafefaff7; vsm4k.vi v1, v2, 3 = 0x8621a0f7 (these words use OP_V_CRYPTO=1110111).
- Format layout holds for vd/funct3/uimm5/vs2/vm/funct6 (1000 cases). Opcode currently 1110111, not Volume II OP-V 1010111 (see bugs).
- Field isolation: vd/vs2/uimm5/funct6 bits independent of the other fields (1000 cases).
- vd/vs2 swap: swapping operands 0 and 1 swaps bits [11:7] and [24:20] and preserves all other bits (1000 cases).
- Too few operands, non-vector vd/vs2, and non-Imm at operand 2 return Err (1000 cases).
- Extra operand, trailing v0.t, out-of-range uimm5, and OP-V opcode currently disagree with the claimed RISC-V Crypto Volume II contract (see bugs).

## Environment (encode_v_crypto_vi)

- Reference: RISC-V Cryptography Extensions Volume II (Zvksh vsm3c.vi funct6=101011, Zvksed vsm4k.vi funct6=100001, opcode OP-V=1010111, funct3=010, vm=1, not maskable). llvm-mc 15.0.6 does not recognize these mnemonics.
- Harness: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs, cargo test --lib encode_v_crypto_vi, proptest cases=1000.
- Dispatch: encoder/mod.rs:1018 "vsm3c.vi" => encode_v_crypto_vi(operands, 0b101011); encoder/mod.rs:1021 "vsm4k.vi" => encode_v_crypto_vi(operands, 0b100001). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_v_crypto_vi NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / format / isolation / vd-vs2-swap / arity-bad-regs (passing) and opcode / extra / uimm-oob / mask-v0.t (filed bugs). Closed: tier round spent; remaining documented gaps are the four filed bugs.

## Quirks (encode_v_crypto_vi)

- vreg_num lowercases; get_vreg does not accept Imm(0..=31) as a bare vector register number.
- get_imm accepts only Operand::Imm.
- vm is hardcoded to 1. Zvksh/Zvksed VI is not maskable; a trailing v0.t is currently ignored (bug).
- Assembly order is vd, vs2, uimm5. Operand 2 is a 5-bit unsigned immediate.
- OP_V_CRYPTO is 0b1110111 (OP-P). Volume II uses OP-V 0b1010111 (bug).
- The SUT truncates uimm with `& 0x1F` and does not range-check.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vid_v)

- Valid 1-operand unmasked vid.v with vd ∈ {v0..v31} matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vid.v v0 = 0x5208a057; vid.v v1 = 0x5208a0d7; vid.v v31 = 0x5208afd7. Masked KAT: llvm-mc vid.v v1, v0.t = 0x5008a0d7.
- Format layout holds: opcode=1010111, funct3=010 (OPMVV), vm=1, vs2=0, vs1=10001, vd in [11:7], funct6=010100 (1000 cases).
- Field isolation: non-vd bits independent of vd (1000 cases).
- Empty operand list and non-vector vd return Err (1000 cases).
- Extra operand and trailing v0.t currently disagree with llvm-mc (see bugs).

## Environment (encode_vid_v)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_vid_v_pbt.rs, cargo test --lib encode_vid_v, proptest cases=1000.
- Dispatch: encoder/mod.rs:1015 "vid.v" => encode_vid_v(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vid_v NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 1-op / format / isolation / arity-bad-regs / extra / mask-v0.t. Closed: tier round spent; remaining documented gaps are the extra-operand and v0.t bugs.

## Quirks (encode_vid_v)

- vreg_num lowercases; llvm-mc rejects uppercase register names.
- get_vreg does not accept Imm(0..=31) as a bare vector register number.
- vm is hardcoded to 1 (unmasked). RISC-V V 1.0 vid.v is maskable; llvm-mc accepts trailing v0.t for vd ∈ {1..31} and rejects vid.v v0, v0.t (destination overlaps mask).
- Assembly order is vd (vector). vs2 is encoded as 0. vs1/rs1 is hardcoded 10001. funct3 is 010 (OPMVV).
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vmv_v_i)

- Valid 2-operand unmasked vmv.v.i with vd ∈ {v0..v31} and simm5 ∈ [-16, 15] matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vmv.v.i v0, 0 = 0x5e003057; vmv.v.i v1, 2 = 0x5e0130d7; vmv.v.i v31, 15 = 0x5e07bfd7; vmv.v.i v1, -1 = 0x5e0fb0d7; vmv.v.i v1, -16 = 0x5e0830d7.
- Format layout holds: opcode=1010111, funct3=011, vm=1, vs2=0, vd in [11:7], simm5 in [19:15], funct6=010111 (1000 cases).
- Field isolation: vd/simm5 bits independent of the other field (1000 cases).
- simm5 two's complement: bits[19:15] = (simm as u32) & 0x1F for simm ∈ [-16, 15] (1000 cases; bounds -16/-1/0/15 forced).
- Too few operands, non-vector vd, and non-Imm at operand 1 return Err (1000 cases).
- Extra operand, trailing v0.t, and out-of-range immediates currently disagree with llvm-mc (see bugs).

## Environment (encode_vmv_v_i)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_vmv_v_i_pbt.rs, cargo test --lib encode_vmv_v_i, proptest cases=1000.
- Dispatch: encoder/mod.rs:1008 "vmv.v.i" => encode_vmv_v_i(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vmv_v_i NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / format / isolation / simm5 / arity-bad-regs / extra / mask-v0.t / imm-oob. Closed: tier round spent; remaining documented gaps are the extra-operand, v0.t, and imm-oob bugs.

## Quirks (encode_vmv_v_i)

- vreg_num lowercases; llvm-mc rejects uppercase register names.
- get_vreg does not accept Imm(0..=31) as a bare vector register number.
- get_imm accepts only Operand::Imm.
- vm is hardcoded to 1 (unmasked). RISC-V V 1.0 vmv.v.i has no masked form; llvm-mc rejects trailing v0.t.
- Assembly order is vd (vector), simm5 (signed 5-bit). vs2 is encoded as 0. funct3 is 011 (OPIVI).
- llvm-mc signed simm5 range is [-16, 15]. The SUT truncates with `& 0x1F` and does not range-check.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vmv_v_x)

- Valid 2-operand unmasked vmv.v.x with vd ∈ {v0..v31} and rs1 ∈ {x0..x31} matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vmv.v.x v0, x0 = 0x5e004057; vmv.v.x v1, x2 = 0x5e0140d7; vmv.v.x v31, x30 = 0x5e0f4fd7.
- Format layout holds: opcode=1010111, funct3=100, vm=1, vs2=0, vd in [11:7], rs1 in [19:15], funct6=010111 (1000 cases).
- Field isolation: vd/rs1 bits independent of the other field (1000 cases).
- vd/rs1 swap: swapping numeric identities of operand 0 (still a v-reg) and operand 1 (still a GPR) swaps bits [11:7] and [19:15] and preserves all other bits (1000 cases).
- ABI alias: encode_vmv_v_x([v{vd}, ABI[rs1]]) equals encode_vmv_v_x([v{vd}, x{rs1}]) for all 32 ABI names (1000 cases).
- Too few operands, non-vector vd, and non-GPR rs1 (excluding Imm(0..31), which get_reg documents as GCC bare register numbers) return Err (1000 cases).
- Extra operand and trailing v0.t currently disagree with llvm-mc (see bugs).

## Environment (encode_vmv_v_x)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_vmv_v_x_pbt.rs, cargo test --lib encode_vmv_v_x, proptest cases=1000.
- Dispatch: encoder/mod.rs:1005 "vmv.v.x" => encode_vmv_v_x(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vmv_v_x NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / format / isolation / vd-rs1-swap / ABI / arity-bad-regs / extra / mask-v0.t. Closed: tier round spent; remaining documented gaps are the extra-operand and v0.t bugs.

## Quirks (encode_vmv_v_x)

- vreg_num and reg_num lowercase; llvm-mc rejects uppercase register names.
- get_vreg does not accept Imm(0..=31) as a bare vector register number; get_reg does accept Imm(0..=31) as a GCC bare GPR number.
- vm is hardcoded to 1 (unmasked). RISC-V V 1.0 vmv.v.x has no masked form; llvm-mc rejects trailing v0.t.
- Assembly order is vd (vector), rs1 (integer). vs2 is encoded as 0. funct3 is 100 (OPIVX).
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vmv_v_v)

- Valid 2-operand unmasked vmv.v.v with vd, vs1 ∈ {v0..v31} matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vmv.v.v v0, v0 = 0x5e000057; vmv.v.v v1, v2 = 0x5e0100d7; vmv.v.v v31, v30 = 0x5e0f0fd7.
- Format layout holds: opcode=1010111, funct3=000, vm=1, vs2=0, vd in [11:7], vs1 in [19:15], funct6=010111 (1000 cases).
- Field isolation: vd/vs1 bits independent of the other field (1000 cases).
- vd/vs1 swap: swapping operands 0 and 1 swaps bits [11:7] and [19:15] and preserves all other bits (1000 cases).
- Too few operands and non-vector registers (GPR/FP/v32/non-Reg) at either position return Err (1000 cases).
- Extra operand and trailing v0.t currently disagree with llvm-mc (see bugs).

## Environment (encode_vmv_v_v)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_vmv_v_v_pbt.rs, cargo test --lib encode_vmv_v_v, proptest cases=1000.
- Dispatch: encoder/mod.rs:1002 "vmv.v.v" => encode_vmv_v_v(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vmv_v_v NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / format / isolation / vd-vs1-swap / arity-bad-regs / extra / mask-v0.t. Closed: tier round spent; remaining documented gaps are the extra-operand and v0.t bugs.

## Quirks (encode_vmv_v_v)

- vreg_num lowercases; llvm-mc rejects uppercase register names.
- get_vreg does not accept Imm(0..=31) as a bare vector register number.
- vm is hardcoded to 1 (unmasked). RISC-V V 1.0 vmv.v.v has no masked form; llvm-mc rejects trailing v0.t.
- Assembly order is vd, vs1 (two vector registers). vs2 is encoded as 0.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_v_arith_vi)

- Valid 3-operand unmasked OPIVI with vd, vs2 ∈ {v0..v31}, (mnem,funct6,imm) ∈ signed_family × [-16,15] ∪ slide_family × [0,31] matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` except vslideup.vi when vd overlaps vs2 (llvm-mc architectural overlap check; skipped). signed_family = {(vadd.vi,000000),(vand.vi,001001),(vor.vi,001010),(vxor.vi,001011)}; slide_family = {(vslideup.vi,001110),(vslidedown.vi,001111)}. KAT pins vadd.vi v0, v0, 0 = 0x02003057; vadd.vi v1, v2, 3 = 0x0221b0d7; vadd.vi v31, v30, 15 = 0x03e7bfd7; vadd.vi v1, v2, -1 = 0x022fb0d7; vadd.vi v1, v2, -16 = 0x022830d7; vand.vi v1, v2, 3 = 0x2621b0d7; vor.vi v1, v2, 3 = 0x2a21b0d7; vxor.vi v1, v2, 3 = 0x2e21b0d7; vslideup.vi v1, v2, 3 = 0x3a21b0d7; vslidedown.vi v1, v2, 3 = 0x3e21b0d7.
- Format layout holds: opcode=1010111, funct3=011, vm=1, vd in [11:7], simm5 in [19:15], vs2 in [24:20], funct6 in [31:26] (1000 cases over funct6 0..63 and simm [-16,15]).
- Field isolation: vd/vs2/simm5/funct6 bits independent of the other fields (1000 cases).
- simm5 two's complement: bits[19:15] = (simm as u32) & 0x1F for simm ∈ [-16,15] (1000 cases; bounds -16/-1/0/15 forced).
- Too few operands, non-vector vd/vs2, and non-Imm at operand 2 return Err (1000 cases).
- Extra operand, trailing v0.t, and out-of-range immediates currently disagree with llvm-mc (see bugs).

## Environment (encode_v_arith_vi)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_v_arith_vi_pbt.rs, cargo test --lib encode_v_arith_vi, proptest cases=1000.
- Dispatch: encoder/mod.rs:980-997 vadd.vi/vand.vi/vor.vi/vxor.vi/vslideup.vi/vslidedown.vi => encode_v_arith_vi(operands, funct6). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_v_arith_vi NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / format / isolation / simm5 / arity-bad-regs / extra / mask-v0.t / imm-oob. Closed: tier round spent; remaining documented gaps are the extra-operand, mask-v0.t, and imm-oob bugs.

## Quirks (encode_v_arith_vi)

- vreg_num lowercases; llvm-mc rejects uppercase register names.
- get_vreg does not accept Imm(0..=31) as a bare vector register number.
- get_imm accepts only Operand::Imm.
- vm is hardcoded to 1 (unmasked). Dispatcher TODO encoder/mod.rs:952: masked variants (v0.t) are not yet supported.
- Assembly order is vd, vs2, imm (RISC-V V). Operand 2 is a 5-bit immediate, not a register.
- llvm-mc rejects vslideup.vi when vd overlaps vs2. vslidedown.vi overlap is accepted. That is an architectural constraint, not an encoding-layout rule; the SUT still encodes those combinations.
- llvm-mc signed OPIVI range is [-16, 15]; vslideup.vi / vslidedown.vi use unsigned [0, 31]. The SUT truncates with `& 0x1F` and does not range-check.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_v_arith_vx)

- Valid 3-operand unmasked OPIVX with vd, vs2 ∈ {v0..v31}, rs1 ∈ {x0..x31}, (mnem,funct6) ∈ {(vadd.vx,000000),(vsub.vx,000010),(vand.vx,001001),(vor.vx,001010),(vxor.vx,001011),(vslideup.vx,001110),(vslidedown.vx,001111)} matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` except vslideup/vslidedown when vd overlaps vs2 (llvm-mc architectural overlap check; skipped). KAT pins vadd.vx v0, v0, x0 = 0x02004057; vadd.vx v1, v2, x3 = 0x0221c0d7; vadd.vx v31, v30, x29 = 0x03eecfd7; vsub.vx v1, v2, x3 = 0x0a21c0d7; vand.vx v1, v2, x3 = 0x2621c0d7; vor.vx v1, v2, x3 = 0x2a21c0d7; vxor.vx v1, v2, x3 = 0x2e21c0d7; vslideup.vx v1, v2, x3 = 0x3a21c0d7; vslidedown.vx v1, v2, x3 = 0x3e21c0d7.
- Format layout holds: opcode=1010111, funct3=100, vm=1, vd in [11:7], rs1 in [19:15], vs2 in [24:20], funct6 in [31:26] (1000 cases over funct6 0..63).
- Field isolation: vd/vs2/rs1/funct6 bits independent of the other fields (1000 cases).
- vs2/rs1 swap: swapping operands 1 and 2 swaps bits [24:20] and [19:15] and preserves all other bits (1000 cases).
- ABI alias: encoding rs1 as zero/ra/sp/.../t6 equals encoding rs1 as xN (1000 cases).
- Too few operands, non-vector vd/vs2, and non-GPR rs1 return Err (1000 cases).
- Extra operand and trailing v0.t currently disagree with llvm-mc (see bugs).

## Environment (encode_v_arith_vx)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_v_arith_vx_pbt.rs, cargo test --lib encode_v_arith_vx, proptest cases=1000.
- Dispatch: encoder/mod.rs:976-994 vadd.vx/vsub.vx/vand.vx/vor.vx/vxor.vx/vslideup.vx/vslidedown.vx => encode_v_arith_vx(operands, funct6). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_v_arith_vx NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / format / isolation / vs2-rs1-swap / ABI / arity-bad-regs / extra / mask-v0.t. Closed: tier round spent; remaining documented gaps are the extra-operand and mask-v0.t bugs.

## Quirks (encode_v_arith_vx)

- vreg_num lowercases; llvm-mc rejects uppercase register names.
- get_vreg does not accept Imm(0..=31) as a bare vector register number; get_reg does accept Imm(0..=31) as a GPR number for rs1.
- vm is hardcoded to 1 (unmasked). Dispatcher TODO encoder/mod.rs:952: masked variants (v0.t) are not yet supported.
- Assembly order is vd, vs2, rs1 (RISC-V V). Operand 2 is a GPR, not a vector register.
- llvm-mc rejects vslideup.vx / vslidedown.vx when vd overlaps vs2 ("destination vector register group cannot overlap the source vector register group"). That is an architectural constraint, not an encoding-layout rule; the SUT still encodes those combinations.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_v_arith_vv)

- Valid 3-operand unmasked OPIVV with vd, vs2, vs1 ∈ {v0..v31}, (mnem,funct6) ∈ {(vadd.vv,000000),(vsub.vv,000010),(vand.vv,001001),(vor.vv,001010),(vxor.vv,001011)} matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vadd.vv v0, v0, v0 = 0x02000057; vadd.vv v1, v2, v3 = 0x022180d7; vadd.vv v31, v30, v29 = 0x03ee8fd7; vsub.vv v1, v2, v3 = 0x0a2180d7; vand.vv v1, v2, v3 = 0x262180d7; vor.vv v1, v2, v3 = 0x2a2180d7; vxor.vv v1, v2, v3 = 0x2e2180d7.
- Format layout holds: opcode=1010111, funct3=000, vm=1, vd in [11:7], vs1 in [19:15], vs2 in [24:20], funct6 in [31:26] (1000 cases over funct6 0..63).
- Field isolation: vd/vs2/vs1/funct6 bits independent of the other fields (1000 cases).
- vs2/vs1 swap: swapping operands 1 and 2 swaps bits [24:20] and [19:15] and preserves all other bits (1000 cases).
- Too few operands, non-vector registers (GPR/FP/v32), and non-Reg kinds at any of the three positions return Err (1000 cases).
- Extra operand and trailing v0.t currently disagree with llvm-mc (see bugs).

## Environment (encode_v_arith_vv)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6). Default riscv64 without +v rejects vadd.vv / vsub.vv / vand.vv / vor.vv / vxor.vv.
- Harness: src/backend/riscv/assembler/encoder/encode_v_arith_vv_pbt.rs, cargo test --lib encode_v_arith_vv, proptest cases=1000.
- Dispatch: encoder/mod.rs:974-987 vadd.vv/vsub.vv/vand.vv/vor.vv/vxor.vv => encode_v_arith_vv(operands, funct6). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_v_arith_vv NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / format / isolation / vs2-vs1-swap / arity-bad-regs / extra / mask-v0.t. Closed: tier round spent; remaining documented gaps are the extra-operand and mask-v0.t bugs.

## Quirks (encode_v_arith_vv)

- vreg_num lowercases; llvm-mc rejects uppercase register names.
- get_vreg does not accept Imm(0..=31) as a bare vector register number (unlike get_reg for GPRs).
- vm is hardcoded to 1 (unmasked). Dispatcher TODO encoder/mod.rs:947: masked variants (v0.t) are not yet supported.
- Assembly order is vd, vs2, vs1 (RISC-V V), not ALU rd, rs1, rs2.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vstore)

- Valid 2-operand unit-stride vector stores with vs3 ∈ {v0..v31}, rs1 ∈ {x0..x31} ∪ ABI ∪ {fp}, (mnem,width,sumop) ∈ {(vse8.v,000,0),(vse16.v,101,0),(vse32.v,110,0),(vse64.v,111,0),(vsm.v,000,0x0B)} match llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vse8.v v0, (a0) = 0x02050027; vse16.v v1, (a1) = 0x0205d0a7; vse32.v v2, (sp) = 0x02016127; vse64.v v31, (zero) = 0x02007fa7; vsm.v v0, (a0) = 0x02b50027; vse8.v v0, (x10) = 0x02050027.
- Format layout holds: opcode=0100111, nf=000, mew=0, mop=00, vm=1, sumop in [24:20], rs1 in [19:15], width in [14:12], vs3 in [11:7] (1000 cases over width 0..7 and sumop 0..31).
- Operand::Mem { offset: 0 } and Operand::Reg for rs1 encode the same word (1000 cases).
- ABI names (zero/ra/sp/a0/…/fp) encode the same rs1 field as xN (1000 cases).
- Field isolation: vs3/rs1/width/sumop bits independent of the other fields (1000 cases).
- Too few operands, non-vector vs3, and non-GPR rs1 return Err (1000 cases).
- Non-zero Mem offset and non-Mem/non-Reg operand 1 return Err (1000 cases).
- Extra operand currently disagrees with llvm-mc (see bugs).

## Environment (encode_vstore)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6). Default riscv64 without +v rejects vse*.v / vsm.v.
- Harness: src/backend/riscv/assembler/encoder/encode_vstore_pbt.rs, cargo test --lib encode_vstore, proptest cases=1000.
- Dispatch: encoder/mod.rs:962-965 vse{8,16,32,64}.v => encode_vstore(operands, width, 0); encoder/mod.rs:969 vsm.v => encode_vstore(operands, 0b000, 0x0B). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vstore NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / format / mem-reg / ABI / isolation / arity-bad-regs / extra / nonzero-offset. Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_vstore)

- encode_vstore accepts Operand::Reg as rs1; llvm-mc requires parentheses and rejects `vse8.v v0, a0` and `vse8.v v0, 0(a0)`.
- vreg_num / reg_num lowercase; llvm-mc rejects uppercase register names.
- vm is hardcoded to 1 (unmasked). Dispatcher TODO encoder/mod.rs:947: masked variants (v0.t) are not yet supported.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vload)

- Valid 2-operand unit-stride vector loads with vd ∈ {v0..v31}, rs1 ∈ {x0..x31} ∪ ABI ∪ {fp}, (mnem,width,lumop) ∈ {(vle8.v,000,0),(vle16.v,101,0),(vle32.v,110,0),(vle64.v,111,0),(vlm.v,000,0x0B)} match llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vle8.v v0, (a0) = 0x02050007; vle16.v v1, (a1) = 0x0205d087; vle32.v v2, (sp) = 0x02016107; vle64.v v31, (zero) = 0x02007f87; vlm.v v0, (a0) = 0x02b50007; vle8.v v0, (x10) = 0x02050007.
- Format layout holds: opcode=0000111, nf=000, mew=0, mop=00, vm=1, lumop in [24:20], rs1 in [19:15], width in [14:12], vd in [11:7] (1000 cases over width 0..7 and lumop 0..31).
- Operand::Mem { offset: 0 } and Operand::Reg for rs1 encode the same word (1000 cases).
- ABI names (zero/ra/sp/a0/…/fp) encode the same rs1 field as xN (1000 cases).
- Field isolation: vd/rs1/width/lumop bits independent of the other fields (1000 cases).
- Too few operands, non-vector vd, and non-GPR rs1 return Err (1000 cases).
- Non-zero Mem offset and non-Mem/non-Reg operand 1 return Err (1000 cases).
- Extra operand currently disagrees with llvm-mc (see bugs).

## Environment (encode_vload)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6). Default riscv64 without +v rejects vle*.v / vlm.v.
- Harness: src/backend/riscv/assembler/encoder/encode_vload_pbt.rs, cargo test --lib encode_vload, proptest cases=1000.
- Dispatch: encoder/mod.rs:954-957 vle{8,16,32,64}.v => encode_vload(operands, width, 0); encoder/mod.rs:966 vlm.v => encode_vload(operands, 0b000, 0x0B). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vload NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / format / mem-reg / ABI / isolation / arity-bad-regs / extra / nonzero-offset. Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_vload)

- encode_vload accepts Operand::Reg as rs1 ("parenthesized register may be parsed differently"); llvm-mc requires parentheses and rejects `vle8.v v0, a0` and `vle8.v v0, 0(a0)`.
- vreg_num / reg_num lowercase; llvm-mc rejects uppercase register names.
- vm is hardcoded to 1 (unmasked). Dispatcher TODO encoder/mod.rs:944: masked variants (v0.t) are not yet supported.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vsetvl)

- Valid 3-GPR vsetvl with rd, rs1, rs2 ∈ {x0..x31} ∪ ABI ∪ {fp} matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vsetvl a0, a1, a2 = 0x80c5f557; vsetvl zero, ra, sp = 0x8020f057; vsetvl x0, x0, x0 = 0x80007057; vsetvl x31, x31, x31 = 0x81ffffd7; vsetvl x10, x11, x12 = 0x80c5f557; vsetvl t0, t1, t2 = 0x807372d7.
- Format layout holds: opcode=1010111, funct3=111, bits[31:25]=1000000, rd in [11:7], rs1 in [19:15], rs2 in [24:20] (1000 cases).
- ABI names (zero/ra/sp/a0/…/fp) encode the same word as xN (1000 cases).
- Field isolation: rd/rs1/rs2 bits independent of the other fields (1000 cases).
- Too few operands and FP/vector registers as rd/rs1/rs2 return Err (1000 cases).
- Non-register operand kinds (Imm outside 0..=31, Symbol, Label, Mem, Csr, FenceArg, RoundingMode, SymbolOffset) at any of the three positions return Err (1000 cases).
- Extra operand currently disagrees with llvm-mc (see bugs).

## Environment (encode_vsetvl)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6). Default riscv64 without +v rejects vsetvl.
- Harness: src/backend/riscv/assembler/encoder/encode_vsetvl_pbt.rs, cargo test --lib encode_vsetvl_, proptest cases=1000.
- Dispatch: encoder/mod.rs:949 "vsetvl" => encode_vsetvl(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vsetvl NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 3-op / format / ABI / isolation / arity-FP / extra / nonreg. Closed: tier round spent; remaining documented gap is the extra-operand bug.

## Quirks (encode_vsetvl)

- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rd/rs1/rs2.
- llvm-mc rejects uppercase register names; reg_num lowercases.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vsetivli)

- Valid 6-operand named vsetivli with rd ∈ GPR, uimm ∈ 0..=31, SEW ∈ {e8,e16,e32,e64}, LMUL ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta/tu, ma/mu matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vsetivli a0, 1, e32, m1, ta, ma = 0xcd00f557; vsetivli a0, 0, e8, m8, tu, mu = 0xc0307557; vsetivli a0, 31, e64, mf2, ta, ma = 0xcdfff557; vsetivli x10, 5, e16, mf4, tu, ma = 0xc8e2f557; vsetivli zero, 0, e8, m1, tu, mu = 0xc0007057.
- Valid raw 10-bit vtypei immediate 0..=1023 matches llvm-mc (1000 cases). KAT pins vsetivli a0, 1, 0 = 0xc000f557.
- Format layout holds: opcode=1010111, funct3=111, bits[31:30]=11, rd in [11:7], uimm in [19:15], vtypei in [29:20] packed [ma][ta][sew][lmul] (1000 cases).
- ABI names (zero/ra/sp/a0/…/fp) encode the same word as xN (1000 cases).
- Field isolation: rd/uimm/vtypei bits independent of the other fields (1000 cases).
- FP and vector registers as rd return Err (1000 cases).
- Extra operand, two-operand (missing vtypei), SEW e128/e256/e512/e1024, AVL outside 0..=31, and raw vtypei 1024..=2047 currently disagree with llvm-mc (see bugs).

## Environment (encode_vsetivli)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6). Default riscv64 without +v rejects vsetivli.
- Harness: src/backend/riscv/assembler/encoder/encode_vsetivli_pbt.rs, cargo test --lib encode_vsetivli, proptest cases=1000.
- Dispatch: encoder/mod.rs:945 "vsetivli" => encode_vsetivli(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vsetivli NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of named/imm/format/ABI/isolation/arity/extra/wide-SEW/FP/AVL-OOB/vtypei-OOB. Closed: tier round spent; remaining documented gaps are the five filed bugs.

## Quirks (encode_vsetivli)

- llvm-mc requires all four named vtype fields in order e, m, ta|tu, ma|mu. It also accepts a raw immediate 0..=1023 (10 bits, not 11 as for vsetvli) and prints the decoded named form.
- llvm-mc rejects uppercase field names, AVL outside 0..=31, and out-of-range immediates (1024, 2047).
- get_imm does not accept a register as AVL; llvm-mc also requires an integer AVL.
- parse_vtypei lowercases field names; llvm-mc is case-sensitive.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_vsetvli)

- Valid 6-operand named vsetvli with SEW ∈ {e8,e16,e32,e64}, LMUL ∈ {m1,m2,m4,m8,mf2,mf4,mf8}, ta/tu, ma/mu matches llvm-mc `-triple=riscv64 -mattr=+v -show-encoding` (1000 cases). KAT pins vsetvli a0, a1, e32, m1, ta, ma = 0x0d05f557; vsetvli zero, ra, e8, m8, tu, mu = 0x0030f057; vsetvli a0, a1, e64, mf2, ta, ma = 0x0df5f557; vsetvli a0, a1, e16, mf4, tu, ma = 0x08e5f557; vsetvli x10, x11, e32, m1, ta, ma = 0x0d05f557.
- Valid raw 11-bit vtypei immediate 0..=2047 matches llvm-mc (1000 cases). KAT pins vsetvli a0, a1, 0 = 0x0005f557.
- Format layout holds: opcode=1010111, funct3=111, bit31=0, rd in [11:7], rs1 in [19:15], vtypei in [30:20] packed [ma][ta][sew][lmul] (1000 cases).
- ABI names (zero/ra/sp/a0/…/fp) encode the same word as xN (1000 cases).
- Field isolation: rd/rs1/vtypei bits independent of the other fields (1000 cases).
- FP and vector registers as rd/rs1 return Err (1000 cases).
- Extra operand, two-operand (missing vtypei), and SEW e128/e256/e512/e1024 currently disagree with llvm-mc (see bugs).

## Environment (encode_vsetvli)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+v -show-encoding (LLVM 15.0.6). Default riscv64 without +v rejects vsetvli.
- Harness: src/backend/riscv/assembler/encoder/encode_vsetvli_pbt.rs, cargo test --lib encode_vsetvli, proptest cases=1000.
- Dispatch: encoder/mod.rs:941 "vsetvli" => encode_vsetvli(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_vsetvli NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of named/imm/format/ABI/isolation/arity/extra/wide-SEW/FP. Closed: tier round spent; remaining documented gaps are the three filed bugs.

## Quirks (encode_vsetvli)

- llvm-mc requires all four named vtype fields in order e, m, ta|tu, ma|mu. It also accepts a raw immediate 0..=2047 and prints the decoded named form.
- llvm-mc rejects uppercase field names and out-of-range immediates (2048, -1).
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rd.
- parse_vtypei lowercases field names; llvm-mc is case-sensitive.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_c_jalr)

- Valid 1-operand C.JALR with rs1 ∈ {x1..x31} matches llvm-mc `-triple=riscv64 -mattr=+c -show-encoding` (1000 cases). KAT pins c.jalr ra = 0x9082, c.jalr x1 = 0x9082, c.jalr sp = 0x9102, c.jalr a0 = 0x9502, c.jalr x31 = 0x9f82, c.jalr t0 = 0x9282.
- CR-type layout holds: op=10, funct4=1001, rs1 in bits[11:7], rs2 bits[6:2]=0 (1000 cases).
- ABI names (ra/sp/a0/t6/fp/s0/…) encode the same halfword as xN (1000 cases).
- Field isolation: op/funct4/rs2 bits independent of rs1; rs1 field equals rs1 (1000 cases).
- Empty operand list and FP src return Err (1000 cases).
- Extra operand and rs1=x0 currently disagree with llvm-mc (see bugs): extra ignored; rs1=x0 encodes C.EBREAK halfword 0x9002.

## Environment (encode_c_jalr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+c -show-encoding (LLVM 15.0.6). Default riscv64 without +c rejects C.JALR.
- Harness: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs, cargo test --lib encode_c_jalr, proptest cases=1000.
- Dispatch: encoder/mod.rs:933 "c.jalr" => encode_c_jalr(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_c_jalr NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 1-op / CR-type / ABI / isolation / arity-FP / extra / rs1=x0. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_c_jalr)

- llvm-mc prints `c.jalr ra` / `c.jalr x1` as `jalr ra` with a 16-bit encoding ([0x82,0x90]). Other rs1 print as `jalr rs1`.
- C.JALR allows rs1=x2 (unlike C.LUI).
- llvm-mc rejects `c.jalr x0` / `c.jalr zero` because rs1=x0 is C.EBREAK (0x9002), not C.JALR. SUT currently emits 0x9002 (bug).
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rs1.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_c_jr)

- Valid 1-operand C.JR with rs1 ∈ {x1..x31} matches llvm-mc `-triple=riscv64 -mattr=+c -show-encoding` (1000 cases). KAT pins c.jr ra = 0x8082, c.jr x1 = 0x8082, c.jr sp = 0x8102, c.jr a0 = 0x8502, c.jr x31 = 0x8f82, c.jr t0 = 0x8282.
- CR-type layout holds: op=10, funct4=1000, rs1 in bits[11:7], rs2 bits[6:2]=0 (1000 cases).
- ABI names (ra/sp/a0/t6/fp/s0/…) encode the same halfword as xN (1000 cases).
- Field isolation: op/funct4/rs2 bits independent of rs1; rs1 field equals rs1 (1000 cases).
- Empty operand list and FP src return Err (1000 cases).
- Extra operand and rs1=x0 currently disagree with llvm-mc (see bugs): extra ignored; rs1=x0 encodes reserved halfword 0x8002.

## Environment (encode_c_jr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+c -show-encoding (LLVM 15.0.6). Default riscv64 without +c rejects C.JR.
- Harness: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs, cargo test --lib encode_c_jr, proptest cases=1000.
- Dispatch: encoder/mod.rs:930 "c.jr" => encode_c_jr(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_c_jr NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 1-op / CR-type / ABI / isolation / arity-FP / extra / rs1=x0. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_c_jr)

- llvm-mc prints `c.jr ra` / `c.jr x1` as `ret` with a 16-bit encoding ([0x82,0x80]). Other rs1 print as `jr rs1`.
- C.JR allows rs1=x2 (unlike C.LUI).
- llvm-mc rejects `c.jr x0` / `c.jr zero` because rs1=x0 is reserved. SUT currently emits 0x8002 (bug).
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rs1.
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_c_add)

- Valid 2-operand C.ADD with rd ∈ {x0..x31} and rs2 ∈ {x1..x31} matches llvm-mc `-triple=riscv64 -mattr=+c -show-encoding` (1000 cases). KAT pins c.add t1, t0 = 0x9316, c.add x1, x2 = 0x908a, c.add a0, a1 = 0x952e, c.add x31, x31 = 0x9ffe, c.add x1, x1 = 0x9086, c.add sp, ra = 0x9106.
- CR-type layout holds: op=10, funct4=1001, rd in bits[11:7], rs2 in bits[6:2] (1000 cases).
- ABI names (zero/ra/sp/a0/t6/fp/s0/…) encode the same halfword as xN (1000 cases).
- Field isolation: rd bits[11:7] independent of rs2; rs2/op/funct4 bits independent of rd (1000 cases).
- Empty/1-operand and FP dest/src return Err (1000 cases).
- Extra operand and rs2=x0 currently disagree with llvm-mc (see bugs): extra ignored; rs2=x0 encodes as C.JALR (rd≠0) or C.EBREAK (rd=0).

## Environment (encode_c_add)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -mattr=+c -show-encoding (LLVM 15.0.6). Default riscv64 without +c rejects C.ADD.
- Harness: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs, cargo test --lib encode_c_add_pbt, proptest cases=1000.
- Dispatch: encoder/mod.rs:929 "c.add" => encode_c_add(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and reported encode_c_add NOT LINKED; Rust cargo tests are not those binaries). Sweep was a manual audit of 2-op / CR-type / ABI / isolation / arity-FP / extra / rs2=x0. Closed: tier round spent; remaining documented gaps are the two filed bugs.

## Quirks (encode_c_add)

- llvm-mc prints `c.add x1, x2` as `add ra, ra, sp` with a 16-bit encoding ([0x8a,0x90]).
- llvm-mc encodes `c.add x0, x1` as a HINT; SUT also encodes it (no rd=x0 rejection, unlike C.LUI). ISA: C.ADD with rd=x0 and rs2≠x0 is HINT.
- C.ADD allows rd=x2 (unlike C.LUI).
- llvm-mc rejects `c.add x1, x0` because rs2=x0 is the C.JALR encoding, and `c.add x0, x0` because that encoding is C.EBREAK. SUT currently emits those halfwords (bugs).
- get_reg accepts Imm(0..=31) as a GCC bare GPR number. llvm-mc rejects numeric rd.
- proptest 1.11 requires `#[test]` inside `proptest!`.

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

# Confirmed invariants (encode_beqz)

- Valid `beqz rs, Imm(0)` word matches llvm-mc `-triple=riscv64 -show-encoding` for ABI and xN names (1000 cases). KAT pins beqz a0,0 = 0x00050063, beqz zero,0 = 0x00000063, beqz t6,0 = 0x000f8063, beqz fp,0 = 0x00040063.
- Documented expansion: word/reloc of encode_beqz([rs, Symbol(tgt)]) equals encode_branch_instr([rs, x0, Symbol(tgt)], funct3=000) and equals with rs2=zero (1000 cases). llvm-mc `beq rs, x0, 0` matches SUT word (1000 cases).
- B-type layout holds: opcode=OP_BRANCH (0b1100011), funct3=000 (BEQ), rs1 as given, rs2=x0, imm=0; reloc is always RelocType::Branch with addend 0 (1000 cases).
- ABI names and xN encode identically; fp aliases s0/x8 (1000 cases).
- Symbol / Label / Reg-as-label targets with the same string yield identical WordWithReloc (1000 cases). Imm targets stringify into reloc.symbol while the word stays zero-imm BEQ (1000 cases).
- Empty / single-operand arity, invalid rs (FP/vector/unknown/non-Reg), and invalid targets (Mem/Csr/Fence/RM/SymbolOffset/MemSymbol) return Err (1000 cases).
- A third operand currently disagrees with llvm-mc (see bug encode_beqz_extra_operand): extras are silently ignored.

## Environment (encode_beqz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15).
- Harness: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs, cargo test --lib encode_beqz, proptest cases=1000.
- Dispatch: encoder/mod.rs "beqz" => encode_beqz(operands). Operands passed through.
- coverage_gaps had no LLVM profraw (C++ reporter; encode_beqz NOT LINKED there). Sweep was a manual audit of llvm-mc / beq expansion / B-type / ABI / target forms / Imm / arity / invalid / extra. Closed: tier round spent; remaining documented gap is the filed extra-operand bug.

## Quirks (encode_beqz)

- Unlike encode_branch_instr, encode_beqz always returns WordWithReloc even for Imm targets (get_branch_target stringifies Imm); the B-type immediate field is always 0 and the reloc carries the target string.
- get_branch_target intentionally accepts Reg as a label name (e.g. beqz a0, t1 where t1 is a label).
- proptest 1.11 requires `#[test]` inside `proptest!`.

# Confirmed invariants (encode_bnez)

- Documented expansion: word/reloc of encode_bnez([rs, Symbol(tgt)]) equals encode_branch_instr([rs, x0, Symbol(tgt)], funct3=0b001) and llvm-mc `bnez rs, 0` / `bne rs, x0, 0`.
- B-type layout: opcode OP_BRANCH, funct3=001 (BNE), rs2=x0, imm field always 0 (reloc carries target), RelocType::Branch, addend=0.
- ABI / xN / fp (x8) aliases and Symbol/Label/Reg-as-label / Imm targets are equivalent for the reloc symbol string.
- get_reg Imm(0..31) bare-number path equals Reg(xN).
- Arity < 2 and invalid rs/target return Err.
- A third operand currently disagrees with llvm-mc (see bug encode_bnez_extra_operand.md) — same class as encode_beqz.

## Environment (encode_bnez)

- llvm-mc: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding
- Harness: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs, cargo test --lib encode_bnez -- --test-threads=1
- Dispatch: encoder/mod.rs "bnez" => encode_bnez(operands). Operands passed through without arity check.
- coverage_gaps had no LLVM profraw (C++ reporter; encode_bnez NOT LINKED there); cargo execution is the evidence.

## Quirks (encode_bnez)

- Unlike encode_branch_instr, encode_bnez always returns WordWithReloc even for Imm targets (imm never folded into the B-type immediate field).
