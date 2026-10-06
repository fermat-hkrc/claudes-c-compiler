# Properties: encode_fp_cmp

## encode_fp_cmp_diff_3op_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler). State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no OP-FP decoder in tree. Same-job sibling encode_fp_sgnj / encode_fp_arith / encode_r rejected — encode_fp_sgnj uses FP rd (different register-class job); encode_fp_arith uses rm in funct3 for FADD-family (different job); encode_r is a private packer. llvm-mc is an independent assembler the SUT's README claims to replace for RV64GC textual assembly.
- Doc contract: float.rs:103 "Result goes to integer register" — asserted fingerprint 6a44d6b3
- Seed: encode_fp_sgnj_pbt.rs encode_fp_sgnj_diff_3op_llvm_mc
- Formal: ∀ mn ∈ {feq.s, flt.s, fle.s, feq.d, flt.d, fle.d}, rd ∈ GPRNames, rs1,rs2 ∈ FPRNames. encode_fp_cmp([Reg(rd), Reg(rs1), Reg(rs2)], funct7(mn), funct3(mn)) = Word(llvm-mc(mn rd, rs1, rs2))
- Test file: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_cmp
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: {feq.s,flt.s,fle.s,feq.d,flt.d,fle.d}, rd: GPRNames, rs1: FPRNames, rs2: FPRNames }
  relation:
    op: eq
    lhs: encode_fp_cmp([Reg(rd), Reg(rs1), Reg(rs2)], funct7(mn), funct3(mn))
    rhs: Word(llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rs2))
