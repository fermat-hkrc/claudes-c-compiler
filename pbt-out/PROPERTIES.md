# Properties: encode_bltz

## encode_bltz_diff_llvm_mc
- Tier: 5
- Rationale: Strongest independent oracle is llvm-mc assembling `bltz rs, 0`. State machine N/A (pure). Round-trip N/A (no decoder). In-tree `encode_branch_instr(BLT)` shares `encode_b`/`get_reg` so used only as metamorphic, not primary differential.
- Doc contract: src/backend/riscv/assembler/README.md:329 "`blez/bgez/...`| Corresponding `bge`/`blt` with x0" — asserted fingerprint b10d7541
- Seed: encode_bgez_pbt.rs differential pattern (none for bltz specifically)
- Formal: ∀ rs ∈ GPRNames. encode_bltz([Reg(rs), Imm(0)]).word = llvm_mc("bltz rs, 0") ∧ reloc = Branch/"0"/0
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: "encode_bltz([Reg(rs), Imm(0)]).word"
    rhs: "llvm_mc(format!(\"bltz {}, 0\", rs))"
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_bltz_diff_llvm_mc_blt
- Tier: 5
- Rationale: Documented expansion BLTZ = BLT rs, x0 must match independent llvm-mc of the base form (strengthens the pseudo path beyond shared in-tree helpers).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:322 "// blt rs, x0" — asserted fingerprint 7b98a108
- Seed: (none)
- Formal: ∀ rs ∈ GPRNames. encode_bltz([Reg(rs), Imm(0)]).word = llvm_mc("blt rs, x0, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: differential
predicate:
  quantifier: forall
  vars: [rs]
  domain: { rs: gpr_name }
  relation:
    op: eq
    lhs: "encode_bltz([Reg(rs), Imm(0)]).word"
    rhs: "llvm_mc(format!(\"blt {}, x0, 0\", rs))"
generators:
  rs: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:322
```

## encode_bltz_eq_blt_rs_x0
- Tier: 4
- Rationale: Algebraic metamorphic — pseudo equals same-job base encoder on [rs, x0, label] (and zero alias). Weaker than llvm-mc differential (shared encode_b) but pins the documented expansion in-tree.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:322 "// blt rs, x0" — asserted fingerprint 7b98a108
- Seed: (none)
- Formal: ∀ rs ∈ 0..31, ∀ tgt ∈ LabelIdents. encode_bltz([xN(rs), Symbol(tgt)]) = encode_branch_instr(BLT, [xN(rs), x0, Symbol(tgt)]) = encode_branch_instr(BLT, [xN(rs), zero, Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: "0..=31", tgt: ident }
  relation:
    op: eq
    lhs: "encode_bltz([xN(rs), Symbol(tgt)])"
    rhs: "encode_branch_instr(BLT, [xN(rs), x0, Symbol(tgt)])"
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:322
```

## encode_bltz_isa_b_type
- Tier: 4
- Rationale: B-type invariant — opcode BRANCH, funct3=BLT(100), rs1=rs, rs2=x0, imm=0 deferred to Branch reloc.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:322 "// blt rs, x0" — asserted fingerprint 7b98a108
- Seed: (none)
- Formal: ∀ rs ∈ 0..31, ∀ tgt. let (w,k,s,a) = encode_bltz([xN(rs), Symbol(tgt)]). unpack_b(w) = (OP_BRANCH, 0b100, rs, 0, 0) ∧ k=Branch ∧ s=tgt ∧ a=0
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, tgt]
  domain: { rs: "0..=31", tgt: ident }
  body: "unpack_b(word)=(OP_BRANCH,0b100,rs,0,0) ∧ reloc=Branch/tgt/0"
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:318-324
```

## encode_bltz_abi_xn_alias
- Tier: 4
- Rationale: ABI names, xN, and fp/s0 aliases encode identically.
- Doc contract: src/backend/riscv/assembler/README.md:329 "`blez/bgez/...`| Corresponding `bge`/`blt` with x0" — asserted fingerprint b10d7541
- Seed: (none)
- Formal: ∀ n ∈ 0..31, ∀ tgt. encode_bltz([ABI(n), Symbol(tgt)]) = encode_bltz([xN(n), Symbol(tgt)]) ∧ (n=8 ⇒ fp form equal)
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: "0..=31", tgt: ident }
  relation:
    op: eq
    lhs: "encode_bltz([ABI(n), Symbol(tgt)])"
    rhs: "encode_bltz([xN(n), Symbol(tgt)])"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_bltz_target_forms
- Tier: 4
- Rationale: Symbol / Label / Reg-as-label targets with the same string are equivalent (get_branch_target contract).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:378 "Some(Operand::Symbol(s)) | Some(Operand::Label(s)) => Ok(s.clone())," — asserted fingerprint 0c04e437
- Seed: (none)
- Formal: ∀ rs, s. encode_bltz([xN(rs), Symbol(s)]) = encode_bltz([xN(rs), Label(s)]) = encode_bltz([xN(rs), Reg(s)])
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs, s]
  domain: { rs: "0..=31", s: ident }
  relation:
    op: eq
    lhs: "encode_bltz([xN(rs), Symbol(s)])"
    rhs: "encode_bltz([xN(rs), Label(s)])"
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  s: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:376
```

