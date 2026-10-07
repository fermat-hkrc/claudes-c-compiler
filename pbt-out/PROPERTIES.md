# Properties: encode_bgtu

## encode_bgtu_diff_llvm_mc
- Tier: 5
- Rationale: Strongest independent differential — llvm-mc is an external RISC-V assembler with no shared encode_b/get_reg source. State machine rejected (pure function). Round-trip rejected (no in-tree BGTU/BLTU decoder). Differential vs encode_branch_instr(bltu) demoted to metamorphic (shared encode_b).
- Doc contract: README.md:330 "| `bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants            |" — asserted fingerprint 100b43c8; pseudo.rs:361 "word: encode_b(OP_BRANCH, 0b110, rs2, rs1, 0), // bltu rs2, rs1" — asserted fingerprint 6ebdd4d8
- Seed: encode_bgt_pbt.rs encode_bgt_diff_llvm_mc (sibling)
- Formal: ∀ rs, rt ∈ GPRNames. encode_bgtu([Reg(rs), Reg(rt), Imm(0)]).word = llvm_mc("bgtu rs, rt, 0") ∧ reloc = Branch("0", 0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: differential
predicate:
  quantifier: forall
  vars: [rs, rt]
  domain: { rs: gpr_name, rt: gpr_name }
  relation:
    op: eq
    lhs: "sut_reloc([Reg(rs), Reg(rt), Imm(0)]).word"
    rhs: "llvm_mc_word(format!(\"bgtu {}, {}, 0\", rs, rt))"
generators:
  rs: { gen: gpr_name }
  rt: { gen: gpr_name }
evidence: README.md:330; RISC-V Unprivileged ISA BGTU=BLTU swapped; llvm-mc KAT gate
```

## encode_bgtu_diff_llvm_mc_bltu
- Tier: 5
- Rationale: Documented expansion vs independent assembler (bltu rt, rs, 0). Strengthens the differential by checking the ISA expansion form, not only the pseudo mnemonic.
- Doc contract: README.md:330 "| `bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants            |" — asserted fingerprint 100b43c8
- Seed: encode_bgt_pbt.rs encode_bgt_diff_llvm_mc_blt
- Formal: ∀ rs, rt ∈ GPRNames. encode_bgtu([Reg(rs), Reg(rt), Imm(0)]).word = llvm_mc("bltu rt, rs, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: differential
predicate:
  quantifier: forall
  vars: [rs, rt]
  domain: { rs: gpr_name, rt: gpr_name }
  relation:
    op: eq
    lhs: "sut_reloc([Reg(rs), Reg(rt), Imm(0)]).word"
    rhs: "llvm_mc_word(format!(\"bltu {}, {}, 0\", rt, rs))"
generators:
  rs: { gen: gpr_name }
  rt: { gen: gpr_name }
evidence: README.md:330; RISC-V ISA BGTU rs,rt,off = BLTU rt,rs,off
```

## encode_bgtu_eq_bltu_swapped
- Tier: 4
- Rationale: Algebraic metamorphic — documented operand-swap expansion against in-tree encode_branch_instr(BLTU). Weaker than llvm-mc differential (shared encode_b) but covers WordWithReloc including symbol targets.
- Doc contract: pseudo.rs:361 "word: encode_b(OP_BRANCH, 0b110, rs2, rs1, 0), // bltu rs2, rs1" — asserted fingerprint 6ebdd4d8
- Seed: encode_bgt_pbt.rs encode_bgt_eq_blt_swapped
- Formal: ∀ r1,r2 ∈ 0..31, tgt ∈ Idents. encode_bgtu([x(r1),x(r2),Symbol(tgt)]) = encode_branch_instr([x(r2),x(r1),Symbol(tgt)], funct3=BLTU)
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r1, r2, tgt]
  domain: { r1: reg_num, r2: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: "sut_reloc([x(r1), x(r2), Symbol(tgt)])"
    rhs: "bltu_reloc([x(r2), x(r1), Symbol(tgt)])"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: ident }
evidence: pseudo.rs:361; RISC-V ISA BGTU = BLTU swapped
```

## encode_bgtu_isa_b_type
- Tier: 4
- Rationale: Algebraic invariant — B-type field layout per RISC-V unprivileged ISA (opcode BRANCH, funct3 BLTU=0b110, rs1=rt, rs2=rs, imm=0, reloc Branch).
- Doc contract: pseudo.rs:361 "word: encode_b(OP_BRANCH, 0b110, rs2, rs1, 0), // bltu rs2, rs1" — asserted fingerprint 6ebdd4d8
- Seed: encode_bgt_pbt.rs encode_bgt_isa_b_type
- Formal: ∀ r1,r2 ∈ 0..31, tgt ∈ Idents. let (w,k,s,a)=encode_bgtu([x(r1),x(r2),Symbol(tgt)]). unpack_b(w)=(OP_BRANCH, 0b110, r2, r1, 0) ∧ k=Branch ∧ s=tgt ∧ a=0
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r1, r2, tgt]
  domain: { r1: reg_num, r2: reg_num, tgt: ident }
  body: "unpack_b(word)=(OP_BRANCH, FUNCT3_BLTU, r2, r1, 0) ∧ reloc=Branch(tgt,0)"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: ident }
