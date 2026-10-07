# Properties: encode_bgtz

## encode_bgtz_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential vs independent llvm-mc RISC-V assembler. State machine N/A (pure). Round-trip N/A (no decoder). encode_branch_instr shared encode_b so used only as weaker metamorphic.
- Doc contract: src/backend/riscv/assembler/README.md:329 "| `blez/bgez/...`| Corresponding `bge`/`blt` with x0                    |" — asserted fingerprint dcddf900
- Seed: encode_blez_pbt.rs encode_blez_diff_llvm_mc
- Formal: ∀ rs ∈ GPRNames. encode_bgtz([Reg(rs), Imm(0)]).word = llvm_mc("bgtz rs, 0") ∧ reloc=(Branch,"0",0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: encode_bgtz([Reg(rs), Imm(0)]).word
    rhs: llvm_mc("bgtz " + rs + ", 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_bgtz_diff_llvm_mc_blt
- Tier: 5
- Rationale: Documented expansion BGTZ = BLT x0, rs must match llvm-mc of the base form.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:331 "word: encode_b(OP_BRANCH, 0b100, 0, rs2, 0), // blt x0, rs" — asserted fingerprint 4b76b768
- Seed: encode_blez_pbt.rs encode_blez_diff_llvm_mc_bge
- Formal: ∀ rs ∈ GPRNames. encode_bgtz([Reg(rs), Imm(0)]).word = llvm_mc("blt x0, rs, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: encode_bgtz([Reg(rs), Imm(0)]).word
    rhs: llvm_mc("blt x0, " + rs + ", 0")
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:331
```

## encode_bgtz_eq_blt_x0_rs
- Tier: 4
- Rationale: Metamorphic expansion identity vs encode_branch_instr(BLT) with [x0, rs, label].
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:331 "word: encode_b(OP_BRANCH, 0b100, 0, rs2, 0), // blt x0, rs" — asserted fingerprint 4b76b768
- Seed: encode_blez_pbt.rs encode_blez_eq_bge_x0_rs
- Formal: ∀ rs ∈ 0..31, tgt ∈ Idents. encode_bgtz([xN(rs), Symbol(tgt)]) = encode_branch_instr(BLT, [x0, xN(rs), Symbol(tgt)]) = encode_branch_instr(BLT, [zero, xN(rs), Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: u32_0_31, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgtz([Reg(xN(rs)), Symbol(tgt)])
    rhs: encode_branch_instr(BLT, [Reg(x0), Reg(xN(rs)), Symbol(tgt)])
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:331
```

## encode_bgtz_isa_b_type
- Tier: 4
- Rationale: B-type layout invariant from RISC-V ISA (independent unpacker).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:331 "word: encode_b(OP_BRANCH, 0b100, 0, rs2, 0), // blt x0, rs" — asserted fingerprint 4b76b768
- Seed: encode_blez_pbt.rs encode_blez_isa_b_type
- Formal: ∀ rs ∈ 0..31, tgt ∈ Idents. let w=encode_bgtz([xN(rs),Symbol(tgt)]).word in unpack_b(w)=(OP_BRANCH, BLT, rs1=0, rs2=rs, imm=0) ∧ reloc=(Branch,tgt,0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: u32_0_31, tgt: ident }
  body: unpack_b(encode_bgtz([xN(rs), Symbol(tgt)]).word) == (OP_BRANCH, 0b100, 0, rs, 0)
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:331
```

## encode_bgtz_abi_xn_alias
- Tier: 4
- Rationale: ABI name / xN / fp≡s0≡x8 aliases must encode identically.
- Doc contract: src/backend/riscv/assembler/README.md:329 "| `blez/bgez/...`| Corresponding `bge`/`blt` with x0                    |" — asserted fingerprint dcddf900
- Seed: encode_blez_pbt.rs encode_blez_abi_xn_alias
- Formal: ∀ n ∈ 0..31, tgt ∈ Idents. encode_bgtz([ABI(n),Symbol(tgt)]) = encode_bgtz([xN(n),Symbol(tgt)]) ∧ (n=8 ⇒ fp form equal)
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: u32_0_31, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgtz([Reg(abi(n)), Symbol(tgt)])
    rhs: encode_bgtz([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_bgtz_target_forms
- Tier: 4
- Rationale: get_branch_target treats Symbol/Label/Reg-as-label with same string equivalently.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:381 "Some(Operand::Reg(s)) => Ok(s.clone())," — asserted fingerprint 88c78fb0
- Seed: encode_blez_pbt.rs encode_blez_target_forms
- Formal: ∀ rs ∈ 0..31, s ∈ Idents. encode_bgtz([xN(rs),Symbol(s)]) = encode_bgtz([xN(rs),Label(s)]) = encode_bgtz([xN(rs),Reg(s)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, s]
  domain: { rs: u32_0_31, s: ident }
  relation:
    op: eq
    lhs: encode_bgtz([xN(rs), Symbol(s)])
    rhs: encode_bgtz([xN(rs), Label(s)])
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:381
```

## encode_bgtz_imm_target
- Tier: 4
- Rationale: Imm target stringifies into reloc; word stays zero-imm BLT.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:331 "word: encode_b(OP_BRANCH, 0b100, 0, rs2, 0), // blt x0, rs" — asserted fingerprint 4b76b768
- Seed: encode_blez_pbt.rs encode_blez_imm_target
- Formal: ∀ rs ∈ 0..31, imm ∈ ImmSamples. encode_bgtz([xN(rs),Imm(imm)]) yields B-type BLT rs1=0 rs2=rs imm=0 reloc=(Branch, format!("{}",imm), 0)
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, imm]
  domain: { rs: u32_0_31, imm: i64 }
  body: "reloc.symbol == format!(\"{}\", imm) && unpack imm==0"
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:331
```

## encode_bgtz_imm_as_rs
- Tier: 4
- Rationale: get_reg GCC bare-number path Imm(0..=31) equals xN.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:489 "Some(Operand::Imm(n)) if *n >= 0 && *n <= 31 => Ok(*n as u32)," — domain-restriction fingerprint 4df5b8ba
- Seed: encode_blez_pbt.rs encode_blez_imm_as_rs
- Formal: ∀ n ∈ 0..31, tgt ∈ Idents. encode_bgtz([Imm(n), Symbol(tgt)]) = encode_bgtz([xN(n), Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: u32_0_31, tgt: ident }
  relation:
    op: eq
    lhs: encode_bgtz([Imm(n), Symbol(tgt)])
    rhs: encode_bgtz([Reg(xN(n)), Symbol(tgt)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:489
```

## encode_bgtz_neg_arity
- Tier: 3
- Rationale: Two-operand form requires ≥2 operands; fewer must Err.
- Doc contract: src/backend/riscv/assembler/README.md:329 "| `blez/bgez/...`| Corresponding `bge`/`blt` with x0                    |" — asserted fingerprint dcddf900
- Seed: encode_blez_pbt.rs encode_blez_neg_arity
- Formal: ∀ ops. len(ops)<2 ⇒ encode_bgtz(ops)=Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: holds
    expr: encode_bgtz(ops).is_err()
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_bgtz_neg_invalid_rs
- Tier: 3
- Rationale: Non-GPR rs must Err via get_reg.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:486 "reg_num(name).ok_or_else(|| format!(\"invalid integer register: {}\", name))" — asserted fingerprint b41eabbb
- Seed: encode_blez_pbt.rs encode_blez_neg_invalid_rs
- Formal: ∀ bad ∉ GPR, tgt ∈ Idents. encode_bgtz([bad, Symbol(tgt)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, tgt]
  domain: { bad: invalid_rs, tgt: ident }
  relation:
    op: holds
    expr: encode_bgtz([bad, Symbol(tgt)]).is_err()
generators:
  bad: { gen: string, type: Operand }
  tgt: { gen: string, type: String }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:486
```

## encode_bgtz_neg_invalid_target
- Tier: 3
- Rationale: Target forms outside Symbol/Label/Imm/Reg must Err.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:384 "_ => Err(format!(\"expected branch target at operand {}\", idx))," — asserted fingerprint da4c41b9
- Seed: encode_blez_pbt.rs encode_blez_neg_invalid_target
- Formal: ∀ rs ∈ GPR, bad ∉ {Symbol,Label,Imm,Reg}. encode_bgtz([Reg(rs), bad])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bgtz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, bad]
  domain: { rs: gpr_name, bad: invalid_target }
  relation:
    op: holds
    expr: encode_bgtz([Reg(rs), bad]).is_err()
generators:
  rs: { gen: string, type: String }
  bad: { gen: string, type: Operand }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:384
```

## encode_bgtz_neg_extra
- Tier: 3
- Rationale: README two-operand form + llvm-mc reject extra operands; SUT must Err.
- Doc contract: src/backend/riscv/assembler/README.md:329 "| `blez/bgez/...`| Corresponding `bge`/`blt` with x0                    |" — asserted fingerprint dcddf900
- Seed: encode_blez_pbt.rs encode_blez_neg_extra
- Formal: ∀ rs ∈ GPR, tgt ∈ Idents, extra ∈ Operand. encode_bgtz([Reg(rs), Symbol(tgt), extra])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_bgtz_pbt.rs
- Status: failing
- Counterexample: encode_bgtz([Reg("a0"), Symbol("foo"), Reg("a1")]) → Ok(WordWithReloc { word: 0x00a04063 })
- Bug report: pbt-out/bug_reports/encode_bgtz_extra_operand.md

```property
function: encoder.encode_bgtz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, tgt, extra]
  domain: { rs: gpr_name, tgt: ident, extra: Operand }
  relation:
    op: holds
    expr: encode_bgtz([Reg(rs), Symbol(tgt), extra]).is_err()
generators:
  rs: { gen: string, type: String }
  tgt: { gen: string, type: String }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:329
```
