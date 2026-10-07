# Properties: encode_bnez

## encode_bnez_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree BNEZ decoder). Shared-source encode_branch_instr used only as weaker metamorphic. Evidence: README.md:328 `beqz/bnez` → `beq/bne rs, x0, label`; RISC-V ISA BNEZ = BNE rs, x0.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_beqz_pbt.rs encode_beqz_diff_llvm_mc (sibling)
- Formal: ∀ rs ∈ GPRNames. word(encode_bnez([Reg(rs), Imm(0)])) = llvm_mc("bnez rs, 0") ∧ reloc = Branch/"0"/0
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: GPRNames }
  relation:
    op: eq
    lhs: word(encode_bnez([Reg(rs), Imm(0)]))
    rhs: llvm_mc("bnez rs, 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_bnez_diff_llvm_mc_bne
- Tier: 5
- Rationale: Documented expansion vs independent assembler `bne rs, x0, 0`. Strengthens differential beyond the mnemonic spelling.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_beqz_pbt.rs encode_beqz_diff_llvm_mc_beq
- Formal: ∀ rs ∈ GPRNames. word(encode_bnez([Reg(rs), Imm(0)])) = llvm_mc("bne rs, x0, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: GPRNames }
  relation:
    op: eq
    lhs: word(encode_bnez([Reg(rs), Imm(0)]))
    rhs: llvm_mc("bne rs, x0, 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_bnez_eq_bne_x0
- Tier: 4
- Rationale: Algebraic metamorphic — documented expansion to BNE rs, x0 via encode_branch_instr(funct3=001). Same-job sibling (bne dispatch).
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_beqz_pbt.rs encode_beqz_eq_beq_x0
- Formal: ∀ rs ∈ 0..31, tgt ∈ LabelIdents. encode_bnez([Reg(xN(rs)), Symbol(tgt)]) = encode_branch_instr([Reg(xN(rs)), Reg(x0), Symbol(tgt)], 0b001) = encode_branch_instr([Reg(xN(rs)), Reg(zero), Symbol(tgt)], 0b001)
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: 0..31, tgt: LabelIdents }
  relation:
    op: eq
    lhs: encode_bnez([Reg(xN(rs)), Symbol(tgt)])
    rhs: encode_branch_instr([Reg(xN(rs)), Reg("x0"), Symbol(tgt)], 0b001)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_bnez_isa_b_type
- Tier: 4
- Rationale: Algebraic invariant — B-type field layout per RISC-V unprivileged ISA; funct3 must be 001 (BNE), rs2=x0, imm deferred to reloc.
- Doc contract: (none on encode_bnez body) — ISA layout from encode_b comment chain / opcode constants in mod.rs
- Seed: encode_beqz_pbt.rs encode_beqz_isa_b_type
- Formal: ∀ rs ∈ 0..31, tgt ∈ LabelIdents. unpack_b(word(encode_bnez([Reg(xN(rs)), Symbol(tgt)]))) = (OP_BRANCH, 0b001, rs, 0, 0) ∧ reloc=(Branch, tgt, 0)
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: 0..31, tgt: LabelIdents }
  body: unpack_b(word(encode_bnez([Reg(xN(rs)), Symbol(tgt)]))) == (OP_BRANCH, 0b001, rs, 0, 0) && reloc == (Branch, tgt, 0)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:422
```

## encode_bnez_abi_xn_alias
- Tier: 4
- Rationale: Algebraic metamorphic — ABI name, xN, and fp (for n=8) must encode identically.
- Doc contract: (none on encode_bnez) — register alias table in parser/encoder
- Seed: encode_beqz_pbt.rs encode_beqz_abi_xn_alias
- Formal: ∀ n ∈ 0..31, tgt ∈ LabelIdents. encode_bnez([Reg(abi(n)), Symbol(tgt)]) = encode_bnez([Reg(xN(n)), Symbol(tgt)]); if n=8 also via "fp"
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: 0..31, tgt: LabelIdents }
  relation:
    op: eq
    lhs: encode_bnez([Reg(abi(n)), Symbol(tgt)])
    rhs: encode_bnez([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:475
```

## encode_bnez_target_forms
- Tier: 4
- Rationale: get_branch_target treats Symbol/Label/Reg-as-label equivalently (doc on get_branch_target).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:380 "A register name can also be a symbol/label name (e.g. `beqz a0, t1`" — asserted fingerprint bc6f2199
- Seed: encode_beqz_pbt.rs encode_beqz_target_forms
- Formal: ∀ rs ∈ 0..31, s ∈ LabelIdents. encode_bnez([Reg(xN(rs)), Symbol(s)]) = encode_bnez([Reg(xN(rs)), Label(s)]) = encode_bnez([Reg(xN(rs)), Reg(s)])
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, s]
  domain: { rs: 0..31, s: LabelIdents }
  body: encode_bnez([xN(rs), Symbol(s)]) == encode_bnez([xN(rs), Label(s)]) == encode_bnez([xN(rs), Reg(s)])
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:376
```

## encode_bnez_imm_target
- Tier: 4
- Rationale: Imm target stringified into reloc; word always zero-imm BNE (reloc carries target).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:379 "Some(Operand::Imm(v)) => Ok(format!("{}", v))," — asserted fingerprint ad06d32a
- Seed: encode_beqz_pbt.rs encode_beqz_imm_target
- Formal: ∀ rs ∈ 0..31, imm ∈ ImmTargets. unpack_b(word(encode_bnez([xN(rs), Imm(imm)]))) = (OP_BRANCH, 0b001, rs, 0, 0) ∧ reloc.symbol = format!(imm) ∧ addend=0
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, imm]
  domain: { rs: 0..31, imm: ImmTargets }
  body: unpack_b(word(...)) == (OP_BRANCH, 0b001, rs, 0, 0) && symbol == format(imm)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -64, max: 64, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:379
```


