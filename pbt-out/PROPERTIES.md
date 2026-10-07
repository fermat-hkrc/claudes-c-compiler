# Properties: encode_bgt

## encode_bgt_diff_llvm_mc
- Tier: 5
- Rationale: Strongest independent reference is llvm-mc RISC-V assembler. State machine rejected (pure function). Round-trip rejected (no BGT decoder). Differential vs encode_branch_instr used as secondary metamorphic only (shared encode_b/get_reg).
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants" — asserted fingerprint 2ee023dd
- Seed: (none — no prior unit test for encode_bgt)
- Formal: ∀ rs,rt ∈ GPRNames. encode_bgt([Reg(rs), Reg(rt), Imm(0)]).word = llvm_mc("bgt rs, rt, 0") ∧ reloc=(Branch,"0",0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: differential
predicate:
  quantifier: forall
  vars: [rs, rt]
  domain: { rs: gpr_name, rt: gpr_name }
  relation:
    op: eq
    lhs: encode_bgt([Reg(rs), Reg(rt), Imm(0)]).word
    rhs: llvm_mc("bgt rs, rt, 0")
generators:
  rs: { gen: string, class: gpr_name }
  rt: { gen: string, class: gpr_name }
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_bgt_diff_llvm_mc_blt
- Tier: 5
- Rationale: Documented expansion BGT rs,rt → BLT rt,rs checked against independent llvm-mc of the real instruction (strengthens differential beyond mnemonic alias).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:341 "// blt rs2, rs1" — asserted fingerprint faae9ea6
- Seed: (none)
- Formal: ∀ rs,rt ∈ GPRNames. encode_bgt([Reg(rs), Reg(rt), Imm(0)]).word = llvm_mc("blt rt, rs, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: differential
predicate:
  quantifier: forall
  vars: [rs, rt]
  domain: { rs: gpr_name, rt: gpr_name }
  relation:
    op: eq
    lhs: encode_bgt([Reg(rs), Reg(rt), Imm(0)]).word
    rhs: llvm_mc("blt rt, rs, 0")
generators:
  rs: { gen: string, class: gpr_name }
  rt: { gen: string, class: gpr_name }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:341
```

## encode_bgt_eq_blt_swapped
- Tier: 4
- Rationale: Algebraic metamorphic — same-job expansion through encode_branch_instr(BLT) with swapped register order must agree on full WordWithReloc.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:341 "// blt rs2, rs1" — asserted fingerprint faae9ea6
- Seed: (none)
- Formal: ∀ r1,r2 ∈ 0..31, tgt ∈ Idents. encode_bgt([xN(r1), xN(r2), Symbol(tgt)]) = encode_branch_instr([xN(r2), xN(r1), Symbol(tgt)], funct3=BLT)
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r1, r2, tgt]
  domain: { r1: reg_num, r2: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgt([Reg(xN(r1)), Reg(xN(r2)), Symbol(tgt)])
    rhs: encode_branch_instr([Reg(xN(r2)), Reg(xN(r1)), Symbol(tgt)], 0b100)
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, class: ident }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:341
```

## encode_bgt_isa_b_type
- Tier: 4
- Rationale: Algebraic invariant — B-type field layout per RISC-V unprivileged ISA (independent unpack, not a copy of encode_b): opcode BRANCH, funct3 BLT, rs1=rt, rs2=rs, imm=0, reloc Branch.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:341 "// blt rs2, rs1" — asserted fingerprint faae9ea6
- Seed: (none)
- Formal: ∀ r1,r2 ∈ 0..31, tgt ∈ Idents. unpack_b(encode_bgt([xN(r1),xN(r2),Symbol(tgt)]).word) = (OP_BRANCH, 0b100, r2, r1, 0) ∧ reloc=(Branch,tgt,0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r1, r2, tgt]
  domain: { r1: reg_num, r2: reg_num, tgt: ident }
  body: unpack_b(encode_bgt([xN(r1),xN(r2),Symbol(tgt)]).word) == (OP_BRANCH, 0b100, r2, r1, 0)
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, class: ident }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:341
```

## encode_bgt_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic — ABI names, xN, and fp≡s0≡x8 must produce identical encodings.
- Doc contract: src/backend/riscv/assembler/README.md:330 "Swapped-operand `blt`/`bge` variants" — asserted fingerprint 2ee023dd
- Seed: (none)
- Formal: ∀ n,m ∈ 0..31, tgt ∈ Idents. encode_bgt([ABI(n),ABI(m),Symbol(tgt)]) = encode_bgt([xN(n),xN(m),Symbol(tgt)]); n=8 ⇒ fp alias holds on either side
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, tgt]
  domain: { n: reg_num, m: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgt([Reg(ABI(n)), Reg(ABI(m)), Symbol(tgt)])
    rhs: encode_bgt([Reg(xN(n)), Reg(xN(m)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, class: ident }
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_bgt_target_forms
- Tier: 4
- Rationale: get_branch_target treats Symbol/Label/Reg-as-label equivalently for the same string.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:380 "A register name can also be a symbol/label name" — asserted fingerprint 4ea15218
- Seed: (none)
- Formal: ∀ r1,r2 ∈ 0..31, s ∈ Idents. encode_bgt([xN(r1),xN(r2),Symbol(s)]) = encode_bgt(...,Label(s)) = encode_bgt(...,Reg(s))
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r1, r2, s]
  domain: { r1: reg_num, r2: reg_num, s: ident }
  body: encode_bgt([xN(r1),xN(r2),Symbol(s)]) == encode_bgt([xN(r1),xN(r2),Label(s)]) == encode_bgt([xN(r1),xN(r2),Reg(s)])
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, class: ident }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:380
```

## encode_bgt_imm_target
- Tier: 4
- Rationale: Imm target is stringified into reloc.symbol; word stays zero-imm BLT with swapped regs.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:341 "// blt rs2, rs1" — asserted fingerprint faae9ea6
- Seed: (none)
- Formal: ∀ r1,r2 ∈ 0..31, imm ∈ ImmSample. unpack_b(encode_bgt([xN(r1),xN(r2),Imm(imm)]).word)=(OP_BRANCH,0b100,r2,r1,0) ∧ reloc=(Branch, str(imm), 0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r1, r2, imm]
  domain: { r1: reg_num, r2: reg_num, imm: branch_imm_sample }
  body: unpack_b(word)==(OP_BRANCH,0b100,r2,r1,0) AND reloc.symbol==format!("{}",imm)
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:341
```

## encode_bgt_imm_as_reg
- Tier: 4
- Rationale: GCC bare Imm(0..31) register path via get_reg must equal xN for both rs and rt.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:490 "GCC sometimes emits bare register numbers (0-31) in inline asm" — asserted fingerprint f1b1a1fb
- Seed: (none)
- Formal: ∀ n,m ∈ 0..31, tgt ∈ Idents. encode_bgt([Imm(n),Imm(m),Symbol(tgt)]) = encode_bgt([xN(n),xN(m),Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, tgt]
  domain: { n: reg_num, m: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgt([Imm(n), Imm(m), Symbol(tgt)])
    rhs: encode_bgt([Reg(xN(n)), Reg(xN(m)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, class: ident }
evidence: src/backend/riscv/assembler/encoder/mod.rs:490
```

## encode_bgt_neg_arity
- Tier: 3
- Rationale: Negative contract — fewer than 3 operands must Err (llvm-mc "too few operands"; README three-operand form).
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu`" — domain-restriction fingerprint 7094e033
- Seed: (none)
- Formal: ∀ ops. len(ops)<3 ⇒ encode_bgt(ops) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops_lt3 }
  relation:
    op: holds
    expr: encode_bgt(ops).is_err()
generators:
  ops: { gen: list, elem: operand, maxLen: 2 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_bgt_neg_invalid_regs
- Tier: 3
- Rationale: Invalid rs or rt (FP/vector/out-of-range/non-reg) must Err.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:488 "invalid integer register" — asserted fingerprint 4a1a5b19
- Seed: (none)
- Formal: ∀ bad ∈ InvalidRs, good ∈ GPRNames, tgt ∈ Idents. encode_bgt([bad,Reg(good),Symbol(tgt)])=Err ∧ encode_bgt([Reg(good),bad,Symbol(tgt)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, tgt]
  domain: { bad: invalid_rs, good: gpr_name, tgt: ident }
  body: encode_bgt([bad, Reg(good), Symbol(tgt)]).is_err() AND encode_bgt([Reg(good), bad, Symbol(tgt)]).is_err()
generators:
  bad: { gen: operand, class: invalid_rs }
  good: { gen: string, class: gpr_name }
  tgt: { gen: string, class: ident }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:488
```

## encode_bgt_neg_invalid_target
- Tier: 3
- Rationale: Target forms get_branch_target rejects (Mem/Csr/Fence/…) must Err.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:383 "expected branch target at operand" — asserted fingerprint 5e30155f
- Seed: (none)
- Formal: ∀ r1,r2 ∈ GPRNames, bad ∈ InvalidTarget. encode_bgt([Reg(r1),Reg(r2),bad]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [r1, r2, bad]
  domain: { r1: gpr_name, r2: gpr_name, bad: invalid_target }
  relation:
    op: holds
    expr: encode_bgt([Reg(r1), Reg(r2), bad]).is_err()
generators:
  r1: { gen: string, class: gpr_name }
  r2: { gen: string, class: gpr_name }
  bad: { gen: operand, class: invalid_target }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:383
```

## encode_bgt_neg_extra
- Tier: 3
- Rationale: Four-or-more operands must Err (llvm-mc rejects extra operand; README documents three-operand bgt rs, rt, label). encode_bgt only reads indices 0..2 and silently ignores extras.
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu`" three-operand swapped-blt form — domain-restriction fingerprint 7094e033
- Seed: (none)
- Formal: ∀ rs,rt ∈ GPRNames, tgt ∈ Idents, extra ∈ Operand. encode_bgt([Reg(rs),Reg(rt),Symbol(tgt),extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgt_pbt.rs
- Status: failing
- Counterexample: encode_bgt([Reg("zero"), Reg("zero"), Symbol("foo"), Reg("zero")]) → Ok(WordWithReloc); also Reg("a0"),Reg("a1"),Symbol("foo"),Reg("a2")
- Bug report: bug_reports/encode_bgt_extra_operand.md

```property
function: encoder.encode_bgt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, rt, tgt, extra]
  domain: { rs: gpr_name, rt: gpr_name, tgt: ident, extra: operand }
  relation:
    op: holds
    expr: encode_bgt([Reg(rs), Reg(rt), Symbol(tgt), extra]).is_err()
generators:
  rs: { gen: string, class: gpr_name }
  rt: { gen: string, class: gpr_name }
  tgt: { gen: string, class: ident }
  extra: { gen: operand, class: extra_operand }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:330
```