generators:
  mn: { gen: oneof, options: ["feq.s", "flt.s", "fle.s", "feq.d", "flt.d", "fle.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
evidence: README.md:309 feq/flt/fle; encoder/mod.rs:737-739 and :765-767 dispatch; RISC-V Unprivileged ISA OP-FP compare
```

## encode_fp_cmp_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout. Stronger differential is p1; this pins opcode/rd/rs1/rs2/funct3/funct7 independently of llvm-mc so a mapping bug cannot hide a packer bug.
- Doc contract: encoder/mod.rs:313 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: encode_fp_sgnj_pbt.rs encode_fp_sgnj_r_type_fields
- Formal: ∀ rd ∈ 0..31, rs1 ∈ 0..31, rs2 ∈ 0..31, (f7,f3) ∈ ISA_CMP. let w = encode_fp_cmp([Reg(x{rd}), Reg(f{rs1}), Reg(f{rs2})], f7, f3). unpack_r(w) = (OP_OP_FP, f3, rd, rs1, rs2, f7)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_cmp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, f7, f3]
  domain: { rd: 0..31, rs1: 0..31, rs2: 0..31, f7: {0b1010000,0b1010001}, f3: {0,1,2} }
  body: unpack_r(encode_fp_cmp([Reg(x{rd}),Reg(f{rs1}),Reg(f{rs2})], f7, f3)) == (0b1010011, f3, rd, rs1, rs2, f7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  f7: { gen: oneof, options: [80, 81] }
  f3: { gen: int, min: 0, max: 2, type: u32 }
evidence: encoder/mod.rs:313 R-type layout; encoder/mod.rs:377 OP_OP_FP
```

## encode_fp_cmp_abi_xn_fn_alias
- Tier: 4
- Rationale: Algebraic metamorphic — ABI names and numeric names are documented aliases of the same 5-bit encoding (reg_num / freg_num). Independent of llvm-mc.
- Doc contract: parser.rs:22-23 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7, f0-f31, ft0-ft11, fs0-fs11, fa0-fa7" — asserted (alias table). encoder/mod.rs:194-248 GPR ABI; encoder/mod.rs:245-293 FPR ABI.
- Seed: encode_fp_sgnj_pbt.rs encode_fp_sgnj_abi_fn_alias
- Formal: ∀ n,m,p ∈ 0..31, (f7,f3) ∈ ISA_CMP. encode_fp_cmp([Reg(x{n}), Reg(f{m}), Reg(f{p})], f7, f3) = encode_fp_cmp([Reg(gabi(n)), Reg(fabi(m)), Reg(fabi(p))], f7, f3)
- Test file: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_cmp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, p, f7, f3]
  domain: { n: 0..31, m: 0..31, p: 0..31 }
  relation:
    op: eq
    lhs: encode_fp_cmp([Reg(x{n}), Reg(f{m}), Reg(f{p})], f7, f3)
    rhs: encode_fp_cmp([Reg(gabi(n)), Reg(fabi(m)), Reg(fabi(p))], f7, f3)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  p: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:194-293 ABI alias tables; parser.rs:22-23
```

## encode_fp_cmp_diff_rs1_eq_rs2_llvm_mc
- Tier: 5
- Rationale: Differential vs llvm-mc on the same-rs1=rs2 form (legal FEQ/FLT/FLE). Strengthens p1 toward the rs1=rs2 edge the previous FSGNJ campaign used for fmv/fabs/fneg; here it is just a valid compare of a register with itself.
- Doc contract: float.rs:103 "Result goes to integer register" — asserted fingerprint 6a44d6b3
- Seed: encode_fp_sgnj_pbt.rs encode_fp_sgnj_diff_rs1_eq_rs2_llvm_mc
- Formal: ∀ mn ∈ CMP, rd ∈ GPRNames, rs ∈ FPRNames. encode_fp_cmp([Reg(rd), Reg(rs), Reg(rs)], funct7(mn), funct3(mn)) = Word(llvm-mc(mn rd, rs, rs))
- Test file: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_cmp
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs]
  domain: { mn: CMP, rd: GPRNames, rs: FPRNames }
  relation:
    op: eq
    lhs: encode_fp_cmp([Reg(rd), Reg(rs), Reg(rs)], funct7(mn), funct3(mn))
    rhs: Word(llvm_mc(mn + " " + rd + ", " + rs + ", " + rs))
generators:
  mn: { gen: oneof, options: ["feq.s", "flt.s", "fle.s", "feq.d", "flt.d", "fle.d"] }
  rd: { gen: string }
  rs: { gen: string }
evidence: README.md:309; llvm-mc accepts feq.s a0, fa1, fa1
```

## encode_fp_cmp_neg_arity_class
- Tier: 3
- Rationale: Negative/error contract. llvm-mc errors on too few operands, FP rd, GPR in an FP slot, and non-register tokens. get_reg/get_freg return Err for missing/wrong-class operands. encode_fp_cmp's own comment asserts integer rd.
- Doc contract: float.rs:103 "Result goes to integer register" — asserted fingerprint 6a44d6b3
- Seed: encode_fp_sgnj_pbt.rs encode_fp_sgnj_neg_arity_gpr
- Formal: ∀ (f7,f3) ∈ ISA_CMP, fp ∈ FPRNames, gpr ∈ GPRNames, bad ∉ {Reg, Imm(0..=31)}. encode_fp_cmp([], f7, f3) is Err ∧ encode_fp_cmp([Reg(gpr)], f7, f3) is Err ∧ encode_fp_cmp([Reg(gpr), Reg(fp)], f7, f3) is Err ∧ encode_fp_cmp([Reg(fp), Reg(fp), Reg(fp)], f7, f3) is Err ∧ encode_fp_cmp([Reg(gpr), Reg(gpr), Reg(fp)], f7, f3) is Err ∧ encode_fp_cmp([Reg(gpr), Reg(fp), Reg(gpr)], f7, f3) is Err ∧ encode_fp_cmp([bad, Reg(fp), Reg(fp)], f7, f3) is Err ∧ encode_fp_cmp([Reg(gpr), Imm(0), Reg(fp)], f7, f3) is Err ∧ encode_fp_cmp([Reg(gpr), Reg(fp), Imm(0)], f7, f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f7, f3, fp, gpr, bad]
  domain: { f7: ISA_CMP.f7, f3: ISA_CMP.f3, fp: FPRNames, gpr: GPRNames, bad: NonRegOperand }
  body: encode_fp_cmp(wrong_arity_or_class(fp,gpr,bad), f7, f3) is Err
generators:
  fp: { gen: string }
  gpr: { gen: string }
expected_error: String
evidence: float.rs:103 integer rd; get_reg/get_freg class checks; llvm-mc invalid operand / too few operands
```

## encode_fp_cmp_neg_extra
- Tier: 3
- Rationale: Negative/error contract. llvm-mc rejects a 4th operand on FEQ/FLT/FLE ("invalid operand for instruction"). Public dispatch passes the operand list through unchanged, so extra tokens from the parser reach this helper. encode_fp_cmp currently ignores operands past index 2.
- Doc contract: (none) on extra arity for encode_fp_cmp itself. README.md:352 documents a 6-field R-type with no extra operand; llvm-mc rejects extras. Contract evidence: inferred (public wrapper encoder/mod.rs:737-739 / :765-767 passes the operand slice through; llvm-mc and the assembler README claim to encode the same textual RV64GC).
- Seed: encode_fp_sgnj_pbt.rs encode_fp_sgnj_neg_extra
- Formal: ∀ mn ∈ CMP, rd ∈ GPRNames, rs1,rs2 ∈ FPRNames, extra ∈ Operand. encode_fp_cmp([Reg(rd), Reg(rs1), Reg(rs2), extra], funct7(mn), funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
- Status: failing
- Counterexample: encode_fp_cmp([Reg("x0"), Reg("f0"), Reg("f0"), Imm(0)], 0b1010000, 0b010)
- Bug report: pbt-out/bug_reports/encode_fp_cmp_extra_operand.md

```property
function: encoder.encode_fp_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, extra]
  domain: { mn: CMP, rd: GPRNames, rs1: FPRNames, rs2: FPRNames, extra: Operand }
  body: encode_fp_cmp([Reg(rd), Reg(rs1), Reg(rs2), extra], funct7(mn), funct3(mn)) is Err
