# Properties: encode_snez

## encode_snez_diff_llvm_mc
- Tier: 5
- Rationale: Strongest applicable is Differential vs independent llvm-mc assembling `snez rd, rs`. State machine rejected (pure encoder). Algebraic round-trip rejected (no in-tree SNEZ/SLTU decoder). encode_alu_reg(sltu) shares encode_r/get_reg so is metamorphic, not primary differential. README.md:325 and RISC-V ISA pin SNEZ = SLTU rd, x0, rs; llvm-mc is an independent assembler reference.
- Doc contract: src/backend/riscv/assembler/README.md:325 "`snez rd, rs` → `sltu rd, x0, rs`" — asserted fingerprint 1d0a48e1
- Seed: encode_seqz_pbt.rs encode_seqz_diff_llvm_mc (sibling two-operand set-flag pseudo)
- Formal: ∀ rd, rs ∈ GPRNames. encode_snez([Reg(rd), Reg(rs)]) = Word(w) ∧ llvm_mc("snez rd, rs") = w
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "sut_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"snez {}, {}\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: src/backend/riscv/assembler/README.md:325
```

## encode_snez_diff_llvm_mc_sltu
- Tier: 5
- Rationale: Differential vs llvm-mc on the documented expansion `sltu rd, x0, rs` (independent of the SNEZ mnemonic path). Strengthens the expansion contract separately from the mnemonic.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:266 "// sltu rd, x0, rs2" — asserted fingerprint 7a1fc959
- Seed: encode_seqz_pbt.rs encode_seqz_diff_llvm_mc_sltiu
- Formal: ∀ rd, rs ∈ GPRNames. encode_snez([Reg(rd), Reg(rs)]) = Word(w) ∧ llvm_mc("sltu rd, x0, rs") = w
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: "sut_word([Reg(rd), Reg(rs)])"
    rhs: "llvm_mc_word(format!(\"sltu {}, x0, {}\", rd, rs))"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:266
```

## encode_snez_eq_sltu_x0
- Tier: 4
- Rationale: Algebraic metamorphic — same-job expansion via encode_alu_reg(SLTU) on [rd, x0, rs]. Shared encode_r so weaker than llvm-mc differential; still checks the expansion wiring.
- Doc contract: src/backend/riscv/assembler/README.md:325 "`snez rd, rs` → `sltu rd, x0, rs`" — asserted fingerprint 1d0a48e1
- Seed: encode_negw_pbt encode_negw_eq_subw_x0
- Formal: ∀ rd, rs ∈ 0..31. encode_snez([xN(rd), xN(rs)]) = encode_alu_reg([xN(rd), x0, xN(rs)], funct3=0b011, funct7=0)
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: "0..=31", rs: "0..=31" }
  relation:
    op: eq
    lhs: "sut_word([xN(rd), xN(rs)])"
    rhs: "sltu_word([xN(rd), x0, xN(rs)])"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:325
```

## encode_snez_isa_fields
- Tier: 4
- Rationale: Algebraic invariant — R-type OP layout for SLTU rd, x0, rs2: opcode=0110011, funct3=011, funct7=0000000, rs1=0, rd/rs2 in fields. Documented bounds 0 and 31 sampled exactly via reg_num.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:266 "// sltu rd, x0, rs2" — asserted fingerprint 7a1fc959
- Seed: encode_negw_pbt R-type fields
- Formal: ∀ rd, rs ∈ 0..31. let w = encode_snez([xN(rd), xN(rs)]). w[6:0]=0110011 ∧ w[11:7]=rd ∧ w[14:12]=011 ∧ w[19:15]=0 ∧ w[24:20]=rs ∧ w[31:25]=0000000
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: "0..=31", rs: "0..=31" }
  body: "opcode/funct3/funct7/rs1=0/rd/rs2 fields of sut_word match SLTU rd,x0,rs"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:395
```

