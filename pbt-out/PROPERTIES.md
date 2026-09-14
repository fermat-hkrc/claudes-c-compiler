# Properties: encode_neg

## encode_neg_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree NEG/SUB decoder. encode_negw rejected (same-job gate: SUBW / OP-32). encode_alu_reg(sub) shares encode_r/get_reg so is not an independent differential. SUT-boundary: internal-helper of the RISC-V assembler; mapping operands <-> `neg rd, rs`.
- Seed: (none)
- Formal: ∀ rd, rs ∈ GPRNames. encode_neg([Reg(rd), Reg(rs)]) = Word(w) ∧ llvm-mc(-triple=riscv64, "neg rd, rs") = w
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_neg([Reg(rd), Reg(rs)])
    rhs: llvm_mc_word("neg " + rd + ", " + rs)
generators:
  rd: { gen: oneof, items: ["zero","ra","sp","gp","tp","t0","t1","t2","s0","fp","s1","a0","a1","a2","a3","a4","a5","a6","a7","s2","s3","s4","s5","s6","s7","s8","s9","s10","s11","t3","t4","t5","t6","x0","x1","x8","x31"] }
  rs: { gen: oneof, items: ["zero","ra","sp","gp","tp","t0","t1","t2","s0","fp","s1","a0","a1","a2","a3","a4","a5","a6","a7","s2","s3","s4","s5","s6","s7","s8","s9","s10","s11","t3","t4","t5","t6","x0","x1","x8","x31"] }
evidence: src/backend/riscv/assembler/README.md:321 neg rd, rs = sub rd, x0, rs; encoder/mod.rs:749 "neg" => encode_neg
```

## encode_neg_diff_llvm_mc_sub
- Tier: 2
- Rationale: Contract-surface sweep. README.md:321 names the expansion `sub rd, x0, rs`; comparing encode_neg to llvm-mc's SUB (not the in-tree encode_alu_reg) is an independent differential of that expansion. Same stronger-oracle rejections as encode_neg_diff_llvm_mc.
- Seed: README.md:321; encode_neg_kat_llvm_mc sub a0, x0, a1
- Formal: ∀ rd, rs ∈ GPRNames. encode_neg([Reg(rd), Reg(rs)]) = llvm-mc(-triple=riscv64, "sub rd, x0, rs")
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_neg([Reg(rd), Reg(rs)])
    rhs: llvm_mc_word("sub " + rd + ", x0, " + rs)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:321 neg rd, rs = sub rd, x0, rs
```

## encode_neg_eq_sub_x0
- Tier: 4c
- Rationale: Documented expansion identity. Stronger differential vs llvm-mc is the sibling property above. This metamorphic uses the in-tree SUB encoder (encode_alu_reg) as the expansion target named by README.md:321. Shared encode_r/get_reg disclosed; still independently specified by the expansion table.
- Seed: README.md:321
- Formal: ∀ rd, rs ∈ GPRNames. encode_neg([Reg(rd), Reg(rs)]) = encode_alu_reg([Reg(rd), Reg("x0"), Reg(rs)], 0b000, 0b0100000)
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: gpr_name, rs: gpr_name }
  relation:
    op: eq
    lhs: encode_neg([Reg(rd), Reg(rs)])
    rhs: encode_alu_reg([Reg(rd), Reg("x0"), Reg(rs)], 0b000, 0b0100000)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/README.md:321; pseudo.rs:242 comment sub rd, x0, rs2; encoder/mod.rs:502 "sub" => encode_alu_reg(..., 0b000, 0b0100000)
```

## encode_neg_isa_fields
- Tier: 4d
- Rationale: RISC-V R-type SUB field layout is an exact structural predicate on every success word. Stronger differential and expansion metamorphic are sibling properties; this pins opcode/funct3/funct7/rs1=x0 independently of llvm-mc.
- Seed: encoder/mod.rs:272-276 R-type comment
- Formal: ∀ rd, rs ∈ 0..31. let Word(w) = encode_neg([Reg("x"+rd), Reg("x"+rs)]). (w&0x7F)=0b0110011 ∧ ((w>>7)&0x1F)=rd ∧ ((w>>12)&7)=0 ∧ ((w>>15)&0x1F)=0 ∧ ((w>>20)&0x1F)=rs ∧ ((w>>25)&0x7F)=0b0100000
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs]
  domain: { rd: u32_0_31, rs: u32_0_31 }
  body: let Word(w) = encode_neg([Reg(xN(rd)), Reg(xN(rs))]). (w & 0x7F) == 0b0110011 && ((w >> 7) & 0x1F) == rd && ((w >> 12) & 7) == 0 && ((w >> 15) & 0x1F) == 0 && ((w >> 20) & 0x1F) == rs && ((w >> 25) & 0x7F) == 0b0100000
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:272-276 R-type layout; encoder/mod.rs:329 OP_OP=0b0110011; RISC-V ISA SUB funct7=0100000 funct3=000; README.md:321 rs1=x0
```