evidence: RISC-V Unprivileged ISA B-type; pseudo.rs:361-363
```

## encode_bgtu_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic — ABI names, xN, and fp/s0/x8 aliases encode identically for the same register numbers.
- Doc contract: (none on encode_bgtu; ABI table is architectural)
- Seed: encode_bgt_pbt.rs encode_bgt_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31, tgt. encode_bgtu([ABI(n),ABI(m),Symbol(tgt)]) = encode_bgtu([xN(n),xN(m),Symbol(tgt)]); when n=8 also fp; when m=8 also fp
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, tgt]
  domain: { n: reg_num, m: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: "sut_reloc([ABI(n), ABI(m), Symbol(tgt)])"
    rhs: "sut_reloc([xN(n), xN(m), Symbol(tgt)])"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: ident }
evidence: RISC-V ABI register names; get_reg alias table
```

## encode_bgtu_target_forms
- Tier: 4
- Rationale: Metamorphic — Symbol / Label / Reg-as-label targets with the same string yield identical reloc.
- Doc contract: pseudo.rs:378 "Some(Operand::Symbol(s)) | Some(Operand::Label(s)) => Ok(s.clone())," — asserted fingerprint ad06d32a; pseudo.rs:382 "Some(Operand::Reg(s)) => Ok(s.clone())," — asserted
- Seed: encode_bgt_pbt.rs encode_bgt_target_forms
- Formal: ∀ r1,r2,s. encode_bgtu([x(r1),x(r2),Symbol(s)]) = encode_bgtu([…,Label(s)]) = encode_bgtu([…,Reg(s)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [r1, r2, s]
  domain: { r1: reg_num, r2: reg_num, s: ident }
  body: "sut(Symbol(s)) = sut(Label(s)) = sut(Reg(s))"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: ident }
evidence: pseudo.rs:377-383 get_branch_target
```

## encode_bgtu_imm_target
- Tier: 4
- Rationale: Invariant — Imm target is stringified into reloc.symbol; machine-word immediate stays 0 (reloc deferred).
- Doc contract: pseudo.rs:379 "Some(Operand::Imm(v)) => Ok(format!(\"{}\", v))," — asserted fingerprint ad06d32a
- Seed: encode_bgt_pbt.rs encode_bgt_imm_target
- Formal: ∀ r1,r2,imm. encode_bgtu([x(r1),x(r2),Imm(imm)]) yields B-type BLTU swapped with off=0, reloc Branch(format!("{}",imm), 0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r1, r2, imm]
  domain: { r1: reg_num, r2: reg_num, imm: i64_edge }
  body: "unpack_b(w)=(OP_BRANCH,BLTU,r2,r1,0) ∧ reloc=Branch(format!(imm),0)"
generators:
  r1: { gen: int, min: 0, max: 31, type: u32 }
  r2: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -64, max: 64, type: i64 }
evidence: pseudo.rs:379
```

## encode_bgtu_imm_as_reg
- Tier: 4
- Rationale: Metamorphic sweep — bare Imm(0..31) as rs/rt (get_reg GCC bare-number path) equals xN form.
- Doc contract: (none on encode_bgtu; get_reg bare-number path)
- Seed: encode_bgt_pbt.rs encode_bgt_imm_as_reg
- Formal: ∀ n,m ∈ 0..31, tgt. encode_bgtu([Imm(n),Imm(m),Symbol(tgt)]) = encode_bgtu([xN(n),xN(m),Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, tgt]
  domain: { n: reg_num, m: reg_num, tgt: ident }
  relation:
    op: eq
    lhs: "sut_reloc([Imm(n), Imm(m), Symbol(tgt)])"
    rhs: "sut_reloc([xN(n), xN(m), Symbol(tgt)])"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: ident }
evidence: get_reg Imm 0..31 path
```

## encode_bgtu_neg_arity
- Tier: 3
- Rationale: Negative/error — under-arity (<3 operands) must Err (llvm-mc rejects too few operands; README three-operand form).
- Doc contract: README.md:330 "| `bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants            |" — domain-restriction fingerprint 100b43c8
- Seed: encode_bgt_pbt.rs encode_bgt_neg_arity
- Formal: ∀ ops. |ops| < 3 ⇒ encode_bgtu(ops) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: holds
    expr: encode_bgtu(&ops).is_err()
generators:
  ops: { gen: short_ops }
expected_error: String
evidence: README.md:330; llvm-mc rejects too few operands
```

## encode_bgtu_neg_invalid_regs
- Tier: 3
- Rationale: Negative/error — invalid rs or rt must Err.
- Doc contract: (none on encode_bgtu; get_reg rejects non-GPR — domain-restriction)
- Seed: encode_bgt_pbt.rs encode_bgt_neg_invalid_regs
- Formal: ∀ bad ∉ GPR, good ∈ GPR, tgt. encode_bgtu([bad,good,tgt])=Err ∧ encode_bgtu([good,bad,tgt])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, tgt]
  domain: { bad: invalid_rs, good: gpr_name, tgt: ident }
  body: "encode_bgtu([bad,good,tgt]).is_err() ∧ encode_bgtu([good,bad,tgt]).is_err()"
generators:
  bad: { gen: invalid_rs }
  good: { gen: gpr_name }
  tgt: { gen: ident }
expected_error: String
evidence: get_reg contract
```

## encode_bgtu_neg_invalid_target
- Tier: 3
- Rationale: Negative/error — target kinds outside Symbol/Label/Imm/Reg must Err.
- Doc contract: pseudo.rs:383 "_ => Err(format!(\"expected branch target at operand {}\", idx))," — domain-restriction fingerprint da4c41b9
- Seed: encode_bgt_pbt.rs encode_bgt_neg_invalid_target
- Formal: ∀ rs,rt ∈ GPR, bad ∉ {Symbol,Label,Imm,Reg}. encode_bgtu([rs,rt,bad]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bgtu
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, rt, bad]
  domain: { rs: gpr_name, rt: gpr_name, bad: invalid_target }
  relation:
    op: holds
    expr: encode_bgtu([rs,rt,bad]).is_err()
generators:
  rs: { gen: gpr_name }
  rt: { gen: gpr_name }
  bad: { gen: invalid_target }
expected_error: String
evidence: pseudo.rs:383
```

## encode_bgtu_neg_extra
- Tier: 3
- Rationale: Negative/error — trailing fourth operand must be rejected (README three-operand form; llvm-mc errors). Family defect known on siblings (encode_bgt/ble/…).
- Doc contract: README.md:330 "| `bgt/ble/bgtu/bleu` | Swapped-operand `blt`/`bge` variants            |" — asserted fingerprint 100b43c8
- Seed: encode_bgt_pbt.rs encode_bgt_neg_extra; INVARIANTS.md family bug
- Formal: ∀ rs,rt ∈ GPR, tgt ∈ Idents, extra ∈ Operand. encode_bgtu([rs,rt,Symbol(tgt),extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtu_pbt.rs
- Status: failing
- Counterexample: rs="zero", rt="zero", tgt="foo", extra=Reg("zero")
- Bug report: bug_reports/encode_bgtu_extra_operand.md

```property
function: encode_bgtu
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, rt, tgt, extra]
  domain: { rs: gpr_name, rt: gpr_name, tgt: ident, extra: extra_operand }
  relation:
    op: holds
    expr: encode_bgtu([rs,rt,Symbol(tgt),extra]).is_err()
generators:
  rs: { gen: gpr_name }
  rt: { gen: gpr_name }
  tgt: { gen: ident }
  extra: { gen: extra_operand }
expected_error: String
evidence: README.md:330; llvm-mc rejects extra operand
```
