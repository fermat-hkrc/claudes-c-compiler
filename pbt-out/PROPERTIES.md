# Properties: encode_fcvt_from_int

## encode_fcvt_from_int_diff_2op_llvm_mc
- Tier: 4
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler) for the shared RV64GC FCVT.{S,D}.{W,WU,L,LU} encoding contract. State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no OP-FP decoder in tree. encode_r / encode_fcvt_int / encode_fcvt_fp as siblings rejected — same-job gate (private packer / float-to-int inverse / float-to-float). Domain is all 8 dispatch mnemonics; omitted rm must match llvm-mc's 2-operand encoding.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:258 (sibling 2-op llvm-mc differential on the inverse conversion)
- Formal: ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs. llvm-mc(mnemonic rd, rs1) = encode_fcvt_from_int([Reg(rd), Reg(rs1)], funct7(mnemonic), rs2(mnemonic)) as Word
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_from_int([Reg("f0"), Reg("x0")], 0b1101001, 0) vs llvm-mc("fcvt.d.w f0, x0"): SUT 0xd2007053 != llvm-mc 0xd2000053
- Bug report: bug_reports/encode_fcvt_from_int_dw_omitted_rm.md

```property
function: encoder.encode_fcvt_from_int
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: FCVT_FROM_INT, rd: fp_reg, rs1: gpr }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn))
    rhs: llvm_mc_word(mn + " " + rd + ", " + rs1)
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.w","fcvt.d.wu","fcvt.d.l","fcvt.d.lu"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/mod.rs:751-754,779-782
```

## encode_fcvt_from_int_diff_rm_llvm_mc
- Tier: 4
- Rationale: Same differential, with an explicit RoundingMode in {rne,rtz,rdn,rup,rmm,dyn}. The word-equality mapping requires llvm-mc to produce an encoding; llvm-mc 15 errors on an rm operand for FCVT.D.W/WU, so those two have no llvm-mc word to compare and are not in this property's domain. They remain in the 2-op differential (which does have a llvm-mc word) and in the R-type / omitted-rm=dyn properties.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:273 (sibling rm llvm-mc differential)
- Formal: ∀ mnemonic ∈ FCVT.S.{W,WU,L,LU} ∪ FCVT.D.{L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs, ∀ rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. llvm-mc(mnemonic rd, rs1, rm) = encode_fcvt_from_int([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mnemonic), rs2(mnemonic)) as Word
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_from_int
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: FCVT_FROM_INT_WITH_RM, rd: fp_reg, rs1: gpr, rm: RM }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), rs2(mn))
    rhs: llvm_mc_word(mn + " " + rd + ", " + rs1 + ", " + rm)
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.l","fcvt.d.lu"] }
  rd: { gen: string }
  rs1: { gen: string }
  rm: { gen: oneof, items: ["rne","rtz","rdn","rup","rmm","dyn"] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:751-754,781-782; parser.rs:41
```

## encode_fcvt_from_int_r_type_fields
- Tier: 3
- Rationale: Algebraic invariant from the documented R-type layout (mod.rs:327, README.md:352). Weaker than differential; kept because it pins opcode/rm/rd/rs1/rs2/funct7 on all 8 mnemonics including FCVT.D.W/WU, which llvm-mc treats specially. Stronger state-machine / round-trip rejected as above.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:288 (sibling R-type unpack)
- Formal: ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ 0..31, ∀ rs1 ∈ 0..31, ∀ (rm_name, rm) ∈ RM. unpack_r(encode_fcvt_from_int([Reg(f{rd}), Reg(x{rs1}), RoundingMode(rm_name)], funct7(mnemonic), rs2(mnemonic))) = (OP_OP_FP, rm, rd, rs1, rs2(mnemonic), funct7(mnemonic))
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_from_int
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm_name, rm]
  domain: { mn: FCVT_FROM_INT, rd: u32_0_31, rs1: u32_0_31, rm_name: RM_NAME, rm: RM_BITS }
  relation:
    op: eq
    lhs: unpack_r(sut_word([Reg("f"+rd), Reg("x"+rs1), RoundingMode(rm_name)], funct7(mn), rs2(mn)))
    rhs: (OP_OP_FP, rm, rd, rs1, rs2(mn), funct7(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.w","fcvt.d.wu","fcvt.d.l","fcvt.d.lu"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rm_name: { gen: oneof, items: ["rne","rtz","rdn","rup","rmm","dyn"] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:327
```

## encode_fcvt_from_int_abi_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names (ft0/fa0/fs0, zero/ra/sp/fp/s0/a0/...) must encode the same rd/rs1 as fN/xN. Documented by freg_num / reg_num. fp is an alias of s0/x8.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:312 (sibling ABI alias)
- Formal: ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ n,m ∈ 0..31. encode_fcvt_from_int([Reg(fabi(n)), Reg(gabi(m))], funct7, rs2) = encode_fcvt_from_int([Reg(f{n}), Reg(x{m})], funct7, rs2) ∧ (n=8 ⇒ encode([Reg("fs0"), Reg("fp")]) equals the f8/x8 encoding)
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_from_int
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, m]
  domain: { mn: FCVT_FROM_INT, n: u32_0_31, m: u32_0_31 }
  relation:
    op: eq
    lhs: sut_word([Reg(fabi(n)), Reg(gabi(m))], funct7(mn), rs2(mn))
    rhs: sut_word([Reg("f"+n), Reg("x"+m)], funct7(mn), rs2(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.w","fcvt.d.wu","fcvt.d.l","fcvt.d.lu"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:200,248
```

## encode_fcvt_from_int_rm_default_dyn
- Tier: 3
- Rationale: Algebraic metamorphic: omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111. Documented by the 2-operand branch (float.rs:137-140) and parse_rm. Stronger differential already covers llvm-mc's 2-op form for the 6 mnemonics that share this convention.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:330 (sibling omitted-rm=dyn)
- Formal: ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs. encode_fcvt_from_int([Reg(rd), Reg(rs1)], f7, rs2) = encode_fcvt_from_int([Reg(rd), Reg(rs1), RoundingMode("dyn")], f7, rs2) ∧ unpack_r(that).rm = 0b111
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_from_int
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: FCVT_FROM_INT, rd: fp_reg, rs1: gpr }
  relation:
    op: eq
    lhs: sut_word([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn))
    rhs: sut_word([Reg(rd), Reg(rs1), RoundingMode("dyn")], funct7(mn), rs2(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.w","fcvt.d.wu","fcvt.d.l","fcvt.d.lu"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/float.rs:137-140
```

## encode_fcvt_from_int_neg_arity_class
- Tier: 2
- Rationale: Negative/error contract: empty, 1-operand, GPR in the FP rd slot, FP in the integer rs1 slot, and non-Reg rd must Err. get_freg requires an FP register at operand 0 (Imm, including 0..=31, is invalid there). get_reg requires an integer register at operand 1; its own comment documents Imm(0..=31) as a GCC bare GPR number (mod.rs:388-389) as a valid encoding for the integer slot. llvm-mc rejects the same class errors for names and missing operands.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:348 (sibling arity/class)
- Formal: ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ fp ∈ FPRegs, ∀ gpr ∈ GPRs, ∀ bad ∈ NonReg. encode_fcvt_from_int([], f7, rs2) is Err ∧ encode_fcvt_from_int([Reg(fp)], f7, rs2) is Err ∧ encode_fcvt_from_int([Reg(gpr), Reg(gpr)], f7, rs2) is Err ∧ encode_fcvt_from_int([Reg(fp), Reg(fp)], f7, rs2) is Err ∧ encode_fcvt_from_int([bad, Reg(gpr)], f7, rs2) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_from_int
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, fp, gpr, bad]
  domain: { mn: FCVT_FROM_INT, fp: fp_reg, gpr: gpr, bad: non_reg }
  relation:
    op: holds
    lhs: encode_fcvt_from_int([], f7, rs2).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.w","fcvt.d.wu","fcvt.d.l","fcvt.d.lu"] }
  fp: { gen: string }
  gpr: { gen: string }
evidence: src/backend/riscv/assembler/encoder/float.rs:133-134; encoder/mod.rs:412-419
```

## encode_fcvt_from_int_neg_extra
- Tier: 2
- Rationale: Negative/error contract: a 4th operand must be rejected. llvm-mc errors on extra operands (`invalid operand for instruction`). The encoder's documented job is to encode RISC-V instructions; extra operands are not in the ISA form. The body currently ignores operands beyond index 2 — this property is expected to falsify that.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:392 (sibling extra-operand)
- Formal: ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs, ∀ extra ∈ Operand. encode_fcvt_from_int([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], f7, rs2) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_from_int([Reg("f0"), Reg("x0"), RoundingMode("rne"), Imm(0)], 0b1101000, 0) -> Ok(Word(0xd0000053))
- Bug report: bug_reports/encode_fcvt_from_int_extra_operand.md

```property
function: encoder.encode_fcvt_from_int
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: FCVT_FROM_INT, rd: fp_reg, rs1: gpr, extra: Operand }
  relation:
    op: holds
    lhs: encode_fcvt_from_int([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], f7, rs2).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.w","fcvt.d.wu","fcvt.d.l","fcvt.d.lu"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: llvm-mc rejects extra operands; README.md:308; encoder/mod.rs:3
