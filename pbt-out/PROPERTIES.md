# Properties: encode_seqz

## encode_seqz_diff_llvm_mc
- Tier: 5
- Rationale: Strongest runnable oracle is differential vs independent llvm-mc (RISC-V assembler). State machine N/A (pure). Round-trip N/A (no decoder). encode_alu_imm shares encode_i/get_reg so is metamorphic, not primary differential.
- Doc contract: src/backend/riscv/assembler/README.md:324 "`seqz rd, rs` → `sltiu rd, rs, 1`" — asserted fingerprint bfb844fb
- Seed: (none) — pattern generalized from encode_not_pbt
- Formal: ∀ rd, rs ∈ GPRNames. word(encode_seqz([Reg(rd), Reg(rs)])) = llvm_mc("seqz rd, rs")
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "encode_seqz_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"seqz {}, {}\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: src/backend/riscv/assembler/README.md:324
```

## encode_seqz_diff_llvm_mc_sltiu
- Tier: 5
- Rationale: Documented expansion `sltiu rd, rs, 1` must match the same machine word as seqz under llvm-mc (independent of in-tree encode_alu_imm).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:260 "sltiu rd, rs1, 1" — asserted fingerprint d520aff8
- Seed: encode_not_diff_llvm_mc_xori
- Formal: ∀ rd, rs ∈ GPRNames. word(encode_seqz([Reg(rd), Reg(rs)])) = llvm_mc("sltiu rd, rs, 1")
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "encode_seqz_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"sltiu {}, {}, 1\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: src/backend/riscv/assembler/README.md:324
```

## encode_seqz_eq_sltiu_1
- Tier: 4
- Rationale: Algebraic metamorphic — documented expansion equals in-tree SLTIU (funct3=011, imm=1). Weaker than llvm-mc differential (shared helpers) but pins the project-local expansion.
- Doc contract: src/backend/riscv/assembler/README.md:324 — asserted fingerprint bfb844fb
- Seed: encode_not_eq_xori_m1
- Formal: ∀ rd, rs ∈ 0..31. encode_seqz([xN(rd), xN(rs)]) = encode_alu_imm([xN(rd), xN(rs), Imm(1)], 0b011)
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: "0..=31", rs: "0..=31" }
  relation:
    op: eq
    lhs: "encode_seqz_word([xN(rd), xN(rs)])"
    rhs: "encode_alu_imm_word([xN(rd), xN(rs), Imm(1)], 0b011)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:324
```

## encode_seqz_isa_fields
- Tier: 4
- Rationale: I-type invariant for SLTIU: opcode OP-IMM, funct3=011, imm12=1, rd/rs1 fields.
- Doc contract: pseudo.rs:260 "sltiu rd, rs1, 1" — asserted fingerprint d520aff8
- Seed: encode_not_isa_fields
- Formal: ∀ rd, rs ∈ 0..31. let w = encode_seqz([xN(rd), xN(rs)]). opcode(w)=0010011 ∧ rd(w)=rd ∧ funct3(w)=011 ∧ rs1(w)=rs ∧ imm12(w)=1
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: "0..=31", rs: "0..=31" }
  body: "opcode/rd/funct3/rs1/imm12 match SLTIU rd,rs,1"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:260
```

## encode_seqz_abi_xn_alias
- Tier: 4
- Rationale: ABI names, xN, fp/s0, zero/x0, and Imm(0..=31) bare numbers must encode identically.
- Doc contract: README.md:324 two-operand form — asserted fingerprint bfb844fb
- Seed: encode_not_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31. encode_seqz(ABI(n),ABI(m)) = encode_seqz(xN(n),xN(m)) = encode_seqz(Imm(n),Imm(m)) (and fp/zero aliases when n/m ∈ {0,8})
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: "0..=31", m: "0..=31" }
  relation:
    op: eq
    lhs: "encode_seqz_word(ABI)"
    rhs: "encode_seqz_word(xN)"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:465
```

## encode_seqz_field_isolation
- Tier: 4
- Rationale: Strengthening — rd bits independent of rs1; non-rd bits independent of rd.
- Doc contract: pseudo.rs:260 I-type layout — asserted fingerprint d520aff8
- Seed: encode_not_field_isolation
- Formal: ∀ rd, rs_a, rs_b. rd_field(seqz(rd,rs_a))=rd_field(seqz(rd,rs_b)); ∀ rd_a, rd_b, rs. (seqz(rd_a,rs) & ~rd_mask) = (seqz(rd_b,rs) & ~rd_mask)
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs_a, rs_b, rd_a, rd_b, rs]
  domain: { all: "0..=31" }
  body: "rd field independent of rs1; non-rd bits independent of rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs_a: { gen: int, min: 0, max: 31, type: u32 }
  rs_b: { gen: int, min: 0, max: 31, type: u32 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:398
```

## encode_seqz_neg_arity
- Tier: 3
- Rationale: Negative/error — fewer than 2 operands must Err (get_reg missing).
- Doc contract: README.md:324 two-operand `seqz rd, rs` — asserted fingerprint bfb844fb
- Seed: encode_not_neg_arity
- Formal: ∀ ops. |ops| < 2 ⇒ encode_seqz(ops) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len < 2" }
  relation:
    op: holds
    expr: encode_seqz(ops).is_err()
generators:
  ops: { gen: short_ops }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:324
```

## encode_seqz_neg_invalid
- Tier: 3
- Rationale: Non-GPR / invalid operand kinds must Err.
- Doc contract: get_reg expects integer register — mod.rs:465 — other fingerprint 4a1a5b19
- Seed: encode_not_neg_invalid
- Formal: ∀ bad ∈ InvalidOperand, good ∈ GPRNames, which ∈ {both,bad_rd,bad_rs}. encode_seqz(ops(which)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.pseudo.encode_seqz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, which]
  domain: { bad: invalid_operand, good: gpr_name, which: "0..=2" }
  relation:
    op: holds
    expr: encode_seqz(ops).is_err()
generators:
  bad: { gen: invalid_operand }
  good: { gen: gpr_name }
  which: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:465
```

## encode_seqz_neg_extra
- Tier: 3
- Rationale: README documents exactly two operands; llvm-mc rejects a third. encode_seqz currently only reads indices 0 and 1 — fails the property (same class as encode_not/encode_mv extra-operand bugs).
- Doc contract: README.md:324 `seqz rd, rs` (two operands) — asserted fingerprint bfb844fb
- Seed: encode_not_neg_extra
- Formal: ∀ rd, rs ∈ GPRNames, extra ∈ Operand. encode_seqz([Reg(rd), Reg(rs), extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_seqz_pbt.rs
- Status: failing
- Counterexample: encode_seqz([Reg("zero"), Reg("zero"), Reg("zero")]) → Ok(Word(0x00103013))
- Bug report: bug_reports/encode_seqz_extra_operand.md

```property
function: encoder.pseudo.encode_seqz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: extra_operand }
  relation:
    op: holds
    expr: encode_seqz([rd, rs, extra]).is_err()
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
  extra: { gen: extra_operand }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:324
```
