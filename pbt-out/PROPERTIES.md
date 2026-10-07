# Properties: encode_negw

## encode_negw_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle for the documented expansion is encoding agreement with llvm-mc assembling `negw rd, rs`, an independent assembler of the same SUBW-with-x0 word the README claims. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree NEGW/SUBW decoder). Differential vs encode_alu_reg_w(subw) rejected as primary — shared encode_r/get_reg (used as a weaker metamorphic instead). Differential vs encode_neg rejected — different job (SUB / OP vs SUBW / OP-32). llvm-mc `negw` uses the same SUBW x0 encoding this assembler documents, so encoding-equality vs llvm-mc `negw` is a matching-contract pair.
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: (none) — no project-owned encode_negw unit test; llvm-mc KAT `negw a0, a1` = 0x40b0053b
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR. encode_negw([Reg(rd), Reg(rs)]) = llvm-mc("negw rd, rs")
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_negw
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_negw([Reg(rd), Reg(rs)]).word
    rhs: llvm_mc("negw rd, rs").word
generators:
  rd: { gen: string }
  rs: { gen: string }
evidence: src/backend/riscv/assembler/README.md:322; encoder/mod.rs:864
```

## encode_negw_diff_llvm_mc_subw
- Tier: 5
- Rationale: Metamorphic/differential required at standard tier. README.md:322 documents `negw rd, rs` → `subw rd, x0, rs`. Encoding agreement with llvm-mc assembling that expansion is an independent check of the same contract (llvm-mc may pretty-print the word as `negw`, but the bytes must match). State machine rejected. Round-trip rejected (no decoder). Differential vs in-tree encode_alu_reg_w rejected as primary (shared encode_r).
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: (none)
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR. encode_negw([Reg(rd), Reg(rs)]) = llvm-mc("subw rd, x0, rs")
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_negw
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_negw([Reg(rd), Reg(rs)]).word
    rhs: llvm_mc("subw rd, x0, rs").word
generators:
  rd: { gen: string }
  rs: { gen: string }
evidence: src/backend/riscv/assembler/README.md:322; RISC-V Unprivileged ISA NEGW = SUBW rd, x0, rs
```

## encode_negw_eq_subw_x0
- Tier: 4
- Rationale: Documented expansion is SUBW with rs1=x0. encode_alu_reg_w(funct3=000, funct7=0100000) is the same-job sibling for subw, used as a weaker metamorphic because it shares encode_r/get_reg with encode_negw. Stronger llvm-mc differentials cover independence.
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ rs ∈ 0..31. encode_negw([Reg(xN(rd)), Reg(xN(rs))]) = encode_alu_reg_w([Reg(xN(rd)), Reg(x0), Reg(xN(rs))], 0b000, 0b0100000)
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_negw
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  relation:
    op: eq
    lhs: encode_negw([Reg(xN(rd)), Reg(xN(rs))]).word
    rhs: encode_alu_reg_w([Reg(xN(rd)), Reg(x0), Reg(xN(rs))], 0b000, 0b0100000).word
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:322
```

## encode_negw_isa_fields
- Tier: 4
- Rationale: RISC-V R-type SUBW layout is an exact structural invariant of the documented expansion: opcode OP-32=0b0111011, funct3=000, funct7=0100000, rs1=x0, rd and rs2 from the two operands. Stronger differentials cover the full word; this pins each field so a swapped rs1/rs2 or OP vs OP-32 mix-up cannot hide.
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ rs ∈ 0..31. let w = encode_negw([Reg(xN(rd)), Reg(xN(rs))]).word in (w & 0x7F = 0b0111011) ∧ ((w>>7)&0x1F = rd) ∧ ((w>>12)&7 = 0) ∧ ((w>>15)&0x1F = 0) ∧ ((w>>20)&0x1F = rs) ∧ ((w>>25)&0x7F = 0b0100000)
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_negw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  body: opcode(w)=OP_OP_32 and rd_field=rd and funct3=0 and rs1=0 and rs2=rs and funct7=0b0100000
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:322; encoder/mod.rs:389 R-type; encoder/mod.rs:445 OP_OP_32
```

