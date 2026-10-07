# Properties: encode_mv

## encode_mv_diff_llvm_mc_add
- Tier: 5
- Rationale: Strongest oracle for the documented expansion is encoding agreement with llvm-mc assembling `add rd, x0, rs`, an independent assembler of the same ADD word the README claims. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree MV/ADD decoder). Encoding-equality vs llvm-mc `mv` rejected as primary — llvm-mc expands MV to ADDI (`addi rd, rs, 0`) while README.md:319 and pseudo.rs:228-229 claim ADD for RV64C/C.MV eligibility (different encoding contract; semantic agreement is a separate property). Differential vs encode_alu_reg(add) rejected as primary — shared encode_r/get_reg (used as a weaker metamorphic instead).
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: (none) — no project-owned encode_mv unit test; llvm-mc KAT `add a0, x0, a1` = 0x00b00533
- Formal: ∀ rd ∈ GPR, ∀ rs ∈ GPR. encode_mv([Reg(rd), Reg(rs)]) = llvm-mc("add rd, x0, rs")
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mv
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_mv([Reg(rd), Reg(rs)]).word
    rhs: llvm_mc("add rd, x0, rs").word
generators:
  rd: { gen: string }
  rs: { gen: string }
evidence: src/backend/riscv/assembler/README.md:319; pseudo.rs:228-229; encoder/mod.rs:857
```

## encode_mv_sem_vs_llvm_mc_mv
- Tier: 5
- Rationale: Metamorphic/differential required at standard tier. llvm-mc `mv` uses ADDI while the SUT uses ADD; both must leave `rs` in `rd` (RISC-V MV semantics; x0 writes discarded). Independent interpreter of ADD/ADDI (not a copy of encode_mv). State machine rejected. Encoding-equality vs llvm-mc `mv` rejected (documented ADD vs ADDI). Round-trip rejected (no decoder).
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: (none)
- Formal: ∀ rd ∈ GPR\{x0}, ∀ rs ∈ GPR. sim(encode_mv([Reg(rd), Reg(rs)]))[rd] = sim(llvm-mc("mv rd, rs"))[rd] = init[rs]
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mv
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_nonzero, rs: gpr }
  relation:
    op: eq
    lhs: sim(encode_mv([Reg(xN(rd)), Reg(xN(rs))]))[rd]
    rhs: sim(llvm_mc("mv xN(rd), xN(rs)"))[rd]
generators:
  rd: { gen: int, min: 1, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: RISC-V Unprivileged ISA MV copies rs to rd; assembler/README.md:319 ADD expansion is semantically the copy
```

## encode_mv_isa_fields
- Tier: 4
- Rationale: Documented expansion is R-type ADD (opcode OP-OP, funct3=000, funct7=0000000, rs1=x0). Field unpack from RISC-V R-type, not from the encoder body. Stronger encoding differential covers this domain; this pins the ISA layout including bounds rd/rs ∈ {0,31}.
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: (none)
- Formal: ∀ rd ∈ 0..31, ∀ rs ∈ 0..31. encode_mv = Word(w) ∧ opcode(w)=0110011 ∧ rd(w)=rd ∧ funct3(w)=000 ∧ rs1(w)=0 ∧ rs2(w)=rs ∧ funct7(w)=0000000
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  relation:
    op: holds
    expr: is_add_x0(encode_mv([Reg(xN(rd)), Reg(xN(rs))]), rd, rs)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: RISC-V Unprivileged ISA R-type ADD; assembler/README.md:319; pseudo.rs:228-229
```

## encode_mv_abi_xn_alias
- Tier: 4
- Rationale: ABI names, xN, fp/s0, and zero/x0 are the same GPR. Metamorphic invariance under name alias. Stronger encoding differential already uses mixed names; this pins alias equality including Imm(0..=31) as GCC bare register numbers (get_reg).
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: (none)
- Formal: ∀ n,m ∈ 0..31. encode_mv([ABI(n), ABI(m)]) = encode_mv([xN(n), xN(m)]) ∧ encode_mv([Imm(n), Imm(m)]) = encode_mv([xN(n), xN(m)]) ∧ (n=8 ⇒ encode_mv([fp, xN(m)]) = encode_mv([xN(8), xN(m)]))
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: u32_0_31, m: u32_0_31 }
  relation:
    op: eq
    lhs: encode_mv([Reg(ABI(n)), Reg(ABI(m))]).word
    rhs: encode_mv([Reg(xN(n)), Reg(xN(m))]).word
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:457-465 get_reg ABI/xN/Imm(0..=31); parser.rs:22-23 register names
```

