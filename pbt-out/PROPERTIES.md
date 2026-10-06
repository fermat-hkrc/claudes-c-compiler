# Properties: encode_jalr

## encode_jalr_diff_3op_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree JALR decoder). encode_i / encode_c_jalr / jr / ret / encode_jal rejected as same-job siblings (private packer / compressed / jalr x0 pseudo / J-type).
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:93 "jalr rd, rs1, offset  OR  jalr rd, offset(rs1)  OR  jalr rs1" — asserted fingerprint f3a7dd4e
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs encode_jal_diff_imm_llvm_mc
- Formal: ∀ rd, rs1 ∈ GPR, ∀ off ∈ [-2048, 2047]. encode_jalr([Reg(rd), Reg(rs1), Imm(off)]) = Word(llvm-mc("jalr rd, rs1, off"))
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jalr
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, off]
  domain: { rd: gpr, rs1: gpr, off: i12 }
  relation:
    op: eq
    lhs: encode_jalr([Reg(rd), Reg(rs1), Imm(off)])
    rhs: Word(llvm_mc("jalr rd, rs1, off"))
generators:
  rd: { gen: string }
  rs1: { gen: string }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/README.md:353 I-type layout; llvm-mc -triple=riscv64
```

## encode_jalr_isa_i_type
- Tier: 4
- Rationale: Algebraic invariant from the RISC-V I-type layout (README.md:353 / encoder/mod.rs:285). Unpack is independent of encode_i (ISA field extraction, not a copy of the packer).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:285 "I-type: imm[31:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 274cd4b0
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs encode_jal_isa_j_type
- Formal: ∀ rd, rs1 ∈ [0,31], ∀ off ∈ [-2048, 2047]. let w = encode_jalr([Reg(x{rd}), Reg(x{rs1}), Imm(off)]) in unpack_i(w) = (OP_JALR=0b1100111, rd, funct3=0, rs1, off)
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jalr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, off]
  domain: { rd: u32 0..=31, rs1: u32 0..=31, off: i12 }
  relation:
    op: eq
    lhs: unpack_i(encode_jalr([Reg(x{rd}), Reg(x{rs1}), Imm(off)]))
    rhs: (0b1100111, rd, 0, rs1, off)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:285
```

## encode_jalr_one_operand_is_ra
- Tier: 4
- Rationale: Metamorphic restatement of the documented 1-operand form (implicit rd = ra, offset = 0). Also checked differentially vs llvm-mc `jalr rs1`.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:96 "jalr rs1 (rd = ra, offset = 0)" — asserted fingerprint 7eebd324
- Seed: src/backend/riscv/assembler/encoder/base.rs:96-98
- Formal: ∀ rs1 ∈ GPR. encode_jalr([Reg(rs1)]) = encode_jalr([Reg("ra"), Reg(rs1), Imm(0)]) = Word(llvm-mc("jalr rs1"))
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jalr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs1]
  domain: { rs1: gpr }
  relation:
    op: eq
    lhs: encode_jalr([Reg(rs1)])
    rhs: encode_jalr([Reg("ra"), Reg(rs1), Imm(0)])
generators:
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/base.rs:96
```

## encode_jalr_two_op_reg_eq_zero_imm
- Tier: 4
- Rationale: Metamorphic restatement of the documented 2-operand register form (implicit offset = 0).
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:101 "jalr rd, rs1  (offset = 0)" — asserted fingerprint 4f4624bd
- Seed: src/backend/riscv/assembler/encoder/base.rs:101-106
- Formal: ∀ rd, rs1 ∈ GPR. encode_jalr([Reg(rd), Reg(rs1)]) = encode_jalr([Reg(rd), Reg(rs1), Imm(0)]) = Word(llvm-mc("jalr rd, rs1"))
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jalr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1]
  domain: { rd: gpr, rs1: gpr }
  relation:
    op: eq
    lhs: encode_jalr([Reg(rd), Reg(rs1)])
    rhs: encode_jalr([Reg(rd), Reg(rs1), Imm(0)])
generators:
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/base.rs:101
```