generators:
  extra: { gen: oneof, options: ["Imm", "Reg", "RoundingMode", "Symbol", "Mem"] }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on 4th token; encoder/mod.rs:737-739 pass-through
```

## encode_fp_cmp_neg_rm_fourth
- Tier: 3
- Rationale: Negative/error contract distinct from extra-Imm: FEQ/FLT/FLE have no rounding-mode field (ISA uses funct3 as the comparison). llvm-mc rejects `feq.s a0, fa1, fa2, rne`. encode_fp_arith (different job) does accept a 4th RoundingMode; this symbol must not.
- Doc contract: (none) on encode_fp_cmp itself. Contrast float.rs:65 "Check for optional rounding mode" on encode_fp_arith — that comment is not this function's contract. RISC-V Unprivileged ISA FEQ/FLT/FLE: funct3 is the comparison, not rm. llvm-mc: "invalid operand for instruction" on rne.
- Seed: encode_fp_sgnj_pbt.rs encode_fp_sgnj_neg_rm_fourth
- Formal: ∀ mn ∈ CMP, rd ∈ GPRNames, rs1,rs2 ∈ FPRNames, rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fp_cmp([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode(rm)], funct7(mn), funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_cmp_pbt.rs
- Status: failing
- Counterexample: encode_fp_cmp([Reg("x0"), Reg("f0"), Reg("f0"), RoundingMode("rne")], 0b1010000, 0b010)
- Bug report: pbt-out/bug_reports/encode_fp_cmp_rm_fourth.md

```property
function: encoder.encode_fp_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rm]
  domain: { mn: CMP, rd: GPRNames, rs1: FPRNames, rs2: FPRNames, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  body: encode_fp_cmp([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode(rm)], funct7(mn), funct3(mn)) is Err
generators:
  rm: { gen: oneof, options: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
expected_error: String
evidence: RISC-V Unprivileged ISA FEQ/FLT/FLE no rm; llvm-mc rejects 4th rne
```
