# Properties: encode_alu_reg_w (requested encode_op32)

## encode_alu_reg_w_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree OP-32 decoder). encode_r / encode_alu_reg / C.ADDW rejected as primary differential (same-job gate: private packer / OP 64-bit / compressed).
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_diff_llvm_mc
- Formal: ∀ mn ∈ OP32, rd, rs1, rs2 ∈ GPRNames. encode_alu_reg_w([Reg(rd), Reg(rs1), Reg(rs2)], funct3(mn), funct7(mn)) = Word(w) ∧ w = llvm-mc("mn rd, rs1, rs2")
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg_w
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: op32_mnemonic, rd: gpr_name, rs1: gpr_name, rs2: gpr_name }
  relation:
    op: eq
    lhs: encode_alu_reg_w([Reg(rd), Reg(rs1), Reg(rs2)], funct3(mn), funct7(mn))
    rhs: llvm_mc("mn rd, rs1, rs2")
generators:
  mn: { gen: string }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
evidence: src/backend/riscv/assembler/README.md:297-298 addw/subw/sllw/srlw/sraw/mulw/divw/remw are R-type; encoder/mod.rs:569-638 dispatch to encode_alu_reg_w
```

## encode_alu_reg_w_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout. Stronger differential is the sibling property; this pins opcode OP_OP_32 and field placement independently of llvm-mc.
- Doc contract: encoder/mod.rs:294 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_r_type_fields
- Formal: ∀ mn ∈ OP32, rd, rs1, rs2 ∈ 0..31. let w = encode_alu_reg_w([Reg(x{rd}), Reg(x{rs1}), Reg(x{rs2})], funct3(mn), funct7(mn)) in Word form. unpack_r(w) = (opcode=0b0111011, funct3(mn), rd, rs1, rs2, funct7(mn))
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg_w
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: op32_mnemonic, rd: u32_0_31, rs1: u32_0_31, rs2: u32_0_31 }
  relation:
    op: eq
    lhs: unpack_r(encode_alu_reg_w([Reg(x{rd}), Reg(x{rs1}), Reg(x{rs2})], funct3(mn), funct7(mn)))
    rhs: (0b0111011, funct3(mn), rd, rs1, rs2, funct7(mn))
generators:
  mn: { gen: string }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:294 R-type layout; encoder/mod.rs:351 OP_OP_32; README.md:352
```

## encode_alu_reg_w_abi_xn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names, xN, and fp=s0/x8 name the same GPR. Stronger differential is the sibling property; this is a behavior-preserving rename.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_abi_xn_alias
- Formal: ∀ n, m, k ∈ 0..31, mn ∈ OP32. encode_alu_reg_w([Reg(x{n}), Reg(x{m}), Reg(x{k})], f3, f7) = encode_alu_reg_w([Reg(ABI[n]), Reg(ABI[m]), Reg(ABI[k])], f3, f7). When n=8 or m=8 or k=8, Reg("fp") agrees.
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg_w
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, k, mn]
  domain: { n: u32_0_31, m: u32_0_31, k: u32_0_31, mn: op32_mnemonic }
  relation:
    op: eq
    lhs: encode_alu_reg_w([Reg(x{n}), Reg(x{m}), Reg(x{k})], f3, f7)
    rhs: encode_alu_reg_w([Reg(ABI[n]), Reg(ABI[m]), Reg(ABI[k])], f3, f7)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
  mn: { gen: string }
evidence: parser.rs:22 GPR names; encoder/mod.rs:166-208 ABI and xN plus fp=s0
```

## encode_alu_reg_w_imm_as_reg
- Tier: 4
- Rationale: Metamorphic under get_reg's documented GCC bare-register-number contract: Imm n in 0..=31 encodes as x{n}. Stronger differential is the sibling property.
- Doc contract: encoder/mod.rs:370 "GCC sometimes emits bare register numbers (0-31) in inline asm" — caller precondition (get_reg) fingerprint f1b1a1fb
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_imm_as_reg
- Formal: ∀ n, m, k ∈ 0..31, mn ∈ OP32. encode_alu_reg_w([Imm(n), Imm(m), Imm(k)], f3, f7) = encode_alu_reg_w([Reg(x{n}), Reg(x{m}), Reg(x{k})], f3, f7)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg_w
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, k, mn]
  domain: { n: u32_0_31, m: u32_0_31, k: u32_0_31, mn: op32_mnemonic }
  relation:
    op: eq
    lhs: encode_alu_reg_w([Imm(n), Imm(m), Imm(k)], f3, f7)
    rhs: encode_alu_reg_w([Reg(x{n}), Reg(x{m}), Reg(x{k})], f3, f7)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
  mn: { gen: string }
evidence: encoder/mod.rs:370 GCC bare register numbers 0-31
```

## encode_alu_reg_w_neg_extra
- Tier: 5
- Rationale: Negative/error contract: llvm-mc rejects a fourth operand (`invalid operand for instruction`). The encoder claims to encode textual assembly as llvm-mc does. Stronger oracles do not apply to invalid arity.
- Doc contract: encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_neg_extra
- Formal: ∀ mn ∈ OP32, rd, rs1, rs2 ∈ GPRNames, extra ∈ Operand. encode_alu_reg_w([Reg(rd), Reg(rs1), Reg(rs2), extra], f3, f7) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: failing
- Counterexample: encode_alu_reg_w([Reg("x0"), Reg("x0"), Reg("x0"), Imm(0)], 0, 0) -> Ok(Word(59))
- Bug report: bug_reports/encode_alu_reg_w_extra_operand.md