## encode_jalr_two_op_mem_eq_3op
- Tier: 4
- Rationale: Metamorphic: the documented `jalr rd, offset(rs1)` form must encode the same I-type word as `jalr rd, rs1, offset`. Also checked differentially vs llvm-mc `jalr rd, off(rs1)`.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:93 "jalr rd, rs1, offset  OR  jalr rd, offset(rs1)  OR  jalr rs1" — asserted fingerprint f3a7dd4e
- Seed: src/backend/riscv/assembler/encoder/base.rs:107-110
- Formal: ∀ rd, rs1 ∈ GPR, ∀ off ∈ [-2048, 2047]. encode_jalr([Reg(rd), Mem{base: rs1, offset: off}]) = encode_jalr([Reg(rd), Reg(rs1), Imm(off)]) = Word(llvm-mc("jalr rd, off(rs1)"))
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jalr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1, off]
  domain: { rd: gpr, rs1: gpr, off: i12 }
  body: encode_jalr([Reg(rd), Mem(rs1, off)]) == encode_jalr([Reg(rd), Reg(rs1), Imm(off)])
generators:
  rd: { gen: string }
  rs1: { gen: string }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/base.rs:93
```

## encode_jalr_abi_xn_alias
- Tier: 4
- Rationale: ABI names, xN, and fp=s0/x8 must encode the same rd/rs1. Metamorphic alias invariance of reg_num.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs encode_jal_abi_xn_alias
- Formal: ∀ n, m ∈ [0,31], ∀ off ∈ [-2048, 2047]. encode_jalr([Reg(x{n}), Reg(x{m}), Imm(off)]) = encode_jalr([Reg(ABI[n]), Reg(ABI[m]), Imm(off)]); when n=8, Reg("fp") agrees with x8
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jalr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, off]
  domain: { n: u32 0..=31, m: u32 0..=31, off: i12 }
  relation:
    op: eq
    lhs: encode_jalr([Reg(x{n}), Reg(x{m}), Imm(off)])
    rhs: encode_jalr([Reg(ABI[n]), Reg(ABI[m]), Imm(off)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/parser.rs:22
```

