# Properties: encode_not

## encode_not_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle for the documented expansion is encoding agreement with llvm-mc assembling `not rd, rs`, an independent assembler of the same XORI-with-minus-one word the README claims. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree NOT/XORI decoder). Differential vs encode_alu_imm(xori) rejected as primary — shared encode_i/get_reg (used as a weaker metamorphic instead). Unlike MV, llvm-mc `not` uses the same XORI -1 encoding this assembler documents, so encoding-equality vs llvm-mc `not` is a matching-contract pair.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: (none) — no project-owned encode_not unit test; llvm-mc KAT `not a0, a1` = 0xfff5c513
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR. encode_not([Reg(rd), Reg(rs)]) = llvm-mc("not rd, rs")
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_not([Reg(rd), Reg(rs)]).word
    rhs: llvm_mc("not rd, rs").word
generators:
  rd: { gen: string }
  rs: { gen: string }
evidence: src/backend/riscv/assembler/README.md:320; pseudo.rs:236; encoder/mod.rs:860
```

## encode_not_diff_llvm_mc_xori
- Tier: 5
- Rationale: Metamorphic/differential required at standard tier. README.md:320 documents `not rd, rs` → `xori rd, rs, -1`. Encoding agreement with llvm-mc assembling that expansion is an independent check of the same contract (llvm-mc may pretty-print the word as `not`, but the bytes must match). State machine rejected. Round-trip rejected (no decoder). Differential vs in-tree encode_alu_imm rejected as primary (shared encode_i).
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: (none)
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR. encode_not([Reg(rd), Reg(rs)]) = llvm-mc("xori rd, rs, -1")
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_not([Reg(rd), Reg(rs)]).word
    rhs: llvm_mc("xori rd, rs, -1").word
generators:
  rd: { gen: string }
  rs: { gen: string }
evidence: src/backend/riscv/assembler/README.md:320; RISC-V Unprivileged ISA NOT = XORI rd, rs, -1
```

## encode_not_eq_xori_m1
- Tier: 4
- Rationale: Documented expansion is XORI with imm=-1. encode_alu_imm(funct3=100) is the same-job sibling for xori, used as a weaker metamorphic because it shares encode_i/get_reg with encode_not. Stronger llvm-mc differentials cover independence.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ rs ∈ 0..31. encode_not([Reg(xN(rd)), Reg(xN(rs))]) = encode_alu_imm([Reg(xN(rd)), Reg(xN(rs)), Imm(-1)], 0b100)
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  relation:
    op: eq
    lhs: encode_not([Reg(xN(rd)), Reg(xN(rs))]).word
    rhs: encode_alu_imm([Reg(xN(rd)), Reg(xN(rs)), Imm(-1)], 0b100).word
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:320; encoder/mod.rs:598 "xori" => encode_alu_imm(operands, 0b100)
```

## encode_not_isa_fields
- Tier: 4
- Rationale: Documented expansion is I-type XORI (opcode OP-IMM=0010011, funct3=100, imm12=-1). Field unpack from RISC-V I-type, not from the encoder body. Stronger encoding differential covers this domain; this pins the ISA layout including bounds rd/rs ∈ {0,31}.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ rs ∈ 0..31. encode_not = Word(w) ∧ opcode(w)=0010011 ∧ rd(w)=rd ∧ funct3(w)=100 ∧ rs1(w)=rs ∧ imm12(w)=0xFFF
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  relation:
    op: holds
    expr: is_xori_m1(encode_not([Reg(xN(rd)), Reg(xN(rs))]), rd, rs)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: RISC-V Unprivileged ISA I-type XORI; assembler/README.md:320
```

