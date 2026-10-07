# Properties: encode_blez

## encode_blez_kat_llvm_mc
- Tier: 5
- Rationale: Reference/KAT gate for the llvm-mc differential connection. Pins known encodings before randomized differential runs. Stronger state-machine N/A (pure). Round-trip N/A (no decoder).
- Doc contract: src/backend/riscv/assembler/README.md:329 "Corresponding `bge`/`blt` with x0" — asserted fingerprint e9cf8d73
- Seed: (none)
- Formal: ∀ known (rs, want) ∈ KAT. llvm-mc("blez rs, 0") = want ∧ encode_blez([Reg(rs), Imm(0)]) = WordWithReloc{word=want, Branch, "0", 0}
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: reference
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: kat_gpr }
  relation:
    op: eq
    lhs: encode_blez([Reg(rs), Imm(0)]).word
    rhs: llvm_mc("blez rs, 0")
generators:
  rs: { gen: string, const: "a0" }
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_blez_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle — independent llvm-mc assembler. Differential over all GPR name forms. State machine rejected (pure). Round-trip rejected (no decoder).
- Doc contract: src/backend/riscv/assembler/README.md:329 "Corresponding `bge`/`blt` with x0" — asserted fingerprint e9cf8d73
- Seed: encode_beqz_pbt.rs encode_beqz_diff_llvm_mc
- Formal: ∀ rs ∈ GPR_names. encode_blez([Reg(rs), Imm(0)]).word = llvm-mc("blez rs, 0") ∧ reloc=(Branch,"0",0)
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: encode_blez([Reg(rs), Imm(0)]).word
    rhs: llvm_mc("blez " + rs + ", 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_blez_diff_llvm_mc_bge
- Tier: 5
- Rationale: Documented expansion BLEZ = BGE x0, rs checked against independent assembler (not shared encode_b path).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:304 "bge x0, rs" — asserted fingerprint 3248f1d1
- Seed: encode_beqz_pbt.rs encode_beqz_diff_llvm_mc_beq
- Formal: ∀ rs ∈ GPR_names. encode_blez([Reg(rs), Imm(0)]).word = llvm-mc("bge x0, rs, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: encode_blez([Reg(rs), Imm(0)]).word
    rhs: llvm_mc("bge x0, " + rs + ", 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:304
```

## encode_blez_eq_bge_x0
- Tier: 4
- Rationale: Metamorphic — blez rs, label ≡ bge x0, rs, label via encode_branch_instr (weaker than llvm-mc but checks in-tree expansion).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:304 "bge x0, rs" — asserted fingerprint 3248f1d1
- Seed: encode_beqz_pbt.rs encode_beqz_eq_beq_x0
- Formal: ∀ rs ∈ 0..31, tgt ∈ Idents. encode_blez([xN(rs), Symbol(tgt)]) = encode_branch_instr([x0, xN(rs), Symbol(tgt)], BGE) = encode_branch_instr([zero, xN(rs), Symbol(tgt)], BGE)
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_blez([Reg(xN(rs)), Symbol(tgt)])
    rhs: encode_branch_instr([Reg(x0), Reg(xN(rs)), Symbol(tgt)], 0b101)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:304
```

## encode_blez_isa_b_type
- Tier: 4
- Rationale: Algebraic invariant — B-type field layout per RISC-V ISA: opcode BRANCH, funct3=BGE, rs1=x0, rs2=rs, imm=0 (deferred to reloc).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:304 "bge x0, rs" — asserted fingerprint 3248f1d1
- Seed: encode_beqz_pbt.rs encode_beqz_isa_b_type
- Formal: ∀ rs ∈ 0..31, tgt ∈ Idents. let w = encode_blez([xN(rs), Symbol(tgt)]).word in unpack_b(w)=(OP_BRANCH, 0b101, 0, rs, 0) ∧ reloc=(Branch, tgt, 0)
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: reg_num, tgt: ident }
  body: unpack_b(encode_blez([xN(rs), Symbol(tgt)]).word) == (OP_BRANCH, 0b101, 0, rs, 0)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:304
```

## encode_blez_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic — ABI name, xN, and fp/s0 aliases produce identical encoding.
- Doc contract: src/backend/riscv/assembler/README.md:329 "Corresponding `bge`/`blt` with x0" — asserted fingerprint e9cf8d73
- Seed: encode_beqz_pbt.rs encode_beqz_abi_xn_alias
- Formal: ∀ n ∈ 0..31, tgt ∈ Idents. encode_blez([ABI(n), Symbol(tgt)]) = encode_blez([xN(n), Symbol(tgt)]) ∧ (n=8 ⇒ encode_blez([fp, Symbol(tgt)]) equals same)
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_blez([Reg(abi(n)), Symbol(tgt)])
    rhs: encode_blez([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_blez_target_forms
- Tier: 4
- Rationale: Metamorphic — Symbol/Label/Reg-as-label targets with same string are equivalent (get_branch_target contract).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:380 "A register name can also be a symbol/label name (e.g. `beqz a0, t1`" — asserted fingerprint bc6f2199
- Seed: encode_beqz_pbt.rs encode_beqz_target_forms
- Formal: ∀ rs ∈ 0..31, s ∈ Idents. encode_blez([xN(rs), Symbol(s)]) = encode_blez([xN(rs), Label(s)]) = encode_blez([xN(rs), Reg(s)])
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, s]
  domain: { rs: reg_num, s: ident }
  relation:
    op: eq
    lhs: encode_blez([xN(rs), Symbol(s)])
    rhs: encode_blez([xN(rs), Label(s)])
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:380
```

## encode_blez_imm_target
- Tier: 4
- Rationale: Imm target stringified into reloc symbol; word stays zero-imm BGE.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:378 "Some(Operand::Imm(v)) => Ok(format!("{}", v))" — asserted fingerprint d1dd010f
- Seed: encode_beqz_pbt.rs encode_beqz_imm_target
- Formal: ∀ rs ∈ 0..31, imm ∈ ImmDomain. encode_blez([xN(rs), Imm(imm)]) = WordWithReloc{B-type BGE x0,rs imm0, Branch, format!("{}",imm), 0}
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, imm]
  domain: { rs: reg_num, imm: imm_domain }
  body: unpack_b(word)=(OP_BRANCH,0b101,0,rs,0) ∧ symbol=format!("{}",imm)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -64, max: 64, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:378
```

## encode_blez_imm_as_rs
- Tier: 4
- Rationale: get_reg bare Imm(0..31) path equals xN (GCC bare-number).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:482 "GCC sometimes emits bare register numbers (0-31) in inline asm" — asserted fingerprint f1b1a1fb
- Seed: encode_bnez_pbt.rs encode_bnez_imm_as_rs
- Formal: ∀ n ∈ 0..31, tgt ∈ Idents. encode_blez([Imm(n), Symbol(tgt)]) = encode_blez([xN(n), Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_blez([Imm(n), Symbol(tgt)])
    rhs: encode_blez([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:482
```

## encode_blez_neg_arity
- Tier: 3
- Rationale: Negative — fewer than 2 operands must Err (README two-operand form; llvm-mc rejects).
- Doc contract: src/backend/riscv/assembler/README.md:329 "Corresponding `bge`/`blt` with x0" — asserted fingerprint e9cf8d73
- Seed: encode_beqz_pbt.rs encode_beqz_neg_arity
- Formal: ∀ ops. |ops| < 2 ⇒ encode_blez(ops) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: holds
    expr: encode_blez(ops).is_err()
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_blez_neg_invalid_rs
- Tier: 3
- Rationale: Negative — non-GPR rs must Err.
- Doc contract: (none) — inferred from get_reg contract
- Seed: encode_beqz_pbt.rs encode_beqz_neg_invalid_rs
- Formal: ∀ bad ∉ GPR, tgt ∈ Idents. encode_blez([bad, Symbol(tgt)]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, tgt]
  domain: { bad: invalid_rs, tgt: ident }
  relation:
    op: holds
    expr: encode_blez([bad, Symbol(tgt)]).is_err()
generators:
  bad: { gen: string }
  tgt: { gen: string, type: String }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:476
```

## encode_blez_neg_invalid_target
- Tier: 3
- Rationale: Negative — Mem/Csr/Fence/etc targets must Err.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:383 "expected branch target at operand {}" — asserted fingerprint fdb4f80b
- Seed: encode_beqz_pbt.rs encode_beqz_neg_invalid_target
- Formal: ∀ rs ∈ GPR, bad ∉ {Symbol,Label,Imm,Reg}. encode_blez([Reg(rs), bad]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_blez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, bad]
  domain: { rs: gpr_name, bad: invalid_target }
  relation:
    op: holds
    expr: encode_blez([Reg(rs), bad]).is_err()
generators:
  rs: { gen: string, type: String }
  bad: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:383
```

## encode_blez_neg_extra
- Tier: 3
- Rationale: Negative — extra third operand must Err (README two-operand; llvm-mc rejects). Same class of bug found in beqz/bnez/sgtz siblings.
- Doc contract: src/backend/riscv/assembler/README.md:329 "Corresponding `bge`/`blt` with x0" — asserted fingerprint e9cf8d73
- Seed: encode_beqz_pbt.rs encode_beqz_neg_extra
- Formal: ∀ rs ∈ GPR, tgt ∈ Idents, extra ∈ Operand. encode_blez([Reg(rs), Symbol(tgt), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_blez_pbt.rs
- Status: failing
- Counterexample: encode_blez([Reg("a0"), Symbol("foo"), Reg("a1")]) → Ok(WordWithReloc { word: 0x00a05063 })
- Bug report: pbt-out/bug_reports/encode_blez_extra_operand.md

```property
function: encode_blez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, tgt, extra]
  domain: { rs: gpr_name, tgt: ident, extra: extra_operand }
  relation:
    op: holds
    expr: encode_blez([Reg(rs), Symbol(tgt), extra]).is_err()
generators:
  rs: { gen: string, type: String }
  tgt: { gen: string, type: String }
  extra: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:329
```
