# Properties: encode_fp_sgnj

## encode_fp_sgnj_diff_3op_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler). State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no OP-FP decoder in tree. Same-job sibling encode_fp_arith / encode_r rejected — encode_fp_arith uses rm in funct3 for FADD-family (different job); encode_r is a private packer. llvm-mc is an independent assembler the SUT's README claims to replace for RV64GC textual assembly.
- Doc contract: (none) — encode_fp_sgnj has no function-level rustdoc. Module contract encoder/mod.rs:3 "Encodes RISC-V instructions into 32-bit machine code words." — asserted fingerprint 077a9290. README.md:310 "fsgnj/fsgnjn/fsgnjx." — asserted fingerprint 2e953dbe.
- Seed: encode_fp_arith_pbt.rs:358 encode_fp_arith_diff_3op_llvm_mc
- Formal: ∀ mn ∈ {fsgnj.s, fsgnjn.s, fsgnjx.s, fmin.s, fmax.s, fsgnj.d, fsgnjn.d, fsgnjx.d, fmin.d, fmax.d}, ∀ rd, rs1, rs2 ∈ FPRegs. encode_fp_sgnj([Reg(rd), Reg(rs1), Reg(rs2)], funct7(mn), funct3(mn)) = llvm-mc("mn rd, rs1, rs2") as little-endian u32
- Test file: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_sgnj
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: FP_SGNJ_MNEMONICS, rd: FPRegs, rs1: FPRegs, rs2: FPRegs }
  relation:
    op: eq
    lhs: encode_fp_sgnj([Reg(rd), Reg(rs1), Reg(rs2)], funct7(mn), funct3(mn))
    rhs: llvm_mc_word("mn rd, rs1, rs2")
