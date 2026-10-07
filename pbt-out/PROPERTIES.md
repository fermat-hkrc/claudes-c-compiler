# Properties: encode_sgtz

## encode_sgtz_diff_llvm_mc
- Tier: 5
- Rationale: Strongest independent oracle is differential vs llvm-mc assembling `sgtz rd, rs`. State machine rejected (pure encoder). Round-trip rejected (no in-tree SGTZ decoder). Primary differential vs encode_alu_reg(slt) rejected as primary because shared encode_r/get_reg; kept as weaker metamorphic.
- Doc contract: README.md:327 "`sgtz rd, rs`  | `slt rd, x0, rs`" — asserted fingerprint e67ed4eb
- Seed: encode_sltz_pbt.rs / encode_snez_pbt.rs (sibling differential pattern)
- Formal: ∀ rd, rs ∈ GPRNames. encode_sgtz([Reg(rd), Reg(rs)]) = Word(w) ∧ llvm_mc("sgtz rd, rs") = w
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "sut_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"sgtz {}, {}\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: README.md:327; pseudo.rs:278; encoder/mod.rs:879
```

## encode_sgtz_diff_llvm_mc_slt
- Tier: 5
- Rationale: Documented expansion `sgtz` = `slt rd, x0, rs` must match llvm-mc's independent encoding of the real instruction (not only the pseudo).
- Doc contract: README.md:327 "`sgtz rd, rs`  | `slt rd, x0, rs`" — asserted fingerprint e67ed4eb
- Seed: encode_sltz_pbt.rs encode_sltz_diff_llvm_mc_slt
- Formal: ∀ rd, rs ∈ GPRNames. encode_sgtz([Reg(rd), Reg(rs)]) = Word(w) ∧ llvm_mc("slt rd, x0, rs") = w
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "sut_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"slt {}, x0, {}\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: README.md:327; RISC-V Unprivileged ISA SGTZ
```

## encode_sgtz_eq_slt_x0
- Tier: 4
- Rationale: Algebraic metamorphic — documented expansion equals in-tree encode_alu_reg(SLT) with rs1=x0. Weaker than llvm-mc (shared helpers) but pins the expansion contract inside the crate.
- Doc contract: pseudo.rs:278 "// slt rd, x0, rs2" — asserted fingerprint 0889d026
- Seed: encode_snez_pbt.rs encode_snez_eq_sltu_x0
- Formal: ∀ rd, rs ∈ 0..31. encode_sgtz([x(rd), x(rs)]) = encode_alu_reg([x(rd), x0, x(rs)], funct3=010, funct7=0)
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: "0..=31", rs: "0..=31" }
  relation:
    op: eq
    lhs: "sut_word([Reg(xN(rd)), Reg(xN(rs))])"
    rhs: "slt_word([Reg(xN(rd)), Reg(\"x0\"), Reg(xN(rs))])"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: pseudo.rs:278; README.md:327
```

## encode_sgtz_isa_fields
- Tier: 4
- Rationale: Algebraic invariant — R-type OP layout for SLT with rs1 fixed to x0. Documents opcode/funct3/funct7/rs1 and bounds 0 and 31.
- Doc contract: pseudo.rs:278 "// slt rd, x0, rs2" — asserted fingerprint 0889d026
- Seed: encode_snez_pbt.rs encode_snez_isa_fields
- Formal: ∀ rd, rs ∈ 0..31. let w = encode_sgtz([x(rd), x(rs)]).word in (w&0x7F=OP) ∧ ((w>>7)&0x1F=rd) ∧ ((w>>12)&7=010) ∧ ((w>>15)&0x1F=0) ∧ ((w>>20)&0x1F=rs) ∧ ((w>>25)&0x7F=0)
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: "0..=31", rs: "0..=31" }
  body: "word fields match R-type SLT with rs1=x0"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: pseudo.rs:278; RISC-V ISA R-type SLT
```

