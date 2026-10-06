# Properties: encode_branch_instr

Requested `--func encode_branch` is absent from `src/backend/riscv/assembler/encoder/base.rs`. The in-scope B-type encoder is `encode_branch_instr`.

## encode_branch_instr_diff_imm_llvm_mc
- Tier: 4
- Rationale: Strongest evidenced oracle is differential vs llvm-mc RISC-V assembler. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree B-type decoder). encode_b / beqz / bnez / bgez / bltz / bgt / C.BEQZ rejected as same-job siblings (private packer / zero-compare / swapped-operand / compressed). Reference KAT pins the llvm-mc connection before PBT.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:372 encode_jal_diff_imm_llvm_mc
- Formal: ∀ mn ∈ {beq,bne,blt,bge,bltu,bgeu}, rs1,rs2 ∈ GPR, off ∈ even[-4096,4094]. encode_branch_instr([Reg(rs1),Reg(rs2),Imm(off)], funct3(mn)) = Word(w) ∧ w = llvm-mc("mn rs1, rs2, off")
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_branch_instr
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rs1, rs2, off]
  domain: { mn: {beq,bne,blt,bge,bltu,bgeu}, rs1: gpr, rs2: gpr, off: even[-4096,4094] }
  relation:
    op: eq
    lhs: encode_branch_instr([Reg(rs1), Reg(rs2), Imm(off)], funct3(mn))
    rhs: Word(llvm_mc("mn rs1, rs2, off"))
generators:
  mn: { gen: oneof, values: ["beq","bne","blt","bge","bltu","bgeu"] }
  rs1: { gen: string }
  rs2: { gen: string }
  off: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: README.md:303 B-type beq/bne/blt/bge/bltu/bgeu; encoder/mod.rs:468-473 dispatch
```

## encode_branch_instr_isa_b_type
- Tier: 3
- Rationale: Algebraic invariant from RISC-V B-type layout. Stronger differential is the sibling property above; this unpacks opcode/funct3/rs1/rs2/imm independently of encode_b.
- Doc contract: encoder/mod.rs:301 "B-type: imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode" — asserted fingerprint ab669e38
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:401 encode_jal_isa_j_type
- Formal: ∀ funct3 ∈ {0,1,4,5,6,7}, rs1,rs2 ∈ 0..31, off ∈ even[-4096,4094]. let w = encode_branch_instr([Reg(x{rs1}),Reg(x{rs2}),Imm(off)], funct3) in Word. unpack_b(w) = (OP_BRANCH=0b1100011, funct3, rs1, rs2, off)
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_branch_instr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [funct3, rs1, rs2, off]
  domain: { funct3: {0,1,4,5,6,7}, rs1: 0..31, rs2: 0..31, off: even[-4096,4094] }
  relation:
    op: eq
    lhs: unpack_b(word(encode_branch_instr([Reg(x{rs1}),Reg(x{rs2}),Imm(off)], funct3)))
    rhs: (0b1100011, funct3, rs1, rs2, off)
generators:
  funct3: { gen: oneof, values: [0,1,4,5,6,7], type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: encoder/mod.rs:301 B-type layout; README.md:355
```

## encode_branch_instr_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic — ABI names, xN, and fp=s0/x8 encode the same rs1/rs2. Stronger differential already covers numeric encodings; this checks alias invariance.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:411 encode_jal_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31, off ∈ even[-4096,4094]. encode_branch_instr([Reg(x{n}),Reg(x{m}),Imm(off)], 0) = encode_branch_instr([Reg(ABI[n]),Reg(ABI[m]),Imm(off)], 0) ∧ (n=8 ⇒ fp alias equals x8)
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_branch_instr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, off]
  domain: { n: 0..31, m: 0..31, off: even[-4096,4094] }
  relation:
    op: eq
    lhs: encode_branch_instr([Reg(x{n}),Reg(x{m}),Imm(off)], 0)
    rhs: encode_branch_instr([Reg(ABI[n]),Reg(ABI[m]),Imm(off)], 0)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: parser.rs:22 ABI and xN names; encoder/mod.rs:154 reg_num
