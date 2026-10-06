# Properties: encode_fp_unary

## encode_fp_unary_diff_2op_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler, same OP-FP FSQRT job). State machine rejected (pure function). Round-trip rejected (no OP-FP decoder). encode_fp_arith / encode_r / encode_fcvt_fp rejected by same-job gate (3-operand rs2-from-operand / private packer / different mnemonic family). SUT-boundary: internal-helper; encode_instruction (mod.rs:727,755) passes operands through. Mapping: [Reg(rd), Reg(rs1)] <-> `mn rd, rs1` with default DYN rm and hardwired rs2=0. Domain is FSQRT.S/D mnemonics dispatched to this symbol.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_diff_3op_llvm_mc
- Formal: ∀ mn ∈ {fsqrt.s,fsqrt.d}, ∀ rd,rs1 ∈ FPRegs. encode_fp_unary([Reg(rd),Reg(rs1)], funct7(mn), 0) = llvm-mc(mn rd, rs1)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_unary
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: FP_UNARY_MN, rd: FPRegs, rs1: FPRegs }
  relation:
    op: eq
    lhs: encode_fp_unary([Reg(rd), Reg(rs1)], funct7(mn), 0)
    rhs: llvm_mc(mn + " " + rd + ", " + rs1)
generators:
  mn: { gen: oneof, options: ["fsqrt.s", "fsqrt.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:727 fsqrt.s => encode_fp_unary; mod.rs:755 fsqrt.d; README.md:307-308; RISC-V Unprivileged ISA FSQRT rs2=00000; llvm-mc -triple=riscv64 -mattr=+f,+d
```

## encode_fp_unary_diff_rm_llvm_mc
- Tier: 2
- Rationale: Optional 3rd operand is a documented rounding mode (parser.rs:41 closed set; encode_fp_unary body encodes it into funct3). Differential vs llvm-mc over {rne,rtz,rdn,rup,rmm,dyn} pins rm[14:12]. Same stronger-oracle rejection as the 2-op differential.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_diff_rm_llvm_mc
- Formal: ∀ mn ∈ {fsqrt.s,fsqrt.d}, ∀ rd,rs1 ∈ FPRegs, ∀ rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fp_unary([Reg(rd),Reg(rs1),RoundingMode(rm)], funct7(mn), 0) = llvm-mc(mn rd, rs1, rm)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_unary
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: FP_UNARY_MN, rd: FPRegs, rs1: FPRegs, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  relation:
    op: eq
    lhs: encode_fp_unary([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), 0)
    rhs: llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rm)
generators:
  mn: { gen: oneof, options: ["fsqrt.s", "fsqrt.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, options: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: src/backend/riscv/assembler/parser.rs:41 RoundingMode closed set; encoder/mod.rs:474 parse_rm; RISC-V rm encodings 000/001/010/011/100/111
```

## encode_fp_unary_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the R-type layout comment (mod.rs:318) and OP_OP_FP (mod.rs:377). FSQRT hardwires rs2=00000; S/D fmt lives in bits 26:25 (00 for fsqrt.s funct7=0101100, 01 for fsqrt.d funct7=0101101). Weaker than differential; still checks opcode/rd/rs1/rs2/rm/funct7 independently of llvm-mc. Documented bounds 0..=31 for registers and the closed RM set are sampled exactly.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_r_type_fields
- Formal: ∀ rd,rs1 ∈ 0..31, ∀ rm ∈ {0,1,2,3,4,7}, ∀ (funct7,rs2) ∈ {(0b0101100,0),(0b0101101,0)}. let w = encode_fp_unary([Reg(f{rd}),Reg(f{rs1}),RoundingMode(name(rm))], funct7, rs2) in unpack_r(w) = (OP_OP_FP=0b1010011, rm, rd, rs1, rs2, funct7)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_unary
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, rm, funct7]
  domain: { rd: 0..31, rs1: 0..31, rm: {0,1,2,3,4,7}, funct7: {0b0101100, 0b0101101} }
  body: unpack_r(encode_fp_unary([Reg(f{rd}),Reg(f{rs1}),RoundingMode(name(rm))], funct7, 0)) == (0b1010011, rm, rd, rs1, 0, funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, options: [0, 1, 2, 3, 4, 7] }
  funct7: { gen: oneof, options: [44, 45] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:318 R-type layout; mod.rs:377 OP_OP_FP; RISC-V FSQRT rs2=00000
```

## encode_fp_unary_abi_fn_alias
- Tier: 4
- Rationale: Metamorphic alias: FP ABI names (ft0/fa0/fs0/...) and fN encode the same 5-bit register (freg_num). Independent of llvm-mc. Not a same-job sibling differential.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_abi_fn_alias
- Formal: ∀ n,m ∈ 0..31, ∀ funct7 ∈ {0b0101100,0b0101101}. encode_fp_unary([Reg(f{n}),Reg(f{m})], funct7, 0) = encode_fp_unary([Reg(FABI[n]),Reg(FABI[m])], funct7, 0)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_unary
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, funct7]
  domain: { n: 0..31, m: 0..31, funct7: {0b0101100, 0b0101101} }
  relation:
    op: eq
    lhs: encode_fp_unary([Reg(f{n}), Reg(f{m})], funct7, 0)
    rhs: encode_fp_unary([Reg(FABI[n]), Reg(FABI[m])], funct7, 0)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [44, 45] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:239 freg_num ABI and fN aliases
```

## encode_fp_unary_rm_default_dyn
- Tier: 4
- Rationale: Metamorphic: omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111. Documented default when the optional rounding-mode operand is absent (body float.rs:89-91; RISC-V DYN=111; llvm-mc omits dyn in print but encodes 111).
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_rm_default_dyn
- Formal: ∀ rd,rs1 ∈ FPRegs, ∀ funct7 ∈ {0b0101100,0b0101101}. encode_fp_unary([Reg(rd),Reg(rs1)], funct7, 0) = encode_fp_unary([Reg(rd),Reg(rs1),RoundingMode("dyn")], funct7, 0) ∧ unpack_rm(that) = 0b111
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_unary
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1, funct7]
  domain: { rd: FPRegs, rs1: FPRegs, funct7: {0b0101100, 0b0101101} }
  body: encode_fp_unary([Reg(rd),Reg(rs1)], funct7, 0) == encode_fp_unary([Reg(rd),Reg(rs1),RoundingMode("dyn")], funct7, 0) && unpack_rm == 0b111
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [44, 45] }
evidence: src/backend/riscv/assembler/encoder/float.rs:89-91 omitted rm => 0b111; RISC-V DYN=111
```

## encode_fp_unary_neg_arity_gpr
- Tier: 5
- Rationale: Negative/error contract from llvm-mc (too few operands; GPR invalid in FP slot) and get_freg (mod.rs:402 expected float register). Empty, 1-operand, GPR rd/rs1, and non-Reg rd must Err. Documented error: llvm-mc "too few operands" / "invalid operand".
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_neg_arity_gpr
- Formal: ∀ funct7 ∈ {0b0101100,0b0101101}, ∀ fp ∈ FPRegs, ∀ gpr ∈ GPRegs, ∀ bad ∉ Reg(FP). encode_fp_unary([], funct7, 0) is Err ∧ encode_fp_unary([Reg(fp)], funct7, 0) is Err ∧ encode_fp_unary([Reg(gpr),Reg(fp)], funct7, 0) is Err ∧ encode_fp_unary([Reg(fp),Reg(gpr)], funct7, 0) is Err ∧ encode_fp_unary([bad,Reg(fp)], funct7, 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_unary
oracle: negative_error
predicate:
  quantifier: forall
  vars: [funct7, fp, gpr, bad]
  domain: { funct7: {0b0101100, 0b0101101}, fp: FPRegs, gpr: GPRegs, bad: NonReg }
  body: encode_fp_unary(empty|one|gpr-rd|gpr-rs1|nonreg-rd, funct7, 0) is Err
generators:
  funct7: { gen: oneof, options: [44, 45] }
  fp: { gen: int, min: 0, max: 31, type: u32 }
  gpr: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:402 get_freg; llvm-mc too few operands / invalid operand for GPR in FP slot
```

