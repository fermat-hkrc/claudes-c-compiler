# Properties: encode_fp_arith_d

## encode_fp_arith_d_diff_3op_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler, same OP-FP D-extension job). State machine rejected (pure function). Round-trip rejected (no OP-FP decoder). encode_fp_arith / encode_r / encode_fp_sgnj rejected by same-job gate (callee this symbol wraps / private packer / funct3-not-rm). SUT-boundary: internal-helper; encode_instruction (mod.rs:749) passes operands through. Mapping: [Reg(rd), Reg(rs1), Reg(rs2)] <-> `mn rd, rs1, rs2` with default DYN rm. Domain is D-only mnemonics dispatched to this symbol.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_diff_3op_llvm_mc
- Formal: ∀ mn ∈ {fadd.d,fsub.d,fmul.d,fdiv.d}, ∀ rd,rs1,rs2 ∈ FPRegs. encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2)], funct7(mn)) = llvm-mc(mn rd, rs1, rs2)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith_d
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: FP_ARITH_D_MN, rd: FPRegs, rs1: FPRegs, rs2: FPRegs }
  relation:
    op: eq
    lhs: encode_fp_arith_d([Reg(rd), Reg(rs1), Reg(rs2)], funct7(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rs2)
generators:
  mn: { gen: oneof, options: ["fadd.d", "fsub.d", "fmul.d", "fdiv.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:749 fadd.d => encode_fp_arith_d; README.md:307-308; RISC-V Unprivileged ISA OP-FP fmt=D; llvm-mc -triple=riscv64 -mattr=+f,+d
```

## encode_fp_arith_d_diff_rm_llvm_mc
- Tier: 2
- Rationale: Optional 4th operand is a documented rounding mode (float.rs:65 on the callee; parser.rs:41 closed set). Differential vs llvm-mc over {rne,rtz,rdn,rup,rmm,dyn} pins rm[14:12]. Same stronger-oracle rejection as the 3-op differential.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_diff_rm_llvm_mc
- Formal: ∀ mn ∈ {fadd.d,fsub.d,fmul.d,fdiv.d}, ∀ rd,rs1,rs2 ∈ FPRegs, ∀ rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2),RoundingMode(rm)], funct7(mn)) = llvm-mc(mn rd, rs1, rs2, rm)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith_d
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rm]
  domain: { mn: FP_ARITH_D_MN, rd: FPRegs, rs1: FPRegs, rs2: FPRegs, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  relation:
    op: eq
    lhs: encode_fp_arith_d([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode(rm)], funct7(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rs2 + ", " + rm)
generators:
  mn: { gen: oneof, options: ["fadd.d", "fsub.d", "fmul.d", "fdiv.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, options: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: src/backend/riscv/assembler/encoder/float.rs:65 optional rounding mode; encoder/mod.rs:472 parse_rm; RISC-V rm encodings 000/001/010/011/100/111
```

## encode_fp_arith_d_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the R-type layout comment (mod.rs:316) and OP_OP_FP (mod.rs:379). D-extension fmt lives in bits 26:25 and must be 01 for every funct7 this dispatcher supplies (0000001/0000101/0001001/0001101). Weaker than differential; still checks opcode/rd/rs1/rs2/rm/funct7/fmt independently of llvm-mc. Documented bounds 0..=31 for registers and the closed RM set are sampled exactly.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_r_type_fields
- Formal: ∀ rd,rs1,rs2 ∈ 0..31, ∀ rm ∈ {0,1,2,3,4,7}, ∀ funct7 ∈ {1,5,9,13}. let w = encode_fp_arith_d([Reg(f{rd}),Reg(f{rs1}),Reg(f{rs2}),RoundingMode(name(rm))], funct7) in unpack_r(w) = (OP_OP_FP=0b1010011, rm, rd, rs1, rs2, funct7) ∧ ((w >> 25) & 0b11) = 0b01
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith_d
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, rm, funct7]
  domain: { rd: 0..31, rs1: 0..31, rs2: 0..31, rm: {0,1,2,3,4,7}, funct7: FP_ARITH_D_FUNCT7 }
  body: unpack_r(encode_fp_arith_d([Reg(f{rd}),Reg(f{rs1}),Reg(f{rs2}),RoundingMode(name(rm))], funct7)) == (0b1010011, rm, rd, rs1, rs2, funct7) && fmt(w) == 0b01
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, options: [0, 1, 2, 3, 4, 7] }
  funct7: { gen: oneof, options: [1, 5, 9, 13] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:316 R-type layout; mod.rs:379 OP_OP_FP; RISC-V OP-FP fmt=01 for D
```

## encode_fp_arith_d_abi_fn_alias
- Tier: 4
- Rationale: Metamorphic alias: FP ABI names (ft0/fa0/fs0/...) and fN encode the same 5-bit register (freg_num). Independent of llvm-mc. Not a same-job sibling differential.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_abi_fn_alias
- Formal: ∀ n,m,p ∈ 0..31, ∀ funct7 ∈ {1,5,9,13}. encode_fp_arith_d([Reg(f{n}),Reg(f{m}),Reg(f{p})], funct7) = encode_fp_arith_d([Reg(FABI[n]),Reg(FABI[m]),Reg(FABI[p])], funct7)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith_d
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, p, funct7]
  domain: { n: 0..31, m: 0..31, p: 0..31, funct7: FP_ARITH_D_FUNCT7 }
  relation:
    op: eq
    lhs: encode_fp_arith_d([Reg(f{n}), Reg(f{m}), Reg(f{p})], funct7)
    rhs: encode_fp_arith_d([Reg(FABI[n]), Reg(FABI[m]), Reg(FABI[p])], funct7)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  p: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [1, 5, 9, 13] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:237-285 freg_num ABI and f0-f31
```

## encode_fp_arith_d_rm_default_dyn
- Tier: 4
- Rationale: Metamorphic: omitted rounding mode equals explicit dyn (rm=111), matching RISC-V default and llvm-mc (omitted dyn prints as 3-op with encoding rm=111). Callee float.rs:71-72 else branch 0b111.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_rm_default_dyn
- Formal: ∀ rd,rs1,rs2 ∈ FPRegs, ∀ funct7 ∈ {1,5,9,13}. encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2)], funct7) = encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2),RoundingMode("dyn")], funct7)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith_d
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, funct7]
  domain: { rd: FPRegs, rs1: FPRegs, rs2: FPRegs, funct7: FP_ARITH_D_FUNCT7 }
  relation:
    op: eq
    lhs: encode_fp_arith_d([Reg(rd), Reg(rs1), Reg(rs2)], funct7)
    rhs: encode_fp_arith_d([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode("dyn")], funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [1, 5, 9, 13] }
