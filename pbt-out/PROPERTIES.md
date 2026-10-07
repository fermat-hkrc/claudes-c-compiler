# Properties: encode_ble

## encode_ble_diff_llvm_mc
- Tier: 5
- Rationale: Strongest independent oracle is differential vs llvm-mc (trusted external assembler). State machine rejected (pure function). Round-trip rejected (no in-tree BLE decoder). Differential vs encode_branch_instr(bge) is same-job but shares encode_b/get_reg — kept as metamorphic, not primary.
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants" — asserted fingerprint fe7c7d0f
- Seed: encode_bgt_pbt.rs encode_bgt_diff_llvm_mc (sibling three-operand branch pseudo)
- Formal: ∀ rs, rt ∈ GPRNames. encode_ble([Reg(rs), Reg(rt), Imm(0)]).word = llvm_mc("ble rs, rt, 0") ∧ reloc = Branch("0", 0)
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: differential
predicate:
  quantifier: forall
  vars: [rs, rt]
  domain: { rs: gpr_name, rt: gpr_name }
  relation:
    op: eq
    lhs: "sut_word(Reg(rs), Reg(rt), Imm(0))"
    rhs: "llvm_mc_word(format!(\"ble {}, {}, 0\", rs, rt))"
generators:
  rs: { gen: string, type: String }
  rt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_ble_diff_llvm_mc_bge
- Tier: 5
- Rationale: Documented expansion BLE rs,rt = BGE rt,rs must match independent llvm-mc bge encoding (metamorphic/differential dual check).
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants" — asserted fingerprint fe7c7d0f
- Seed: encode_bgt_pbt.rs encode_bgt_diff_llvm_mc_blt
- Formal: ∀ rs, rt ∈ GPRNames. encode_ble([Reg(rs), Reg(rt), Imm(0)]).word = llvm_mc("bge rt, rs, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: differential
predicate:
  quantifier: forall
  vars: [rs, rt]
  domain: { rs: gpr_name, rt: gpr_name }
  relation:
    op: eq
    lhs: "sut_word(Reg(rs), Reg(rt), Imm(0))"
    rhs: "llvm_mc_word(format!(\"bge {}, {}, 0\", rt, rs))"
generators:
  rs: { gen: string, type: String }
  rt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_ble_eq_bge_swapped
- Tier: 4
- Rationale: Algebraic metamorphic — BLE rs,rt,lbl ≡ BGE rt,rs,lbl via encode_branch_instr(FUNCT3_BGE). Weaker than llvm-mc differential (shared helpers) but pins the documented expansion independently of external tool.
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants" — asserted fingerprint fe7c7d0f
- Seed: encode_bgt_pbt.rs encode_bgt_eq_blt_swapped
- Formal: ∀ r1,r2 ∈ 0..31, tgt ∈ Idents. encode_ble([x(r1),x(r2),Symbol(tgt)]) = encode_branch_instr([x(r2),x(r1),Symbol(tgt)], funct3=BGE)
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r1, r2, tgt]
  domain: { r1: "0..31", r2: "0..31", tgt: ident }
  relation:
    op: eq
    lhs: "encode_ble([x(r1), x(r2), Symbol(tgt)])"
    rhs: "encode_branch_instr([x(r2), x(r1), Symbol(tgt)], 0b101)"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_ble_isa_b_type
- Tier: 4
- Rationale: Algebraic invariant — B-type layout per RISC-V ISA with funct3=BGE and registers swapped (rs1=rt, rs2=rs), imm=0, reloc Branch(tgt,0).
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants" — asserted fingerprint fe7c7d0f
- Seed: encode_bgt_pbt.rs encode_bgt_isa_b_type
- Formal: ∀ r1,r2 ∈ 0..31, tgt ∈ Idents. let w,k,s,a = encode_ble(...). unpack_b(w)=(OP_BRANCH, BGE, r2, r1, 0) ∧ k=Branch ∧ s=tgt ∧ a=0
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r1, r2, tgt]
  domain: { r1: "0..31", r2: "0..31", tgt: ident }
  body: "unpack_b(word)=(OP_BRANCH,0b101,r2,r1,0) && reloc=Branch(tgt,0)"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_ble_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic — ABI names, xN, and fp/s0/x8 aliases encode identically for the same register numbers.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_bgt_pbt.rs encode_bgt_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31, tgt. encode_ble(ABI(n),ABI(m),tgt) = encode_ble(xN(n),xN(m),tgt); n=8 ⇒ fp form equal; m=8 ⇒ fp form equal
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, tgt]
  domain: { n: "0..31", m: "0..31", tgt: ident }
  relation:
    op: eq
    lhs: "encode_ble(abi(n), abi(m), tgt)"
    rhs: "encode_ble(xn(n), xn(m), tgt)"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: encoder register alias table
```

## encode_ble_target_forms
- Tier: 4
- Rationale: Metamorphic — Symbol/Label/Reg-as-label with same string yield identical WordWithReloc (get_branch_target contract).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:378 "Some(Operand::Symbol(s)) | Some(Operand::Label(s)) => Ok(s.clone())," — asserted fingerprint 5e30155f
- Seed: encode_bgt_pbt.rs encode_bgt_target_forms
- Formal: ∀ r1,r2,s. encode_ble(...,Symbol(s)) = encode_ble(...,Label(s)) = encode_ble(...,Reg(s))
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r1, r2, s]
  domain: { r1: "0..31", r2: "0..31", s: ident }
  body: "encode_ble(Symbol(s)) == encode_ble(Label(s)) == encode_ble(Reg(s))"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:378
```