## encode_fp_unary_neg_extra
- Tier: 5
- Rationale: Negative/error contract: llvm-mc rejects a 4th operand after optional rm ("invalid operand for instruction"). encode_instruction public wrapper passes operands through. Extra operand must Err, not silently encode FSQRT.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_neg_extra
- Formal: ∀ mn ∈ {fsqrt.s,fsqrt.d}, ∀ rd,rs1 ∈ FPRegs, ∀ extra ∈ Operand. encode_fp_unary([Reg(rd),Reg(rs1),RoundingMode("rne"), extra], funct7(mn), 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: failing
- Counterexample: encode_fp_unary([Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b0101100, 0) -> Ok(Word(0x58000053))
- Bug report: pbt-out/bug_reports/encode_fp_unary_extra_operand.md

```property
function: encoder.encode_fp_unary
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: FP_UNARY_MN, rd: FPRegs, rs1: FPRegs, extra: Operand }
  body: encode_fp_unary([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], funct7(mn), 0) is Err
generators:
  mn: { gen: oneof, options: ["fsqrt.s", "fsqrt.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc -triple=riscv64 -mattr=+f,+d rejects `fsqrt.s fa0, fa1, rne, 0`; encode_instruction mod.rs:727 passes operands through
```

## encode_fp_unary_neg_non_rm_third
- Tier: 5
- Rationale: Negative/error contract: llvm-mc requires the optional 3rd operand to be a rounding-mode mnemonic ("operand must be a valid floating point rounding mode mnemonic"). A 3rd non-RoundingMode operand must Err, not encode as DYN.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_neg_non_rm_fourth
- Formal: ∀ mn ∈ {fsqrt.s,fsqrt.d}, ∀ rd,rs1 ∈ FPRegs, ∀ extra ∉ RoundingMode. encode_fp_unary([Reg(rd),Reg(rs1), extra], funct7(mn), 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs
- Status: failing
- Counterexample: encode_fp_unary([Reg("f0"), Reg("f0"), Imm(0)], 0b0101100, 0) -> Ok(Word(0x58007053))
- Bug report: pbt-out/bug_reports/encode_fp_unary_non_rm_third.md

```property
function: encoder.encode_fp_unary
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: FP_UNARY_MN, rd: FPRegs, rs1: FPRegs, extra: NonRoundingMode }
  body: encode_fp_unary([Reg(rd), Reg(rs1), extra], funct7(mn), 0) is Err
generators:
  mn: { gen: oneof, options: ["fsqrt.s", "fsqrt.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects `fsqrt.s fa0, fa1, 0` with "operand must be a valid floating point rounding mode mnemonic"; parser.rs:41 RoundingMode closed set
```