evidence: src/backend/riscv/assembler/encoder/float.rs:71-72 else { 0b111 }; RISC-V default DYN
```

## encode_fp_arith_d_neg_arity_gpr
- Tier: 4
- Rationale: Negative/error contract. get_freg errors on missing index and on non-FP Reg (mod.rs:400-406). llvm-mc rejects too-few operands and GPR in an FP slot. Empty / 1 / 2 operands, GPR names, and non-Reg operands must return Err.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_neg_arity_gpr
- Formal: ∀ funct7 ∈ {1,5,9,13}, ∀ ops ∈ {[], [Reg(fp)], [Reg(fp),Reg(fp)], [Reg(gpr),Reg(fp),Reg(fp)], [Reg(fp),Reg(gpr),Reg(fp)], [Reg(fp),Reg(fp),Reg(gpr)], [nonReg,Reg(fp),Reg(fp)]}. encode_fp_arith_d(ops, funct7) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_arith_d
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, funct7]
  domain: { ops: arity_or_gpr_or_nonreg, funct7: FP_ARITH_D_FUNCT7 }
  relation:
    op: throws
    expr: encode_fp_arith_d(ops, funct7)
generators:
  funct7: { gen: oneof, options: [1, 5, 9, 13] }
  ops: { gen: oneof, options: ["empty", "arity1", "arity2", "gpr_rd", "nonreg_rd"] }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:400-406 get_freg; llvm-mc "too few operands" / "invalid operand for instruction"
```

## encode_fp_arith_d_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 5th operand after rd,rs1,rs2[,rm] ("invalid operand for instruction"). encode_instruction passes extra operands through. A 4th RoundingMode is in-domain (optional rm); a 5th operand is not. Documented assembler contract is the ISA encoding of a 3-or-4-operand FP arith instruction, not "ignore trailing operands".
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_neg_extra
- Formal: ∀ mn ∈ {fadd.d,fsub.d,fmul.d,fdiv.d}, ∀ rd,rs1,rs2 ∈ FPRegs, ∀ extra. encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2),RoundingMode("rne"), extra], funct7(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: failing
- Counterexample: encode_fp_arith_d([Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 1) → Ok(Word(33554515))
- Bug report: pbt-out/bug_reports/encode_fp_arith_d_trailing_operand.md

```property
function: encoder.encode_fp_arith_d
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, extra, funct7]
  domain: { rd: FPRegs, rs1: FPRegs, rs2: FPRegs, extra: Operand, funct7: FP_ARITH_D_FUNCT7 }
  relation:
    op: throws
    expr: encode_fp_arith_d([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode("rne"), extra], funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [1, 5, 9, 13] }
  extra: { gen: oneof, options: ["Imm(0)", "Reg(x0)", "RoundingMode(rne)"] }
expected_error: String
evidence: llvm-mc rejects `fadd.d fa0, fa1, fa2, rne, rne` as invalid operand; encoder/mod.rs:749 passes operands through
```

## encode_fp_arith_d_neg_non_rm_fourth
- Tier: 4
- Rationale: Negative/error contract. A 4th operand that is not a RoundingMode is not the documented optional rm (float.rs:65). llvm-mc rejects `fadd.d fa0, fa1, fa2, x1` and unknown rm mnemonics. The SUT currently maps non-RoundingMode 4th operands to rm=111 (dyn) and still encodes — that is the law under test, not a pre-weakened skip.
- Doc contract: (none)
- Seed: src/backend/riscv/assembler/encoder/encode_fp_arith_pbt.rs encode_fp_arith_neg_non_rm_fourth
- Formal: ∀ rd,rs1,rs2 ∈ FPRegs, ∀ extra ∉ RoundingMode, ∀ funct7 ∈ {1,5,9,13}. encode_fp_arith_d([Reg(rd),Reg(rs1),Reg(rs2), extra], funct7) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_arith_d_pbt.rs
- Status: failing
- Counterexample: encode_fp_arith_d([Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 1) → Ok(Word(33583187))
- Bug report: pbt-out/bug_reports/encode_fp_arith_d_non_rm_fourth.md

```property
function: encoder.encode_fp_arith_d
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, extra, funct7]
  domain: { rd: FPRegs, rs1: FPRegs, rs2: FPRegs, extra: Operand \ RoundingMode, funct7: FP_ARITH_D_FUNCT7 }
  relation:
    op: throws
    expr: encode_fp_arith_d([Reg(rd), Reg(rs1), Reg(rs2), extra], funct7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  funct7: { gen: oneof, options: [1, 5, 9, 13] }
  extra: { gen: oneof, options: ["Imm(0)", "Reg(x0)", "Mem", "Csr"] }
expected_error: String
evidence: llvm-mc "operand must be a valid floating point rounding mode mnemonic" / "invalid operand"; float.rs:65 optional rounding mode
```