## encode_bltz_imm_target
- Tier: 4
- Rationale: Imm target stringifies into reloc symbol; word stays zero-imm BLT.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:379 "Some(Operand::Imm(v)) => Ok(format!("{}", v))," — asserted fingerprint ad06d32a
- Seed: (none)
- Formal: ∀ rs, imm. encode_bltz([xN(rs), Imm(imm)]) yields B-type BLT with imm_field=0, reloc.symbol=format!(imm), Branch, addend=0
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs, imm]
  domain: { rs: "0..=31", imm: i64_edge }
  body: "word is zero-imm BLT ∧ reloc.symbol == format!(imm)"
generators:
  rs: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -4096, max: 4094, type: i64 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:380
```

## encode_bltz_imm_as_rs
- Tier: 4
- Rationale: Imm(0..31) as rs matches xN (get_reg bare-number path).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:489 "Some(Operand::Imm(n)) if *n >= 0 && *n <= 31 => Ok(*n as u32)," — asserted fingerprint 4df5b8ba
- Seed: (none)
- Formal: ∀ n ∈ 0..31, ∀ tgt. encode_bltz([Imm(n), Symbol(tgt)]) = encode_bltz([xN(n), Symbol(tgt)])
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, tgt]
  domain: { n: "0..=31", tgt: ident }
  relation:
    op: eq
    lhs: "encode_bltz([Imm(n), Symbol(tgt)])"
    rhs: "encode_bltz([xN(n), Symbol(tgt)])"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  tgt: { gen: string, type: String }
evidence: src/backend/riscv/assembler/encoder/mod.rs:481
```

## encode_bltz_neg_arity
- Tier: 3
- Rationale: Too few operands must Err (llvm-mc rejects; two-operand form).
- Doc contract: src/backend/riscv/assembler/README.md:329 "`blez/bgez/...`| Corresponding `bge`/`blt` with x0" — asserted fingerprint b10d7541
- Seed: (none)
- Formal: ∀ ops. len(ops) < 2 ⇒ encode_bltz(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len<2" }
  relation:
    op: throws
    expr: "encode_bltz(ops)"
generators:
  ops: { gen: list, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:329
```

## encode_bltz_neg_invalid_rs
- Tier: 3
- Rationale: Invalid/FP/non-GPR rs must Err.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:490 "expected register at operand {}, got {:?}" — asserted fingerprint 783b781e
- Seed: (none)
- Formal: ∀ bad ∉ GPR, ∀ tgt. encode_bltz([bad, Symbol(tgt)]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, tgt]
  domain: { bad: invalid_rs, tgt: ident }
  relation:
    op: throws
    expr: "encode_bltz([bad, Symbol(tgt)])"
generators:
  bad: { gen: string, type: Operand }
  tgt: { gen: string, type: String }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:481
```

## encode_bltz_neg_invalid_target
- Tier: 3
- Rationale: Non Symbol/Label/Imm/Reg targets must Err.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:383 "_ => Err(format!("expected branch target at operand {}", idx))," — asserted fingerprint da4c41b9
- Seed: (none)
- Formal: ∀ rs ∈ GPR, ∀ bad ∉ {Symbol,Label,Imm,Reg}. encode_bltz([Reg(rs), bad]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_bltz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, bad]
  domain: { rs: gpr_name, bad: invalid_target }
  relation:
    op: throws
    expr: "encode_bltz([Reg(rs), bad])"
generators:
  rs: { gen: string, type: String }
  bad: { gen: string, type: Operand }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:376
```

## encode_bltz_neg_extra
- Tier: 3
- Rationale: Extra operand must Err — README two-operand form; llvm-mc rejects. Sibling encode_bgez/blez already filed the same class of bug.
- Doc contract: src/backend/riscv/assembler/README.md:329 "`blez/bgez/...`| Corresponding `bge`/`blt` with x0" — asserted fingerprint b10d7541
- Seed: encode_bgez_neg_extra / bug encode_bgez_extra_operand
- Formal: ∀ rs ∈ GPR, ∀ tgt, ∀ extra. encode_bltz([Reg(rs), Symbol(tgt), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_bltz_pbt.rs
- Status: failing
- Counterexample: encode_bltz([Reg("zero"), Symbol("foo"), Reg("zero")]) → Ok (expected Err)
- Bug report: bug_reports/encode_bltz_extra_operand.md

```property
function: encoder.encode_bltz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs, tgt, extra]
  domain: { rs: gpr_name, tgt: ident, extra: operand }
  relation:
    op: throws
    expr: "encode_bltz([Reg(rs), Symbol(tgt), extra])"
generators:
  rs: { gen: string, type: String }
  tgt: { gen: string, type: String }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:329
```
