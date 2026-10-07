# Properties: encode_beqz

## encode_beqz_diff_llvm_mc
- Tier: 4
- Rationale: Strongest independent reference is llvm-mc (RISC-V assembler). State machine rejected (pure encoder). Round-trip rejected (no BEQZ decoder). Differential vs encode_branch_instr rejected as primary (shared encode_b/get_reg; used as metamorphic instead). Doc evidence: README.md:328 beqz → beq rs, x0, label; RISC-V ISA BEQZ = BEQ rs, x0, offset.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_branch_instr_pbt.rs KAT beq zero-imm form (none on encode_beqz itself)
- Formal: ∀ rs ∈ GPRNames. word(encode_beqz([Reg(rs), Imm(0)])) = llvm_mc("beqz rs, 0") ∧ reloc_type=Branch ∧ symbol="0" ∧ addend=0
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: word(encode_beqz([Reg(rs), Imm(0)]))
    rhs: llvm_mc("beqz " + rs + ", 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_beqz_diff_llvm_mc_beq
- Tier: 4
- Rationale: Documented expansion sweep against independent assembler (`beq rs, x0, 0`), not in-tree encode_branch_instr. Strengthen round after first green differential batch.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: (none)
- Formal: ∀ rs ∈ GPRNames. word(encode_beqz([Reg(rs), Imm(0)])) = llvm_mc("beq rs, x0, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: word(encode_beqz([Reg(rs), Imm(0)]))
    rhs: llvm_mc("beq " + rs + ", x0, 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_beqz_eq_beq_x0
- Tier: 3
- Rationale: Documented expansion beqz rs, label = beq rs, x0, label. Metamorphic vs same-job sibling encode_branch_instr(..., funct3=000) on Symbol targets.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: (none)
- Formal: ∀ rs ∈ 0..31, tgt ∈ LabelIdents. encode_beqz([Reg(xN(rs)), Symbol(tgt)]) = encode_branch_instr([Reg(xN(rs)), Reg(x0), Symbol(tgt)], 0b000) as WordWithReloc (word, Branch, tgt, 0)
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_beqz([Reg(xN(rs)), Symbol(tgt)])
    rhs: encode_branch_instr([Reg(xN(rs)), Reg("x0"), Symbol(tgt)], 0b000)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_beqz_isa_b_type
- Tier: 3
- Rationale: B-type layout from RISC-V ISA / encoder/mod.rs encode_b comment. beqz fixes funct3=000, rs2=x0, imm=0 (reloc deferred).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:420 "/// B-type: imm[12|10:5] | rs2 | rs1 | funct3 | imm[4:1|11] | opcode" — asserted fingerprint 8772767d; section comment pseudo.rs:281 "// Branch pseudo-instructions" — other fingerprint c33ca75f
- Seed: encode_branch_instr_pbt.rs encode_branch_instr_isa_b_type
- Formal: ∀ rs ∈ 0..31, tgt ∈ LabelIdents. let w = word(encode_beqz([Reg(xN(rs)), Symbol(tgt)])). unpack_b(w) = (OP_BRANCH, 0b000, rs, 0, 0) ∧ reloc=(Branch, tgt, 0)
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: reg_num, tgt: ident }
  body: unpack_b(word(encode_beqz([Reg(xN(rs)), Symbol(tgt)]))) == (OP_BRANCH, 0b000, rs, 0, 0)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:420
```

## encode_beqz_abi_xn_alias
- Tier: 3
- Rationale: ABI names and xN must encode identically; fp aliases s0/x8.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_sgtz_pbt.rs encode_sgtz_abi_xn_alias
- Formal: ∀ n ∈ 0..31, tgt ∈ LabelIdents. encode_beqz([Reg(abi(n)), Symbol(tgt)]).word = encode_beqz([Reg(xN(n)), Symbol(tgt)]).word
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: word(encode_beqz([Reg(abi(n)), Symbol(tgt)]))
    rhs: word(encode_beqz([Reg(xN(n)), Symbol(tgt)]))
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/parser.rs:21
```

## encode_beqz_target_forms
- Tier: 3
- Rationale: get_branch_target accepts Symbol | Label | Reg | Imm; Symbol/Label/Reg with same string must yield identical reloc. Imm stringifies (separate property).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:281 "// Branch pseudo-instructions" — other fingerprint c33ca75f
- Seed: encode_branch_instr_pbt reloc Symbol/Label/Reg
- Formal: ∀ rs ∈ 0..31, s ∈ LabelIdents. encode_beqz([Reg(xN(rs)), Symbol(s)]) = encode_beqz([Reg(xN(rs)), Label(s)]) = encode_beqz([Reg(xN(rs)), Reg(s)]) as WordWithReloc
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, s]
  domain: { rs: reg_num, s: ident }
  relation:
    op: eq
    lhs: encode_beqz([Reg(xN(rs)), Symbol(s)])
    rhs: encode_beqz([Reg(xN(rs)), Label(s)])
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:376
```

## encode_beqz_imm_target
- Tier: 3
- Rationale: Strengthen round — Imm branch target is accepted by get_branch_target via format!("{}", v); word stays zero-imm BEQ with reloc symbol = decimal string.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:281 "// Branch pseudo-instructions" — other fingerprint c33ca75f
- Seed: (none)
- Formal: ∀ rs ∈ 0..31, imm ∈ ℤ. encode_beqz([Reg(xN(rs)), Imm(imm)]) = WordWithReloc(BEQ rs,x0,0, Branch, format(imm), 0)
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, imm]
  domain: { rs: reg_num, imm: int }
  body: "reloc.symbol == format!(imm) && unpack_b(word) == (OP_BRANCH, 0, rs, 0, 0)"
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:379
```

## encode_beqz_neg_arity
- Tier: 2
- Rationale: Two-operand form required; missing operands must Err. llvm-mc rejects too few operands.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_sgtz_pbt encode_sgtz_neg_arity
- Formal: ∀ ops. len(ops) < 2 ⇒ encode_beqz(ops) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: holds
    expr: encode_beqz(ops).is_err()
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:328
```

## encode_beqz_neg_invalid
- Tier: 2
- Rationale: Invalid rs (FP/vector/unknown) or invalid target types (Mem/Csr/Fence/RM/SymbolOffset) must Err. Implemented as encode_beqz_neg_invalid_rs + encode_beqz_neg_invalid_target.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_sgtz_pbt encode_sgtz_neg_invalid
- Formal: ∀ bad_rs ∈ InvalidReg, good_tgt ∈ LabelIdents. encode_beqz([bad_rs, Symbol(good_tgt)]) = Err(_) ∧ ∀ good_rs ∈ GPR, bad_tgt ∈ InvalidTarget. encode_beqz([Reg(good_rs), bad_tgt]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_beqz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good]
  domain: { bad: invalid_operand, good: gpr_or_ident }
  relation:
    op: holds
    expr: encode_beqz(ops(bad, good)).is_err()
generators:
  bad: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:473
```

## encode_beqz_neg_extra
- Tier: 2
- Rationale: Exactly two operands (README + llvm-mc rejects third). SUT must not silently ignore extras.
- Doc contract: src/backend/riscv/assembler/README.md:328 "| `beqz/bnez`    | `beq/bne rs, x0, label`                              |" — asserted fingerprint d12d645c
- Seed: encode_sgtz_pbt encode_sgtz_neg_extra / test_encode_sgtz_regression_extra_operand
- Formal: ∀ rs ∈ GPRNames, tgt ∈ LabelIdents, extra ∈ Operand. encode_beqz([Reg(rs), Symbol(tgt), extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_beqz_pbt.rs
- Status: failing
- Counterexample: encode_beqz([Reg("a0"), Symbol("foo"), Reg("a1")])
- Bug report: bug_reports/encode_beqz_extra_operand.md
- Re-verified: PBT_TEST_JOBS=1 cargo test --lib test_encode_beqz_regression_extra_operand -- --test-threads=1 → FAIL (serial)

```property
function: encode_beqz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, tgt, extra]
  domain: { rs: gpr_name, tgt: ident, extra: extra_operand }
  relation:
    op: holds
    expr: encode_beqz([Reg(rs), Symbol(tgt), extra]).is_err()
generators:
  rs: { gen: string, type: String }
  tgt: { gen: string, type: String }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:328
```