## encode_not_abi_xn_alias
- Tier: 4
- Rationale: ABI names, xN, fp/s0, and zero/x0 must encode the same register number. Imm(0..=31) is the documented GCC bare-number path in get_reg. Algebraic metamorphic over name aliases; stronger encoding differential already covers ABI/xN via llvm-mc.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: (none)
- Formal: ∀ n,m ∈ 0..31. encode_not([ABI(n), ABI(m)]) = encode_not([xN(n), xN(m)]) = encode_not([Imm(n), Imm(m)]) ∧ (n=8 ⇒ encode_not([fp, xN(m)]) equals) ∧ (m=8 ⇒ encode_not([xN(n), fp]) equals)
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: u32_0_31, m: u32_0_31 }
  relation:
    op: eq
    lhs: encode_not([Reg(ABI(n)), Reg(ABI(m))]).word
    rhs: encode_not([Reg(xN(n)), Reg(xN(m))]).word
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:459-468 get_reg ABI/xN/Imm(0..=31); fp is s0/x8
```

## encode_not_field_isolation
- Tier: 4
- Rationale: Contract-surface sweep (coverage_gaps had no Rust .profraw; C++ reporter listed encode_not NOT LINKED). I-type layout from the RISC-V Unprivileged ISA: rd occupies bits [11:7] independently of rs1; opcode/funct3/rs1/imm occupy the other bits independently of rd. Stronger encoding differential already covers the word; this pins field isolation including bounds 0 and 31.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: encode_mv_field_isolation (same two-operand pseudo shape)
- Formal: ∀ rd,rs_a,rs_b ∈ 0..31. rd_field(encode_not(rd, rs_a)) = rd_field(encode_not(rd, rs_b)). ∀ rd_a,rd_b,rs ∈ 0..31. (encode_not(rd_a, rs) & ¬rd_mask) = (encode_not(rd_b, rs) & ¬rd_mask)
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs_a, rs_b, rd_a, rd_b, rs]
  domain: { rd: u32_0_31, rs_a: u32_0_31, rs_b: u32_0_31, rd_a: u32_0_31, rd_b: u32_0_31, rs: u32_0_31 }
  relation:
    op: holds
    expr: rd_independent_of_rs1(encode_not) && non_rd_independent_of_rd(encode_not)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs_a: { gen: int, min: 0, max: 31, type: u32 }
  rs_b: { gen: int, min: 0, max: 31, type: u32 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: RISC-V Unprivileged ISA I-type layout; assembler/README.md:320
```

## encode_not_neg_arity
- Tier: 3
- Rationale: README documents the two-operand form `not rd, rs`. llvm-mc rejects too few operands. Negative/error contract on arity < 2. Stronger oracles do not cover the error path.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: (none)
- Formal: ∀ ops. len(ops) < 2 ⇒ encode_not(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand_vec_len_lt_2 }
  relation:
    op: holds
    expr: encode_not(ops).is_err()
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:320 two-operand form; llvm-mc "too few operands"
```

## encode_not_neg_invalid
- Tier: 3
- Rationale: get_reg rejects non-GPR names (FP, vector, unknown, out-of-range Imm). llvm-mc rejects FP dest/src for `not`. Negative/error contract on invalid operands at either position.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: (none)
- Formal: ∀ bad ∈ InvalidOperand, ∀ good ∈ GPR, ∀ which ∈ {0,1,both}. encode_not(ops_with_bad) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, which]
  domain: { bad: invalid_operand, good: gpr_name, which: 0..2 }
  relation:
    op: holds
    expr: encode_not(place_bad(bad, good, which)).is_err()
generators:
  bad: { gen: string }
  good: { gen: string }
  which: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: encoder/mod.rs:459-468 get_reg; llvm-mc rejects fa0/non-GPR
```

## encode_not_neg_extra
- Tier: 3
- Rationale: README documents exactly two operands. llvm-mc rejects a third operand ("invalid operand for instruction"). encode_not has no arity check and only reads operands 0 and 1, so extras are currently ignored — that is the contract under test, not a generator exclusion. Negative/error contract.
- Doc contract: pseudo.rs:236 "xori rd, rs1, -1" — asserted fingerprint da2c30d4
- Seed: encode_mv extra-operand property (same two-operand pseudo shape)
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR, ∀ extra. encode_not([Reg(rd), Reg(rs), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_not_pbt.rs
- Status: failing
- Counterexample: encode_not([Reg("zero"), Reg("zero"), Reg("zero")]) → Ok(Word(0xfff04013))
- Bug report: bug_reports/encode_not_extra_operand.md

```property
function: encoder.encode_not
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: extra_operand }
  relation:
    op: holds
    expr: encode_not([Reg(rd), Reg(rs), extra]).is_err()
generators:
  rd: { gen: string }
  rs: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:320 two-operand form; llvm-mc rejects extra operand
```