## encode_bnez_imm_as_rs
- Tier: 4
- Rationale: Algebraic metamorphic strengthen — get_reg accepts Imm(0..31) as bare register numbers (GCC inline asm path). Same encoding as xN.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:481 "GCC sometimes emits bare register numbers (0-31) in inline asm" — asserted fingerprint f1b1a1fb
- Seed: (none)
- Formal: ∀ n ∈ 0..31, tgt ∈ LabelIdents. encode_bnez([Imm(n), Symbol(tgt)]) = encode_bnez([Reg(xN(n)), Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: 0..31, tgt: LabelIdents }
  relation:
    op: eq
    lhs: encode_bnez([Imm(n), Symbol(tgt)])
    rhs: encode_bnez([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:480
```

## encode_bnez_neg_arity
- Tier: 3
- Rationale: Negative/error — two-operand form per README; arity < 2 must Err (get_reg/get_branch_target missing operand).
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_beqz_pbt.rs encode_beqz_neg_arity
- Formal: ∀ ops. len(ops) < 2 ⇒ encode_bnez(ops) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: ShortOps }
  relation:
    op: holds
    expr: encode_bnez(ops).is_err()
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_bnez_neg_invalid_rs
- Tier: 3
- Rationale: Negative/error — non-GPR first operand must Err via get_reg.
- Doc contract: (none on encode_bnez) — get_reg rejects non-integer registers
- Seed: encode_beqz_pbt.rs encode_beqz_neg_invalid_rs
- Formal: ∀ bad ∈ InvalidRs, tgt ∈ LabelIdents. encode_bnez([bad, Symbol(tgt)]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, tgt]
  domain: { bad: InvalidRs, tgt: LabelIdents }
  relation:
    op: holds
    expr: encode_bnez([bad, Symbol(tgt)]).is_err()
generators:
  bad: { gen: oneof }
  tgt: { gen: string, type: String }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:475
```

## encode_bnez_neg_invalid_target
- Tier: 3
- Rationale: Negative/error — Mem/Csr/Fence/etc targets rejected by get_branch_target.
- Doc contract: pseudo.rs:387 expected branch target error path
- Seed: encode_beqz_pbt.rs encode_beqz_neg_invalid_target
- Formal: ∀ rs ∈ GPRNames, bad ∈ InvalidTarget. encode_bnez([Reg(rs), bad]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bnez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, bad]
  domain: { rs: GPRNames, bad: InvalidTarget }
  relation:
    op: holds
    expr: encode_bnez([Reg(rs), bad]).is_err()
generators:
  rs: { gen: string, type: String }
  bad: { gen: oneof }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:387
```

## encode_bnez_neg_extra
- Tier: 3
- Rationale: Negative/error — README documents two-operand form; llvm-mc rejects a third operand. encode_bnez must Err on extra operands (same contract as sibling beqz).
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_beqz_pbt.rs encode_beqz_neg_extra / test_encode_beqz_regression_extra_operand
- Formal: ∀ rs ∈ GPRNames, tgt ∈ LabelIdents, extra ∈ Operand. encode_bnez([Reg(rs), Symbol(tgt), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bnez_pbt.rs
- Status: failing
- Counterexample: encode_bnez([Reg("a0"), Symbol("foo"), Reg("a1")]) → Ok (expected Err)
- Bug report: bug_reports/encode_bnez_extra_operand.md

```property
function: encode_bnez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, tgt, extra]
  domain: { rs: GPRNames, tgt: LabelIdents, extra: Operand }
  relation:
    op: holds
    expr: encode_bnez([Reg(rs), Symbol(tgt), extra]).is_err()
generators:
  rs: { gen: string, type: String }
  tgt: { gen: string, type: String }
  extra: { gen: oneof }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:328
```
