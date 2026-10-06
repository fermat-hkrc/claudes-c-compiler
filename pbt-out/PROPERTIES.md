# Properties: encode_fcvt_int

## encode_fcvt_int_diff_2op_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is Differential against llvm-mc (independent RISC-V assembler, same RV64GC F/D contract). State machine rejected — encode_fcvt_int is a pure function with no lifecycle. Algebraic round-trip via in-tree decoder rejected — no OP-FP decoder. encode_r / encode_fp_unary / encode_fcvt_from_int as differential sibling rejected — same-job gate (private packer / FP-rd unary / integer-to-float inverse, not this mnemonic family).
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_diff_2op_llvm_mc (sibling OP-FP 2-op llvm-mc differential)
- Formal: ∀ mn ∈ {fcvt.w.s, fcvt.wu.s, fcvt.l.s, fcvt.lu.s, fcvt.w.d, fcvt.wu.d, fcvt.l.d, fcvt.lu.d}, rd ∈ GPRNames, rs1 ∈ FPRNames. encode_fcvt_int([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn)) = llvm-mc("-triple=riscv64 -mattr=+f,+d -show-encoding", "mn rd, rs1")
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_int
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: fcvt_int_mnemonics, rd: gpr_names, rs1: fpr_names }
  relation:
    op: eq
    lhs: encode_fcvt_int([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn))
    rhs: llvm_mc(asm_2op(mn, rd, rs1))
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: README.md line 308 fcvt all int/float conversions
```

## encode_fcvt_int_diff_rm_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for the optional rounding-mode operand. Same stronger-oracle rejection as 2-op. RISC-V ISA places rm in funct3; omitted vs explicit is a separate metamorphic property.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_diff_rm_llvm_mc
- Formal: ∀ mn ∈ FcvtIntMn, rd ∈ GPRNames, rs1 ∈ FPRNames, rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fcvt_int([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), rs2(mn)) = llvm-mc("mn rd, rs1, rm")
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_int
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: fcvt_int_mnemonics, rd: gpr_names, rs1: fpr_names, rm: rounding_modes }
  relation:
    op: eq
    lhs: encode_fcvt_int([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), rs2(mn))
    rhs: llvm_mc(asm_3op(mn, rd, rs1, rm))
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rm: { gen: oneof, items: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: encoder/mod.rs line 482 parse_rm three-bit encoding
```

## encode_fcvt_int_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant — R-type field unpack per README.md:352 and encoder/mod.rs:327. Weaker than differential; still pins opcode/rd/rm/rs1/rs2/funct7 independently of llvm-mc. rs2 is the ISA dest-width field, not hardwired 0 (unlike FSQRT).
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_r_type_fields
- Formal: ∀ mn, rd ∈ 0..31, rs1 ∈ 0..31, rm ∈ RM. let w = encode_fcvt_int([Reg(x{rd}), Reg(f{rs1}), RoundingMode(rm)], funct7(mn), rs2(mn)). unpack_r(w) = (OP_OP_FP, rm_enc, rd, rs1, rs2(mn), funct7(mn))
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_int
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: fcvt_int_mnemonics, rd: 0..31, rs1: 0..31, rm: rounding_modes }
  body: unpack_r(word) == (OP_OP_FP, rm_enc, rd, rs1, rs2_mn, funct7_mn)
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, items: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: encoder/mod.rs line 327 R-type field layout
```

## encode_fcvt_int_abi_alias
- Tier: 4
- Rationale: Algebraic metamorphic — ABI names (a0/t0/fp/... and fa0/ft0/...) must encode the same rd/rs1 as xN/fN. Independent of llvm-mc; grounded in reg_num/freg_num tables.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_abi_fn_alias
- Formal: ∀ n,m ∈ 0..31, mn. encode_fcvt_int([Reg(x{n}), Reg(f{m})], f7, rs2) = encode_fcvt_int([Reg(gabi(n)), Reg(fabi(m))], f7, rs2). Also fp aliases s0 (rd=8).
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_int
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, m]
  domain: { mn: fcvt_int_mnemonics, n: 0..31, m: 0..31 }
  relation:
    op: eq
    lhs: encode_fcvt_int([Reg(xn(n)), Reg(fn(m))], funct7(mn), rs2(mn))
    rhs: encode_fcvt_int([Reg(gabi(n)), Reg(fabi(m))], funct7(mn), rs2(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: parser.rs line 22 Register ABI names xN and fN
```