```

## encode_branch_instr_reloc_symbol
- Tier: 4
- Rationale: Differential/invariant for the reloc arm. Symbol and Label 3rd operands produce WordWithReloc whose word equals the zero-offset encoding (llvm-mc `mn rs1, rs2, 0`) with RelocType::Branch, symbol=s, addend=0. Round-trip rejected (no decoder of pending relocs).
- Doc contract: encoder/mod.rs:75 "R_RISCV_BRANCH - 12-bit PC-relative branch (B-type)" — asserted fingerprint 21770b33
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:425 encode_jal_reloc_symbol
- Formal: ∀ mn ∈ {beq,bne,blt,bge,bltu,bgeu}, rs1,rs2 ∈ GPR, s ∈ ident. encode_branch_instr([Reg(rs1),Reg(rs2),Symbol(s)], funct3(mn)) = WordWithReloc{word=llvm-mc("mn rs1, rs2, 0"), reloc_type=Branch, symbol=s, addend=0} ∧ same for Label(s)
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_branch_instr
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rs1, rs2, s]
  domain: { mn: {beq,bne,blt,bge,bltu,bgeu}, rs1: gpr, rs2: gpr, s: ident }
  relation:
    op: eq
    lhs: encode_branch_instr([Reg(rs1),Reg(rs2),Symbol(s)], funct3(mn))
    rhs: WordWithReloc{word: llvm_mc("mn rs1, rs2, 0"), reloc_type: Branch, symbol: s, addend: 0}
generators:
  mn: { gen: oneof, values: ["beq","bne","blt","bge","bltu","bgeu"] }
  rs1: { gen: string }
  rs2: { gen: string }
  s: { gen: string }
evidence: encoder/mod.rs:75 RelocType::Branch; README.md:383 R_RISCV_BRANCH; base.rs:134-141
```

## encode_branch_instr_neg_imm_oob_odd
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects immediates that are not a multiple of 2 in [-4096, 4094]. README B-type is a 13-bit signed even offset. Documented bound must be sampled at bound±1.
- Doc contract: README.md:355 "B-type:  [imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode]" — asserted fingerprint 0bc31053
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:445 encode_jal_neg_imm_oob_odd
- Formal: ∀ rs1,rs2 ∈ GPR, imm ∈ ℤ. (imm odd ∨ imm ∉ [-4096,4094]) ∧ llvm-mc rejects "beq rs1, rs2, imm" ⇒ encode_branch_instr([Reg(rs1),Reg(rs2),Imm(imm)], 0) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: failing
- Counterexample: encode_branch_instr([Reg("x0"), Reg("x0"), Imm(1)], 0) → Ok(Word(0x00000063))
- Bug report: bug_reports/encode_branch_instr_imm_oob_odd.md

```property
function: encoder.encode_branch_instr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs1, rs2, imm]
  domain: { rs1: gpr, rs2: gpr, imm: oob_or_odd_b_imm }
  relation:
    op: holds
    lhs: encode_branch_instr([Reg(rs1),Reg(rs2),Imm(imm)], 0).is_err()
expected_error: String
generators:
  rs1: { gen: string }
  rs2: { gen: string }
  imm: { gen: int, type: i64 }
evidence: llvm-mc "immediate must be a multiple of 2 bytes in the range [-4096, 4094]"; README.md:355 B-type bit 0 implicit 0
```

## encode_branch_instr_neg_extra
- Tier: 4
- Rationale: Negative/error. llvm-mc rejects a fourth operand. B-type is three-operand. Extra operands must Err, not be ignored.
- Doc contract: base.rs:143 "branch: expected offset or label as 3rd operand" — asserted fingerprint b57fa706
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:462 encode_jal_neg_extra
- Formal: ∀ rs1,rs2 ∈ GPR, off ∈ even[-4096,4094], extra ∈ Operand. encode_branch_instr([Reg(rs1),Reg(rs2),Imm(off), extra], 0) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: failing
- Counterexample: encode_branch_instr([Reg("x0"), Reg("x0"), Imm(0), Imm(0)], 0) → Ok(Word(0x00000063))
- Bug report: bug_reports/encode_branch_instr_extra_operand.md

```property
function: encoder.encode_branch_instr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs1, rs2, off, extra]
  domain: { rs1: gpr, rs2: gpr, off: even[-4096,4094], extra: Operand }
  relation:
    op: holds
    lhs: encode_branch_instr([Reg(rs1),Reg(rs2),Imm(off),extra], 0).is_err()
expected_error: String
generators:
  rs1: { gen: string }
  rs2: { gen: string }
  off: { gen: int, min: -4096, max: 4094, type: i64 }
  extra: { gen: string }
evidence: llvm-mc "invalid operand for instruction" on extra; B-type is rs1,rs2,offset
```