```

## encode_fcvt_from_int_neg_non_rm_third
- Tier: 2
- Rationale: Negative/error contract: a 3rd operand that is not RoundingMode must be rejected. Optional 3rd is only a rounding mode (parser.rs:41). llvm-mc errors (`operand must be a valid floating point rounding mode mnemonic`). The body currently maps a non-RM 3rd to rm=DYN — this property is expected to falsify that.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:132 "Integer to float: result in float register, source in integer register" — asserted fingerprint e5f07d38
- Seed: src/backend/riscv/assembler/encoder/encode_fcvt_int_pbt.rs:408 (sibling non-rm 3rd)
- Formal: ∀ mnemonic ∈ FCVT.{S,D}.{W,WU,L,LU}, ∀ rd ∈ FPRegs, ∀ rs1 ∈ GPRs, ∀ extra ∈ Operand \ RoundingMode. encode_fcvt_from_int([Reg(rd), Reg(rs1), extra], f7, rs2) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_from_int_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_from_int([Reg("f0"), Reg("x0"), Imm(0)], 0b1101000, 0) -> Ok(Word(0xd0007053))
- Bug report: bug_reports/encode_fcvt_from_int_non_rm_third.md

```property
function: encoder.encode_fcvt_from_int
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: FCVT_FROM_INT, rd: fp_reg, rs1: gpr, extra: Operand minus RoundingMode }
  relation:
    op: holds
    lhs: encode_fcvt_from_int([Reg(rd), Reg(rs1), extra], f7, rs2).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["fcvt.s.w","fcvt.s.wu","fcvt.s.l","fcvt.s.lu","fcvt.d.w","fcvt.d.wu","fcvt.d.l","fcvt.d.lu"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: parser.rs:41; llvm-mc rejects non-rm 3rd; float.rs:135-140
```