```property
function: encoder.encode_alu_reg_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, extra]
  domain: { mn: op32_mnemonic, rd: gpr_name, rs1: gpr_name, rs2: gpr_name, extra: extra_operand }
  relation:
    op: throws
    expr: encode_alu_reg_w([Reg(rd), Reg(rs1), Reg(rs2), extra], f3, f7)
expected_error: String
generators:
  mn: { gen: string }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  extra: { gen: string }
evidence: llvm-mc rejects addw x1, x2, x3, x4; README.md:6-7 textual assembly as emitted by codegen
```

## encode_alu_reg_w_neg_arity_fp
- Tier: 5
- Rationale: Negative/error contract: empty/missing operands and FP/non-GPR 3rd operands are not integer registers. get_reg returns Err for those. Stronger oracles do not apply to invalid inputs.
- Doc contract: encoder/mod.rs:370 "GCC sometimes emits bare register numbers (0-31) in inline asm" — caller precondition (get_reg) fingerprint f1b1a1fb
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_neg_arity_fp
- Formal: ∀ mn ∈ OP32, rd, rs1, rs2 ∈ GPRNames, fp ∈ FpNames, bad ∈ NonRegOperand. encode_alu_reg_w([], f3, f7) = Err ∧ encode_alu_reg_w([Reg(rd)], f3, f7) = Err ∧ encode_alu_reg_w([Reg(rd), Reg(rs1)], f3, f7) = Err ∧ encode_alu_reg_w([Reg(fp), Reg(rs1), Reg(rs2)], f3, f7) = Err ∧ encode_alu_reg_w([Reg(rd), Reg(fp), Reg(rs2)], f3, f7) = Err ∧ encode_alu_reg_w([Reg(rd), Reg(rs1), Reg(fp)], f3, f7) = Err ∧ encode_alu_reg_w([Reg(rd), Reg(rs1), bad], f3, f7) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, fp, bad]
  domain: { mn: op32_mnemonic, rd: gpr_name, rs1: gpr_name, rs2: gpr_name, fp: fp_name, bad: non_reg_operand }
  relation:
    op: throws
    expr: encode_alu_reg_w([], f3, f7)
expected_error: String
generators:
  mn: { gen: string }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  fp: { gen: string }
  bad: { gen: string }
evidence: encoder/mod.rs:367-375 get_reg expects integer register or Imm 0-31
```

## encode_alu_reg_w_neg_oob_imm
- Tier: 5
- Rationale: Negative/error contract: get_reg accepts Imm only in 0..=31. Values outside that window must Err. Documented bound sampled at -1, 32, and extremes.
- Doc contract: encoder/mod.rs:370 "GCC sometimes emits bare register numbers (0-31) in inline asm" — caller precondition (get_reg) fingerprint f1b1a1fb
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_neg_oob_imm
- Formal: ∀ mn ∈ OP32, rd, rs1 ∈ GPRNames, imm ∈ i64 \ [0, 31]. encode_alu_reg_w([Reg(rd), Reg(rs1), Imm(imm)], f3, f7) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, imm]
  domain: { mn: op32_mnemonic, rd: gpr_name, rs1: gpr_name, imm: i64_outside_0_31 }
  relation:
    op: throws
    expr: encode_alu_reg_w([Reg(rd), Reg(rs1), Imm(imm)], f3, f7)
expected_error: String
generators:
  mn: { gen: string }
  rd: { gen: string }
  rs1: { gen: string }
  imm: { gen: int, type: i64 }
evidence: encoder/mod.rs:370-371 Imm accepted iff 0 <= n <= 31
```

## encode_alu_reg_w_neg_invalid_name
- Tier: 5
- Rationale: Negative/error contract: names that are not integer GPRs (x32, foo, v0, xzr, w0) must Err at rd, rs1, or rs2. Stronger oracles do not apply to invalid register names.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: encode_alu_reg_pbt.rs:encode_alu_reg_neg_invalid_name
- Formal: ∀ mn ∈ OP32, rd, rs1, rs2 ∈ GPRNames, bad ∈ {x32, x33, x99, foo, v0, v31, xzr, w0}, which ∈ {0,1,2}. encode_alu_reg_w(ops with ops[which]=Reg(bad), f3, f7) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_alu_reg_w_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_alu_reg_w
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, bad, which]
  domain: { mn: op32_mnemonic, rd: gpr_name, rs1: gpr_name, rs2: gpr_name, bad: invalid_gpr_name, which: 0..2 }
  relation:
    op: throws
    expr: encode_alu_reg_w(ops_with_bad_at(which), f3, f7)
expected_error: String
generators:
  mn: { gen: string }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  bad: { gen: string }
  which: { gen: int, min: 0, max: 2, type: u32 }
evidence: encoder/mod.rs:367-369 invalid integer register Err; parser.rs:22 GPR set
```
