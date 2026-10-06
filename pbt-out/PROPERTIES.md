# Properties: encode_jal

## encode_jal_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree JAL decoder). encode_j / encode_j_pseudo / encode_jalr / C.J rejected as same-job siblings (private packer / jal x0 pseudo / I-type / compressed).
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:52 "jal rd, offset  OR  jal offset (rd = ra)" — asserted fingerprint 8bd543de
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:402 encode_auipc_diff_imm_llvm_mc
- Formal: ∀ rd ∈ GPR, ∀ off ∈ {k·2 | k ∈ ℤ, off ∈ [-1048576, 1048574]}. encode_jal([Reg(rd), Imm(off)]) = Word(llvm-mc("jal rd, off"))
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, off]
  domain: { rd: gpr, off: even_jal_imm }
  relation:
    op: eq
    lhs: encode_jal([Reg(rd), Imm(off)])
    rhs: Word(llvm_mc("jal rd, off"))
generators:
  rd: { gen: string }
  off: { gen: int, min: -1048576, max: 1048574, type: i64 }
evidence: src/backend/riscv/assembler/README.md:357 J-type layout; llvm-mc -triple=riscv64
```

## encode_jal_one_operand_is_ra
- Tier: 4
- Rationale: Metamorphic restatement of the documented 1-operand form (implicit rd = ra). Also checked differentially vs llvm-mc `jal off`.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:52 "jal rd, offset  OR  jal offset (rd = ra)" — asserted fingerprint 8bd543de
- Seed: src/backend/riscv/assembler/encoder/base.rs:52-57
- Formal: ∀ off ∈ even_jal_imm. encode_jal([Imm(off)]) = encode_jal([Reg("ra"), Imm(off)]) = Word(llvm-mc("jal off"))
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [off]
  domain: { off: even_jal_imm }
  relation:
    op: eq
    lhs: encode_jal([Imm(off)])
    rhs: encode_jal([Reg("ra"), Imm(off)])
generators:
  off: { gen: int, min: -1048576, max: 1048574, type: i64 }
evidence: src/backend/riscv/assembler/encoder/base.rs:52
```

## encode_jal_isa_j_type
- Tier: 4
- Rationale: Algebraic invariant from the RISC-V J-type layout (README.md:357 / encoder/mod.rs:313). Unpack is independent of encode_j (ISA field extraction, not a copy of the packer).
- Doc contract: src/backend/riscv/assembler/README.md:357 "J-type:  [imm[20|10:1|11|19:12]         |  rd  | opcode]" — asserted fingerprint 2ea465b7
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:412 encode_auipc_isa_u_type
- Formal: ∀ rd ∈ [0,31], ∀ off ∈ even_jal_imm. let w = encode_jal([Reg(xN), Imm(off)]) in unpack_j(w) = (OP_JAL=0b1101111, rd, off)
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, off]
  domain: { rd: u32 0..=31, off: even_jal_imm }
  relation:
    op: eq
    lhs: unpack_j(encode_jal([Reg(x{rd}), Imm(off)]))
    rhs: (0b1101111, rd, off)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -1048576, max: 1048574, type: i64 }
evidence: src/backend/riscv/assembler/README.md:357
```

## encode_jal_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names (ra/sp/a0/…/fp) and xN encode the same rd field.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:422 encode_auipc_abi_xn_alias
- Formal: ∀ n ∈ [0,31], ∀ off ∈ even_jal_imm. encode_jal([Reg(xN), Imm(off)]) = encode_jal([Reg(ABI[n]), Imm(off)]) ∧ (n=8 ⇒ also equals encode_jal([Reg("fp"), Imm(off)]))
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, off]
  domain: { n: u32 0..=31, off: even_jal_imm }
  relation:
    op: eq
    lhs: encode_jal([Reg(x{n}), Imm(off)])
    rhs: encode_jal([Reg(ABI[n]), Imm(off)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -1048576, max: 1048574, type: i64 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:150 reg_num ABI aliases
```