generators:
  mn: { gen: oneof, choices: ["fsgnj.s", "fsgnjn.s", "fsgnjx.s", "fmin.s", "fmax.s", "fsgnj.d", "fsgnjn.d", "fsgnjx.d", "fmin.d", "fmax.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
evidence: encoder/mod.rs:730-734 and :758-762 dispatch; README.md:310; llvm-mc RISC-V OP-FP
```

## encode_fp_sgnj_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout. Stronger differential is P1; this pins field placement independently of llvm-mc (opcode, funct3 as the op not rm, rd/rs1/rs2, funct7).
- Doc contract: encoder/mod.rs:313 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12.
- Seed: encode_fp_arith_pbt.rs:391 encode_fp_arith_r_type_fields
- Formal: ∀ mn ∈ FP_SGNJ_MNEMONICS, ∀ rd, rs1, rs2 ∈ 0..31. let w = encode_fp_sgnj([Reg(f{rd}), Reg(f{rs1}), Reg(f{rs2})], funct7(mn), funct3(mn)) in unpack_r(w) = (0b1010011, funct3(mn), rd, rs1, rs2, funct7(mn))
- Test file: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_sgnj
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2]
  domain: { mn: FP_SGNJ_MNEMONICS, rd: 0..31, rs1: 0..31, rs2: 0..31 }
  relation:
    op: eq
    lhs: unpack_r(encode_fp_sgnj([Reg(f{rd}), Reg(f{rs1}), Reg(f{rs2})], funct7(mn), funct3(mn)))
    rhs: (OP_OP_FP, funct3(mn), rd, rs1, rs2, funct7(mn))
generators:
  mn: { gen: oneof, choices: ["fsgnj.s", "fsgnjn.s", "fsgnjx.s", "fmin.s", "fmax.s", "fsgnj.d", "fsgnjn.d", "fsgnjx.d", "fmin.d", "fmax.d"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:313 R-type layout; encoder/mod.rs:377 OP_OP_FP
```

## encode_fp_sgnj_abi_fn_alias
- Tier: 4
- Rationale: Algebraic metamorphic — fN and ABI names (ft0/fa0/fs0/...) are the same 5-bit encodings per freg_num. Stronger differential is P1; this isolates alias equality.
- Doc contract: parser.rs:22-23 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7, f0-f31, ft0-ft11, fs0-fs11, fa0-fa7" — asserted fingerprint 2b0db8f8. encoder/mod.rs:240-286 freg_num maps both families onto 0..31.
- Seed: encode_fp_arith_pbt.rs:414 encode_fp_arith_abi_fn_alias
- Formal: ∀ mn ∈ FP_SGNJ_MNEMONICS, ∀ n, m, p ∈ 0..31. encode_fp_sgnj([Reg(f{n}), Reg(f{m}), Reg(f{p})], funct7(mn), funct3(mn)) = encode_fp_sgnj([Reg(ABI(n)), Reg(ABI(m)), Reg(ABI(p))], funct7(mn), funct3(mn))
- Test file: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_sgnj
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, m, p]
  domain: { mn: FP_SGNJ_MNEMONICS, n: 0..31, m: 0..31, p: 0..31 }
  relation:
    op: eq
    lhs: encode_fp_sgnj([Reg(f{n}), Reg(f{m}), Reg(f{p})], funct7(mn), funct3(mn))
    rhs: encode_fp_sgnj([Reg(ABI(n)), Reg(ABI(m)), Reg(ABI(p))], funct7(mn), funct3(mn))
generators:
  mn: { gen: oneof, choices: ["fsgnj.s", "fsgnjn.s", "fsgnjx.s", "fmin.s", "fmax.s", "fsgnj.d", "fsgnjn.d", "fsgnjx.d", "fmin.d", "fmax.d"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  p: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:240-286 freg_num ABI and fN maps; parser.rs:22-23
```

## encode_fp_sgnj_diff_rs1_eq_rs2_llvm_mc
- Tier: 2
- Rationale: Documented pseudo expansions fmv/fabs/fneg are fsgnj/fsgnjx/fsgnjn with rs1=rs2 (README.md:339-341). Generator is pinned to that documented case rather than leaving it to chance under P1. Differential vs llvm-mc on the six sign-injection mnemonics.
- Doc contract: README.md:339 "   | `fmv.s/d`      | `fsgnj.s/d rd, rs, rs`                               |" — asserted fingerprint 9f36f337.
- Seed: README.md:339-341 pseudo table; encode_fp_arith_pbt.rs 3-op differential
- Formal: ∀ mn ∈ {fsgnj.s, fsgnjn.s, fsgnjx.s, fsgnj.d, fsgnjn.d, fsgnjx.d}, ∀ rd, rs ∈ FPRegs. encode_fp_sgnj([Reg(rd), Reg(rs), Reg(rs)], funct7(mn), funct3(mn)) = llvm-mc("mn rd, rs, rs")
- Test file: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_sgnj
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs]
  domain: { mn: FSGNJ_ONLY_MNEMONICS, rd: FPRegs, rs: FPRegs }
  relation:
    op: eq
    lhs: encode_fp_sgnj([Reg(rd), Reg(rs), Reg(rs)], funct7(mn), funct3(mn))
    rhs: llvm_mc_word("mn rd, rs, rs")
generators:
  mn: { gen: oneof, choices: ["fsgnj.s", "fsgnjn.s", "fsgnjx.s", "fsgnj.d", "fsgnjn.d", "fsgnjx.d"] }
  rd: { gen: string }
  rs: { gen: string }
evidence: README.md:339-341 fmv/fabs/fneg expand to fsgnj/fsgnjx/fsgnjn rd, rs, rs
```

## encode_fp_sgnj_neg_arity_gpr
- Tier: 5
- Rationale: Negative/error contract. get_freg returns Err for missing operands, GPR names, and non-Reg variants. llvm-mc rejects the same (too few operands / invalid operand). Stronger oracles do not apply to the invalid domain.
- Doc contract: (none) on encode_fp_sgnj itself. get_freg at encoder/mod.rs:404-409 "expected float register at operand {}" / "invalid float register". parser.rs:22-23 register classes. llvm-mc: "too few operands" / "invalid operand".
- Seed: encode_fp_arith_pbt.rs:447 encode_fp_arith_neg_arity_gpr
- Formal: ∀ mn ∈ FP_SGNJ_MNEMONICS, ∀ fp, fp2 ∈ FPRegs, ∀ gpr ∈ GPRs, ∀ bad ∉ FP-Reg. encode_fp_sgnj([], f7, f3) is Err ∧ encode_fp_sgnj([Reg(fp)], f7, f3) is Err ∧ encode_fp_sgnj([Reg(fp), Reg(fp2)], f7, f3) is Err ∧ encode_fp_sgnj with GPR or non-Reg in any of the three slots is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fp_sgnj
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, fp, fp2, gpr, bad]
  domain: { mn: FP_SGNJ_MNEMONICS, fp: FPRegs, fp2: FPRegs, gpr: GPRs, bad: non_fp_reg_operand }
  relation:
    op: holds
    lhs: encode_fp_sgnj_rejects_arity_gpr_nonreg(mn, fp, fp2, gpr, bad)
    rhs: true
generators:
  mn: { gen: oneof, choices: ["fsgnj.s", "fsgnjn.s", "fsgnjx.s", "fmin.s", "fmax.s", "fsgnj.d", "fsgnjn.d", "fsgnjx.d", "fmin.d", "fmax.d"] }
  fp: { gen: string }
  fp2: { gen: string }
  gpr: { gen: string }
  bad: { gen: string }
expected_error: String
evidence: encoder/mod.rs:404-409 get_freg; llvm-mc rejects too few / GPR in FP slot
```

## encode_fp_sgnj_neg_extra
- Tier: 5
- Rationale: Negative/error contract. ISA R-type FSGNJ/FMIN take exactly three FP registers; llvm-mc rejects a 4th operand. The public dispatch passes operands through unchanged, so extra operands are caller-reachable. Stronger oracles do not apply to the invalid domain.
- Doc contract: (none) on encode_fp_sgnj itself. encoder/mod.rs:313 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12. llvm-mc: "invalid operand for instruction" on a 4th token.
- Seed: encode_fp_arith_pbt.rs:500 encode_fp_arith_neg_extra
- Formal: ∀ mn ∈ FP_SGNJ_MNEMONICS, ∀ rd, rs1, rs2 ∈ FPRegs, ∀ extra ∈ Operand. encode_fp_sgnj([Reg(rd), Reg(rs1), Reg(rs2), extra], funct7(mn), funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
- Status: failing
- Counterexample: encode_fp_sgnj([Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 0b0010000, 0b000) -> Ok(Word(0x20000053))
- Bug report: bug_reports/encode_fp_sgnj_extra_operand.md

```property
function: encoder.encode_fp_sgnj
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, extra]
  domain: { mn: FP_SGNJ_MNEMONICS, rd: FPRegs, rs1: FPRegs, rs2: FPRegs, extra: Operand }
  relation:
    op: holds
    lhs: encode_fp_sgnj([Reg(rd), Reg(rs1), Reg(rs2), extra], funct7(mn), funct3(mn)).is_err()
    rhs: true
generators:
  mn: { gen: oneof, choices: ["fsgnj.s", "fsgnjn.s", "fsgnjx.s", "fmin.s", "fmax.s", "fsgnj.d", "fsgnjn.d", "fsgnjx.d", "fmin.d", "fmax.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: README.md:352 R-type 3-register; llvm-mc rejects 4th operand; encoder/mod.rs:730-762 pass-through
```

## encode_fp_sgnj_neg_rm_fourth
- Tier: 5
- Rationale: Negative/error contract distinct from extra-Imm: FSGNJ/FMIN have no rounding-mode field (ISA uses funct3 as the op). llvm-mc rejects `fsgnj.s fa0, fa1, fa2, rne`. encode_fp_arith (different job) does accept a 4th RoundingMode; this symbol must not.
- Doc contract: (none) on encode_fp_sgnj itself. Contrast encoder/mod.rs:65 "Check for optional rounding mode" on encode_fp_arith — that comment is not this function's contract. RISC-V Unprivileged ISA FSGNJ/FMIN: funct3 is the operation, not rm. llvm-mc: "invalid operand for instruction" on rne.
- Seed: encode_fp_arith_pbt.rs:518 encode_fp_arith_neg_non_rm_fourth (inverted: here ANY 4th including RM is invalid)
- Formal: ∀ mn ∈ FP_SGNJ_MNEMONICS, ∀ rd, rs1, rs2 ∈ FPRegs, ∀ rm ∈ {rne, rtz, rdn, rup, rmm, dyn}. encode_fp_sgnj([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode(rm)], funct7(mn), funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fp_sgnj_pbt.rs
- Status: failing
- Counterexample: encode_fp_sgnj([Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne")], 0b0010000, 0b000) -> Ok(Word(0x20000053))
- Bug report: bug_reports/encode_fp_sgnj_rm_fourth.md

```property
function: encoder.encode_fp_sgnj
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rm]
  domain: { mn: FP_SGNJ_MNEMONICS, rd: FPRegs, rs1: FPRegs, rs2: FPRegs, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  relation:
    op: holds
    lhs: encode_fp_sgnj([Reg(rd), Reg(rs1), Reg(rs2), RoundingMode(rm)], funct7(mn), funct3(mn)).is_err()
    rhs: true
generators:
  mn: { gen: oneof, choices: ["fsgnj.s", "fsgnjn.s", "fsgnjx.s", "fmin.s", "fmax.s", "fsgnj.d", "fsgnjn.d", "fsgnjx.d", "fmin.d", "fmax.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  rm: { gen: oneof, choices: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
expected_error: String
evidence: RISC-V ISA FSGNJ/FMIN have no rm; llvm-mc rejects 4th rne; encoder/mod.rs:730-762
```