## encode_jalr_neg_imm_oob
- Tier: 5
- Rationale: llvm-mc rejects JALR immediates outside signed 12-bit [-2048, 2047]. README.md:353 I-type imm[11:0] is a 12-bit field. encode_i masks with 0xFFF so out-of-range values currently wrap; the documented assembler contract is rejection (same as llvm-mc).
- Doc contract: src/backend/riscv/assembler/README.md:353 "I-type:  [    imm[11:0]  | rs1 | funct3 |  rd  | opcode]" — asserted fingerprint a59ffa62
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs encode_jal_neg_imm_oob_odd
- Formal: ∀ rd, rs1 ∈ GPR, ∀ off ∈ ℤ \ [-2048, 2047]. llvm-mc("jalr rd, rs1, off") errors ∧ encode_jalr([Reg(rd), Reg(rs1), Imm(off)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: failing
- Counterexample: rd = "x0", rs1 = "x1", imm = 2048 → Ok(Word(0x80008067)) (2048 wrapped to -2048)
- Bug report: pbt-out/bug_reports/encode_jalr_imm_oob.md

```property
function: encoder.encode_jalr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, off]
  domain: { rd: gpr, rs1: gpr, off: i64_outside_i12 }
  relation:
    op: throws
    expr: encode_jalr([Reg(rd), Reg(rs1), Imm(off)])
expected_error: String
generators:
  rd: { gen: string }
  rs1: { gen: string }
  off: { gen: int, type: i64 }
evidence: llvm-mc operand must be an integer in the range [-2048, 2047]; README.md:353 imm[11:0]
```

## encode_jalr_neg_arity_fp
- Tier: 5
- Rationale: Empty list and arity > 3 must Err (base.rs:121). FP dest/src is not a GPR (get_reg rejects). Extra operand llvm-mc rejects. Combined negative-error covering documented error strings.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:121 "jalr: wrong number of operands" — asserted fingerprint 80c613db
- Seed: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs encode_jal_neg_extra / encode_jal_neg_fp / encode_jal_neg_empty
- Formal: encode_jalr([]) = Err(_) ∧ ∀ rd, rs1 ∈ GPR, ∀ off ∈ [-2048, 2047], ∀ extra. encode_jalr([Reg(rd), Reg(rs1), Imm(off), extra]) = Err(_) ∧ ∀ fp ∈ FPR, ∀ gpr ∈ GPR, ∀ off ∈ [-2048, 2047]. encode_jalr([Reg(fp), Reg(gpr), Imm(off)]) = Err(_) ∧ encode_jalr([Reg(gpr), Reg(fp), Imm(off)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jalr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, off, extra, fp]
  domain: { rd: gpr, rs1: gpr, off: i12, extra: operand, fp: fpr }
  relation:
    op: throws
    expr: encode_jalr([])
expected_error: String
generators:
  rd: { gen: string }
  rs1: { gen: string }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
  extra: { gen: string }
  fp: { gen: string }
evidence: src/backend/riscv/assembler/encoder/base.rs:121
```

## encode_jalr_one_operand_mem
- Tier: 4
- Rationale: Metamorphic + differential. llvm-mc accepts `jalr off(rs1)` as jalr ra, off(rs1). Analogous to documented 1-operand `jalr rs1` (implicit rd=ra, offset=0) applied to the documented mem form. Strengthening round after first batch.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:93 "jalr rd, rs1, offset  OR  jalr rd, offset(rs1)  OR  jalr rs1" — asserted fingerprint f3a7dd4e
- Seed: src/backend/riscv/assembler/encoder/base.rs:93
- Formal: ∀ rs1 ∈ GPR, ∀ off ∈ [-2048, 2047]. encode_jalr([Mem{base: rs1, offset: off}]) = encode_jalr([Reg("ra"), Reg(rs1), Imm(off)]) = Word(llvm-mc("jalr off(rs1)"))
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: failing
- Counterexample: rs1 = "x2", off = 8 → Err("expected register at operand 0, got Some(Mem { base: \"x2\", offset: 8 })")
- Bug report: pbt-out/bug_reports/encode_jalr_one_operand_mem.md

```property
function: encoder.encode_jalr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs1, off]
  domain: { rs1: gpr, off: i12 }
  relation:
    op: eq
    lhs: encode_jalr([Mem(rs1, off)])
    rhs: encode_jalr([Reg("ra"), Reg(rs1), Imm(off)])
generators:
  rs1: { gen: string }
  off: { gen: int, min: -2048, max: 2047, type: i64 }
evidence: src/backend/riscv/assembler/encoder/base.rs:93; llvm-mc jalr off(rs1)
```

## encode_jalr_reloc_lo
- Tier: 2
- Rationale: Differential vs the assembler's own reloc contract. README.md:334 `call` expands to `jalr ra, %pcrel_lo(sym)(ra)`; README.md:25 lists %pcrel_lo/%lo parsing; encode_load/encode_alu_imm handle MemSymbol for I-type. encode_jalr is the I-type JALR encoder and must emit WordWithReloc (PcrelLo12I / Lo12I) with word equal to jalr rd, 0(rs1).
- Doc contract: src/backend/riscv/assembler/README.md:334 "| `call sym`     | `auipc ra, %pcrel_hi(sym)` + `jalr ra, %pcrel_lo(sym)(ra)` |" — asserted fingerprint 0b68bdd3
- Seed: src/backend/riscv/assembler/encoder/encode_load MemSymbol arm
- Formal: ∀ rd, rs1 ∈ GPR, ∀ s ∈ ident. encode_jalr([Reg(rd), MemSymbol{base: rs1, symbol: "%pcrel_lo(s)"}]) = WordWithReloc{word: encode_jalr([Reg(rd), Mem{rs1,0}]), type: PcrelLo12I, symbol: s, addend: 0} and likewise %lo → Lo12I
- Test file: src/backend/riscv/assembler/encoder/encode_jalr_pbt.rs
- Status: failing
- Counterexample: rd = "ra", rs1 = "ra", s = "foo" → Err("jalr: invalid operands")
- Bug report: pbt-out/bug_reports/encode_jalr_pcrel_lo.md

```property
function: encoder.encode_jalr
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, s]
  domain: { rd: gpr, rs1: gpr, s: ident }
  relation:
    op: eq
    lhs: encode_jalr([Reg(rd), MemSymbol("%pcrel_lo(s)", rs1)]).word
    rhs: encode_jalr([Reg(rd), Mem(rs1, 0)])
generators:
  rd: { gen: string }
  rs1: { gen: string }
  s: { gen: string }
evidence: src/backend/riscv/assembler/README.md:334
```