## encode_sgtz_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic — ABI names, xN, fp/s0, zero/x0, and Imm(0..=31) bare-number path must yield identical encodings.
- Doc contract: README.md:327 "`sgtz rd, rs`" — asserted fingerprint 6645c7f6
- Seed: encode_snez_pbt.rs encode_snez_abi_xn_alias
- Formal: ∀ n, m ∈ 0..31. encode_sgtz(ABI(n), ABI(m)) = encode_sgtz(xN(n), xN(m)) = encode_sgtz(Imm(n), Imm(m)) (and fp/zero aliases when applicable)
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: "0..=31", m: "0..=31" }
  body: "ABI/xN/fp/zero/Imm aliases agree"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: get_reg Imm 0..=31 path; ABI table
```

## encode_sgtz_field_isolation
- Tier: 4
- Rationale: Metamorphic — rd bits independent of rs2; non-rd bits independent of rd.
- Doc contract: (none) — structural R-type packing invariant
- Seed: encode_snez_pbt.rs encode_snez_field_isolation
- Formal: ∀ rd, rs_a, rs_b. rd_field(encode(rd,rs_a))=rd_field(encode(rd,rs_b)); ∀ rd_a, rd_b, rs. (encode(rd_a,rs) & ~rd_mask) = (encode(rd_b,rs) & ~rd_mask)
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs_a, rs_b, rd_a, rd_b, rs]
  domain: { each: "0..=31" }
  body: "rd independent of rs2; non-rd bits independent of rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: encode_r packing
```

## encode_sgtz_neg_arity
- Tier: 3
- Rationale: Negative/error — fewer than 2 operands must Err (get_reg bounds; llvm-mc rejects missing operand).
- Doc contract: README.md:327 "`sgtz rd, rs`" — asserted fingerprint 6645c7f6
- Seed: encode_snez_pbt.rs encode_snez_neg_arity
- Formal: ∀ ops. len(ops) < 2 ⇒ encode_sgtz(ops) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len < 2" }
  relation:
    op: holds
    expr: "encode_sgtz(ops).is_err()"
generators:
  ops: { gen: short_ops }
expected_error: String
evidence: README.md:327; llvm-mc rejects `sgtz a0`
```

## encode_sgtz_neg_invalid
- Tier: 3
- Rationale: Negative/error — FP/vector/invalid names and non-GPR operand kinds must Err.
- Doc contract: README.md:327 "`sgtz rd, rs`" — asserted fingerprint 6645c7f6
- Seed: encode_snez_pbt.rs encode_snez_neg_invalid
- Formal: ∀ bad ∈ InvalidOperand, good ∈ GPRNames, which ∈ {0,1,2}. encode_sgtz(ops(which, bad, good)) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sgtz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, which]
  domain: { bad: invalid_operand, good: gpr_name, which: "0..=2" }
  relation:
    op: holds
    expr: "encode_sgtz(ops).is_err()"
generators:
  bad: { gen: invalid_operand }
  good: { gen: gpr_name }
expected_error: String
evidence: get_reg rejects non-GPR; llvm-mc rejects `sgtz fa0, a1`
```

## encode_sgtz_neg_extra
- Tier: 3
- Rationale: Negative/error — README documents two-operand form; llvm-mc rejects a third operand. encode_sgtz must Err on extra operand (same contract as sibling sltz/snez/seqz bugs).
- Doc contract: README.md:327 "`sgtz rd, rs`" — asserted fingerprint 6645c7f6
- Seed: encode_sltz_pbt.rs encode_sltz_neg_extra (known sibling bug class)
- Formal: ∀ rd, rs ∈ GPRNames, extra ∈ Operand. encode_sgtz([Reg(rd), Reg(rs), extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_sgtz_pbt.rs
- Status: failing
- Counterexample: encode_sgtz([Reg("zero"), Reg("zero"), Reg("zero")])
- Bug report: bug_reports/encode_sgtz_extra_operand.md

```property
function: encode_sgtz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: any_operand }
  relation:
    op: holds
    expr: "encode_sgtz([Reg(rd), Reg(rs), extra]).is_err()"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
  extra: { gen: extra_operand }
expected_error: String
evidence: README.md:327; llvm-mc rejects `sgtz a0, a1, a2`
```