## encode_branch_instr_neg_arity_fp
- Tier: 4
- Rationale: Negative/error. Empty list, missing 3rd operand, and FP registers as rs1/rs2 must Err. llvm-mc: too few operands / invalid operand. get_reg rejects FP names.
- Doc contract: parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:507 encode_jal_neg_fp / encode_jal_neg_empty
- Formal: ∀ fp ∈ FPR, rs ∈ GPR, off ∈ even[-4096,4094]. encode_branch_instr([], 0) = Err ∧ encode_branch_instr([Reg(rs),Reg(rs)], 0) = Err ∧ encode_branch_instr([Reg(fp),Reg(rs),Imm(off)], 0) = Err ∧ encode_branch_instr([Reg(rs),Reg(fp),Imm(off)], 0) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_branch_instr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp, rs, off]
  domain: { fp: fpr, rs: gpr, off: even[-4096,4094] }
  relation:
    op: holds
    lhs: encode_branch_instr([],0).is_err() && encode_branch_instr([Reg(rs),Reg(rs)],0).is_err() && encode_branch_instr([Reg(fp),Reg(rs),Imm(off)],0).is_err()
expected_error: String
generators:
  fp: { gen: string }
  rs: { gen: string }
  off: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: llvm-mc too-few / invalid operand; encoder/mod.rs:356 get_reg integer registers only
```

## encode_branch_instr_symbol_offset_addend
- Tier: 4
- Rationale: Differential. llvm-mc accepts `beq rs1, rs2, foo+N` as a branch fixup with value foo+N. Operand::SymbolOffset is the parser's representation of symbol+addend (parser.rs:29). Reloc.addend must preserve N; word equals zero-offset encoding.
- Doc contract: parser.rs:29 "Symbol with addend: symbol+offset or symbol-offset" — asserted fingerprint 24ca6602
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs:474 encode_jal_symbol_offset_addend
- Formal: ∀ rs1,rs2 ∈ GPR, s ∈ ident, addend ∈ ℤ\{0}. encode_branch_instr([Reg(rs1),Reg(rs2),SymbolOffset(s,addend)], 0) = WordWithReloc{word=llvm-mc("beq rs1, rs2, 0"), reloc_type=Branch, symbol=s, addend=addend}
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: failing
- Counterexample: encode_branch_instr([Reg("x0"), Reg("x0"), SymbolOffset("foo", 1)], 0) → Err("branch: expected offset or label as 3rd operand")
- Bug report: bug_reports/encode_branch_instr_symbol_offset.md

```property
function: encoder.encode_branch_instr
oracle: differential
predicate:
  quantifier: forall
  vars: [rs1, rs2, s, addend]
  domain: { rs1: gpr, rs2: gpr, s: ident, addend: i64\{0} }
  relation:
    op: eq
    lhs: encode_branch_instr([Reg(rs1),Reg(rs2),SymbolOffset(s,addend)], 0)
    rhs: WordWithReloc{word: llvm_mc("beq rs1, rs2, 0"), reloc_type: Branch, symbol: s, addend: addend}
generators:
  rs1: { gen: string }
  rs2: { gen: string }
  s: { gen: string }
  addend: { gen: int, type: i64 }
evidence: parser.rs:29 SymbolOffset; llvm-mc fixup value foo+4; Reloc.addend i64
```

## encode_branch_instr_neg_bad_3rd
- Tier: 4
- Rationale: Sweep — documented error path for a 3rd operand that is not offset or label (Mem/Csr/Fence/RoundingMode). SymbolOffset is a separate failing reloc contract. Stronger oracles do not apply to this rejection path.
- Doc contract: base.rs:143 "branch: expected offset or label as 3rd operand" — asserted fingerprint b57fa706
- Seed: (none) — coverage_gaps file-level; manual arm audit of `_` match
- Formal: ∀ rs1,rs2 ∈ GPR, ∀ third ∈ {Mem, Csr, FenceArg, RoundingMode}. encode_branch_instr([Reg(rs1),Reg(rs2), third], 0) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_branch_instr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_branch_instr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs1, rs2, third]
  domain: { rs1: gpr, rs2: gpr, third: Mem|Csr|FenceArg|RoundingMode }
  relation:
    op: holds
    lhs: encode_branch_instr([Reg(rs1),Reg(rs2),third], 0).is_err()
expected_error: String
generators:
  rs1: { gen: string }
  rs2: { gen: string }
  third: { gen: string }
evidence: base.rs:143 branch: expected offset or label as 3rd operand
```