## encode_negw_abi_xn_alias
- Tier: 4
- Rationale: get_reg accepts ABI names, xN, fp/s0, zero/x0, and Imm(0..=31). Same architectural register must encode identically regardless of spelling. Metamorphic under name transform; llvm-mc differentials already cover ABI and xN independently.
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: (none)
- Formal: ∀ n ∈ 0..31, ∀ m ∈ 0..31. encode_negw([Reg(xN(n)), Reg(xN(m))]) = encode_negw([Reg(ABI(n)), Reg(ABI(m))]) = encode_negw([Imm(n), Imm(m)]) ∧ (n=8 ⇒ also fp) ∧ (m=8 ⇒ also fp)
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_negw
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: u32_0_31, m: u32_0_31 }
  relation:
    op: eq
    lhs: encode_negw([Reg(xN(n)), Reg(xN(m))]).word
    rhs: encode_negw([Reg(ABI(n)), Reg(ABI(m))]).word
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:461 get_reg; README.md:322 two-operand GPR form
```

## encode_negw_neg_arity
- Tier: 3
- Rationale: README documents the two-operand form `negw rd, rs`. llvm-mc rejects too few operands. Missing operands must Err. Stronger oracles do not apply to the invalid domain.
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: (none)
- Formal: ∀ ops. |ops| < 2 ⇒ encode_negw(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_negw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand_vecs_len_lt_2 }
  relation:
    op: throws
    expr: encode_negw(ops)
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:322 two-operand form; llvm-mc rejects arity < 2
```

## encode_negw_neg_invalid
- Tier: 3
- Rationale: Non-GPR operands (FP, vector, symbol, mem, CSR, out-of-range Imm, invalid names) must be rejected. llvm-mc rejects FP dest/src. get_reg returns Err for those. Stronger oracles do not apply to the invalid domain.
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: (none)
- Formal: ∀ bad ∉ GPR, ∀ good ∈ GPR. encode_negw([bad, bad]) is Err ∧ encode_negw([bad, Reg(good)]) is Err ∧ encode_negw([Reg(good), bad]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_negw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good]
  domain: { bad: non_gpr_operand, good: gpr_name }
  relation:
    op: throws
    expr: encode_negw([bad, Reg(good)])
generators:
  bad: { gen: string }
  good: { gen: string }
expected_error: String
evidence: encoder/mod.rs:461 get_reg; llvm-mc rejects FP/non-GPR for negw
```

## encode_negw_neg_extra
- Tier: 3
- Rationale: README documents exactly two operands. llvm-mc rejects a third operand (`invalid operand for instruction`). Extra operands must Err. The body has no arity check (only get_reg 0 and 1), so this is the error-path property most likely to fail.
- Doc contract: (none) — encode_negw has no rustdoc or inline comment. Expansion asserted at src/backend/riscv/assembler/README.md:322 "`negw rd, rs` | `subw rd, x0, rs`" — asserted fingerprint 015cf432
- Seed: encode_not_pbt.rs extra-operand property; encode_neg extra-operand regression
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR, ∀ extra. encode_negw([Reg(rd), Reg(rs), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_negw_pbt.rs
- Status: failing
- Counterexample: encode_negw([Reg("zero"), Reg("zero"), Reg("zero")]) → Ok(Word(0x4000003b))
- Bug report: pbt-out/bug_reports/encode_negw_extra_operand.md

```property
function: encoder.encode_negw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: operand }
  relation:
    op: throws
    expr: encode_negw([Reg(rd), Reg(rs), extra])
generators:
  rd: { gen: string }
  rs: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:322 two-operand form; llvm-mc rejects extra operand
```