## encode_jal_reloc_symbol
- Tier: 2
- Rationale: Differential on the reloc-form word (imm=0) plus algebraic invariant on RelocType::Jal / symbol / addend=0.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:73 "R_RISCV_JAL - 20-bit PC-relative jump (J-type)" — asserted fingerprint a9a52d0e
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:430 encode_auipc_reloc_pcrel_hi
- Formal: ∀ rd ∈ GPR, ∀ s ∈ ident. encode_jal([Reg(rd), Symbol(s)]) = WordWithReloc { word: llvm-mc("jal rd, 0"), reloc: {Jal, s, addend=0} } and the same for Label(s)
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, s]
  domain: { rd: gpr, s: ident }
  relation:
    op: eq
    lhs: encode_jal([Reg(rd), Symbol(s)])
    rhs: WordWithReloc(llvm_mc("jal rd, 0"), RelocType::Jal, s, 0)
generators:
  rd: { gen: string }
  s: { gen: string }
evidence: src/backend/riscv/assembler/encoder/mod.rs:73
```

## encode_jal_neg_imm_oob_odd
- Tier: 5
- Rationale: Negative/error contract from llvm-mc (and RISC-V J-type): immediate must be a multiple of 2 in [-1048576, 1048574]. Documented bounds sampled at bound±1 and odd values.
- Doc contract: src/backend/riscv/assembler/README.md:384 "   - **R_RISCV_JAL** (J-type): 20-bit signed offset, bit-scattered" — asserted fingerprint fe4276fb
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:450 encode_auipc_neg_imm_oob
- Formal: ∀ rd ∈ GPR, ∀ imm ∈ i64. (imm odd ∨ imm < -1048576 ∨ imm > 1048574) ∧ llvm-mc rejects "jal rd, imm" ⇒ encode_jal([Reg(rd), Imm(imm)]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: failing
- Counterexample: encode_jal([Reg("x0"), Imm(1)])
- Bug report: bug_reports/encode_jal_imm_oob_odd.md

```property
function: encoder.encode_jal
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr, imm: oob_or_odd_jal_imm }
  relation:
    op: throws
    expr: encode_jal([Reg(rd), Imm(imm)])
expected_error: String
generators:
  rd: { gen: string }
  imm: { gen: int, type: i64 }
evidence: llvm-mc "immediate must be a multiple of 2 bytes in the range [-1048576, 1048574]"; README.md:357
```

## encode_jal_neg_extra
- Tier: 5
- Rationale: JAL is one- or two-operand (base.rs:52). llvm-mc rejects a third operand. Extra operands must Err, not be silently ignored (else branch does not check len==2).
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:52 "jal rd, offset  OR  jal offset (rd = ra)" — asserted fingerprint 8bd543de
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:460 encode_auipc_neg_extra
- Formal: ∀ rd ∈ GPR, ∀ off ∈ even_jal_imm, ∀ extra. encode_jal([Reg(rd), Imm(off), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: failing
- Counterexample: encode_jal([Reg("x0"), Imm(0), Imm(0)])
- Bug report: bug_reports/encode_jal_extra_operand.md

```property
function: encoder.encode_jal
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, off, extra]
  domain: { rd: gpr, off: even_jal_imm, extra: Operand }
  relation:
    op: throws
    expr: encode_jal([Reg(rd), Imm(off), extra])
expected_error: String
generators:
  rd: { gen: string }
  off: { gen: int, min: -1048576, max: 1048574, type: i64 }
  extra: { gen: string }
