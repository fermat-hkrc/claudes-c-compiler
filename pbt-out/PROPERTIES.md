# PROPERTIES — encode_bgez

## encode_bgez_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential vs independent llvm-mc. State machine rejected (pure). Round-trip rejected (no decoder). Sibling encode_branch_instr shares encode_b (metamorphic only).
- Doc contract: pseudo.rs:313 "// bge rs, x0" — asserted fingerprint 93651925
- Seed: encode_blez_pbt.rs encode_blez_diff_llvm_mc (sibling pattern)
- Formal: ∀ rs ∈ GPRNames. encode_bgez([Reg(rs), Imm(0)]).word = llvm-mc("bgez rs, 0") ∧ reloc=(Branch,"0",0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: encode_bgez([Reg(rs), Imm(0)]).word
    rhs: llvm_mc("bgez " + rs + ", 0")
generators:
  rs: { gen: string, type: String }
evidence: README.md:329; RISC-V ISA BGEZ=BGE rs,x0; llvm-mc -triple=riscv64
```

## encode_bgez_diff_llvm_mc_bge
- Tier: 5
- Rationale: Documented expansion BGEZ rs ≡ BGE rs, x0 must agree with llvm-mc on the expanded form.
- Doc contract: pseudo.rs:313 "// bge rs, x0" — asserted fingerprint 93651925
- Seed: encode_blez_pbt.rs encode_blez_diff_llvm_mc_bge
- Formal: ∀ rs ∈ GPRNames. encode_bgez([Reg(rs), Imm(0)]).word = llvm-mc("bge rs, x0, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: encode_bgez([Reg(rs), Imm(0)]).word
    rhs: llvm_mc("bge " + rs + ", x0, 0")
generators:
  rs: { gen: string, type: String }
evidence: README.md:329; pseudo.rs:313
```

## encode_bgez_eq_bge_rs_x0
- Tier: 4
- Rationale: Metamorphic same-job expansion via encode_branch_instr(BGE).
- Doc contract: pseudo.rs:313 "// bge rs, x0" — asserted fingerprint 93651925
- Seed: encode_blez_pbt.rs encode_blez_eq_bge_x0
- Formal: ∀ rs ∈ 0..31, ∀ tgt ∈ Idents. encode_bgez([xN(rs), Symbol(tgt)]) = encode_branch_instr([xN(rs), x0, Symbol(tgt)], BGE) = encode_branch_instr([xN(rs), zero, Symbol(tgt)], BGE)
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgez([Reg(xN(rs)), Symbol(tgt)])
    rhs: encode_branch_instr([Reg(xN(rs)), Reg("x0"), Symbol(tgt)], 0b101)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: README.md:329; pseudo.rs:309-315
```

## encode_bgez_isa_b_type
- Tier: 4
- Rationale: B-type layout invariant — opcode BRANCH, funct3 BGE, rs1=rs, rs2=x0, imm=0, reloc Branch.
- Doc contract: pseudo.rs:313 "// bge rs, x0" — asserted fingerprint 93651925
- Seed: encode_blez_pbt.rs encode_blez_isa_b_type
- Formal: ∀ rs ∈ 0..31, ∀ tgt. let (w,k,s,a)=encode_bgez([xN(rs),Symbol(tgt)]). unpack_b(w)=(OP_BRANCH, BGE, rs, 0, 0) ∧ (k,s,a)=(Branch,tgt,0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: reg_num, tgt: ident }
  body: unpack_b(encode_bgez([xN(rs), Symbol(tgt)]).word) == (OP_BRANCH, BGE, rs, 0, 0)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: RISC-V Unprivileged ISA B-type; pseudo.rs:309-315
```

## encode_bgez_abi_xn_alias
- Tier: 4
- Rationale: ABI name, xN, and fp(≡x8) are aliases for the same encoding.
- Doc contract: (none)
- Seed: encode_blez_pbt.rs encode_blez_abi_xn_alias
- Formal: ∀ n ∈ 0..31, ∀ tgt. encode_bgez([ABI(n), Symbol(tgt)]) = encode_bgez([xN(n), Symbol(tgt)]) ∧ (n=8 ⇒ encode_bgez([fp, Symbol(tgt)]) equals them)
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgez([Reg(ABI(n)), Symbol(tgt)])
    rhs: encode_bgez([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: encoder get_reg / reg_num ABI table
```

## encode_bgez_target_forms
- Tier: 4
- Rationale: Symbol, Label, and Reg-as-label with the same string yield identical WordWithReloc.
- Doc contract: pseudo.rs:383 "expected branch target at operand {}" — asserted fingerprint fdb4f80b
- Seed: encode_blez_pbt.rs encode_blez_target_forms
- Formal: ∀ rs, s. encode_bgez([xN(rs), Symbol(s)]) = encode_bgez([xN(rs), Label(s)]) = encode_bgez([xN(rs), Reg(s)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, s]
  domain: { rs: reg_num, s: ident }
  relation:
    op: eq
    lhs: encode_bgez([xN(rs), Symbol(s)])
    rhs: encode_bgez([xN(rs), Label(s)])
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, type: String }
evidence: pseudo.rs:376-385 get_branch_target
```

## encode_bgez_imm_target
- Tier: 4
- Rationale: Imm target stringifies into reloc.symbol; word stays zero-imm BGE rs,x0.
- Doc contract: pseudo.rs:379 "Some(Operand::Imm(v)) => Ok(format!" — asserted fingerprint 6d9a81ec
- Seed: encode_blez_pbt.rs encode_blez_imm_target
- Formal: ∀ rs, imm. encode_bgez([xN(rs), Imm(imm)]) yields unpack_b=(BRANCH,BGE,rs,0,0) ∧ reloc=(Branch, format(imm), 0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, imm]
  domain: { rs: reg_num, imm: imm_edge }
  body: unpack + reloc.symbol == format(imm) && addend == 0
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -64, max: 64, type: i64 }
evidence: pseudo.rs:379
```

## encode_bgez_imm_as_rs
- Tier: 4
- Rationale: get_reg accepts bare Imm(0..31) (GCC path); must equal xN.
- Doc contract: mod.rs:487 "Some(Operand::Imm(n)) if *n >= 0 && *n <= 31 => Ok(*n as u32)," — asserted fingerprint 4df5b8ba
- Seed: encode_blez_pbt.rs encode_blez_imm_as_rs
- Formal: ∀ n ∈ 0..31, ∀ tgt. encode_bgez([Imm(n), Symbol(tgt)]) = encode_bgez([xN(n), Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgez([Imm(n), Symbol(tgt)])
    rhs: encode_bgez([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: mod.rs:484-485
```

## encode_bgez_neg_arity
- Tier: 3
- Rationale: Fewer than 2 operands must Err (README two-operand form; llvm-mc rejects).
- Doc contract: README.md:329 "Corresponding `bge`/`blt` with x0" — domain-restriction fingerprint e9cf8d73
- Seed: encode_blez_pbt.rs encode_blez_neg_arity
- Formal: ∀ ops. len(ops)<2 ⇒ encode_bgez(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: holds
    expr: encode_bgez(ops).is_err()
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: README.md:329; llvm-mc rejects missing operand
```

## encode_bgez_neg_invalid_rs
- Tier: 3
- Rationale: Non-GPR first operand must Err.
- Doc contract: mod.rs:488 "expected register at operand {}, got {:?}" — domain-restriction fingerprint 783b781e
- Seed: encode_blez_pbt.rs encode_blez_neg_invalid_rs
- Formal: ∀ bad ∉ GPR, ∀ tgt. encode_bgez([bad, Symbol(tgt)]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, tgt]
  domain: { bad: invalid_rs, tgt: ident }
  relation:
    op: holds
    expr: encode_bgez([bad, Symbol(tgt)]).is_err()
generators:
  bad: { gen: string, type: Operand }
  tgt: { gen: string, type: String }
expected_error: String
evidence: mod.rs:479-488 get_reg
```

## encode_bgez_neg_invalid_target
- Tier: 3
- Rationale: Target not Symbol/Label/Imm/Reg must Err.
- Doc contract: pseudo.rs:383 "expected branch target at operand {}" — domain-restriction fingerprint fdb4f80b
- Seed: encode_blez_pbt.rs encode_blez_neg_invalid_target
- Formal: ∀ rs ∈ GPR, ∀ bad ∉ {Symbol,Label,Imm,Reg}. encode_bgez([Reg(rs), bad]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, bad]
  domain: { rs: gpr_name, bad: invalid_target }
  relation:
    op: holds
    expr: encode_bgez([Reg(rs), bad]).is_err()
generators:
  rs: { gen: string, type: String }
  bad: { gen: string, type: Operand }
expected_error: String
evidence: pseudo.rs:376-385
```

## encode_bgez_neg_extra
- Tier: 3
- Rationale: Extra third operand must Err (llvm-mc rejects; README two-operand form). Same class as encode_blez/beqz/bnez.
- Doc contract: README.md:329 "Corresponding `bge`/`blt` with x0" — domain-restriction fingerprint e9cf8d73
- Seed: encode_blez_pbt.rs encode_blez_neg_extra
- Formal: ∀ rs ∈ GPR, ∀ tgt ∈ Idents, ∀ extra. encode_bgez([Reg(rs), Symbol(tgt), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgez_pbt.rs
- Status: failing
- Counterexample: encode_bgez([Reg("a0"), Symbol("foo"), Reg("a1")]) → Ok(WordWithReloc { word: 0x00055063 }) instead of Err
- Bug report: bug_reports/encode_bgez_extra_operand.md

```property
function: encoder.encode_bgez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, tgt, extra]
  domain: { rs: gpr_name, tgt: ident, extra: extra_operand }
  relation:
    op: holds
    expr: encode_bgez([Reg(rs), Symbol(tgt), extra]).is_err()
generators:
  rs: { gen: string, type: String }
  tgt: { gen: string, type: String }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: README.md:329; llvm-mc rejects extra operand
```