## encode_snez_abi_xn_alias
- Tier: 4
- Rationale: Algebraic metamorphic — ABI names, xN, fp/s0, zero/x0, and Imm(0..=31) bare-number path must yield the same word.
- Doc contract: src/backend/riscv/assembler/README.md:325 "`snez rd, rs` → `sltu rd, x0, rs`" — asserted fingerprint 1d0a48e1
- Seed: encode_seqz_pbt encode_seqz_abi_xn_alias
- Formal: ∀ n,m ∈ 0..31. encode_snez([ABI(n), ABI(m)]) = encode_snez([xN(n), xN(m)]) = encode_snez([Imm(n), Imm(m)]) (and fp/zero aliases when n or m ∈ {0,8})
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: "0..=31", m: "0..=31" }
  body: "ABI/xN/fp/zero/Imm aliases produce equal words"
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:467
```

## encode_snez_field_isolation
- Tier: 4
- Rationale: Algebraic invariant — rd bits independent of rs2; non-rd bits independent of rd.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:266 "// sltu rd, x0, rs2" — asserted fingerprint 7a1fc959
- Seed: encode_seqz_pbt encode_seqz_field_isolation
- Formal: ∀ rd, rs_a, rs_b. rd-field(encode_snez(rd,rs_a)) = rd-field(encode_snez(rd,rs_b)); ∀ rd_a, rd_b, rs. (encode_snez(rd_a,rs) & ~rd_mask) = (encode_snez(rd_b,rs) & ~rd_mask)
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs_a, rs_b, rd_a, rd_b, rs]
  domain: { rd: "0..=31", rs_a: "0..=31", rs_b: "0..=31", rd_a: "0..=31", rd_b: "0..=31", rs: "0..=31" }
  body: "rd field independent of rs2; non-rd bits independent of rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs_a: { gen: int, min: 0, max: 31, type: u32 }
  rs_b: { gen: int, min: 0, max: 31, type: u32 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:395
```

## encode_snez_neg_arity
- Tier: 3
- Rationale: Negative/error — fewer than 2 operands must Err (README two-operand form; llvm-mc "too few operands").
- Doc contract: src/backend/riscv/assembler/README.md:325 "`snez rd, rs`" — domain-restriction fingerprint 1d0a48e1
- Seed: encode_seqz_pbt encode_seqz_neg_arity
- Formal: ∀ ops. |ops| < 2 ⇒ encode_snez(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "len < 2" }
  relation:
    op: holds
    expr: "encode_snez(ops).is_err()"
generators:
  ops: { gen: short_ops }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:325
```

## encode_snez_neg_invalid
- Tier: 3
- Rationale: Negative/error — non-GPR / FP / Mem / Csr / out-of-range Imm must Err.
- Doc contract: src/backend/riscv/assembler/README.md:325 "`snez rd, rs`" (GPR operands) — domain-restriction fingerprint 1d0a48e1
- Seed: encode_seqz_pbt encode_seqz_neg_invalid
- Formal: ∀ bad ∈ InvalidOperand, good ∈ GPRNames, which ∈ {0,1,2}. encode_snez(ops(which,bad,good)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_snez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, which]
  domain: { bad: invalid_operand, good: gpr_name, which: "0..=2" }
  relation:
    op: holds
    expr: "encode_snez(ops).is_err()"
generators:
  bad: { gen: invalid_operand }
  good: { gen: gpr_name }
  which: { gen: int, min: 0, max: 2, type: u8 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:325
```

## encode_snez_neg_extra
- Tier: 3
- Rationale: Negative/error — a third operand must be rejected. README documents exactly two operands; llvm-mc rejects `snez a0, a1, a2`. Same class as the encode_seqz extra-operand bug.
- Doc contract: src/backend/riscv/assembler/README.md:325 "`snez rd, rs`" — domain-restriction fingerprint 1d0a48e1
- Seed: encode_seqz_pbt encode_seqz_neg_extra / bug_reports/encode_seqz_extra_operand.md
- Formal: ∀ rd, rs ∈ GPRNames, ∀ extra. encode_snez([Reg(rd), Reg(rs), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_snez_pbt.rs
- Status: failing
- Counterexample: encode_snez([Reg("zero"), Reg("zero"), Reg("zero")]) → Ok(Word(0x00003033))
- Bug report: bug_reports/encode_snez_extra_operand.md

```property
function: encoder.encode_snez
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: extra_operand }
  relation:
    op: holds
    expr: "encode_snez([Reg(rd), Reg(rs), extra]).is_err()"
generators:
  rd: { gen: gpr_name }
  rs: { gen: gpr_name }
  extra: { gen: extra_operand }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:325
```
