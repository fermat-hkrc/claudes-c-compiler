# Properties: encode_sfence_vma

## encode_sfence_vma_diff_llvm_mc
- Tier: 3
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (same job: assemble `sfence.vma` textual source into a 32-bit word). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SFENCE.VMA decoder). encode_r / encode_fence / encode_csr rejected as primary differential (same-job gate: private packer / FENCE I-type / CSR I-type). SUT-boundary: internal-helper, operands passed through from encode_instruction (encoder/mod.rs:699).
- Doc contract: src/backend/riscv/assembler/encoder/system.rs:25 "Encode sfence.vma rs1, rs2" — asserted fingerprint d9dbbac7
- Seed: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:288 encode_sc_diff_llvm_mc
- Formal: ∀ arity ∈ {0,1,2}, rs1, rs2 ∈ GPRNames. encode_sfence_vma(ops(arity, rs1, rs2)) = llvm-mc("sfence.vma" + ops_asm(arity, rs1, rs2)) as little-endian u32
- Test file: src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sfence_vma
oracle: differential
predicate:
  quantifier: forall
  vars: [arity, rs1, rs2]
  domain: { arity: {0,1,2}, rs1: GPRNames, rs2: GPRNames }
  relation:
    op: eq
    lhs: encode_sfence_vma(ops(arity, rs1, rs2))
    rhs: llvm_mc_word(sfence_asm(arity, rs1, rs2))
generators:
  arity: { gen: int, min: 0, max: 2, type: u8 }
  rs1: { gen: string }
  rs2: { gen: string }
evidence: src/backend/riscv/assembler/encoder/system.rs:25-29; encoder/mod.rs:699; llvm-mc -triple=riscv64 -show-encoding
```

## encode_sfence_vma_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the function's own format comment and RISC-V R-type layout. Weaker than differential; kept as an independent ISA unpack that does not copy encode_r. Stronger rejected as above.
- Doc contract: src/backend/riscv/assembler/encoder/system.rs:26 "Format: funct7=0001001 | rs2 | rs1 | funct3=000 | rd=00000 | opcode=1110011" — asserted fingerprint 97f722f3
- Seed: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:302 encode_sc_r_type_fields
- Formal: ∀ rs1, rs2 ∈ {0..31}. let w = encode_sfence_vma([x(rs1), x(rs2)]). opcode(w)=0b1110011 ∧ rd(w)=0 ∧ funct3(w)=0 ∧ rs1(w)=rs1 ∧ rs2(w)=rs2 ∧ funct7(w)=0b0001001
- Test file: src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sfence_vma
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs1, rs2]
  domain: { rs1: 0..31, rs2: 0..31 }
  body: unpack_r(encode_sfence_vma([Reg(xN(rs1)), Reg(xN(rs2))])) == (opcode=0b1110011, rd=0, funct3=0, rs1, rs2, funct7=0b0001001)
generators:
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/system.rs:26; encoder/mod.rs:307
```

## encode_sfence_vma_defaults
- Tier: 4
- Rationale: Algebraic metamorphic from the function's own rustdoc: empty encodes as zero,zero; one operand encodes as rs1,zero. Independent of llvm-mc.
- Doc contract: src/backend/riscv/assembler/encoder/system.rs:27 "If no operands: sfence.vma zero, zero" — asserted fingerprint ce6669b1
- Seed: src/backend/riscv/assembler/encoder/encode_fence_pbt.rs:375 encode_fence_empty_is_iorw
- Formal: ∀ rs1 ∈ GPRNames. encode_sfence_vma([]) = encode_sfence_vma([zero, zero]) = encode_sfence_vma([x0, x0]) ∧ encode_sfence_vma([rs1]) = encode_sfence_vma([rs1, zero]) = encode_sfence_vma([rs1, x0])
- Test file: src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sfence_vma
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs1]
  domain: { rs1: GPRNames }
  body: encode_sfence_vma([]) == encode_sfence_vma([zero,zero]) == encode_sfence_vma([x0,x0]) AND encode_sfence_vma([rs1]) == encode_sfence_vma([rs1, zero]) == encode_sfence_vma([rs1, x0])
generators:
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/system.rs:27-28
```

## encode_sfence_vma_abi_xn_alias
- Tier: 4
- Rationale: Algebraic metamorphic: ABI names, xN, and fp=s0/x8 encode the same rs1/rs2. RISC-V register aliases are a documented assembler contract (parser.rs:22).
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:323 encode_sc_abi_xn_alias
- Formal: ∀ n, m ∈ {0..31}. encode_sfence_vma([xN(n), xN(m)]) = encode_sfence_vma([ABI(n), ABI(m)]) ∧ (n=8 ⇒ encode_sfence_vma([fp, xN(m)]) equals that word) ∧ (m=8 ⇒ encode_sfence_vma([xN(n), fp]) equals that word)
- Test file: src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sfence_vma
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: 0..31, m: 0..31 }
  relation:
    op: eq
    lhs: encode_sfence_vma([Reg(xN(n)), Reg(xN(m))])
    rhs: encode_sfence_vma([Reg(ABI(n)), Reg(ABI(m))])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/parser.rs:22
```

