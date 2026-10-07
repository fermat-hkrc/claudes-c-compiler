# Properties: encode_sext_w

## encode_sext_w_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle for the documented expansion is encoding agreement with llvm-mc assembling `sext.w rd, rs`, an independent assembler of the same ADDIW-with-imm-0 word the README and inline comment claim. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SEXT.W/ADDIW decoder). Differential vs encode_alu_imm_w(addiw) rejected as primary — shared encode_i/get_reg (used as a weaker metamorphic instead). llvm-mc `sext.w` uses the same ADDIW imm=0 encoding this assembler documents, so encoding-equality vs llvm-mc `sext.w` is a matching-contract pair.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs:334 llvm-mc KAT `sext.w x1, x2`
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR. encode_sext_w([Reg(rd), Reg(rs)]) = llvm-mc("sext.w rd, rs")
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sext_w
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_sext_w([Reg(rd), Reg(rs)]).word
    rhs: llvm_mc("sext.w rd, rs").word
generators:
  rd: { gen: string }
  rs: { gen: string }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:254; src/backend/riscv/assembler/README.md:323; encoder/mod.rs:867
```

## encode_sext_w_diff_llvm_mc_addiw
- Tier: 5
- Rationale: Metamorphic/differential required at standard tier. README.md:323 and pseudo.rs:254 document `sext.w rd, rs` → `addiw rd, rs, 0`. Encoding agreement with llvm-mc assembling that expansion is an independent check of the same contract (llvm-mc may pretty-print the word as `sext.w`, but the bytes must match). State machine rejected. Round-trip rejected (no decoder). Differential vs in-tree encode_alu_imm_w rejected as primary (shared encode_i).
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: src/backend/riscv/assembler/encoder/encode_alu_imm_w_pbt.rs:327 encode_alu_imm_w_kat_llvm_mc_addiw_zero
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR. encode_sext_w([Reg(rd), Reg(rs)]) = llvm-mc("addiw rd, rs, 0")
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sext_w
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_sext_w([Reg(rd), Reg(rs)]).word
    rhs: llvm_mc("addiw rd, rs, 0").word
generators:
  rd: { gen: string }
  rs: { gen: string }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:254; src/backend/riscv/assembler/README.md:323; RISC-V Unprivileged ISA SEXT.W = ADDIW rd, rs, 0
```

## encode_sext_w_eq_addiw_0
- Tier: 4
- Rationale: Documented expansion is ADDIW with imm=0. encode_alu_imm_w(funct3=000) is the same-job sibling for addiw, used as a weaker metamorphic because it shares encode_i/get_reg with encode_sext_w. Stronger llvm-mc differentials cover independence.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ rs ∈ 0..31. encode_sext_w([Reg(xN(rd)), Reg(xN(rs))]) = encode_alu_imm_w([Reg(xN(rd)), Reg(xN(rs)), Imm(0)], 0)
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sext_w
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  relation:
    op: eq
    lhs: encode_sext_w([Reg(xN(rd)), Reg(xN(rs))]).word
    rhs: encode_alu_imm_w([Reg(xN(rd)), Reg(xN(rs)), Imm(0)], 0).word
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/pseudo.rs:254; src/backend/riscv/assembler/README.md:323
```

## encode_sext_w_isa_fields
- Tier: 4
- Rationale: RISC-V I-type ADDIW layout is an exact structural invariant: opcode OP-IMM-32=0011011, funct3=000, imm12=0, rd and rs1 in their fields. Stronger llvm-mc differentials already pin the whole word; this isolates each field including bounds 0 and 31.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ rs ∈ 0..31. let w = encode_sext_w([Reg(xN(rd)), Reg(xN(rs))]). w[6:0]=0011011 ∧ w[11:7]=rd ∧ w[14:12]=000 ∧ w[19:15]=rs ∧ w[31:20]=0
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sext_w
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  relation:
    op: holds
    lhs: isa_i_type_addiw_imm0(encode_sext_w([Reg(xN(rd)), Reg(xN(rs))]).word, rd, rs)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:396 I-type; encoder/mod.rs:446 OP_OP_IMM_32; RISC-V Unprivileged ISA ADDIW
```

