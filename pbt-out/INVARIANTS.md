# Confirmed invariants (encode_branch_instr)

- Valid `mn rs1, rs2, off` for mn in {beq,bne,blt,bge,bltu,bgeu}, even off in [-4096, 4094], rs1/rs2 in x0..x31 / ABI names matches llvm-mc `-triple=riscv64 -show-encoding` (1000 cases). KAT pins beq/bne/blt/bge/bltu/bgeu x0,x1,0 = 0x00100063 / 0x00101063 / 0x00104063 / 0x00105063 / 0x00106063 / 0x00107063; beq x1,x2,4=0x00208263; bne x1,x2,-4=0xfe209ee3; beq x1,x2,4094=0x7e208fe3; beq x1,x2,-4096=0x80208063.
- B-type layout holds: opcode=0b1100011, funct3 in bits[14:12], rs1 in bits[19:15], rs2 in bits[24:20], reconstructed even offset matches (1000 cases).
- ABI names, xN, and fp=s0/x8 encode the same rs1/rs2 (1000 cases).
- Symbol/Label/Reg reloc-form word equals `mn rs1, rs2, 0` with RelocType::Branch, symbol=s, addend=0 (1000 cases).
- Empty operand list, missing 3rd operand, FP rs1/rs2, and Mem/Csr/Fence/RoundingMode as 3rd operand return Err (1000 cases).
- Odd/out-of-range immediates, extra operands, and SymbolOffset currently disagree with llvm-mc (see bugs).

## Environment (encode_branch_instr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs, cargo test --lib encode_branch_instr, proptest cases=1000.
- Dispatch: encoder/mod.rs:468-473 beq/bne/blt/bge/bltu/bgeu => encode_branch_instr.
- Requested `--func encode_branch` is absent from base.rs; the in-scope symbol is encode_branch_instr.
- Sibling encode_b / beqz / bnez / bgez / bltz / bgt / C.BEQZ are not same-job independent differentials.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_branch_instr_neg_bad_3rd. Closed: tier round spent; remaining documented gaps are the three filed bugs.
- Three failing properties are SUT bugs. See pbt-out/bug_reports/encode_branch_instr_*.md.

## Quirks (encode_branch_instr)

- ASCII case of register names is accepted (reg_num to_lowercase); llvm-mc RISC-V is case-sensitive.
- get_reg accepts Imm 0..31 as bare register numbers (encoder/mod.rs:356 GCC inline asm).
- No operands.len() == 3 check; extra operands are ignored (see bugs).
- Immediate is `*imm as i32` then encode_b drops bit 0; values outside even [-4096, 4094] wrap/truncate (see bugs).
- SymbolOffset is not matched and returns "branch: expected offset or label as 3rd operand" (see bugs).
- 3rd-operand Reg is treated as a jump-target symbol (matches parser classifying a label that looks like a register).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 21-line body.

# Confirmed invariants (encode_jalr)

- Valid `jalr rd, rs1, off` for off in [-2048, 2047] and rd/rs1 in x0..x31 / ABI names matches llvm-mc `-triple=riscv64 -show-encoding` (1000 cases). KAT pins jalr x0,x1,0=0x00008067, jalr x1,x2,0=0x000100e7, jalr x1,x2,8=0x008100e7, jalr x1,x2,-8=0xff8100e7, jalr x1,x2,2047=0x7ff100e7, jalr x1,x2,-2048=0x800100e7, jalr x1,x2,1=0x001100e7 (odd imm accepted).
- 1-operand `jalr rs1` equals `jalr ra, rs1, 0` and llvm-mc `jalr rs1` (1000 cases). KAT pins jalr x1=0x000080e7.
- 2-operand `jalr rd, rs1` equals `jalr rd, rs1, 0` (1000 cases).
- 2-operand `jalr rd, off(rs1)` equals `jalr rd, rs1, off` and llvm-mc (1000 cases).
- I-type layout holds: opcode=0b1100111, funct3=0, rd in bits[11:7], rs1 in bits[19:15], reconstructed signed imm12 matches (1000 cases).
- ABI names, xN, and fp=s0/x8 encode the same rd/rs1 (1000 cases).
- Empty operand list, extra operand, and FP dest/src return Err (1000 cases).
- Out-of-range immediates, 1-operand Mem, and MemSymbol %pcrel_lo/%lo currently disagree with llvm-mc (see bugs).

## Environment (encode_jalr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs, cargo test --lib encode_jalr, proptest cases=1000.
- Dispatch: encoder/mod.rs:463 `"jalr" => encode_jalr(operands)`.
- Sibling encode_i / encode_c_jalr / jr / ret / encode_jal are not same-job independent differentials.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_jalr_one_operand_mem / encode_jalr_reloc_lo. Closed: tier round spent; remaining documented gaps are the three filed bugs.
- Three failing properties are SUT bugs. See pbt-out/bug_reports/encode_jalr_*.md.

## Quirks (encode_jalr)

- ASCII case of register names is accepted (reg_num to_lowercase); llvm-mc RISC-V is case-sensitive.
- get_reg accepts Imm 0..31 as bare register numbers (encoder/mod.rs:351 GCC inline asm).
- operands.len() is matched exactly (1/2/3); extra operands return Err (unlike encode_jal).
- Immediate is `imm as i32` then encode_i masks with 0xFFF; values outside [-2048, 2047] wrap (see bugs).
- 1-operand arm only accepts Reg via get_reg; Mem `jalr off(rs1)` is rejected (see bugs).
- 2-operand arm matches Reg and Mem only; MemSymbol %pcrel_lo/%lo is "jalr: invalid operands" (see bugs).
- Odd immediates are valid for JALR (LSB of the computed target is cleared at execution).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 32-line body.

# Confirmed invariants (encode_jal)

- Valid `jal rd, off` for even off in [-1048576, 1048574] and rd in x0..x31 / ABI names matches llvm-mc `-triple=riscv64 -show-encoding` (1000 cases). KAT pins jal x0,0=0x0000006f, jal x1,0=0x000000ef, jal x1,4=0x004000ef, jal x1,-4=0xffdff0ef, jal x1,1048574=0x7ffff0ef, jal x1,-1048576=0x800000ef.
- 1-operand `jal off` equals `jal ra, off` and llvm-mc `jal off` (1000 cases). KAT pins jal 4=0x004000ef.
- J-type layout holds: opcode=0b1101111, rd in bits[11:7], reconstructed even offset matches (1000 cases).
- ABI names, xN, and fp=s0/x8 encode the same rd (1000 cases).
- Symbol/Label reloc-form word equals `jal rd, 0` with RelocType::Jal, symbol=s, addend=0 (1000 cases, 1-op and 2-op).
- Empty operand list and FP dest return Err (1000 cases).
- Odd/out-of-range immediates, extra operands, and SymbolOffset currently disagree with llvm-mc (see bugs).

## Environment (encode_jal)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs, cargo test --lib encode_jal, proptest cases=1000.
- Dispatch: encoder/mod.rs:460 `"jal" => encode_jal(operands)`.
- Sibling encode_j / encode_j_pseudo / encode_jalr / C.J are not same-job independent differentials.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_jal_one_operand_reloc / encode_jal_neg_fp / encode_jal_neg_empty. Closed: tier round spent; remaining documented gaps are the three filed bugs.
- Three failing properties are SUT bugs. See pbt-out/bug_reports/encode_jal_*.md.

## Quirks (encode_jal)

- ASCII case of register names is accepted (reg_num to_lowercase); llvm-mc RISC-V is case-sensitive.
- get_reg accepts Imm 0..31 as bare register numbers (encoder/mod.rs:351 GCC inline asm).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- Immediate is `*imm as i32` then encode_j drops bit 0; values outside even [-1048576, 1048574] wrap/truncate (see bugs).
- SymbolOffset is not matched and returns "jal: invalid operand" (see bugs).
- 1-operand Reg is treated as a jump-target symbol (matches llvm-mc `jal ra` / `jal x1`).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 40-line body.

# Confirmed invariants (encode_auipc)

- Valid `auipc rd, imm` for imm in [0, 1048575] and rd in x0..x31 / ABI names matches llvm-mc `-triple=riscv64 -show-encoding` (1000 cases). KAT pins auipc x0,0=0x00000017, auipc x1,1=0x00001097, auipc x1,1048575=0xFFFFF097.
- U-type layout holds: opcode=0b0010111, rd in bits[11:7], imm20 in bits[31:12] (1000 cases).
- ABI names, xN, and fp=s0/x8 encode the same rd (1000 cases).
- `%pcrel_hi(sym)` reloc-form word equals `auipc rd, 0` with RelocType::PcrelHi20, symbol=sym, addend=0 (1000 cases).
- `%got_pcrel_hi` / `%tls_ie_pcrel_hi` / `%tls_gd_pcrel_hi` same word with GotHi20 / TlsGotHi20 / TlsGdHi20 (1000 cases). KAT pins auipc t0, %got_pcrel_hi(x) word=0x00000297.
- Arity < 2, FP dest, and Label/Mem/Csr/Fence/SymbolOffset/MemSymbol as operand 1 return Err (1000 cases).
- Out-of-range immediates, extra operands, plain/%hi/%lo/%pcrel_lo/%tprel_* symbols, and `%pcrel_hi(sym+N)` addend currently disagree with llvm-mc (see bugs).

## Environment (encode_auipc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs, cargo test --lib encode_auipc, proptest cases=1000.
- Dispatch: encoder/mod.rs:455 `"auipc" => encode_auipc(operands)`.
- Sibling encode_lui / encode_c_lui / encode_u are not same-job independent differentials.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_auipc_neg_arity / encode_auipc_neg_fp / encode_auipc_neg_bad_operand / encode_auipc_pcrel_hi_addend. Closed: tier round spent; remaining documented gaps are the four filed bugs.
- Four failing properties are SUT bugs. See pbt-out/bug_reports/encode_auipc_*.md.

## Quirks (encode_auipc)

- ASCII case of register names is accepted (reg_num to_lowercase); llvm-mc RISC-V is case-sensitive.
- get_reg accepts Imm 0..31 as bare register numbers (encoder/mod.rs:349 GCC inline asm).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- Immediate is `(*imm as u32) << 12` then masked; values outside [0, 1048575] wrap (see bugs).
- Symbol arm forwards every Symbol through parse_reloc_modifier, including plain names as PcrelHi20 (see bugs).
- extract_modifier_symbol keeps `foo+4` as the symbol and addend is hardcoded 0 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 19-line body.

# Confirmed invariants (encode_lui)

- Valid `lui rd, imm` for imm in [0, 1048575] and rd in x0..x31 / ABI names matches llvm-mc `-triple=riscv64 -show-encoding` (1000 cases). KAT pins lui x0,0=0x00000037, lui x1,1=0x000010B7, lui x1,1048575=0xFFFFF0B7.
- U-type layout holds: opcode=0b0110111, rd in bits[11:7], imm20 in bits[31:12] (1000 cases).
- ABI names, xN, and fp=s0/x8 encode the same rd (1000 cases).
- `%hi(sym)` reloc-form word equals `lui rd, 0` with RelocType::Hi20, symbol=sym, addend=0 (1000 cases).
- `%tprel_hi(sym)` same word with RelocType::TprelHi20 (1000 cases). KAT pins lui t0, %tprel_hi(x) word=0x000002B7.
- Arity < 2, FP dest, and Label/Mem/Csr/Fence/SymbolOffset/MemSymbol as operand 1 return Err (1000 cases).
- Out-of-range immediates, extra operands, plain/%pcrel_hi/%lo symbols, and `%hi(sym+N)` addend currently disagree with llvm-mc (see bugs).

## Environment (encode_lui)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/riscv/assembler/encoder/encode_lui_pbt.rs, cargo test --lib encode_lui, proptest cases=1000.
- Dispatch: encoder/mod.rs:452 `"lui" => encode_lui(operands)`.
- Sibling encode_auipc / encode_c_lui / encode_u are not same-job independent differentials.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_lui_neg_arity / encode_lui_neg_fp / encode_lui_neg_bad_operand / encode_lui_hi_addend. Closed: tier round spent; remaining documented gaps are the four filed bugs.
- Four failing properties are SUT bugs. See pbt-out/bug_reports/encode_lui_*.md.

## Quirks (encode_lui)

- ASCII case of register names is accepted (reg_num to_lowercase); llvm-mc RISC-V is case-sensitive.
- get_reg accepts Imm 0..31 as bare register numbers (encoder/mod.rs:348 GCC inline asm).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- Immediate is `(*imm as u32) << 12` then masked; values outside [0, 1048575] wrap (see bugs).
- Symbol arm special-cases only `%tprel_hi(`; every other symbol is Hi20 (see bugs).
- extract_modifier_symbol keeps `foo+4` as the symbol and addend is hardcoded 0 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 22-line body.

# Confirmed invariants (encode_ldr_str_auto)

- Valid unsigned LDR/STR Wt/Xt, [Xn|SP, #pimm] with auto-detected size matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins ldr x0,[x1]=0xF9400020, ldr w0,[x1]=0xB9400020, str x0,[x1]=0xF9000020.
- Valid unsigned LDR/STR St/Dt/Qt matches llvm-mc including Q opc=11/10 and shift=4 (1000 cases). KAT pins ldr s0,[x1]=0xBD400020, ldr d0,[x1]=0xFD400020, ldr q0,[x1]=0x3DC00020.
- Load XOR store = bit 22 for W/X/S/D/Q/B/H at offset 0; W XOR X at equal Rt/Rn = bit 30 (1000 cases).
- Empty / non-Reg first operand / arity-1 Reg returns Err (1000 cases).
- Aliases lr, xzr, wzr, XZR, WZR, X0, W0, x31, w31 match llvm-mc (1000 cases).
- LDR literal Symbol is WordWithReloc Ldr19 addend 0 matching llvm-mc `ldr Rt, #0`; STR literal returns Err (1000 cases).
- Bt/Ht, bare Vn, GNU fp, and SP dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_ldr_str_auto)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). GNU as 2.38 agrees on Wt/Xt/Bt/Ht/St/Dt/Qt and rejects bare Vn / SP dest / STR-literal.
- Harness: src/backend/arm/assembler/encoder/encode_ldr_str_auto_pbt.rs, cargo test --lib encode_ldr_str_auto, proptest cases=1000.
- Dispatch: encoder/mod.rs:484-486 `"ldr" => encode_ldr_str_auto(operands, true)`, `"str" => encode_ldr_str_auto(operands, false)`.
- Sibling encode_ldr_str is not a same-job independent differential (callee, explicit size).
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_ldr_str_auto_neg_v_reg / encode_ldr_str_auto_diff_fp_alias. Closed: tier round spent; remaining documented gaps are the three filed bugs.
- Five failing properties are 4 SUT bugs (B/H size, bare V, fp alias, SP dest). See pbt-out/bug_reports/encode_ldr_str_auto_*.md.

## Quirks (encode_ldr_str_auto)

- ASCII case of register names is accepted (to_lowercase).
- parse_reg_num maps xzr/wzr/x31/w31 to 31 (ZR). SP as Rt is a separate failing property (see bugs).
- Default unknown prefix is 64-bit; V and B/H currently hit that path (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 25-line body.

# Confirmed invariants (encode_cond_branch)

- Reloc-form B.cond word (imm19=0) matches llvm-mc `b.{cond} #0` for all 18 condition names including cs/hs, cc/lo, al, nv, and ASCII case (1000 cases). KAT pins b.eq foo word=0x54000000 CondBr19 ELF 280; llvm-mc b.eq #0=0x54000000, b.ne #0=0x54000001, b.nv #0=0x5400000f, b.al #0=0x5400000e, b.hs/cs #0=0x54000002, b.lo/cc #0=0x54000003.
- ARM B.cond layout holds: bits[31:24]=01010100, bit 4=0, imm19=0 in reloc form, cond in bits[3:0] (1000 cases).
- Symbol/Label addend 0; SymbolOffset preserves addend; reloc type CondBr19 (1000 cases).
- cs==hs, cc==lo; invertible cond pairs XOR 1; cond change only touches bits[3:0] (1000 cases).
- Empty operand list and unknown condition names return Err (1000 cases).
- Mem/Shift/Extend/RegArrangement/Expr return Err (sweep, 1000 cases).
- Parser-misclassified Reg/Cond/Barrier names are treated as symbols with CondBr19 (sweep, 1000 cases; gas accepts `b.eq x0` as a label).
- Immediate PC-offset form, extra operands, and :lo12: modifiers currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_cond_branch)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). GNU as 2.38 agrees on `b.{cond} #imm` (aligned ±1 MiB), `b.{cond} label`, all 16 conditions, and rejects extra/empty/unknown/unaligned/modifier.
- ARM ARM Conditional branch (immediate): 01010100 imm19 0 cond. Offset/4 signed 19-bit.
- Dispatch: encoder/mod.rs:351-353 `b.{cond}` => encode_cond_branch; encoder/mod.rs:356-370 GNU aliases beq/bne/.../bal.
- Sibling encode_branch / encode_cbz / encode_tbz are not same-job differentials.
- Harness: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs, cargo test --lib encode_cond_branch, proptest cases=1000.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_cond_branch_neg_bad_operand / encode_cond_branch_symbol_misclassified / encode_cond_branch_neg_modifier. Closed: every documented behavior has a property; remaining gaps are the three filed bugs.
- Three failing properties are SUT bugs. See pbt-out/bug_reports/encode_cond_branch_*.md.

## Quirks (encode_cond_branch)

- ASCII case of condition names is accepted (encode_cond to_lowercase).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- Operand 0 is get_symbol only; Imm PC offsets are rejected (see bugs).
- get_symbol accepts Modifier/ModifierOffset as the inner symbol (see bugs).
- get_symbol accepts parser-misclassified Reg/Cond/Barrier as symbols (gas-compatible for `b.eq x0`).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 14-line body.

# Confirmed invariants (encode_mov)

- Valid integer register MOV (x/w 0..30, xzr/wzr, sp — not wsp) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases; WSP is a bug). KAT pins mov x0,x1=0xaa0103e0, mov sp,x1=0x9100003f.
- Width-appropriate MOV immediates materialize the same value as llvm-mc, allowing MOVZ/MOVN/ORR-bitmask aliases and README.md:287 movz+movk Words (1000 cases). KAT pins mov x0,#0=0xd2800000.
- NEON MOV 8b/16b, INS-from-GPR, UMOV .s/.d, and element INS match llvm-mc (1000 cases). KAT pins 16b=0x4ea11c20, INS d[1]=0x4e181c20, UMOV x0 v0.d[1]=0x4e183c00, elem s[3]<-s[0]=0x6e1c0420.
- X vs W register MOV at equal rd/rm in 0..30 differ only in sf bit 31 (1000 cases).
- Arity 0 and 1 return Err (1000 cases).
- lr / uppercase Xn aliases match llvm-mc (sweep, 1000 cases).
- WSP, extra operands, mixed X/W, FP scalar, mov sp #imm, lane OOB, 16b-vs-8b, 4s vector, and 64-bit imm on W currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_mov)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). GNU as 2.38 agrees on GPR/NEON 8b-16b/INS/UMOV and rejects extra/mixed/FP/sp-imm/4s/64-bit-on-W.
- Harness: src/backend/arm/assembler/encoder/encode_mov_pbt.rs, cargo test --lib encode_mov_, proptest cases=1000.
- Dispatch: encoder/mod.rs:373 `"mov" => encode_mov(operands)`.
- Sibling encode_movz/movk/movn and encode_neon_ins/umov are not same-job independent differentials.
- README.md:287 asserts movz+movk expansion for multi-instruction immediates (gas/llvm-mc reject those as a single `mov`) — value reconstruct, not encoding match.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_mov_diff_alt_spellings (passing). Closed: tier round spent; remaining documented gaps are the failing WSP/extra/mixed/FP/sp-imm/lane/arr/4s/W-imm64 paths already filed as bugs.
- Nine failing properties are SUT bugs. See pbt-out/bug_reports/encode_mov_*.md.

## Quirks (encode_mov)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_mov distinguishes only the exact name `sp`, not `wsp` (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- MOVZ/MOVN vs ORR-bitmask alias choice for the same immediate follows README.md:287 search order (unshifted MOVZ, unshifted MOVN, then bitmask) rather than llvm-mc preferred shifted MOVZ — user-visible value agrees.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_mov match arms.

# Confirmed invariants (encode_adrp)

- Reloc-form ADRP word (immhi=immlo=0) matches llvm-mc `adrp Xd, #0` for Rd in 0..31 including xzr (1000 cases). KAT pins adrp x0, foo word=0x90000000 AdrpPage21; adrp xzr, foo word=0x9000001f; adrp x0, :got:foo word=0x90000000 AdrGotPage21.
- ARM ADRP layout holds: op=1, bits[28:24]=10000, Rd, reloc-form imm21=0 for Symbol/Label/SymbolOffset/Modifier-got (1000 cases).
- Changing Rd only changes bits[4:0]; changing symbol or GOT vs page reloc does not change the word (1000 cases).
- Symbol/Label produce AdrpPage21 addend 0; SymbolOffset preserves addend (1000 cases). Modifier{got} produces AdrGotPage21 addend 0 (1000 cases; ModifierOffset is a bug).
- Parser-misclassified Reg/Cond/Barrier names (s1, v0, d1, cc, lt, le, st, ld) are treated as symbols with AdrpPage21 (1000 cases).
- lr / uppercase Xn / x31 aliases encode Rd correctly (sweep, 1000 cases).
- Empty, missing operand 1, :lo12:, :got_lo12:, Imm, and Mem second operands return Err (1000 cases; extra operand does not — see bugs).
- W dest, SP, FP/SIMD dest, extra operands, and :got:sym+addend currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_adrp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). GNU as 2.38 agrees on symbol/GOT reloc forms and rejects W/SP/FP/extra/:lo12:/#imm.
- ARM ARM ADRP: 1 immlo[1:0] 10000 immhi[18:0] Rd. Rd is Xd (X31=XZR, not SP). GNU syntax is `adrp Xd, label` / `adrp Xd, :got:label`. llvm-mc additionally accepts page-aligned `#imm`; gas rejects `#imm` (README.md:12 gas contract — Imm is out of domain).
- Dispatch: encoder/mod.rs:524 `"adrp" => encode_adrp(operands)`.
- Sibling encode_adr is not a same-job differential (op=0 / AdrPrelLo21). Linker reloc::encode_adrp patches displacement (different job).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_adrp_diff_alt_spellings. Closed: every documented behavior has a property; remaining gaps are the three filed bugs.
- Three failing properties are SUT bugs. See pbt-out/bug_reports/encode_adrp_*.md.

## Quirks (encode_adrp)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_adrp does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- Modifier { kind: "got" } is handled; ModifierOffset { kind: "got" } is not (see bugs).
- GNU as rejects `adrp x0, #imm`; llvm-mc accepts page-aligned immediates. README.md:12 claims gas, so Imm Err is in-contract.
- llvm-mc accepts `x31` as XZR; GNU as rejects `x31`. Sweep used llvm-mc aliasing.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the ~40-line body.

# Confirmed invariants (encode_ldr_str)

- Valid unsigned LDR/STR/LDRB/STRB/LDRH/STRH Rt, [Xn|SP, #pimm] with pimm = imm12*(1<<size) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins ldr x0,[x1]=0xF9400020, ldr x0,[x1,#8]=0xF9400420, str w2,[x3,#4]=0xB9000462, ldrb w0,[x1,#1]=0x39400420, ldrh w0,[x1,#2]=0x79400420.
- Unscaled/pre/post with simm9 in [-256,255] (excluding writeback Rt==Rn) match llvm-mc, including LDR-to-LDUR canonicalization (1000 cases). KAT pins ldr x0,[x1,#4]=0xF8404020, post #8=0xF8408420, pre #8=0xF8408C20, [x1,x2]=0xF8626820.
- ARM unsigned GPR layout: size [31:30], bits[29:27]=111, V=0, bits[25:24]=01, opc=01 load / 00 store, imm12, Rn, Rt (1000 cases).
- Metamorphic: Rt+1 adds 1, Rn+1 adds 32, imm12+1 adds 1<<10, load XOR store = 1<<22, pre XOR post = 0b10<<10 (1000 cases).
- Arity 0/1 and non-memory second operands return Err (1000 cases).
- SIMD S/D/Q unsigned LDR/STR match llvm-mc (1000 cases, sweep).
- Alt spellings x31 / uppercase Xn,XZR,SP / lr match llvm-mc (1000 cases, sweep).
- LDR literal Symbol produces WordWithReloc { Ldr19, symbol, addend 0 } with opc 00/01; STR literal returns Err (1000 cases, sweep).
- SP dest, XZR/x31 base, W base, W-index without extend, writeback Rt==Rn, out-of-range offset, extra operand, and byte lsl #0 S-bit currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_ldr_str)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs, cargo test --lib encode_ldr_str, proptest cases=1000.

# Confirmed invariants (encode_uxtb)

- Valid UXTB Wd, Wn with Wd, Wn in {w0..w30, wzr, w31} (and w31/WZR/uppercase aliases) match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins uxtb w0,w1=0x53001c20, uxtb wzr,wzr=0x53001fff, ubfm w0,w1,#0,#7 aliases to the same word.
- ARM UXTB/UBFM-#0,#7 32-bit layout holds: sf=0; opc=10; bits[28:23]=100110; N=0; immr=0; imms=7; Rn/Rd match (1000 cases).
- UXTB equals UBFM with #0,#7 of the same W registers and llvm-mc (1000 cases).
- Rd/Rn n vs n+1 differ only in that 5-bit field (1000 cases).
- Arity 0 and 1 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) return Err (sweep, 1000 cases).
- ASCII case-fold / w31 / WZR aliases match llvm-mc (sweep, 1000 cases).
- X destination, extra operands, SP/WSP, X-register source, and FP/SIMD registers currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_uxtb)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). GNU as 2.38 agrees.
- ARM ARM C6 UXTB is the 32-bit-only alias of UBFM Wd, Wn, #0, #7: sf=0 opc=10 100110 N=0 immr=0 imms=7 Rn Rd. llvm-mc/gas accept UXTB Xd, Wn and canonicalize it to UXTB Wd, Wn (same 32-bit encoding). Register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:442 `"uxtb" => encode_uxtb(operands)`.
- Sibling encode_sxth / encode_sxtb / encode_uxth / encode_uxtw are not same-job independent differentials (SBFM / imms=15 / UXTW-ORR; shared get_reg / same crate). encode_ubfm is the UBFM alias (shared get_reg / same crate); used only as algebraic.metamorphic #0,#7.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 7-line body plus encode_uxtb_neg_fp / encode_uxtb_neg_invalid_name / encode_uxtb_neg_nonreg / encode_uxtb_diff_alt_spellings / encode_uxtb_meta_rd_rn. Closed: every documented behavior has a property; tier round spent.
- Five failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_uxtb_*.md.

## Quirks (encode_uxtb)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_uxtb does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- get_reg width of Rn is discarded; sf comes only from Rd, so an X destination encodes 64-bit UBFM (see bugs).
- llvm-mc aliases `ubfm w0, w1, #0, #7` to `uxtb w0, w1`; encodings still compare.
- llvm-mc/gas accept `uxtb x0, w0` and rewrite it as `uxtb w0, w0` (32-bit). They reject `uxtb x0, x1`.
- llvm-mc accepts `w31` as WZR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 7-line body.

# Confirmed invariants (encode_uxth)

- Valid UXTH Wd, Wn with Wd, Wn in {w0..w30, wzr, w31} (and w31/WZR/uppercase aliases) match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins uxth w0,w1=0x53003c20, uxth wzr,wzr=0x53003fff, ubfm w0,w1,#0,#15 aliases to the same word.
- ARM UXTH/UBFM-#0,#15 32-bit layout holds: sf=0; opc=10; bits[28:23]=100110; N=0; immr=0; imms=15; Rn/Rd match (1000 cases).
- UXTH equals UBFM with #0,#15 of the same W registers and llvm-mc (1000 cases).
- Rd/Rn n vs n+1 differ only in that 5-bit field (1000 cases).
- Arity 0 and 1 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) return Err (sweep, 1000 cases).
- ASCII case-fold / w31 / WZR aliases match llvm-mc (sweep, 1000 cases).
- X destination, extra operands, SP/WSP, X-register source, and FP/SIMD registers currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_uxth)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). GNU as 2.38 agrees.
- ARM ARM C6 UXTH is the 32-bit-only alias of UBFM Wd, Wn, #0, #15: sf=0 opc=10 100110 N=0 immr=0 imms=15 Rn Rd. llvm-mc/gas accept UXTH Xd, Wn and canonicalize it to UXTH Wd, Wn (same 32-bit encoding). Register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:439 `"uxth" => encode_uxth(operands)`.
- Sibling encode_sxth / encode_sxtb / encode_uxtb / encode_uxtw are not same-job independent differentials (SBFM / imms=7 / UXTW-ORR; shared get_reg / same crate). encode_ubfm is the UBFM alias (shared get_reg / same crate); used only as algebraic.metamorphic #0,#15.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 7-line body plus encode_uxth_neg_fp / encode_uxth_neg_invalid_name / encode_uxth_neg_nonreg / encode_uxth_diff_alt_spellings / encode_uxth_meta_rd_rn. Closed: every documented behavior has a property; tier round spent.
- Five failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_uxth_*.md.

## Quirks (encode_uxth)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_uxth does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- get_reg width of Rn is discarded; sf comes only from Rd, so an X destination encodes 64-bit UBFM (see bugs).
- llvm-mc aliases `ubfm w0, w1, #0, #15` to `uxth w0, w1`; encodings still compare.
- llvm-mc/gas accept `uxth x0, w0` and rewrite it as `uxth w0, w0` (32-bit). They reject `uxth x0, x1`.
- llvm-mc accepts `w31` as WZR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 7-line body.

# Confirmed invariants (encode_sxtb)

- Valid SXTB Rd, Wn with Rd in {x0..x30, xzr, x31, lr} or {w0..w30, wzr, w31} and Wn in {w0..w30, wzr, w31} match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins sxtb w0,w1=0x13001c20, sxtb x0,w1=0x93401c20, sxtb wzr,wzr=0x13001fff, sbfm w0,w1,#0,#7 aliases to the same word.
- ARM SXTB/SBFM-#0,#7 layout holds: opc=00; bits[28:23]=100110; N=sf; immr=0; imms=7; Rn/Rd match (1000 cases).
- SXTB equals SBFM with #0,#7 of the same registers and llvm-mc (1000 cases).
- Rd/Rn n vs n+1 differ only in that 5-bit field (1000 cases).
- Arity 0 and 1 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) return Err (sweep, 1000 cases).
- ASCII case-fold / x31 / LR / X-source-with-64-bit-dest aliases match llvm-mc (sweep, 1000 cases).
- Extra operands, SP/WSP, Wd+Xn, and FP/SIMD registers currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_sxtb)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM C6 SXTB is the alias of SBFM Rd, Rn, #0, #7: sf 00 100110 N=sf immr=0 imms=7 Rn Rd. Forms SXTB Wd, Wn and SXTB Xd, Wn. Register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:435 `"sxtb" => encode_sxtb(operands)`.
- Sibling encode_sxth / encode_sxtw / encode_uxtb are not same-job independent differentials (imms=15 / 32-bit-only / UBFM; shared get_reg / same crate). encode_sbfm is the SBFM alias (shared get_reg / same crate); used only as algebraic.metamorphic #0,#7.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 8-line body plus encode_sxtb_neg_fp / encode_sxtb_neg_invalid_name / encode_sxtb_neg_nonreg / encode_sxtb_diff_alt_spellings / encode_sxtb_meta_rd_rn. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_sxtb_*.md.

## Quirks (encode_sxtb)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_sxtb does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- get_reg width of Rn is discarded; sf comes only from Rd (see bugs).
- llvm-mc aliases `sbfm w0, w1, #0, #7` to `sxtb w0, w1`; encodings still compare.
- llvm-mc accepts `x31` as XZR/WZR and canonicalizes `sxtb Xd, Xn` to `sxtb Xd, Wn`.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 8-line body.

# Confirmed invariants (encode_mneg)

- Valid MNEG Rd, Rn, Rm with same-width GPRs in {x0..x30, xzr, x31, lr} or {w0..w30, wzr, w31} match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins mneg x0,x1,x2=0x9b02fc20, mneg w0,w1,w2=0x1b02fc20, mneg xzr,xzr,xzr=0x9b1fffff, mneg lr,x1,x2=0x9b02fc3e, msub x0,x1,x2,xzr aliases to the same word.
- ARM MNEG/MSUB-ZR layout holds: bits[30:21]=0011011000; o0=1; Ra=31; Rm/Rn/Rd match; sf from width (1000 cases).
- MNEG equals MSUB with Ra=ZR of the same registers and llvm-mc (1000 cases).
- X-form XOR W-form = 1<<31 (1000 cases).
- Rd/Rn/Rm n vs n+1 differ only in that 5-bit field (1000 cases).
- Arity 0, 1, and 2 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) return Err (sweep, 1000 cases).
- ASCII case-fold / x31 / LR aliases match llvm-mc (sweep, 1000 cases).
- Extra operands, SP/WSP, mixed W/X widths, and FP/SIMD registers currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_mneg)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Data-processing (3 source) MNEG is the alias of MSUB Rd, Rn, Rm, ZR: sf 00 11011 000 Rm o0=1 Ra=11111 Rn Rd. Register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:409 `"mneg" => encode_mneg(operands)`.
- Sibling encode_msub is not a same-job independent differential (4-operand vs 3-operand alias; shared get_reg / same TU); used only as algebraic.metamorphic Ra=ZR alias.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 12-line body plus encode_mneg_neg_fp / encode_mneg_neg_invalid_name / encode_mneg_neg_nonreg / encode_mneg_diff_alt_spellings / encode_mneg_meta_rd_rn_rm. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_mneg_*.md.

## Quirks (encode_mneg)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_mneg does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- get_reg width of Rn/Rm is discarded; sf comes only from Rd (see bugs).
- llvm-mc aliases `msub x0, x1, x2, xzr` to `mneg x0, x1, x2`; encodings still compare.
- llvm-mc accepts `x31` as XZR/WZR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 12-line body.

# Confirmed invariants (encode_smaddl)

- Valid SMADDL Xd, Wn, Wm, Xa with Rd/Ra in {x0..x30, xzr, x31, lr} and Rn/Rm in {w0..w30, wzr, w31} match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins smaddl x0,w1,w2,x3=0x9b220c20, smaddl xzr,wzr,wzr,xzr=0x9b3f7fff, smaddl x0,w1,w2,xzr=0x9b227c20 (alias smull), smaddl lr,w1,w2,x30=0x9b22783e.
- ARM SMADDL layout holds: sf=1; bits[30:21]=0011011001; o0=0; Rm/Ra/Rn/Rd match the generated fields (1000 cases).
- SMADDL with Ra=XZR equals SMULL of the same Xd,Wn,Wm and llvm-mc (1000 cases).
- SMADDL XOR UMADDL of the same registers = 1<<23 (U bit) (1000 cases).
- Rd/Rn/Rm/Ra n vs n+1 differ only in that 5-bit field (1000 cases).
- Arity 0, 1, 2, and 3 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) return Err (sweep, 1000 cases).
- ASCII case-fold / x31 / LR aliases match llvm-mc (sweep, 1000 cases).
- Extra operands, SP/WSP, wrong W/X widths, and FP/SIMD registers currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_smaddl)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Data-processing (3 source) SMADDL: sf=1 U=0 11011 001 Rm o0=0 Ra Rn Rd. SMULL is the alias with Ra=XZR. Rd/Ra are XZR not SP; Rn/Rm are WZR not WSP.
- Dispatch: encoder/mod.rs:405 `"smaddl" => encode_smaddl(operands)`.
- Sibling encode_umaddl is not a same-job differential (U=1 unsigned); used only as metamorphic U-bit XOR. encode_smull is the Ra=XZR alias (shared get_reg / same TU).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 12-line body plus encode_smaddl_neg_fp / encode_smaddl_neg_invalid_name / encode_smaddl_neg_nonreg / encode_smaddl_diff_alt_spellings / encode_smaddl_meta_rd_rn_rm_ra. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_smaddl_*.md.

## Quirks (encode_smaddl)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_smaddl does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- get_reg width is discarded (see bugs).
- llvm-mc aliases `smaddl x0, w1, w2, xzr` to `smull x0, w1, w2`; encodings still compare.
- llvm-mc accepts `x31` as XZR/WZR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 12-line body.

# Confirmed invariants (encode_crc32)

- Valid CRC32/CRC32C (B/H/W with Wd,Wn,Wm and X with Wd,Wn,Xm; ZR aliases; w31/x31/uppercase/lr-as-Xm) match llvm-mc `-triple=aarch64 -mattr=+crc -show-encoding` (1000 cases). KAT pins crc32b w0,w1,w2=0x1ac24020, crc32h=0x1ac24420, crc32w=0x1ac24820, crc32x w0,w1,x2=0x9ac24c20, crc32cb=0x1ac25020, crc32cx=0x9ac25c20, crc32b wzr,wzr,wzr=0x1adf43ff, crc32x wzr,w0,xzr=0x9adf4c1f.
- ARM CRC32 layout holds: bits[30:21]=0011010110; bit31=sf; bits[20:16]=Rm; bits[15:13]=010; bit12=C; bits[11:10]=sz; bits[9:5]=Rn; bits[4:0]=Rd (1000 cases).
- CRC32C XOR CRC32 of the same size = 1<<12; B XOR H = 1<<10; W XOR X = (1<<31)|(1<<10) (1000 cases).
- Rd/Rn/Rm n vs n+1 differ only in that 5-bit field (1000 cases).
- Arity 0, 1, and 2 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) return Err (sweep, 1000 cases).
- Extra operands, SP/WSP, wrong W/X widths, and FP/SIMD registers currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_crc32)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+crc -show-encoding (LLVM 15.0.6).
- ARM ARM CRC32/CRC32C: sf 00 11010110 Rm 010 C sz Rn Rd. C=1 for CRC32C. sz 00/01/10/11 = B/H/W/X. sf=1 only for X. Rd/Rn always W; Rm is W for B/H/W and X for X. Register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:1047-1048 `"crc32b"|...|"crc32cx" => encode_crc32(mnemonic, operands)`.
- Sibling encode_clz/encode_cls are not same-job differentials (Data-processing 2-source).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 20-line body plus encode_crc32_neg_fp / encode_crc32_neg_invalid_name / encode_crc32_neg_nonreg / encode_crc32_diff_alt_spellings. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_crc32_*.md.

## Quirks (encode_crc32)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_crc32 does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- get_reg width is discarded; sf/sz come only from the mnemonic (see bugs).
- Unknown mnemonic `_` arm encodes as crc32b (sf=0,sz=00); not caller-reachable from encode_instruction (match on the eight names after lowercase).
- llvm-mc requires `-mattr=+crc`.
- llvm-mc accepts `w31`/`x31` as WZR/XZR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 20-line body.

# Confirmed invariants (encode_tbz)

- Symbol/label/SymbolOffset form of TBZ/TBNZ emits WordWithReloc TstBr14 (ELF 279) with imm14=0 (1000 cases). KAT pins tbz x0, #0, foo = word 0x36000000, reloc TstBr14 symbol=foo addend=0.
- ARM Test-and-branch layout holds: bits[30:25]=011011; bit31=b5; bit24=op; bits[23:19]=b40; bits[18:5]=0; bits[4:0]=Rt (1000 cases).
- TBNZ XOR TBZ = 1<<24 with identical reloc (1000 cases).
- Rt n vs n+1 differs only in bits[4:0] (1000 cases).
- Arity 0, 1, and 2 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-symbol label kinds (Mem/Shift/Extend/RegArrangement/Expr/RegList) return Err (sweep, 1000 cases).
- Immediate PC-offset form, extra operands, SP/WSP, FP/SIMD Rt, and out-of-range bits currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_tbz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Test and branch (immediate): b5 011011 op b40 imm14 Rt. Bit = b5:b40 in 0..63 (W requires b5=0 so 0..31). Offset/4 in ±32 KiB.
- Dispatch: encoder/mod.rs:455-456 `"tbz" => encode_tbz(operands, false)` / `"tbnz" => encode_tbz(operands, true)`.
- Sibling encode_cbz is not a same-job differential (CondBr19, no bit operand).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 16-line body plus encode_tbz_neg_invalid_name / encode_tbz_meta_rt_isolation / encode_tbz_neg_bad_label_kind. Closed: every documented behavior has a property; tier round spent.
- Five failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_tbz_*.md.

## Quirks (encode_tbz)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_tbz does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- Bit number is masked to 6 bits with no range check; get_reg width is discarded (see bugs).
- Operand 2 is get_symbol only; Imm PC offsets are rejected (see bugs).
- llvm-mc aliases `tbz x0, #0, #0` to `tbz w0, #0, #0` when bit < 32 (b5=0); encodings still compare.
- llvm-mc accepts `x31` as XZR/WZR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 16-line body.

# Confirmed invariants (encode_tst)

- Valid TST shifted-register with matching W/X GPRs 0..31 (ZR, LR, x31, uppercase) and optional lsl/lsr/asr/ror in range match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins tst x0,x1=0xea01001f, tst w0,w1=0x6a01001f, tst x0,x1,ror #7=0xeac11c1f, ands xzr,x0,x1 aliases tst x0,x1.
- Valid TST bitmask-immediate form matches llvm-mc (1000 cases). KAT pins tst x0,#1=0xf240001f, tst w0,#1=0x7200001f.
- ARM ANDS shifted-register layout with Rd=31, opc=11 holds: bits[4:0]=31; bits[30:29]=11; bits[28:24]=01010; bit21=0; sf/Rn/Rm/shift/imm6 match (1000 cases).
- encode_tst(ops) equals encode_logical([ZR]++ops, 0b11) with ZR = wzr iff Rn is 32-bit (1000 cases).
- Field isolation: Rn+1 flips bits[9:5]; Rm+1 flips bits[20:16]; amt+1 flips bits[15:10]; W vs X flips only bit 31 (1000 cases).
- Arity 0 and 1 return Err (1000 cases).
- Unparsable names (x32, foo, empty, r0) return Err (sweep, 1000 cases).
- Non-bitmask immediates llvm-mc rejects (#0, #-1, …) return Err (sweep, 1000 cases).
- Extra operands, SP/WSP, mixed W/X, FP/SIMD, and out-of-range shift amounts currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_tst)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Logical (shifted register) ANDS Rd=XZR/WZR: sf 11 01010 shift 0 Rm imm6 Rn Rd. Logical (immediate) ANDS Rd=XZR/WZR: sf 11 100100 N immr imms Rn Rd.
- Dispatch: encoder/mod.rs:433 `"tst" => encode_tst(operands)`.
- Sibling encode_logical is not a same-job differential (3-operand ANDS); used only as metamorphic alias. encode_cmp/encode_cmn are SUBS/ADDS aliases.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the 12-line body plus encode_tst_neg_invalid_name / encode_tst_neg_invalid_imm / encode_tst_neg_shift_oor. Closed: every documented behavior has a property; tier round spent.
- Five failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_tst_*.md.

## Quirks (encode_tst)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_tst does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra non-Shift operands are ignored (see bugs).
- Shift amount is masked `& 0x3F` with no W/X range check (see bugs).
- llvm-mc aliases `ands xzr, x0, x1` to `tst x0, x1`; encodings still compare.
- llvm-mc accepts `x31` as XZR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 12-line body.

# Confirmed invariants (encode_fnmadd_fnmsub)

- Valid FNMADD/FNMSUB with matching S or D registers 0..31 (lowercase or uppercase) match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins fnmadd s0,s1,s2,s3=0x1f220c20, fnmadd d0,d1,d2,d3=0x1f620c20, fnmsub s0,s1,s2,s3=0x1f228c20, fnmsub d0,d1,d2,d3=0x1f628c20, fnmadd s31,s31,s31,s31=0x1f3f7fff, uppercase S0..S3=0x1f220c20.
- ARM FP 3-source layout with o1=1 holds: bits[31:24]=0b00011111; bit21=1; ftype/o0/Rm/Ra/Rn/Rd match the generated fields (1000 cases).
- Encodings of the same registers differ only in the mutated field; FNMADD XOR FNMSUB = 1<<15; S vs D flips only bit 22 (1000 cases).
- Arity < 4 and non-register operands return Err (1000 cases).
- Unparsable register names (foo, s32, empty, r0) return Err (sweep, 1000 cases).
- Extra operands, mixed S/D, GPR, Q/V/B, SP, and H-register ftype currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_fnmadd_fnmsub)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6); half-precision -mattr=+fullfp16.
- ARM ARM Floating-point data-processing (3 source): M=0 S=0 11111 ftype o1 Rm o0 Ra Rn Rd. FNMADD o1=1 o0=0, FNMSUB o1=1 o0=1. ftype 00=S, 01=D, 11=H.
- Dispatch: encoder/mod.rs:565-566 fnmadd/fnmsub => encode_fnmadd_fnmsub(operands, is_sub).
- Sibling encode_fmadd_fmsub is not a same-job differential (o1=0 non-negated class).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_fnmadd_fnmsub_neg_invalid_name. Closed: every documented behavior has a property; tier round spent.
- Three failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fnmadd_fnmsub_*.md.

## Quirks (encode_fnmadd_fnmsub)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_fnmadd_fnmsub does not distinguish them (see bugs).
- get_reg accepts FP prefixes (d/s/q/v/h/b) and GPR (see bugs).
- No operands.len() check; extra operands are ignored (see bugs).
- ftype is only `rd_name.starts_with('d')` else 00, so H encodes as S (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 16-line body.

# Confirmed invariants (encode_stop)

- Valid STADD/STCLR/STEOR/STSET variants (4 ops × 6 release/byte/half suffixes; no acquire) with Rs in {w/x 0..30, wzr/xzr} (W-only for byte/half) and base Xn|SP match llvm-mc `-triple=aarch64 -mattr=+lse -show-encoding` (1000 cases). KAT pins stadd w0,[x1]=0xB820003F, stadd x0,[x1]=0xF820003F, staddl w0,[x1]=0xB860003F, staddb w0,[x1]=0x3820003F, staddh w0,[x1]=0x7820003F, stclr w0,[x1]=0xB820103F, steor w0,[x1]=0xB820203F, stset w0,[x1]=0xB820303F, stadd xzr,[sp]=0xF83F03FF.
- ARM STADD alias layout holds: bits[29:24]=0b111000; bit21=1; bit15=0; bits[11:10]=0; A=0; Rt=31; size/R/opc/Rs/Rn match the generated fields (1000 cases).
- Encodings of the same registers differ only in the mutated field; STADDL XOR STADD = 1<<22; STCLR/STEOR/STSET XOR STADD = opc<<12; uppercase STADD matches lowercase (1000 cases).
- Arity < 2 and non-Mem second operand return Err (1000 cases).
- Unparsable register/base names (foo, x32, empty, r0) return Err (sweep, 1000 cases).
- Unknown mnemonics that do not start with stadd/stclr/steor/stset return Err (sweep, 1000 cases).
- ASCII case-fold of the mnemonic matches llvm-mc (sweep, 1000 cases).
- Extra operands, SP/WSP as Rs, XZR/W as base, FP Rs, STADDB with X, and nonzero Mem offset currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_stop)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+lse -show-encoding (LLVM 15.0.6).
- ARM ARM STADD is the store alias of LDADD with Rt=WZR/XZR: size 111000 A R 1 Rs 0 opc 00 Rn Rt. A=0; Rt=31. opc STADD=000 STCLR=001 STEOR=010 STSET=011. R=release. Rs is ZR not SP; Rn is Xn|SP not ZR. STADDB/STADDH require W registers. Optional offset only #0.
- Dispatch: encoder/mod.rs:1065-1068 stadd*|stclr*|steor*|stset* => encode_stop(mnemonic, operands).
- Sibling encode_cas / encode_swp / encode_ldop are not same-job differentials (different LSE class / operand grammar).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_stop_neg_invalid_name, encode_stop_diff_alt_spellings, encode_stop_neg_unknown_op. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_stop_*.md.

## Quirks (encode_stop)

- ASCII case of the mnemonic is accepted (to_lowercase).
- parse_reg_num maps SP and XZR both to 31; encode_stop does not distinguish them (see bugs).
- get_reg accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- operands.len() < 2 does not reject extra operands (see bugs).
- Operand::Mem { base, .. } ignores offset (see bugs).
- llvm-mc aliases `ldadd xzr, xzr, [sp]` to `stadd xzr, [sp]`; encodings still compare.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 45-line body.

# Confirmed invariants (encode_ldop)

- Valid LDADD/LDCLR/LDEOR/LDSET variants (4 ops × 12 acquire/release/byte/half suffixes) with Rs/Rt in {w/x 0..30, wzr/xzr} (W-only for byte/half) and base Xn|SP match llvm-mc `-triple=aarch64 -mattr=+lse -show-encoding` (1000 cases). KAT pins ldadd x0,x1,[x2]=0xF8200041, ldadd w0,w1,[x2]=0xB8200041, ldadda w0,w1,[x2]=0xB8A00041, ldaddl w0,w1,[x2]=0xB8600041, ldaddal w0,w1,[x2]=0xB8E00041, ldaddb w0,w1,[x2]=0x38200041, ldaddh w0,w1,[x2]=0x78200041, ldclr w0,w1,[x2]=0xB8201041, ldeor w0,w1,[x2]=0xB8202041, ldset w0,w1,[x2]=0xB8203041, ldadd xzr,xzr,[sp]=0xF83F03FF.
- ARM LDADD layout holds: bits[29:24]=0b111000; bit21=1; bit15=0; bits[11:10]=0; size/A/R/opc/Rs/Rn/Rt match the generated fields (1000 cases).
- Encodings of the same registers differ only in the mutated field; LDADDA XOR LDADD = 1<<23; LDADDL XOR LDADD = 1<<22; LDADDAL XOR LDADD = (1<<23)|(1<<22); LDCLR/LDEOR/LDSET XOR LDADD = opc<<12; uppercase LDADD matches lowercase (1000 cases).
- Arity < 3 and non-Mem third operand return Err (1000 cases).
- Unparsable register/base names (foo, x32, empty, r0) return Err (sweep, 1000 cases).
- Unknown mnemonics that do not start with ldadd/ldclr/ldeor/ldset return Err (sweep, 1000 cases).
- ASCII case-fold of the mnemonic matches llvm-mc (sweep, 1000 cases).
- Extra operands, SP/WSP as Rs/Rt, XZR/W as base, mixed W/X, FP Rs/Rt, LDADDB with X, and nonzero Mem offset currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_ldop)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+lse -show-encoding (LLVM 15.0.6).
- ARM ARM LDADD: size 111000 A R 1 Rs 0 opc 00 Rn Rt. opc LDADD=000 LDCLR=001 LDEOR=010 LDSET=011. A=acquire, R=release. Rs/Rt are ZR not SP; Rn is Xn|SP not ZR. LDADDB/LDADDH require W registers. Optional offset only #0.
- Dispatch: encoder/mod.rs:1050-1061 ldadd*|ldclr*|ldeor*|ldset* => encode_ldop(mnemonic, operands).
- Sibling encode_cas / encode_swp / encode_stop are not same-job differentials (different LSE class / operand grammar).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_ldop_neg_invalid_name, encode_ldop_diff_alt_spellings, encode_ldop_neg_unknown_op. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldop_*.md.

## Quirks (encode_ldop)

- ASCII case of the mnemonic is accepted (to_lowercase).
- parse_reg_num maps SP and XZR both to 31; encode_ldop does not distinguish them (see bugs).
- get_reg accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- operands.len() < 3 does not reject extra operands (see bugs).
- Operand::Mem { base, .. } ignores offset (see bugs).
- llvm-mc aliases `ldadd xzr, xzr, [sp]` to `stadd xzr, [sp]`; encodings still compare.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 45-line body.

# Confirmed invariants (encode_swp)

- Valid SWP variants {swp,swpa,swpal,swpl,swpb,swpab,swpalb,swplb,swph,swpah,swpalh,swplh} with Rs/Rt in {w/x 0..30, wzr/xzr} (W-only for byte/half) and base Xn|SP match llvm-mc `-triple=aarch64 -mattr=+lse -show-encoding` (1000 cases). KAT pins swp x0,x1,[x2]=0xF8208041, swp w0,w1,[x2]=0xB8208041, swpa w0,w1,[x2]=0xB8A08041, swpl w0,w1,[x2]=0xB8608041, swpal w0,w1,[x2]=0xB8E08041, swpb w0,w1,[x2]=0x38208041, swph w0,w1,[x2]=0x78208041, swp xzr,xzr,[sp]=0xF83F83FF.
- ARM SWP layout holds: bits[29:24]=0b111000; bit21=1; bit15=1; bits[14:10]=0; size/A/R/Rs/Rn/Rt match the generated fields (1000 cases).
- Encodings of the same (size,A,R) differ only in the mutated register field; SWPA XOR SWP = 1<<23; SWPL XOR SWP = 1<<22; SWPAL XOR SWP = (1<<23)|(1<<22); uppercase SWP matches lowercase (1000 cases).
- Arity < 3 and non-Mem third operand return Err (1000 cases).
- Unparsable register/base names (foo, x32, empty, r0) return Err (sweep, 1000 cases).
- ASCII case-fold of the mnemonic matches llvm-mc (sweep, 1000 cases).
- Extra operands, SP/WSP as Rs/Rt, XZR/W as base, mixed W/X, FP Rs/Rt, SWPB with X, and nonzero Mem offset currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_swp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+lse -show-encoding (LLVM 15.0.6).
- ARM ARM SWP: size 111000 A R 1 Rs 1 000 00 Rn Rt. A=acquire, R=release. Rs/Rt are ZR not SP; Rn is Xn|SP not ZR. SWPB/SWPH require W registers. Optional offset only #0.
- Dispatch: encoder/mod.rs:1045-1047 swp* => encode_swp(mnemonic, operands).
- Sibling encode_cas / encode_ldop / encode_stop are not same-job differentials (different LSE class / operand grammar).
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_swp_neg_invalid_name and encode_swp_diff_alt_spellings. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_swp_*.md.

## Quirks (encode_swp)

- ASCII case of the mnemonic is accepted (to_lowercase).
- parse_reg_num maps SP and XZR both to 31; encode_swp does not distinguish them (see bugs).
- get_reg accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- operands.len() < 3 does not reject extra operands (see bugs).
- Operand::Mem { base, .. } ignores offset (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 30-line body.

# Confirmed invariants (encode_tlbi)

- Valid implemented TLBI ops (no-Xt: vmalle1is/vmalle1/alle1is/alle1/alle2is/vmalls12e1is/vmalls12e1; Xt-required v8.0 plus FEAT_TLBIRANGE with Xt in {x0..x30, xzr, x31, lr}; ASCII case and surrounding space/tab) match llvm-mc `-triple=aarch64 -show-encoding` (`-mattr=+tlb-rmi` for range ops) and the ARM SYS formula 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt with CRn=8 (1000 cases). KAT pins vmalle1is=0xd508831f, vae1is x0=0xd5088320, vale1is x0=0xd50883a0, alle2is=0xd50c831f, rvae1is x0=0xd5088220, vae1is xzr=0xd508833f.
- ARM TLBI layout holds: bits[31:21]=0b11010101000; CRn=8; extracted (op1,CRm,op2,Rt) match the ARM table; no-Xt Rt=31 (1000 cases).
- Encodings of the same Xt-required op differ only in Rt bits[4:0] (1000 cases).
- ASCII case-fold and surrounding space/tab are behavior-preserving on the valid domain (1000 cases).
- Malformed Xt (x32, empty, #0, foo) return Err containing "invalid register" or "unsupported tlbi operation" (sweep, 1000 cases).
- Unknown operation names that llvm-mc rejects return Err containing "unsupported tlbi operation" or "invalid register" (sweep, 1000 cases).
- Missing Xt, extra Xt on no-Xt ops, W/SP/SIMD Xt, and unimplemented ARM default-CPU ops (alle2/alle3/vae3/vale3) currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_tlbi)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6); range ops -mattr=+tlb-rmi.
- ARM ARM TLBI is SYS with CRn=8. No-Xt: VMALLE*/ALLE*/VMALLS12E1*. Xt-required: VA*/VALE*/VAAE*/VAALE*/ASIDE*/IPAS2*/R*.
- Dispatch: encoder/mod.rs:990 `"tlbi" => encode_tlbi(operands, raw_operands)`. Raw operand string passed through; `_operands` unused.
- Sibling encode_ic / encode_dc / encode_at / encode_sys are not same-job differentials (different SYS encodings).
- encode_tlbi splits on the first comma, lowercases the op, optionally parses Rt via parse_reg_num (default 31), then patches bits[4:0] of a GCC base word.
- RIPAS2E1OS is SYS #4, C8, C4, #3 (op2=3), matching llvm-mc; not op2=4.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_tlbi_neg_invalid_reg and encode_tlbi_neg_unknown_op. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_tlbi_*.md.

## Quirks (encode_tlbi)

- Surrounding whitespace and ASCII case are accepted (trim + to_lowercase).
- llvm-mc accepts `x31` as XZR; GNU gas rejects `x31`. The differential used llvm-mc.
- parse_reg_num accepts `lr` as 30 (passing); it does not accept `fp` (x29), which gas/llvm-mc do.
- Extra operands after a parsed Xt-required register (`vae1is, x0, x1`) fail parse_reg_num and return Err (passing).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the ~60-line body.

# Confirmed invariants (encode_at)

- Valid {s1e1r, s1e1w, s1e0r, s1e0w} with Xt in {x0..x30, xzr, x31, lr} (ASCII case and surrounding space/tab) match llvm-mc `-triple=aarch64 -show-encoding` and the ARM SYS formula 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt with op1=0, CRn=7, CRm=8, op2∈{0,1,2,3} (1000 cases). KAT pins s1e1r x0=0xd5087800, s1e1w x0=0xd5087820, s1e0r x0=0xd5087840, s1e0w x0=0xd5087860, s1e1r xzr=0xd508781f.
- ARM AT layout holds: bits[31:21]=0b11010101000; op1=0; CRn=7; CRm=8; op2 distinguishes the four ops; Rt=t (1000 cases).
- Encodings of the same op differ only in Rt bits[4:0]; S1E1W XOR S1E1R = 0x20; S1E0R XOR S1E1R = 0x40; S1E0W XOR S1E1R = 0x60 (1000 cases).
- ASCII case-fold and surrounding space/tab are behavior-preserving on the valid domain (1000 cases).
- Unknown operation names that llvm-mc rejects return Err containing "unsupported at operation" or "invalid register" (1000 cases).
- Extra operands after Xt return Err, matching llvm-mc/gas (1000 cases).
- Malformed Xt (x32, empty, #0, foo) return Err containing "invalid register" (sweep, 1000 cases).
- Missing Xt, W/SP/SIMD Xt, and unimplemented ARM AT ops (s1e2r/…) currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_at)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM AT: S1E1R = SYS #0, C7, C8, #0, Xt; S1E1W op2=1; S1E0R op2=2; S1E0W op2=3. Xt required, 64-bit GPR.
- Dispatch: encoder/mod.rs:998 `"at" => encode_at(operands, raw_operands)`. Raw operand string passed through; `_operands` unused.
- Sibling encode_ic / encode_dc / encode_tlbi / encode_sys are not same-job differentials (different SYS encodings).
- encode_at splits on the first comma, lowercases the op, optionally parses Rt via parse_reg_num (default 31), then patches bits[4:0] of a GCC base word.
- No ARM codegen caller currently emits `at`; encode_instruction still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_at_neg_invalid_reg. Closed: every documented behavior has a property; tier round spent.
- Three failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_at_*.md.

## Quirks (encode_at)

- Surrounding whitespace and ASCII case are accepted (trim + to_lowercase).
- llvm-mc accepts `x31` as XZR; GNU gas rejects `x31`. The differential used llvm-mc.
- parse_reg_num accepts `lr` as 30 (passing); it does not accept `fp` (x29), which gas/llvm-mc do.
- Extra operands after a parsed register (`s1e1r, x0, x1`) fail parse_reg_num and return Err (passing).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 20-line body.

# Confirmed invariants (encode_sys)

- Valid SYS op1∈[0,7] × CRn∈C0–C15 × CRm∈C0–C15 × op2∈[0,7] × Xt∈{x0..x30, xzr, x31, lr, omitted} matches the ARM SYS formula 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt (1000 cases). KAT pins #0,c0,c0,#0,x0=0xd5080000; #3,c7,c14,#1,x0=0xd50b7e20; #0,c7,c1,#0 (omitted Xt)=0xd508711f; #7,c15,c15,#7,x0=0xd50fffe0; #3,c7,c14,#1,xzr=0xd50b7e3f. The GNU alias `fp` (x29) is rejected (see bugs).
- ARM SYS layout holds: bits[31:21]=0b11010101000; extracted op1/CRn/CRm/op2/Rt equal the generated fields (1000 cases).
- Encodings of the same (op1,CRn,CRm,op2) differ only in Rt bits[4:0]; omitted Xt encodes as xzr; xzr encodes as x31 (1000 cases).
- ASCII case-fold and surrounding space/tab (and optional `#`) are behavior-preserving on the valid domain (1000 cases).
- Fewer than 4 operands return Err, matching llvm-mc/gas (1000 cases).
- Malformed Xt that parse_reg_num rejects (x32, foo, empty) return Err containing "invalid register" (sweep, 1000 cases).
- Non-numeric op1/CRn/CRm/op2 return Err containing "invalid op1/CRn/CRm/op2" (sweep, 1000 cases).
- Out-of-range fields, extra operands, and W/SP/SIMD Xt currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_sys)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM SYS: SYS #<op1>, <Cn>, <Cm>, #<op2>{, <Xt>} = 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt; Xt optional → XZR.
- Dispatch: encoder/mod.rs:997 `"sys" => encode_sys(raw_operands)`. Raw operand string passed through unchanged.
- Sibling encode_ic / encode_dc / encode_tlbi / encode_at are not same-job differentials (named aliases of fixed SYS encodings).
- encode_sys splits on comma, strips optional `#` on op1/op2 and optional `c` on Cn/Cm, optionally parses Rt via parse_reg_num, then packs with field masks.
- No ARM codegen caller currently emits `sys`; encode_instruction still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_sys_neg_invalid_reg and encode_sys_neg_non_numeric. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_sys_*.md.

## Quirks (encode_sys)

- Surrounding whitespace and ASCII case are accepted (trim + to_lowercase).
- llvm-mc accepts `x31` as XZR; GNU gas rejects `x31`. The differential used llvm-mc.
- Optional `#` on op1/op2 is accepted (trim_start_matches('#')), matching llvm-mc/gas.
- llvm-mc aliases SYS to DC/IC when fields match; encodings still compare.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 22-line body.

# Confirmed invariants (encode_dc)

- Valid {civac, cvac, cvap, cvau, ivac, zva} with Xt in {x0..x30, xzr, x31, lr} (ASCII case and surrounding space/tab) match llvm-mc `-triple=aarch64 -show-encoding` (`-mattr=+ccpp` for CVAP) and the ARM SYS formula 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt (1000 cases). KAT pins civac x0=0xd50b7e20, cvac x0=0xd50b7a20, cvap x0=0xd50b7c20, cvau x0=0xd50b7b20, ivac x0=0xd5087620, zva x0=0xd50b7420, civac xzr=0xd50b7e3f.
- ARM DC layout holds: bits[31:21]=0b11010101000; CRn=7; op2=1; CIVAC op1=3 CRm=14; CVAC CRm=10; CVAP CRm=12; CVAU CRm=11; IVAC op1=0 CRm=6; ZVA op1=3 CRm=4; Rt=t (1000 cases).
- Encodings of the same op differ only in Rt bits[4:0]; CIVAC and CVAC encode distinctly (1000 cases).
- ASCII case-fold and surrounding space/tab are behavior-preserving on the valid domain (1000 cases).
- Malformed Xt (x32, empty, #0, foo) return Err containing "invalid register" or "unsupported dc variant" (sweep, 1000 cases).
- Unknown op names that do not substring-match an implemented token return Err (sweep, 1000 cases).
- Substring names (civacs, gzva), missing Xt, extra operands, and W/SP/SIMD Xt currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_dc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6), -mattr=+ccpp for CVAP.
- ARM ARM DC: CIVAC = SYS #3, C7, C14, #1, Xt; CVAC = SYS #3, C7, C10, #1, Xt; CVAP = SYS #3, C7, C12, #1, Xt; CVAU = SYS #3, C7, C11, #1, Xt; IVAC = SYS #0, C7, C6, #1, Xt; ZVA = SYS #3, C7, C4, #1, Xt.
- Dispatch: encoder/mod.rs:983 `"dc" => encode_dc(operands, raw_operands)`. Parser emits Symbol+Reg; raw_operands passed through unchanged.
- Sibling encode_ic / encode_tlbi / encode_at / encode_sys are not same-job differentials (different SYS encodings).
- encode_dc lowercases the first Symbol (or the raw string), takes Rt from operands.get(1) or last Reg or 0, then `contains()`-matches the six names.
- No ARM codegen caller currently emits `dc`; encode_instruction still routes the mnemonic.
- Unimplemented ARM DC ops llvm-mc accepts on the default CPU (cisw, csw, isw) are a codegen subset (encoder/mod.rs:3), not in the positive domain.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_dc_neg_invalid_reg and encode_dc_neg_unknown_nonsubstr. Closed: every documented behavior has a property; tier round spent.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_dc_*.md.

## Quirks (encode_dc)

- Surrounding whitespace and ASCII case are accepted (to_lowercase on Symbol).
- llvm-mc accepts `x31` as XZR; GNU gas rejects `x31`. The differential used llvm-mc.
- parse_reg_num accepts `lr` as 30 (passing); it does not accept `fp` (x29), which gas/llvm-mc do.
- llvm-mc needs `-mattr=+ccpp` to assemble CVAP; encodings then match the SUT.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 50-line body.

# Confirmed invariants (encode_ic)

- Valid IALLUIS / IALLU (no Xt) and IVAU with Xt in {x0..x30, xzr, x31, lr} (ASCII case and surrounding space/tab) match llvm-mc `-triple=aarch64 -show-encoding` and the ARM SYS formula 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt (1000 cases). KAT pins ialluis=0xd508711f, iallu=0xd508751f, ivau x0=0xd50b7520, ivau xzr=0xd50b753f.
- ARM IC layout holds: bits[31:21]=0b11010101000; IALLUIS op1=0 CRn=7 CRm=1 op2=0 Rt=31; IALLU CRm=5; IVAU op1=3 CRn=7 CRm=5 op2=1 Rt=t (1000 cases).
- IVAU encodings differ only in Rt bits[4:0]; IALLU XOR IALLUIS = 0x400 (CRm nibble) (1000 cases).
- ASCII case-fold and surrounding space/tab are behavior-preserving on the valid domain (1000 cases).
- Unknown operation names return Err containing "unsupported ic operation" or "invalid register", matching llvm-mc/gas (1000 cases).
- Malformed IVAU Xt (x32, empty, extra operands, #imm) return Err containing "invalid register" (sweep, 1000 cases).
- IALLUIS/IALLU with a register, IVAU without Xt, and IVAU with W/SP/SIMD currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_ic)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM IC: IALLUIS = SYS #0, C7, C1, #0; IALLU = SYS #0, C7, C5, #0; IVAU = SYS #3, C7, C5, #1, Xt.
- Dispatch: encoder/mod.rs:983 `"ic" => encode_ic(raw_operands)`. Raw operand string passed through unchanged; parser does not lowercase it.
- Sibling encode_dc / encode_tlbi / encode_at / encode_sys are not same-job differentials (different SYS encodings).
- encode_ic splits on the first comma, lowercases the op, optionally parses Rt via parse_reg_num, then patches bits[4:0].
- No ARM codegen caller currently emits `ic`; encode_instruction still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_ic_neg_invalid_reg. Closed: every documented behavior has a property; tier round spent.
- Three failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ic_*.md.

## Quirks (encode_ic)

- Surrounding whitespace and ASCII case are accepted (trim + to_lowercase).
- llvm-mc accepts `x31` as XZR; GNU gas rejects `x31`. The differential used llvm-mc.
- parse_reg_num accepts `lr` as 30 (passing); it does not accept `fp` (x29), which gas/llvm-mc do.
- Extra operands after a parsed register (`ivau, x0, x1`) fail parse_reg_num and return Err (sweep passing).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 17-line body.

# Confirmed invariants (encode_bti)

- Valid targets {omitted, c, j, jc} (ASCII case and surrounding space/tab) match llvm-mc `-triple=aarch64 -show-encoding` and the ARM BTI formula 0xD503241F | (j<<7) | (c<<6) (1000 cases). KAT pins omitted/c/j/jc = 0xd503241f/0xd503245f/0xd503249f/0xd50324df.
- ARM BTI layout holds: bits[31:12]=0xD5032, CRm bits[11:8]=0b0100, op2[0]/bit5=0, Rt bits[4:0]=11111 (1000 cases).
- j and c flags are independent: encode("j") XOR encode("") = 1<<7; encode("c") XOR encode("") = 1<<6; encode("jc") = encode("j") XOR encode("c") XOR encode(""); four targets encode distinctly (1000 cases).
- ASCII case-fold and surrounding space/tab are behavior-preserving on the valid domain (1000 cases).
- Unknown targets and extra operands (comma or space) return Err containing "unsupported bti target", matching llvm-mc/gas (1000 cases, strengthened round included space-separated extras and near-miss names).
- Known-answer: `bti` = 0xd503241f; `bti c` = 0xd503245f; `bti j` = 0xd503249f; `bti jc` = 0xd50324df.

## Environment (encode_bti)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM BTI: HINT with CRm=0b0100, op2=0bxx0 (j=op2[2], c=op2[1], op2[0]=0) = 0xD503241F | (j<<7) | (c<<6).
- Dispatch: encoder/mod.rs:972 `"bti" => encode_bti(raw_operands)`. Raw operand string passed through unchanged; parser does not lowercase it.
- Sibling encode_hint / NOP/YIELD/WFE/WFI/SEV/SEVL are not same-job differentials (HINT aliases with a different operand grammar).
- encode_bti trims and lowercases the raw string, then matches four literals; anything else is Err.
- No ARM codegen caller currently emits `bti`; encode_instruction still routes the mnemonic.
- rustdoc at system.rs:541 is a stale HINT copy-paste ("Encode HINT #imm"), not a BTI domain restriction.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Strengthening round 1/1 plus contract-surface sweep 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of omitted/c/j/jc/layout/j-c-bits/case-ws/unknown/extra. Closed: every documented behavior has a property; tier round spent.

## Quirks (encode_bti)

- Surrounding whitespace and ASCII case are accepted (trim + to_lowercase).
- Internal whitespace ("j c", "c x0") is rejected as an unknown target.
- Extra operands after a valid target are rejected (unlike encode_hint, which ignores extras).
- llvm-mc prints BTI as `hint #32/#34/#36/#38`; encoding bytes still match.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 12-line body.

# Confirmed invariants (encode_hint)

- Valid imm 0..=127 matches llvm-mc `-triple=aarch64 -show-encoding` and the ARM HINT formula 0xD503201F | (imm << 5) (1000 cases). Bounds 0 and 127 pinned by the generator and KAT (hint #0/#1/#7/#127).
- ARM HINT layout holds: bits[31:12]=0xD5032, bits[11:5]=imm[6:0], bits[4:0]=11111 (1000 cases).
- Two valid encodings differ only in bits[11:5]; encode(imm) XOR encode(0) = imm << 5 (1000 cases).
- Empty operand slice and non-Imm first operand return Err, matching llvm-mc/gas (1000 cases).
- Known-answer: `hint #0` = 0xd503201f; `hint #1` = 0xd503203f; `hint #7` = 0xd50320ff; `hint #127` = 0xd5032fff.
- Extra operands and Imm outside 0..=127 currently disagree with llvm-mc/gas (see bugs): extras ignored; oob Imm wrapped via CRm=((imm as u32)>>3)&0xF and op2=(imm as u32)&0x7 (`#-1` → `#127`, `#128` → `#0`).

## Environment (encode_hint)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM HINT: 1101 0101 0000 0011 0010 CRm op2 11111 = 0xD503201F | (CRm << 8) | (op2 << 5); imm ∈ 0..=127.
- Dispatch: encoder/mod.rs:967 `"hint" => encode_hint(operands)`. Operands passed through unchanged.
- Sibling NOP/YIELD/WFE/WFI/SEV/SEVL/BTI are not same-job differentials (HINT aliases without a free imm).
- encode_hint uses get_imm on operand 0; extra operands ignored; out-of-range Imm truncated via `as u32` CRm/op2 masks.
- No ARM codegen caller currently emits `hint`; encode() still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of valid-imm/layout/isolation/empty/wrong-kind/extra/oob. Closed: tier round spent.
- Two failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_hint_*.md.

## Quirks (encode_hint)

- Extra operands beyond index 0 are ignored (see bugs).
- Imm outside 0..=127 is masked into CRm:op2 (see bugs).
- get_imm rejects missing and non-Imm operand 0 (passing negative properties).
- llvm-mc prints aliases for some HINT immediates (hint #0 as nop, #1 as yield, …); encoding bytes still match.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 8-line body.

# Confirmed invariants (encode_brk)

- Valid imm16 0..=65535 matches llvm-mc `-triple=aarch64 -show-encoding` and the ARM BRK formula 0xD4200000 | (imm << 5) (1000 cases). Bounds 0 and 65535 pinned by the generator.
- ARM BRK layout holds: bits[31:21]=0b11010100001, bits[20:5]=imm16, bits[4:0]=00000 (1000 cases).
- Two valid encodings differ only in bits[20:5]; encode(imm) XOR encode(0) = imm << 5 (1000 cases).
- Empty operand slice and non-Imm first operand return Err, matching llvm-mc/gas (1000 cases).
- Known-answer: `brk #0` = 0xd4200000; `brk #1` = 0xd4200020; `brk #65535` = 0xd43fffe0.
- Extra operands and Imm outside 0..=65535 currently disagree with llvm-mc/gas (see bugs): extras ignored; oob Imm truncated via `as u32 & 0xFFFF` (`#-1` → `#65535`, `#65536` → `#0`).

## Environment (encode_brk)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM BRK: 1101 0100 001 imm16 00000 = 0xD4200000 | (imm16 << 5); imm16 ∈ 0..=65535.
- Dispatch: encoder/mod.rs:988 `"brk" => encode_brk(operands)`. Operands passed through unchanged.
- Sibling encode_svc/encode_hvc/encode_smc are not same-job differentials (different opcodes).
- encode_brk uses get_imm on operand 0; extra operands ignored; out-of-range Imm truncated via `imm as u32 & 0xFFFF`.
- ARM codegen caller emit.rs:1750 `trap_instruction` emits `"brk #0"`.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of valid-imm/layout/isolation/empty/wrong-kind/extra/oob. Closed: tier round spent.
- Two failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_brk_*.md.

## Quirks (encode_brk)

- Extra operands beyond index 0 are ignored (see bugs).
- Imm outside 0..=65535 is masked into imm16 (see bugs).
- get_imm rejects missing and non-Imm operand 0 (passing negative properties).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 3-line body.

# Confirmed invariants (encode_smc)

- Valid imm16 0..=65535 matches llvm-mc `-triple=aarch64 -show-encoding` and the ARM SMC formula 0xD4000003 | (imm << 5) (1000 cases). Bounds 0 and 65535 pinned by the generator.
- ARM SMC layout holds: bits[31:21]=0b11010100000, bits[20:5]=imm16, bits[4:0]=00011 (1000 cases).
- Two valid encodings differ only in bits[20:5]; encode(imm) XOR encode(0) = imm << 5 (1000 cases).
- Empty operand slice and non-Imm first operand return Err, matching llvm-mc/gas (1000 cases).
- Known-answer: `smc #0` = 0xd4000003; `smc #1` = 0xd4000023; `smc #65535` = 0xd41fffe3.
- Extra operands and Imm outside 0..=65535 currently disagree with llvm-mc/gas (see bugs): extras ignored; oob Imm truncated via `as u32 & 0xFFFF` (`#-1` → `#65535`, `#65536` → `#0`).

## Environment (encode_smc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM SMC: 1101 0100 000 imm16 00011 = 0xD4000003 | (imm16 << 5); imm16 ∈ 0..=65535.
- Dispatch: encoder/mod.rs:985 `"smc" => encode_smc(operands)`. Operands passed through unchanged.
- Sibling encode_svc/encode_hvc/encode_brk are not same-job differentials (different opcodes).
- encode_smc uses get_imm on operand 0; extra operands ignored; out-of-range Imm truncated via `imm as u32 & 0xFFFF`.
- No ARM codegen caller currently emits `smc`; encode() still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of valid-imm/layout/isolation/empty/wrong-kind/extra/oob. Closed: tier round spent.
- Two failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_smc_*.md.

# Confirmed invariants (encode_hvc)

- Valid imm16 0..=65535 matches llvm-mc `-triple=aarch64 -show-encoding` and the ARM HVC formula 0xD4000002 | (imm << 5) (1000 cases). Bounds 0 and 65535 pinned by the generator.
- ARM HVC layout holds: bits[31:21]=0b11010100000, bits[20:5]=imm16, bits[4:0]=00010 (1000 cases).
- Two valid encodings differ only in bits[20:5]; encode(imm) XOR encode(0) = imm << 5 (1000 cases).
- Empty operand slice and non-Imm first operand return Err, matching llvm-mc/gas (1000 cases).
- Known-answer: `hvc #0` = 0xd4000002; `hvc #1` = 0xd4000022; `hvc #65535` = 0xd41fffe2.
- Extra operands and Imm outside 0..=65535 currently disagree with llvm-mc/gas (see bugs): extras ignored; oob Imm truncated via `as u32 & 0xFFFF` (`#-1` → `#65535`, `#65536` → `#0`).

## Environment (encode_hvc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM HVC: 1101 0100 000 imm16 00010 = 0xD4000002 | (imm16 << 5); imm16 ∈ 0..=65535.
- Dispatch: encoder/mod.rs:980 `"hvc" => encode_hvc(operands)`. Operands passed through unchanged.
- Sibling encode_svc/encode_smc/encode_brk are not same-job differentials (different opcodes).
- encode_hvc uses get_imm on operand 0; extra operands ignored; out-of-range Imm truncated via `imm as u32 & 0xFFFF`.
- No ARM codegen caller currently emits `hvc`; encode() still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of valid-imm/layout/isolation/empty/wrong-kind/extra/oob. Closed: tier round spent.
- Two failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_hvc_*.md.

## Quirks (encode_hvc)

- Extra operands beyond index 0 are ignored (see bugs).
- Imm outside 0..=65535 is masked into imm16 (see bugs).
- get_imm rejects missing and non-Imm operand 0 (passing negative properties).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 3-line body.

# Confirmed invariants (encode_svc)

- Valid imm16 0..=65535 matches llvm-mc `-triple=aarch64 -show-encoding` and the ARM SVC formula 0xD4000001 | (imm << 5) (1000 cases). Bounds 0 and 65535 pinned by the generator.
- ARM SVC layout holds: bits[31:21]=0b11010100000, bits[20:5]=imm16, bits[4:0]=00001 (1000 cases).
- Two valid encodings differ only in bits[20:5]; encode(imm) XOR encode(0) = imm << 5 (1000 cases).
- Empty operand slice and non-Imm first operand return Err, matching llvm-mc/gas (1000 cases).
- Known-answer: `svc #0` = 0xd4000001; `svc #1` = 0xd4000021; `svc #65535` = 0xd41fffe1.
- Extra operands and Imm outside 0..=65535 currently disagree with llvm-mc/gas (see bugs): extras ignored; oob Imm truncated via `as u32 & 0xFFFF` (`#-1` → `#65535`, `#65536` → `#0`).

## Environment (encode_svc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM SVC: 1101 0100 000 imm16 00001 = 0xD4000001 | (imm16 << 5); imm16 ∈ 0..=65535.
- Dispatch: encoder/mod.rs:977 `"svc" => encode_svc(operands)`. Operands passed through unchanged.
- Sibling encode_hvc/encode_smc/encode_brk are not same-job differentials (different opcodes).
- encode_svc uses get_imm on operand 0; extra operands ignored; out-of-range Imm truncated via `imm as u32 & 0xFFFF`.
- No ARM codegen caller currently emits `svc`; encode() still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of valid-imm/layout/isolation/empty/wrong-kind/extra/oob. Closed: tier round spent.
- Two failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_svc_*.md.

## Quirks (encode_svc)

- Extra operands beyond index 0 are ignored (see bugs).
- Imm outside 0..=65535 is masked into imm16 (see bugs).
- get_imm rejects missing and non-Imm operand 0 (passing negative properties).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 3-line body.

# Confirmed invariants (encode_msr)

- Generic S-form with in-range fields (op0 0..=3, op1 0..=7, CRn/CRm 0..=15, op2 0..=7) × Xt matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc word equals the unmasked ARM MSR formula 0xD5000000 | (enc << 5) | Rt.
- Numbered families dbgbcr/dbgbvr/dbgwcr/dbgwvr n∈0..=15 and pmevcntr/pmevtyper n∈0..=30 match llvm-mc (1000 cases).
- PSTATE immediate daifset/daifclr/spsel × imm 0..=15 matches llvm-mc (1000 cases).
- ARM MSR layout holds for named sysregs: bits[31:21]=0b11010101000 (group + L=0); bits[4:0]=Rt; different Xt differ only in bits[4:0] (1000 cases).
- ASCII case-fold of a named sysreg is encoding-invariant (1000 cases).
- Known-answer: `msr tpidr_el0, x0` = 0xd51bd040; `msr nzcv, x0` = 0xd51b4200; `msr nzcv, xzr` = 0xd51b421f; `msr daifset, #2` = 0xd50342df; `msr spsel, #1` = 0xd50041bf; `msr spsel, x0` = 0xd5184200; llvm-mc `msr cntv_cval_el0, x0` = 0xd51be340 (SUT disagrees).
- Named table vs llvm-mc currently disagrees on oslsr_el1 (read-only, SUT encodes) and cntv_cval_el0 (wrong op2). Extra operands, Wt/SP/FP Xt, out-of-range S-form, and PSTATE imm outside 0..=15 currently disagree (see bugs).

## Environment (encode_msr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM MSR (register): 1101 0101 00 0 op0 op1 CRn CRm op2 Rt = 0xD5000000 | (sysreg << 5) | Rt; Xt is a 64-bit GPR (not SP).
- ARM ARM MSR (immediate): 1101 0101 0000 0 op1 0100 CRm op2 11111; daifset op1=3 op2=6; daifclr op1=3 op2=7; spsel op1=0 op2=5; CRm = imm ∈ 0..=15.
- Dispatch: encoder/mod.rs:974 `"msr" => encode_msr(operands)`. Operands passed through unchanged.
- Sibling encode_mrs is not a same-job differential (MRS L=1 read, reversed operands, no PSTATE immediate).
- encode_msr requires operand 0 Symbol; uses get_imm for daifset/daifclr (and spsel when Imm); uses get_reg on operand 1 for the register form (discards is_64); extra operands ignored; unknown names fall through to parse_generic_sysreg / parse_numbered_sysreg; sysreg_encoding masks out-of-range fields; PSTATE imm is masked `& 0xF`.
- No ARM codegen caller currently emits `msr`; encode() still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of named/generic/numbered/imm/layout/case-fold/extra/wrong-src/unknown/arity/oob. Closed: tier round spent.
- Six failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_msr_*.md.

# Confirmed invariants (encode_mrs)

- Generic S-form with in-range fields (op0 0..=3, op1 0..=7, CRn/CRm 0..=15, op2 0..=7) × Xt matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc word equals the unmasked ARM MRS formula 0xD5200000 | (enc << 5) | Rt.
- Numbered families dbgbcr/dbgbvr/dbgwcr/dbgwvr n∈0..=15 and pmevcntr/pmevtyper n∈0..=30 match llvm-mc (1000 cases).
- ARM MRS layout holds for named sysregs: bits[31:21]=0b11010101001 (group + L=1); bits[4:0]=Rt; different Xt differ only in bits[4:0] (1000 cases).
- ASCII case-fold of a named sysreg is encoding-invariant (1000 cases).
- Known-answer: `mrs x0, tpidr_el0` = 0xd53bd040; `mrs x0, nzcv` = 0xd53b4200; `mrs xzr, nzcv` = 0xd53b421f; llvm-mc `mrs x0, cntv_cval_el0` = 0xd53be340 (SUT disagrees).
- Named table vs llvm-mc currently disagrees on oslar_el1 (write-only, SUT encodes) and cntv_cval_el0 (wrong op2). Extra operands, Wt/SP/FP dest, and out-of-range S-form currently disagree (see bugs).

## Environment (encode_mrs)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM MRS: 1101 0101 00 1 op0 op1 CRn CRm op2 Rt = 0xD5200000 | (sysreg << 5) | Rt; Xt is a 64-bit GPR (not SP).
- Dispatch: encoder/mod.rs:971 `"mrs" => encode_mrs(operands)`. Operands passed through unchanged.
- Sibling encode_msr is not a same-job differential (MSR L=0 write, reversed operands, immediate PSTATE form).
- encode_mrs uses get_reg on operand 0 (discards is_64) and requires operand 1 to be Symbol; extra operands ignored; unknown names fall through to parse_generic_sysreg / parse_numbered_sysreg; sysreg_encoding masks out-of-range fields.
- Callers: codegen/globals.rs:25 and codegen/intrinsics.rs:241 emit `mrs x0, tpidr_el0`.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of named/generic/numbered/layout/case-fold/extra/wrong-dest/oob. Closed: tier round spent.
- Five failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_mrs_*.md.

# Confirmed invariants (encode_dsb)

- Named DSB options (sy/st/ld/ish/ishst/ishld/nsh/nshst/nshld/osh/oshst/oshld), including case folds, match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Barrier vs Symbol encodings are identical.
- ARM DSB layout holds for named options: 0xD503309F | (CRm << 8); bits[31:12]=0xD5033; bits[7:5]=100; bits[4:0]=11111; different names differ only in bits[11:8] (1000 cases).
- Unknown Barrier/Symbol names return Err containing "unknown dsb option" (1000 cases).
- Known-answer: `dsb sy` = 0xd5033f9f; `dsb ish` = 0xd5033b9f; llvm-mc `dsb #0` = 0xd503309f (SUT disagrees).
- Imm 0..=15, extra operands, empty slice, and non-barrier kinds currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_dsb)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). Cross-checked against GNU as (aarch64-linux-gnu-as).
- ARM ARM DSB: 1101 0101 0000 0011 0011 CRm 100 11111. Named CRm as above; assembler also accepts #imm 0..=15 as CRm.
- Dispatch: encoder/mod.rs:967 `"dsb" => encode_dsb(operands)`. Operands passed through unchanged.
- Sibling encode_dmb is not a same-job differential (DMB op2=101 vs DSB op2=100).
- encode_dsb matches only first-operand Barrier/Symbol names; unknown names Err; any other first-operand kind (including Imm and empty) silently encodes SY; extra operands ignored.
- Parser: Barrier for the 12 names (parser.rs:1918-1923); Imm for `#n` and bare integers (parser.rs:1991, 2020).
- No codegen caller currently emits `dsb`; encode() still routes the mnemonic.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of named Barrier/Symbol, unknown Err, and `_ => 0b1111`.
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_dsb_*.md.

# Confirmed invariants (encode_dmb)

- Named DMB options (sy/st/ld/ish/ishst/ishld/nsh/nshst/nshld/osh/oshst/oshld), including case folds, match llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Barrier vs Symbol encodings are identical.
- ARM DMB layout holds for named options: 0xD50330BF | (CRm << 8); bits[31:12]=0xD5033; bits[7:5]=101; bits[4:0]=11111; different names differ only in bits[11:8] (1000 cases).
- Unknown Barrier/Symbol names return Err containing "unknown dmb option" (1000 cases).
- Known-answer: `dmb sy` = 0xd5033fbf; `dmb ish` = 0xd5033bbf; llvm-mc `dmb #0` = 0xd50330bf (SUT disagrees).
- Imm 0..=15, extra operands, empty slice, and non-barrier kinds currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_dmb)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). Cross-checked against GNU as (aarch64-linux-gnu-as).
- ARM ARM DMB: 1101 0101 0000 0011 0011 CRm 101 11111. Named CRm as above; assembler also accepts #imm 0..=15 as CRm.
- Dispatch: encoder/mod.rs:964 `"dmb" => encode_dmb(operands)`. Operands passed through unchanged.
- Sibling encode_dsb is not a same-job differential (DSB op2=100 vs DMB op2=101).
- encode_dmb matches only first-operand Barrier/Symbol names; unknown names Err; any other first-operand kind (including Imm and empty) silently encodes SY; extra operands ignored.
- Parser: Barrier for the 12 names (parser.rs:1918-1923); Imm for `#n` and bare integers (parser.rs:1991, 2020).
- Callers: codegen/atomics.rs:141-143 `dmb ishld`/`ishst`/`ish`; codegen/intrinsics.rs:78,81 `dmb ish`/`ishst`.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of named Barrier/Symbol, unknown Err, and `_ => 0b1111`.
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_dmb_*.md.

# Confirmed invariants (encode_neon_two_misc_narrow)

- Valid XTN/SQXTN/UQXTN/SQXTUN with matching Tb/Ta and 2-suffix Q variants matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). ARM mapping: XTN U=0 opcode=10010; SQXTN U=0 opcode=10100; UQXTN U=1 opcode=10100; SQXTUN U=1 opcode=10010; Ta in {8H,4S,2D}; Tb {8B/16B, 4H/8H, 2S/4S} from Q.
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only is_high in bit 30; only U in bit 29; only opcode in bits[16:12] (1000 cases).
- Success-path word: bit31=0, Q at 30, U at 29, bits[28:24]=01110, size at [23:22] from Ta (8h=00, 4s=01, 2d=10), bits[21:17]=10000, opcode at [16:12], bits[11:10]=10, Rn at [9:5], Rd at [4:0] (1000 cases).
- Arity 0–1 returns Err (1000 cases).
- Imm/Mem/Label in dest or src slot returns Err (1000 cases, sweep).
- Uppercase mnemonic and V-prefix registers match llvm-mc (1000 cases).
- Known-answer (llvm-mc connection): `xtn v0.8b, v1.8h` = 0x0e212820; `xtn2 v0.16b, v1.8h` = 0x4e212820; `xtn v0.4h, v1.4s` = 0x0e612820; `xtn v0.2s, v1.2d` = 0x0ea12820; `xtn2 v0.4s, v1.2d` = 0x4ea12820; `sqxtn v0.8b, v1.8h` = 0x0e214820; `uqxtn v0.8b, v1.8h` = 0x2e214820; `sqxtun v0.8b, v1.8h` = 0x2e212820; `xtn v31.8b, v31.8h` = 0x0e212bff; `xtn v0.8b, v0.8h` = 0x0e212800; `xtn v15.4h, v16.4s` = 0x0e612a0f.
- Extra operand, dest-arrangement mismatch, and bare/GPR dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_two_misc_narrow)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD two-register miscellaneous (asimdmisc): 0 Q U 01110 size 10000 opcode 10 Rn Rd.
- Dispatch: encoder/mod.rs:676-677 sqxtun/sqxtun2; mod.rs:941-946 uqxtn/uqxtn2/sqxtn/sqxtn2/xtn/xtn2. Operands passed through unchanged.
- Sibling encode_neon_two_misc / encode_neon_fcvtn / encode_neon_xtl are not same-job differentials.
- encode_neon_two_misc_narrow checks operands.len() < 2 only; dest arrangement discarded (`_arr_d`); size from source 8h/4s/2d; Q from is_high; get_neon_reg accepts Operand::Reg.
- parse_reg_num lowercases prefixes; maps sp/wsp/xzr/wzr to 31, lr to 30.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus neg_nonreg (passing).
- Three failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_two_misc_narrow_*.md.

# Confirmed invariants (encode_neon_scalar_qshrn)

- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only U in bit 29; only is_rounding in bits[15:10]; only dest/shift in bits[23:16] (1000 cases).
- Arity 0–2 returns Err (1000 cases).
- Shift 0, dest_esize+1, -1, 64, 255 returns Err (1000 cases).
- Dest prefix x/w/q/v/d returns Err (1000 cases, sweep).
- Imm/Mem/Label in dest or src slot returns Err (1000 cases, sweep).
- Known-answer (llvm-mc connection): `sqshrn h0, s1, #1` = 0x5f1f9420; `sqshrn b0, h1, #1` = 0x5f0f9420; `sqshrn s0, d1, #1` = 0x5f3f9420; `sqshrn h0, s1, #16` = 0x5f109420; `sqshrn b0, h1, #8` = 0x5f089420; `sqshrn s0, d1, #32` = 0x5f209420; `sqrshrn h0, s1, #1` = 0x5f1f9c20; `uqshrn h0, s1, #1` = 0x7f1f9420; `uqrshrn h0, s1, #1` = 0x7f1f9c20; `sqshrn b31, h31, #8` = 0x5f0897ff; `sqshrn s15, d16, #17` = 0x5f2f960f; `SQSHRN H0, S1, #1` = 0x5f1f9420.
- Valid-domain encoding currently disagrees with llvm-mc/gas (bit 28); extra operand and dest/src class mismatch currently disagree (see bugs).

## Environment (encode_neon_scalar_qshrn)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD scalar shift by immediate (asisdshf): 01 U 11111 immh:immb opcode Rn Rd; dest/src B<-H / H<-S / S<-D; shift in 1..=dest_esize; SQSHRN U=0 opcode=100101; SQRSHRN U=0 opcode=100111; UQSHRN U=1 opcode=100101; UQRSHRN U=1 opcode=100111.
- Dispatch: encoder/mod.rs:725-728 sqshrn => encode_neon_scalar_qshrn when dest is Operand::Reg, else encode_neon_qshrn (vector). uqshrn/sqrshrn/uqrshrn always go to encode_neon_qshrn.
- Sibling encode_neon_qshrn is not a same-job differential (vector Vd.Tb).
- encode_neon_scalar_qshrn checks operands.len() < 3 only; extra ignored; dest size from starts_with b/h/s (sp starts with s); source is any Operand::Reg accepted by parse_reg_num.
- parse_reg_num lowercases prefixes; maps sp/wsp/xzr/wzr to 31, lr to 30.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus neg_unsupported_dest / neg_nonreg (passing).
- Three failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_scalar_qshrn_*.md.

# Confirmed invariants (encode_neon_scalar_two_misc)

- Valid SQABS/SQNEG with matching B/H/S/D registers (b0–b31 / h / s / d) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). ARM mapping: SQABS U=0 opcode=00111; SQNEG U=1 opcode=00111.
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only U in bit 29; only opcode in bits[16:12]; only dest prefix in size bits[23:22] (1000 cases).
- Success-path word: bits[31:30]=01, U at 29, bits[28:24]=11110, size at [23:22] from dest prefix (b=00, h=01, s=10, d=11), bits[21:17]=10000, opcode at [16:12], bits[11:10]=10, Rn at [9:5], Rd at [4:0] (1000 cases).
- Arity 0–1 returns Err (1000 cases).
- Imm/Mem/Label in either slot returns Err (1000 cases).
- Dest prefix x/w/q/v returns Err (1000 cases, sweep).
- Uppercase B/H/S/D prefix and SQABS/SQNEG mnemonic match llvm-mc (1000 cases).
- Known-answer: `sqabs d0, d1` = 0x5ee07820; `sqneg d0, d1` = 0x7ee07820; `sqabs s0, s1` = 0x5ea07820; `sqabs h0, h1` = 0x5e607820; `sqabs b0, b1` = 0x5e207820; `sqabs d31, d31` = 0x5ee07bff; `sqneg b0, b1` = 0x7e207820; `SQABS D0, D1` = 0x5ee07820; `sqabs d15, d16` = 0x5ee07a0f; `sqneg s31, s0` = 0x7ea0781f.
- Extra operand and dest/src class mismatch currently disagree with llvm-mc/gas (see bugs). Dest `sp` is treated as Sd.

## Environment (encode_neon_scalar_two_misc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD scalar two-register miscellaneous: 01 U 11110 size 10000 opcode 10 Rn Rd; dest/src same B/H/S/D; SQABS U=0 opcode=00111; SQNEG U=1 opcode=00111.
- Dispatch: encoder/mod.rs:674-682 sqabs/sqneg => encode_neon_scalar_two_misc when dest is Operand::Reg, else encode_neon_two_misc (vector). Dispatcher currently passes SQNEG as U=0 opcode=01000 — properties feed the ARM/llvm-mc mapping into this function.
- Sibling encode_neon_two_misc is not a same-job differential (vector Vd.T).
- encode_neon_scalar_two_misc checks operands.len() < 2 only; extra ignored; dest size from starts_with b/h/s/d (sp starts with s); source is any Operand::Reg accepted by parse_reg_num.
- parse_reg_num lowercases prefixes; maps sp/wsp/xzr/wzr to 31, lr to 30.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus neg_unsupported_dest (passing).
- Two failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_scalar_two_misc_*.md.

# Confirmed invariants (encode_neon_scalar_addp)

- Valid ADDP Dd, Vn.2d with d0–d31 / v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5] (1000 cases).
- Success-path word: bits[31:30]=01, bit29=0, bits[28:24]=11110, bits[23:22]=11, bits[21:17]=11000, bits[16:12]=11011, bits[11:10]=10, Rn at [9:5], Rd at [4:0] (1000 cases). Template 0x5ef1b800 | (rn<<5) | rd.
- Arity 0–1 returns Err (1000 cases).
- Arrangement ≠ 2d returns Err (1000 cases).
- Imm/Mem/Label in either slot returns Err (1000 cases).
- Uppercase D/V prefix and ADDP mnemonic match llvm-mc (1000 cases).
- Known-answer: `addp d0, v1.2d` = 0x5ef1b820; `addp d31, v31.2d` = 0x5ef1bbff; `addp d0, v0.2d` = 0x5ef1b800; `addp d15, v16.2d` = 0x5ef1ba0f; `addp D0, V1.2D` = 0x5ef1b820; `ADDP d0, v1.2d` = 0x5ef1b820.
- Extra operand and non-D dest / non-V source currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_scalar_addp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD scalar pairwise ADDP: 01 0 11110 11 11000 11011 10 Rn Rd; dest Dd only; source Vn.2D only.
- Dispatch: encoder/mod.rs:641-646 addp => encode_neon_scalar_addp when operands.len()==2 and dest starts with d/D, else encode_neon_three_same (vector ADDP).
- Sibling encode_neon_three_same / encode_neon_faddp are not same-job differentials.
- encode_neon_scalar_addp checks operands.len() < 2 only; extra ignored; dest any Operand::Reg accepted by parse_reg_num; source arrangement must be "2d" but source prefix is not checked.
- parse_reg_num lowercases prefixes; parser lowercases arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus diff_alt_spellings.
- Two failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_scalar_addp_*.md.

# Confirmed invariants (encode_neon_scalar_three_same)

- Valid ADD/SUB Dd, Dn, Dm with d0–d31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16]; add vs sub differs only in U bit 29 (1000 cases).
- Success-path word: bits[31:30]=01, U at 29, bits[28:24]=11110, size at [23:22], bit21=1, Rm at [20:16], opcode at [15:11], bit10=1, Rn at [9:5], Rd at [4:0] for opcode 0..31 and size 0..3 (1000 cases).
- Arity 0–2 returns Err (1000 cases).
- Imm/Mem/Label in any slot returns Err (1000 cases).
- Uppercase D prefix and uppercase ADD/SUB match llvm-mc (1000 cases).
- Known-answer: `add d0, d1, d2` = 0x5ee28420; `sub d0, d1, d2` = 0x7ee28420; `add d31, d31, d31` = 0x5eff87ff; `add d0, d0, d0` = 0x5ee08400; `add d15, d16, d17` = 0x5ef1860f; `sub d31, d0, d1` = 0x7ee1841f.
- Extra operand and non-D source currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_scalar_three_same)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD scalar three-same ADD/SUB: 01 U 11110 size 1 Rm opcode 1 Rn Rd; size=11 (D only); opcode=10000; U=0 ADD, U=1 SUB.
- Dispatch: encoder/mod.rs:305-310 add/sub => encode_neon_scalar_three_same when is_neon_scalar_d_reg_op.
- Sibling encode_neon_three_same / encode_neon_add_sub are vector forms, not same-job differentials.
- encode_neon_scalar_three_same checks operands.len() < 3 only; extra ignored; any Operand::Reg accepted by parse_reg_num.
- parse_reg_num lowercases prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus diff_alt_spellings.
- Two failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_scalar_three_same_*.md.

# Confirmed invariants (encode_neon_faddp)

- Valid vector FADDP Vd.T, Vn.T, Vm.T with T in {2s,4s,2d}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid scalar FADDP Sd, Vn.2S and FADDP Dd, Vn.2D matches llvm-mc (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path vector word: bit31=0, Q/sz from dest T (2s=0/0, 4s=1/0, 2d=1/1), U=1, bits[28:24]=01110, bit21=1, Rm at [20:16], bits[15:10]=110101, Rn at [9:5], Rd at [4:0].
- Success-path scalar word: bits[31:24]=01111110, sz at bit22, bits[21:17]=11000, bits[16:12]=01101, bits[11:10]=10, Rn at [9:5], Rd at [4:0].
- Invalid T (8b/16b/4h/8h/1d/…) returns Err (1000 cases).
- Arity 0–1 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Known-answer: `faddp v0.2s, v1.2s, v2.2s` = 0x2e22d420; `faddp v0.4s, v1.4s, v2.4s` = 0x6e22d420; `faddp v0.2d, v1.2d, v2.2d` = 0x6e62d420; `faddp s0, v1.2s` = 0x7e30d820; `faddp d0, v1.2d` = 0x7e70d820; `faddp v31.2s, v31.2s, v31.2s` = 0x2e3fd7ff.
- Extra operand, mismatched T, and scalar dest-size mismatch currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_faddp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same FADDP: 0 Q 1 01110 0 sz 1 Rm 110101 Rn Rd. T in {2S,4S,2D}. Q/sz = 0/0 2S, 1/0 4S, 1/1 2D.
- ARM SISD FADDP: 01 1 11110 0 sz 11000 01101 10 Rn Rd. Sd+Vn.2S (sz=0) or Dd+Vn.2D (sz=1).
- Dispatch: encoder/mod.rs:758 `"faddp" => encode_neon_faddp(operands)`.
- Sibling encode_neon_float_three_same / encode_neon_scalar_addp are different opcodes, not same-job differentials.
- encode_neon_faddp uses operands.len() >= 3 for vector (no max); source arrangements discarded; scalar dest is any Operand::Reg accepted by parse_reg_num.
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (uppercase V alt-spellings).
- Three failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_faddp_*.md.

# Confirmed invariants (encode_neon_bitwise_insert)

- Valid BIT/BIF Vd.T, Vn.T, Vm.T with T in {8b,16b}, size in {0b10,0b11}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16]; only size in bits[23:22] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T=16b, U=1, bits[28:24]=01110, size bits[23:22]=10 BIT / 11 BIF, bit21=1, Rm at [20:16], bits[15:10]=000111, Rn at [9:5], Rd at [4:0]. 8b vs 16b differs only in Q.
- Arity 0–2 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Known-answer: `bit v0.8b, v1.8b, v2.8b` = 0x2ea21c20; `bif v0.8b, v1.8b, v2.8b` = 0x2ee21c20; `bit v0.16b, v1.16b, v2.16b` = 0x6ea21c20; `bif v31.16b, v0.16b, v1.16b` = 0x6ee11c1f.
- Extra operand, T∉{8b,16b}, mismatched T, and GPR/SP/bare-V/FP currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_bitwise_insert)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same (BIT/BIF): 0 Q 1 01110 ss 1 Rm 000111 Rn Rd. T in {8B,16B} only. size=10 BIT, size=11 BIF. Q=1 iff T=16B.
- Dispatch: encoder/mod.rs:754-755 `"bit" => encode_neon_bitwise_insert(operands, 0b10); "bif" => encode_neon_bitwise_insert(operands, 0b11)`.
- Sibling encode_neon_bsl / encode_neon_bic are different opcodes, not same-job differentials.
- encode_neon_bitwise_insert checks operands.len() < 3; extra ignored; source arrangements discarded; Q=1 iff arr_d=="16b" else 0 (no 8b check).
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (uppercase V alt-spellings).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_bitwise_insert_*.md.

# Confirmed invariants (encode_neon_fcvtn)

- Valid FCVTN/FCVTN2 Vd.{4h,8h,2s,4s}, Vn.{4s,2d} with ARM-correct (Tb,Ta,Q) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: bit31=0, Q at 30 (is_high), U=0, bits[28:24]=01110, bit23=0, sz at 22 (0 for .4s, 1 for .2d), bits[21:17]=10000, bits[16:12]=10110, bits[11:10]=10, Rn at [9:5], Rd at [4:0] (1000 cases).
- Rd/Rn isolation in bits[4:0]/[9:5]; Q isolation at bit 30 (1000 cases).
- Arity 0–1 always Err (1000 cases).
- Imm/Mem/Label in either slot always Err (1000 cases).
- Uppercase V prefix matches llvm-mc (1000 cases).
- Known-answer: `fcvtn v0.4h, v1.4s` = 0x0e216820; `fcvtn2 v0.8h, v1.4s` = 0x4e216820; `fcvtn v0.2s, v1.2d` = 0x0e616820; `fcvtn2 v0.4s, v1.2d` = 0x4e616820.

## Environment (encode_neon_fcvtn)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD two-register miscellaneous FCVTN{2}: Tb=4H Ta=4S (Q=0) or 8H (Q=1); Tb=2S Ta=2D (Q=0) or 4S (Q=1); U=0; opcode=10110; sz=0 single→half, sz=1 double→single.
- Dispatch: encoder/mod.rs:539-540 fcvtn/fcvtn2 => encode_neon_fcvtn(operands, is_high).
- encode_neon_fcvtn has no arity maximum; dest arrangement discarded; source "2s" accepted as sz=0; get_neon_reg accepts Operand::Reg and x/w prefixes on RegArrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: neg_nonreg passing.
- Three SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_fcvtn_*.md.

## Quirks (encode_neon_fcvtn)

- Extra operands beyond index 1 are ignored (see bugs).
- Dest arrangement is discarded (see bugs).
- Source arrangement 2s encodes as sz=0 (see bugs).
- Bare Operand::Reg dest encodes as Vd with sz from source (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus neg_nonreg.

# Confirmed invariants (encode_neon_fcvtl)

- Valid FCVTL/FCVTL2 Vd.{4s,2d}, Vn.{4h,8h,2s,4s} with ARM-correct (Ta,Tb,Q) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: bit31=0, Q at 30 (is_high), U=0, bits[28:24]=01110, bit23=0, sz at 22 (0 for .4s, 1 for .2d), bits[21:17]=10000, bits[16:12]=10111, bits[11:10]=10, Rn at [9:5], Rd at [4:0] (1000 cases).
- Rd/Rn isolation in bits[4:0]/[9:5]; Q isolation at bit 30 (1000 cases).
- Arity 0–1 always Err (1000 cases).
- Imm/Mem/Label in either slot always Err (1000 cases).
- Uppercase V prefix matches llvm-mc (1000 cases).
- Known-answer: `fcvtl v0.4s, v1.4h` = 0x0e217820; `fcvtl2 v0.4s, v1.8h` = 0x4e217820; `fcvtl v0.2d, v1.2s` = 0x0e617820; `fcvtl2 v0.2d, v1.4s` = 0x4e617820.

## Environment (encode_neon_fcvtl)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD two-register miscellaneous FCVTL{2}: Ta=4S Tb=4H (Q=0) or 8H (Q=1); Ta=2D Tb=2S (Q=0) or 4S (Q=1); U=0; opcode=10111; sz=0 half→single, sz=1 single→double.
- Dispatch: encoder/mod.rs:535-536 fcvtl/fcvtl2 => encode_neon_fcvtl(operands, is_high).
- encode_neon_fcvtl has no arity maximum; source arrangement discarded; dest "2s" accepted as sz=0; get_neon_reg accepts Operand::Reg and x/w prefixes on RegArrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: neg_nonreg passing.
- Three SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_fcvtl_*.md.

## Quirks (encode_neon_fcvtl)

- Extra operands beyond index 1 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Dest arrangement 2s encodes as sz=0 (see bugs).
- X-prefixed RegArrangement dest encodes as V (see bugs).
- Bare Operand::Reg dest returns Err via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus neg_nonreg.

# Confirmed invariants (encode_neon_float_elem)

- Rd/Rn isolation in bits[4:0]/[9:5]; U isolation at bit 29 (1000 cases).
- Arity 0–2 always Err (1000 cases).
- Dest arrangements 8b/16b/4h/8h/1d/4b/empty always Err (1000 cases).
- Uppercase V prefix encodes the same word as lowercase v (1000 cases).
- llvm-mc mapping KAT: `fmul v0.2s, v1.2s, v2.s[0]` = 0x0f829020.
- Valid-domain llvm-mc agreement currently fails: SUT size field is 00/01 instead of ARM 10/11 (see bugs).

## Environment (encode_neon_float_elem)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD vector x indexed element (FP): T in {2S,4S,2D}; size=10 (S) index H:L 0..3, M=Rm[4]; size=11 (D) index H 0..1, L=0, M=Rm[4]; Rm v0-v31. FMUL U=0 opcode=1001; FMLA U=0 opcode=0001; FMLS U=0 opcode=0101; FMULX U=1 opcode=1001.
- Dispatch: encoder/mod.rs:456-459 fmul RegLane => encode_neon_float_elem(operands, 1, 0b1001) (dispatcher U=1 is FMULX's U; properties call the helper with ARM-correct U=0); encoder/mod.rs:534-545 fmla/fmls.
- encode_neon_float_elem checks operands.len() < 3 only; source arrangement discarded; lane elem_size discarded; no index range check; sz shifted to bit 22 only; get_neon_reg accepts Operand::Reg and x/w prefixes on RegArrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: unsupported-t/alt-spellings passing, lane-elem failing.
- Six SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_float_elem_*.md.

## Quirks (encode_neon_float_elem)

- Size bit 23 is clear (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- X-prefixed RegArrangement dest encodes as V (see bugs).
- Out-of-range index wraps via H:L bits (see bugs).
- Lane elem_size is ignored (see bugs).
- Bare Operand::Reg dest returns Err via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus unsupported-t/alt-spellings/lane-elem.

# Confirmed invariants (encode_neon_elem)

- Valid MUL/MLA/MLS/SQDMULH/SQRDMULH Vd.T, Vn.T, Vm.Ts[idx] with T in {4h,8h,2s,4s}, ARM-correct (U, opcode) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: bit31=0, Q at 30, U at 29, bits[28:24]=01111, size at [23:22] (01 for .h, 10 for .s), L at 21, M at 20, Rm[3:0] at [19:16], opcode at [15:12], H at 11, bit10=0, Rn at [9:5], Rd at [4:0] (1000 cases).
- Rd/Rn isolation in bits[4:0]/[9:5]; U isolation at bit 29 (1000 cases).
- Arity 0–2 always Err (1000 cases).
- Dest arrangements 8b/16b/2d/1d/4b/empty always Err (1000 cases).
- Uppercase V prefix matches llvm-mc (1000 cases).
- Known-answer: `mul v0.4h, v1.4h, v2.h[2]` = 0x0f628020; `mla v0.8h, v1.8h, v15.h[7]` = 0x6f7f0820; `mls v0.2s, v1.2s, v31.s[3]` = 0x2fbf4820; `sqdmulh v0.4s, v1.4s, v2.s[1]` = 0x4fa2c020; `sqrdmulh v31.4h, v30.4h, v0.h[0]` = 0x0f40d3df.

## Environment (encode_neon_elem)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- ARM ARM Advanced SIMD vector x indexed element (non-long): T in {4H,8H,2S,4S}; size=00/11 reserved; size=01 Rm v0-v15 index H:L:M 0..7; size=10 Rm v0-v31 (M=Rm[4]) index H:L 0..3. MUL U=0 opcode=1000; MLA U=1 opcode=0000; MLS U=1 opcode=0100; SQDMULH U=0 opcode=1100; SQRDMULH U=0 opcode=1101.
- Dispatch: encoder/mod.rs:307-310 mul RegLane; encoder/mod.rs:782-787 sqdmulh/sqrdmulh; encoder/mod.rs:793-796 mla/mls (dispatcher currently passes u_bit=0 for MLA/MLS; properties call encode_neon_elem with ARM-correct U).
- encode_neon_elem checks operands.len() < 3 only; source arrangement discarded; lane elem_size discarded; no index range check; half-word Rm is `rm & 0xF`; get_neon_reg accepts Operand::Reg (empty arrangement then fails neon_arr_to_q_size) and x/w prefixes on RegArrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: unsupported-t/alt-spellings passing, index-oob/lane-elem failing.
- Six SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_elem_*.md.

## Quirks (encode_neon_elem)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- H-lane Rm v16-v31 is truncated to v0-v15 (see bugs).
- X-prefixed RegArrangement dest encodes as V (see bugs).
- Out-of-range index wraps via H:L:M bits (see bugs).
- Lane elem_size is ignored (see bugs).
- Bare Operand::Reg dest returns Err via empty arrangement (unlike encode_neon_elem_long).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus unsupported-t/alt-spellings/index-oob/lane-elem.

# Confirmed invariants (encode_neon_elem_long)

- Valid SMULL/UMULL/SMLAL/UMLAL/SMLSL/UMLSL/SQDMULL/SQDMLAL/SQDMLSL (+ `2`) Vd.{4s,2d}, Vn.{4h,8h,2s,4s}, Vm.{h,s}[idx] with ARM-correct (U, opcode, is_high) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: bit31=0, Q at 30 (is_high), U at 29, bits[28:24]=01111, size at [23:22] (01 for .h, 10 for .s), L at 21, M at 20, Rm[3:0] at [19:16], opcode at [15:12], H at 11, bit10=0, Rn at [9:5], Rd at [4:0] (1000 cases).
- Rd/Rn isolation in bits[4:0]/[9:5]; U isolation at bit 29 (1000 cases).
- Arity 0–2 always Err (1000 cases).
- Index > 7 for .h and > 3 for .s always Err (1000 cases).
- Source arrangements other than 4h/8h/2s/4s always Err (1000 cases).
- Uppercase V prefix matches llvm-mc (1000 cases).
- Known-answer: `smull v0.4s, v1.4h, v2.h[2]` = 0x0f62a020; `smull2 v0.4s, v1.8h, v2.h[7]` = 0x4f72a820; `smull v0.2d, v1.2s, v31.s[3]` = 0x0fbfa820.

## Environment (encode_neon_elem_long)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD vector x indexed element (long): size=01 Rm v0-v15 index H:L:M 0..7; size=10 Rm v0-v31 (M=Rm[4]) index H:L 0..3; dest 4h/8h→4s, 2s/4s→2d.
- Dispatch: encoder/mod.rs:318-329 smull/umull RegLane; encoder/mod.rs:749-775 sqdmlal/sqdmlsl/sqdmull (+2); encoder/mod.rs:837-890 umlal/smlal/umlsl/smlsl/umull2/smull2.
- encode_neon_elem_long checks operands.len() < 3 only; dest `_arr_d` discarded; lane `elem_size` discarded; half-word Rm is `rm & 0xF`; get_neon_reg accepts Operand::Reg and x/w/s/sp prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: index-oob/unsupported-src/alt-spellings passing, lane-elem-mismatch failing.
- Five SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_elem_long_*.md.

## Quirks (encode_neon_elem_long)

- Extra operands beyond index 2 are ignored (see bugs).
- Destination arrangement is discarded (see bugs).
- H-lane Rm v16-v31 is truncated to v0-v15 (see bugs).
- Operand::Reg and x/w/s/sp prefixes encode as V registers (see bugs).
- Lane elem_size is ignored (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus index-oob/unsupported-src/alt-spellings/lane-elem.

# Confirmed invariants (encode_neon_logical)

- Valid AND/ORR/EOR Vd.T, Vn.T, Vm.T with T in {8b,16b}, v0–v31, opc in {0b00,0b01,0b10} matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: bit31=0, Q at 30 (1 iff T=16b), U at 29, bits[28:24]=01110, size at [23:22] (00 AND / 10 ORR / 00 EOR), bit21=1, Rm at [20:16], bits[15:10]=000111, Rn at [9:5], Rd at [4:0] (1000 cases).
- AND vs ORR differ only in size bits[23:22]; AND vs EOR differ only in U bit 29; 8b vs 16b XOR = 1<<30 (1000 cases).
- Rd/Rn/Rm isolation in bits[4:0]/[9:5]/[20:16] (1000 cases).
- Arity 0–2 always Err (1000 cases).
- opc outside {0,1,2,3} always Err("unsupported NEON logical opc") (1000 cases).
- Uppercase V prefix matches llvm-mc (1000 cases).
- Known-answer: `and v0.8b, v1.8b, v2.8b` = 0x0e221c20; `and v0.16b` = 0x4e221c20; `orr v0.8b` = 0x0ea21c20; `orr v0.16b` = 0x4ea21c20; `eor v0.8b` = 0x2e221c20; `eor v0.16b` = 0x6e221c20; `and v31.16b, v31.16b, v31.16b` = 0x4e3f1fff; `eor v15.16b, v16.16b, v17.16b` = 0x6e311e0f.

## Environment (encode_neon_logical)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same AND/ORR/EOR T in {8B,16B}; size field is the logical opcode discriminator, not element size.
- Dispatch: encoder/mod.rs:295-298 and/orr/eor/ands => encode_logical; data_processing.rs:461-463 NEON vector form (first operand RegArrangement) passes through to encode_neon_logical.
- encode_neon_logical does not check operands.len() > 3; arr_n/arr_m discarded; Q is 1 iff dest arrangement is exactly "16b"; get_neon_reg accepts Operand::Reg; opc=0b11 encodes as EOR.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: arity/unsupported-opc/alt-spellings passing, ANDS failing.
- Five SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_logical_*.md.

## Quirks (encode_neon_logical)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded (see bugs).
- Arrangements other than 16b encode Q=0 (see bugs).
- Operand::Reg and x/w prefixes encode as V registers (see bugs).
- opc=0b11 (ANDS) encodes as EOR (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus arity/unsupported-opc/alt-spellings/ANDS.

---

# Confirmed invariants (encode_neon_three_diff)

- Valid LONG three-different (saddl/uaddl/ssubl/usubl/sabal/uabal/sabdl/uabdl/smlal/umlal/smlsl/umlsl/smull/umull) with Ta=widen(Tb), Tb in {8b,16b,4h,8h,2s,4s}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path LONG word: bit31=0, Q at 30, U at 29, bits[28:24]=01110, size at [23:22] from Tb, bit21=1, Rm at [20:16], opcode at [15:12], bits[11:10]=00, Rn at [9:5], Rd at [4:0] (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29; opcode isolation in bits[15:12]; is_high XOR = 1<<30 on 8b (1000 cases).
- Arity 0–2 always Err (1000 cases).
- Unsupported source arrangement {1d,2d,1q,empty} always Err (1000 cases).
- Known-answer: `saddl v0.8h, v1.8b, v2.8b` = 0x0e220020; `saddl2 v0.8h, v1.16b, v2.16b` = 0x4e220020; `saddl v0.4s, v1.4h, v2.4h` = 0x0e620020; `saddl2 v0.4s, v1.8h, v2.8h` = 0x4e620020; `saddl v0.2d, v1.2s, v2.2s` = 0x0ea20020; `saddl2 v0.2d, v1.4s, v2.4s` = 0x4ea20020; `uaddl v0.8h, v1.8b, v2.8b` = 0x2e220020; `ssubl v0.8h, v1.8b, v2.8b` = 0x0e222020; `usubl v0.8h, v1.8b, v2.8b` = 0x2e222020; `smull v0.8h, v1.8b, v2.8b` = 0x0e22c020; `umull v0.8h, v1.8b, v2.8b` = 0x2e22c020; llvm-mc `saddw v0.8h, v1.8h, v2.8b` = 0x0e221020 (SUT disagrees).

## Environment (encode_neon_three_diff)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-different LONG Tb in {8B,16B,4H,8H,2S,4S}, Ta = widen(Tb); WIDE Vd.Ta, Vn.Ta, Vm.Tb; size:Q=11:x reserved.
- Dispatch: encoder/mod.rs:314/325 smull/umull vector; 681-688 uabal/sabal/uabdl/sabdl (+2); 815-905 usubl/ssubl/usubw/ssubw/uaddl/saddl/uaddw/saddw/umlal/smlal/umlsl/smlsl/umull2/smull2.
- encode_neon_three_diff checks operands.len() < 3; extra ignored; dest and Rm arrangements discarded; size/Q from Vn; is_high forces Q=1.
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: unsupported-src passing, Rm Tb mismatch failing.
- Five SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_three_diff_*.md.

## Quirks (encode_neon_three_diff)

- Extra operands beyond index 2 are ignored (see bugs).
- Destination arrangement is discarded (see bugs).
- Vm arrangement is discarded (see bugs).
- WIDE size/Q come from Vn, not narrow Vm (see bugs).
- Operand::Reg and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus unsupported-src and Rm Tb.

---

# Confirmed invariants (encode_neon_three_same)

- Valid integer three-same (cmeq/cmhi/cmhs/cmge/cmgt/cmtst/sqadd/uqadd/sqsub/uqsub/sshl/ushl/sqshl/uqshl/srshl/urshl/sqrshl/uqrshl/addp) with matching T in {8b,16b,4h,8h,2s,4s,2d}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: bit31=0, Q at 30, U at 29, bits[28:24]=01110, size at [23:22], bit21=1, Rm at [20:16], opcode at [15:11], bit10=1, Rn at [9:5], Rd at [4:0] (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29; opcode isolation in bits[15:11]; 8b vs 16b XOR = 1<<30 (1000 cases).
- Arity 0–2 always Err (1000 cases).
- Invalid NEON names (v32, foo, empty) and non-register kinds (Imm/Mem/Symbol) always Err (1000 cases).
- Known-answer: `cmeq v0.8b, v1.8b, v2.8b` = 0x2e228c20; `cmeq v0.16b` = 0x6e228c20; `cmeq v0.2d` = 0x6ee28c20; `cmhi v0.8b` = 0x2e223420; `cmgt v0.4s` = 0x4ea23420; `uqsub v31.8h, v30.8h, v29.8h` = 0x6e7d2fdf; `sqadd v0.2d` = 0x4ee20c20; `sshl v15.4h, v16.4h, v17.4h` = 0x0e71460f.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD three-same: T in {8B,16B,4H,8H,2S,4S,2D}; size:Q=11:0 reserved.
- Dispatch: encoder/mod.rs:567-638 cmeq/cmhi/…/addp; encoder/mod.rs:302 mul vector.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded; Q/size come only from dest (see bugs).
- neon_arr_to_q_size accepts 1d (reserved for three-same) (see bugs).
- get_neon_reg accepts Operand::Reg, so bare Vn encodes (see bugs).
- parse_reg_num accepts x/w/d/s/q/h/b prefixes, so GPR dest encodes as Vd (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg / neon_arr_to_q_size / Ok Word).

---

# Confirmed invariants (encode_neon_mls)

- Valid MLS Vd.T, Vn.T, Vm.T with T in {8b,16b,4h,8h,2s,4s}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=1 at bit29, bits[28:24]=01110, size at [23:22], bit21=1, Rm at [20:16], bits[15:10]=100101, Rn at [9:5], Rd at [4:0].
- Arity 0–2 returns Err (1000 cases).
- Uppercase MLS/V prefix with matching T matches llvm-mc (1000 cases).
- Imm/Mem/Label at any slot returns Err (1000 cases).
- Known-answer: `mls v0.8b, v1.8b, v2.8b` = 0x2e229420; `mls v0.16b, v1.16b, v2.16b` = 0x6e229420; `mls v0.4h, v1.4h, v2.4h` = 0x2e629420; `mls v0.8h, v1.8h, v2.8h` = 0x6e629420; `mls v0.2s, v1.2s, v2.2s` = 0x2ea29420; `mls v0.4s, v1.4s, v2.4s` = 0x6ea29420; `mls v31.8b, v31.8b, v31.8b` = 0x2e3f97ff; `mls v0.8b, v0.8b, v0.8b` = 0x2e209400; `mls v15.4s, v16.4s, v17.4s` = 0x6eb1960f.
- Extra operand, mismatched T, reserved 1d/2d, and bare/GPR dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_mls)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same MLS: T in {8B,16B,4H,8H,2S,4S}; size:Q=11:x reserved; U=1; bits[15:10]=100101.
- Dispatch: encoder/mod.rs:783 `"mls" => if RegLane then encode_neon_elem else encode_neon_mls(operands)` with operands passed through for the vector form.
- encode_neon_mls has no operands.len() check; extra ignored; source arrangements discarded; neon_arr_to_q_size accepts 1d/2d as size=11.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus reserved T and non-register operands.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_mls_*.md.

## Quirks (encode_neon_mls)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded (see bugs).
- 1d/2d dest T is encoded with size=11 (see bugs).
- Operand::Reg source and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

# Confirmed invariants (encode_neon_mla)

- Valid MLA Vd.T, Vn.T, Vm.T with T in {8b,16b,4h,8h,2s,4s}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=0 at bit29, bits[28:24]=01110, size at [23:22], bit21=1, Rm at [20:16], bits[15:10]=100101, Rn at [9:5], Rd at [4:0].
- Arity 0–2 returns Err (1000 cases).
- Uppercase MLA/V prefix with matching T matches llvm-mc (1000 cases).
- Imm/Mem/Label at any slot returns Err (1000 cases).
- Known-answer: `mla v0.8b, v1.8b, v2.8b` = 0x0e229420; `mla v0.16b, v1.16b, v2.16b` = 0x4e229420; `mla v0.4h, v1.4h, v2.4h` = 0x0e629420; `mla v0.8h, v1.8h, v2.8h` = 0x4e629420; `mla v0.2s, v1.2s, v2.2s` = 0x0ea29420; `mla v0.4s, v1.4s, v2.4s` = 0x4ea29420; `mla v31.8b, v31.8b, v31.8b` = 0x0e3f97ff; `mla v0.8b, v0.8b, v0.8b` = 0x0e209400; `mla v15.4s, v16.4s, v17.4s` = 0x4eb1960f.
- Extra operand, mismatched T, reserved 1d/2d, and bare/GPR dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_mla)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same MLA: T in {8B,16B,4H,8H,2S,4S}; size:Q=11:x reserved; U=0; bits[15:10]=100101.
- Dispatch: encoder/mod.rs:778 `"mla" => if RegLane then encode_neon_elem else encode_neon_mla(operands)` with operands passed through for the vector form.
- encode_neon_mla has no operands.len() check; extra ignored; source arrangements discarded; neon_arr_to_q_size accepts 1d/2d as size=11.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus reserved T and non-register operands.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_mla_*.md.

## Quirks (encode_neon_mla)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded (see bugs).
- 1d/2d dest T is encoded with size=11 (see bugs).
- Operand::Reg source and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

# Confirmed invariants (encode_neon_pmul)

- Valid PMUL Vd.T, Vn.T, Vm.T with T in {8b,16b}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30 (1 iff T=16b), U=1 at bit29, bits[28:24]=01110, size[23:22]=00, bit21=1, Rm at [20:16], bits[15:10]=100111, Rn at [9:5], Rd at [4:0].
- Arity 0–2 returns Err (1000 cases).
- Uppercase PMUL/V prefix with matching T matches llvm-mc (1000 cases).
- Imm/Mem/Label at any slot returns Err (1000 cases).
- Known-answer: `pmul v0.8b, v1.8b, v2.8b` = 0x2e229c20; `pmul v0.16b, v1.16b, v2.16b` = 0x6e229c20; `pmul v31.8b, v31.8b, v31.8b` = 0x2e3f9fff; `pmul v0.8b, v0.8b, v0.8b` = 0x2e209c00; `pmul v15.16b, v16.16b, v17.16b` = 0x6e319e0f.
- Extra operand, mismatched T, reserved non-byte T, and bare/GPR dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_pmul)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same PMUL: T in {8B,16B} only; size must be 00; U=1; bits[15:10]=100111.
- Dispatch: encoder/mod.rs:775 `"pmul" => encode_neon_pmul(operands)` with operands passed through.
- encode_neon_pmul has no operands.len() check; extra ignored; source arrangements discarded; Q=1 iff arr_d=="16b" else 0 (non-byte T encoded as 8B).
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus reserved T and non-register operands.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_pmul_*.md.

## Quirks (encode_neon_pmul)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded (see bugs).
- Non-byte dest T is encoded as 8B (see bugs).
- Operand::Reg source and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

# Confirmed invariants (encode_neon_mul)

- Valid MUL Vd.T, Vn.T, Vm.T with T in {8b,16b,4h,8h,2s,4s}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=0 at bit29, bits[28:24]=01110, size at [23:22], bit21=1, Rm at [20:16], bits[15:10]=100111, Rn at [9:5], Rd at [4:0].
- Arity 0–2 returns Err (1000 cases).
- Uppercase MUL/V prefix with matching T matches llvm-mc (1000 cases).
- Imm/Mem/Label at any slot returns Err (1000 cases).
- Known-answer: `mul v0.8b, v1.8b, v2.8b` = 0x0e229c20; `mul v0.16b, v1.16b, v2.16b` = 0x4e229c20; `mul v0.4h, v1.4h, v2.4h` = 0x0e629c20; `mul v0.8h, v1.8h, v2.8h` = 0x4e629c20; `mul v0.2s, v1.2s, v2.2s` = 0x0ea29c20; `mul v0.4s, v1.4s, v2.4s` = 0x4ea29c20; `mul v31.8b, v31.8b, v31.8b` = 0x0e3f9fff; `mul v0.8b, v0.8b, v0.8b` = 0x0e209c00; `mul v15.4s, v16.4s, v17.4s` = 0x4eb19e0f.
- Extra operand, mismatched T, reserved 1d/2d, and bare/GPR dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_mul)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same MUL: T in {8B,16B,4H,8H,2S,4S}; size:Q=11:x reserved; U=0; bits[15:11]=10011.
- Dispatch: encoder/mod.rs:290-296 textual `mul` with RegArrangement goes to encode_neon_three_same / encode_neon_elem; encode_mul (data_processing.rs:586-588) still forwards RegArrangement dest to encode_neon_mul.
- encode_neon_mul has no operands.len() check; extra ignored; source arrangements discarded; neon_arr_to_q_size accepts 1d/2d.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg other / alt spellings / reserved T).

## Quirks (encode_neon_mul)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded (see bugs).
- neon_arr_to_q_size 1d/2d is encoded (see bugs).
- Operand::Reg source and x/w prefixes encode as V registers (see bugs).
- Bare dest (empty arrangement) hits unsupported arrangement Err via neon_arr_to_q_size.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

# Confirmed invariants (encode_neon_xtl)

- Valid UXTL/SXTL/UXTL2/SXTL2 Vd.Ta, Vn.Tb with mandated (Ta,Tb,Q) pairs, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; U in bit 29; is_high with matching Tb differs only in Q bit 30 (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U at bit29, bits[28:23]=011110, immh at [22:19] from source esize (8b|16b→0001, 4h|8h→0010, 2s|4s→0100), immb=000 at [18:16], bits[15:10]=101001, Rn at [9:5], Rd at [4:0].
- Arity 0–1 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label at dest or src returns Err (1000 cases).
- Known-answer: `uxtl v0.8h, v1.8b` = 0x2f08a420; `sxtl v0.8h, v1.8b` = 0x0f08a420; `uxtl2 v0.8h, v1.16b` = 0x6f08a420; `sxtl2 v0.4s, v1.8h` = 0x4f10a420; `uxtl v0.2d, v1.2s` = 0x2f20a420; `uxtl v0.4s, v1.4h` = 0x2f10a420; `uxtl2 v0.2d, v1.4s` = 0x6f20a420; `sxtl v31.8h, v31.8b` = 0x0f08a7ff; `uxtl v0.8h, v0.8b` = 0x2f08a400; `sxtl v15.4s, v16.4h` = 0x0f10a60f.
- Extra operand, mismatched Ta/Tb, and GPR/bare dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_xtl)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD shift-by-immediate UXTL/SXTL (USHLL/SSHLL #0): Ta in {8H,4S,2D}; Tb 8B/4H/2S (Q=0) or 16B/8H/4S (Q=1); U=1 unsigned / U=0 signed; immh from source esize; immb=000; bits[15:10]=101001.
- Dispatch: encoder/mod.rs:895-898 uxtl/uxtl2/sxtl/sxtl2 => encode_neon_xtl.
- encode_neon_xtl checks operands.len() < 2; extra ignored; dest arrangement discarded; Q from is_high only.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg other / alt spellings).

## Quirks (encode_neon_xtl)

- Extra operands beyond index 1 are ignored (see bugs).
- Destination arrangement is discarded (see bugs).
- Operand::Reg dest and x/w prefixes encode as V registers (see bugs).
- Bare source (empty arrangement) hits unsupported-source Err.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_neon_two_misc)

- Valid matching-T ABS/NEG/CLS/CLZ/REV16/REV32/SQABS/SQNEG Vd.T, Vn.T with opcode-legal T, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; U in bit 29; 8b vs 16b differs only in Q bit 30 (1000 cases).
- Success-path 32-bit word for matching-T: bit31=0, Q at bit30, U at bit29, bits[28:24]=01110, size at [23:22] from dest T, bits[21:17]=10000, opcode at [16:12], bits[11:10]=10, Rn at [9:5], Rd at [4:0].
- Arity 0–1 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label/GPR/bare-V at dest or src returns Err (1000 cases).
- Known-answer: `abs v0.8b, v1.8b` = 0x0e20b820; `abs v0.16b, v1.16b` = 0x4e20b820; `abs v0.2d, v1.2d` = 0x4ee0b820; `neg v0.8b, v1.8b` = 0x2e20b820; `cls v0.8b, v1.8b` = 0x0e204820; `clz v0.8b, v1.8b` = 0x2e204820; `rev16 v0.8b, v1.8b` = 0x0e201820; `sqabs v0.8b, v1.8b` = 0x0e207820; `sqneg v0.2d, v1.2d` = 0x6ee07820; `neg v31.2d, v31.2d` = 0x6ee0bbff; llvm-mc `saddlp v0.4h, v1.8b` = 0x0e202820 (SUT disagrees).
- Extra operand, mismatched T, reserved T (abs .1d, cls .2d), and pairwise-long size-from-dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_two_misc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD two-register miscellaneous: matching-T ABS/NEG/SQABS/SQNEG T in {8B,16B,4H,8H,2S,4S,2D}; CLS/CLZ T in {8B,16B,4H,8H,2S,4S}; REV16 T in {8B,16B}; REV32 T in {8B,16B,4H,8H}; SADDLP/UADDLP/SADALP/UADALP dest Ta from source Tb, size from source esize.
- Dispatch: encoder/mod.rs:324-326 neg; 616 abs; 618-628 cls/clz/rev16/rev32; 630-633 saddlp/uaddlp/sadalp/uadalp; 637-645 sqabs/sqneg.
- encode_neon_two_misc does not check operands.len(); dest arrangement drives Q/size; source arrangement discarded.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg other / alt spellings).

## Quirks (encode_neon_two_misc)

- Extra operands beyond index 1 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- neon_arr_to_q_size accepts 1d/2d; opcode-reserved T is encoded (see bugs).
- Pairwise-long size comes from dest T, not source Tb (see bugs).
- Operand::Reg dest/src with empty arrangement fails neon_arr_to_q_size (Err).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_neon_shrn)

- Valid SHRN/SHRN2/RSHRN/RSHRN2 Vd.Tb, Vn.Ta, #shift with Ta in {8h,4s,2d}, Tb mandated by (Ta, Q), shift in [1, dest_esize], v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; SHRN vs SHRN2 differs only in Q bit 30; SHRN vs RSHRN differs only in bits[15:10] (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=0 at bit29, bits[28:23]=011110, immh:immb at [22:16] = source_esize-shift, bits[15:10]=100001 (SHRN) or 100011 (RSHRN), Rn at [9:5], Rd at [4:0]. dest_esize: 8h→8, 4s→16, 2d→32.
- Arity 0–2 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label at dest/src/imm slots returns Err (1000 cases).
- Unsupported source Ta in {8b,16b,4h,2s,1d,1q} returns Err (1000 cases).
- Known-answer: `shrn v0.8b, v1.8h, #1` = 0x0f0f8420; `shrn v0.8b, v1.8h, #8` = 0x0f088420; `shrn2 v0.16b, v1.8h, #1` = 0x4f0f8420; `shrn v0.4h, v1.4s, #1` = 0x0f1f8420; `shrn v0.4h, v1.4s, #16` = 0x0f108420; `shrn2 v0.8h, v1.4s, #16` = 0x4f108420; `shrn v0.2s, v1.2d, #1` = 0x0f3f8420; `shrn v0.2s, v1.2d, #32` = 0x0f208420; `shrn2 v0.4s, v1.2d, #1` = 0x4f3f8420; `rshrn v0.8b, v1.8h, #1` = 0x0f0f8c20; `rshrn2 v0.16b, v1.8h, #8` = 0x4f088c20; `shrn v31.8b, v31.8h, #8` = 0x0f0887ff; `shrn v0.8b, v0.8h, #1` = 0x0f0f8400; `shrn v15.4h, v16.4s, #9` = 0x0f17860f.
- Extra operand, mismatched dest Tb, bare V dest, and i64 shift truncated via `as u32` currently disagree with llvm-mc/gas (see bugs). Shift 0 and dest_esize+1 are rejected by the half_bits check.

## Environment (encode_neon_shrn)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD shift-by-immediate SHRN/RSHRN: Ta in {8H,4S,2D}; Tb 8B/16B, 4H/8H, 2S/4S; shift in [1, dest_esize]; U=0; opcode 100001/100011; immh != 0000; immh:immb = source_esize - shift.
- Dispatch: encoder/mod.rs:648-651 `shrn`/`shrn2`/`rshrn`/`rshrn2` => encode_neon_shrn.
- Sibling encode_neon_qshrn is saturating (same-job gate fails). encode_neon_sqshrun is signed-to-unsigned saturating. encode_neon_scalar_qshrn is scalar. encode_neon_three_diff_narrow is ADDHN/SUBHN.
- encode_neon_shrn checks operands.len() < 3; extra ignored; dest arrangement discarded; shift is `get_imm as u32` then range-checked against half_bits = source/2.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg other / alt spellings / unsupported Ta).

## Quirks (encode_neon_shrn)

- Extra operands beyond index 2 are ignored (see bugs).
- Destination arrangement is discarded (see bugs).
- Shift is `get_imm as u32` before the range check; values congruent to a valid shift modulo 2^32 encode (see bugs). Shift 0 and dest_esize+1 are rejected.
- Operand::Reg dest and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_neon_sri)

- Valid SRI Vd.T, Vn.T, #shift with T in {8b,16b,4h,8h,2s,4s,2d}, shift in [1, esize(T)], v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; 8b vs 16b at the same shift differs only in Q bit 30 (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=1 at bit29, bits[28:23]=011110, immh:immb at [22:16] = 2*esize-shift, bits[15:10]=010001, Rn at [9:5], Rd at [4:0]. Q from T: 8b=0, 16b=1, 4h=0, 8h=1, 2s=0, 4s=1, 2d=1. esize: 8b/16b=8, 4h/8h=16, 2s/4s=32, 2d=64.
- Arity 0–2 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label at dest/src slots returns Err (1000 cases).
- Known-answer: `sri v0.8b, v1.8b, #1` = 0x2f0f4420; `sri v0.16b, v1.16b, #8` = 0x6f084420; `sri v0.4h, v1.4h, #1` = 0x2f1f4420; `sri v0.8h, v1.8h, #16` = 0x6f104420; `sri v0.2s, v1.2s, #1` = 0x2f3f4420; `sri v0.4s, v1.4s, #32` = 0x6f204420; `sri v0.2d, v1.2d, #1` = 0x6f7f4420; `sri v0.2d, v1.2d, #64` = 0x6f404420; `sri v31.8b, v31.8b, #8` = 0x2f0847ff; `sri v0.8b, v0.8b, #1` = 0x2f0f4400; `sri v15.4s, v16.4s, #17` = 0x6f2f460f; `sri v0.8b, v1.8b, #8` = 0x2f084420.
- Extra operand, mismatched T, bare source / GPR dest, and shift 0 currently disagree with llvm-mc/gas (see bugs). Debug overflow panic at neon.rs:1297 for 8-bit T when `16 - shift` underflows (shift as u32 from i64 -1).

## Environment (encode_neon_sri)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD shift-by-immediate SRI: T in {8B,16B,4H,8H,2S,4S,2D}; shift in [1, esize]; matching arrangements; U=1; opcode=010001; immh != 0000; immh:immb = 2*esize - shift.
- Dispatch: encoder/mod.rs:706 `"sri" => encode_neon_sri(operands)`.
- Sibling encode_neon_sli is SLI / left-insert / opcode 010101 (same-job gate fails). encode_neon_ushr is USHR / opcode 000001. encode_neon_sshr is SSHR / U=0. encode_neon_shift_right is a generic right-shift helper (independence gate).
- encode_neon_sri checks operands.len() < 3; extra ignored; source arrangement discarded; shift masked per T with no range check.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg other / alt spellings).

## Quirks (encode_neon_sri)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Shift is `get_imm as u32` with no range check; #0 encodes reserved immh=0000; negatives overflow in debug (see bugs).
- Operand::Reg dest/src and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_neon_shl)

- Valid SHL Vd.T, Vn.T, #shift with T in {8b,16b,4h,8h,2s,4s,2d}, shift in [0, esize(T)-1], v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; 8b vs 16b at the same shift differs only in Q bit 30 (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=0 at bit29, bits[28:23]=011110, immh:immb at [22:16] = esize+shift, bits[15:10]=010101, Rn at [9:5], Rd at [4:0]. Q from T: 8b=0, 16b=1, 4h=0, 8h=1, 2s=0, 4s=1, 2d=1. esize: 8b/16b=8, 4h/8h=16, 2s/4s=32, 2d=64.
- Arity 0–2 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label at dest/src slots returns Err (1000 cases).
- Known-answer: `shl v0.8b, v1.8b, #0` = 0x0f085420; `shl v0.8b, v1.8b, #7` = 0x0f0f5420; `shl v0.16b, v1.16b, #0` = 0x4f085420; `shl v0.4h, v1.4h, #0` = 0x0f105420; `shl v0.8h, v1.8h, #15` = 0x4f1f5420; `shl v0.2s, v1.2s, #0` = 0x0f205420; `shl v0.4s, v1.4s, #31` = 0x4f3f5420; `shl v0.2d, v1.2d, #0` = 0x4f405420; `shl v0.2d, v1.2d, #63` = 0x4f7f5420; `shl v31.8b, v31.8b, #1` = 0x0f0957ff; `shl v0.8b, v0.8b, #0` = 0x0f085400; `shl v15.4s, v16.4s, #16` = 0x4f30560f.
- Extra operand, mismatched T, bare source / GPR dest, and shift OOB currently disagree with llvm-mc/gas (see bugs). Debug overflow panic at neon.rs:1245 for 8-bit T when `8 + shift` overflows (shift as u32 from i64 -1).

## Environment (encode_neon_shl)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD shift-by-immediate SHL: T in {8B,16B,4H,8H,2S,4S,2D}; shift in [0, esize-1]; matching arrangements; U=0; opcode=010101; immh != 0000; immh:immb = esize + shift.
- Dispatch: encoder/mod.rs:702 `"shl" => encode_neon_shl(operands)`.
- Sibling encode_neon_sli is SLI / U=1 (same-job gate fails). encode_neon_shift_left_imm is a generic left-shift helper (independence gate). encode_neon_sshr / encode_neon_ushr are right-shift encodings.
- encode_neon_shl checks operands.len() < 3; extra ignored; source arrangement discarded; shift masked per T with no range check.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg other / alt spellings).

## Quirks (encode_neon_shl)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Shift is `get_imm as u32` with no range check; #esize encodes reserved immh=0000; negatives overflow in debug (see bugs).
- Operand::Reg dest/src and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_neon_sshr)

- Valid SSHR Vd.T, Vn.T, #shift with T in {8b,16b,4h,8h,2s,4s,2d}, shift in [1, esize(T)], v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; 8b vs 16b at the same shift differs only in Q bit 30 (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=0 at bit29, bits[28:23]=011110, immh:immb at [22:16] = 2*esize-shift, bits[15:10]=000001, Rn at [9:5], Rd at [4:0]. Q from T: 8b=0, 16b=1, 4h=0, 8h=1, 2s=0, 4s=1, 2d=1. esize: 8b/16b=8, 4h/8h=16, 2s/4s=32, 2d=64.
- Arity 0–2 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label at dest/src slots returns Err (1000 cases).
- Known-answer: `sshr v0.8b, v1.8b, #1` = 0x0f0f0420; `sshr v0.16b, v1.16b, #8` = 0x4f080420; `sshr v0.4h, v1.4h, #1` = 0x0f1f0420; `sshr v0.8h, v1.8h, #16` = 0x4f100420; `sshr v0.2s, v1.2s, #1` = 0x0f3f0420; `sshr v0.4s, v1.4s, #32` = 0x4f200420; `sshr v0.2d, v1.2d, #1` = 0x4f7f0420; `sshr v0.2d, v1.2d, #64` = 0x4f400420; `sshr v31.8b, v31.8b, #8` = 0x0f0807ff; `sshr v0.8b, v0.8b, #1` = 0x0f0f0400; `sshr v15.4s, v16.4s, #17` = 0x4f2f060f.
- Extra operand, mismatched T, bare source / GPR dest, and shift 0 currently disagree with llvm-mc/gas (see bugs). Debug overflow panic at neon.rs:1218 for 8-bit T when `16 - shift` underflows.

## Environment (encode_neon_sshr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD shift-by-immediate SSHR: T in {8B,16B,4H,8H,2S,4S,2D}; shift in [1, esize]; matching arrangements; U=0; opcode=000001; immh != 0000; immh:immb = 2*esize - shift.
- Dispatch: encoder/mod.rs:699 `"sshr" => encode_neon_sshr(operands)`.
- Sibling encode_neon_ushr is USHR / U=1 (same-job gate fails). encode_neon_shift_imm is a near-copy USHR helper (independence gate). encode_neon_shift_right is a generic shift-right table.
- encode_neon_sshr checks operands.len() < 3; extra ignored; source arrangement discarded; shift masked per T with no range check.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / get_neon_reg other / alt spellings).

## Quirks (encode_neon_sshr)

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Shift is `get_imm as u32` with no range check; #0 encodes reserved immh=0000; negatives overflow in debug (see bugs).
- Operand::Reg dest/src and x/w prefixes encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_neon_ushr)

- Valid USHR Vd.T, Vn.T, #shift with T in {8b,16b,4h,8h,2s,4s,2d}, shift in [1, esize(T)], v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; 8b vs 16b at the same shift differs only in Q bit 30 (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U=1 at bit29, bits[28:23]=011110, immh:immb at [22:16] = 2*esize-shift, bits[15:10]=000001, Rn at [9:5], Rd at [4:0]. Q from T: 8b=0, 16b=1, 4h=0, 8h=1, 2s=0, 4s=1, 2d=1. esize: 8b/16b=8, 4h/8h=16, 2s/4s=32, 2d=64.
- Arity 0–2 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label at dest/src slots returns Err (1000 cases).
- Known-answer: `ushr v0.8b, v1.8b, #1` = 0x2f0f0420; `ushr v0.16b, v1.16b, #8` = 0x6f080420; `ushr v0.4h, v1.4h, #1` = 0x2f1f0420; `ushr v0.8h, v1.8h, #16` = 0x6f100420; `ushr v0.2s, v1.2s, #1` = 0x2f3f0420; `ushr v0.4s, v1.4s, #32` = 0x6f200420; `ushr v0.2d, v1.2d, #1` = 0x6f7f0420; `ushr v0.2d, v1.2d, #64` = 0x6f400420; `ushr v31.8b, v31.8b, #8` = 0x2f0807ff; `ushr v0.8b, v0.8b, #1` = 0x2f0f0400; `ushr v15.4s, v16.4s, #17` = 0x6f2f060f.
- Extra operand, mismatched T, GPR dest, and shift 0 currently disagree with llvm-mc/gas (see bugs). Debug overflow panic at neon.rs:1192 for 8-bit T when shift > 16.

## Environment (encode_neon_ushr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD shift-by-immediate USHR: T in {8B,16B,4H,8H,2S,4S,2D}; shift in [1, esize]; matching arrangements; U=1; opcode=000001; immh != 0000; immh:immb = 2*esize - shift.
- Dispatch: encoder/mod.rs:696 `"ushr" => encode_neon_ushr(operands)`.
- Sibling encode_neon_shift_imm is a near-copy (independence gate). encode_neon_sshr / encode_neon_shift_right are different jobs.
- encode_neon_ushr checks operands.len() < 3; extra ignored; source arrangement discarded; shift masked per T with no range check.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus uppercase V alt-spellings and non-register operands.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_ushr_*.md.

# Confirmed invariants (encode_neon_add_sub)

- Valid ADD/SUB Vd.T, Vn.T, Vm.T with T in {8b,16b,4h,8h,2s,4s,2d}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16]; add vs sub differs only in U bit 29 (1000 cases).
- Success-path 32-bit word: bit31=0, Q at bit30, U at bit29, bits[28:24]=01110, size at [23:22], bit21=1, Rm at [20:16], bits[15:11]=10000, bit10=1, Rn at [9:5], Rd at [4:0]. Q/size from T: 8b=(0,00), 16b=(1,00), 4h=(0,01), 8h=(1,01), 2s=(0,10), 4s=(1,10), 2d=(1,11).
- Arity 0–2 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with lowercase T matches llvm-mc (1000 cases).
- Imm/Mem/Label at any operand slot returns Err (1000 cases).
- Known-answer: `add v0.8b, v1.8b, v2.8b` = 0x0e228420; `add v0.16b, v1.16b, v2.16b` = 0x4e228420; `add v0.2d, v1.2d, v2.2d` = 0x4ee28420; `sub v0.8b, v1.8b, v2.8b` = 0x2e228420; `add v31.8b, v31.8b, v31.8b` = 0x0e3f87ff; `add v0.8b, v0.8b, v0.8b` = 0x0e208400; `add v15.4s, v16.4s, v17.4s` = 0x4eb1860f.
- Extra operand, mismatched/reserved T, and bare/GPR currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_add_sub)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same ADD/SUB: T in {8B,16B,4H,8H,2S,4S,2D}; size:Q=11:0 (1D) reserved; matching arrangements; U=0 ADD / U=1 SUB; opcode=10000.
- Dispatch: encoder/mod.rs:259-266 `add`/`sub` => encode_add_sub (when not scalar D-reg); data_processing.rs:296-300 NEON vector form calls encode_neon_add_sub.
- Sibling encode_neon_three_same / encode_neon_scalar_three_same are different jobs, not same-job differentials.
- encode_neon_add_sub has no arity check; extra ignored; source arrangements discarded; neon_arr_to_q_size accepts 1d.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus uppercase V alt-spellings and non-register operands.
- Three SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_add_sub_*.md.

# Confirmed invariants (encode_neon_pmull)

- Valid PMULL Vd.1q, Vn.1d, Vm.1d and PMULL2 Vd.1q, Vn.2d, Vm.2d with v0–v31 matches llvm-mc `-triple=aarch64 -mattr=+aes -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16]; pmull vs pmull2 differs only in Q bit 30 (1000 cases).
- Success-path 64-bit word: bit31=0, Q at bit30, bits[29:24]=001110, size[23:22]=11, bit21=1, Rm at [20:16], bits[15:11]=11100, bit10=0, Rn at [9:5], Rd at [4:0].
- Arity 0–2 returns Err (1000 cases).
- Uppercase mnemonic/V prefix with 1Q/1D/2D matches llvm-mc (1000 cases).
- Imm/Mem/Label at any operand slot returns Err (1000 cases).
- Known-answer: `pmull v0.1q, v1.1d, v2.1d` = 0x0ee2e020; `pmull2 v0.1q, v1.2d, v2.2d` = 0x4ee2e020; `pmull v31.1q, v31.1d, v31.1d` = 0x0effe3ff; `pmull v0.1q, v0.1d, v0.1d` = 0x0ee0e000. llvm-mc 8h KAT: `pmull v0.8h, v1.8b, v2.8b` = 0x0e22e020 (SUT disagrees).
- Extra operand, invalid T, GPR dest, and 8h form currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_pmull)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+aes -show-encoding.
- ARM ARM Advanced SIMD three-different PMULL{2}: Ta in {8H,1Q}; Tb is 8B/16B or 1D/2D; U=0 opcode=1110; size=00 (8-bit) or 11 (64-bit); Q=0 PMULL / Q=1 PMULL2.
- Dispatch: encoder/mod.rs:779-780 `"pmull"` / `"pmull2"` => encode_neon_pmull.
- Sibling encode_neon_three_diff / encode_neon_pmul are different opcodes, not same-job differentials.
- encode_neon_pmull checks operands.len() < 3; extra ignored; all three arrangements discarded; size hardcoded to 11.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus uppercase V alt-spellings and non-register operands.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_pmull_*.md.

# Confirmed invariants (encode_neon_eor3)

- Valid EOR3 Vd.16b, Vn.16b, Vm.16b, Vk.16b with v0–v31 matches llvm-mc `-triple=aarch64 -mattr=+sha3 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16]; only Rk in bits[14:10] (1000 cases).
- Success-path word: bits[31:24]=11001110, bits[23:21]=000, Rm at [20:16], bit15=0, Rk/Ra at [14:10], Rn at [9:5], Rd at [4:0] = 0xce000000 | (Rm<<16) | (Rk<<10) | (Rn<<5) | Rd.
- Arity 0–3 returns Err (1000 cases).
- Uppercase V prefix with 16b arrangement matches llvm-mc (1000 cases).
- Imm/Mem/Label at any operand slot returns Err (1000 cases).
- Known-answer: `eor3 v0.16b, v1.16b, v2.16b, v3.16b` = 0xce020c20; `eor3 v31.16b, v31.16b, v31.16b, v31.16b` = 0xce1f7fff; `eor3 v0.16b, v0.16b, v0.16b, v0.16b` = 0xce000000.
- Extra operand, T≠16b, mismatched T, and GPR/bare-V currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_eor3)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+sha3 -show-encoding.
- ARM ARM Cryptographic three-register SHA3 EOR3: only arrangement 16B; Vd, Vn, Vm, Va.
- Dispatch: encoder/mod.rs:776 `"eor3" => encode_neon_eor3`.
- Sibling encode_neon_aes / encode_neon_logical are different opcodes, not same-job differentials.
- encode_neon_eor3 checks operands.len() < 4; extra ignored; all four arrangements discarded.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus uppercase V alt-spellings and non-register operands.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_eor3_*.md.

# Confirmed invariants (encode_neon_zip_uzp)

- Valid ZIP1/ZIP2/UZP1/UZP2/TRN1/TRN2 with T in {8b,16b,4h,8h,2s,4s,2d}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path word: bit31=0, Q at bit30, bits[29:24]=001110, size at [23:22], bit21=0, Rm at [20:16], bit15=0, opc at [14:12], bits[11:10]=10, Rn at [9:5], Rd at [4:0]. Q/size from T: 8b=(0,00), 16b=(1,00), 4h=(0,01), 8h=(1,01), 2s=(0,10), 4s=(1,10), 2d=(1,11). opc: UZP1=001 TRN1=010 ZIP1=011 UZP2=101 TRN2=110 ZIP2=111.
- Arity 0–2 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Known-answer: `zip1 v0.8b, v1.8b, v2.8b` = 0x0e023820; `zip1 v0.16b, v1.16b, v2.16b` = 0x4e023820; `zip1 v0.2d, v1.2d, v2.2d` = 0x4ec23820; `zip2 v0.8b, v1.8b, v2.8b` = 0x0e027820; `uzp1 v0.8b, v1.8b, v2.8b` = 0x0e021820; `uzp2 v0.8b, v1.8b, v2.8b` = 0x0e025820; `trn1 v0.8b, v1.8b, v2.8b` = 0x0e022820; `trn2 v0.8b, v1.8b, v2.8b` = 0x0e026820.
- Extra operand, reserved 1d, mismatched T, and bare/GPR source currently disagree with llvm-mc/gas (see bugs). Dest as Operand::Reg (empty arrangement) already Errs via neon_arr_to_q_size.

## Environment (encode_neon_zip_uzp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD permute (ZIP/UZP/TRN): T in {8B,16B,4H,8H,2S,4S,2D}; size:Q=11:0 (1D) reserved; matching arrangements.
- Dispatch: encoder/mod.rs:682-683 trn1/trn2; encoder/mod.rs:770-773 uzp1/uzp2/zip1/zip2 => encode_neon_zip_uzp(operands, opc, false).
- Sibling encode_neon_ext / encode_neon_tbl / encode_neon_tbx are different opcodes, not same-job differentials.
- encode_neon_zip_uzp checks operands.len() < 3; extra ignored; source arrangements discarded; neon_arr_to_q_size accepts 1d; `_is_zip` unused.
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus uppercase V alt-spellings.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_zip_uzp_*.md.

# Confirmed invariants (encode_neon_across)

- Valid UMAXV/UMINV/SMAXV/SMINV with dest Bd/Hd/Sd matching T in {8b,16b,4h,8h,4s}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5] (1000 cases).
- Flipping U differs only in bit 29; changing opcode differs only in bits[16:12] (1000 cases).
- Success-path word: bit31=0, Q at bit30, U at bit29, bits[28:24]=01110, size at [23:22], bits[21:17]=11000, opcode at [16:12], bits[11:10]=10, Rn at [9:5], Rd at [4:0]. Q/size from T: 8b=(0,00), 16b=(1,00), 4h=(0,01), 8h=(1,01), 4s=(1,10). Known-answer: `umaxv b0, v1.8b` = 0x2e30a820; `uminv b0, v1.8b` = 0x2e31a820; `smaxv b0, v1.8b` = 0x0e30a820; `sminv b0, v1.8b` = 0x0e31a820; `umaxv s0, v1.4s` = 0x6eb0a820; `smaxv s0, v1.4s` = 0x4eb0a820.
- Arity 0–1 returns Err (1000 cases).
- Uppercase mnemonic/V/B/H/S prefix with lowercase T matches llvm-mc (1000 cases).
- Extra operand, reserved 2s/1d/2d, and GPR dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_across)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD across lanes (UMAXV/UMINV/SMAXV/SMINV): dest Bd/Hd/Sd matching T; T in {8B,16B,4H,8H,4S}; size:Q=10:0 (2S) and size=11 reserved.
- Dispatch: encoder/mod.rs:693-696 umaxv/uminv/smaxv/sminv => encode_neon_across(operands, U, opcode).
- Sibling encode_neon_addv / encode_neon_across_long are different opcodes, not same-job differentials. encode_neon_across uses the correct `(opcode << 12) | (0b10 << 10)` placement.
- encode_neon_across checks operands.len() < 2; extra ignored; dest type discarded (`let (rd, _)`); T filtered only by neon_arr_to_q_size (accepts 2s/1d/2d).
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus uppercase alt-spellings.
- Three SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_across_*.md.

# Confirmed invariants (encode_neon_addv)

- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5] (1000 cases). Rd/Rn packing is correct even though opcode bits are not.
- Arity 0–1 returns Err (1000 cases).
- ARM ADDV success-path word (independent of SUT): 0 Q 0 01110 size 11000 11011 10 Rn Rd = 0x0e31b800 | (Q<<30) | (size<<22) | (Rn<<5) | Rd. Q/size from T: 8b=(0,00), 16b=(1,00), 4h=(0,01), 8h=(1,01), 4s=(1,10). llvm-mc KAT mapping matches this formula (`addv b0, v1.8b` = 0x0e31b820).
- SUT currently disagrees with that word (`0b110111 << 10` vs opcode 11011 at [16:12] and 10 at [11:10]); extra operands, reserved 2s/1d/2d, and GPR dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_addv)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD across lanes (ADDV): dest Bd/Hd/Sd matching T; T in {8B,16B,4H,8H,4S}; size:Q=10:0 (2S) and size=11 reserved.
- Dispatch: encoder/mod.rs:690 `"addv" => encode_neon_addv`.
- Caller: codegen/intrinsics.rs:190 `addv b0, v0.8b`.
- Sibling encode_neon_across / encode_neon_across_long are different opcodes, not same-job differentials. encode_neon_across uses the correct `(opcode << 12) | (0b10 << 10)` placement.
- encode_neon_addv checks operands.len() < 2; extra ignored; dest type discarded (`let (rd, _)`); T filtered only by neon_arr_to_q_size (accepts 2s/1d/2d).
- get_neon_reg accepts Operand::Reg and RegArrangement; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit of the four-statement body.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_addv_*.md.

# Confirmed invariants (encode_neon_bsl)

- Valid BSL Vd.T, Vn.T, Vm.T with T in {8b,16b}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T=16b, bits[29:24]=101110, size bits[23:22]=01, bit21=1, Rm at [20:16], bits[15:10]=000111, Rn at [9:5], Rd at [4:0] = 0x2e601c00 | (Q<<30) | (Rm<<16) | (Rn<<5) | Rd. 8b vs 16b differs only in Q.
- Arity 0–2 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Known-answer: `bsl v0.8b, v1.8b, v2.8b` = 0x2e621c20; `bsl v0.16b, v1.16b, v2.16b` = 0x6e621c20; `bsl v31.8b, v31.8b, v31.8b` = 0x2e7f1fff; `bsl v31.16b, v0.16b, v1.16b` = 0x6e611c1f.
- Extra operand, T∉{8b,16b}, mismatched T, and GPR/SP/bare-V/FP currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_bsl)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD three-same (BSL): 0 Q 1 01110 size=01 1 Rm 000111 Rn Rd. T in {8B,16B} only. Q=1 iff T=16B.
- Dispatch: encoder/mod.rs:699 `"bsl" => encode_neon_bsl`.
- Sibling encode_neon_bic / encode_neon_bitwise_insert (BIT/BIF) are different opcodes, not same-job differentials.
- encode_neon_bsl checks operands.len() < 3; extra ignored; source arrangements discarded; Q=1 iff arr_d=="16b" else 0 (no 8b check).
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (uppercase V alt-spellings).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_bsl_*.md.

# Confirmed invariants (encode_neon_rev64)

- Valid REV64 Vd.T, Vn.T with T in {8b,16b,4h,8h,2s,4s}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T in {16b,8h,4s}, bits[29:24]=001110, size bits[23:22] = B:00 H:01 S:10, bits[21:16]=100000, bits[15:10]=000010, Rn at [9:5], Rd at [4:0] = 0x0e200800 | (Q<<30) | (size<<22) | (Rn<<5) | Rd.
- Arity 0–1 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Dest GPR/SP/bare-V/FP scalar returns Err via empty arrangement rejected by neon_arr_to_q_size.
- Known-answer: `rev64 v0.8b, v1.8b` = 0x0e200820; `rev64 v0.16b, v1.16b` = 0x4e200820; `rev64 v0.4h, v1.4h` = 0x0e600820; `rev64 v0.4s, v1.4s` = 0x4ea00820; `rev64 v31.8b, v31.8b` = 0x0e200bff; `rev64 v31.4s, v0.4s` = 0x4ea0081f.
- Extra operand, T in {1d,2d} (size=11 reserved), mismatched T, and GPR source currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_rev64)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. Range/error contract: aarch64-linux-gnu-as (README claims gas).
- ARM ARM Advanced SIMD two-register miscellaneous (REV64): 0 Q 0 01110 size 10 0000 0000 10 Rn Rd. T in {8B,16B,4H,8H,2S,4S} only. size=11 reserved. Q=1 iff T in {16B,8H,4S}.
- Dispatch: encoder/mod.rs:750 `"rev64" => encode_neon_rev64`.
- Sibling encode_cnt / encode_neon_not / encode_neon_rbit are different two-misc opcodes, not same-job differentials. encode_rev is scalar REV.
- llvm-mc aliases `rev64 x0, x1` to scalar `rev`; gas rejects GPR `rev64`. encode_neon_rev64 dest GPR Errs (empty arrangement); source GPR encodes.
- encode_neon_rev64 checks operands.len() < 2; extra ignored; source arrangement discarded; neon_arr_to_q_size accepts 1d/2d.
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (uppercase V alt-spellings).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_rev64_*.md.

# Confirmed invariants (encode_neon_not)

- Valid NOT Vd.T, Vn.T with T in {8b,16b}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc canonicalizes `not` to `mvn`.
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T=16b, bits[29:24]=101110, size bits[23:22]=00, bits[21:16]=100000, bits[15:10]=010110, Rn at [9:5], Rd at [4:0] = 0x2e205800 | (Q<<30) | (Rn<<5) | Rd. 8b vs 16b differs only in Q.
- Arity 0–1 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Known-answer: `not v0.8b, v1.8b` = 0x2e205820; `not v0.16b, v1.16b` = 0x6e205820; `not v31.8b, v31.8b` = 0x2e205bff; `not v31.16b, v0.16b` = 0x6e20581f.
- Extra operand, T∉{8b,16b}, mismatched T, and GPR/SP/bare-V/FP currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_not)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. Range/error contract: aarch64-linux-gnu-as (README claims gas).
- ARM ARM Advanced SIMD two-register miscellaneous (NOT/MVN): 0 Q 10 1110 size=00 100000 010110 Rn Rd. T in {8B,16B} only. Q=1 iff T=16B.
- Dispatch: encoder/mod.rs:692 `"not" => encode_neon_not`. encode_mvn routes vector MVN into encode_neon_not (caller, not same-job differential).
- Sibling encode_cnt / encode_neon_rbit are different two-misc opcodes, not same-job differentials. encode_neon_rbit does check T in {8b,16b}; encode_neon_not does not.
- encode_neon_not checks operands.len() < 2; extra ignored; source arrangement discarded; Q=1 iff arr_d=="16b" else 0 (no 8b check).
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (uppercase V alt-spellings).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_not_*.md.

# Confirmed invariants (encode_cnt)

- Valid CNT Vd.T, Vn.T with T in {8b,16b}, v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T=16b, bits[29:24]=001110, size bits[23:22]=00, bits[21:16]=100000, bits[15:10]=010110, Rn at [9:5], Rd at [4:0] = 0x0e205800 | (Q<<30) | (Rn<<5) | Rd.
- Arity 0–1 returns Err (1000 cases).
- Uppercase V prefix with lowercase T matches llvm-mc (1000 cases).
- Known-answer: `cnt v0.8b, v1.8b` = 0x0e205820; `cnt v0.16b, v1.16b` = 0x4e205820; `cnt v31.8b, v31.8b` = 0x0e205bff; `cnt v31.16b, v0.16b` = 0x4e20581f.
- Extra operand, T∉{8b,16b}, mismatched T, and GPR/SP/bare-V/FP currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_cnt)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. Range/error contract: aarch64-linux-gnu-as (README claims gas).
- ARM ARM Advanced SIMD two-register miscellaneous (CNT): 0 Q 00 1110 size=00 100000 010110 Rn Rd. T in {8B,16B} only. Q=1 iff T=16B.
- Dispatch: encoder/mod.rs:517 `"cnt" => encode_cnt`.
- Sibling encode_neon_not / encode_neon_rbit are different two-misc opcodes, not same-job differentials.
- encode_cnt checks operands.len() < 2; extra ignored; `_arr_n` unused; Q=1 iff arr_d=="16b" else 0 (no 8b check).
- Parser lowercases arrangement; parse_reg_num lowercases V/X/W prefixes.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. 1000 cases.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (uppercase V alt-spellings).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_cnt_*.md.

# Confirmed invariants (encode_neon_mvni)

- Valid MVNI Vd.T, #imm with T in {4h,8h} (no shift) and T in {2s,4s} with LSL {0,8,16,24}, v0–v31, imm8 0–255 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid MVNI Vd.{2s,4s}, #imm8, msl #{8,16} matches llvm-mc (1000 cases).
- Changing only Rd differs only in bits[4:0] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T in {8h,4s}, op at 29 = 1, bits[28:24]=01111, bit23=0, bits[22:19]=0, abc at [18:16], cmode at [15:12], o2=0 at 11, bit10=1, defgh at [9:5], Rd at [4:0].
- Arity 0–1 returns Err (1000 cases).
- Illegal T (including 8b/16b/2d), 2s/4s LSL amount not in {0,8,16,24}, and 2s/4s MSL amount not in {8,16} return Err (1000 cases).
- Known-answer: `mvni v0.4s, #0` = 0x6f000400; `mvni v0.2s, #0` = 0x2f000400; `mvni v0.4s, #255` = 0x6f0707e0; `mvni v31.4s, #0xaa` = 0x6f05055f; `mvni v0.4s, #1, lsl #8` = 0x6f002420; `mvni v0.4s, #1, lsl #16` = 0x6f004420; `mvni v0.4s, #1, lsl #24` = 0x6f006420; `mvni v0.2s, #1` = 0x2f000420; `mvni v0.8h, #1` = 0x6f008420; `mvni v0.4h, #1` = 0x2f008420; `mvni v0.2s, #0, msl #8` = 0x2f00c400; `mvni v0.4s, #1, msl #16` = 0x6f00d420.
- 4h/8h LSL #8, extra operands, out-of-range imm, illegal H LSL amount, and LSR currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_mvni)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. Range/error contract: aarch64-linux-gnu-as (README claims gas).
- ARM ARM Advanced SIMD modified immediate (MVNI): 0 Q 1 01111 00000 abc cmode o2 1 defgh Rd. T in {4H,8H,2S,4S} only. Q=1 iff T in {8H,4S}. op=1 always. cmode 10x0 (H, x=shift/8), 0xx0 (S LSL, xx=shift/8), 110x (S MSL, x=(amount==16)).
- Dispatch: encoder/mod.rs:961 `"mvni" => encode_neon_mvni`.
- Sibling encode_neon_movi is inverted immediate (op=0, extra 8B/16B/2D), not a same-job differential.
- encode_neon_mvni checks operands.len() < 2; extra ignored; imm8 via `imm as u32 & 0xFF`; 4h/8h hard-codes cmode=1000; non-lsl/non-msl Shift on 2s/4s encodes as no-shift.
- Parser lowercases arrangement; Shift tokens are lsl/lsr/asr/ror only (parser.rs) — MSL is not produced by the parser but the encoder implements it and llvm-mc accepts it.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. Co-generate (T, shift, imm).
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (invalid T / illegal 2s/4s LSL / illegal MSL).
- Five failing property/regression groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_mvni_*.md.

# Confirmed invariants (encode_neon_movi)

- Valid MOVI Vd.T, #imm with T in {8b,16b,4h,8h} (no shift), T in {2s,4s} with LSL {0,8,16,24}, and T=2d with each byte 0x00 or 0xFF, v0–v31, imm8 0–255 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on KAT vectors.
- Changing only Rd differs only in bits[4:0] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T in {16b,8h,4s,2d}, op at 29 = 1 iff T=2d, bits[28:24]=01111, bit23=0, bits[22:19]=0, abc at [18:16], cmode at [15:12], o2=0 at 11, bit10=1, defgh at [9:5], Rd at [4:0].
- Arity 0–1 returns Err (1000 cases).
- Illegal T, 2s/4s LSL amount not in {0,8,16,24}, and 2d bytes other than 0x00/0xFF return Err (1000 cases).
- Known-answer: `movi v0.16b, #0` = 0x4f00e400; `movi v0.8b, #0` = 0x0f00e400; `movi v0.16b, #255` = 0x4f07e7e0; `movi v31.16b, #0xaa` = 0x4f05e55f; `movi v0.4s, #0` = 0x4f000400; `movi v0.4s, #1, lsl #8` = 0x4f002420; `movi v0.4s, #1, lsl #16` = 0x4f004420; `movi v0.4s, #1, lsl #24` = 0x4f006420; `movi v0.2s, #1` = 0x0f000420; `movi v0.8h, #1` = 0x4f008420; `movi v0.4h, #1` = 0x0f008420; `movi v0.2d, #0` = 0x6f00e400; `movi v0.2d, #-1` = 0x6f07e7e0; `movi v0.2d, #0xff00000000000000` = 0x6f04e400.
- 4h/8h LSL #8, 2s/4s MSL, extra operands, and 8-bit imm outside [0,255] currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_movi)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. Range/error contract: aarch64-linux-gnu-as (README claims gas).
- ARM ARM Advanced SIMD modified immediate (MOVI): 0 Q op 01111 00000 abc cmode o2 1 defgh Rd. T in {8B,16B,4H,8H,2S,4S,2D}. Q=1 iff T in {16B,8H,4S,2D}. op=1 iff T=2D. cmode 1110 (8B/16B/2D), 10x0 (H, x=shift/8), 0xx0 (S LSL, xx=shift/8), 110x (S MSL, x=(amount==16)).
- Dispatch: encoder/mod.rs:687 `"movi" => encode_neon_movi`.
- Sibling encode_neon_mvni is inverted immediate (op=1 on 2S/4S/4H/8H), not a same-job differential. It does implement MSL.
- encode_neon_movi checks operands.len() < 2; extra ignored except 2s/4s LSL peek; imm8 via `imm as u32 & 0xFF`; 4h/8h hard-codes cmode=1000; non-lsl Shift on 2s/4s encodes as no-shift.
- Parser lowercases arrangement; Shift tokens are lsl/lsr/asr/ror only (parser.rs:1885) — MSL is not produced by the parser.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. Co-generate (T, shift, imm) — 2d imm is expanded from 8 bits to 0x00/0xFF bytes.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (invalid T / illegal LSL amount / bad 2d byte).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_movi_*.md.

# Confirmed invariants (encode_neon_ext)

- Valid EXT Vd.T, Vn.T, Vm.T, #i with T in {8b,16b}, matching arrangements, i in [0, imax(T)], v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on KAT vectors.
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; only Rm in bits[20:16] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff T=16b, bits[29:24]=101110, bits[23:21]=000, Rm at [20:16], bit15=0, imm4 at [14:11], bit10=0, Rn at [9:5], Rd at [4:0].
- Uppercase V/T spellings match llvm-mc (1000 cases).
- Arity 0–3 returns Err (1000 cases).
- Known-answer: `ext v0.16b, v1.16b, v2.16b, #3` = 0x6e021820; `ext v0.8b, v1.8b, v2.8b, #3` = 0x2e021820; `ext v0.8b, v1.8b, v2.8b, #0` = 0x2e020020; `ext v31.16b, v30.16b, v29.16b, #15` = 0x6e1d7bdf; `ext v31.8b, v0.8b, v31.8b, #7` = 0x2e1f381f; `ext v0.16b, v1.16b, v2.16b, #0` = 0x6e020020.
- Extra operand, invalid T, out-of-range index, mismatched T, and GPR/bare-V dest currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_ext)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. Range/error contract: aarch64-linux-gnu-as (README claims gas). llvm-mc wraps OOR index (8B mod 8, 16B mod 16); gas rejects OOR.
- ARM ARM Advanced SIMD extract (EXT): 0 Q 10 1110 00 0 Rm 0 imm4 0 Rn Rd. T in {8B,16B}. Q=1 iff T=16B. Index 0–7 (8B) / 0–15 (16B). Q=0 and imm4<3>!=0 is UNALLOCATED.
- Dispatch: encoder/mod.rs:675 `"ext" => encode_neon_ext`.
- Sibling encode_neon_tbl / encode_neon_tbx / encode_neon_zip_uzp are different opcodes, not same-job differentials.
- encode_neon_ext checks operands.len() < 4; extra ignored; Q=1 iff arr_d=="16b"; source arrangements discarded; index as u32 then & 0xF; get_neon_reg accepts Operand::Reg; parse_reg_num accepts w/x/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases arrangement (parser.rs:1970) so uppercase T is not caller-reachable as a distinct token; register names keep original case and parse_reg_num lowercases.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. Co-generate (T, index) — independent 0..=15 with prop_assume vs imax(8b)=7 exhausts global rejects.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (mismatched T / GPR-or-bare dest).
- Five failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_ext_*.md.

# Confirmed invariants (encode_neon_umov)

- Valid UMOV Wd, Vn.Ts[i] with Ts in {b,h,s}, i in [0, imax(Ts)], v0–v31, W0–W30/WZR and UMOV Xd, Vn.D[i] with i in [0,1], X0–X30/XZR matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; changing only Rn differs only in bits[9:5] (1000 cases).
- Success-path word: bit31=0, Q at bit30 = 1 iff Ts=D, bit29=0, bits[28:21]=01110000, imm5 at [20:16], bits[15:10]=001111, Rn at [9:5], Rd at [4:0].
- Uppercase W/X/V spellings match llvm-mc (1000 cases).
- Arity 0/1, invalid dest/src names (x32/foo/empty/w), unsupported elem_size {q,8b,16b,4h,empty,x}, and non-RegLane second operand (bare vN, vN.8b, Imm) return Err (1000 cases).
- Known-answer: `umov w0, v0.b[0]` = 0x0e013c00; `umov w0, v0.h[0]` = 0x0e023c00; `umov w0, v0.s[0]` = 0x0e043c00; `umov x0, v0.d[0]` = 0x4e083c00; `umov w0, v0.b[15]` = 0x0e1f3c00; `umov x31, v31.d[1]` = 0x4e183fff; `umov wzr, v0.b[0]` = 0x0e013c1f; `umov w1, v2.h[7]` = 0x0e1e3c41; `umov w3, v4.s[3]` = 0x0e1c3c83; `umov x5, v6.d[1]` = 0x4e183cc5.
- Extra operand, out-of-range lane, wrong dest width, SP/WSP, and FP-as-GPR dest currently disagree with llvm-mc (see bugs).

## Environment (encode_neon_umov)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. llvm-mc accepts `umov` and disassembles S/D forms as `mov` (UMOV alias).
- ARM ARM Advanced SIMD copy (UMOV): 0 Q 0 01110 000 imm5 001111 Rn Rd. Wd+B/H/S Q=0; Xd+D Q=1.
- Dispatch: encoder/mod.rs:679 `"umov" => encode_neon_umov`.
- Sibling encode_neon_dup / encode_neon_ins / encode_mov are different opcodes or a multi-form alias encoder, not same-job differentials.
- encode_neon_umov checks operands.len() < 2; extra ignored; index bits masked; Q from dest is_64; parse_reg_num accepts w/x/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31.
- Parser lowercases elem_size (parser.rs:1945) so uppercase Ts is not caller-reachable; register names keep original case and parse_reg_num lowercases.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. Co-generate (ts, index) — independent 0..=15 with prop_assume vs imax(d)=1 exhausts global rejects.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (unsupported elem_size / non-lane src).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_umov_*.md.

# Confirmed invariants (encode_neon_ins)

- Valid INS (general) Vd.Ts[i], Wn|Xn|WZR|XZR with Ts in {b,h,s,d}, i in [0, imax(Ts)], v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Wn for B/H/S, Xn for D.
- Valid INS (element) Vd.Ts[di], Vn.Ts[si] with matching Ts and in-range lanes matches llvm-mc (1000 cases).
- Changing only Rd differs only in bits[4:0]; changing only Rn differs only in bits[9:5] (1000 cases).
- Success-path general word: bit31=0, bit30=1, bit29=0, bits[28:21]=01110000, imm5 at [20:16], bits[15:10]=000111, Rn at [9:5], Rd at [4:0]. Element form: bit29=1, bit15=0, imm4 at [14:11], bit10=1.
- Uppercase V/W/X spellings match llvm-mc (1000 cases).
- Arity 0/1 and invalid dest names (v32/foo/empty/v) return Err (1000 cases).
- Known-answer: `ins v0.b[0], w1` = 0x4e011c20; `ins v0.h[0], w1` = 0x4e021c20; `ins v0.s[0], w1` = 0x4e041c20; `ins v0.d[0], x1` = 0x4e081c20; `ins v0.b[15], w1` = 0x4e1f1c20; `ins v31.d[1], x30` = 0x4e181fdf; `ins v0.b[0], wzr` = 0x4e011fe0; `ins v0.b[0], v1.b[0]` = 0x6e010420; `ins v0.h[3], v2.h[1]` = 0x6e0e1440; `ins v0.d[1], v4.d[0]` = 0x6e180480.
- Extra operand, out-of-range lane, wrong GPR width, SP/WSP, FP-as-GPR, and mismatched element sizes currently disagree with llvm-mc (see bugs).

## Environment (encode_neon_ins)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. llvm-mc accepts `ins` and disassembles it as `mov` (INS alias).
- ARM ARM Advanced SIMD INS (general): 0 1 0 01110 000 imm5 000111 Rn Rd. INS (element): 0 1 1 01110 000 imm5 0 imm4 1 Rn Rd.
- Dispatch: encoder/mod.rs:679 `"ins" => encode_neon_ins`.
- Sibling encode_neon_dup / encode_neon_umov are different opcodes, not same-job differentials.
- encode_neon_ins checks operands.len() < 2; extra ignored; index bits masked; parse_reg_num accepts w/x/d/s/q/v/h/b and maps sp/wsp/xzr/wzr to 31; `_src_size` discarded.
- Parser lowercases elem_size (parser.rs:1945) so uppercase Ts is not caller-reachable; register names keep original case and parse_reg_num lowercases.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered. Co-generate (ts, index) — independent 0..=15 with prop_assume vs imax(d)=1 exhausts global rejects.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (arity / alt-spellings).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_ins_*.md.

# Confirmed invariants (encode_neon_tbx)

- Valid vector TBX with Ta in {8b,16b}, Vd/Vm in v0–v31, 1–4 consecutive wrapping table registers all .16B matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Ta=8b XOR Ta=16b at equal Rd/Rn/Rm/len = 1<<30 (1000 cases).
- Changing only nregs in {1,2,3,4} differs only in len bits [14:13]; len = nregs-1 (1000 cases).
- Success-path word: bit 31=0, Q at 30, bits [29:24]=001110, bits [23:21]=000, Rm at [20:16], bit 15=0, len at [14:13], op=1 at 12, bits [11:10]=00, Rn at [9:5], Rd at [4:0].
- Uppercase V/T spellings match llvm-mc (1000 cases).
- Known-answer: `tbx v0.8b, {v1.16b}, v2.8b` encodes as 0x0e021020; `tbx v0.16b, {v1.16b}, v2.16b` as 0x4e021020; 2-reg 0x0e033020; 3-reg 0x4e045020; 4-reg 0x0e057020; wrap `{v31.16b, v0.16b}` as 0x0e0233e0.

## Environment (encode_neon_tbx)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD table lookup TBX: `0 Q 00 1110 00 0 Rm 0 len op 00 Rn Rd` with op=1. Ta in {8B,16B}. Table is 1–4 consecutive .16B registers wrapping at 31. Vm.Ta matches Vd.Ta. Q=1 iff Ta=16B.
- Dispatch: encoder/mod.rs:736 `"tbx" => encode_neon_tbx`. Sibling encode_neon_tbl is TBL (op=0), different job.
- Callers: assembler README NEON permute table lists tbl/tbx.
- Parser `parser.rs:2030-2072` builds Operand::RegList; rejects empty lists; range syntax expands wrapping consecutives. Encoder still panics if given an empty list directly.
- encode_neon_tbx checks operands.len() < 3; extra ignored; Q=1 iff arr_d=="16b"; only regs[0] and len are encoded; (num_regs-1)&0x3 wraps n>4; get_neon_reg accepts Operand::Reg; Vm arrangement discarded.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit (list/Vm kinds / alt-spellings).
- Ten failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_tbx_*.md.

# Confirmed invariants (encode_neon_ld_st_multi)

- Valid LD/ST multiple-structure (n in {1,2,3,4}, T in {8b,16b,4h,8h,2s,4s,1d,2d} for n=1 and {8b,16b,4h,8h,2s,4s,2d} for n≥2, consecutive wrapping v0–v31, Xn|SP base, no-offset, legal immediate post-index #n_regs*(Q?16:8), and register post-index Xm) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on KAT vectors.
- Success-path word is ARM AdvSIMD load/store multiple structures: 0 Q 001100 L(bit22) post(bit23) Rm opcode size Rn Rt. No-offset Rm=00000 bit23=0; imm post-index Rm=11111 bit23=1; register post-index Rm=Xm bit23=1. L=1 load / 0 store. Opcode: LD1/ST1 1/2/3/4 regs = 0111/1010/0110/0010; LD2=1000; LD3=0100; LD4=0000.
- Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; load vs store flips only L bit 22 (1000 cases).
- Fewer than 2 operands, non-RegList dest, non-Mem second operand, Imm-as-second, swapped operands, and [Xn, #imm] always Err (1000 cases).
- Known-answer: llvm-mc `ld1 {v0.16b}, [x0]` = 0x4c407000; `ld1 {v0.8b}, [x1]` = 0x0c407020; `st1 {v0.16b}, [x0]` = 0x4c007000; `ld1 {v0.4s, v1.4s}, [x2]` = 0x4c40a840; `ld2 {v0.16b, v1.16b}, [x1]` = 0x4c408020; `ld3 {v0.8h, v1.8h, v2.8h}, [x2]` = 0x4c404440; `ld4 {v0.4s, v1.4s, v2.4s, v3.4s}, [x3]` = 0x4c400860; `ld1 {v0.16b}, [x1], #16` = 0x4cdf7020; `ld1 {v0.16b}, [sp]` = 0x4c4073e0; `ld1 {v31.2d}, [x30]` = 0x4c407fdf; `ld2 {v31.16b, v0.16b}, [x1]` = 0x4c40803f; `ld1 {v0.1d}, [x0]` = 0x0c407c00; `ld1 {v0.16b}, [x1], x2` = 0x4cc27020.
- Extra operand, W/XZR/x31/FP base, LD2/3/4 wrong list length, uppercase arrangement, non-consecutive lists, illegal post-index #imm, and .1d on LD2/3/4 currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_ld_st_multi)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM AdvSIMD load/store multiple structures: 0 Q 001100 L post Rm opcode size Rn Rt; no-offset Rm=00000; imm post-index Rm=11111 bit23=1; register post-index Rm=Xm bit23=1.
- Dispatch: encoder/mod.rs:735-742 ld1-4/st1-4 => encode_neon_ld_st_dispatch; neon.rs:889-896 RegList => encode_neon_ld_st_multi.
- Callers: encoder dispatch only.
- Sibling encode_neon_ld_st_single is single-structure/element (different first-operand kind). Sibling encode_neon_ld1r / encode_neon_ldnr are replicate class.
- encode_neon_ld_st_multi checks operands.len() < 2; extra non-Imm/Reg ignored; Mem { offset: 0 } only; MemPostIndex always Rm=11111 ignoring offset; parse_reg_num accepts w/d/s/q/v/h/b and maps xzr/x31/sp to 31; neon_arr_to_q_size is lowercase-only; only regs[0] supplies Rt; LD2/3/4 opcode ignores list length.
- Parser rejects empty register lists (parser.rs:2069-2071); empty RegList therefore not caller-reachable.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries). Sweep was a manual arm audit (reg-post / alt-spellings / nonconsecutive / bad #imm / Mem offset / .1d on LD2-4).
- Seven failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_ld_st_multi_*.md.

# Confirmed invariants (encode_neon_ld_st_single)

- Valid LD/ST single-structure (n in {1,2,3,4}, sz in {b,h,s,d}, in-range lane, consecutive wrapping v0–v31, Xn|SP base, no-offset and legal immediate post-index #n*esize) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on KAT vectors.
- Success-path word is ARM AdvSIMD load/store single structure: Q 0011010 L R Rm opcode S size Rn Rt. No-offset Rm=00000 bit23=0; imm post-index Rm=11111 bit23=1. L=1 load / 0 store. R=0 for 1,3; R=1 for 2,4. opcode 000/010/100 (.B/.H/.S|.D) for n<=2, 001/011/101 for n>=3; .D uses size=01 S=0.
- Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; load vs store flips only L bit 22; n=1 vs n=2 flips only R bit 21 (1000 cases).
- Fewer than 2 operands, non-RegListIndexed dest, non-Mem second operand, list length != n, unsupported size, invalid names (foo/v32/x32/r0/empty), and [Xn, #imm] always Err (1000 cases).
- Known-answer: llvm-mc `st1 {v0.s}[0], [x3]` = 0x0d008060; `ld1 {v0.b}[0], [x1]` = 0x0d400020; `ld1 {v0.h}[0], [x1]` = 0x0d404020; `ld1 {v0.s}[0], [x1]` = 0x0d408020; `ld1 {v0.d}[0], [x1]` = 0x0d408420; `ld1 {v0.b}[15], [x1]` = 0x4d401c20; `ld2 {v0.s, v1.s}[0], [x3]` = 0x0d608060; `ld3 {v0.s, v1.s, v2.s}[0], [x3]` = 0x0d40a060; `ld4 {v0.s, v1.s, v2.s, v3.s}[0], [x3]` = 0x0d60a060; `st2 {v0.s, v1.s}[0], [x3]` = 0x0d208060; `ld1 {v0.s}[0], [x1], #4` = 0x0ddf8020; `ld1 {v0.s}[0], [sp]` = 0x0d4083e0; `ld1 {v31.d}[1], [x30]` = 0x4d4087df; `ld2 {v31.s, v0.s}[0], [x1]` = 0x0d60803f; `ld1 {v0.s}[0], [x1], x2` = 0x0dc28020.
- Extra operand, W/XZR/x31/FP base, uppercase arrangement, out-of-range lane, non-consecutive lists, register post-index, and illegal post-index #imm currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_ld_st_single)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM AdvSIMD load/store single structure: Q 0011010 L R Rm opcode S size Rn Rt; no-offset Rm=00000; imm post-index Rm=11111 bit23=1; register post-index Rm=Xm bit23=1.
- Dispatch: encoder/mod.rs:733-741 ld1-4/st1-4 => encode_neon_ld_st_dispatch; neon.rs:889-892 RegListIndexed => encode_neon_ld_st_single.
- Callers: encoder dispatch only.
- Sibling encode_neon_ld_st_multi is multiple-structures (different first-operand kind). Sibling encode_neon_ld1r / encode_neon_ldnr are replicate class.
- encode_neon_ld_st_single checks only operands.len() < 2 (extra ignored unless Imm); Mem { offset: 0 } only; MemPostIndex always Rm=11111 ignoring offset; parse_reg_num accepts w/d/s/q/v/h/b and maps xzr/x31/sp to 31; arrangement match is lowercase-only; index bits are masked with no range check; only regs[0] supplies Rt.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (alt-spellings / index-oor / nonconsecutive / register post-index / illegal #imm / Mem offset).
- Seven failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_ld_st_single_*.md.

# Confirmed invariants (encode_neon_ld1r)

- Valid LD1R (T in {8b,16b,4h,8h,2s,4s,1d,2d}, v0–v31, Xn|SP base, no-offset and immediate post-index #esize) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path LD1R word is ARM AdvSIMD replicate: 0 Q 001101 L R=1 S=0 Rm opcode=110 size Rn Rt with bit12=0, bit21=0. No-offset L=0 Rm=0; imm post-index L=1 Rm=11111.
- Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; 8b vs 16b (4h vs 8h, 2s vs 4s, 1d vs 2d) flips only Q bit 30 (1000 cases).
- Fewer than 2 operands, non-RegList dest, non-Mem second operand, list length != 1, unsupported T, invalid names (foo/x32/v32/r0/empty), and [Xn, #imm] always Err (1000 cases).
- Uppercase V/X spellings match llvm-mc (1000 cases).
- Known-answer: llvm-mc `ld1r {v0.8b}, [x1]` = 0x0d40c020; `ld1r {v0.16b}, [x1]` = 0x4d40c020; `ld1r {v0.4h}, [x1]` = 0x0d40c420; `ld1r {v0.2d}, [x1]` = 0x4d40cc20; `ld1r {v0.8b}, [x1], #1` = 0x0ddfc020; `ld1r {v0.8b}, [sp]` = 0x0d40c3e0; `ld1r {v31.2d}, [x30]` = 0x4d40cfdf; `ld1r {v0.8b}, [x1], x2` = 0x0dc2c020.
- Extra operand, W/XZR/x31/FP base, register post-index, and illegal post-index #imm currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_ld1r)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM AdvSIMD load/store single structure (replicate): 0 Q 001101 L R=1 S=0 Rm opcode=110 size Rn Rt; no-offset Rm=00000 L=0; imm post-index Rm=11111 L=1; register post-index Rm=Xm L=1.
- Dispatch: encoder/mod.rs:732 `"ld1r" => encode_neon_ld1r(operands)`.
- Callers: encoder dispatch only.
- Sibling encode_neon_ldnr is LD2R/LD3R/LD4R (different mnemonic). Sibling encode_neon_ld_st_single / encode_neon_ld_st_multi are different ARM classes.
- encode_neon_ld1r checks only operands.len() < 2 (extra ignored); Mem { offset: 0 } only; MemPostIndex always Rm=11111 ignoring offset; parse_reg_num accepts w/d/s/q/v/h/b and maps xzr/x31/sp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (register post-index / illegal #imm / alt spellings / invalid names / Mem offset).
- Four failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_ld1r_*.md.

# Confirmed invariants (encode_neon_ldnr)

- Valid LD3R (T in {8b,16b,4h,8h,2s,4s,1d,2d}, consecutive wrapping v0–v31, Xn|SP base, no-offset and immediate post-index #3*esize) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on KAT vectors.
- Success-path LD3R word is ARM AdvSIMD replicate: 0 Q 001101 L 1 0 Rm opcode=111 size Rn Rt with bit12=0, bit21=0. No-offset L=0 Rm=0; imm post-index L=1 Rm=11111.
- Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; 8b vs 16b (4h vs 8h, 2s vs 4s, 1d vs 2d) flips only Q bit 30 (1000 cases, n in {2,3,4}).
- Fewer than 2 operands, non-RegList dest, non-Mem second operand, wrong list length, unsupported T, and invalid names (foo/x32/v32/r0/empty) always Err (1000 cases).
- Known-answer: llvm-mc `ld2r {v0.8b, v1.8b}, [x1]` = 0x0d60c020; `ld3r {v0.8b, v1.8b, v2.8b}, [x1]` = 0x0d40e020 (SUT matches); `ld4r {v0.8b, v1.8b, v2.8b, v3.8b}, [x1]` = 0x0d60e020; `ld2r {v0.8b, v1.8b}, [x1], #2` = 0x0dffc020; `ld2r {v0.8b, v1.8b}, [x1], x2` = 0x0de2c020; `ld2r {v0.8b, v1.8b}, [sp]` = 0x0d60c3e0; `ld2r {v31.2d, v0.2d}, [x30]` = 0x4d60cfdf; SUT LD3R post `[x1], #3` = 0x0ddfe020.
- LD2R/LD4R encodings, extra operand, W/XZR/x31/FP base, register post-index, illegal post-index #imm, non-consecutive lists, and [Xn, #imm] currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_ldnr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on KAT vectors.
- ARM AdvSIMD load/store single structure (replicate): 0 Q 001101 L R=1 S Rm opcode size Rn Rt; opcode 110 (LD1R/LD2R) / 111 (LD3R/LD4R); S at bit 21 (1 for LD2R/LD4R).
- Dispatch: encoder/mod.rs:655-657 ld2r/ld3r/ld4r => encode_neon_ldnr(operands, 2/3/4). ld1r uses encode_neon_ld1r, not this function.
- Callers: encoder dispatch only.
- Sibling encode_neon_ld1r is LD1R (different mnemonic). Sibling encode_neon_ld_st_single / encode_neon_ld_st_multi are different ARM classes.
- encode_neon_ldnr checks only operands.len() < 2 (extra ignored); uses regs[0] + len only; Mem { base, .. } ignores offset; MemPostIndex always Rm=11111 ignoring offset; S placed at bit 12; parse_reg_num accepts w/d/s/q/v/h/b and maps xzr/x31/sp to 31.
- Parser rejects empty register lists (parser.rs:2069-2071); empty RegList therefore not caller-reachable.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (mixed arrangement / LD3R differential).
- Seven failing property groups are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_ldnr_*.md.

# Confirmed invariants (encode_ldrs)

- Valid unsigned LDRSB/LDRSH (Wt/Xt including wzr/xzr, Xn|SP base, imm12 in 0..4095, scale 1 or 2) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid unscaled / pre-index / post-index with simm9 in [-256,255] (excluding unpredictable Rt==Rn writeback) matches llvm-mc (1000 cases).
- Valid register-offset (lsl/sxtx/uxtw/sxtw, amount 0 for byte and 0/1 for half) matches llvm-mc including option/S (1000 cases).
- Alternate spellings x31/w31, uppercase, lr match llvm-mc (1000 cases).
- Success-path word is ARM LDRSB/LDRSH: size 111 V=0 opc; unsigned [25:24]=01 imm12; unscaled [25:24]=00 bit21=0 [11:10]=00 imm9; pre [11:10]=11; post [11:10]=01; regoff bit21=1 [11:10]=10. opc=10 Xt / 11 Wt; size=00 byte / 01 half.
- Metamorphic: Rt+1 adds 1; Rn+1 adds 32; imm12+1 adds 1<<10; pre XOR post = 0b10<<10; Xt vs Wt flips bit 22; ldrsb vs ldrsh flips bit 30 (1000 cases).
- Fewer than 2 operands, non-memory 2nd operand, and invalid names (foo/x32/w32/empty/r0/x) always Err (1000 cases).
- Known-answer: `ldrsb x0, [x1]` = 0x39800020; `ldrsb w0, [x1]` = 0x39c00020; `ldrsh x0, [x1]` = 0x79800020; `ldrsh w0, [x1]` = 0x79c00020; `ldrsb x0, [x1, #4095]` = 0x39bffc20; `ldrsh x0, [x1, #8190]` = 0x79bffc20; `ldrsb x0, [x1, #-1]` = 0x389ff020; `ldrsb x0, [x1, #4]!` = 0x38804c20; `ldrsb w0, [x1], #4` = 0x38c04420; `ldrsh x0, [x1, x2]` = 0x78a26820.
- Extra operand, SP/WSP dest, SIMD dest, W base, XZR base, W index without extend, writeback Rt==Rn, out-of-range offset, and illegal shift currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_ldrs)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM LDRSB/LDRSH unsigned: size 111 V=0 01 opc imm12 Rn Rt; unscaled LDURSB/LDURSH bits[25:24]=00 bits[11:10]=00 simm9 [-256,255]; pre 11, post 01; register offset bit21=1 option S bits[11:10]=10.
- Dispatch: encoder/mod.rs:334-335 `ldrsb` => encode_ldrs(operands, 0b00); `ldrsh` => encode_ldrs(operands, 0b01).
- Callers: encoder dispatch only.
- Sibling encode_ldrsw / encode_ldr_str / encode_ldur_stur are different opcodes/jobs, not same-job differentials.
- encode_ldrs checks only operands.len() < 2 (extra ignored); parse_reg_num accepts any x/w/d/s/q/v/h/b prefix and maps sp/xzr to 31; out-of-range offsets are masked to imm9; S bit is shift_amount > 0 with no scale check.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid names).
- Nine failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldrs_*.md.

# Confirmed invariants (encode_neon_dup)

- Valid DUP GPR form (T in {8b,16b,4h,8h,2s,4s,2d}, v0–v31, Wn including wzr for T≠2d, Xn including xzr for T=2d) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid DUP element form with matching T/size and in-range index matches llvm-mc (1000 cases).
- Success-path word is ARM Advanced SIMD DUP: 0 Q 0 01110 000 imm5 opc Rn Rd. GPR opc=000011; element opc=000001. Q(16b/8h/4s/2d)=1 else 0. imm5 size 00001/00010/00100/01000; element imm5 encodes index in the high bits of the size marker.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; 8b vs 16b (4h vs 8h, 2s vs 4s) flips only Q bit 30; GPR vs element flips only bit 11 (1000 cases).
- Fewer than 2 operands, non-register kinds, invalid names, unsupported T, dest without arrangement, invalid RegLane names, and unsupported elem_size always Err (1000 cases).
- Known-answer: `dup v0.8b, w1` = 0x0e010c20; `dup v0.16b, w1` = 0x4e010c20; `dup v0.4s, w1` = 0x4e040c20; `dup v0.2d, x1` = 0x4e080c20; `dup v0.16b, v1.b[0]` = 0x4e010420; `dup v0.4s, v1.s[3]` = 0x4e1c0420; `dup v0.2d, v1.d[1]` = 0x4e180420; `dup v0.4s, wzr` = 0x4e040fe0; `dup V0.4S, W1` = 0x4e040c20.
- Extra operand, wrong-width GPR (X on 32-bit T / W on .2d), SP/WSP/FP-as-GPR, out-of-range lane index, and dest T vs element-size mismatch currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_neon_dup)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD DUP (general): 0 Q 0 01110 000 imm5 000011 Rn Rd; DUP (element): opcode 000001. T in {8B,16B,4H,8H,2S,4S,2D}; GPR Wn for T!=2D, Xn for T=2D.
- Dispatch: encoder/mod.rs:670 `"dup" => encode_neon_dup(operands)`.
- Callers: encoder dispatch only.
- Sibling encode_neon_umov / encode_neon_ins are different opcodes (001111 / 000111), not same-job differentials.
- encode_neon_dup checks only operands.len() < 2 (extra ignored); parse_reg_num accepts any x/w/d/s/q/v/h/b/sp prefix; element index is masked (`index & 0xF` etc.); dest T is used only for Q, elem_size independently encodes imm5.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid RegLane names / unsupported elem_size).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_dup_*.md.

# Confirmed invariants (encode_fsqrt)

- Valid scalar FSQRT (s0–s31 / d0–d31, including uppercase S/D) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word is ARM FP 1-source FSQRT: 0 00 11110 ftype 1 opcode=000011 10000 Rn Rd. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b000011<<15)|(0b10000<<10)|(rn<<5)|rd with ftype=01 for D else 00.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D flips only ftype bit 22 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/s32/d32/h32/x32/r0/s/d/empty) always Err (1000 cases).
- Known-answer: `fsqrt s0, s1` = 0x1e21c020; `fsqrt d0, d1` = 0x1e61c020; `fsqrt s31, s31` = 0x1e21c3ff; `fsqrt d31, d0` = 0x1e61c01f; `fsqrt S0, S1` = 0x1e21c020; llvm-mc fp16 `fsqrt h0, h1` = 0x1ee1c020.
- Extra operand, mixed S/D / GPR / QVB / SP, and H registers currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_fsqrt)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (+ `-mattr=+fullfp16` for H).
- ARM ARM Floating-point data-processing (1 source) FSQRT: M=0 S=0 11110 ftype 1 opcode=000011 10000 Rn Rd; ftype 00=S, 01=D, 11=H.
- Dispatch: encoder/mod.rs:411-413 scalar fsqrt (non-RegArrangement) => encode_fsqrt. Vector form goes to encode_neon_float_two_misc (out of this function's contract).
- Callers: encoder dispatch only.
- Sibling encode_fabs/encode_fneg are different opcodes (000001 / 000010), not same-job differentials. Sibling encode_fp_1src is FRINT*. Sibling encode_neon_float_two_misc is vector.
- encode_fsqrt does not check operands.len() (extra ignored); takes ftype from dest starts_with('d') only (H encoded as S; mixed S/D / GPR / SP / QVB accepted via parse_reg_num).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid names).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fsqrt_*.md.

# Confirmed invariants (encode_fneg)

- Valid scalar FNEG (s0–s31 / d0–d31, including uppercase S/D) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word is ARM FP 1-source FNEG: 0 00 11110 ftype 1 opcode=000010 10000 Rn Rd. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b000010<<15)|(0b10000<<10)|(rn<<5)|rd with ftype=01 for D else 00.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D flips only ftype bit 22 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/s32/d32/h32/x32/r0/s/d/empty) always Err (1000 cases).
- Known-answer: `fneg s0, s1` = 0x1e214020; `fneg d0, d1` = 0x1e614020; `fneg s31, s31` = 0x1e2143ff; `fneg d31, d0` = 0x1e61401f; `fneg S0, S1` = 0x1e214020; llvm-mc fp16 `fneg h0, h1` = 0x1ee14020.
- Extra operand, mixed S/D / GPR / QVB / SP, and H registers currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_fneg)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (+ `-mattr=+fullfp16` for H).
- ARM ARM Floating-point data-processing (1 source) FNEG: M=0 S=0 11110 ftype 1 opcode=000010 10000 Rn Rd; ftype 00=S, 01=D, 11=H.
- Dispatch: encoder/mod.rs:406-408 scalar fneg (non-RegArrangement) => encode_fneg. Vector form goes to encode_neon_float_two_misc (out of this function's contract).
- Callers: encoder dispatch only.
- Sibling encode_fabs/encode_fsqrt are different opcodes (000001 / 000011), not same-job differentials. Sibling encode_fp_1src is FRINT*. Sibling encode_neon_float_two_misc is vector.
- encode_fneg does not check operands.len() (extra ignored); takes ftype from dest starts_with('d') only (H encoded as S; mixed S/D / GPR / SP / QVB accepted via parse_reg_num).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid names).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fneg_*.md.

# Confirmed invariants (encode_fmadd_fmsub)

- Valid scalar FMADD/FMSUB (s0–s31 / d0–d31, including uppercase S/D) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word is ARM FP 3-source: 0 00 11111 ftype o1=0 Rm o0 Ra Rn Rd. Equivalently w = (0b00011111<<24)|(ftype<<22)|(rm<<16)|(o0<<15)|(ra<<10)|(rn<<5)|rd with ftype=01 for D else 00, o0=1 for FMSUB else 0.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; Ra+1 increments bits[14:10] only; Rm+1 increments bits[20:16] only; S vs D flips only ftype bit 22; FMADD XOR FMSUB is o0 bit 15 (1000 cases).
- Fewer than 4 operands, non-register kinds, and invalid names (foo/s32/d32/h32/x32/r0/s/d/empty) always Err (1000 cases).
- Known-answer: `fmadd s0, s1, s2, s3` = 0x1f020c20; `fmadd d0, d1, d2, d3` = 0x1f420c20; `fmsub s0, s1, s2, s3` = 0x1f028c20; `fmsub d0, d1, d2, d3` = 0x1f428c20; `fmadd s31, s31, s31, s31` = 0x1f1f7fff; `fmadd S0, S1, S2, S3` = 0x1f020c20; llvm-mc fp16 `fmadd h0, h1, h2, h3` = 0x1fc20c20.
- Extra operand, mixed S/D / GPR / QVB / SP, and H registers currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_fmadd_fmsub)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (+ `-mattr=+fullfp16` for H).
- ARM ARM Floating-point data-processing (3 source): M=0 S=0 11111 ftype o1 Rm o0 Ra Rn Rd; FMADD o1=0 o0=0, FMSUB o1=0 o0=1; ftype 00=S, 01=D, 11=H.
- Dispatch: encoder/mod.rs:435-436 `fmadd` => encode_fmadd_fmsub(operands, false); `fmsub` => encode_fmadd_fmsub(operands, true).
- Callers: encoder dispatch only.
- Sibling encode_fnmadd_fnmsub is o1=1 negated fused class, not a same-job differential. Sibling encode_fp_arith is 2-source. Sibling encode_madd is integer MADD.
- encode_fmadd_fmsub does not check operands.len() (extra ignored); takes ftype from dest starts_with('d') only (H encoded as S; mixed S/D / GPR / SP / QVB accepted via parse_reg_num).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid names).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fmadd_fmsub_*.md.

# Confirmed invariants (encode_fabs)

- Valid scalar FABS (s0–s31 / d0–d31, including uppercase S/D) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word is ARM FP 1-source FABS: 0 00 11110 ftype 1 opcode=000001 10000 Rn Rd. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b000001<<15)|(0b10000<<10)|(rn<<5)|rd with ftype=01 for D else 00.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D flips only ftype bit 22 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/s32/d32/h32/x32/r0/s/d/empty) always Err (1000 cases).
- Known-answer: `fabs s0, s1` = 0x1e20c020; `fabs d0, d1` = 0x1e60c020; `fabs s31, s31` = 0x1e20c3ff; `fabs d31, d0` = 0x1e60c01f; `fabs S0, S1` = 0x1e20c020; llvm-mc fp16 `fabs h0, h1` = 0x1ee0c020.
- Extra operand, mixed S/D / GPR / QVB / SP, and H registers currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_fabs)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (+ `-mattr=+fullfp16` for H).
- ARM ARM Floating-point data-processing (1 source) FABS: M=0 S=0 11110 ftype 1 opcode=000001 10000 Rn Rd; ftype 00=S, 01=D, 11=H.
- Dispatch: encoder/mod.rs:408-411 scalar fabs (non-RegArrangement) => encode_fabs. Vector form goes to encode_neon_float_two_misc (out of this function's contract).
- Callers: encoder dispatch only.
- Sibling encode_fneg/encode_fsqrt are different opcodes (000010 / 000011), not same-job differentials. Sibling encode_fp_1src is FRINT*. Sibling encode_neon_float_two_misc is vector.
- encode_fabs does not check operands.len() (extra ignored); takes ftype from dest starts_with('d') only (H encoded as S; mixed S/D / GPR / SP / QVB accepted via parse_reg_num).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid names).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fabs_*.md.

# Confirmed invariants (encode_neon_float_two_misc)

- Valid vector FP two-misc (T in {2s,4s,2d}, v0–v31 including V0/V31 uppercase) matches llvm-mc `-triple=aarch64 -show-encoding` for ARM-correct (U, size_hi, opcode) of fneg/fabs/fsqrt/frintn/p/m/z/a/x/i/fcvtzs/fcvtzu/ucvtf/scvtf/frecpe/frsqrte (1000 cases).
- Success-path word is ARM Advanced SIMD two-register miscellaneous: 0 Q U 01110 size 10000 opcode 10 Rn Rd with size=(size_hi<<1)|sz. Q(2s)=0, Q(4s)=1, Q(2d)=1; sz(2s)=0, sz(4s)=0, sz(2d)=1.
- Metamorphic: U toggles only bit 29; size_hi toggles only bit 23; 2s vs 4s toggles only Q (bit 30) (1000 cases).
- Unsupported T (8b/16b/4h/8h/1d/1s/3s/8s/empty/b/h), fewer than 2 operands, non-register kinds, invalid names (v32/foo/empty/v/v-1/v99), and dest Operand::Reg (no arrangement) always Err (1000 cases).
- Known-answer: `fneg v0.4s, v1.4s` = 0x6ea0f820; `fabs v0.4s, v1.4s` = 0x4ea0f820; `fsqrt v0.4s, v1.4s` = 0x6ea1f820; `fneg v0.2s, v1.2s` = 0x2ea0f820; `fneg v0.2d, v1.2d` = 0x6ee0f820; `ucvtf v0.4s, v1.4s` = 0x6e21d820; `fcvtzs v0.4s, v1.4s` = 0x4ea1b820.
- Extra operand, dest/src T mismatch, non-V prefixes, bare source, and SP/WSP/XZR/WZR/LR currently encode instead of Err (see bugs).

## Environment (encode_neon_float_two_misc)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD two-register miscellaneous (FP): 0 Q U 01110 size 10000 opcode 10 Rn Rd; T in {2S,4S,2D}; size[1]=size_hi, size[0]=sz.
- Dispatch: encoder/mod.rs:406-458 vector fneg/fabs/fsqrt/frint*/fcvtzs/fcvtzu/ucvtf/scvtf => encode_neon_float_two_misc. Note: dispatch passes size_hi=0 for fneg; ARM FNEG is size_hi=1. Out of this function's contract.
- Callers: encoder dispatch only.
- Sibling encode_neon_two_misc is integer two-misc (different size map). Sibling encode_neon_float_cmp_zero is compare-with-zero. Sibling encode_neon_float_three_same is three-register.
- encode_neon_float_two_misc does not check operands.len() (extra ignored); takes Q/sz from dest arrangement only; get_neon_reg accepts Operand::Reg and x/w/d/s/q/v/h/b prefixes; parse_reg_num maps sp/wsp/xzr/wzr to 31 and lr to 30.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (alt-spellings / SP aliases).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_float_two_misc_*.md.

# Confirmed invariants (encode_ubfx)

- Same-width GPR UBFX (x0–x30/xzr/lr and w0–w30/wzr, 0<=lsb<R, 1<=width<=R-lsb) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc may disassemble some encodings as LSR/UBFM aliases; the 32-bit word still matches.
- encode_ubfx([Rd, Rn, lsb, width]) equals encode_ubfm([Rd, Rn, lsb, lsb+width-1]) (ARM ARM UBFX alias of UBFM) (1000 cases).
- Success-path word is ARM Bitfield Move UBFM: sf 10 100110 N immr imms Rn Rd with N=sf, immr=lsb, imms=lsb+width-1. Equivalently w = (sf<<31)|(0b10<<29)|(0b100110<<23)|(sf<<22)|(lsb<<16)|((lsb+width-1)<<10)|(rn<<5)|rd. opc bits[30:29]=10.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Alternate spellings (x31/w31, XZR/WZR, LR, uppercase) match llvm-mc (1000 cases).
- Known-answer: `ubfx w0, w1, #0, #1` = 0x53000020; `ubfx w0, w1, #1, #1` = 0x53010420; `ubfx x0, x1, #1, #8` = 0xd3412020; `ubfx wzr, wzr, #31, #1` = 0x531f7fff; `ubfx x0, xzr, #63, #1` = 0xd37fffe0; `ubfx lr, x1, #8, #16` = 0xd3485c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_ubfx)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Unsigned Bitfield Extract UBFX: UBFX <Rd>, <Rn>, #<lsb>, #<width> aliases UBFM <Rd>, <Rn>, #<lsb>, #(<lsb>+<width>-1). Encoding sf 10 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. Constraints 0<=lsb<datasize, 1<=width<=datasize-lsb.
- Dispatch: encoder/mod.rs:885 "ubfx" => encode_ubfx(operands).
- Callers: encoder dispatch only. encode_ubfm_pbt uses encode_ubfx as algebraic UBFX alias, not as a unit test of encode_ubfx.
- Sibling encode_ubfm is the raw immr/imms form (not a same-job differential). Sibling encode_sbfx/encode_bfxil/encode_ubfiz are different opc or alias mapping.
- encode_ubfx does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31; lsb/width are `as u32` with no range check; `lsb + width - 1` overflows in debug on width=0.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ubfx (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / lsb-width).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ubfx_*.md.

# Confirmed invariants (encode_ubfm)

- Same-width GPR UBFM (x0–x30/xzr/lr and w0–w30/wzr, 0<=immr,imms<R) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc may disassemble some encodings as UBFX/UBFIZ/LSL/LSR aliases; the 32-bit word still matches.
- encode_ubfm([Rd, Rn, lsb, lsb+width-1]) equals encode_ubfx([Rd, Rn, lsb, width]) (ARM ARM UBFX alias of UBFM) (1000 cases).
- Success-path word is ARM Bitfield Move UBFM: sf 10 100110 N immr imms Rn Rd with N=sf. Equivalently w = (sf<<31)|(0b10<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd. opc bits[30:29]=10.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Alternate spellings (x31/w31, XZR/WZR, LR, uppercase) match llvm-mc (1000 cases).
- Known-answer: `ubfm w0, w1, #0, #0` = 0x53000020; `ubfm w0, w1, #1, #0` = 0x53010020; `ubfm x0, x1, #1, #8` = 0xd3412020; `ubfm wzr, wzr, #31, #0` = 0x531f03ff; `ubfm x0, xzr, #63, #63` = 0xd37fffe0; `ubfm lr, x1, #8, #16` = 0xd348403e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range immr/imms currently encode instead of Err (see bugs).

## Environment (encode_ubfm)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Bitfield Move UBFM: UBFM <Rd>, <Rn>, #<immr>, #<imms>. Encoding sf 10 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. Constraints 0<=immr,imms<datasize.
- Dispatch: encoder/mod.rs:887 "ubfm" => encode_ubfm(operands).
- Callers: encoder dispatch only. encode_uxtw_pbt uses encode_ubfm as algebraic UXTW alias, not as a unit test of encode_ubfm.
- Sibling encode_ubfx / encode_ubfiz are alias lsb/width forms (not same-job differentials). Sibling encode_sbfm/encode_bfm are different opc.
- encode_ubfm does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31; immr/imms are `as u32` with no range check.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ubfm (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / immr-imms).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ubfm_*.md.

# Confirmed invariants (encode_sbfx)

- Same-width GPR SBFX (x0–x30/xzr/lr and w0–w30/wzr, 0<=lsb<R, 1<=width<=R-lsb) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc may disassemble some encodings as ASR (when the extract reaches the MSB); the 32-bit word still matches.
- encode_sbfx([Rd, Rn, lsb, width]) equals encode_sbfm([Rd, Rn, lsb, lsb+width-1]) (ARM ARM SBFX alias of SBFM) (1000 cases).
- Success-path word: sf at 31 from Rd width, opc=00 at [30:29], bits [28:23]=100110, N=sf at 22, immr=lsb at [21:16], imms=lsb+width-1 at [15:10], Rn at [9:5], Rd at [4:0].
- Changing Rd does not change opcode/imm/Rn fields; changing Rn does not change opcode/imm/Rd.
- Fewer than 4 operands always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-imm (Shift/Mem/Symbol/Label/Cond/RegArrangement) in any of the four slots always Err (Imm allowed only in slots 2-3).
- Known-answer: `sbfx w0, w1, #0, #1` encodes as 0x13000020; `sbfx w0, w1, #1, #1` as 0x13010420; `sbfx x0, x1, #1, #8` as 0x93412020; `sbfx wzr, wzr, #31, #1` as 0x131f7fff; `sbfx x0, xzr, #63, #1` as 0x937fffe0; `sbfx lr, x1, #8, #16` as 0x93485c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_sbfx)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Signed Bitfield Extract SBFX: SBFX <Rd>, <Rn>, #<lsb>, #<width> aliases SBFM <Rd>, <Rn>, #<lsb>, #(<lsb>+<width>-1). Encoding sf 00 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. Constraints 0<=lsb<datasize, 1<=width<=datasize-lsb.
- Dispatch: encoder/mod.rs:886 "sbfx" => encode_sbfx(operands).
- Callers: encoder dispatch only.
- Sibling encode_sbfm is the raw immr/imms form (not a same-job differential). Sibling encode_ubfx/encode_bfxil are different opc.
- encode_sbfx does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31; lsb/width are `as u32` with no range check; `lsb + width - 1` overflows in debug on width=0.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_sbfx (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / lsb-width).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_sbfx_*.md.

# Confirmed invariants (encode_sbfm)

- Valid SBFM Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc may disassemble some encodings as SBFIZ/SBFX/ASR aliases; the 32-bit word still matches.
- Success-path word is ARM Bitfield Move SBFM: sf 00 100110 N immr imms Rn Rd with N=sf. Equivalently w = (sf<<31)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd. opc bits[30:29]=00.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `sbfm w0, w1, #0, #0` = 0x13000020; `sbfm w0, w1, #1, #0` = 0x13010020; `sbfm x0, x1, #1, #8` = 0x93412020; `sbfm wzr, wzr, #31, #0` = 0x131f03ff; `sbfm x0, xzr, #63, #63` = 0x937fffe0; `sbfm lr, x1, #8, #16` = 0x9348403e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range immr/imms currently encode instead of Err (see bugs).

## Environment (encode_sbfm)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Bitfield Move SBFM: SBFM <Rd>, <Rn>, #<immr>, #<imms>. Encoding sf 00 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. Constraints 0<=immr,imms<datasize.
- Dispatch: encoder/mod.rs:888 "sbfm" => encode_sbfm(operands).
- Callers: encoder dispatch only.
- Sibling encode_sbfiz / encode_sbfx are alias lsb/width forms (not same-job differentials). Sibling encode_ubfm/encode_bfm are different opc.
- encode_sbfm does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31; immr/imms are `as u32` with no range check.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_sbfm (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / immr-imms).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_sbfm_*.md.

# Confirmed invariants (encode_bfm)

- Valid BFM Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc may disassemble some encodings as BFI/BFXIL aliases; the 32-bit word still matches.
- Success-path word is ARM Bitfield Move BFM: sf 01 100110 N immr imms Rn Rd with N=sf. Equivalently w = (sf<<31)|(0b01<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd. opc bits[30:29]=01.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `bfm w0, w1, #0, #0` = 0x33000020; `bfm w0, w1, #1, #0` = 0x33010020; `bfm x0, x1, #1, #8` = 0xb3412020; `bfm wzr, wzr, #31, #0` = 0x331f03ff; `bfm x0, xzr, #63, #63` = 0xb37fffe0; `bfm lr, x1, #8, #16` = 0xb348403e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range immr/imms currently encode instead of Err (see bugs).

## Environment (encode_bfm)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Bitfield Move BFM: BFM <Rd>, <Rn>, #<immr>, #<imms>. Encoding sf 01 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. Constraints 0<=immr,imms<datasize.
- Dispatch: encoder/mod.rs:891 "bfm" => encode_bfm(operands).
- Callers: encoder dispatch only.
- Sibling encode_bfi / encode_bfxil are alias lsb/width forms (not same-job differentials). Sibling encode_ubfm/encode_sbfm are different opc.
- encode_bfm does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31; immr/imms are `as u32` with no range check.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_bfm (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / immr-imms).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_bfm_*.md.

# Confirmed invariants (encode_ubfiz)

- Valid UBFIZ Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc may disassemble some encodings as UBFX/LSL/LSR aliases; the 32-bit word still matches.
- Success-path word is ARM Bitfield Move UBFM: sf 10 100110 N immr imms Rn Rd with N=sf, immr=(-lsb MOD R), imms=width-1. Equivalently w = (sf<<31)|(0b10<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd. opc bits[30:29]=10.
- Algebraic alias: encode_ubfiz(Rd,Rn,#lsb,#width) = encode_ubfm(Rd,Rn,#(-lsb rem_euclid R),#(width-1)) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `ubfiz w0, w1, #0, #1` = 0x53000020; `ubfiz w0, w1, #1, #1` = 0x531f0020; `ubfiz x0, x1, #1, #8` = 0xd37f1c20; `ubfiz wzr, wzr, #31, #1` = 0x530103ff; `ubfiz x0, xzr, #63, #1` = 0xd34103e0; `ubfiz lr, x1, #8, #16` = 0xd3783c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_ubfiz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Bitfield Move UBFIZ alias of UBFM: UBFIZ <Rd>, <Rn>, #<lsb>, #<width> equivalent to UBFM <Rd>, <Rn>, #(-lsb MOD datasize), #(width-1). Encoding sf 10 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. Constraints 0<=lsb<datasize, 1<=width<=datasize-lsb.
- Dispatch: encoder/mod.rs:889 "ubfiz" => encode_ubfiz(operands).
- Callers: encoder dispatch only.
- Sibling encode_ubfm is the raw immr/imms form (algebraic alias after ARM mapping, not a same-job differential). Sibling encode_sbfiz/encode_ubfx/encode_bfi are different opc or alias mapping.
- encode_ubfiz does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31; `width - 1` panics in debug when width=0; immr uses wrapping_sub so lsb>=R encodes rather than Err.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ubfiz (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / lsb-width).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ubfiz_*.md.

# Confirmed invariants (encode_sbfiz)

- Valid SBFIZ Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). llvm-mc may disassemble some encodings as SBFX or ASR aliases; the 32-bit word still matches.
- Success-path word is ARM Bitfield Move SBFM: sf 00 100110 N immr imms Rn Rd with N=sf, immr=(-lsb MOD R), imms=width-1. Equivalently w = (sf<<31)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd. opc bits[30:29]=00.
- Algebraic alias: encode_sbfiz(Rd,Rn,#lsb,#width) = encode_sbfm(Rd,Rn,#(-lsb rem_euclid R),#(width-1)) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `sbfiz w0, w1, #0, #1` = 0x13000020; `sbfiz w0, w1, #1, #1` = 0x131f0020; `sbfiz x0, x1, #1, #8` = 0x937f1c20; `sbfiz wzr, wzr, #31, #1` = 0x130103ff; `sbfiz x0, xzr, #63, #1` = 0x934103e0; `sbfiz lr, x1, #8, #16` = 0x93783c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_sbfiz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Bitfield Move SBFIZ alias of SBFM: SBFIZ <Rd>, <Rn>, #<lsb>, #<width> equivalent to SBFM <Rd>, <Rn>, #(-lsb MOD datasize), #(width-1). Encoding sf 00 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. Constraints 0<=lsb<datasize, 1<=width<=datasize-lsb.
- Dispatch: encoder/mod.rs:890 "sbfiz" => encode_sbfiz(operands).
- Callers: encoder dispatch only.
- Sibling encode_sbfm is the raw immr/imms form (algebraic alias after ARM mapping, not a same-job differential). Sibling encode_ubfiz/encode_sbfx/encode_bfi are different opc or alias mapping.
- encode_sbfiz does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31; `width - 1` panics in debug when width=0; immr uses wrapping_sub so lsb>=R encodes rather than Err.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_sbfiz (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / lsb-width).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_sbfiz_*.md.

# Confirmed invariants (encode_rev)

- Valid REV Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `rev w0, w1` = 0x5ac00820.
- Success-path word is ARM Data-processing (1 source) REV: sf 1 0 11010110 00000 opc Rn Rd with opc=000010 (32-bit) / 000011 (64-bit). Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(opc<<10)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = (1<<31)|(1<<10) because opcode LSB also flips (unlike REV16/CLZ/RBIT) (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `rev w0, w1` = 0x5ac00820; `rev x0, x1` = 0xdac00c20; `rev wzr, wzr` = 0x5ac00bff; `rev xzr, xzr` = 0xdac00fff; `rev lr, x1` = 0xdac00c3e; `rev x0, xzr` = 0xdac00fe0.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_rev)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `rev w0, w1` = 0x5ac00820.
- ARM ARM Data-processing (1 source) REV: REV <Wd>, <Wn> / REV <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 opc Rn Rd; opc=000010 (W) / 000011 (X); register 31 is ZR not SP. Vector byte-reverse is rev16/rev32/rev64, not the `rev` mnemonic.
- Dispatch: encoder/mod.rs:910 scalar rev => encode_rev (no RegArrangement split).
- Callers: encoder dispatch only.
- Sibling encode_rev16/encode_rev32/encode_rbit/encode_clz/encode_cls are different opcodes. encode_neon_two_misc is the vector form for rev16/rev32/rev64 (not a differential sibling).
- encode_rev does not check operands.len() (extra ignored); takes sf/opc from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_rev (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_rev_*.md.

# Confirmed invariants (encode_rev32)

- Valid REV32 Xd,Xn including xzr, x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `rev32 x0, x1` = 0xdac00820. W form is not valid.
- Direct-call NEON path REV32 Vd.8b/16b/4h/8h, Vn.T matches llvm-mc (1000 cases). Public dispatch routes RegArrangement to encode_neon_two_misc, not encode_rev32.
- Success-path scalar word is ARM Data-processing (1 source) REV32: 1 1 0 11010110 00000 000010 Rn Rd. Equivalently w = (1<<31)|(1<<30)|(0b011010110<<21)|(0b000010<<10)|(rn<<5)|rd. sf always 1; bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000010.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `rev32 x0, x1` = 0xdac00820; `rev32 xzr, xzr` = 0xdac00bff; `rev32 lr, x1` = 0xdac0083e; `rev32 x0, xzr` = 0xdac00be0; `rev32 v0.8b, v1.8b` = 0x2e200820; `rev32 v0.16b, v1.16b` = 0x6e200820; `rev32 v0.4h, v1.4h` = 0x2e600820; `rev32 v0.8h, v1.8h` = 0x6e600820.
- Extra operand, SP/WSP, W registers, mixed W/X, FP/SIMD prefixes, and NEON T in {2s,4s,2d,1d} currently encode instead of Err (see bugs).

## Environment (encode_rev32)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `rev32 x0, x1` = 0xdac00820.
- ARM ARM Data-processing (1 source) REV32: REV32 <Xd>, <Xn> only. Encoding 1 1 0 11010110 00000 000010 Rn Rd; register 31 is ZR not SP. Vector form is Advanced SIMD two-register miscellaneous REV32, T in {8B,16B,4H,8H}.
- Dispatch: encoder/mod.rs:579-581 scalar rev32 => encode_rev32; NEON RegArrangement => encode_neon_two_misc(operands, 1, 0b00000).
- Callers: encoder dispatch only (scalar).
- Sibling encode_neon_two_misc is the vector form used by public dispatch (not a differential sibling for scalar). Sibling encode_rev/encode_rev16/encode_rbit/encode_clz/encode_cls are different opcodes.
- encode_rev32 does not check operands.len() (extra ignored); hardcodes sf=1 and discards is_64 from get_reg; parse_reg_num maps sp/wsp to 31; NEON path uses neon_arr_to_q_size which accepts 2s/4s/1d/2d.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_rev32 (arity / extra / SP / W-form / mixed W-X / FP / nonreg / invalid-name / alt-spellings / neon invalid T).
- Six failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_rev32_*.md.

# Confirmed invariants (encode_rev16)

- Valid REV16 Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `rev16 w0, w1` = 0x5ac00420.
- Success-path word is ARM Data-processing (1 source) REV16: sf 1 0 11010110 00000 000001 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(0b000001<<10)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000001.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `rev16 w0, w1` = 0x5ac00420; `rev16 x0, x1` = 0xdac00420; `rev16 wzr, wzr` = 0x5ac007ff; `rev16 xzr, xzr` = 0xdac007ff; `rev16 lr, x1` = 0xdac0043e; `rev16 x0, xzr` = 0xdac007e0.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_rev16)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `rev16 w0, w1` = 0x5ac00420.
- ARM ARM Data-processing (1 source) REV16: REV16 <Wd>, <Wn> / REV16 <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000001 Rn Rd; register 31 is ZR not SP. Vector form is Advanced SIMD two-register miscellaneous REV16, dispatched to encode_neon_two_misc, not encode_rev16.
- Dispatch: encoder/mod.rs:576-578 scalar rev16 => encode_rev16; NEON RegArrangement => encode_neon_two_misc.
- Callers: encoder dispatch only.
- Sibling encode_neon_two_misc is the vector form (not a differential sibling for scalar). Sibling encode_rev/encode_rev32/encode_rbit/encode_clz/encode_cls are different opcodes.
- encode_rev16 does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_rev16 (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_rev16_*.md.

# Confirmed invariants (encode_rbit)

- Valid RBIT Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `rbit w0, w1` = 0x5ac00020.
- Direct-call NEON path RBIT Vd.8b/16b, Vn.8b/16b matches llvm-mc (1000 cases). Dispatch routes RegArrangement to encode_neon_rbit, not encode_rbit.
- Success-path scalar word is ARM Data-processing (1 source) RBIT: sf 1 0 11010110 00000 000000 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000000.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `rbit w0, w1` = 0x5ac00020; `rbit x0, x1` = 0xdac00020; `rbit wzr, wzr` = 0x5ac003ff; `rbit xzr, xzr` = 0xdac003ff; `rbit lr, x1` = 0xdac0003e; `rbit x0, xzr` = 0xdac003e0; `rbit v0.8b, v1.8b` = 0x2e605820; `rbit v0.16b, v1.16b` = 0x6e605820.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_rbit)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `rbit w0, w1` = 0x5ac00020.
- ARM ARM Data-processing (1 source) RBIT: RBIT <Wd>, <Wn> / RBIT <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000000 Rn Rd; register 31 is ZR not SP. Vector form is Advanced SIMD two-register miscellaneous RBIT, T in {8B,16B}.
- Dispatch: encoder/mod.rs:902-909 scalar rbit => encode_rbit; NEON RegArrangement => encode_neon_rbit.
- Callers: encoder dispatch only.
- Sibling encode_neon_rbit is the vector form (not a differential sibling for scalar). Sibling encode_clz/encode_cls/encode_rev are different opcodes.
- encode_rbit does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_rbit (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings / neon).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_rbit_*.md.

# Confirmed invariants (encode_fp_arith)

- Valid FADD/FSUB/FMUL/FDIV/FMAX/FMIN/FMAXNM/FMINNM Sd,Sn,Sm / Dd,Dn,Dm including s31/d31 and uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `fadd s0, s1, s2` = 0x1e222820.
- Success-path word is ARM Floating-point data-processing (2 source): 00011110 ftype 1 Rm opcode 10 Rn Rd with ftype 00=S / 01=D. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(rm<<16)|(opcode<<12)|(0b10<<10)|(rn<<5)|rd. opcode 0000=FMUL, 0001=FDIV, 0010=FADD, 0011=FSUB, 0100=FMAX, 0101=FMIN, 0110=FMAXNM, 0111=FMINNM.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; Rm+1 increments bits[20:16] only; S vs D xor = 1<<22; FADD xor FSUB = 1<<12 (1000 cases).
- Fewer than 3 operands, non-register kinds (Imm/Symbol/Label/Mem/Cond/Shift), and invalid names (foo/s32/empty/r0) always Err (1000 cases).
- Known-answer: `fadd s0, s1, s2` = 0x1e222820; `fadd d0, d1, d2` = 0x1e622820; `fsub s0, s1, s2` = 0x1e223820; `fmul s0, s1, s2` = 0x1e220820; `fdiv s0, s1, s2` = 0x1e221820; `fmax s0, s1, s2` = 0x1e224820; `fmin s0, s1, s2` = 0x1e225820; `fmaxnm s0, s1, s2` = 0x1e226820; `fminnm s0, s1, s2` = 0x1e227820; `fadd s31, s31, s31` = 0x1e3f2bff; `fadd h0, h1, h2` = 0x1ee22820 (llvm-mc fp16).
- Extra operand, mixed S/D, GPR, SP/WSP, Q/V/B, and H (wrong ftype) currently encode instead of matching llvm-mc/gas (see bugs).

## Environment (encode_fp_arith)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fadd s0, s1, s2` = 0x1e222820.
- ARM ARM Floating-point data-processing (2 source): M=0 S=0 11110 ftype 1 Rm opcode 10 Rn Rd. ftype 00=S 01=D 11=H. Register 31 is a valid S/D/H index.
- Dispatch: encoder/mod.rs:377-404 scalar fadd/fsub/fmul/fdiv/fmax/fmin/fmaxnm/fminnm => encode_fp_arith; RegArrangement => encode_neon_float_three_same.
- Callers: encoder dispatch only.
- Sibling encode_neon_float_three_same is the vector form (not a differential sibling).
- encode_fp_arith does not check operands.len() (extra ignored); takes ftype from Rd prefix only (`starts_with('d')` else 00); parse_reg_num maps sp/wsp/x/w/q/v/h/b.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fp_arith (invalid-name).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fp_arith_*.md.

# Confirmed invariants (encode_fmov)

- Valid FMOV Sd,Sn / Dd,Dn / Sd,Wn / Dd,Xn / Wd,Sn / Xd,Dn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `fmov s0, s1` = 0x1e204020 and `fmov s0, w1` = 0x1e270020.
- Success-path FP-to-FP word is ARM FMOV (register): 00011110 ftype 1 000000 10000 Rn Rd with ftype 00=S / 01=D. Equivalently w = (0b00011110<<24)|(ftype<<22)|(0b100000<<16)|(0b10000<<10)|(rn<<5)|rd.
- Success-path GP↔FP word is ARM FMOV (general): sf 00 11110 ftype 1 00 opcode 000000 Rn Rd with opcode 111 GP→FP / 110 FP→GP; sf/ftype 0/00 for S/W and 1/01 for D/X.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D xor = 1<<22; GP→FP xor FP→GP = 1<<16 (1000 cases).
- Fewer than 2 operands, non-register kinds (Symbol/Label/Mem/Cond/Shift/Expr), and invalid names (foo/s32/empty/r0) always Err (1000 cases).
- Known-answer: `fmov s0, s1` = 0x1e204020; `fmov d0, d1` = 0x1e604020; `fmov s0, w1` = 0x1e270020; `fmov d0, x1` = 0x9e670020; `fmov w0, s1` = 0x1e260020; `fmov x0, d1` = 0x9e660020; `fmov s0, wzr` = 0x1e2703e0; `fmov d0, lr` = 0x9e6703c0; `fmov h0, h1` = 0x1ee04020 (llvm-mc fp16); `fmov x0, v0.d[1]` = 0x9eae0000.
- Extra operand, mixed S/D, size-mismatched GP/FP, Q/V/B, SP/WSP, H (wrong ftype), and V.D[1] currently encode or reject incorrectly (see bugs).

## Environment (encode_fmov)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fmov s0, s1` = 0x1e204020.
- ARM ARM FMOV (register): 0 00 11110 ftype 1 000000 10000 Rn Rd. FMOV (general): sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:376 "fmov" => encode_fmov.
- Callers: encoder dispatch only. Codegen emits S/W and D/X register forms (float_ops.rs, cast_ops.rs, alu.rs); asm_emitter.rs:205 notes "fmov requires d/s register form, not v".
- encode_fmov checks operands.len() < 2 only (extra ignored); is_fp_reg includes q/v/h/b and "sp"; ftype is 01 iff a name starts with 'd' else 00; GP-FP path ignores GP width; RegLane is not matched.
- FMOV (immediate) is a documented TODO (fp_scalar.rs:14-15) — Design Caveat, not a bug.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fmov (nonreg / invalid-name / V.D[1]).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fmov_*.md.

# Confirmed invariants (encode_extr)

- Valid EXTR Wd,Wn,Wm / Xd,Xn,Xm with 0 <= lsb < R (R=32/64), including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `extr w0, w1, w2, #0` = 0x13820020.
- Success-path word is ARM Extract EXTR: sf 00 100111 N 0 Rm imms Rn Rd with N=sf, imms=lsb. Equivalently w = (sf<<31)|(0b00100111<<23)|(sf<<22)|(rm<<16)|(lsb<<10)|(rn<<5)|rd. bits[30:23]=00100111; bit21=0.
- Algebraic alias: encode_extr(Rd,Rn,Rn,#lsb) = llvm-mc("ror Rd, Rn, #lsb") (ARM ROR immediate alias when Rn=Rm) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; Rm+1 increments bits[20:16] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds at the wrong slot, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `extr w0, w1, w2, #0` = 0x13820020; `extr w0, w1, w2, #1` = 0x13820420; `extr w0, w1, w2, #31` = 0x13827c20; `extr x0, x1, x2, #0` = 0x93c20020; `extr x0, x1, x2, #63` = 0x93c2fc20; `extr wzr, wzr, wzr, #0` = 0x139f03ff; `extr lr, x1, x2, #8` = 0x93c2203e; `ror w0, w1, #1` = 0x13810420.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb currently encode or panic instead of Err (see bugs).

## Environment (encode_extr)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `extr w0, w1, w2, #0` = 0x13820020.
- ARM ARM Extract EXTR: EXTR <Wd>, <Wn>, <Wm>, #<lsb> / EXTR <Xd>, <Xn>, <Xm>, #<lsb>. Encoding sf 00 100111 N 0 Rm imms Rn Rd; N=sf; 0 <= lsb <= 31 (W) / 63 (X); register 31 is ZR not SP. ROR (immediate) is the alias of EXTR when Rn=Rm.
- Dispatch: encoder/mod.rs:894 "extr" => encode_extr.
- Callers: encoder dispatch only.
- Sibling encode_shift ROR is a different mnemonic (3-operand shift) — not a differential sibling; used only as ARM alias when Rn=Rm.
- encode_extr does not check operands.len() (extra ignored); takes sf from Rd without checking Rn/Rm width or FP prefix; parse_reg_num maps sp/wsp to 31; lsb is `as u32` with no ARM range check (negative panics on `lsb << 10` in debug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_extr (arity / extra / SP / mixed W-X / FP / lsb / nonreg / invalid-name / alt-spellings).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_extr_*.md.

# Confirmed invariants (encode_clz)

- Valid CLZ Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `clz w0, w1` = 0x5ac01020.
- Success-path word is ARM Data-processing (1 source) CLZ: sf 1 0 11010110 00000 000100 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(0b000100<<10)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000100.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `clz w0, w1` = 0x5ac01020; `clz x0, x1` = 0xdac01020; `clz wzr, wzr` = 0x5ac013ff; `clz xzr, xzr` = 0xdac013ff; `clz lr, x1` = 0xdac0103e; `clz x0, xzr` = 0xdac013e0.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_clz)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `clz w0, w1` = 0x5ac01020.
- ARM ARM Data-processing (1 source) CLZ: CLZ <Wd>, <Wn> / CLZ <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000100 Rn Rd; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:573-575 scalar clz => encode_clz; NEON RegArrangement => encode_neon_two_misc.
- Callers: encoder dispatch only.
- Sibling encode_cls is a different opcode (000101, count leading sign bits) — not a differential sibling.
- encode_clz does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_clz (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_clz_*.md.

# Confirmed invariants (encode_cls)

- Valid CLS Wd,Wn / Xd,Xn including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `cls w0, w1` = 0x5ac01420.
- Success-path word is ARM Data-processing (1 source) CLS: sf 1 0 11010110 00000 000101 Rn Rd. Equivalently w = (sf<<31)|(1<<30)|(0b011010110<<21)|(0b000101<<10)|(rn<<5)|rd. bits[30]=1; bits[29]=0; bits[28:21]=11010110; bits[20:16]=00000; bits[15:10]=000101.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; X vs W xor = 1<<31 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `cls w0, w1` = 0x5ac01420; `cls x0, x1` = 0xdac01420; `cls wzr, wzr` = 0x5ac017ff; `cls xzr, xzr` = 0xdac017ff; `cls lr, x1` = 0xdac0143e; `cls x0, xzr` = 0xdac017e0.
- Extra operand, SP/WSP, mixed W/X, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_cls)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `cls w0, w1` = 0x5ac01420.
- ARM ARM Data-processing (1 source) CLS: CLS <Wd>, <Wn> / CLS <Xd>, <Xn>. Encoding sf 1 0 11010110 00000 000101 Rn Rd; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:570-572 scalar cls => encode_cls; NEON RegArrangement => encode_neon_two_misc.
- Callers: encoder dispatch only.
- Sibling encode_clz is a different opcode (000100, count leading zeros) — not a differential sibling.
- encode_cls does not check operands.len() (extra ignored); takes sf from Rd without checking Rn width or FP prefix; parse_reg_num maps sp/wsp to 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_cls (arity / extra / SP / mixed W-X / FP / nonreg / invalid-name / alt-spellings).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_cls_*.md.

# Confirmed invariants (encode_cas)

- Valid CAS/CASA/CASAL/CASL on matching W or X Rs/Rt with [Xn|SP], and CASB/CASH (plus acquire/release) on W Rs/Rt with [Xn|SP], including wzr/xzr and sp-as-base, matches llvm-mc `-triple=aarch64 -mattr=+lse -show-encoding` (1000 cases). gas aarch64-linux-gnu-as -march=armv8-a+lse agrees on `cas x0, x1, [x2]` = 0xc8a07c41.
- Success-path word is ARM CAS: size 001000 1 L 1 Rs o0 11111 Rn Rt. size 00=byte 01=half 10=word 11=doubleword; L=1 for CASA/CASAL; o0=1 for CASL/CASAL; bits[29:24]=001000; bit23=1; bit21=1; bits[14:10]=11111. Equivalently w = (size<<30)|(0b001000<<24)|(1<<23)|(L<<22)|(1<<21)|(rs<<16)|(o0<<15)|(0b11111<<10)|(rn<<5)|rt.
- Metamorphic: Rt+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; Rs+1 increments bits[20:16] only; CASA xor CAS = 1<<22; CASL xor CAS = 1<<15; CASAL xor CAS = (1<<22)|(1<<15); uppercase mnemonic matches lowercase (1000 cases).
- Fewer than 3 operands, non-Mem third operand (Imm/Symbol/Cond/pre/post/reg-offset), and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `cas x0, x1, [x2]` = 0xC8A07C41; `cas w0, w1, [x2]` = 0x88A07C41; `casa w0, w1, [x2]` = 0x88E07C41; `casl w0, w1, [x2]` = 0x88A0FC41; `casal w0, w1, [x2]` = 0x88E0FC41; `casb w0, w1, [x2]` = 0x08A07C41; `cash w0, w1, [x2]` = 0x48A07C41; `cas xzr, xzr, [sp]` = 0xC8BF7FFF.
- Extra operand, SP/WSP as Rs/Rt, XZR/x31/WZR as base, W/WSP as base, mixed W/X, FP/SIMD prefixes, CASB/CASH with X, and nonzero Mem offset currently encode instead of Err (see bugs).
- gas accepts optional `#0` offset (ARM `{,#0}`); llvm-mc 15 rejects `#0`. At the encode_cas boundary `[Xn]` and `[Xn, #0]` are the same Operand::Mem{offset:0}, so the #0 form is not distinguishable here.

## Environment (encode_cas)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+lse -show-encoding. gas aarch64-linux-gnu-as -march=armv8-a+lse agrees on `cas x0, x1, [x2]` = 0xc8a07c41.
- ARM ARM Compare and Swap: size 001000 1 L 1 Rs o0 11111 Rn Rt. Rs/Rt are ZR not SP at 31; Rn is SP not ZR at 31. CASB/CASH require W registers.
- Dispatch: encoder/mod.rs:920-922 cas/casa/casal/casl/casb/casab/casalb/caslb/cash/casah/casalh/caslh => encode_cas.
- Callers: encoder dispatch only.
- Sibling encode_swp is a different LSE class (not a differential sibling).
- encode_cas checks operands.len() < 3 only (extra operands ignored); takes size from Rs (or b/h suffix) without checking Rt width or FP prefix; Mem offset is discarded (`base, ..`).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_cas (arity / extra / SP / XZR-base / W-base / mixed W-X / FP / casb-X / nonzero offset / non-mem / invalid-name / alt-spellings).
- Eight failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_cas_*.md.

# Confirmed invariants (encode_bfxil)

- Valid BFXIL Wd,Wn / Xd,Xn with 0 <= lsb < R and 1 <= width <= R-lsb (R=32/64), including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `bfxil w0, w1, #0, #1` = 0x33000020.
- Success-path word is BFM: sf 01 100110 N immr imms Rn Rd with N=sf, immr=lsb, imms=lsb+width-1. Equivalently w = (sf<<31)|(0b01<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd.
- Algebraic alias: encode_bfxil(Rd,Rn,#lsb,#width) = encode_bfm(Rd,Rn,#lsb,#(lsb+width-1)) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds at the wrong slot, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `bfxil w0, w1, #0, #1` = 0x33000020; `bfxil w0, w1, #1, #1` = 0x33010420; `bfxil x0, x1, #1, #8` = 0xb3412020; `bfxil wzr, wzr, #31, #1` = 0x331f7fff; `bfxil x0, xzr, #63, #1` = 0xb37fffe0; `bfxil lr, x1, #8, #16` = 0xb3485c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_bfxil)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `bfxil w0, w1, #0, #1` = 0x33000020.
- ARM ARM Bitfield Move BFXIL alias of BFM: BFXIL Wd, Wn, #lsb, #width <=> BFM Wd, Wn, #lsb, #(lsb+width-1) with 0 <= lsb < 32 and 1 <= width <= 32-lsb (64-bit analog). Encoding sf 01 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:893 "bfxil" => encode_bfxil.
- Callers: encoder dispatch only.
- Sibling encode_bfm is the raw form (used as algebraic alias after ARM mapping, not as a differential sibling).
- encode_bfxil computes imms = lsb + width - 1 without checking ARM bounds; width=0 and lsb=0 panics in debug (subtract overflow); large lsb+width panics (add overflow).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_bfxil (arity / extra / SP / mixed W-X / FP / lsb-width / nonreg / invalid-name / alt-spellings).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_bfxil_*.md.

# Confirmed invariants (encode_bfi)

- Valid BFI Wd,Wn / Xd,Xn with 0 <= lsb < R and 1 <= width <= R-lsb (R=32/64), including wzr/xzr, w31/x31, lr, uppercase, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). gas aarch64-linux-gnu-as agrees on `bfi w0, w1, #0, #1` = 0x33000020.
- Success-path word is BFM: sf 01 100110 N immr imms Rn Rd with N=sf, immr=(-lsb MOD R), imms=width-1. Equivalently w = (sf<<31)|(0b01<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd.
- Algebraic alias: encode_bfi(Rd,Rn,#lsb,#width) = encode_bfm(Rd,Rn,#(-lsb MOD R),#(width-1)) (1000 cases).
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only (1000 cases).
- Fewer than 4 operands, non-register/non-imm kinds at the wrong slot, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `bfi w0, w1, #0, #1` = 0x33000020; `bfi w0, w1, #1, #1` = 0x331f0020; `bfi x0, x1, #1, #8` = 0xb37f1c20; `bfi wzr, wzr, #31, #1` = 0x330103ff; `bfi x0, xzr, #63, #1` = 0xb34103e0; `bfi lr, x1, #8, #16` = 0xb3783c3e.
- Extra operand, SP/WSP, mixed W/X, FP/SIMD prefixes, and out-of-range lsb/width currently encode or panic instead of Err (see bugs).

## Environment (encode_bfi)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding. gas aarch64-linux-gnu-as agrees on `bfi w0, w1, #0, #1` = 0x33000020.
- ARM ARM Bitfield Move BFI alias of BFM: BFI Wd, Wn, #lsb, #width <=> BFM Wd, Wn, #(-lsb MOD 32), #(width-1) with 0 <= lsb < 32 and 1 <= width <= 32-lsb (64-bit analog). Encoding sf 01 100110 N immr imms Rn Rd; N=sf; register 31 is ZR not SP. llvm-mc prefers BFXIL in disassembly when immr=0 (lsb=0) because the encodings coincide.
- Dispatch: encoder/mod.rs:892 "bfi" => encode_bfi.
- Callers: encoder dispatch only.
- Sibling encode_bfm is the raw form (used as algebraic alias after ARM mapping, not as a differential sibling).
- encode_ubfiz uses wrapping_sub for immr; encode_bfi uses non-wrapping `reg_width - lsb`, which panics in debug when lsb > reg_width.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_bfi (arity / extra / SP / mixed W-X / FP / lsb-width / nonreg / invalid-name / alt-spellings).
- Five failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_bfi_*.md.

# Confirmed invariants (encode_neon_aes)

- Valid AES AESE/AESD/AESMC/AESIMC Vd.16b, Vn.16b (including v31, uppercase V/16B) matches llvm-mc `-triple=aarch64 -mattr=+aes -show-encoding` (1000 cases). gas aarch64-linux-gnu-as -march=armv8-a+crypto agrees on `aese v0.16b, v1.16b` = 0x4e284820.
- Success-path word: 0100 1110 0010 1000 opcode 10 Rn Rd. Equivalently w = (0b01001110<<24)|(0b0010100<<17)|(opc<<12)|(0b10<<10)|(rn<<5)|rd. opc AESE=00100 AESD=00101 AESMC=00110 AESIMC=00111; bits[31:24]=01001110; bits[23:17]=0010100; bits[11:10]=10.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; AESD XOR AESE = 1<<12; AESMC XOR AESE = 1<<13 (1000 cases).
- Fewer than 2 operands, non-register dest/src kinds, and invalid names (v32/foo/empty/v/v99/v-1) always Err (1000 cases).
- Known-answer: `aese v0.16b, v1.16b` = 0x4e284820; `aesd v0.16b, v1.16b` = 0x4e285820; `aesmc v0.16b, v1.16b` = 0x4e286820; `aesimc v0.16b, v1.16b` = 0x4e287820; `aese v31.16b, v31.16b` = 0x4e284bff; `aesd v31.16b, v0.16b` = 0x4e28581f; `aesmc v0.16b, v31.16b` = 0x4e286be0; `aesimc v15.16b, v16.16b` = 0x4e287a0f.
- Extra operand, T other than .16b, mismatched T, bare Vn, non-V prefix, and SP/WSP currently encode incorrectly (see bugs).

## Environment (encode_neon_aes)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -mattr=+aes -show-encoding. gas aarch64-linux-gnu-as -march=armv8-a+crypto agrees on `aese v0.16b, v1.16b` = 0x4e284820.
- ARM ARM Cryptographic AES: 0 1 0 0 1 1 1 0 size 1 01000 opcode 10 Rn Rd. size must be 00 (otherwise unallocated). opcode 00100=AESE 00101=AESD 00110=AESMC 00111=AESIMC. Assembly syntax is only Vd.16B, Vn.16B.
- Dispatch: encoder/mod.rs:747-750 aese/aesd/aesmc/aesimc => encode_neon_aes with opcodes 00100/00101/00110/00111.
- Callers: encoder dispatch only; no ARM codegen emitter of aese/aesd/aesmc/aesimc found (x86 AES-NI is a different ISA).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_neon_aes (arity / extra / T / mismatch / bare Reg / GPR prefix / SP / WSP / nonreg src / invalid-name).
- Six failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_aes_*.md.

# Confirmed invariants (encode_fcvt_precision)

- Valid scalar FCVT Sd|Dd|Hd, Sn|Dn|Hn with dest precision != src precision (including s31/d31/h31, uppercase) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases; half uses -mattr=+fullfp16).
- Success-path word: 0 00 11110 ftype 1 0001 opc 10000 Rn Rd. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b0001<<17)|(opc<<15)|(0b10000<<10)|(rn<<5)|rd. ftype 00=S 01=D 11=H source; opc 00=S 01=D 11=H dest; bits[31:24]=00011110; bit21=1; bits[20:17]=0001; bits[14:10]=10000.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S dest vs D dest (H src) flips only bit 15; S src vs D src (H dest) flips only bit 22 (1000 cases).
- Fewer than 2 operands, non-register kinds, invalid names (foo/s32/d32/h32/empty/r0), and GPR/Q/V/B/WSP in either slot always Err (1000 cases).
- Known-answer: `fcvt d0, s1` = 0x1e22c020; `fcvt s0, d1` = 0x1e624020; `fcvt h0, s1` = 0x1e23c020; `fcvt s0, h1` = 0x1ee24020; `fcvt d0, h1` = 0x1ee2c020; `fcvt h0, d1` = 0x1e63c020; `fcvt d31, s31` = 0x1e22c3ff; `fcvt s31, d0` = 0x1e62401f.
- Extra operand, same-precision S/D/H, and SP dest/src currently encode incorrectly (see bugs).

## Environment (encode_fcvt_precision)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fcvt d0, s1` = 0x1e22c020.
- ARM ARM Floating-point data-processing (1 source) FCVT: 0 00 11110 ftype 1 0001 opc 10000 Rn Rd. ftype 00=S 01=D 11=H source; opc 00=S 01=D 11=H dest; ftype==opc is unallocated.
- Dispatch: encoder/mod.rs:460 "fcvt" => encode_fcvt_precision. fcvtzs/fcvtzu/etc. go to encode_fcvt_rounding. fcvtl/fcvtn go to NEON.
- Callers: encoder dispatch; codegen/cast_ops.rs:74-78 emits `fcvt d0, s0` / `fcvt s0, d0`.
- Unlike encode_fcmp / encode_fp_1src, this encoder already selects ftype/opc from s/d/h prefixes, so half-precision conversions match llvm-mc.
- parse_reg_num maps sp to 31 and dest/src first char 's' selects S, so SP encodes as S31 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fcvt_precision (arity / extra / same-precision / GPR / QVB / WSP / SP dest+src / half / nonreg / invalid-name).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fcvt_precision_*.md.

# Confirmed invariants (encode_fcmp)

- Valid scalar FCMP Sn,Sm or Dn,Dm (including s31/d31, uppercase) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid FCMP Sn|Dn, #0.0 via Operand::Imm(0) matches llvm-mc `fcmp Sn|Dn, #0.0` (1000 cases).
- Success-path register word: 0 00 11110 ftype 1 Rm 001000 Rn 00000. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(rm<<16)|(0b001000<<10)|(rn<<5). ftype 00=S 01=D; bits[15:10]=001000; bit21=1; opc=00000.
- Success-path #0.0 word: Rm=00000, opc=01000. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b001000<<10)|(rn<<5)|0b01000.
- Metamorphic: Rn+1 increments bits[9:5] only; Rm+1 increments bits[20:16] only; S vs D flips only bit 22; register (Rm=0) XOR #0.0 = 1<<3 (1000 cases).
- Non-zero Imm, non-register kinds (Imm(1)/Symbol/Label/Mem/Cond/Shift), and invalid names (foo/s32/d32/empty/r0) always Err (1000 cases).
- Known-answer: `fcmp s0, s1` = 0x1e212000; `fcmp d0, d1` = 0x1e612000; `fcmp s0, #0.0` = 0x1e202008; `fcmp d0, #0.0` = 0x1e602008; `fcmp s31, s31` = 0x1e3f23e0; `fcmp d31, d0` = 0x1e6023e0; llvm-mc +fullfp16 `fcmp h0, h1` = 0x1ee12000 / `fcmp h0, #0.0` = 0x1ee02008 (SUT currently 0x1e212000 / 0x1e202008, see bugs).
- One operand, extra operand, mixed S/D (and GPR/SP/QVB), and H registers currently encode incorrectly (see bugs).

## Environment (encode_fcmp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fcmp s0, s1` = 0x1e212000.
- ARM ARM Floating-point compare: 0 00 11110 ftype 1 Rm 001000 Rn opc. ftype 00=S 01=D 11=H; opc 00000=FCMP register, 01000=FCMP #0.0, 10000=FCMPE register, 11000=FCMPE #0.0. Immediate form is only #0.0.
- Dispatch: encoder/mod.rs:439 "fcmp" => encode_fcmp. fccmp is a different mnemonic. fcmpe is not dispatched.
- Callers: encoder dispatch; codegen/comparison.rs:15-19 emits `fcmp s0, s1` / `fcmp d0, d1` (never the #0.0 form).
- Encoder mapping: Operand::Imm(0) <-> `#0.0` (Operand has no float-immediate variant). Parser currently turns textual `#0.0` into Operand::Expr("0.0"), which encode_fcmp rejects via get_reg — a parser/encoder seam, not claimed as this function's contract.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fcmp (arity / extra / mixed S-D / GPR / QVB / SP / half ftype / nonzero imm / nonreg / invalid-name).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fcmp_*.md.

# Confirmed invariants (encode_int_to_float)

- Valid integer SCVTF/UCVTF Sd|Dd, Wn|Xn (including wzr/xzr, w31/x31, lr, uppercase, mixed S/X and D/W) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: sf 00 11110 ftype 1 00 opcode 000000 Rn Rd. Equivalently w = (sf<<31)|(0b11110<<24)|(ftype<<22)|(1<<21)|(opcode<<16)|(rn<<5)|rd. sf 0=W source 1=X source; ftype 00=S dest 01=D dest; opcode 010=SCVTF 011=UCVTF; bits[30:29]=00, bits[20:19]=00 (rmode), bits[15:10]=0, bit21=1.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; W vs X source flips only bit 31; S vs D dest flips only bit 22; SCVTF XOR UCVTF = 1<<16 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/s32/empty/r0) always Err (1000 cases).
- Known-answer: `scvtf s0, w1` = 0x1e220020; `scvtf d0, x1` = 0x9e620020; `ucvtf s0, w1` = 0x1e230020; `ucvtf d0, x1` = 0x9e630020; `scvtf s0, x1` = 0x9e220020; `scvtf d31, xzr` = 0x9e6203ff; `scvtf d0, lr` = 0x9e6203c0; llvm-mc +fullfp16 `scvtf h0, w1` = 0x1ee20020 (SUT currently 0x1e220020, see bugs).
- Extra operand, SP/WSP source, GP dest / FP source / QVB, and H dest currently encode incorrectly (see bugs).

## Environment (encode_int_to_float)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `scvtf s0, w1` = 0x1e220020.
- ARM ARM Conversion between floating-point and integer: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd. For integer SCVTF/UCVTF rmode=00, opcode=010/011. sf 0=W source 1=X source; ftype 00=S 01=D 11=H; register 31 is ZR not SP. Fixed-point form uses bit21=0 plus scale.
- Dispatch: encoder/mod.rs:454-459 ucvtf/scvtf => encode_ucvtf/encode_scvtf => encode_int_to_float. Vector RegArrangement goes to encode_neon_float_two_misc. SIMD-scalar `scvtf s0, s1` = 0x5e21d820 is a different class.
- Callers: encoder dispatch; codegen/cast_ops.rs:53-66 emits `scvtf`/`ucvtf` d0/s0, x0.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_int_to_float (arity / extra / SP src / GP dest / FP source / QVB / H ftype / nonreg / invalid-name).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_int_to_float_*.md.

# Confirmed invariants (encode_fp_1src)

- Valid scalar FRINTN/P/M/Z/A/X/I Sd,Sn or Dd,Dn (including s31/d31, uppercase, all 7 mnemonics) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: 0 00 11110 ftype 1 opcode 10000 Rn Rd. Equivalently w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(opcode<<15)|(0b10000<<10)|(rn<<5)|rd. ftype 00=S 01=D; bits[14:10]=10000; bit21=1.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D flips only bit 22; FRINTN XOR FRINTP = 1<<15 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/s32/d32/empty/r0) always Err (1000 cases).
- Known-answer: `frintn s0, s1` = 0x1e244020; `frintn d0, d1` = 0x1e644020; `frintp s0, s1` = 0x1e24c020; `frintz s0, s1` = 0x1e25c020; `frinti s0, s1` = 0x1e27c020; `frintn s31, s31` = 0x1e2443ff; llvm-mc +fullfp16 `frintn h0, h1` = 0x1ee44020 (SUT currently 0x1e244020, see bugs).
- Extra operand, mixed S/D (and GPR/SP/QVB), and H registers currently encode incorrectly (see bugs).

## Environment (encode_fp_1src)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `frintn s0, s1` = 0x1e244020.
- ARM ARM Floating-point data-processing (1 source): M=0 S=0 11110 ftype 1 opcode 10000 Rn Rd. ftype 00=S 01=D 11=H. FRINTN=001000 FRINTP=001001 FRINTM=001010 FRINTZ=001011 FRINTA=001100 FRINTX=001110 FRINTI=001111.
- Dispatch: encoder/mod.rs:414-434 frintn/p/m/z/a/x/i => encode_fp_1src. Vector RegArrangement goes to encode_neon_float_two_misc.
- Callers: encoder dispatch only (no codegen emitter of scalar frint* found).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fp_1src (arity / extra / mixed S-D / GPR / QVB / SP / half ftype / nonreg / invalid-name / uppercase / S vs D / all 7 opcodes).
- Three failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fp_1src_*.md.

# Confirmed invariants (encode_fcvt_rounding)

- Valid integer FCVT* Wd|Xd, Sn|Dn (including wzr/xzr, w31/x31, lr, uppercase, all 10 mnemonics fcvtzs/zu/as/au/ns/nu/ms/mu/ps/pu) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd. Equivalently w = (sf<<31)|(0b11110<<24)|(ftype<<22)|(1<<21)|(rmode<<19)|(opcode<<16)|(rn<<5)|rd. bits[30:29]=00, bits[15:10]=0, bit21=1.
- Metamorphic: Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; W vs X flips only bit 31; S vs D flips only bit 22; FCVTZS XOR FCVTZU = 1<<16 (1000 cases).
- Fewer than 2 operands, non-register kinds, and invalid names (foo/x32/s32/empty/r0) always Err (1000 cases).
- Known-answer: `fcvtzs w0, s1` = 0x1e380020; `fcvtzs x0, d1` = 0x9e780020; `fcvtzu w0, s1` = 0x1e390020; `fcvtas w0, s1` = 0x1e240020; `fcvtzs xzr, d0` = 0x9e78001f; `fcvtzs lr, s0` = 0x9e38001e; llvm-mc +fullfp16 `fcvtzs w0, h1` = 0x1ef80020 (SUT currently 0x1e380020, see bugs).
- Extra operand, SP/WSP dest, FP dest / GP source / QVB, and H source currently encode incorrectly (see bugs).

## Environment (encode_fcvt_rounding)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (half: -mattr=+fullfp16). gas aarch64-linux-gnu-as agrees on `fcvtzs w0, s1` = 0x1e380020.
- ARM ARM Conversion between floating-point and integer: sf 00 11110 ftype 1 rmode opcode 000000 Rn Rd. sf 0=W dest 1=X dest; ftype 00=S 01=D 11=H; register 31 is ZR not SP.
- Dispatch: encoder/mod.rs:440-453 fcvtzs/fcvtzu/fcvtas/au/ns/nu/ms/mu/ps/pu => encode_fcvt_rounding. fcvtzs/fcvtzu with RegArrangement go to NEON instead.
- Callers: encoder dispatch; codegen/cast_ops.rs emits `fcvtzs`/`fcvtzu`.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_fcvt_rounding (arity / extra / SP dest / FP dest / GP source / QVB / H ftype / nonreg / invalid-name).
- Four failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_fcvt_rounding_*.md.

# Confirmed invariants (encode_smulh)

- Valid SMULH Xd, Xn, Xm (including xzr, lr, x31, uppercase, LR/XZR) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: sf=1 op54=00 11011 op31=010 o0=0 Ra=11111; w = 0x9B407C00 | (rm<<16) | (rn<<5) | rd.
- Metamorphic: SMULH XOR UMULH is bit 23 (U); Rd+1 / Rn+1 / Rm+1 update only that field (1000 cases).
- Fewer than 3 operands, non-register operands, and invalid names (foo/x32/empty/r0) always Err (1000 cases).
- Known-answer: `smulh x0, x1, x2` = 0x9B427C20; `smulh xzr, xzr, xzr` = 0x9B5F7FFF; `smulh lr, x1, x30` = 0x9B5E7C3E; `smulh x0, x1, xzr` = 0x9B5F7C20.
- Extra operand, W registers (including wzr), SP/WSP, and FP/SIMD prefixes currently encode instead of Err (see bugs).

## Environment (encode_smulh)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) SMULH: 1 00 11011 010 Rm 0 11111 Rn Rd; 64-bit only; register 31 is XZR not SP.
- Dispatch: encoder/mod.rs:275 "smulh" => encode_smulh.
- Callers: encoder dispatch; no codegen emitter of smulh found (umulh is used in i128_ops.rs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_smulh (arity / extra / W-width / WZR / SP / FP / nonreg / invalid-name / alt-spellings / field independence).

# Confirmed invariants (encode_prfm)

- Valid PRFM (immediate) with named prfop or #imm5 in 0..31, base Xn|SP, pimm = imm12*8 in [0, 32760] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Uppercase prfop/Xn/SP spellings match llvm-mc (1000 cases).
- Success-path unsigned word: size=11 V=0 opc=10; 1111 1001 10 imm12 Rn Rt. Equivalently w = 0xF9800000 | (imm12<<10) | (rn<<5) | prfop.
- Metamorphic: prfop+1 increments Rt only; Rn+1 increments Rn field only; imm12+1 increments imm12 only (1000 cases).
- Fewer than 2 operands, unaligned/negative/too-large pimm, pre/post-index, Imm/Label/Cond address, unknown prfop name, #imm5 outside 0..31, Reg-as-prfop, base foo/x32, invalid index name, and PRFM literal Symbol always Err (1000 cases).
- Known-answer: `prfm pldl1keep, [x0]` = 0xF9800000; `prfm pldl1keep, [x1, #8]` = 0xF9800420; `prfm pldl1strm, [sp, #16]` = 0xF9800BE1; `prfm #31, [x0]` = 0xF980001F; `prfm pldl1keep, [x0, #32760]` = 0xF9BFFC00; llvm-mc `prfm pldl1keep, [x0, x1]` = 0xF8A16800 (SUT currently 0xF9216800, see bugs).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM PRFM (immediate): 1111 1001 10 imm12 Rn Rt; pimm multiple of 8 in [0,32760]. PRFM (register): 11 111 0 00 10 1 Rm option S 10 Rn Rt; option UXTW/LSL/SXTW/SXTX; S amount 0 or 3. Rt is 5-bit prfop. Rn is Xn|SP (31=SP, not XZR).
- Dispatch: encoder/mod.rs:917 "prfm" => encode_prfm.
- Callers: encoder dispatch; inline_asm.rs mentions prfm/prefetch.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- parse_reg_num maps w/wsp/xzr/x31/FP prefixes, so invalid bases encode (see bugs).
- PRFM (register) uses `(0b10 << 23)` instead of `(0b10 << 22)` (see bugs).
- Bare W-index defaults to UXTW (see bugs).
- Shift amount > 0 is encoded as S=1 regardless of 1 vs 3 (see bugs).
- PRFM (literal) Symbol returns Err("not yet supported") — Doc evidence load_store.rs:759-761.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_prfm (arity / extra / W-base / XZR / x31 / FP / offset range / pre/post / unknown prfop / imm5 range / W-index / bad shift / Reg-prfop / invalid names / literal).
- Seven failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_prfm_*.md.

# Confirmed invariants (encode_ldtr_sized)

- Valid LDTRB/LDTRH/STTRB/STTRH Wt, [Xn|SP{, #simm}] with simm in [-256, 255] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Uppercase Wt/Xn/SP/WZR spellings match llvm-mc (1000 cases).
- w31 dest encodes as wzr and matches llvm-mc (1000 cases).
- Success-path word: size 111 V=0 00 opc 0 imm9 10 Rn Rt. size=00 byte / 01 half; opc=01 load / 00 store. Equivalently w = (size<<30) | 0x38000800 | (opc<<22) | ((imm9 as u32 & 0x1FF)<<12) | (rn<<5) | rt.
- Metamorphic: size bit XOR = 1<<30; load XOR store = 1<<22; Rt+1 adds 1; Rn+1 adds 32; imm9+1 only changes bits[20:12] (1000 cases).
- Fewer than 2 operands and non-Mem addressing (pre/post/reg-offset/Imm/Symbol/Label) always Err (1000 cases).
- Known-answer: `ldtrb w0, [x1]` = 0x38400820; `ldtrh w0, [x1]` = 0x78400820; `sttrb w0, [x1]` = 0x38000820; `sttrh w0, [x1]` = 0x78000820; `ldtrb w0, [x1, #-256]` = 0x38500820; `ldtrb w0, [sp, #255]` = 0x384ffbe0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM LDTRB/LDTRH/STTRB/STTRH: size 111 V=0 00 opc 0 imm9 10 Rn Rt. Syntax Wt, [Xn|SP{, #simm}]; simm9 in [-256,255]. Register 31 is WZR for Rt, SP for Rn. Sibling encode_ldur_stur is LDUR/STUR/LDTR/STTR auto-size (different job).
- Dispatch: encoder/mod.rs:340-343 ldtrh/sttrh/ldtrb/sttrb => encode_ldtr_sized.
- Callers: encoder dispatch only (no codegen sites emit these mnemonics).

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- get_reg discards is_64, so Xt dest encodes as Wt (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP as Rt encodes as WZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- W base and XZR base encode as Xn/SP (see bugs).
- Out-of-range offsets wrap with imm9 = offset & 0x1FF (see bugs).
- Non-Mem forms correctly return Err.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ldtr_sized (arity / extra / Xt dest / SP / FP / W-base / XZR-base / offset range / pre/post/regoff / w31 alias / uppercase).
- Seven failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldtr_sized_*.md.

# Confirmed invariants (encode_ldrsw)

- Valid unsigned LDRSW Xt, [Xn|SP, #pimm] with pimm = imm12*4 in [0, 16380] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Alternate spellings x31, uppercase Xn/SP/XZR, lr match llvm-mc (1000 cases).
- Valid unscaled (simm9 in [-256,255]), pre-index, and post-index forms match llvm-mc when writeback does not use Rt==Rn (unless Rn is SP) (1000 cases).
- Valid register-offset LDRSW with lsl/sxtx on Xm and uxtw/sxtw on Wm, amount in {0,2}, matches llvm-mc (1000 cases).
- Success-path unsigned word: size=10 111 V=0 01 opc=10 imm12 Rn Rt. Equivalently w = 0xB9800000 | (imm12<<10) | (rn<<5) | rt.
- Unscaled: bits[25:24]=00, bit21=0, bits[11:10]=00, imm9 at [20:12]. Pre bits[11:10]=11; post=01. Register: bit21=1, bits[11:10]=10.
- Metamorphic: Rt+1 adds 1, Rn+1 adds 32, imm12+1 adds 1<<10; pre XOR post = 0b10<<10 (1000 cases).
- Fewer than 2 operands, Imm/Cond/Barrier/Shift/Extend/RegList/MemExpr/Label at the address slot, lsl #1/#3, uxtx, and base "foo" always Err (1000 cases).
- Known-answer: `ldrsw x0, [x1]` = 0xB9800020; `ldrsw x0, [x1, #4]` = 0xB9800420; `ldrsw x0, [x1, #16380]` = 0xB9BFFC20; `ldrsw x0, [x1, #-4]` = 0xB89FC020; `ldrsw x0, [x1, #4]!` = 0xB8804C20; `ldrsw x0, [x1], #4` = 0xB8804420; `ldrsw x0, [x1, x2]` = 0xB8A26820; `ldrsw x0, [sp, #4]` = 0xB98007E0; `ldrsw xzr, [x1]` = 0xB980003F; `ldrsw x30, [x2]` = 0xB980005E.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM LDRSW unsigned: size=10 111 0 01 10 imm12 Rn Rt, pimm in [0,16380] multiple of 4. LDURSW / pre / post: simm9 in [-256,255]. Register: option in {UXTW,LSL,SXTW,SXTX}, S amount 0 or 2. Literal: 10 011 000 imm19 Rt. Dest Xt (31=XZR), base Xn|SP.
- Dispatch: encoder/mod.rs:333 "ldrw" | "ldrsw" => encode_ldrsw.
- Callers: prologue.rs IrType::I32 => ldrsw; peephole.rs ldrsw forwarding; emit.rs; variadic.rs.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- get_reg discards is_64, so Wt dest encodes (see bugs).
- parse_reg_num maps sp to 31, so SP as Rt encodes as XZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- W base and XZR base encode as Xn/SP (see bugs).
- Bare W index defaults to LSL rather than requiring uxtw/sxtw (see bugs).
- Out-of-range offsets fall through to unscaled with imm9 = offset & 0x1FF (see bugs).
- Pre/post Rt==Rn is encoded (llvm-mc: unpredictable) (see bugs).
- Operand::Symbol (LDRSW literal) is not handled (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ldrsw (unsigned / unscaled / pre / post / regoff / alt-spellings / bad extend / invalid base / literal / extra / W-dest / SP / FP / W-base / XZR-base / W-index / writeback overlap / offset range).
- Ten failing properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldrsw_*.md.

# Confirmed invariants (encode_ldaxr_stlxr)

- Valid LDAXR/STLXR/LDAXRB/STLXRB/LDAXRH/STLXRH with Rt/Rn/Ws in 0..31 (xzr/wzr at 31 for data/status, sp at 31 for base, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Alternate spellings x31, uppercase Xn/SP/XZR, lr match llvm-mc (1000 cases).
- encode_ldaxr_stlxr XOR encode_ldxr_stxr at equal operands = 1<<15 (ARM ARM o0) (1000 cases). ldaxr XOR stlxr(wzr) = 1<<22 (L). X XOR W = 1<<30 (size). stlxrb XOR stlxrh = 1<<30.
- Success-path word: size 001000 0 L 0 Rs o0=1 Rt2=11111 Rn Rt. Equivalently load w = (size<<30) | 0x085FFC00 | (rn<<5) | rt; store w = (size<<30) | 0x0800FC00 | (ws<<16) | (rn<<5) | rt. size is 0b11/0b10 for X/W, 0b00 byte, 0b01 half.
- Fewer than required operands, Imm/Symbol/pre/post-index, invalid base names (foo, x32), MemRegOffset, and MemExpr always Err (1000 cases).
- Known-answer: `ldaxr x0, [x1]` = 0xC85FFC20; `ldaxr w0, [x1]` = 0x885FFC20; `ldaxrb w0, [x1]` = 0x085FFC20; `ldaxrh w0, [x1]` = 0x485FFC20; `stlxr w0, x1, [x2]` = 0xC800FC41; `stlxr w0, w1, [x2]` = 0x8800FC41; `ldaxr x0, [sp]` = 0xC85FFFE0; `ldaxr lr, [x2]` = 0xC85FFC5E.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Load/Store Exclusive LDAXR/STLXR: size 001000 0 L 0 Rs o0=1 Rt2=11111 Rn Rt. Syntax LDAXR Wt/Xt, [Xn|SP]{,#0}; STLXR Ws, Wt/Xt, [Xn|SP]{,#0}; byte/half take Wt. Register 31 is ZR for Rt/Ws, SP for Rn. Sibling encode_ldxr_stxr is o0=0 (different job).
- Dispatch: encoder/mod.rs:354-359 ldaxr/stlxr/ldaxrb/stlxrb/ldaxrh/stlxrh => encode_ldaxr_stlxr.
- Callers: src/backend/arm/codegen/inline_asm.rs Acquire => ldaxr, Release => stlxr, AcqRel/SeqCst => both.

## Quirks

- Extra operands beyond the exclusive arity are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP as Rt encodes as ZR (see bugs).
- W register as base and XZR as base encode as Xn/SP (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- get_reg discards is_64 for STLXR status, so X-as-Ws encodes (see bugs).
- forced_size overrides data width, so ldaxrb Xt encodes as Wt (see bugs).
- Mem { base, .. } ignores offset, so nonzero exclusive offset encodes as [Xn] (see bugs).
- No Ws-vs-Rt/Rn overlap check (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_ldaxr_stlxr (arity / extra / SP / FP / W-base / XZR-base / X-Ws / offset / Ws-overlap / X-data-byte / alt-spellings / MemRegOffset).
- Nine failing negative-contract properties/regressions are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldaxr_stlxr_*.md.

# Confirmed invariants (encode_uxtw)

- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty, x-1, x99, w), and non-register kinds at GPR slots always Err (1000 cases each).
- Known-answer (llvm-mc, not SUT): `uxtw x0, w1` = 0xD3407C20; `uxtw xzr, wzr` = 0xD3407FFF; `uxtw lr, w0` = 0xD3407C1E; `ubfm x0, x1, #0, #31` aliases to 0xD3407C20. SUT currently emits 32-bit ORR/MOV instead (see bugs).
- Intended success-path word (ARM ARM / llvm-mc): sf=1 opc=10 bits[28:23]=100110 N=1 immr=0 imms=31 Rn Rd. Equivalently w = 0xD3407C00 | (rn<<5) | rd. SUT does not satisfy this (MOV encoding).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM C6 UXTW: alias of UBFM Xd, Xn, #0, #31. sf=1 opc=10 N=1 immr=0 imms=31 Rn Rd. Syntax UXTW Xd, Wn only. Register 31 is XZR/WZR, never SP. Sibling encode_sxtw is SBFM (opc=00, different job). Sibling encode_uxth/uxtb use imms=15/7.
- Dispatch: encoder/mod.rs:296 `"uxtw" => encode_uxtw(operands)` (scalar only; no NEON arrangement path).
- Callers: assembler README Extensions table lists uxtw.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- is_64 from get_reg is discarded; W dest is encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Body emits 32-bit ORR (MOV Wd, Wn) instead of 64-bit UBFM (see bugs). The producing comment mentions both encodings; ARM ARM / llvm-mc / gas require UBFM.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_uxtw (arity / extra / Wd / SP / FP / nonreg / invalid name / alt-spellings / UBFM alias / ARM fields).
- Five failing properties (plus KATs/regressions) are SUT bugs, not quirks: MOV-not-UBFM, extra operand, W dest, SP-as-ZR, FP-as-GPR. See pbt-out/bug_reports/encode_uxtw_*.md.

# Confirmed invariants (encode_umull)

- Valid UMULL Xd, Wn, Wm with Rd/Rn/Rm in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR, LR, uppercase match llvm-mc (1000 cases).
- encode_umull XOR encode_smull at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- encode_umull(Xd,Wn,Wm) = encode_umaddl(Xd,Wn,Wm,XZR) = llvm-mc of both mnemonics (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 101, Rm at [20:16], o0=0 at 15, Ra=11111 at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9BA07C00 | (rm<<16) | (rn<<5) | rd.
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `umull x0, w1, w2` = 0x9ba27c20; `umull xzr, wzr, wzr` = 0x9bbf7fff; `umull lr, w1, w2` = 0x9ba27c3e; `umaddl x0, w1, w2, xzr` aliases to 0x9ba27c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) UMULL: alias of UMADDL with Ra=XZR. sf=1 op54=00 11011 U=1 01 Rm o0=0 Ra=11111 Rn Rd. Syntax UMULL Xd, Wn, Wm. Register 31 is XZR/WZR, never SP. Sibling encode_smull is U=0 (different job).
- Dispatch: encoder/mod.rs:258-267 `"umull"` + first operand RegArrangement => NEON path, else scalar encode_umull.
- Callers: assembler README Data Processing table lists umull.

## Quirks

- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_umull (arity / extra / width / SP / FP / nonreg / invalid name / alt-spellings / U bit / alias / ARM fields).
- Four failing negative-contract properties are SUT bugs, not quirks: extra operand, wrong width, SP-as-ZR, FP-as-GPR. See pbt-out/bug_reports/encode_umull_*.md.

# Confirmed invariants (encode_neon_rbit)

- Valid RBIT Vd.T, Vn.T with T in {8b,16b} and Vd/Vn in v0..v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Q bit: encode(.8b) XOR encode(.16b) = 1<<30 (1000 cases).
- Success-path word: 0 Q 1 01110 01 10000 00101 10 Rn Rd. Equivalently w = (Q<<30) | 0x2E605800 | (rn<<5) | rd.
- Rd+1 adds 1; Rn+1 adds 32 (1000 cases).
- Fewer than 2 operands, dest T not in {8b,16b}, Imm/Mem/Shift/RegList/Label dest, Imm source, and invalid names (v32, foo, empty, v, v99, v-1) always Err.
- Known-answer: `rbit v0.8b, v1.8b` = 0x2e605820; `rbit v31.16b, v0.16b` = 0x6e60581f; `rbit v31.8b, v31.8b` = 0x2e605bff.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD two-register miscellaneous RBIT (vector): T is 8B or 16B. Encoding 0 Q 1 01110 01 10000 00101 10 Rn Rd.
- Dispatch: encoder/mod.rs:902-909 `"rbit"` + first operand RegArrangement => encode_neon_rbit, else scalar encode_rbit.
- Parser lowercases arrangements; `is_register` accepts x/w/d/s/q/v/h/b and sp/wsp/xzr/wzr/lr, so `x0.8b` and `sp.8b` are caller-reachable.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Operand::Reg source is accepted (see bugs).
- parse_reg_num accepts x/w/d/s/q/h/b prefixes and maps sp to 31 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of encode_neon_rbit (arity / extra / T / Q / Rd / Rn / mismatch / bare src / Imm / invalid name / prefix / SP).

# Confirmed invariants (encode_umulh)

- Valid UMULH Xd, Xn, Xm with Rd/Rn/Rm in 0..31 (xzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31, XZR, LR, uppercase Xn match llvm-mc (1000 cases).
- encode_umulh XOR encode_smulh at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 110, Rm at [20:16], o0=0 at 15, Ra=11111 at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9BC07C00 | (rm<<16) | (rn<<5) | rd.
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `umulh x0, x1, x2` = 0x9bc27c20; `umulh xzr, xzr, xzr` = 0x9bdf7fff; `umulh lr, x1, x30` = 0x9bde7c3e; `umulh x0, x1, xzr` = 0x9bdf7c20; `smulh x0, x1, x2` = 0x9b427c20 (XOR = 1<<23).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) UMULH: sf=1 op54=00 11011 op31=110 Rm o0=0 Ra=11111 Rn Rd. Syntax UMULH Xd, Xn, Xm. No 32-bit form. Register 31 is XZR, never SP. Sibling encode_smulh is U=0 (different job).
- Dispatch: encoder/mod.rs:274 `"umulh" => encode_umulh(operands)` (scalar only; no NEON arrangement path).
- Callers: assembler README Data Processing table lists umulh.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- is_64 from get_reg is discarded; W registers are encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr).

---

# Confirmed invariants (encode_umaddl)

- Valid UMADDL Xd, Wn, Wm, Xa with Rd/Rn/Rm/Ra in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR/WZR, LR, uppercase Xn/Wn match llvm-mc (1000 cases).
- encode_umaddl(Xd, Wn, Wm, XZR) equals encode_umull(Xd, Wn, Wm), and both match llvm-mc `umaddl ..., xzr` / `umull` (1000 cases).
- encode_umaddl XOR encode_smaddl at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 101, Rm at [20:16], o0=0 at 15, Ra at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9BA00000 | (rm<<16) | (ra<<10) | (rn<<5) | rd.
- Fewer than 4 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `umaddl x0, w1, w2, x3` = 0x9ba20c20; `umaddl xzr, wzr, wzr, xzr` = 0x9bbf7fff; `umaddl lr, w1, w2, x30` = 0x9ba2783e; `umaddl x0, w1, w2, xzr` / `umull x0, w1, w2` aliases to 0x9ba27c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) UMADDL: sf=1 U=1 11011 101 Rm o0=0 Ra Rn Rd. UMULL Xd, Wn, Wm is the alias of UMADDL Xd, Wn, Wm, XZR. Register 31 is XZR/WZR, never SP/WSP. Dest and accumulator are Xd/Xa; multiply sources are Wn/Wm.
- Dispatch: encoder/mod.rs:270 `"umaddl" => encode_umaddl(operands)` (scalar only; no NEON arrangement path).
- Sibling encode_umull is the same format with Ra=XZR (alias). Sibling encode_smaddl is U=0 (different job).
- Callers: assembler README Data Processing table lists umaddl. No codegen emission of scalar umaddl found.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- is_64 from get_reg is discarded; W dest, X sources, and W acc are encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr).

---

# Confirmed invariants (encode_sxtw)

- Valid SXTW Xd, Wn with Rd/Rn in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR, LR, uppercase, and Xd,Xn (llvm-mc canonicalizes to Xd,Wn) match llvm-mc (1000 cases).
- encode_sxtw(Xd, Wn) equals encode_sbfm(Xd, Xn, #0, #31), and both match llvm-mc `sxtw` (1000 cases).
- Success-path word: sf=1 at bit 31, opc=00 at [30:29], 100110 at [28:23], N=1 at 22, immr=0 at [21:16], imms=31 at [15:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x93407C00 | (rn<<5) | rd.
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `sxtw x0, w1` = 0x93407c20; `sxtw xzr, wzr` = 0x93407fff; `sxtw lr, w0` = 0x93407c1e; `sbfm x0, x1, #0, #31` aliases to the same word as `sxtw x0, w1`.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM SXTW is the alias of SBFM Xd, Xn, #0, #31: sf=1 00 100110 N=1 immr=0 imms=31 Rn Rd. Assembler syntax: SXTW Xd, Wn only (no Wd form). Register 31 is XZR/WZR, never SP/WSP. llvm-mc also accepts SXTW Xd, Xn (canonicalizes source to W).
- Dispatch: encoder/mod.rs:293 `"sxtw" => encode_sxtw(operands)`.
- Sibling encode_sbfm is the same format with caller immr/imms (alias at #0,#31). Sibling encode_sxth is imms=15 (different job). Sibling encode_uxtw is UBFM/MOV (different job).
- Callers: assembler README Extensions table lists sxtw. Codegen emits `sxtw x0, w0` in cast_ops.rs / atomics.rs / f128.rs / alu.rs / peephole.rs.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- Dest/src width from get_reg is discarded; W dest is encoded as 64-bit SXTW (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / W dest / SP / FP / non-Reg / invalid name / x31 / uppercase / lr / Xd,Xn).

# Confirmed invariants (encode_sxth)

- Valid SXTH Wd, Wn and Xd, Wn with Rd/Rn in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR, LR, uppercase, and Xd,Xn (llvm-mc canonicalizes to Xd,Wn) match llvm-mc (1000 cases).
- encode_sxth(Rd, Rn) equals encode_sbfm(Rd, Rn, #0, #15) with matching dest width, and both match llvm-mc `sxth` (1000 cases).
- Success-path word: sf at bit 31, opc=00 at [30:29], 100110 at [28:23], N=sf at 22, immr=0 at [21:16], imms=15 at [15:10], Rn at [9:5], Rd at [4:0]. Equivalently w = (is_64 ? 0x93403C00 : 0x13003C00) | (rn<<5) | rd.
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `sxth w0, w1` = 0x13003c20; `sxth x0, w1` = 0x93403c20; `sxth wzr, wzr` = 0x13003fff; `sbfm w0, w1, #0, #15` aliases to the same word as `sxth w0, w1`.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM SXTH is the alias of SBFM Rd, Rn, #0, #15: sf 00 100110 N=sf immr=0 imms=15 Rn Rd. Assembler syntax: SXTH Wd, Wn or SXTH Xd, Wn. Register 31 is WZR/XZR, never SP/WSP. llvm-mc also accepts SXTH Xd, Xn (canonicalizes source to W).
- Dispatch: encoder/mod.rs:294 `"sxth" => encode_sxth(operands)`.
- Sibling encode_sbfm is the same format with caller immr/imms (alias at #0,#15). Sibling encode_sxtb is imms=7 (different job). Sibling encode_uxth is UBFM opc=10 (different job).
- Callers: assembler README Extensions table lists sxth. Codegen emits `sxth x0, w0` in cast_ops.rs / atomics.rs / f128.rs.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- is_64 from source get_reg is discarded; W dest + X source is encoded as 32-bit SXTH (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr / Xd,Xn).

---

# Confirmed invariants (encode_smull)

- Valid SMULL Xd, Wn, Wm with Rd/Rn/Rm in 0..31 (xzr/wzr at 31, lr as X30) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Alternate spellings x31/w31, XZR/WZR, LR, uppercase Xn/Wn match llvm-mc (1000 cases).
- encode_smull(Xd, Wn, Wm) equals encode_smaddl(Xd, Wn, Wm, XZR) and both match llvm-mc `smull` / `smaddl ..., xzr` (1000 cases).
- encode_smull XOR encode_umull at equal registers = 1<<23 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=1, bits[30:21]=00 11011 001, Rm at [20:16], o0=0 at 15, Ra=11111 at [14:10], Rn at [9:5], Rd at [4:0]. Equivalently w = 0x9B207C00 | (rm<<16) | (rn<<5) | rd.
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `smull x0, w1, w2` = 0x9b227c20; `smull xzr, wzr, wzr` = 0x9b3f7fff; `smull lr, w1, w2` = 0x9b227c3e; `smaddl x0, w1, w2, xzr` aliases to the same word.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) SMADDL: sf=1 U=0 11011 001 Rm o0=0 Ra Rn Rd. SMULL Xd, Wn, Wm is the alias of SMADDL Xd, Wn, Wm, XZR. Register 31 is XZR/WZR, never SP/WSP. Dest is Xd; sources are Wn/Wm.
- Dispatch: encoder/mod.rs:247-256 `"smull"` with RegArrangement goes to NEON; otherwise encode_smull (scalar). This campaign tests only the scalar helper.
- Sibling encode_smaddl is the same format with caller Ra (alias at Ra=XZR). Sibling encode_umull is U=1 (different job).
- Callers: assembler README Data Processing table lists smull. No codegen emission of scalar smull found.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- is_64 from get_reg is discarded; W dest and X sources are encoded (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / extra / width / SP / FP / non-Reg / invalid name / x31 / uppercase / lr).

---

# Confirmed invariants (encode_shift)

- Valid GP dest matching mnemonic size b/w/l/q, Imm count 0..255 matches llvm-mc `-triple=x86_64 -show-encoding` (1000 cases).
- Valid %cl count form matches llvm-mc (1000 cases).
- 1-operand form matches llvm-mc omitted-count encoding (1000 cases).
- 1-operand encoding equals Imm(1) two-operand encoding (GAS omitted count is 1) (1000 cases).
- Changing only Group 2 /digit (ROL/ROR/RCL/RCR/SHL/SHR/SAR) differs only in ModR/M bits [5:3] (1000 cases).
- Memory dest without segment, including (%rsp)/(%r12) SIB and (%rbp)/(%r13) disp8, matches llvm-mc (1000 cases).
- RIP-relative memory with trailing imm8 (count 2..255) produces one R_X86_64_PC32 reloc with addend -5 (1000 cases).
- Arity 0/3/4 and non-CL register count always Err (1000 cases).
- 1-operand Imm/Label/Indirect always Err (1000 cases).
- Known-answer: `shlq $1, %rax` / `shlq %rax` = [0x48,0xd1,0xe0]; `shll $1, %eax` = [0xd1,0xe0]; `shlw $1, %ax` = [0x66,0xd1,0xe0]; `shlb $1, %al` = [0xd0,0xe0]; `shlq %cl, %rax` = [0x48,0xd3,0xe0]; `shlq $2, %rax` = [0x48,0xc1,0xe0,0x02].

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=x86_64 -show-encoding
- Intel SDM Group 2: D0/D2/C0 r/m8; D1/D3/C1 r/m16/32/64; /0 ROL /1 ROR /2 RCL /3 RCR /4 SHL/SAL /5 SHR /7 SAR. 66 prefix for 16-bit; REX.W for 64-bit. Count is 1, CL, or imm8. GAS omitted count is 1. AT&T operand order is count, dest. SAL is alias of SHL.
- Dispatch: encoder/mod.rs:196-200,670-671 suffixed forms; suffix-less shl/sal/shr/sar/rol/ror/rcl/rcr go through encode_suffixless_shift then encode_shift.
- Callers: assembler README Shifts/Rotates table; codegen/emit.rs:151-153 emits shll/shlq, sarl/sarq, shrl/shrq.
- Siblings encode_double_shift (SHLD/SHRD), encode_sse_shift, encode_avx_shift, encode_bmi2_shift are different jobs.

## Quirks

- FS/GS segment override is not emitted (see bugs).
- Size-mismatched and non-GP dest registers are encoded via reg_num aliases (see bugs).
- Imm count is truncated with `as u8` so 256 encodes as 0 (see bugs).
- llvm-mc accepts `$ -1` as 255; SUT does the same via `as u8` (agreement, not a bug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (1-op Reg/Mem/other, arity, Imm+Reg, Imm+Mem count==1 vs else, CL+Reg, CL+Mem, RIP reloc addend, extra/non-CL, mixed size, non-GP, segment).

---

# Confirmed invariants (encode_sbc)

- Valid three-GPR same-width SBC/SBCS with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_sbc XOR encode_sbc(set_flags=true) = 1<<29 (ARM ARM S bit) (1000 cases).
- encode_sbc XOR encode_adc at equal operands = 1<<30 (ARM ARM op SBC=1 vs ADC=0) (1000 cases).
- encode_sbc(Rd, ZR, Rm, set_flags) equals llvm-mc `ngc`/`ngcs` Rd, Rm (ARM ARM NGC alias) (1000 cases).
- `lr` in any slot encodes as X30 and matches llvm-mc (1000 cases).
- Success-path word: sf at 31, op=1 at 30, S at 29, bits [28:21]=11010000, Rm at [20:16], bits [15:10]=000000, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `sbc x0, x1, x2` encodes as 0xda020020; `sbcs w0, w1, w2` as 0x7a020020; `ngc x0, x1` / `sbc x0, xzr, x1` as 0xda0103e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Add/subtract (with carry) SBC: `sf 1 S 11010000 Rm 000000 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd/Rn/Rm same width. No shifted-register form. NGC Rd, Rm is alias of SBC Rd, ZR, Rm.
- `lr` is a 64-bit alias of X30 (llvm-mc and parse_reg_num).
- Dispatch: encoder/mod.rs:283-284 `"sbc" => encode_sbc(operands, false)`, `"sbcs" => encode_sbc(operands, true)`. Sibling encode_adc is ADC (op=0), different job.
- Callers: assembler README data-processing table lists sbc/sbcs; codegen/i128_ops.rs:73 emits `sbc x1, x3, x5`.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..2 / sf / S / extra / mixed / SP / FP / lr / invalid name / non-Reg).

---

# Confirmed invariants (encode_ret)

- Valid RET with omitted Rn or Rn in {x0–x30, xzr, lr} (including uppercase X0) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Bare `ret` ≡ `ret x30` ≡ `ret lr` = Word(0xd65f03c0) (ARM ARM omitted Xn is X30).
- encode_ret XOR encode_br at equal Rn = 1<<22 (ARM ARM opc RET=0010 vs BR=0000) (1000 cases).
- Success-path word: bits[31:25]=1101011, opc[24:21]=0010, op2[20:16]=11111, op3[15:10]=000000, Rn[9:5], op4[4:0]=00000; w = 0xd65f0000 | (rn << 5).
- Non-register operand kinds (Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label) always Err.
- Invalid register names (x32, w32, foo, empty, r0, x, x-1, x99) always Err.
- Known-answer: `ret` encodes as 0xd65f03c0; `ret x0` as 0xd65f0000; `ret xzr` as 0xd65f03e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Unconditional branch (register) RET: bits[31:25]=1101011 opc=0010 op2=11111 op3=000000 Rn[9:5] op4=00000. Omitted Xn defaults to X30. Rn is Xn; register 31 is XZR, never SP.
- Dispatch: encoder/mod.rs:320 `"ret" => encode_ret`. Sibling encode_br is BR (opc=0000), different job. Sibling encode_blr is BLR (opc=0001), different job.
- Callers: assembler README Branches table lists ret; codegen/prologue.rs:319 emits bare `ret`.

## Quirks

- Extra operands beyond index 0 are ignored (see bugs).
- W-form Rn is accepted and encoded as the matching X register (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as XZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- llvm-mc accepts `ret x31` as `ret xzr`; SUT parse_reg_num also maps x31 to 31 (agreement, not a bug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (empty-default / get_reg success / get_reg None / get_reg other / extra / W / SP / FP).

---

# Confirmed invariants (encode_orn)

- Valid three-GPR same-width ORN with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr and optional LSL/LSR/ASR/ROR in range matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid vector ORN with T in {8b,16b}, Vd/Vn/Vm in v0–v31 matches llvm-mc (1000 cases).
- encode_orn XOR encode_logical(opc=01) = 1<<21 (ARM ARM N bit vs ORR) (1000 cases).
- encode_orn(Rd, ZR, Rm, shift) equals encode_mvn(Rd, Rm, shift) and llvm-mc `orn Rd, ZR, Rm` (documented MVN alias) (1000 cases).
- encode_orn(X-ops) XOR encode_orn(W-ops) at equal register numbers and amt in 0..31 = 1<<31 (ARM ARM sf) (1000 cases).
- Vector T=8b XOR T=16b at equal Rd/Rn/Rm = 1<<30 (ARM ARM Q) (1000 cases).
- Success-path GPR word: sf at 31, opc=01 at [30:29], bits [28:24]=01010, shift at [23:22], N=1 at 21, Rm at [20:16], imm6 at [15:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at GPR slots always Err.
- Known-answer: `orn x0, x1, x2` encodes as 0xaa220020; `orn w0, w1, w2` as 0x2a220020; `orn v0.8b, v1.8b, v2.8b` as 0x0ee21c20; `orn v0.16b, v1.16b, v2.16b` as 0x4ee21c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Logical (shifted register) ORN: `sf 01 01010 shift N=1 Rm imm6 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd/Rn/Rm same width. shift in {LSL,LSR,ASR,ROR}. imm6 0..31 (sf=0) or 0..63 (sf=1).
- ARM ARM Advanced SIMD three-same ORN: `0 Q 0 01110 11 1 Rm 00011 1 Rn Rd`. T in {8B,16B}. Q=1 iff T=16B.
- GNU as / llvm-mc alias: `orn Rd, Rn, #imm` encodes as `orr Rd, Rn, #~imm`.
- Documented MVN alias at data_processing.rs:753: MVN Rd, Rm -> ORN Rd, XZR, Rm.
- Dispatch: encoder/mod.rs:235 `"orn" => encode_orn`. Sibling encode_eon is EON (opc=10), different job. Sibling encode_logical(opc=01) is ORR (N=0), different job.
- Callers: assembler README data-processing and NEON three-same tables list orn.

## Quirks

- Immediate form is not implemented (see bugs).
- Extra operands beyond the optional shift are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Unknown shift kind defaults to LSL (see bugs).
- Shift amount is masked with 0x3F; 32-bit amounts 32..63 encode UNALLOCATED imm6<5>=1 (see bugs).
- NEON T other than 16b encodes Q=0, including 8h/4h/4s/2s/2d/1d (see bugs).
- Source NEON arrangements are discarded; get_neon_reg accepts Operand::Reg (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Imm / Shift / NEON T / extra / get_reg kinds / Q / sf).

---

# Confirmed invariants (encode_neon_tbl)

- Valid vector TBL with Ta in {8b,16b}, Vd/Vm in v0–v31, 1–4 consecutive wrapping table registers all .16B matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Ta=8b XOR Ta=16b at equal Rd/Rn/Rm/len = 1<<30 (1000 cases).
- Changing only nregs in {1,2,3,4} differs only in len bits [14:13]; len = nregs-1 (1000 cases).
- Success-path word: bit 31=0, Q at 30, bits [29:24]=001110, bits [23:21]=000, Rm at [20:16], bit 15=0, len at [14:13], op=0 at 12, bits [11:10]=00, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, missing RegList, and invalid dest names always Err.
- Known-answer: `tbl v0.8b, {v1.16b}, v2.8b` encodes as 0x0e020020; `tbl v0.16b, {v1.16b}, v2.16b` as 0x4e020020; 2-reg 0x0e032020; 3-reg 0x4e044020; 4-reg 0x0e056020; wrap `{v31.16b, v0.16b}` as 0x0e0223e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD table lookup TBL: `0 Q 00 1110 00 0 Rm 0 len op 00 Rn Rd` with op=0. Ta in {8B,16B}. Table is 1–4 consecutive .16B registers wrapping at 31. Vm.Ta matches Vd.Ta. Q=1 iff Ta=16B.
- Dispatch: encoder/mod.rs:729 `"tbl" => encode_neon_tbl`. Sibling encode_neon_tbx is TBX (op=1), different job.
- Callers: assembler README NEON permute table lists tbl/tbx.
- Parser `parser.rs:2030-2072` builds Operand::RegList; rejects empty lists; range syntax expands wrapping consecutives. Encoder still panics if given an empty list directly.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Ta other than 16b encodes Q=0, including 4h/8h/2s/4s/2d/1d (see bugs).
- Empty RegList panics on regs[0] (see bugs).
- nregs>4 wraps via `(num_regs-1)&0x3` (see bugs).
- Only first list register number and len are encoded; later names/arrangements and sequentiality are ignored (see bugs).
- get_neon_reg accepts Operand::Reg, so GPR dest/Vm and bare V in the list encode (see bugs).
- Vm arrangement is discarded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Ta / table list / get_neon_reg Reg dest+Vm / extra / mismatched T).

---

# Confirmed invariants (encode_neon_shift_imm)

- Valid vector USHR with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [1, esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-esize Q=0 vs Q=1 arrangements XOR = 1<<30 (1000 cases).
- Success-path word: bit 31=0, Q at 30, U=1 at 29, bits [28:23]=011110, immh:immb at [22:16]=2*esize-shift, opcode at [15:10]=000001, Rn at [9:5], Rd at [4:0].
- T=1d (Reserved Q=0 && esize==64), fewer than 3 operands, GPR/FP dest, and invalid dest names always Err.
- Known-answer: `ushr v0.8b, v1.8b, #1` encodes as 0x2f0f0420; `ushr v0.16b, v1.16b, #8` as 0x6f080420; `ushr v0.4h, v1.4h, #1` as 0x2f1f0420; `ushr v0.8h, v1.8h, #16` as 0x6f100420; `ushr v0.2s, v1.2s, #1` as 0x2f3f0420; `ushr v0.4s, v1.4s, #32` as 0x6f200420; `ushr v0.2d, v1.2d, #1` as 0x6f7f0420; `#64` as 0x6f400420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate USHR: `0 Q 1 011110 immh immb 00000 1 Rn Rd`. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 is Reserved. shift = (2*esize)-UInt(immh:immb) in [1, esize].
- Dispatch: encoder/mod.rs:658-659 uses encode_neon_ushr / encode_neon_sshr. This symbol is a dead `pub(crate)` helper (`#![allow(dead_code)]`). Documented job remains USHR (neon.rs:372).
- Callers: none. Assembler README NEON shifts table lists ushr/sshr.

## Quirks

- `_is_unsigned` is unused (Rust `_` prefix); U is hardcoded to 1. Docstring says USHR.
- Source arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- Negative Imm panics in debug; shift 0 / esize+1 wrap/mask (see bugs).
- Shift is `get_imm` then `as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- get_neon_reg accepts Operand::Reg, so a bare GPR/FP/V source encodes as Rn (see bugs). Dest as Operand::Reg still Errs via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / T / get_imm as u32 / get_neon_reg Reg dest+source / extra / mismatched T).

---

# Confirmed invariants (encode_negs)

- Valid two-GPR same-width NEGS with Rd/Rm in x0–x30/xzr or w0–w30/wzr and optional LSL/LSR/ASR in range matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_negs(Rd, Rm, shift) equals llvm-mc `negs Rd, Rm, shift` and llvm-mc `subs Rd, ZR, Rm, shift` (1000 cases). Documented alias at data_processing.rs:728.
- encode_negs(X-ops) XOR encode_negs(W-ops) at equal register numbers and amt in 0..31 = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, op=1 at 30, S=1 at 29, bits [28:24]=01011, shift at [23:22], bit 21=0, Rm at [20:16], imm6 at [15:10], Rn=31 at [9:5], Rd at [4:0].
- `lr` in either slot encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-register kinds at Rd/Rm always Err.
- Known-answer: `negs x0, x1` / `subs x0, xzr, x1` encode as 0xeb0103e0; `negs w0, w1` as 0x6b0103e0.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Add/subtract (shifted register) NEGS (alias of SUBS): `sf 1 1 01011 shift 0 Rm imm6 11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd and Rm same width. Exactly two registers plus optional shift. shift in {LSL,LSR,ASR} (not ROR). imm6 0..31 (sf=0) or 0..63 (sf=1).
- `lr` is a 64-bit alias of X30 (llvm-mc and parse_reg_num).
- Dispatch: encoder/mod.rs:279 `"negs" => encode_negs`. Sibling encode_neg is SUB (S=0), different job.
- Callers: assembler README data-processing table; no codegen emission of `negs` found.

## Quirks

- Extra operands beyond the optional shift are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Unknown shift kind (including ROR) defaults to LSL (see bugs).
- Shift amount is masked with 0x3F; 32-bit amounts 32..63 encode UNALLOCATED imm6<5>=1 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..1 / sf / Shift at 2 / shift-kind / imm6 / Rn=31 / invalid name / non-Reg).

---

# Confirmed invariants (encode_neg)

- Valid two-GPR NEG with ABI names / x0–x31 / fp matches llvm-mc `-triple=riscv64 -show-encoding` (1000 cases).
- encode_neg(rd, rs) equals llvm-mc `sub rd, x0, rs` (1000 cases). Documented expansion at README.md:321.
- encode_neg(rd, rs) equals encode_alu_reg([rd, x0, rs], funct3=000, funct7=0100000) (1000 cases).
- Success-path word: opcode[6:0]=0110011, rd[11:7], funct3[14:12]=000, rs1[19:15]=0, rs2[24:20], funct7[31:25]=0100000.
- ABI names and xN (and fp/s0) of the same number encode identically (1000 cases).
- Imm(n) for n in 0..=31 encodes as register xN (get_reg GCC inline-asm extension) (1000 cases).
- Fewer than 2 operands, FP/vector/unknown names, out-of-range Imm, and non-Reg kinds always Err.
- Known-answer: `neg a0, a1` / `sub a0, x0, a1` encode as 0x40b00533; `neg zero, zero` as 0x40000033; `neg t6, ra` / `neg x31, x1` as 0x40100fb3; `neg fp, s0` / `neg x8, x8` as 0x40800433.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=riscv64 -show-encoding
- RISC-V Unprivileged ISA: NEG rd, rs = SUB rd, x0, rs. SUB R-type opcode OP=0110011, funct3=000, funct7=0100000, rs1=x0.
- Dispatch: encoder/mod.rs:749 `"neg" => encode_neg`. Sibling encode_negw is SUBW (out of scope).
- Callers: alu.rs:23 `neg t0, t0`; atomics.rs:450 `neg t2, t2`; intrinsics.rs `neg t3/t5`.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- get_reg accepts Imm 0..=31 as a register number (Doc evidence: encoder/mod.rs:351-352 GCC inline asm).
- reg_num case-folds; llvm-mc rejects uppercase ABI names. SUT is more lenient on codegen-emitted lowercase assembly.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg Reg/Imm/other/missing, extra operand, SUB expansion vs llvm-mc).

---

# Confirmed invariants (encode_neon_shift_right)

- Valid vector SRSHR/URSHR/SSRA/USRA/SRSRA/URSRA with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [1, esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29 (ARM ARM U bit) (1000 cases).
- Same-esize Q=0 vs Q=1 arrangements XOR = 1<<30; opcode pairs differ only in bits [15:10] (1000 cases).
- Success-path word: bit 31=0, Q at 30, U at 29, bits [28:23]=011110, immh:immb at [22:16]=2*esize-shift, opcode at [15:10], Rn at [9:5], Rd at [4:0].
- Shift 0, esize+1, negative, and 1d (Reserved Q=0 && esize==64) always Err; fewer than 3 operands, GPR/FP dest, invalid names, and non-matching operand kinds always Err.
- Known-answer: `srshr v0.8b, v1.8b, #1` encodes as 0x0f0f2420; `urshr v0.16b, v1.16b, #8` as 0x6f082420; `ssra v0.4h, v1.4h, #1` as 0x0f1f1420; `usra v0.8h, v1.8h, #16` as 0x6f101420; `srsra v0.2s, v1.2s, #1` as 0x0f3f3420; `ursra v0.4s, v1.4s, #32` as 0x6f203420; `srshr v0.2d, v1.2d, #1` as 0x4f7f2420; `#64` as 0x4f402420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SRSHR/URSHR/SSRA/USRA/SRSRA/URSRA: `0 Q U 011110 immh immb opcode Rn Rd`. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 is Reserved. shift = (2*esize)-UInt(immh:immb) in [1, esize]. opcode 001001 / 000101 / 001101. U=0 signed / U=1 unsigned.
- Dispatch: encoder/mod.rs:607-612. SSHR/USHR use encode_neon_sshr/ushr (out of scope).
- Callers: assembler README NEON shifts table; no codegen emission found.

## Quirks

- Source arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- Shift is `get_imm as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- get_neon_reg accepts Operand::Reg, so a bare GPR/FP/V source encodes as Rn (see bugs). Dest as Operand::Reg still Errs via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / T / get_imm as u32 / get_neon_reg Reg dest+source / extra / mismatched T).

---

# Confirmed invariants (encode_mvn)

- Valid two-GPR same-width MVN with Rd/Rm in x0–x30/xzr or w0–w30/wzr and optional LSL/LSR/ASR/ROR in range matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_mvn(Rd, Rm, shift) equals llvm-mc `mvn Rd, Rm, shift` and llvm-mc `orn Rd, ZR, Rm, shift` (1000 cases). Documented alias at data_processing.rs:753.
- encode_mvn(X-ops) XOR encode_mvn(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, opc=01 at [30:29], bits [28:24]=01010, shift at [23:22], N=1 at 21, Rm at [20:16], imm6 at [15:10], Rn=31 at [9:5], Rd at [4:0].
- Valid NEON MVN with T in {8b,16b}, Vd/Vn in v0–v31, matches llvm-mc `mvn` and llvm-mc `not` (1000 cases).
- `lr` in either scalar slot encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands always Err.
- Known-answer: `mvn x0, x1` encodes as 0xaa2103e0; `mvn w0, w1` as 0x2a2103e0; `orn x0, xzr, x1` as 0xaa2103e0; `mvn v0.16b, v1.16b` / `not v0.16b, v1.16b` as 0x6e205820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Logical (shifted register) MVN (alias of ORN): `sf 01 01010 shift 1 Rm imm6 11111 Rd`. Register 31 is XZR/WZR, never SP/WSP. Rd and Rm same width. Exactly two registers plus optional shift. shift in {LSL,LSR,ASR,ROR}. imm6 0..31 (sf=0) or 0..63 (sf=1).
- ARM ARM Advanced SIMD NOT (vector, alias MVN): `0 Q 1 01110 00 10000 00101 10 Rn Rd`. T in {8B,16B} only.
- `lr` is a 64-bit alias of X30 (llvm-mc and parse_reg_num).
- Dispatch: encoder/mod.rs:280 `"mvn" => encode_mvn`. NEON dest is branched inside encode_mvn to encode_neon_not.
- Callers: alu.rs:26 `mvn x0, x0`; i128_ops.rs:43-51 `mvn x0, x0` / `mvn x1, x1`; inline_asm.rs:354 `mvn dest, dest`.

## Quirks

- Extra operands beyond the optional shift are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Unknown shift kind defaults to LSL (see bugs).
- Shift amount is masked with 0x3F; 32-bit amounts 32..63 encode UNALLOCATED imm6<5>=1 (see bugs).
- encode_neon_not sets Q from dest=="16b" only; T not in {8b,16b} and mismatched source T are accepted (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..1 / sf / Shift at 2 / shift-kind / imm6 / Rn=31 / neon Q / neon extra).

---

# Confirmed invariants (encode_mul)

- Valid three-GPR same-width MUL with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_mul(Rd, Rn, Rm) equals llvm-mc `mul Rd, Rn, Rm` and llvm-mc `madd Rd, Rn, Rm, ZR` (1000 cases). Documented alias at data_processing.rs:589.
- encode_mul(X-ops) XOR encode_mul(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:21]=0011011000, Rm at [20:16], o0=0 at 15, Ra=31 at [14:10], Rn at [9:5], Rd at [4:0].
- Valid NEON MUL with T in {8b,16b,4h,8h,2s,4s}, Vd/Vn/Vm in v0–v31, matches llvm-mc (1000 cases).
- `lr` in any of the three scalar slots encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 3 operands, non-register operands (Imm/Symbol/Mem/Shift/Cond/Label), and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `mul x0, x1, x2` encodes as 0x9b027c20; `mul w0, w1, w2` as 0x1b027c20; `madd x0, x1, x2, xzr` as 0x9b027c20; `mul v0.16b, v1.16b, v2.16b` as 0x4e229c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) MUL (alias of MADD): `sf 00 11011 000 Rm 0 11111 Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All three registers same width. Exactly three operands. o0 (bit 15) is 0. Ra (bits 14:10) is 31.
- ARM ARM Advanced SIMD MUL (vector): `0 Q 0 01110 size 1 Rm 10011 1 Rn Rd`. T in {8B,16B,4H,8H,2S,4S}. size==11 is UNDEFINED.
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:238-244 mul. NEON dest is routed to encode_neon_three_same / encode_neon_elem; scalar dest to encode_mul. encode_mul itself still has a RegArrangement branch to encode_neon_mul.
- Callers: alu.rs:168,202 `mul w0, w1, w2` / `mul x0, x1, x2`; i128_ops.rs:77 `mul x0, x2, x4`.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- neon_arr_to_q_size accepts 1d/2d, so size==11 encodes (see bugs).
- encode_neon_mul uses dest arrangement only; source T is discarded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..2 / sf / Ra=31 / neon_arr_to_q_size / source T).

---

# Confirmed invariants (encode_msub)

- Valid four-GPR same-width MSUB with Rd/Rn/Rm/Ra in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_msub(Rd, Rn, Rm, ZR) equals llvm-mc `mneg Rd, Rn, Rm` and llvm-mc `msub Rd, Rn, Rm, ZR` (1000 cases). Documented alias at data_processing.rs:677.
- encode_msub(X-ops) XOR encode_msub(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:21]=0011011000, Rm at [20:16], o0=1 at 15, Ra at [14:10], Rn at [9:5], Rd at [4:0].
- `lr` in any of the four slots encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 4 operands, non-register operands (Imm/Symbol/Mem/Shift/Cond/Label), and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `msub x0, x1, x2, x3` encodes as 0x9b028c20; `msub w0, w1, w2, w3` as 0x1b028c20; `msub x0, x1, x2, xzr` / `mneg x0, x1, x2` as 0x9b02fc20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) MSUB: `sf 00 11011 000 Rm 1 Ra Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All four registers same width. Exactly four operands. o0 (bit 15) is 1 (MADD is 0).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:246 msub.
- Callers: alu.rs:178,183,207,211 emit `msub w0, w3, w2, w1` / `msub x0, x3, x2, x1` for remainder.
- Sibling encode_mneg data_processing.rs:677 documents MNEG as MSUB with Ra=XZR.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..3 / sf / parse_reg_num lr / invalid / non-Reg / extra / mixed / SP / FP).

---

# Confirmed invariants (encode_neon_qshrn)

- Valid vector SQSHRN/UQSHRN/SQRSHRN/UQRSHRN (+2) with Ta in {8h,4s,2d}, Tb matching Ta and the 2-suffix, Vd/Vn in v0–v31, shift in [1, dest_esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., is_high=false) XOR encode(..., is_high=true) = 1<<30 (ARM ARM Q bit) (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29 (ARM ARM U bit) (1000 cases).
- encode(..., is_rounding=false) XOR encode(..., is_rounding=true) = 1<<11 (opcode 100101 vs 100111) (1000 cases).
- Success-path word: bit 31=0, Q at 30, U at 29, bits [28:23]=011110, immh:immb at [22:16]=src_esize-shift, opcode at [15:10]=100101/100111, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, unsupported source Ta (not 8h/4s/2d), non-RegArrangement/non-Imm kinds, invalid names (v32, foo, empty, v, v-1), and bare Operand::Reg source always Err.
- Known-answer: `sqshrn v0.8b, v1.8h, #1` encodes as 0x0f0f9420; `#8` as 0x0f089420; `sqshrn2 v0.16b, v1.8h, #1` as 0x4f0f9420; `uqshrn v0.8b, v1.8h, #1` as 0x2f0f9420; `sqrshrn v0.8b, v1.8h, #1` as 0x0f0f9c20; `uqrshrn2 v0.4s, v1.2d, #32` as 0x6f209c20; `sqshrn v0.4h, v1.4s, #16` as 0x0f109420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SQSHRN/UQSHRN/SQRSHRN/UQRSHRN: `0 Q U 011110 immh immb opcode Rn Rd`. opcode 100101 non-rounding / 100111 rounding. U=0 signed / U=1 unsigned. Q=0 lower half / Q=1 (`2` suffix) upper half. dest_esize = 8<<HighestSetBit(immh); shift = 2*esize - UInt(immh:immb) in [1, dest_esize]. Ta/Tb: 8H→8B/16B (1..8), 4S→4H/8H (1..16), 2D→2S/4S (1..32).
- Dispatch: encoder/mod.rs:637-648. Scalar sqshrn (non-arrangement dest) is encode_neon_scalar_qshrn, out of scope.
- Sibling encode_neon_shrn neon.rs:1443-1444 checks `shift > half_bits` with half_bits = source/2.

## Quirks

- Shift range uses source element size (16/32/64), so dest_esize+1 through source_esize encode (see bugs).
- Dest arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- get_neon_reg accepts Operand::Reg, so GPR/FP dest encodes as Vd (see bugs). Reachable from uqshrn/sqshrn2/sqrshrn/uqrshrn (+2).
- Shift is `get_imm as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Ta / get_imm as u32 / get_neon_reg Reg dest+source).

---

# Confirmed invariants (encode_movz)

- Valid GPR + imm16 + optional lsl (hw in {0,1} for W, {0,1,2,3} for X; Rd=31 is xzr/wzr) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_movz(X-ops) XOR encode_movz(W-ops) at equal rd/imm/hw in {0,1} = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:23]=10100101 (opc=10), hw at [22:21], imm16 at [20:5], Rd at [4:0].
- Constant `:abs_g0:`/`:abs_g1:`/`:abs_g2:`/`:abs_g3:` (and `_nc`) encode the extracted 16-bit chunk and match llvm-mc of the resolved `movz Rd, #chunk [, lsl #shift]` (1000 cases).
- `lr` as Rd encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-imm16 second operands (unknown modifier, non-constant abs_g symbol, Symbol/Label/Mem/Reg) always Err.
- Known-answer: `movz x0, #42` encodes as 0xd2800540; `movz w0, #42` as 0x52800540; `movz x0, #42, lsl #16` as 0xd2a00540.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Move wide (immediate) MOVZ: `sf 10 100101 hw imm16 Rd`. Register 31 is XZR/WZR, never SP/WSP. imm16 in [0, 65535]. hw in {0,1} when sf=0; {0,1,2,3} when sf=1. Semantics: Rd := ZeroExtend(imm16) << (hw*16).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:220 movz.
- Callers: emit.rs:911-922 movz Rd, #imm16 [, lsl #N] as the start of MOVZ+MOVK sequences.
- `:abs_g*:` modifiers are documented for movz/movk (data_processing.rs:179-181).

## Quirks

- Extra operands beyond the optional lsl are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Immediate is masked with `(imm as u32) & 0xFFFF` with no range check (see bugs).
- Non-lsl shift kinds default to hw=0; lsl amount is integer-divided by 16 with no range check (see bugs).
- Unresolved abs_g symbols (non-constant) fall through to get_imm and Err; RelocType has no MOVW variants.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg / Modifier abs_g / get_imm / Shift lsl vs other / extra non-Shift / too few / FP / invalid name).

---

# Confirmed invariants (encode_movn)

- Valid GPR + imm16 + optional lsl (hw in {0,1} for W, {0,1,2,3} for X; Rd=31 is xzr/wzr) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_movn(X-ops) XOR encode_movn(W-ops) at equal rd/imm/hw in {0,1} = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:23]=00100101 (opc=00), hw at [22:21], imm16 at [20:5], Rd at [4:0].
- `lr` as Rd encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-imm16 second operands (unknown modifier, non-constant abs_g symbol, Symbol/Label/Mem/Reg) always Err.
- Known-answer: `movn x0, #42` encodes as 0x92800540; `movn w0, #42` as 0x12800540; `movn x0, #42, lsl #16` as 0x92a00540.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Move wide (immediate) MOVN: `sf 00 100101 hw imm16 Rd`. Register 31 is XZR/WZR, never SP/WSP. imm16 in [0, 65535]. hw in {0,1} when sf=0; {0,1,2,3} when sf=1. Semantics: Rd := NOT(ZeroExtend(imm16) << (hw*16)).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:222 movn.
- Callers: emit.rs:873-902 movn Rd, #imm16 [, lsl #N] as the start of MOVN+MOVK sequences.
- `:abs_g*:` modifiers are documented for movz/movk only (data_processing.rs:179-181); encode_movn has no Modifier path.

## Quirks

- Extra operands beyond the optional lsl are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Immediate is masked with `(imm as u32) & 0xFFFF` with no range check (see bugs).
- Non-lsl shift kinds default to hw=0; lsl amount is integer-divided by 16 with no range check (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg / get_imm / Shift lsl vs other / extra non-Shift / too few / FP / invalid name).

---

# Confirmed invariants (encode_movk)

- Valid GPR + imm16 + optional lsl (hw in {0,1} for W, {0,1,2,3} for X; Rd=31 is xzr/wzr) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_movk(X-ops) XOR encode_movk(W-ops) at equal rd/imm/hw in {0,1} = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:23]=11100101, hw at [22:21], imm16 at [20:5], Rd at [4:0].
- Constant `:abs_g0:`/`:abs_g1:`/`:abs_g2:`/`:abs_g3:` (and `_nc`) encode the extracted 16-bit chunk and match llvm-mc of the resolved `movk Rd, #chunk [, lsl #shift]` (1000 cases).
- `lr` as Rd encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 2 operands, invalid names (foo, x32, w32, x, r0, empty), and non-imm16 second operands (unknown modifier, non-constant abs_g symbol, Symbol/Label/Mem/Reg) always Err.
- Known-answer: `movk x0, #42` encodes as 0xf2800540; `movk w0, #42` as 0x72800540; `movk x0, #42, lsl #16` as 0xf2a00540.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Move wide (immediate) MOVK: `sf 11 100101 hw imm16 Rd`. Register 31 is XZR/WZR, never SP/WSP. imm16 in [0, 65535]. hw in {0,1} when sf=0; {0,1,2,3} when sf=1.
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:221 movk.
- Callers: emit.rs:906-928 movk Rd, #imm16 [, lsl #N]; intrinsics.rs:182-184.

## Quirks

- Extra operands beyond the optional lsl are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Immediate is masked with `(imm as u32) & 0xFFFF` with no range check (see bugs).
- Non-lsl shift kinds default to hw=0; lsl amount is integer-divided by 16 with no range check (see bugs).
- Unresolved abs_g symbols (non-constant) fall through to get_imm and Err; RelocType has no MOVW variants.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg / Modifier abs_g / get_imm / Shift lsl vs other / extra non-Shift / too few).

---

# Confirmed invariants (encode_madd)

- Valid four-GPR same-width MADD with Rd/Rn/Rm/Ra in x0–x30/xzr or w0–w30/wzr matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_madd(Rd, Rn, Rm, ZR) equals llvm-mc `mul Rd, Rn, Rm` and llvm-mc `madd Rd, Rn, Rm, ZR` (1000 cases). Documented alias at data_processing.rs:589.
- encode_madd(X-ops) XOR encode_madd(W-ops) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path word: sf at 31, bits [30:21]=0011011000, Rm at [20:16], o0=0 at 15, Ra at [14:10], Rn at [9:5], Rd at [4:0].
- `lr` in any of the four slots encodes as X30 and matches llvm-mc (1000 cases).
- Fewer than 4 operands, non-register operands (Imm/Symbol/Mem/Shift/Cond/Label), and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `madd x0, x1, x2, x3` encodes as 0x9b020c20; `madd w0, w1, w2, w3` as 0x1b020c20; `madd x0, x1, x2, xzr` / `mul x0, x1, x2` as 0x9b027c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Data-processing (3 source) MADD: `sf 00 11011 000 Rm 0 Ra Rn Rd`. Register 31 is XZR/WZR, never SP/WSP. All four registers same width. Exactly four operands. o0 (bit 15) is 0 (MSUB is 1).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:245 madd.
- Callers: i128_ops.rs:79-80 emit `madd x1, x3, x4, x1` / `madd x1, x2, x5, x1`.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg 0..3 / sf / parse_reg_num lr / invalid / non-Reg).

---

# Confirmed invariants (encode_logical)

- Valid AND/ORR/EOR/ANDS shifted-register with Rd/Rn/Rm in x0–x30/xzr or w0–w30/wzr, shift in {lsl,lsr,asr,ror} with amount in [0,31] (W) or [0,63] (X), matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid AND/ORR/EOR/ANDS bitmask-immediate constructed from ARM ARM (size, ones, immr), Rd=31 as SP (AND/ORR/EOR) or XZR (ANDS), Rn=31 as XZR, matches llvm-mc (1000 cases).
- Valid NEON AND/ORR/EOR with T in {8b,16b}, Vd/Vn/Vm in v0–v31, matches llvm-mc (1000 cases).
- encode(opc_a) XOR encode(opc_b) = (opc_a XOR opc_b)<<29 at equal other fields (ARM ARM opc at bits [30:29]) (1000 cases).
- encode(X) XOR encode(W) at equal register numbers = 1<<31 (ARM ARM sf) (1000 cases).
- Success-path shifted-register word: sf at 31, opc at [30:29], bits [28:24]=01010, shift at [23:22], N=0 at 21, Rm at [20:16], imm6 at [15:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, invalid bitmask (0 / all-ones / 0x1234 / 0x5 / 0x1001), third operand that is Symbol/Mem/Label/Cond, and invalid names (foo, x32, w32, x, r0, empty) always Err.
- Known-answer: `and x0, x1, x2` encodes as 0x8a020020; `orr x0, x1, x2` as 0xaa020020; `eor x0, x1, x2` as 0xca020020; `ands x0, x1, x2` as 0xea020020; `and w0, w1, w2` as 0x0a020020; `and x0, x1, #1` as 0x92400020; `and sp, x0, #1` as 0x9240001f; `and v0.16b, v1.16b, v2.16b` as 0x4e221c20; `and v0.8b, v1.8b, v2.8b` as 0x0e221c20; `orr v0.16b, v1.16b, v2.16b` as 0x4ea21c20; `eor v0.16b, v1.16b, v2.16b` as 0x6e221c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Logical (shifted register): `sf opc 01010 shift N Rm imm6 Rn Rd` with N=0. opc 00 AND / 01 ORR / 10 EOR / 11 ANDS.
- ARM ARM Logical (immediate): `sf opc 100100 N immr imms Rn Rd`. Rd=31 is SP for AND/ORR/EOR and XZR for ANDS (TST). llvm-mc rejects SP as Rn.
- ARM ARM Advanced SIMD logical: `0 Q U 01110 size 1 Rm 000111 Rn Rd`. AND U=0 size=00; ORR U=0 size=10; EOR U=1 size=00. T in {8B,16B} only.
- Dispatch: encoder/mod.rs:231-234 and/orr/eor/ands.
- `lr` is a 64-bit alias of X30.

## Quirks

- Extra operands beyond a shift at index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so shifted-register SP encodes as ZR (see bugs).
- Mixed X/W is accepted; sf is taken only from Rd (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- Shift amount is masked with 0x3F with no range check (see bugs).
- Unknown shift kinds default to LSL (see bugs).
- NEON T other than 16b is encoded with Q=0; source arrangements discarded (see bugs).
- ANDS (opc=11) on NEON encodes as EOR-like (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / NEON / Imm / Reg / unsupported-third / invalid-reg / sf).

---

# Confirmed invariants (encode_ldxr_stxr)

- Valid LDXR/STXR/LDXRB/STXRB/LDXRH/STXRH with Rt in x0–x30/xzr or w0–w30/wzr (byte/half always W), Rn in x0–x30/sp, Ws in w0–w30/wzr not aliasing Rt/Xn, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(load) XOR encode(store with Ws=31) at equal Rt/Rn = 1<<22 (ARM ARM L bit) (1000 cases).
- encode(X) XOR encode(W) at equal register numbers = 1<<30 (ARM ARM size 11 vs 10) (1000 cases).
- encode(byte) XOR encode(half) at equal W registers = 1<<30 (ARM ARM size 00 vs 01) (1000 cases).
- Success-path word: bits [29:24]=001000, bit 23=0, bit 21=0 (not pair), o0=0 at bit 15, Rt2=11111 at [14:10], size at [31:30], L at 22, Rs=31 on load else Ws at [20:16], Rn at [9:5], Rt at [4:0].
- Fewer than 2 (load) / 3 (store) operands, non-Reg first operand, non-Mem memory slot (Imm/Symbol/pre/post-index), and invalid base names (foo, x32) always Err.
- Known-answer: `ldxr x0, [x1]` encodes as 0xc85f7c20; `ldxr w0, [x1]` as 0x885f7c20; `ldxrb w0, [x1]` as 0x085f7c20; `ldxrh w0, [x1]` as 0x485f7c20; `stxr w0, x1, [x2]` as 0xc8007c41; `stxr w0, w1, [x2]` as 0x88007c41; `ldxr x0, [sp]` as 0xc85f7fe0; `ldxr lr, [x2]` as 0xc85f7c5e.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Load/Store Exclusive (single): `size 001000 0 L 0 Rs o0 Rt2 Rn Rt`. size=00 byte / 01 half / 10 word / 11 doubleword. Offset absent or #0. o0=0 distinguishes LDXR/STXR from LDAXR/STLXR.
- Rt register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldxr sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- STXR Ws is Wt (31=WZR); llvm-mc rejects Xt/SP as status and rejects Ws aliasing Rt/Xn ("status is also a source"). WZR vs SP is allowed.
- Byte/half data is Wt (llvm-mc rejects `ldxrb x0, [x1]`).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:348-353 ldxr/stxr/ldxrb/stxrb/ldxrh/stxrh.

## Quirks

- Extra operands beyond the memory slot are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `ldxr sp, ...` encodes as ZR (see bugs).
- W-register base is accepted and encoded as the same-number X register (see bugs).
- XZR as base encodes as SP (register 31) (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- X as STXR status is accepted; only the number is used (see bugs).
- forced_size byte/half with an X data register is encoded (see bugs).
- `Mem { base, .. }` ignores a nonzero offset (see bugs).
- STXR Ws overlapping Rt/Rn is encoded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (is_load / get_reg miss / non-Mem / parse_reg_num None / forced_size / is_64).

---

# Confirmed invariants (encode_neon_float_three_same)

- Valid vector FP three-same (FADD/FSUB/FMUL/FDIV/FMAX/FMIN/FMAXNM/FMINNM/FMLA/FMLS/FRECPS/FRSQRTS/FCMEQ/FCMGE/FCMGT/FACGE/FACGT/FABD) with T in {2s,4s,2d}, Vd/Vn/Vm in v0–v31, and ARM-correct (U, size_hi, opcode) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., U=0) XOR encode(..., U=1) = 1<<29 (ARM ARM U bit) (1000 cases).
- encode(..., size_hi=0) XOR encode(..., size_hi=1) = 1<<23 (ARM ARM size[1]) (1000 cases).
- encode(2s) XOR encode(4s) at equal register numbers = 1<<30 (ARM ARM Q bit) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T (2s→0, 4s/2d→1), U at 29, bits [28:24]=01110, size at [23:22]=(size_hi<<1)|sz, bit 21=1, Rm at [20:16], opcode at [15:11], bit 10=1, Rn at [9:5], Rd at [4:0].
- Arrangement other than 2s/4s/2d, fewer than 3 operands, non-register dest/src/Vm, invalid names (v32, foo, empty, v, v-1, v99), and dest Operand::Reg (no arrangement) always Err.
- Known-answer: `fadd v0.4s, v1.4s, v2.4s` encodes as 0x4e22d420; `fadd v0.2s, v1.2s, v2.2s` as 0x0e22d420; `fadd v0.2d, v1.2d, v2.2d` as 0x4e62d420; `fsub v0.4s, v1.4s, v2.4s` as 0x4ea2d420; `fmul v0.4s, v1.4s, v2.4s` as 0x6e22dc20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD three-same FP: `0 Q U 01110 size 1 Rm opcode 1 Rn Rd`. size[1]=size_hi, size[0]=sz (0=single, 1=double).
- Valid T is 2S, 4S, 2D (llvm-mc rejects 8b/16b/4h/8h/1d without +fullfp16).
- Scalar `fadd s0, s1, s2` / `fadd d0, d1, d2` is a different encoding (scalar FP) — not this vector helper.
- Exactly three operands (llvm-mc rejects a fourth).
- Dispatch: encoder/mod.rs:377-496 fadd/fsub/fmul/fdiv/fmax/fmin/fmaxnm/fminnm/fmla/fmls/frecps/frsqrts/fcmeq/fcmge/fcmgt/facge/facgt.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangements are discarded; dest T is used (see bugs). Operand::Reg source (empty arrangement) is accepted (see bugs).
- parse_reg_num accepts x/w/d/s/q/v/h/b prefixes, so non-V names encode as V registers (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_neon_reg dest/Vn/Vm, match 2s/4s/2d/_, arity).

---

# Confirmed invariants (encode_ldxp_stxp)

- Valid LDXP/LDAXP/STXP/STLXP with Rt/Rt2 in x0–x30/xzr or w0–w30/wzr, Rn in x0–x30/sp, Ws in w0–w30/wzr not aliasing Rt/Rt2/Xn, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_ldxp_stxp(..., acqrel=true) XOR encode_ldxp_stxp(..., acqrel=false) = 1<<15 (ARM ARM o0 bit) (1000 cases).
- encode_ldxp_stxp(X-ops) XOR encode_ldxp_stxp(W-ops) = 1<<30 at equal register numbers (ARM ARM sz) (1000 cases).
- Success-path word: bit 31=1, size 11/10 at [31:30], bits [29:24]=001000, bit 23=0, L at 22, o1=1 at 21, Rs=31 on load else Ws at [20:16], o0 at 15, Rt2 at [14:10], Rn at [9:5], Rt at [4:0].
- Fewer than 3 (load) / 4 (store) operands, non-Reg first operand, non-Mem memory slot (Imm/Symbol/pre/post-index), and invalid base names (foo, x32) always Err.
- Known-answer: `ldxp x0, x1, [x2]` encodes as 0xc87f0440; `ldxp w0, w1, [x2]` as 0x887f0440; `ldaxp x0, x1, [x2]` as 0xc87f8440; `stxp w0, x1, x2, [x3]` as 0xc8200861; `stlxp w0, x1, x2, [x3]` as 0xc8208861; `ldxp x0, x1, [sp]` as 0xc87f07e0; `ldxp lr, x1, [x2]` as 0xc87f045e.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Load/Store Exclusive Pair: `size 001000 0 L 1 Rs o0 Rt2 Rn Rt`. size=10 (W pair) / 11 (X pair). Offset absent or #0.
- Rt/Rt2 register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldxp sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- STXP Ws is Wt (31=WZR); llvm-mc rejects Xt/SP as status and rejects Ws aliasing Rt/Rt2/Xn ("status is also a source"). WZR vs SP is allowed.
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg).
- Dispatch: encoder/mod.rs:366-369 ldxp/ldaxp/stxp/stlxp.

## Quirks

- Extra operands beyond the memory slot are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `stxp w0, sp, ...` encodes as ZR (see bugs).
- W-register base is accepted and encoded as the same-number X register (see bugs).
- XZR as base encodes as SP (register 31) (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- sz is taken only from the first data register; mixed X/W encodes (see bugs).
- X as STXP status is accepted; only the number is used (see bugs).
- `Mem { base, .. }` ignores a nonzero offset (see bugs).
- STXP Ws overlapping Rt/Rt2/Rn is encoded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (too-few / non-Reg / non-Mem / parse_reg_num None).

---

# Confirmed invariants (encode_ldur_stur)

- Valid GPR LDUR/STUR/LDTR/STTR with Rt in x0–x30/xzr or w0–w30/wzr, Rn in x0–x30/sp, offset in [-256, 255] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid SIMD LDUR/STUR with Rt in b/h/s/d/q 0–31, Rn in x0–x30/sp, offset in [-256, 255] matches llvm-mc (1000 cases).
- encode_ldur_stur(..., is_load=true) XOR encode_ldur_stur(..., is_load=false) = 1<<22 (ARM ARM opc bit) (1000 cases).
- encode_ldur_stur(..., op2=00) XOR encode_ldur_stur(..., op2=10) = 1<<11 over GPR (ARM ARM unscaled vs unprivileged) (1000 cases).
- Success-path word: bits [29:27]=111, bits [25:24]=00, bit 21=0, imm9 at [20:12], op2 at [11:10], Rn at [9:5], Rt at [4:0]; GPR size 11/10 from X/W; SIMD size/opc from B/H/S/D/Q.
- Fewer than 2 operands, non-Reg first operand, non-Mem second operand (Imm/Symbol/pre/post-index), and invalid base names (foo, x32) always Err.
- Known-answer: `ldur x0, [x1]` encodes as 0xf8400020; `ldur q0, [x1]` as 0x3cc00020; `ldtr x0, [x1]` as 0xf8400820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM unscaled LDUR/STUR: `size 111 V 00 opc 0 imm9 00 Rn Rt`. LDTR/STTR: bits [11:10]=10. simm9 in [-256, 255].
- Rt register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldur sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- SIMD Rt is valid for LDUR/STUR only (llvm-mc rejects `ldtr d0, ...` and `ldur v0, ...`).
- `lr` is a 64-bit alias of X30 (llvm-mc and is_64bit_reg / encode_ldr_str_auto).
- Dispatch: encoder/mod.rs:336-339 ldur/stur/ldtr/sttr.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- imm9 is masked with 0x1FF with no range check (see bugs).
- parse_reg_num maps sp/wsp to 31, so `stur sp, [x0]` encodes as `stur wzr, [x0]` (see bugs).
- W-register base is accepted and encoded as the same-number X register (see bugs).
- XZR as base encodes as SP (register 31) (see bugs).
- SIMD Rt on LDTR/STTR is encoded (see bugs).
- V-register Rt falls through to size=11 opc=01 (D form) (see bugs).
- `lr` is sized as 32-bit because size uses `starts_with('x')` (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / non-Mem / parse_reg_num None / lr alias).

---

# Confirmed invariants (encode_neon_sli)

- Valid vector SLI with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [0, esize(T)-1] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_neon_sli(Tlo) XOR encode_neon_sli(Thi) = 1<<30 for (8b,16b)/(4h,8h)/(2s,4s) at equal shift (ARM ARM Q bit) (1000 cases).
- encode_neon_sli(shift+1) − encode_neon_sli(shift) = 1<<16 when both shifts are in range (ARM ARM immh:immb = esize + shift) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T, U=1 at 29, bits [28:23]=011110, immh:immb at [22:16]=esize+shift, bits [15:10]=010101, Rn at [9:5], Rd at [4:0].
- Arrangement other than 8b/16b/4h/8h/2s/4s/2d (including 1d), fewer than 3 operands, non-register dest/src (Imm/Mem/Symbol/Shift/Cond/Label), invalid names (v32, foo, empty, v, v-1, v99), dest Operand::Reg (no arrangement), and non-Imm shift always Err.
- Known-answer: `sli v0.8b, v1.8b, #0` encodes as 0x2f085420; `sli v0.8b, v1.8b, #7` as 0x2f0f5420; `sli v0.16b, v1.16b, #3` as 0x6f0b5420; `sli v0.4h, v1.4h, #15` as 0x2f1f5420; `sli v0.2d, v1.2d, #0` as 0x6f405420; `sli v0.2d, v1.2d, #63` as 0x6f7f5420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SLI: `0 Q 1 011110 immh:immb 010101 Rn Rd`. Valid T: 8B/16B (shift 0..7), 4H/8H (0..15), 2S/4S (0..31), 2D (0..63). 1D reserved. Scalar `sli d0, d1, #0` is a different encoding (bits[31:30]=01) — not this vector helper.
- Exactly three operands (llvm-mc rejects a fourth).
- Dispatch: encoder/mod.rs:661 `"sli" => encode_neon_sli(operands)`.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded; dest T is used (see bugs). Operand::Reg source (empty arrangement) is accepted (see bugs).
- parse_reg_num accepts x/w/d/s/q/v/h/b prefixes, so non-V names encode as V registers (see bugs).
- Shift is `get_imm as u32` then `(esize + shift) & mask`: negative panics in debug / wraps in release; shift >= esize encodes reserved immh=0000 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_neon_reg Operand::Reg dest / src).

---

# Confirmed invariants (encode_neon_float_cmp_zero)

- Valid vector FCMEQ/FCMGE/FCMGT/FCMLE/FCMLT-to-zero with T in {2s,4s,2d}, Vd/Vn in v0–v31, and ARM-correct (U, size_hi=1, opcode) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_neon_float_cmp_zero(ops, 0, size_hi, opcode) XOR encode_neon_float_cmp_zero(ops, 1, size_hi, opcode) = 1<<29 (ARM ARM U bit) (1000 cases).
- encode_neon_float_cmp_zero(ops, U, 0, opcode) XOR encode_neon_float_cmp_zero(ops, U, 1, opcode) = 1<<23 (size_hi = size[1]) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T (2s→0, 4s/2d→1), U at 29, bits [28:24]=01110, size at [23:22]=(size_hi<<1)|sz, bits [21:17]=10000, opcode at [16:12], bits [11:10]=10, Rn at [9:5], Rd at [4:0].
- Arrangement other than 2s/4s/2d, fewer than 2 operands, non-register dest/src, invalid names (v32, foo, empty, v, v-1, v99), and dest Operand::Reg (no arrangement) always Err.
- Known-answer: `fcmeq v0.4s, v1.4s, #0.0` encodes as 0x4ea0d820; `fcmge v0.4s, v1.4s, #0.0` as 0x6ea0c820; `fcmlt v0.4s, v1.4s, #0.0` as 0x4ea0e820; `fcmeq v0.2s, v1.2s, #0.0` as 0x0ea0d820; `fcmeq v0.2d, v1.2d, #0.0` as 0x4ee0d820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM-correct (U, size_hi, opcode): FCMEQ (0,1,01101), FCMGE (1,1,01100), FCMGT (0,1,01100), FCMLE (1,1,01101), FCMLT (0,1,01110). size = 1sz (not 0sz).
- Valid T is 2S, 4S, 2D (llvm-mc rejects 8b/16b/4h/8h/1d without +fullfp16; 4h/8h is a different FP16 encoding).
- Scalar `fcmeq s0, s1, #0.0` is a different encoding (bits[31:30]=01) — not this vector helper.
- Dispatch passes [Vd, Vn, Imm(0)] for fcmeq/fcmge/fcmgt #0.0; the helper reads only [0] and [1].

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- Source arrangement is discarded; dest T is used (see bugs).
- parse_reg_num accepts x/w/d/s/q/v/h/b prefixes, so non-V names encode as V registers (see bugs).
- Function comment says size=0sz; ARM ARM and llvm-mc use size=1sz. The helper packs the caller-supplied size_hi; dispatcher currently passes size_hi=0 for fcmeq/fcmge/fcmle (out of this symbol's scope).
- Dispatcher passes opcode=01101 for fcmlt; ARM/llvm-mc use 01110 (out of this symbol's scope).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_neon_reg Operand::Reg dest / non-V prefix).

---

# Confirmed invariants (encode_neon_across_long)

- Valid SADDLV/UADDLV with dest V matching T (H for 8B/16B, S for 4H/8H, D for 4S) and Vn in v0–v31 matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_neon_across_long(ops, 0, 0b00011) XOR encode_neon_across_long(ops, 1, 0b00011) = 1<<29 (ARM ARM U bit) (1000 cases).
- Success-path word: bit 31=0, Q at 30 from T, U at 29, bits [28:24]=01110, size at [23:22] from T, bits [21:17]=11000, bits [16:12]=00011, bits [11:10]=10, Rn at [9:5], Rd at [4:0].
- Fewer than 2 operands, non-register dest/src (Imm/Mem/Symbol/Shift/Cond/Label), and invalid names (v32, h32, foo, empty, v, v-1) always Err — including dest RegArrangement with an invalid name.
- Known-answer: `saddlv h0, v1.8b` encodes as 0x0e303820; `uaddlv h0, v0.8b` as 0x2e303800; `saddlv d0, v1.4s` as 0x4eb03820.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- SADDLV/UADDLV dest V is H/S/D matching T (llvm-mc rejects b/q/x/w/v dest and vector-arrangement dest).
- Valid T is 8B, 16B, 4H, 8H, 4S (llvm-mc rejects 2S/1D/2D and unknown qualifiers).
- Exactly two operands (llvm-mc rejects a third operand).
- Codegen emits `uaddlv h0, v0.8b` (alu.rs:65).

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- neon_arr_to_q_size accepts 2s/1d/2d, so reserved T is encoded (see bugs).
- Dest prefix is ignored; only parse_reg_num is used, so b/s/d/q/x/w/v dest encode as the matching-number H/S/D form (see bugs).
- Operand::RegArrangement dest is accepted and its arrangement discarded (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (dest `_` non-Reg / parse_reg_num None on dest RegArrangement).

---

# Confirmed invariants (encode_ldar_stlr)

- Valid LDAR/STLR/LDARB/STLRB/LDARH/STLRH with Wt/Xt Rt (31=XZR/WZR) and Xn|SP base, offset 0, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_ldar_stlr(ops, true, sz) XOR encode_ldar_stlr(ops, false, sz) = 1<<22 (ARM ARM L bit) (1000 cases).
- Success-path word: size at [31:30], bits [29:24]=001000, bit 23=1, L at 22, bit 21=0, Rs=31 at [20:16], o0=1 at 15, Rt2=31 at [14:10], Rn at [9:5], Rt at [4:0].
- Fewer than 2 operands, non-Reg first operand, non-Mem second operand (Imm/Symbol/pre/post/reg-offset), and invalid base names (foo, x32) always Err.
- Known-answer: `ldar x0, [x1]` encodes as 0xc8dffc20; `stlr x0, [x1]` as 0xc89ffc20; `ldar w0, [x1]` as 0x88dffc20; `ldarb w0, [x1]` as 0x08dffc20; `ldarh w0, [x1]` as 0x48dffc20; `ldar xzr, [sp]` as 0xc8dfffff.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- LDAR/STLR Rt register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ldar sp, ...`).
- Rn is Xn|SP (llvm-mc rejects [wN], [xzr], [wzr], [wsp]).
- Offset must be absent or #0 (llvm-mc: "index must be absent or #0").
- Byte/halfword forms take Wt only (llvm-mc rejects `ldarb x0, [x1]`).
- LDAR/STLR take GPR only (llvm-mc rejects `ldar d0, ...`).
- Mixed W data + X base is valid (`ldar w0, [x1]`).

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `stlr sp, [x0]` encodes as `stlr xzr, [x0]` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- W-register base is accepted and encoded as the same number X register (see bugs).
- XZR as base encodes as SP (register 31) (same property as W-base).
- `Mem { base, .. }` ignores a nonzero offset (same property as W-base).
- Xt for ldarb/ldarh/stlrb/stlrh is accepted (size forced; Rt number still encoded).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg non-Reg / parse_reg_num None).

---

# Confirmed invariants (encode_eon)

- Same-width GPR EON (x0–x30/xzr/lr and w0–w30/wzr, optional lsl/lsr/asr/ror with amount in [0,31] W / [0,63] X) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_eon(ops) XOR encode_logical(ops, 0b10) = 1<<21 (ARM ARM EON N=1 vs EOR N=0) (1000 cases).
- Success-path word: sf at 31 from Rd width, opc=10 at [30:29], bits [28:24]=01010, shift at [23:22], N=1 at 21, Rm at [20:16], imm6 at [15:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x99) always Err.
- Known-answer: `eon x0, x1, x2` encodes as 0xca220020; `eon w0, w1, w2` as 0x4a220020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- EON register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `eon sp, ...`).
- EON takes Wt/Xt only (llvm-mc rejects `eon d0, ...`).
- llvm-mc rejects mixed x/w, a 4th non-shift operand, shift amount 32 (W) / 64 (X), and unknown shift kinds.
- llvm-mc accepts `eon Rd, Rn, #imm` as the assembler alias of `eor Rd, Rn, #~imm` (Rd may not be ZR).
- llvm-mc omits `lsl #0` in disassembly of unshifted EON.

## Quirks

- Extra operands beyond a non-Shift index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `eon wsp, ...` encodes as `eon wzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- Shift amount is masked with 0x3F with no width check (see bugs).
- Unknown shift kinds default to LSL via `_ => 0b00` (see bugs).
- No immediate path: get_reg on operand 2 rejects `eon Rd, Rn, #imm` (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (unknown shift `_ => 0b00` / parse_reg_num None).

---

# Confirmed invariants (encode_div)

- Same-width GPR UDIV/SDIV (x0–x30/xzr and w0–w30/wzr, including register 31 as XZR/WZR) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_div(ops, true) XOR encode_div(ops, false) = 1<<10 (ARM ARM UDIV o1=0 vs SDIV o1=1) (1000 cases).
- Success-path word: sf at 31 from Rd width, bit 30=0, S=0 at 29, bits [28:21]=0b11010110, Rm at [20:16], bits [15:11]=00001, o1 at 10, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Non-register operands (Imm/Mem/Symbol/Shift/Cond) in any of the three slots always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Known-answer: `udiv x0, x1, x2` encodes as 0x9ac20820; `sdiv w0, w1, w2` as 0x1ac20c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- UDIV/SDIV register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `udiv sp, ...`).
- UDIV/SDIV take Wt/Xt only (llvm-mc rejects `udiv d0, ...`).
- llvm-mc rejects mixed x/w and a fourth operand.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `sdiv wsp, ...` encodes as `sdiv wzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (get_reg None / parse_reg_num None / FP prefixes).

---

# Confirmed invariants (encode_csneg)

- Same-width GPR CSNEG (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csneg(ops) XOR encode_csinc(ops) = 1<<30 (ARM ARM CSNEG op=1 vs CSINC op=0) (1000 cases).
- encode_csneg([Rd, Rn, Rn, invert(cond)]) equals encode_cneg([Rd, Rn, cond]) over Cond14 (ARM ARM CNEG alias of CSNEG) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csneg x0, x1, x2, eq` encodes as 0xda820420; `csneg w0, w1, w2, ne` as 0x5a821420; `csneg x0, x1, x2, al` as 0xda82e420; `csneg x0, x1, x1, ne` as 0xda811420 (CNEG alias).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSNEG register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csneg sp, ...`).
- CSNEG takes Wt/Xt only (llvm-mc rejects `csneg d0, ...`).
- Cond AL and NV are valid for architectural CSNEG (unlike CNEG alias).
- llvm-mc rejects mixed x/w and a fifth operand.
- llvm-mc disassembles `csneg x0, x1, x1, ne` as `cneg x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csneg sp, ...` encodes as `csneg xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csinv)

- Same-width GPR CSINV (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csinv(ops) XOR encode_csel(ops) = 1<<30 (ARM ARM CSINV op=1 vs CSEL op=0) (1000 cases).
- encode_csinv([Rd, Rn, Rn, invert(cond)]) equals encode_cinv([Rd, Rn, cond]) over Cond14 (ARM ARM CINV alias of CSINV) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=00 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csinv x0, x1, x2, eq` encodes as 0xda820020; `csinv w0, w1, w2, ne` as 0x5a821020; `csinv x0, x1, x2, al` as 0xda82e020; `csinv x0, x1, x1, ne` as 0xda811020 (CINV alias).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSINV register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csinv sp, ...`).
- CSINV takes Wt/Xt only (llvm-mc rejects `csinv d0, ...`).
- Cond AL and NV are valid for architectural CSINV (unlike CINV/CSETM aliases).
- llvm-mc rejects mixed x/w and a fifth operand.
- llvm-mc disassembles `csinv x0, x1, x1, ne` as `cinv x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csinv sp, ...` encodes as `csinv xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csinc)

- Same-width GPR CSINC (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csinc(ops) XOR encode_csel(ops) = 1<<10 (ARM ARM CSINC op2=01 vs CSEL op2=00) (1000 cases).
- encode_csinc([Rd, Rn, Rn, invert(cond)]) equals encode_cinc([Rd, Rn, cond]) over Cond14 (ARM ARM CINC alias of CSINC) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csinc x0, x1, x2, eq` encodes as 0x9a820420; `csinc w0, w1, w2, ne` as 0x1a821420; `csinc x0, x1, x2, al` as 0x9a82e420; `csinc x0, x1, x1, ne` as 0x9a811420 (CINC alias).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSINC register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csinc sp, ...`).
- CSINC takes Wt/Xt only (llvm-mc rejects `csinc d0, ...`).
- Cond AL and NV are valid for architectural CSINC (unlike CINC/CSET aliases).
- llvm-mc rejects mixed x/w and a fifth operand.
- llvm-mc disassembles `csinc x0, x1, x1, ne` as `cinc x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csinc sp, ...` encodes as `csinc xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csetm)

- Same-width GPR CSETM (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases, excluding al/nv) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csetm([Rd, cond]) equals encode_csinv([Rd, ZR, ZR, invert(cond)]) (ARM ARM CSETM alias of CSINV) (1000 cases).
- encode_csetm([Rd, cond]) equals encode_cinv([Rd, ZR, cond]) (CINV with Rn=ZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=31 at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=00 at [11:10], Rn=31 at [9:5], Rd at [4:0].
- Fewer than 2 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in either slot always Err.
- Known-answer: `csetm x0, eq` encodes as 0xda9f13e0; `csetm w0, ne` as 0x5a9f03e0; `csinv x0, xzr, xzr, ne` disassembles as the same CSETM.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSETM register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csetm sp, ...`).
- CSETM takes Wt/Xt only (llvm-mc rejects `csetm d0, ...`).
- Cond AL and NV are invalid for the CSETM alias (unlike architectural CSEL).
- llvm-mc rejects a third operand.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- encode_cond accepts al/nv and invert(cond)=cond XOR 1 is applied with no AL/NV guard (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csetm sp, eq` encodes as `csetm xzr, eq` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cset)

- Same-width GPR CSET (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases, excluding al/nv) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cset([Rd, cond]) equals encode_csinc([Rd, ZR, ZR, invert(cond)]) (ARM ARM CSET alias of CSINC) (1000 cases).
- encode_cset([Rd, cond]) equals encode_cinc([Rd, ZR, cond]) (CINC with Rn=ZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=31 at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=01 at [11:10], Rn=31 at [9:5], Rd at [4:0].
- Fewer than 2 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in either slot always Err.
- Known-answer: `cset x0, eq` encodes as 0x9a9f17e0; `cset w0, ne` as 0x1a9f07e0; `csinc x0, xzr, xzr, ne` disassembles as the same CSET.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSET register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cset sp, ...`).
- CSET takes Wt/Xt only (llvm-mc rejects `cset d0, ...`).
- Cond AL and NV are invalid for the CSET alias (unlike architectural CSEL).
- llvm-mc rejects a third operand.

## Quirks

- Extra operands beyond index 1 are ignored (see bugs).
- encode_cond accepts al/nv and invert(cond)=cond XOR 1 is applied with no AL/NV guard (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cset sp, eq` encodes as `cset xzr, eq` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_csel)

- Same-width GPR CSEL (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond16 including al/nv and hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_csel(ops) XOR encode_csinc(ops) = 1<<10 (ARM ARM CSEL op2=00 vs CSINC op2=01) (1000 cases).
- encode_csel(ops) XOR encode_csinv(ops) = 1<<30 (ARM ARM CSEL op=0 vs CSINV op=1) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm at [20:16], cond at [15:12], op2=00 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 4 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the four slots always Err.
- Known-answer: `csel x0, x1, x2, eq` encodes as 0x9a820020; `csel w0, w1, w2, ne` as 0x1a821020; `csel x0, x1, x2, al` as 0x9a82e020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CSEL register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `csel sp, ...`).
- CSEL takes Wt/Xt only (llvm-mc rejects `csel d0, ...`).
- Cond AL and NV are valid for architectural CSEL (unlike CINC/CINV/CNEG aliases).
- llvm-mc rejects mixed x/w and a fifth operand.

## Quirks

- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `csel sp, ...` encodes as `csel xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn/Rm widths are never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None / parse_reg_num None / get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cneg)

- Same-width GPR CNEG (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cneg([Rd, Rn, cond]) equals encode_csneg([Rd, Rn, Rn, invert(cond)]) (ARM ARM CNEG alias of CSNEG) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=Rn at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the three slots always Err.
- Known-answer: `cneg x0, x1, eq` encodes as 0xda811420; `cneg w0, w1, ne` as 0x5a810420; `cneg x0, x1, eq` equals `csneg x0, x1, x1, ne`.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CNEG register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cneg sp, ...`).
- CNEG takes Wt/Xt only (llvm-mc rejects `cneg d0, ...`).
- Cond AL and NV are invalid for the CNEG alias (llvm-mc: "condition codes AL and NV are invalid for this instruction").
- llvm-mc rejects mixed x/w and a fourth operand.
- llvm-mc disassembles `csneg x0, x1, x1, ne` as `cneg x0, x1, eq` with the same encoding.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- encode_cond accepts al/nv, so CNEG AL/NV encodes as CSNEG with inverted cond (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cneg sp, ...` encodes as `cneg xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None, get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cmp)

- Same-width GPR CMP immediate form (x0–x30/sp/lr and w0–w30/wsp, imm in unshifted 0..4095 or N<<12 with N in 1..4095, plus explicit lsl #12) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-width GPR CMP shifted-register form (x0–x30/xzr/lr and w0–w30/wzr, lsl/lsr/asr in range) matches llvm-mc (1000 cases).
- Extended-register CMP (sxtw/uxtw/sxtx/uxtx, amount 0..4 including bounds) matches llvm-mc (1000 cases).
- Negative immediate `cmp Rn, #-N` for N in 1..4095 matches llvm-mc's gas rewrite to `cmn Rn, #N` (1000 cases).
- encode_cmp(ops) equals encode_add_sub([ZR] ++ ops, is_sub=true, set_flags=true) where ZR is WZR if Rn is 32-bit else XZR (1000 cases).
- Success-path immediate word: Rd=31, S=1, op=1, bits[28:24]=0b10001, sf from Rn width, unshifted imm12, Rn at [9:5].
- 0 or 1 operands always Err.
- Non-register first operand (Imm/Symbol/Mem/Cond/Shift) with a second operand always Err.
- Known-answer: `cmp x0, #42` encodes as 0xf100a81f; `cmp w0, #42` as 0x7100a81f; `cmp x0, x1` as 0xeb01001f; `cmp sp, #0` as 0xf10003ff; `cmp x0, w1, sxtw` as 0xeb21c01f; `cmp x0, #-1` as 0xb100041f (`cmn x0, #1`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CMP immediate-form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `cmp xzr, #0`).
- CMP shifted-register Rm=31 is XZR/WZR; SP as Rm without extend is rejected by llvm-mc.
- Known-answer: `subs xzr, x0, #42` disassembles as `cmp x0, #42` with the same encoding.

## Quirks

- Extra operands beyond a Shift/Extend are ignored (see bugs).
- encode_cmp does not reject XZR/WZR as immediate-form Rn, so `cmp xzr, #0` encodes as `cmp sp, #0` (see bugs).
- sf is taken from the prepended ZR; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp to 31, so `cmp x0, sp` encodes as `cmp x0, xzr` (see bugs).
- encode_add_sub negates a negative Imm with `-imm_signed`, which panics on i64::MIN in debug (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (non-Reg first operand, extended-register form, negative-imm rewrite).

---

# Confirmed invariants (encode_cmn)

- Same-width GPR CMN immediate form (x0–x30/sp/lr and w0–w30/wsp, imm in unshifted 0..4095 or N<<12 with N in 1..4095, plus explicit lsl #12) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-width GPR CMN shifted-register form (x0–x30/xzr/lr and w0–w30/wzr, lsl/lsr/asr in range) matches llvm-mc (1000 cases).
- Extended-register CMN (sxtw/uxtw/sxtx/uxtx, amount 0..4 including bounds) matches llvm-mc (1000 cases).
- Negative immediate `cmn Rn, #-N` for N in 1..4095 matches llvm-mc's gas rewrite to `cmp Rn, #N` (1000 cases).
- encode_cmn(ops) equals encode_add_sub([ZR] ++ ops, is_sub=false, set_flags=true) where ZR is WZR if Rn is 32-bit else XZR (1000 cases).
- Success-path immediate word: Rd=31, S=1, op=0, bits[28:24]=0b10001, sf from Rn width, unshifted imm12, Rn at [9:5].
- 0 or 1 operands always Err.
- Non-register first operand (Imm/Symbol/Mem/Cond/Shift) with a second operand always Err.
- Known-answer: `cmn x0, #42` encodes as 0xb100a81f; `cmn w0, #42` as 0x3100a81f; `cmn x0, x1` as 0xab01001f; `cmn sp, #0` as 0xb10003ff; `cmn x0, w1, sxtw` as 0xab21c01f; `cmn x0, #-1` as 0xf100041f (`cmp x0, #1`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CMN immediate-form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `cmn xzr, #0`).
- CMN shifted-register Rm=31 is XZR/WZR; SP as Rm without extend is rejected by llvm-mc.
- Known-answer: `adds xzr, x0, #42` disassembles as `cmn x0, #42` with the same encoding.

## Quirks

- Extra operands beyond a Shift/Extend are ignored (see bugs).
- encode_cmn does not reject XZR/WZR as immediate-form Rn, so `cmn xzr, #0` encodes as `cmn sp, #0` (see bugs).
- sf is taken from the prepended ZR; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp to 31, so `cmn x0, sp` encodes as `cmn x0, xzr` (see bugs).
- encode_add_sub negates a negative Imm with `-imm_signed`, which panics on i64::MIN in debug (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (non-Reg first operand, extended-register form, negative-imm rewrite).

---

# Confirmed invariants (encode_cinv)

- Same-width GPR CINV (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cinv([Rd, Rn, cond]) equals encode_csinv([Rd, Rn, Rn, invert(cond)]) (ARM ARM CINV alias of CSINV) (1000 cases).
- encode_cinv([Rd, ZR, cond]) equals encode_csetm([Rd, cond]) (CSETM is CINV with Rn=XZR/WZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=1 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=Rn at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=00 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the three slots always Err.
- Known-answer: `cinv x0, x1, eq` encodes as 0xda811020; `cinv w0, w1, ne` as 0x5a810020; `cinv x0, xzr, eq` as 0xda9f13e0 (same as `csetm x0, eq`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CINV register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cinv sp, ...`).
- CINV takes Wt/Xt only (llvm-mc rejects `cinv d0, ...`).
- Cond AL and NV are invalid for the CINV alias (llvm-mc: "condition codes AL and NV are invalid for this instruction").
- llvm-mc rejects mixed x/w and a fourth operand.
- llvm-mc disassembles `cinv x0, xzr, eq` as `csetm x0, eq` with the same encoding.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- encode_cond accepts al/nv, so CINV AL/NV encodes as CSINV with inverted cond (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cinv sp, ...` encodes as `cinv xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None, get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_cinc)

- Same-width GPR CINC (x0–x30/xzr/lr and w0–w30/wzr, cond in Cond14 including hs/lo aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_cinc([Rd, Rn, cond]) equals encode_csinc([Rd, Rn, Rn, invert(cond)]) (ARM ARM CINC alias of CSINC) (1000 cases).
- encode_cinc([Rd, ZR, cond]) equals encode_cset([Rd, cond]) (CSET is CINC with Rn=XZR/WZR) (1000 cases).
- Success-path word: sf at 31 from Rd width, op=0 at 30, S=0 at 29, bits [28:21]=0b11010100, Rm=Rn at [20:16], invert(cond)=cond XOR 1 at [15:12], op2=01 at [11:10], Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands always Err.
- Unknown condition names (zz, foo, eqq, empty, "eq ", always) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Non-register/non-cond (Imm/Mem/Symbol/Shift/Label) in any of the three slots always Err.
- Known-answer: `cinc x0, x1, eq` encodes as 0x9a811420; `cinc w0, w1, ne` as 0x1a810420; `cinc x0, xzr, eq` as 0x9a9f17e0 (same as `cset x0, eq`).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CINC register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cinc sp, ...`).
- CINC takes Wt/Xt only (llvm-mc rejects `cinc d0, ...`).
- Cond AL and NV are invalid for the CINC alias (llvm-mc: "condition codes AL and NV are invalid for this instruction").
- llvm-mc rejects mixed x/w and a fourth operand.
- llvm-mc disassembles `cinc x0, xzr, eq` as `cset x0, eq` with the same encoding.

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- encode_cond accepts al/nv, so CINC AL/NV encodes as CSINC with inverted cond (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cinc sp, ...` encodes as `cinc xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rn width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None, get_reg non-Reg / cond-not-Cond).

---

# Confirmed invariants (encode_adc)

- Same-width GPR ADC/ADCS (x0–x30/xzr and w0–w30/wzr, both S values) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_adc(ops, true) XOR encode_adc(ops, false) = 1<<29 (ARM ARM S bit).
- Success-path word: Rd at [4:0], Rn at [9:5], Rm at [20:16], sf at 31, S at 29, op at 30 = 0, bits [28:21] = 0b11010000, bits [15:10] = 0.
- Fewer than 3 operands always Err.
- Non-register (Imm/Mem/Shift/Symbol/Cond) in any of the three slots always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1) always Err.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADC register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `adc sp, ...`).
- Known-answer: `adc x0, x1, x2` encodes as 0x9a020020.

## Quirks

- encode_adc does not inspect operands beyond index 2, so a trailing Shift is silently dropped (see bugs).
- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit.

---

# Confirmed invariants (encode_add_sub)

- Immediate-form ADD/SUB/ADDS/SUBS with a valid imm12 or auto-shift (N<<12, N in 1..=0xFFF), including negative-imm alias, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Shifted-register form (LSL/LSR/ASR, Rd/Rn not SP, amount in range) matches llvm-mc.
- NEON vector ADD/SUB Vd.T, Vn.T, Vm.T for T in {8b,16b,4h,8h,2s,4s,2d} matches llvm-mc.
- Fewer than 3 operands always returns Err containing "requires 3 operands".
- encode_add_sub([Rd,Rn,Imm(-N)], is_sub, s) equals encode_add_sub([Rd,Rn,Imm(N)], !is_sub, s) for valid positive N.
- :lo12: Modifier and ModifierOffset produce WordWithReloc { AddAbsLo12, symbol, addend } with imm12 field 0 and ADD-immediate opcode bits (1000 cases).
- :tprel_lo12_nc: / :tprel_hi12: produce TlsLeAddTprelLo12 / TlsLeAddTprelHi12 with sh bit 0 / 1 (1000 cases).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Immediate form register 31 is SP/WSP, never XZR/WZR (llvm-mc rejects `add Rd, XZR, #imm`).
- ADDS/SUBS Rd cannot be SP/WSP (llvm-mc rejects `adds sp, ...`).
- Known-answer: `add x0, x1, #42` encodes as 0x9100a820.

## Quirks

- llvm-mc may disassemble `add w0, wsp, #0` as `mov w0, wsp`; the encoding word still matches.
- llvm-mc may rewrite `add x0, x1, #4096, lsl #0` as `add x0, x1, #1, lsl #12`.
- proptest `prop_assert_eq!` format strings cannot use implicit captures (`{asm}`); use `{}` + args.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (FP regs, ADDS Rd=SP, tprel modifiers).
- explicit_shift is true only for lsl#12; other immediate-form shifts are ignored (see bugs).
- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- ADDS/SUBS with Rd=SP encodes register 31 as XZR (see bugs).

---

# Confirmed invariants (IrConst::cast_float_to_target)

- F64 identity: `cast_float_to_target(fv, F64)` is `Some(F64(fv))` with bit-identical payload (NaN payload and signed zero preserved); 1000 random bit patterns.
- Signed in-range truncation toward zero: for I8/I16/I32/I64, the integer payload equals trunc_toward_zero(fv) when that integer is in range (seed: 3.125 → I32(3)).
- IrType::Void always returns None.
- U8 values in 128..=255 are not saturated to i8::MAX (127); the 8-bit pattern equals n as u8. (Storage form is still I8, so to_i64() sign-extends — see bugs.)
- F32 preserves sign of finite-nonzero and infinite inputs; infinities stay infinite.
- Ptr agrees with from_i64(n, Ptr) / ptr_int for in-range exact integers (default LP64 → I64).

## Environment

- Default target_ptr_size is 8 (LP64). IrConst does not implement PartialEq; tests compare via variant match / to_bits().
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit.

## Quirks

- `from_i64` stores U8/U16/U32 as I64; `cast_float_to_target` stores U8 as I8 and U16 as I16 (U32 already I64). `zero()`/`one()` also use I8 for U8.
- F128 arm calls `long_double` → `f64_to_f128_bytes_lossless`, which panics on f64 subnormals (biased_exp=0, mantissa≠0) via `u128` subtraction underflow.

---

# Confirmed invariants (classify_cast_with_f128)

- Identity: classify(ty, ty, native) = Noop for every IrType and both native flags (1000 cases).
- Non-native F128 reduction: classify(from, to, false) = classify(F128↦F64(from), F128↦F64(to), false).
- native flag is a no-op when neither endpoint is F128.
- Native F32/F64 ↔ F128 is FloatToF128 / F128ToFloat with the from_f32 / to_f32 flag.
- Integer-to-integer casts match size/signedness (IntWiden / IntNarrow / SignedToUnsignedSameSize / UnsignedToSignedSameSize / Noop).
- F32→F64 is FloatToFloat { widen: true }; F64→F32 is FloatToFloat { widen: false }.
- f128_is_native=false never returns SignedToF128 / UnsignedToF128 / F128ToSigned / F128ToUnsigned / FloatToF128 / F128ToFloat.
- Ptr ↔ pointer-width integer is Noop (I32/U32 on ILP32, I64/U64 on LP64).

## Environment

- Default target_ptr_size is 8 (LP64). Tests that exercise ILP32 use set_target_ptr_size(4) with a Drop guard so the thread-local is restored.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (RUSTFLAGS/LLVM_PROFILE_FILE unset); sweep was a manual arm audit.

## Quirks

- Ptr normalization (Ptr ≡ U64/U32) is applied only when neither endpoint is float. Float/F128 ↔ Ptr skips it: Ptr→float is SignedToFloat / SignedToF128, and float→Ptr always sets to_u64=true. See bug_reports/classify_cast_ptr_not_normalized_for_float.md.

---

# Confirmed invariants (encode_adr)

- Immediate-form ADR with Xd (x0–x30/xzr) and imm in [-1048576, 1048575] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases, bounds forced).
- Success-path word: bit 31 (op) = 0, bits [28:24] = 0b10000, Rd at [4:0], SignExtend21(immlo[30:29] | immhi[23:5]<<2) = imm.
- Changing Rd does not change opcode/imm fields; changing imm does not change Rd.
- Symbol / Label / SymbolOffset produce WordWithReloc { AdrPrelLo21, symbol, addend } with word = 0x10000000|rd and imm fields 0 (1000 cases).
- Empty operands, Imm-only, Rd-only, Mem second operand, and invalid name x32 always Err.
- Parser-misclassified Reg/Cond/Barrier names at operand 1 are treated as symbols (get_symbol workaround) and emit AdrPrelLo21.
- Known-answer: `adr x0, #0` encodes as 0x10000000; `adr x0, #1` encodes as 0x30000000.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADR register 31 is XZR, never SP (llvm-mc rejects `adr sp, ...`).
- ADR takes Xd only (llvm-mc rejects `adr w0, ...` and `adr d0, ...`).
- 21-bit signed range: [-1048576, 1048575]; llvm-mc rejects #1048576 and #-1048577.

## Quirks

- encode_adr ignores the is_64 flag from get_reg, so W and FP names encode as Xd with the same register number (see bugs).
- parse_reg_num maps sp to 31, so `adr sp, #imm` encodes as `adr xzr, #imm` (see bugs).
- Out-of-range immediates are truncated to 21 bits via `imm as u32` (see bugs). TODO at load_store.rs:697 notes the missing check.
- get_symbol accepts Modifier / ModifierOffset, so `:lo12:` / `:got:` produce AdrPrelLo21 instead of Err (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier + ModifierOffset).

---

# Confirmed invariants (encode_bic)

- Same-width GPR BIC register form (x0–x30/xzr and w0–w30/wzr, optional lsl/lsr/asr/ror in range) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Valid BIC-immediate (inverted value is an AArch64 bitmask), including Rd=SP/WSP, matches llvm-mc (1000 cases). Encodes as AND with #~imm.
- NEON BIC Vd.T, Vn.T, Vm.T for T in {8b, 16b} matches llvm-mc (1000 cases).
- encode_bic([Rd, Rn, Imm(imm)]) equals encode_logical([Rd, Rn, Imm(~imm)], opc=00) for valid bitmasks (1000 cases).
- Fewer than 3 operands always Err.
- Operand 2 that is Mem/Symbol/Cond/Label/Barrier always Err.
- Invalid Rm names (x32, w32, empty, foo, r0, x) always Err.
- Immediates llvm-mc rejects as non-bitmasks (#0, all-ones, #5, #9, #0x11) are also rejected by encode_bic.
- Known-answer: `bic x0, x1, x2` encodes as 0x8a220020; `bic x0, x1, #1` as 0x927ff820; `bic v0.16b, v1.16b, v2.16b` as 0x4e621c20.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Register form: register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `bic sp, ...`).
- Immediate form: Rd of 31 is SP/WSP, not XZR (llvm-mc rejects `bic xzr, x0, #1`; accepts `bic sp, x0, #1`).
- NEON three-same T is 8B or 16B only.
- Shift amount: W-form [0, 31], X-form [0, 63]. Bound+1 (32 / 64) is rejected by llvm-mc.

## Quirks

- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp/wsp to 31, so register-form SP encodes as XZR (see bugs).
- Immediate-form XZR/WZR encodes as SP/WSP (see bugs).
- Shift amount is masked with 0x3F; 32-bit lsl #32 is accepted (see bugs).
- encode_neon_bic sets Q only for 16b; 8h/4s/2d encode as 8b (see bugs).
- Unknown shift kinds fall through to LSL (`_ => 0b00`) (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid bitmask, unsupported third operand, invalid rm, unknown shift kind).

---

# Confirmed invariants (encode_neon_three_diff_narrow)

- Valid ADDHN/RADDHN/SUBHN/RSUBHN (+2) with mandated (Ta,Tb) pairs matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). Ta∈{8h,4s,2d}; Tb is 8b/16b, 4h/8h, 2s/4s according to is_high.
- encode(..., is_high=true) XOR encode(..., is_high=false) = 1<<30 (ARM ARM Q bit).
- encode(..., u=1) XOR encode(..., u=0) = 1<<29 (ARM ARM U bit).
- Success-path word: bit 31 = 0, bits [28:24] = 0b01110, bit 21 = 1, bits [11:10] = 00, Rd at [4:0], Rn at [9:5], Rm at [20:16], opcode at [15:12], size at [23:22] from Ta (8h=00, 4s=01, 2d=10).
- Fewer than 3 operands always Err.
- Unsupported source Ta (not 8h/4s/2d) always Err.
- Non-register (Imm/Mem/Symbol/Shift/Cond/Label) in any of the three slots always Err.
- Invalid NEON register names (v32, v99, foo, empty, v, v-1) always Err.
- Known-answer: `addhn v0.8b, v1.8h, v2.8h` encodes as 0x0e224020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ADDHN2/RADDHN2/SUBHN2/RSUBHN2 set Q=1 (upper half).
- U=1 is the rounding form (RADDHN/RSUBHN); opcode 0b0100 add-family, 0b0110 sub-family.

## Quirks

- Dest arrangement Tb is ignored (see bugs).
- Rm arrangement is ignored; size comes only from operand 1 (see bugs).
- Extra operands beyond 3 are ignored (see bugs).
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b, so GPR/FP dest encodes as Vd (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid register names via get_neon_reg).

---

# Confirmed invariants (encode_bics)

- Same-width GPR BICS register form (x0–x30/xzr and w0–w30/wzr, optional lsl/lsr/asr/ror in range) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode_bics(ops) XOR encode_bic(ops) = 0b11<<29 for the same 3-reg (+ in-range shift) operands (ARM ARM opc field).
- Success-path word: sf at 31, opc=11 at [30:29], bits [28:24]=0b01010, N=1 at 21, Rd at [4:0], Rn at [9:5], Rm at [20:16], shift at [23:22], imm6 at [15:10].
- Fewer than 3 operands always Err.
- Invalid Rm names (x32, w32, empty, foo, r0, x) always Err.
- Known-answer: `bics x0, x1, x2` encodes as 0xea220020; `bics w0, w1, w2` as 0x6a220020.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- Register form: register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `bics sp, ...`).
- Immediate form: GNU/llvm-mc alias `bics Rd, Rn, #imm` → `ands Rd, Rn, #~imm` (Rd=XZR becomes TST). SP/WSP is not a valid Rd.
- Shift amount: W-form [0, 31], X-form [0, 63]. Bound+1 (32 / 64) is rejected by llvm-mc.
- BICS has no NEON form (llvm-mc rejects `bics v0.16b, ...`).

## Quirks

- sf is taken only from operand 0; mixed x/w is not rejected (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- parse_reg_num maps sp/wsp to 31, so register-form SP encodes as XZR (see bugs).
- Shift amount is masked with 0x3F; 32-bit lsl #32 is accepted (see bugs).
- Unknown shift kinds fall through to LSL (`_ => 0b00`) (see bugs).
- Immediate form is not implemented; get_reg on operand 2 returns Err (see bugs).
- A 4th operand that is not Shift is ignored (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (invalid rm names, extra 4th operand).

---

# Confirmed invariants (encode_bl)

- Symbol / Label produce WordWithReloc { Call26, symbol, addend: 0 } with word = 0x94000000 (bits[31:26]=100101, imm26=0) (1000 cases).
- SymbolOffset(s, addend) preserves symbol and addend and still uses Call26 / 0x94000000 (1000 cases).
- Call26.elf_type() = 283 (R_AARCH64_CALL26).
- encode_bl(ops) XOR encode_branch(ops) = 1<<31 for the same SymbolOffset operands; BL reloc is Call26 and B reloc is Jump26 (1000 cases).
- Empty operands always Err.
- Unaligned or out-of-range Imm (bound±1 / ±4, #1, i64::MIN/MAX) always Err.
- Parser-misclassified Reg/Cond/Barrier names produce Call26 with that name (get_symbol workaround).
- Known-answer (llvm-mc): `bl #0` encodes as 0x94000000; `bl #4` as 0x94000001; `bl #-134217728` as 0x96000000. SUT currently rejects Imm (see bugs).
- Known-answer (SUT): `bl foo` → WordWithReloc { 0x94000000, Call26, "foo", 0 }.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM BL range: aligned offsets in [-134217728, 134217724]; llvm-mc rejects #1, #134217728, #-134217732.
- llvm-mc `bl foo` emits R_AARCH64_CALL26 with instruction word 0x94000000 (imm26 filled later).

## Quirks

- encode_bl does not encode the immediate form; get_symbol rejects Imm (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- get_symbol accepts Modifier / ModifierOffset, dropping the kind, so `:lo12:` becomes Call26 (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier).

---

# Confirmed invariants (encode_blr)

- `blr Xn` / `blr xzr` / `blr lr` (x0–x30, xzr, lr, uppercase X0) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: 0xd63f0000 | (rn << 5); bits[31:25]=1101011, opc[24:21]=0001, op4[4:0]=0, Rn at [9:5] (1000 cases).
- encode_blr(ops) XOR encode_br(ops) = 1<<21 for the same Xn operand (ARM ARM opc bit 21) (1000 cases).
- Empty operands always Err.
- Non-register (Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Known-answer: `blr x0` encodes as 0xd63f0000; `blr x17` as 0xd63f0220.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- BLR register 31 is XZR, never SP (llvm-mc rejects `blr sp`).
- BLR takes Xn only (llvm-mc rejects `blr w0` and `blr d0`).
- llvm-mc accepts `blr x31` as `blr xzr`; `blr lr` as `blr x30`.
- Codegen emits `blr x17` for indirect calls (calls.rs:233).

## Quirks

- encode_blr discards the is_64 flag from get_reg, so W names encode as Xn (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `blr sp` encodes as `blr xzr` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_reg (parse_reg_num None via encode_blr_neg_invalid_name).

---

# Confirmed invariants (encode_br)

- `br Xn` / `br xzr` / `br lr` (x0–x30, xzr, lr, uppercase X0) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Success-path word: 0xd61f0000 | (rn << 5); bits[31:25]=1101011, opc[24:21]=0000, op4[4:0]=0, Rn at [9:5] (1000 cases).
- encode_blr(ops) XOR encode_br(ops) = 1<<21 for the same Xn operand (ARM ARM opc bit 21) (1000 cases).
- Empty operands always Err.
- Non-register (Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label) always Err.
- Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) always Err.
- Known-answer: `br x0` encodes as 0xd61f0000; `br x17` as 0xd61f0220.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- BR register 31 is XZR, never SP (llvm-mc rejects `br sp`).
- BR takes Xn only (llvm-mc rejects `br w0` and `br d0`).
- llvm-mc accepts `br x31` as `br xzr`; `br lr` as `br x30`.
- Codegen emits `br x0` for indirect jumps (emit.rs:1760) and `br x17` for jump tables (emit.rs:1808).

## Quirks

- encode_br discards the is_64 flag from get_reg, so W names encode as Xn (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `br sp` encodes as `br xzr` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as GPRs (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_reg (parse_reg_num None via encode_br_neg_invalid_name).

---

# Confirmed invariants (encode_branch)

- Symbol / Label / SymbolOffset produce WordWithReloc { Jump26, symbol, addend } with word = 0x14000000 and imm26 field 0 (1000 cases). ELF type is 282 (R_AARCH64_JUMP26).
- encode_bl(ops).word XOR encode_branch(ops).word = 1<<31 for the same SymbolOffset operands (ARM ARM bit 31); reloc types Call26 vs Jump26; same symbol and addend (1000 cases).
- Success-path reloc word: bits[31:26] = 000101, bits[25:0] = 0.
- Empty operands always Err.
- Unaligned or out-of-range Imm always Err (because all Imm currently Err — see bugs).
- Parser-misclassified Reg/Cond/Barrier names at operand 0 are treated as symbols (get_symbol workaround) and emit Jump26 (1000 cases).
- Known-answer: llvm-mc `b #0` encodes as 0x14000000; `b #4` as 0x14000001. SUT does not yet match (see bugs).

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM B signed PC offset: [-134217728, 134217724], multiple of 4.
- llvm-mc rejects bare `b` (too few operands), `b foo, x0` (invalid operand), `b #1` (expected label or encodable integer pc offset), `b :lo12:foo`.
- Codegen emits `b <label>`, not `b #imm`.

## Quirks

- encode_branch never encodes Imm: get_symbol rejects it (see bugs).
- Extra operands beyond index 0 are ignored (see bugs).
- get_symbol accepts Modifier / ModifierOffset, so `:lo12:` produces Jump26 instead of Err (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier via encode_branch_symbol_misclassified).

---

# Confirmed invariants (encode_cbz)

- Symbol / Label / SymbolOffset produce WordWithReloc { CondBr19, symbol, addend } with word = (sf<<31)|(0b011010<<25)|(op<<24)|Rt and imm19 field 0 (1000 cases). ELF type is 280 (R_AARCH64_CONDBR19).
- encode_cbz(ops, true).word XOR encode_cbz(ops, false).word = 1<<24 for the same SymbolOffset operands (ARM ARM op bit); both reloc types CondBr19; same symbol and addend (1000 cases).
- Success-path reloc word: bits[30:25] = 011010, sf at 31 from Rt width, op at 24 from is_nz, Rt at [4:0], bits[23:5] = 0.
- Empty operands and a missing label always Err.
- Unaligned or out-of-range Imm always Err (because all Imm currently Err — see bugs).
- Mem / Shift / Extend / RegArrangement / Expr / RegList in the label slot always Err.
- Parser-misclassified Reg/Cond/Barrier names at operand 1 are treated as symbols (get_symbol workaround) and emit CondBr19 (1000 cases).
- Known-answer: llvm-mc `cbz x0, #0` encodes as 0xb4000000; `cbz w0, #0` as 0x34000000; `cbnz x0, #4` as 0xb5000020. SUT does not yet match (see bugs). llvm-mc `cbz x0, foo` is a CondBr19 reloc with word 0xb4000000 — SUT matches.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM CBZ/CBNZ signed PC offset: [-1048576, 1048572], multiple of 4.
- CBZ register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `cbz sp, ...`).
- CBZ takes Wt/Xt only (llvm-mc rejects `cbz d0, ...` and `cbz wsp, ...`).
- llvm-mc rejects bare `cbz` / `cbz x0` (too few operands), `cbz x0, #0, x1` (invalid operand), `cbz x0, #1` (expected label or encodable integer pc offset).
- Codegen emits `cbz xN, .Llabel` / `cbnz wN, .Llabel`, not `cbz Rt, #imm`.

## Quirks

- encode_cbz never encodes Imm: get_symbol rejects it (see bugs).
- Extra operands beyond index 1 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `cbz sp, L` encodes as `cbz xzr, L` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- llvm-mc accepted `cbz x0, :lo12:foo` as a branch19 fixup; get_symbol also accepts Modifier (kind discarded).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of get_symbol (Reg/Cond/Barrier via encode_cbz_symbol_misclassified; other kinds via encode_cbz_neg_bad_label_kind).

---

# Confirmed invariants (encode_ccmp_ccmn)

- Same-width GPR CCMP/CCMN immediate form (`Rn, #imm5, #nzcv, cond` with imm5 in [0,31], nzcv in [0,15], all 16 cond codes plus hs/cs/lo/cc aliases) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Same-width GPR CCMP/CCMN register form (`Rn, Rm, #nzcv, cond`) matches llvm-mc (1000 cases).
- encode_ccmp_ccmn(ops, true).word XOR encode_ccmp_ccmn(ops, false).word = 1<<30 (ARM ARM op bit) for both forms (1000 cases).
- Success-path word: sf at 31, op at 30, S=1 at 29, bits [28:21]=0b11010010, cond at [15:12], Rn at [9:5], nzcv at [3:0], bit 10=0, bit 4=0; o2 at 11 is 1 for immediate (imm5 at [20:16]) and 0 for register (Rm at [20:16]).
- Fewer than 4 operands always Err.
- Invalid condition names (xx, foo, empty, eqz, n, zzzz) always Err.
- Invalid Rm names (x32, w32, foo, empty, r0, x, x-1, x99) always Err.
- Mem / Symbol / Shift / Extend / Label / Barrier in slots 1, 2, or 3 always Err.
- Known-answer: `ccmp x0, #0, #0, eq` encodes as 0xfa400800; `ccmp x0, x1, #0, eq` as 0xfa410000; `ccmn x0, #0, #0, eq` as 0xba400800.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- CCMP/CCMN register 31 is XZR/WZR, never SP/WSP (llvm-mc rejects `ccmp sp, ...` and `ccmp wsp, ...`).
- CCMP/CCMN takes Wt/Xt only (llvm-mc rejects `ccmp d0, ...`).
- imm5 unsigned [0, 31]; nzcv unsigned [0, 15]; llvm-mc rejects #-1, #32, #16.
- llvm-mc rejects mixed x/w (`ccmp x0, w1` / `ccmn w0, x0`) and a fifth operand.

## Quirks

- imm5 is stored as `*imm5 as u32 & 0x1F` and nzcv as `*nzcv as u32 & 0xF`, so out-of-range values are truncated (see bugs).
- Extra operands beyond index 3 are ignored (see bugs).
- parse_reg_num maps sp/wsp to 31, so `ccmp sp, ...` encodes as `ccmp xzr, ...` (see bugs).
- parse_reg_num accepts d/s/q/v/h/b prefixes, so FP names encode as 32-bit GPRs (see bugs).
- sf is taken only from operand 0; Rm width is never checked, so mixed x/w encodes (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (encode_cond None, parse_reg_num None on Rm, unsupported operand kinds).

---

# Confirmed invariants (encode_neon_shll)

- Valid SSHLL/USHLL(+2) with mandated (Tb,Ta) and shift in [0, esize-1] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- is_high XOR toggles only Q (bit 30) (1000 cases).
- u_bit XOR toggles only U (bit 29) (1000 cases).
- encode_neon_shll(..., Imm(0)) equals encode_neon_xtl and llvm-mc sxtl/uxtl(+2) (1000 cases).
- Success-path word: bit31=0, Q at 30, U at 29, bits[28:23]=011110, immh:immb=esize+shift, opcode=101001, Rn, Rd.
- Arity < 3, unsupported source Tb, non-matching operand kinds, and invalid NEON names always Err (1000 cases).
- Known-answer: `sshll v0.8h, v1.8b, #0` = 0x0f08a420; `#7` = 0x0f0fa420; `ushll` #0 = 0x2f08a420; `sshll2 v0.8h, v1.16b, #0` = 0x4f08a420; `ushll2 ... #7` = 0x6f0fa420; `sshll v0.4s, v1.4h, #0` = 0x0f10a420; `#15` = 0x0f1fa420; `sshll v0.2d, v1.2s, #0` = 0x0f20a420; `#31` = 0x0f3fa420; `ushll2 v0.2d, v1.4s, #31` = 0x6f3fa420; `sxtl v0.8h, v1.8b` = 0x0f08a420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD SSHLL/USHLL: `0 Q U 011110 immh immb 101001 Rn Rd`. Q=0 Tb={8B,4H,2S}; Q=1 Tb={16B,8H,4S}. Ta is 8H/4S/2D. shift in 0..(esize-1). immh:immb = esize + shift.
- Dispatch: encoder/mod.rs:614-617 ushll/ushll2/sshll/sshll2. Sibling encode_neon_xtl is the documented #0 alias (same job at shift 0).
- Callers: assembler README NEON widen/long table lists sshll/ushll/sxtl/uxtl (+2).

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Dest arrangement is discarded (see bugs).
- Shift is `get_imm as u32` with no range check; #8 for 8b encodes as 16-bit esize; #-1 overflows in debug (see bugs).
- Operand::Reg dest (GPR/FP names) encodes via parse_reg_num (see bugs).
- Q comes only from is_high, not from Tb, so sshll2+8b and sshll+16b encode (see bugs).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity < 3, unsupported Tb, kinds, invalid names, GPR dest, Q vs Tb).

---

# Confirmed invariants (encode_neon_sqshrun)

- Valid vector SQSHRUN/SQRSHRUN (+2) with Ta in {8h,4s,2d}, Tb matching Ta and the 2-suffix, Vd/Vn in v0–v31, shift in [1, dest_esize] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., is_high=false) XOR encode(..., is_high=true) = 1<<30 (ARM ARM Q bit) (1000 cases).
- encode(..., is_rounding=false) XOR encode(..., is_rounding=true) = 1<<11 (opcode 100001 vs 100011) (1000 cases).
- Success-path word: bit 31=0, Q at 30, U=1 at 29, bits [28:23]=011110, immh:immb at [22:16]=src_esize-shift, opcode at [15:10]=100001/100011, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, unsupported source Ta (not 8h/4s/2d), non-RegArrangement/non-Imm kinds, invalid names (v32, foo, empty, v, v-1), and bare Operand::Reg source always Err.
- Known-answer: `sqshrun v0.8b, v1.8h, #1` encodes as 0x2f0f8420; `#8` as 0x2f088420; `sqshrun2 v0.16b, v1.8h, #1` as 0x6f0f8420; `sqrshrun v0.8b, v1.8h, #1` as 0x2f0f8c20; `sqrshrun2 v0.4s, v1.2d, #32` as 0x6f208c20; `sqshrun v0.4h, v1.4s, #16` as 0x2f108420.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift by immediate SQSHRUN/SQRSHRUN: `0 Q 1 011110 immh immb opcode Rn Rd`. opcode 100001 non-rounding / 100011 rounding. U=1 always (signed-to-unsigned saturating narrow). Q=0 lower half / Q=1 (`2` suffix) upper half. dest_esize = 8<<HighestSetBit(immh); shift = 2*esize - UInt(immh:immb) in [1, dest_esize]. Ta/Tb: 8H→8B/16B (1..8), 4S→4H/8H (1..16), 2D→2S/4S (1..32).
- Dispatch: encoder/mod.rs:649-650 sqrshrun/sqrshrun2; encoder/mod.rs:841-842 sqshrun/sqshrun2.
- Sibling encode_neon_shrn neon.rs:1443-1444 checks `shift > half_bits` with half_bits = source/2. encode_neon_qshrn / encode_neon_scalar_qshrn fail the same-job gate.
- Callers: assembler README NEON narrow table; no codegen emission of sqshrun found.

## Quirks

- Shift range uses source element size (16/32/64), so dest_esize+1 through source_esize encode (see bugs).
- Dest arrangement is discarded (see bugs).
- Extra operands beyond index 2 are ignored (see bugs).
- get_neon_reg accepts Operand::Reg, so GPR/FP dest encodes as Vd (see bugs).
- Shift is `*v as u32`, so Imm(1+2^32) encodes as #1 (see bugs).
- Bare Operand::Reg source Errs via empty arrangement (not a bug).
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / Ta / *v as u32 / get_neon_reg Reg dest+source / extra / mismatched Tb).

---

# Confirmed invariants (encode_neon_shift_left_imm)

- Valid vector SQSHL/UQSHL immediate with T in {8b,16b,4h,8h,2s,4s,2d}, Vd/Vn in v0–v31, shift in [0, esize-1] matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- encode(..., u=0) XOR encode(..., u=1) = 1<<29 (ARM ARM U bit SQSHL vs UQSHL) (1000 cases).
- Same-esize Q=0 vs Q=1 arrangements XOR = 1<<30 (1000 cases).
- shift+1 (still in range) adds 1 to immh:immb at bits [22:16] (1000 cases).
- Success-path word: bit 31=0, Q at 30, U at 29, bits [28:23]=011110, immh:immb at [22:16]=esize+shift, opcode at [15:11]=01110, bit 10=1, Rn at [9:5], Rd at [4:0].
- Fewer than 3 operands, unsupported T (1d, 8s, empty), invalid names (v32, foo, empty, v, v-1), non-register kinds, and non-Imm shift always Err.
- Known-answer: `sqshl v0.8b, v1.8b, #0` encodes as 0x0f087420; `#7` as 0x0f0f7420; `uqshl v0.8b, v1.8b, #0` as 0x2f087420; `sqshl v0.16b, v1.16b, #3` as 0x4f0b7420; `sqshl v0.4h, v1.4h, #15` as 0x0f1f7420; `sqshl v0.2d, v1.2d, #0` as 0x4f407420; `#63` as 0x4f7f7420; `uqshl v31.4s, v30.4s, #31` as 0x6f3f77df.

## Environment

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding
- ARM ARM Advanced SIMD shift left (immediate) SQSHL/UQSHL: `0 Q U 011110 immh immb 01110 1 Rn Rd`. T in {8B,16B,4H,8H,2S,4S,2D}. Q=0 && esize==64 is Reserved (no 1D). shift = UInt(immh:immb) - esize in [0, esize-1]. U=0 SQSHL / U=1 UQSHL. opcode=01110.
- Dispatch: encoder/mod.rs:544-551 sqshl Imm => u=0; uqshl Imm => u=1. Register-form sqshl/uqshl go to encode_neon_three_same (different job).
- Sibling encode_neon_shl is SHL (opcode 01010, U=0). Sibling encode_neon_sli is SLI (opcode 01010, U=1). Same-job gate fails.
- Callers: assembler README NEON shifts table lists sqshl/uqshl. Scalar SQSHL Bd/Hd/Sd/Dd uses a different encoding (out of scope).

## Quirks

- Extra operands beyond index 2 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- Negative Imm debug-panics (`esize + (shift as u32)` overflow); shift == esize wraps into the next lane size; i64 Imm truncates via `as u32` (see bugs).
- parse_reg_num accepts x/w/d/s/q/h/b prefixes, so non-V names encode as V registers (see bugs).
- get_neon_reg accepts Operand::Reg, so a bare V/GPR source encodes as Rn (see bugs). Dest as Operand::Reg still Errs via empty arrangement.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit (arity / T / extra / shift range / mismatched T / non-V prefix / Operand::Reg dest+source).


---

# Confirmed invariants (encode_neon_cmp_zero)

- Valid CMEQ/CMGE/CMGT/CMLE/CMLT Vd.T, Vn.T, #0 with T in {8b,16b,4h,8h,2s,4s,2d}, v0–v31, ARM-correct (U, opcode) matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases).
- Changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; U in bit 29 (1000 cases).
- Success-path word: bit31=0, Q at 30, U at 29, bits[28:24]=01110, size at [23:22], bits[21:17]=10000, opcode at [16:12], bits[11:10]=10, Rn at [9:5], Rd at [4:0] (1000 cases).
- Arity 0–1 always Err (1000 cases).
- Invalid T {4b,8d,2h,1s,32b,empty} always Err (1000 cases).
- Imm/Mem dest or Imm src always Err (1000 cases).
- Uppercase V prefix matches llvm-mc (1000 cases).
- Known-answer: `cmeq v0.8b, v1.8b, #0` = 0x0e209820; `cmeq v0.16b` = 0x4e209820; `cmge v0.4s` = 0x6ea08820; `cmgt v0.2d` = 0x4ee08820; `cmle v0.8h` = 0x6e609820; `cmlt v0.4h` = 0x0e60a820; `cmeq v31.8b, v31.8b, #0` = 0x0e209bff.
- Extra operand, mismatched T, reserved .1d, and GPR/non-V names currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_neon_cmp_zero)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding.
- ARM ARM Advanced SIMD two-register miscellaneous integer compare-with-zero: T in {8B,16B,4H,8H,2S,4S,2D}; size:Q=11:0 reserved; (U,opcode): CMEQ (0,01001), CMGE (1,01000), CMGT (0,01000), CMLE (1,01001), CMLT (0,01010).
- Dispatch: encoder/mod.rs:566-584 cmeq/cmge/cmgt Imm(0); encoder/mod.rs:665-666 cmlt/cmle.
- encode_neon_cmp_zero checks operands.len() < 2; extra ignored; source arrangement discarded; neon_arr_to_q_size accepts 1d.
- get_neon_reg accepts Operand::Reg; parse_reg_num accepts x/w/d/s/q/v/h/b.
- Scalar `cmeq Dd, Dn, #0` is a different encoding (bits[31:30]=01) — not this vector helper.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep: invalid-T/nonreg/alt-spellings passing.
- Four SUT bugs, not quirks. See pbt-out/bug_reports/encode_neon_cmp_zero_*.md.

## Quirks (encode_neon_cmp_zero)

- Extra operands beyond index 1 are ignored (see bugs).
- Source arrangement is discarded (see bugs).
- neon_arr_to_q_size 1d is encoded (see bugs).
- Operand::Reg source and x/w prefixes encode as V registers (see bugs).
- Bare dest (empty arrangement) hits unsupported arrangement Err via neon_arr_to_q_size.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit plus invalid-T/nonreg/alt-spellings.

# Confirmed invariants (encode_ldp_stp)

- Valid GPR LDP/STP signed-offset Rt1, Rt2, [Xn|SP, #imm7*scale] with scale 4 (W) or 8 (X), imm7 in [-64,63], LDP requiring Rt1!=Rt2, matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins ldp x0,x1,[x2]=0xA9400440, ldp x0,x1,[x2,#8]=0xA9408440, stp w2,w3,[x4,#4]=0x29008C82.
- Pre/post-index with writeback Rn not in {Rt1,Rt2} (unless SP) match llvm-mc (1000 cases). KAT pins ldp x0,x1,[x2],#16=0xA8C10440, stp x0,x1,[x2,#16]!=0xA9810440.
- ARM pair layout: opc [31:30] 00/10, bits[29:27]=101, V=0, mode 001/010/011, L, imm7, Rt2, Rn, Rt (1000 cases).
- Metamorphic: Rt1+1 adds 1, Rt2+1 adds 1<<10, Rn+1 adds 1<<5, load XOR store = 1<<22, pre XOR post = 0b10<<23 (1000 cases).
- Arity 0/1/2 and non-memory third operands return Err (1000 cases).
- SIMD S/D/Q signed-offset pairs match llvm-mc (1000 cases, sweep). KAT pins ldp d0,d1,[x2,#16]=0x6D410440, ldp s0,s1,[x0]=0x2D400400, ldp q0,q1,[sp,#-32]=0xAD7F07E0.
- Alt spellings Xn/WZR/SP/lr/w31 match llvm-mc (1000 cases, sweep).
- Extra operand, SP dest, XZR/W base, mixed width, writeback overlap, LDP Rt1==Rt2, and out-of-range offset currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_ldp_stp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6).
- Harness: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs, cargo test --lib encode_ldp_stp, proptest cases=1000.
- Dispatch: encoder/mod.rs:492-493 `"ldp"`/`"stp"` => encode_ldp_stp.
- Sibling encode_ldnp_stnp is not a same-job independent differential (non-temporal, bits[25:23]=000).
- coverage_gaps had no LLVM profraw; sweep was a manual arm audit plus SIMD/alt-spellings. Closed: tier round spent.

## Quirks (encode_ldp_stp)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_ldp_stp does not distinguish them for Rt vs Rn (see bugs).
- parse_reg_num accepts W-prefixed bases (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- Rt2 width is discarded; opc/shift come from Rt1 (see bugs).
- Out-of-range/unaligned offsets are shifted and masked into imm7 (see bugs).
- llvm-mc accepts STP with Rt1==Rt2 (including XZR,XZR) but rejects LDP with Rt1==Rt2.
- Writeback with Rn=SP is valid even when Rt numbers match 31.
- proptest 1.11 requires `#[test]` inside `proptest! { }`.


# Confirmed invariants (encode_ldnp_stnp)

- Valid integer LDNP/STNP Rt1, Rt2, [Xn|SP{, #simm}] with simm = imm7*(4 or 8) in the ARM range matches llvm-mc `-triple=aarch64 -show-encoding` (1000 cases). KAT pins ldnp x0,x1,[x2]=0xA8400440, ldnp x0,x1,[x2,#8]=0xA8408440, stnp w2,w3,[x4,#4]=0x28008C82, ldnp x0,x1,[x2,#-8]=0xA87F8440, stnp xzr,xzr,[sp]=0xA8007FFF.
- ARM LDNP/STNP layout holds: opc 101 V=0 000 L imm7 Rt2 Rn Rt; opc=10 Xt / 00 Wt; bits[25:23]=000 (1000 cases).
- Metamorphic: Rt1+1 adds 1, Rt2+1 adds 1<<10, Rn+1 adds 1<<5, imm7+1 isolates bits[21:15], load XOR store = 1<<22 (1000 cases).
- Arity 0/1/2 and non-Mem third operand (including pre/post writeback) return Err (1000 cases).
- Alt spellings uppercase X / wzr,W30,SP / lr / w31 match llvm-mc (1000 cases, sweep).
- Extra operands, SP dest, XZR/x31/W/WSP base, mixed X/W, out-of-range/unaligned offset, and SIMD S/D/Q currently disagree with llvm-mc/gas (see bugs).

## Environment (encode_ldnp_stnp)

- Differential reference: /home/toan/tools/llvm15-official/bin/llvm-mc -triple=aarch64 -show-encoding (LLVM 15.0.6). GNU as 2.38 agrees on SIMD and register-class rejections. gas warns but still encodes LDNP Rt1==Rt2; llvm-mc encodes it without warning — not a rejection contract.
- ARM ARM C6 LDNP/STNP: opc 101 V 000 L imm7 Rt2 Rn Rt. No pre/post-index. Integer opc=00 Wt scale=4 [-256,252]; opc=10 Xt scale=8 [-512,504]. SIMD V=1 opc=00/01/10 for S/D/Q.
- Dispatch: encoder/mod.rs:496-497 `"ldnp"`/`"stnp"` => encode_ldnp_stnp.
- Sibling encode_ldp_stp is not a same-job independent differential (pre/post, bits[25:23] in {001,010,011}; shared get_reg / same crate).
- Doc contract load_store.rs:517 TODO admits V=1 unimplemented on an input get_reg accepts.
- proptest 1.11 requires `#[test]` inside `proptest! { }`. 1000 cases. Sweep round 1/1 spent.
- coverage_gaps had no LLVM profraw in this session (C++ reporter listed unrelated binaries and claimed NOT LINKED). Sweep was a manual arm audit plus encode_ldnp_stnp_diff_alt_spellings. Closed: every documented behavior has a property; remaining gaps are filed bugs.
- Four failing properties are SUT bugs, not quirks. See pbt-out/bug_reports/encode_ldnp_stnp_*.md.

## Quirks (encode_ldnp_stnp)

- ASCII case of register names is accepted (to_lowercase / parse_reg_num).
- parse_reg_num maps SP and XZR both to 31; encode_ldnp_stnp does not distinguish them (see bugs).
- parse_reg_num accepts FP prefixes (d/s/q/v/h/b) (see bugs).
- No operands.len() upper bound; extra operands are ignored (see bugs).
- get_reg width of Rt2 is discarded (see bugs).
- Offset is shifted and masked into imm7 with no range/align check (see bugs).
- V is hardcoded 0 (see bugs / documented limitation).
- llvm-mc accepts `w31` as WZR and `lr` as x30.
- proptest 1.11 requires `#[test]` inside `proptest! { }` or the functions are not registered.
- `coverage_gaps` had no LLVM profraw in this session; sweep was a manual arm audit of the 22-line body.