## encode_sfence_vma_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a third operand (`invalid operand for instruction`). encode_instruction passes operands through (mod.rs:699). The function's rustdoc enumerates only 0/1/2 operands. Extra operands must Err, not be silently ignored.
- Doc contract: src/backend/riscv/assembler/encoder/system.rs:29 "If 2 operands: sfence.vma rs1, rs2" — asserted fingerprint 596b276c
- Seed: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:353 encode_sc_neg_extra
- Formal: ∀ rs1, rs2 ∈ GPRNames, extra ∈ Operand. encode_sfence_vma([rs1, rs2, extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
- Status: failing
- Counterexample: encode_sfence_vma([Reg("x0"), Reg("x0"), Imm(0)]) -> Ok(Word(0x12000073))
- Bug report: bug_reports/encode_sfence_vma_extra_operand.md

```property
function: encoder.encode_sfence_vma
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs1, rs2, extra]
  domain: { rs1: GPRNames, rs2: GPRNames, extra: Operand }
  relation:
    op: throws
    expr: encode_sfence_vma([Reg(rs1), Reg(rs2), extra])
generators:
  rs1: { gen: string }
  rs2: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: llvm-mc rejects sfence.vma x1, x2, x3; system.rs:25-29 enumerates only 0/1/2 operands
```

## encode_sfence_vma_neg_fp
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects FP registers (`sfence.vma fa0, a1` / `sfence.vma a0, ft0` → invalid operand). SFENCE.VMA rs1/rs2 are integer GPRs (vaddr/ASID).
- Doc contract: src/backend/riscv/assembler/encoder/system.rs:25 "Encode sfence.vma rs1, rs2" — asserted fingerprint d9dbbac7
- Seed: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:389 encode_sc_neg_arity_fp
- Formal: ∀ gpr ∈ GPRNames, fp ∈ FPNames. encode_sfence_vma([fp]) is Err ∧ encode_sfence_vma([fp, gpr]) is Err ∧ encode_sfence_vma([gpr, fp]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sfence_vma
oracle: negative_error
predicate:
  quantifier: forall
  vars: [gpr, fp]
  domain: { gpr: GPRNames, fp: FPNames }
  relation:
    op: holds
    expr: encode_sfence_vma([Reg(fp)]).is_err() && encode_sfence_vma([Reg(fp), Reg(gpr)]).is_err() && encode_sfence_vma([Reg(gpr), Reg(fp)]).is_err()
generators:
  gpr: { gen: string }
  fp: { gen: string }
expected_error: String
evidence: llvm-mc sfence.vma fa0, a1 error invalid operand; RISC-V privileged ISA rs1/rs2 are GPRs
```

## encode_sfence_vma_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract. Non-Reg operand kinds (Symbol, Mem, Csr, FenceArg, Label, RoundingMode, Imm outside 0..=31, MemSymbol, SymbolOffset) are not GPR names. llvm-mc requires integer registers. Imm 0..=31 is get_reg's GCC inline-asm contract (encoder/mod.rs:376), not this function's domain — excluded from the invalid generator.
- Doc contract: src/backend/riscv/assembler/encoder/system.rs:25 "Encode sfence.vma rs1, rs2" — asserted fingerprint d9dbbac7
- Seed: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs:428 encode_sc_neg_non_mem
- Formal: ∀ gpr ∈ GPRNames, bad ∈ NonRegOperand. encode_sfence_vma([bad]) is Err ∧ encode_sfence_vma([gpr, bad]) is Err ∧ encode_sfence_vma([bad, gpr]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sfence_vma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sfence_vma
oracle: negative_error
predicate:
  quantifier: forall
  vars: [gpr, bad]
  domain: { gpr: GPRNames, bad: NonRegOperand }
  body: encode_sfence_vma([bad]).is_err() AND encode_sfence_vma([Reg(gpr), bad]).is_err() AND encode_sfence_vma([bad, Reg(gpr)]).is_err()
generators:
  gpr: { gen: string }
  bad: { gen: string }
expected_error: String
evidence: llvm-mc requires integer registers; get_reg (encoder/mod.rs:379) returns Err for non-Reg (except Imm 0..=31)
```