evidence: src/backend/riscv/assembler/encoder/base.rs:52; llvm-mc extra-operand error
```

## encode_jal_symbol_offset_addend
- Tier: 2
- Rationale: Parser emits Operand::SymbolOffset for `sym+N` (parser.rs:908). llvm-mc accepts `jal rd, foo+4` with fixup value foo+4. Relocation.addend is documented (README.md:204). encode_jal's match omits SymbolOffset.
- Doc contract: src/backend/riscv/assembler/README.md:204 "addend:     i64,           // constant addend" — asserted fingerprint afcb725f
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs:540 encode_auipc_pcrel_hi_addend
- Formal: ∀ rd ∈ GPR, ∀ s ∈ ident, ∀ a ∈ i64\{0}. encode_jal([Reg(rd), SymbolOffset(s,a)]) = WordWithReloc { word: llvm-mc("jal rd, 0"), reloc: {Jal, s, addend=a} }
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: failing
- Counterexample: encode_jal([Reg("x0"), SymbolOffset("foo", 1)])
- Bug report: bug_reports/encode_jal_symbol_offset.md

```property
function: encoder.encode_jal
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, s, a]
  domain: { rd: gpr, s: ident, a: nonzero i64 }
  relation:
    op: eq
    lhs: encode_jal([Reg(rd), SymbolOffset(s, a)])
    rhs: WordWithReloc(llvm_mc("jal rd, 0"), RelocType::Jal, s, a)
generators:
  rd: { gen: string }
  s: { gen: string }
  a: { gen: int, type: i64 }
evidence: src/backend/riscv/assembler/README.md:204 Relocation.addend; parser.rs:908 SymbolOffset; llvm-mc jal rd, foo+4
```

## encode_jal_neg_fp
- Tier: 5
- Rationale: FP dest is not a GPR; get_reg returns Err. llvm-mc also rejects `jal fa0, 4`.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:52 "jal rd, offset  OR  jal offset (rd = ra)" — asserted fingerprint 8bd543de
- Seed: src/backend/riscv/assembler/encoder/encode_auipc_pbt.rs encode_auipc_neg_fp
- Formal: ∀ fp ∈ FP-regs, ∀ off ∈ even_jal_imm. encode_jal([Reg(fp), Imm(off)]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp, off]
  domain: { fp: fp_name, off: even_jal_imm }
  relation:
    op: throws
    expr: encode_jal([Reg(fp), Imm(off)])
expected_error: String
generators:
  fp: { gen: string }
  off: { gen: int, min: -1048576, max: 1048574, type: i64 }
evidence: get_reg integer-register check; llvm-mc rejects FP dest
```

## encode_jal_neg_empty
- Tier: 5
- Rationale: Zero operands is neither `jal offset` nor `jal rd, offset`. llvm-mc: too few operands.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:52 "jal rd, offset  OR  jal offset (rd = ra)" — asserted fingerprint 8bd543de
- Seed: encode_auipc_neg_arity
- Formal: encode_jal([]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: negative_error
predicate:
  quantifier: forall
  vars: []
  domain: {}
  relation:
    op: throws
    expr: encode_jal([])
expected_error: String
generators:
  unit: { gen: const, value: 0 }
evidence: src/backend/riscv/assembler/encoder/base.rs:52; llvm-mc too few operands
```

## encode_jal_one_operand_reloc
- Tier: 2
- Rationale: Sweep — 1-operand Symbol/Label arm (base.rs:59, implicit rd=ra) was not driven by the 2-operand reloc property. Documented form `jal offset` with a symbol.
- Doc contract: src/backend/riscv/assembler/encoder/base.rs:52 "jal rd, offset  OR  jal offset (rd = ra)" — asserted fingerprint 8bd543de
- Seed: encode_jal_reloc_symbol
- Formal: ∀ s ∈ ident. encode_jal([Symbol(s)]) = WordWithReloc { word: llvm-mc("jal ra, 0"), reloc: {Jal, s, addend=0} } and the same for Label(s)
- Test file: src/backend/riscv/assembler/encoder/encode_jal_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_jal
oracle: differential
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: ident }
  relation:
    op: eq
    lhs: encode_jal([Symbol(s)])
    rhs: WordWithReloc(llvm_mc("jal ra, 0"), RelocType::Jal, s, 0)
generators:
  s: { gen: string }
evidence: src/backend/riscv/assembler/encoder/base.rs:52-66
```