## encode_sext_w_abi_xn_alias
- Tier: 4
- Rationale: ABI names, xN, fp/s0, zero/x0, and Imm(0..=31) (get_reg GCC bare-number path) must encode identically. Metamorphic under register-name aliasing. Stronger llvm-mc already covers ABI and xN independently.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: (none)
- Formal: ∀ n ∈ 0..31, ∀ m ∈ 0..31. encode_sext_w([Reg(ABI(n)), Reg(ABI(m))]) = encode_sext_w([Reg(xN(n)), Reg(xN(m))]) = encode_sext_w([Imm(n), Imm(m)]) ∧ (n=8 ⇒ fp alias) ∧ (n=0 ⇒ zero alias)
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sext_w
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: u32_0_31, m: u32_0_31 }
  relation:
    op: eq
    lhs: encode_sext_w([Reg(ABI(n)), Reg(ABI(m))]).word
    rhs: encode_sext_w([Reg(xN(n)), Reg(xN(m))]).word
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:463 get_reg; encoder/mod.rs:476 Imm(0..=31)
```

## encode_sext_w_neg_arity
- Tier: 3
- Rationale: README documents two-operand form `sext.w rd, rs`. llvm-mc rejects too few operands ("too few operands for instruction"). Missing operands must return Err. Negative/error contract; stronger value oracles do not reach this path.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: (none)
- Formal: ∀ ops. len(ops) < 2 ⇒ encode_sext_w(ops) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sext_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: short_ops }
  relation:
    op: throws
    lhs: encode_sext_w(ops)
expected_error: String
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
evidence: src/backend/riscv/assembler/README.md:323 two-operand form; llvm-mc rejects missing operand
```

## encode_sext_w_neg_invalid
- Tier: 3
- Rationale: get_reg rejects non-GPR names (FP, vector, unknown) and Imm outside 0..=31. Invalid operands at rd or rs must return Err. llvm-mc rejects FP dest. Negative/error contract.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: (none)
- Formal: ∀ bad ∉ GPR, ∀ good ∈ GPR. encode_sext_w([bad, bad]) is Err ∧ encode_sext_w([bad, Reg(good)]) is Err ∧ encode_sext_w([Reg(good), bad]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sext_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad, good, which]
  domain: { bad: invalid_operand, good: gpr_name, which: 0..2 }
  relation:
    op: throws
    lhs: encode_sext_w(placed(bad, good, which))
expected_error: String
generators:
  bad: { gen: string }
  good: { gen: string }
  which: { gen: int, min: 0, max: 2, type: u8 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:463 get_reg; llvm-mc rejects FP dest
```

## encode_sext_w_neg_extra
- Tier: 3
- Rationale: README documents exactly two operands `sext.w rd, rs`. llvm-mc rejects a third operand ("invalid operand for instruction"). Extra operands must return Err. The function never checks operands.len(), so this is the documented-arity error path.
- Doc contract: src/backend/riscv/assembler/encoder/pseudo.rs:254 "Ok(EncodeResult::Word(encode_i(OP_OP_IMM_32, rd, 0, rs1, 0))) // addiw rd, rs1, 0" — asserted fingerprint 9557309c
- Seed: encode_negw_pbt.rs extra-operand property (same two-operand pseudo pattern)
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR, ∀ extra. encode_sext_w([Reg(rd), Reg(rs), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sext_w_pbt.rs
- Status: failing
- Counterexample: rd="zero", rs="zero", extra=Reg("zero") → Ok(Word(0x0000001b))
- Bug report: bug_reports/encode_sext_w_extra_operand.md

```property
function: encoder.encode_sext_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: extra_operand }
  relation:
    op: throws
    lhs: encode_sext_w([Reg(rd), Reg(rs), extra])
expected_error: String
generators:
  rd: { gen: string }
  rs: { gen: string }
  extra: { gen: string }
evidence: src/backend/riscv/assembler/README.md:323 two-operand form; llvm-mc rejects extra operand
```