## encode_neg_abi_xn_alias
- Tier: 4c
- Rationale: ABI names and xN names (plus fp/s0) are documented aliases of the same 5-bit encoding in reg_num. Encoding must be invariant under renaming to an alias of the same number.
- Seed: encoder/mod.rs:147-182 reg_num
- Formal: ∀ n, m ∈ 0..31. ∀ rd ∈ names(n), rs ∈ names(m). encode_neg([Reg(rd), Reg(rs)]) = encode_neg([Reg("x"+n), Reg("x"+m)])
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: u32_0_31, m: u32_0_31 }
  relation:
    op: eq
    lhs: encode_neg([Reg(abi_name(n)), Reg(abi_name(m))])
    rhs: encode_neg([Reg("x" + n), Reg("x" + m)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:147-182 ABI names and x0-x31; s0 | fp => 8
```

## encode_neg_neg_arity
- Tier: 5
- Rationale: Documented form is two operands (`neg rd, rs`). llvm-mc rejects too few operands. get_reg on missing index returns Err. Negative/error contract for the arity precondition.
- Seed: README.md:321 two-operand form
- Formal: ∀ ops. |ops| < 2 ⇒ is_err(encode_neg(ops))
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand_list_len_0_or_1 }
  relation:
    op: throws
    expr: encode_neg(ops)
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:321 neg rd, rs; encoder/mod.rs:347-355 get_reg missing index Err; llvm-mc too few operands
```

## encode_neg_neg_invalid
- Tier: 5
- Rationale: get_reg requires an integer register (or Imm 0..31). FP/vector/unknown names and non-Reg non-in-range-Imm kinds are documented as invalid. llvm-mc rejects FP regs and immediates-as-text.
- Seed: encoder/mod.rs:347-355 get_reg
- Formal: ∀ ops of length ≥ 2 where operand 0 or 1 is not a valid integer register and not Imm in 0..31. is_err(encode_neg(ops))
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad0, bad1]
  domain: { bad0: invalid_int_reg_operand, bad1: invalid_int_reg_operand }
  relation:
    op: throws
    expr: encode_neg([bad0, bad1])
generators:
  bad0: { gen: oneof, items: ["fa0","ft0","f0","v0","x32","x","foo",""] }
  bad1: { gen: oneof, items: ["fa1","fs0","f31","v31","x32","bar","spx"] }
expected_error: String
evidence: encoder/mod.rs:347-355 get_reg; encoder/mod.rs:147-182 reg_num None outside ABI/x0-x31; llvm-mc invalid operand for fa0
```

## encode_neg_neg_extra
- Tier: 5
- Rationale: Documented form is exactly two operands. llvm-mc rejects a third operand (`invalid operand for instruction`). Extra operands must be rejected, not silently ignored.
- Seed: README.md:321 two-operand form
- Formal: ∀ rd, rs ∈ GPRNames. ∀ extra. is_err(encode_neg([Reg(rd), Reg(rs), extra]))
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: failing
- Counterexample: rd = "zero", rs = "zero", extra = Reg("zero")
- Bug report: pbt-out/bug_reports/encode_neg_extra_operand.md

```property
function: encode_neg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs, extra]
  domain: { rd: gpr_name, rs: gpr_name, extra: Operand }
  relation:
    op: throws
    expr: encode_neg([Reg(rd), Reg(rs), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:321 neg rd, rs (exactly two operands); llvm-mc rejects `neg a0, a1, a2`
```

## encode_neg_imm_regnum
- Tier: 4c
- Rationale: get_reg documents that GCC inline asm may emit bare register numbers 0-31 as Imm. encode_neg must treat Imm(n) for n in 0..31 as register xN. Not a llvm-mc differential (llvm-mc rejects `neg 10, 11`); this is the in-tree documented extension.
- Seed: encoder/mod.rs:351-352
- Formal: ∀ n, m ∈ 0..31. encode_neg([Imm(n), Imm(m)]) = encode_neg([Reg("x"+n), Reg("x"+m)])
- Test file: src/backend/riscv/assembler/encoder/pseudo.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: i64_0_31, m: i64_0_31 }
  relation:
    op: eq
    lhs: encode_neg([Imm(n), Imm(m)])
    rhs: encode_neg([Reg("x" + n), Reg("x" + m)])
generators:
  n: { gen: int, min: 0, max: 31, type: i64 }
  m: { gen: int, min: 0, max: 31, type: i64 }
evidence: encoder/mod.rs:351-352 GCC sometimes emits bare register numbers (0-31) in inline asm
```
