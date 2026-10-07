# Properties: encode_sltz

## encode_sltz_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc (independent assembler). State machine rejected (pure encoder). Round-trip rejected (no SLTZ decoder). encode_alu_reg SLT rejected as primary (shared encode_r/get_reg).
- Doc contract: src/backend/riscv/assembler/README.md:326 "`sltz rd, rs` | `slt rd, rs, x0`" — asserted fingerprint d7effe57
- Seed: encode_snez_pbt.rs (sibling pseudo pattern)
- Formal: ∀ rd, rs ∈ GPRNames. encode_sltz([Reg(rd), Reg(rs)]) = Word(w) ∧ llvm-mc("sltz rd, rs") = w
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "sut_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"sltz {}, {}\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: src/backend/riscv/assembler/README.md:326
```

## encode_sltz_diff_llvm_mc_slt
- Tier: 5
- Rationale: Documented expansion `slt rd, rs, x0` must match llvm-mc independently of the SLTZ mnemonic.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:272 "// slt rd, rs1, x0" — asserted fingerprint 5677673f
- Seed: encode_snez_diff_llvm_mc_sltu
- Formal: ∀ rd, rs ∈ GPRNames. encode_sltz([Reg(rd), Reg(rs)]) = llvm-mc("slt rd, rs, x0")
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "sut_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"slt {}, {}, x0\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:272
```

## encode_sltz_eq_slt_x0
- Tier: 4
- Rationale: Metamorphic — SLTZ equals in-tree encode_alu_reg SLT with rs2=x0 (weaker than llvm-mc but checks shared expansion path).
- Doc contract: src/backend/riscv/assembler/README.md:326 "`sltz rd, rs` | `slt rd, rs, x0`" — asserted fingerprint d7effe57
- Seed: encode_snez_eq_sltu_x0
- Formal: ∀ rd, rs ∈ 0..31. encode_sltz([xN(rd), xN(rs)]) = encode_alu_reg([xN(rd), xN(rs), x0], funct3=010, funct7=0)
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: reg_num, rs: reg_num }
  relation:
    op: eq
    lhs: "sut_word([xN(rd), xN(rs)])"
    rhs: "slt_word([xN(rd), xN(rs), x0])"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:326
```

## encode_sltz_isa_fields
- Tier: 4
- Rationale: R-type OP layout invariant from ISA: opcode=OP, funct3=SLT=010, funct7=0, rs2=x0.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:397 "R-type: funct7|rs2|rs1|funct3|rd|opcode" — other fingerprint 4a1a5b19
- Seed: encode_snez_isa_fields
- Formal: ∀ rd, rs ∈ 0..31. let w = encode_sltz([xN(rd), xN(rs)]).word in (w&0x7F=0b0110011 ∧ rd_field=rd ∧ funct3=0b010 ∧ rs1=rs ∧ rs2=0 ∧ funct7=0)
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: reg_num, rs: reg_num }
  relation:
    op: holds
    expr: "fields(w) match SLT rd, rs, x0"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:397
```

## encode_sltz_abi_xn_alias
- Tier: 4
- Rationale: ABI names, xN, fp/s0, zero/x0, and Imm(0..31) bare numbers must alias to the same encoding.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:469 get_reg accepts Reg and Imm(0..=31) — other fingerprint 4a1a5b19
- Seed: encode_snez_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31. encode_sltz(ABI/xN/Imm forms of n,m) all equal
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: reg_num, m: reg_num }
  relation:
    op: eq
    lhs: "sut_word(abi_forms(n,m))"
    rhs: "sut_word(xn_forms(n,m))"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:469
```

## encode_sltz_field_isolation
- Tier: 4
- Rationale: rd bits independent of rs1; non-rd bits independent of rd.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:397 R-type field layout — other fingerprint 4a1a5b19
- Seed: encode_snez_field_isolation
- Formal: ∀ rd, rs_a, rs_b. rd_field(sltz(rd,rs_a))=rd_field(sltz(rd,rs_b)); ∀ rd_a, rd_b, rs. (sltz(rd_a,rs) & ~rd_mask) = (sltz(rd_b,rs) & ~rd_mask)
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs_a, rs_b, rd_a, rd_b, rs]
  domain: { all: reg_num }
  relation:
    op: holds
    expr: "rd field isolated from rs1; non-rd bits isolated from rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs_a: { gen: int, min: 0, max: 31, type: u32 }
  rs_b: { gen: int, min: 0, max: 31, type: u32 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:397
```

## encode_sltz_neg_arity
- Tier: 3
- Rationale: Fewer than 2 operands must Err (README two-operand form; get_reg fails).
- Doc contract: src/backend/riscv/assembler/README.md:326 "`sltz rd, rs`" — asserted fingerprint d7effe57
- Seed: encode_snez_neg_arity
- Formal: ∀ ops. |ops| < 2 ⇒ encode_sltz(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: throws
    expr: "encode_sltz(ops)"
generators:
  ops: { gen: short_ops }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:326
```

## encode_sltz_neg_invalid
- Tier: 3
- Rationale: Non-GPR / invalid operand kinds must Err.
- Doc contract: src/backend/riscv/assembler/README.md:326 "`sltz rd, rs`" (GPR) — asserted fingerprint d7effe57
- Seed: encode_snez_neg_invalid
- Formal: ∀ bad ∈ InvalidOperand, good ∈ GPRNames, which ∈ {both,bad_rd,bad_rs}. encode_sltz(ops(which)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sltz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, which]
  domain: { bad: invalid_operand, good: gpr_name, which: 0..2 }
  relation:
    op: throws
    expr: "encode_sltz(ops(which, bad, good))"
generators:
  bad: { gen: invalid_operand }
  good: { gen: gpr_name }
  which: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:326
```

## encode_sltz_neg_extra
- Tier: 3
- Rationale: Exactly two operands per README and llvm-mc; extra operands must Err.
- Doc contract: src/backend/riscv/assembler/README.md:326 "`sltz rd, rs`" — asserted fingerprint d7effe57
- Seed: encode_snez_neg_extra
- Formal: ∀ rd, rs ∈ GPRNames, ∀ extra. encode_sltz([Reg(rd), Reg(rs), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sltz_pbt.rs
- Status: failing
- Counterexample: encode_sltz([Reg("zero"), Reg("zero"), Reg("zero")]) → Ok(Word(0x00002033))
- Bug report: bug_reports/encode_sltz_extra_operand.md

```property
function: encode_sltz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: extra_operand }
  relation:
    op: throws
    expr: "encode_sltz([Reg(rd), Reg(rs), extra])"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
  extra: { gen: extra_operand }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:326
```
