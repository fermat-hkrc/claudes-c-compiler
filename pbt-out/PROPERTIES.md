# Properties: encode_fp_arith

## encode_fp_arith_diff_3op_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler, same OP-FP job). State machine rejected (pure function). Round-trip rejected (no OP-FP decoder). encode_fp_arith_d / encode_r / encode_fp_sgnj rejected by same-job gate (wrapper / private packer / funct3-not-rm). SUT-boundary: internal-helper; encode_instruction (mod.rs:721) passes operands through. Mapping: [Reg(rd), Reg(rs1), Reg(rs2)] <-> `mn rd, rs1, rs2` with default DYN rm.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs (3-reg R-type vs llvm-mc)
- Formal: ∀ mn ∈ {fadd.s,fsub.s,fmul.s,fdiv.s,fadd.d,fsub.d,fmul.d,fdiv.d}, ∀ rd,rs1,rs2 ∈ FPRegs. encode_fp_arith([Reg(rd),Reg(rs1),Reg(rs2)], funct7(mn)) = llvm-mc(mn rd, rs1, rs2)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: FP_ARITH_MN, rd: FPRegs, rs1: FPRegs, rs2: FPRegs }
  relation:
    op: eq
    lhs: encode_fp_arith([Reg(rd), Reg(rs1), Reg(rs2)], funct7(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rs2)
generators:
  mn: { gen: oneof, options: ["fadd.s", "fsub.s", "fmul.s", "fdiv.s", "fadd.d", "fsub.d", "fmul.d", "fdiv.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:721 fadd.s => encode_fp_arith; README.md:307-308; RISC-V Unprivileged ISA OP-FP; llvm-mc -triple=riscv64 -mattr=+f,+d
```

## encode_fp_arith_diff_rm_llvm_mc
- Tier: 2
- Rationale: Optional 4th operand is a documented rounding mode (float.rs:65). Differential vs llvm-mc over the closed RM set {rne,rtz,rdn,rup,rmm,dyn} pins rm[14:12]. Same stronger-oracle rejection as the 3-op differential.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: (none) — no existing encode_fp_arith RM test
- Formal: ∀ mn ∈ FP_ARITH_MN, ∀ rd,rs1,rs2 ∈ FPRegs, ∀ rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fp_arith([Reg(rd),Reg(rs1),Reg(rs2),RoundingMode(rm)], funct7(mn)) = llvm-mc(mn rd, rs1, rs2, rm)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rm]
  domain: { mn: FP_ARITH_MN, rd: FPRegs, rs1: FPRegs, rs2: FPRegs, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  relation:
    op: eq
    lhs: encode_fp_arith([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode(rm)], funct7(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rs2 + ", " + rm)
generators:
  mn: { gen: oneof, options: ["fadd.s", "fsub.s", "fmul.s", "fdiv.s", "fadd.d", "fsub.d", "fmul.d", "fdiv.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, options: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: src/backend/riscv/assembler/encoder/float.rs:65 optional rounding mode; encoder/mod.rs:472 parse_rm; RISC-V rm encodings 000/001/010/011/100/111
```

## encode_fp_arith_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the R-type layout comment (mod.rs:316) and OP_OP_FP (mod.rs:379). Weaker than differential; still checks opcode/rd/rs1/rs2/rm/funct7 independently of llvm-mc. Documented bounds 0..=31 for registers and the closed RM set are sampled exactly.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: src/backend/riscv/assembler/encoder/encode_alu_reg_pbt.rs unpack_r
- Formal: ∀ rd,rs1,rs2 ∈ 0..31, ∀ rm ∈ {0,1,2,3,4,7}, ∀ funct7 ∈ FP_ARITH_FUNCT7. let w = encode_fp_arith([Reg(f{rd}),Reg(f{rs1}),Reg(f{rs2}),RoundingMode(name(rm))], funct7) in unpack_r(w) = (OP_OP_FP=0b1010011, rm, rd, rs1, rs2, funct7)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, rm, funct7]
  domain: { rd: 0..31, rs1: 0..31, rs2: 0..31, rm: {0,1,2,3,4,7}, funct7: FP_ARITH_FUNCT7 }
  body: unpack_r(encode_fp_arith([Reg(f{rd}),Reg(f{rs1}),Reg(f{rs2}),RoundingMode(name(rm))], funct7)) == (0b1010011, rm, rd, rs1, rs2, funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, options: [0, 1, 2, 3, 4, 7] }
  funct7: { gen: oneof, options: [0, 4, 8, 12, 1, 5, 9, 13] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:316 R-type layout; mod.rs:379 OP_OP_FP
```

## encode_fp_arith_abi_fn_alias
- Tier: 4
- Rationale: Metamorphic alias: FP ABI names (ft0/fa0/fs0/...) and fN encode the same 5-bit register (freg_num). Independent of llvm-mc. Not a same-job sibling differential.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: src/backend/riscv/assembler/encoder/encode_float_load_pbt.rs encode_float_load_abi_fn_alias
- Formal: ∀ n,m,p ∈ 0..31, ∀ funct7 ∈ FP_ARITH_FUNCT7. encode_fp_arith([Reg(f{n}),Reg(f{m}),Reg(f{p})], funct7) = encode_fp_arith([Reg(FABI[n]),Reg(FABI[m]),Reg(FABI[p])], funct7)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, p, funct7]
  domain: { n: 0..31, m: 0..31, p: 0..31, funct7: FP_ARITH_FUNCT7 }
  relation:
    op: eq
    lhs: encode_fp_arith([Reg(f{n}), Reg(f{m}), Reg(f{p})], funct7)
    rhs: encode_fp_arith([Reg(FABI[n]), Reg(FABI[m]), Reg(FABI[p])], funct7)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  p: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [0, 4, 8, 12, 1, 5, 9, 13] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:237-285 freg_num ABI and f0-f31
```

## encode_fp_arith_rm_default_dyn
- Tier: 4
- Rationale: Metamorphic: omitted rounding mode equals explicit dyn (rm=111), matching RISC-V default and llvm-mc (omitted dyn prints as 3-op with encoding rm=111). float.rs:71-72 else branch 0b111.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: (none)
- Formal: ∀ rd,rs1,rs2 ∈ FPRegs, ∀ funct7 ∈ FP_ARITH_FUNCT7. encode_fp_arith([Reg(rd),Reg(rs1),Reg(rs2)], funct7) = encode_fp_arith([Reg(rd),Reg(rs1),Reg(rs2),RoundingMode("dyn")], funct7)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, funct7]
  domain: { rd: FPRegs, rs1: FPRegs, rs2: FPRegs, funct7: FP_ARITH_FUNCT7 }
  relation:
    op: eq
    lhs: encode_fp_arith([Reg(rd), Reg(rs1), Reg(rs2)], funct7)
    rhs: encode_fp_arith([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode("dyn")], funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [0, 4, 8, 12, 1, 5, 9, 13] }
evidence: src/backend/riscv/assembler/encoder/float.rs:71-72 else { 0b111 }; RISC-V default DYN
```

## encode_fp_arith_neg_arity_gpr
- Tier: 4
- Rationale: Negative/error contract. get_freg errors on missing index and on non-FP Reg (mod.rs:400-406). llvm-mc rejects too-few operands and GPR in an FP slot. Empty / 1 / 2 operands, GPR names, and non-Reg operands must return Err.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs encode_float_store_neg_arity_gpr
- Formal: ∀ funct7 ∈ FP_ARITH_FUNCT7, ∀ ops ∈ {[], [Reg(fp)], [Reg(fp),Reg(fp)], [Reg(gpr),Reg(fp),Reg(fp)], [Reg(fp),Reg(gpr),Reg(fp)], [Reg(fp),Reg(fp),Reg(gpr)], [nonReg,Reg(fp),Reg(fp)]}. encode_fp_arith(ops, funct7) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, funct7]
  domain: { ops: arity_or_gpr_or_nonreg, funct7: FP_ARITH_FUNCT7 }
  relation:
    op: throws
    expr: encode_fp_arith(ops, funct7)
generators:
  funct7: { gen: oneof, options: [0, 4, 8, 12] }
  ops: { gen: oneof, options: ["empty", "arity1", "arity2", "gpr_rd", "nonreg_rd"] }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:400-406 get_freg; llvm-mc "too few operands" / "invalid operand for instruction"
```

## encode_fp_arith_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 5th operand after rd,rs1,rs2[,rm] ("invalid operand for instruction"). encode_instruction passes extra operands through. A 4th RoundingMode is in-domain (optional rm); a 5th operand is not. Documented assembler contract is the ISA encoding of a 3-or-4-operand FP arith instruction, not "ignore trailing operands".
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: src/backend/riscv/assembler/encoder/encode_float_store_pbt.rs encode_float_store_neg_extra
- Formal: ∀ mn ∈ FP_ARITH_MN, ∀ rd,rs1,rs2 ∈ FPRegs, ∀ extra. encode_fp_arith([Reg(rd),Reg(rs1),Reg(rs2),RoundingMode("rne"), extra], funct7(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: failing
- Counterexample: encode_fp_arith([Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0) → Ok(Word(83))
- Bug report: pbt-out/bug_reports/encode_fp_arith_trailing_operand.md

```property
function: encoder.encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, extra, funct7]
  domain: { rd: FPRegs, rs1: FPRegs, rs2: FPRegs, extra: Operand, funct7: FP_ARITH_FUNCT7 }
  relation:
    op: throws
    expr: encode_fp_arith([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode("rne"), extra], funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [0, 4, 8, 12] }
  extra: { gen: oneof, options: ["Imm(0)", "Reg(x0)", "RoundingMode(rne)"] }
expected_error: String
evidence: llvm-mc rejects `fadd.s fa0, fa1, fa2, rne, rne` as invalid operand; encoder/mod.rs:721 passes operands through
```

## encode_fp_arith_neg_non_rm_fourth
- Tier: 4
- Rationale: Negative/error contract. A 4th operand that is not a RoundingMode is not the documented optional rm (float.rs:65). llvm-mc rejects `fadd.s fa0, fa1, fa2, x1` and unknown rm mnemonics. The SUT currently maps non-RoundingMode 4th operands to rm=111 (dyn) and still encodes — that is the law under test, not a pre-weakened skip.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:65 "Check for optional rounding mode" — asserted fingerprint 43e269d8
- Seed: (none)
- Formal: ∀ rd,rs1,rs2 ∈ FPRegs, ∀ extra ∉ RoundingMode, ∀ funct7 ∈ FP_ARITH_FUNCT7. encode_fp_arith([Reg(rd),Reg(rs1),Reg(rs2), extra], funct7) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs
- Status: failing
- Counterexample: encode_fp_arith([Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 0) → Ok(Word(28755))
- Bug report: pbt-out/bug_reports/encode_fp_arith_non_rm_fourth.md

```property
function: encoder.encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, extra, funct7]
  domain: { rd: FPRegs, rs1: FPRegs, rs2: FPRegs, extra: Operand \ RoundingMode, funct7: FP_ARITH_FUNCT7 }
  relation:
    op: throws
    expr: encode_fp_arith([Reg(rd), Reg(rs1), Reg(rs2), extra], funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [0, 4, 8, 12] }
  extra: { gen: oneof, options: ["Imm(0)", "Reg(x0)", "Mem", "Csr"] }
expected_error: String
evidence: llvm-mc "operand must be a valid floating point rounding mode mnemonic" / "invalid operand"; float.rs:65 optional rounding mode
```