## encode_mv_field_isolation
- Tier: 4
- Rationale: R-type rd and rs2 occupy disjoint bit fields; changing one operand must not perturb the other field or the constant opcode/funct3/rs1/funct7 bits.
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: (none)
- Formal: ∀ rd, rs_a, rs_b, rd_a, rd_b, rs ∈ 0..31. rd_field(encode_mv(rd, rs_a)) = rd_field(encode_mv(rd, rs_b)) ∧ (encode_mv(rd_a, rs) & ~rd_mask) = (encode_mv(rd_b, rs) & ~rd_mask)
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs_a, rs_b, rd_a, rd_b, rs]
  domain: { rd: u32_0_31, rs_a: u32_0_31, rs_b: u32_0_31, rd_a: u32_0_31, rd_b: u32_0_31, rs: u32_0_31 }
  relation:
    op: holds
    expr: rd_field_independent_of_rs and rs_field_independent_of_rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs_a: { gen: int, min: 0, max: 31, type: u32 }
  rs_b: { gen: int, min: 0, max: 31, type: u32 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: RISC-V Unprivileged ISA R-type layout; assembler/README.md:319
```

## encode_mv_eq_add_x0
- Tier: 4
- Rationale: README expansion `add rd, x0, rs` is the same job as encode_alu_reg(funct3=000, funct7=0000000) on [rd, x0, rs]. Shared encode_r/get_reg weakens independence, so this is metamorphic not primary differential. llvm-mc ADD is the independent encoding check.
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: encode_neg_eq_sub_x0 in pseudo.rs encode_neg_pbt (same expansion shape)
- Formal: ∀ rd, rs ∈ 0..31. encode_mv([xN(rd), xN(rs)]) = encode_alu_reg([xN(rd), x0, xN(rs)], funct3=000, funct7=0000000)
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  relation:
    op: eq
    lhs: encode_mv([Reg(xN(rd)), Reg(xN(rs))]).word
    rhs: encode_alu_reg([Reg(xN(rd)), Reg(x0), Reg(xN(rs))], 0, 0).word
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: assembler/README.md:319; encoder/base.rs:260 encode_alu_reg ADD
```

## encode_mv_neg_extra
- Tier: 3
- Rationale: README documents a two-operand form; llvm-mc rejects a third operand. Negative/error contract: extra operand must Err. No documented "ignore extra" exclusion.
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: test_encode_neg_regression_extra_operand (pseudo.rs encode_neg_pbt)
- Formal: ∀ rd, rs ∈ GPR, ∀ extra. encode_mv([Reg(rd), Reg(rs), extra]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: failing
- Counterexample: encode_mv([Reg("zero"), Reg("zero"), Reg("zero")]) = Ok(Word(0x00000033))
- Bug report: pbt-out/bug_reports/encode_mv_extra_operand.md

```property
function: encoder.encode_mv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: operand }
  relation:
    op: throws
    expr: encode_mv([Reg(rd), Reg(rs), extra])
    error: String
generators:
  rd: { gen: string }
  rs: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: assembler/README.md:319 two-operand `mv rd, rs`; llvm-mc rejects extra
```

## encode_mv_neg_arity_invalid
- Tier: 3
- Rationale: llvm-mc rejects too-few operands and FP/non-GPR names. get_reg returns Err for missing/invalid slots. Documented error path.
- Doc contract: pseudo.rs:228-229 "Use `add rd, x0, rs` instead of `addi rd, rs, 0` so the instruction is eligible for RV64C compression to C.MV (which requires the ADD form)." — asserted fingerprint 12e6ded6
- Seed: encode_neg_neg_arity / encode_neg_neg_invalid
- Formal: ∀ ops. |ops|<2 ⇒ encode_mv(ops) is Err ∧ ∀ bad ∈ non-GPR. encode_mv([bad, good]) is Err ∧ encode_mv([good, bad]) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, bad, good]
  domain: { ops: short_ops, bad: invalid_operand, good: gpr_name }
  relation:
    op: throws
    expr: encode_mv(ops_or_mixed)
    error: String
generators:
  ops: { gen: string }
  bad: { gen: string }
  good: { gen: string }
expected_error: String
evidence: encoder/mod.rs:457-465 get_reg Err on missing/invalid; llvm-mc too few / invalid operand
```