## encode_fcvt_int_rm_default_dyn
- Tier: 4
- Rationale: Algebraic metamorphic — omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111. RISC-V default rounding is DYN. Grounded in encode_fcvt_int body (operands.len() <= 2 ⇒ rm=0b111) plus ISA/llvm-mc default.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_rm_default_dyn
- Formal: ∀ mn, rd ∈ GPRNames, rs1 ∈ FPRNames. encode_fcvt_int([Reg(rd), Reg(rs1)], f7, rs2) = encode_fcvt_int([Reg(rd), Reg(rs1), RoundingMode("dyn")], f7, rs2) ∧ unpack_r(w).funct3 = 0b111
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_int
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: fcvt_int_mnemonics, rd: gpr_names, rs1: fpr_names }
  relation:
    op: eq
    lhs: encode_fcvt_int([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn))
    rhs: encode_fcvt_int([Reg(rd), Reg(rs1), RoundingMode("dyn")], funct7(mn), rs2(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: encoder/mod.rs line 482 parse_rm three-bit encoding
```

## encode_fcvt_int_neg_arity_class
- Tier: 5
- Rationale: Negative/error contract — empty, 1-operand, FP rd, GPR in the FP slot, and non-Reg rd (excluding Imm 0..=31, which get_reg documents as GCC bare numbers) must Err. llvm-mc rejects the same class. Evidence: get_reg/get_freg error strings; float.rs:117 asserted integer-rd / float-rs1; llvm-mc "invalid operand" / "too few operands".
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_neg_arity_gpr
- Formal: ∀ mn, fp ∈ FPRNames, gpr ∈ GPRNames, bad ∈ NonReg\{Imm(0..=31)}. encode_fcvt_int([], f7, rs2) is Err ∧ encode_fcvt_int([Reg(gpr)], f7, rs2) is Err ∧ encode_fcvt_int([Reg(fp), Reg(fp)], f7, rs2) is Err ∧ encode_fcvt_int([Reg(gpr), Reg(gpr)], f7, rs2) is Err ∧ encode_fcvt_int([bad, Reg(fp)], f7, rs2) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_int
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, fp, gpr, bad]
  domain: { mn: fcvt_int_mnemonics, fp: fpr_names, gpr: gpr_names, bad: non_reg_operands }
  body: encode_fcvt_int_rejects_arity_and_wrong_class(mn, fp, gpr, bad)
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  fp: { gen: string }
  gpr: { gen: string }
  bad: { gen: oneof, items: ["Imm(-1)", "Csr", "FenceArg", "RoundingMode", "Label", "Symbol", "Mem"] }
expected_error: String
evidence: float.rs line 117 integer rd and float rs1
```

## encode_fcvt_int_neg_extra
- Tier: 5
- Rationale: Negative/error contract — a 4th operand is invalid. llvm-mc rejects extra operands ("invalid operand for instruction"). encode_instruction passes operands through unchanged, so the helper's arity contract is the public one. RISC-V FCVT.int encoding has at most rd, rs1, rm.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_neg_extra
- Formal: ∀ mn, rd ∈ GPRNames, rs1 ∈ FPRNames, extra ∈ Operand. encode_fcvt_int([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], f7, rs2) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_int([Reg("x0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b1100000, 0) -> Ok(Word(0xc0000053))
- Bug report: bug_reports/encode_fcvt_int_extra_operand.md

```property
function: encoder.encode_fcvt_int
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: fcvt_int_mnemonics, rd: gpr_names, rs1: fpr_names, extra: Operand }
  body: encode_fcvt_int([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], funct7(mn), rs2(mn)).is_err()
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: README.md line 352 R-type layout has no extra operand field
```

## encode_fcvt_int_neg_non_rm_third
- Tier: 5
- Rationale: Negative/error contract — a 3rd operand that is not RoundingMode is invalid. llvm-mc: "operand must be a valid floating point rounding mode mnemonic". Parser only constructs RoundingMode for {rne,rtz,rdn,rup,rmm,dyn}. A non-RM 3rd token must Err, not silently map to DYN.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:117 "Float to integer: result in integer register, source in float register" — asserted fingerprint 5e744c4d
- Seed: encode_fp_unary_pbt.rs encode_fp_unary_neg_non_rm_third
- Formal: ∀ mn, rd ∈ GPRNames, rs1 ∈ FPRNames, extra ∈ Operand\{RoundingMode}. encode_fcvt_int([Reg(rd), Reg(rs1), extra], f7, rs2) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_int([Reg("x0"), Reg("f0"), Imm(0)], 0b1100000, 0) -> Ok(Word(0xc0007053))
- Bug report: bug_reports/encode_fcvt_int_non_rm_third.md

```property
function: encoder.encode_fcvt_int
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: fcvt_int_mnemonics, rd: gpr_names, rs1: fpr_names, extra: non_rm_operands }
  body: encode_fcvt_int([Reg(rd), Reg(rs1), extra], funct7(mn), rs2(mn)).is_err()
generators:
  mn: { gen: oneof, items: ["fcvt.w.s", "fcvt.wu.s", "fcvt.l.s", "fcvt.lu.s", "fcvt.w.d", "fcvt.wu.d", "fcvt.l.d", "fcvt.lu.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: parser.rs line 41 RoundingMode closed set rne rtz rdn rup rmm dyn
```