## encode_ble_imm_target
- Tier: 4
- Rationale: Imm branch targets stringify into reloc.symbol; machine-word immediate stays 0 (reloc carries the target).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:379 "Some(Operand::Imm(v)) => Ok(format!(\"{}\", v))," — asserted fingerprint 5e30155f
- Seed: encode_bgt_pbt.rs encode_bgt_imm_target
- Formal: ∀ r1,r2,imm. encode_ble(...,Imm(imm)) → unpack_b BGE swapped, off=0, reloc.symbol=format!(imm), addend=0
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r1, r2, imm]
  domain: { r1: "0..31", r2: "0..31", imm: i64_edge }
  body: "word is zero-imm BGE swapped; reloc.symbol == format!(imm)"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:379
```

## encode_ble_imm_as_reg
- Tier: 4
- Rationale: Metamorphic — bare Imm(0..31) as rs/rt (GCC bare-number path in get_reg) equals xN form.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_bgt_pbt.rs encode_bgt_imm_as_reg
- Formal: ∀ n,m ∈ 0..31, tgt. encode_ble(Imm(n),Imm(m),tgt) = encode_ble(xN(n),xN(m),tgt)
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, tgt]
  domain: { n: "0..31", m: "0..31", tgt: ident }
  relation:
    op: eq
    lhs: "encode_ble(Imm(n), Imm(m), Symbol(tgt))"
    rhs: "encode_ble(xN(n), xN(m), Symbol(tgt))"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: get_reg bare Imm path
```

## encode_ble_neg_arity
- Tier: 3
- Rationale: Negative error — under-arity (<3 operands) must Err (README three-operand form; llvm-mc rejects).
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants" — asserted fingerprint fe7c7d0f
- Seed: encode_bgt_pbt.rs encode_bgt_neg_arity
- Formal: ∀ ops. |ops| < 3 ⇒ encode_ble(ops) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len < 3" }
  relation:
    op: throws
    expr: "encode_ble(ops)"
generators:
  ops: { gen: list, maxLen: 2 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:330
```

## encode_ble_neg_invalid_regs
- Tier: 3
- Rationale: Negative error — invalid GPR kinds (FP, vector, out-of-range Imm, Mem, Csr, …) as rs or rt must Err.
- Doc contract: (none) — other fingerprint 00000000
- Seed: encode_bgt_pbt.rs encode_bgt_neg_invalid_regs
- Formal: ∀ bad ∉ ValidGpr, good ∈ GPR, tgt. encode_ble([bad,good,tgt])=Err ∧ encode_ble([good,bad,tgt])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, tgt]
  domain: { bad: invalid_rs, good: gpr_name, tgt: ident }
  body: "encode_ble rejects bad as rs and as rt"
generators:
  bad: { gen: string, type: Operand }
  good: { gen: string, type: String }
  tgt: { gen: string, type: String }
expected_error: String
evidence: get_reg
```

## encode_ble_neg_invalid_target
- Tier: 3
- Rationale: Negative error — target kinds get_branch_target rejects (Mem, Csr, FenceArg, …) must Err.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:383 "_ => Err(format!(\"expected branch target at operand {}\", idx))," — asserted fingerprint 5e30155f
- Seed: encode_bgt_pbt.rs encode_bgt_neg_invalid_target
- Formal: ∀ rs,rt ∈ GPR, bad ∉ {Symbol,Label,Imm,Reg}. encode_ble([rs,rt,bad]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ble
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, rt, bad]
  domain: { rs: gpr_name, rt: gpr_name, bad: invalid_target }
  relation:
    op: throws
    expr: "encode_ble([rs,rt,bad])"
generators:
  rs: { gen: string, type: String }
  rt: { gen: string, type: String }
  bad: { gen: string, type: Operand }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:383
```

## encode_ble_neg_extra
- Tier: 3
- Rationale: Negative error — trailing fourth+ operand must Err (README three-operand form; llvm-mc rejects extra). Sibling encode_bgt has this exact family bug. Serial reconfirm with PBT_TEST_JOBS=1 reproduces.
- Doc contract: src/backend/riscv/assembler/README.md:330 "`bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants" — asserted fingerprint fe7c7d0f
- Seed: encode_bgt_pbt.rs encode_bgt_neg_extra + bug_reports/encode_bgt_extra_operand.md
- Formal: ∀ rs,rt ∈ GPR, tgt ∈ Idents, extra ∈ Operand. encode_ble([rs,rt,Symbol(tgt),extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_ble_pbt.rs
- Status: failing
- Counterexample: rs="zero", rt="zero", tgt="foo", extra=Reg("zero") (also a0,a1,foo,a2)
- Bug report: bug_reports/encode_ble_extra_operand.md

```property
function: encode_ble
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, rt, tgt, extra]
  domain: { rs: gpr_name, rt: gpr_name, tgt: ident, extra: Operand }
  relation:
    op: throws
    expr: "encode_ble([rs,rt,Symbol(tgt),extra])"
generators:
  rs: { gen: string, type: String }
  rt: { gen: string, type: String }
  tgt: { gen: string, type: String }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:330
```
