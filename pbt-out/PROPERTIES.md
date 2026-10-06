# Properties: encode_fcvt_fp

## encode_fcvt_fp_diff_2op_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected — encode_fcvt_fp is a pure function with no lifecycle. Algebraic round-trip via in-tree decoder rejected — no OP-FP decoder. encode_r / encode_fp_unary / encode_fcvt_int as differential sibling rejected — same-job gate (private packer / FSQRT unary / float-to-int). Reference KAT vectors used as a gate, not the primary search. llvm-mc owns the RV64GC assembly encoding contract this assembler claims (README.md:6-7, encoder/mod.rs:3-4).
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:147 "Float to float conversion (e.g., fcvt.s.d, fcvt.d.s)" — asserted fingerprint 61b8af26
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:332 encode_fp_unary_diff_2op_llvm_mc
- Formal: ∀ mn ∈ {fcvt.s.d, fcvt.d.s}, ∀ rd, rs1 ∈ FPRegs. encode_fcvt_fp([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn)) = llvm-mc(mn rd, rs1)
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_fp([Reg("f0"), Reg("f0")], 0b0100001, 0) vs llvm-mc("fcvt.d.s f0, f0"): SUT 0x42007053 != llvm-mc 0x42000053
- Bug report: pbt-out/bug_reports/encode_fcvt_fp_d_s_omitted_rm.md

```property
function: encoder.encode_fcvt_fp
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: {fcvt.s.d, fcvt.d.s}, rd: FPRegs, rs1: FPRegs }
  relation:
    op: eq
    lhs: encode_fcvt_fp([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + rs1)
generators:
  mn: { gen: oneof, items: ["fcvt.s.d", "fcvt.d.s"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/mod.rs:3
```

## encode_fcvt_fp_diff_rm_llvm_mc
- Tier: 5
- Rationale: Same differential as 2-op, covering the optional RoundingMode operand that the ISA places in funct3. Domain is FCVT.S.D only: llvm-mc 15 rejects an rm operand on FCVT.D.S (exact widening). Generator enumerates the closed rm set {rne,rtz,rdn,rup,rmm,dyn}.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:147 "Float to float conversion (e.g., fcvt.s.d, fcvt.d.s)" — asserted fingerprint 61b8af26
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:349 encode_fp_unary_diff_rm_llvm_mc
- Formal: ∀ rd, rs1 ∈ FPRegs, ∀ rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fcvt_fp([Reg(rd), Reg(rs1), RoundingMode(rm)], 0b0100000, 0b00001) = llvm-mc("fcvt.s.d" rd, rs1, rm)
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_fp
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1, rm]
  domain: { rd: FPRegs, rs1: FPRegs, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  relation:
    op: eq
    lhs: encode_fcvt_fp([Reg(rd), Reg(rs1), RoundingMode(rm)], 0b0100000, 0b00001)
    rhs: llvm_mc("fcvt.s.d " + rd + ", " + rs1 + ", " + rm)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  rm: { gen: oneof, items: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: src/backend/riscv/assembler/parser.rs:41
```

## encode_fcvt_fp_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout (encoder/mod.rs:331, README.md:352). Weaker than differential; kept because it pins opcode/rm/rd/rs1/rs2/funct7 independently of llvm-mc. Domain includes both mnemonics and every rm.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:331 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:366 encode_fp_unary_r_type_fields
- Formal: ∀ mn ∈ {fcvt.s.d, fcvt.d.s}, ∀ rd, rs1 ∈ 0..31, ∀ rm ∈ RM. let w = encode_fcvt_fp([Reg(f{rd}), Reg(f{rs1}), RoundingMode(rm.name)], funct7(mn), rs2(mn)) in unpack_r(w) = (OP_OP_FP, rm.bits, rd, rs1, rs2(mn), funct7(mn))
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_fp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: {fcvt.s.d, fcvt.d.s}, rd: 0..31, rs1: 0..31, rm: RM }
  relation:
    op: eq
    lhs: unpack_r(encode_fcvt_fp([Reg(f{rd}), Reg(f{rs1}), RoundingMode(rm.name)], funct7(mn), rs2(mn)))
    rhs: (OP_OP_FP, rm.bits, rd, rs1, rs2(mn), funct7(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.s.d", "fcvt.d.s"] }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: oneof, items: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:331
```

## encode_fcvt_fp_abi_fn_alias
- Tier: 4
- Rationale: Algebraic metamorphic: ABI names (ft0/fa0/fs0/...) must encode the same rd/rs1 as fN. Documented by parser.rs Operand::Reg comment listing both spellings. Independent of llvm-mc.
- Doc contract: src/backend/riscv/assembler/parser.rs:22 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7," — asserted fingerprint 8f55b73d
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:389 encode_fp_unary_abi_fn_alias
- Formal: ∀ mn ∈ {fcvt.s.d, fcvt.d.s}, ∀ n, m ∈ 0..31. encode_fcvt_fp([Reg(f{n}), Reg(f{m})], funct7(mn), rs2(mn)) = encode_fcvt_fp([Reg(FABI[n]), Reg(FABI[m])], funct7(mn), rs2(mn))
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_fp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, n, m]
  domain: { mn: {fcvt.s.d, fcvt.d.s}, n: 0..31, m: 0..31 }
  relation:
    op: eq
    lhs: encode_fcvt_fp([Reg(f{n}), Reg(f{m})], funct7(mn), rs2(mn))
    rhs: encode_fcvt_fp([Reg(FABI[n]), Reg(FABI[m])], funct7(mn), rs2(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.s.d", "fcvt.d.s"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/parser.rs:22
```

## encode_fcvt_fp_rm_default_dyn
- Tier: 4
- Rationale: Algebraic: omitted rm equals explicit RoundingMode("dyn") and unpacks rm=111. encoder/mod.rs:485 and the function body default omitted/non-rm third to 0b111. Documented default, not guessed from current output alone — parse_rm maps "dyn" to 0b111 and the 2-operand path hardwires 0b111.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:485 "Parse a rounding mode to 3-bit encoding." — asserted fingerprint a7ee753b
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:404 encode_fp_unary_rm_default_dyn
- Formal: ∀ mn ∈ {fcvt.s.d, fcvt.d.s}, ∀ rd, rs1 ∈ FPRegs. encode_fcvt_fp([Reg(rd), Reg(rs1)], f7, rs2) = encode_fcvt_fp([Reg(rd), Reg(rs1), RoundingMode("dyn")], f7, rs2) ∧ unpack_r(...).funct3 = 0b111
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_fp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: {fcvt.s.d, fcvt.d.s}, rd: FPRegs, rs1: FPRegs }
  relation:
    op: eq
    lhs: encode_fcvt_fp([Reg(rd), Reg(rs1)], funct7(mn), rs2(mn))
    rhs: encode_fcvt_fp([Reg(rd), Reg(rs1), RoundingMode("dyn")], funct7(mn), rs2(mn))
generators:
  mn: { gen: oneof, items: ["fcvt.s.d", "fcvt.d.s"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/mod.rs:485
```

## encode_fcvt_fp_neg_arity_gpr
- Tier: 4
- Rationale: Negative/error contract. get_freg returns Err for missing operands, GPR names, and non-Reg tokens. llvm-mc rejects the same class (too few operands / invalid operand). Evidence: get_freg (encoder/mod.rs:414) plus llvm-mc error "too few operands" / "invalid operand".
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:147 "Float to float conversion (e.g., fcvt.s.d, fcvt.d.s)" — asserted fingerprint 61b8af26
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:421 encode_fp_unary_neg_arity_gpr
- Formal: ∀ mn ∈ {fcvt.s.d, fcvt.d.s}, ∀ fp ∈ FPRegs, ∀ gpr ∈ GPRs, ∀ bad ∉ FPRegs. encode_fcvt_fp([], f7, rs2) is Err ∧ encode_fcvt_fp([Reg(fp)], f7, rs2) is Err ∧ encode_fcvt_fp([Reg(gpr), Reg(fp)], f7, rs2) is Err ∧ encode_fcvt_fp([Reg(fp), Reg(gpr)], f7, rs2) is Err ∧ encode_fcvt_fp([bad, Reg(fp)], f7, rs2) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fcvt_fp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, fp, gpr, bad]
  domain: { mn: {fcvt.s.d, fcvt.d.s}, fp: FPRegs, gpr: GPRs, bad: NonFpReg }
  relation:
    op: holds
    expr: encode_fcvt_fp([], f7, rs2).is_err() && encode_fcvt_fp([Reg(fp)], f7, rs2).is_err() && encode_fcvt_fp([Reg(gpr), Reg(fp)], f7, rs2).is_err() && encode_fcvt_fp([Reg(fp), Reg(gpr)], f7, rs2).is_err() && encode_fcvt_fp([bad, Reg(fp)], f7, rs2).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["fcvt.s.d", "fcvt.d.s"] }
  fp: { gen: string }
  gpr: { gen: string }
  bad: { gen: string }
evidence: src/backend/riscv/assembler/encoder/mod.rs:414
```

## encode_fcvt_fp_neg_extra
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 4th operand ("invalid operand for instruction"). README.md:6-7 claims the assembler encodes the same textual assembly the compiler emits; extra tokens are not a valid FCVT.S.D / FCVT.D.S form. The encoder must Err, not silently ignore.
- Doc contract: src/backend/riscv/assembler/README.md:308 "fcvt (all int/float conversions)" — asserted fingerprint b74faca6
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:456 encode_fp_unary_neg_extra
- Formal: ∀ mn ∈ {fcvt.s.d, fcvt.d.s}, ∀ rd, rs1 ∈ FPRegs, ∀ extra. encode_fcvt_fp([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], funct7(mn), rs2(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_fp([Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b0100000, 1) -> Ok(Word(0x40100053))
- Bug report: pbt-out/bug_reports/encode_fcvt_fp_extra_operand.md

```property
function: encoder.encode_fcvt_fp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: {fcvt.s.d, fcvt.d.s}, rd: FPRegs, rs1: FPRegs, extra: Operand }
  relation:
    op: holds
    expr: encode_fcvt_fp([Reg(rd), Reg(rs1), RoundingMode("rne"), extra], funct7(mn), rs2(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["fcvt.s.d", "fcvt.d.s"] }
  rd: { gen: string }
  rs1: { gen: string }
  extra: { gen: string }
evidence: src/backend/riscv/assembler/README.md:308
```

## encode_fcvt_fp_neg_non_rm_third
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 3rd token that is not a rounding-mode mnemonic ("operand must be a valid floating point rounding mode mnemonic"). Optional rm is the only legal 3rd operand (parser.rs:41). A non-RoundingMode 3rd must Err, not be mapped to DYN.
- Doc contract: src/backend/riscv/assembler/parser.rs:41 "Rounding mode: rne, rtz, rdn, rup, rmm, dyn" — asserted fingerprint 4d950ca5
- Seed: src/backend/riscv/assembler/encoder/encode_fp_unary_pbt.rs:474 encode_fp_unary_neg_non_rm_third
- Formal: ∀ mn ∈ {fcvt.s.d, fcvt.d.s}, ∀ rd, rs1 ∈ FPRegs, ∀ extra ∉ RoundingMode. encode_fcvt_fp([Reg(rd), Reg(rs1), extra], funct7(mn), rs2(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fcvt_fp_pbt.rs
- Status: failing
- Counterexample: encode_fcvt_fp([Reg("f0"), Reg("f0"), Imm(0)], 0b0100000, 1) -> Ok(Word(0x40107053))
- Bug report: pbt-out/bug_reports/encode_fcvt_fp_non_rm_third.md

```property
function: encoder.encode_fcvt_fp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: {fcvt.s.d, fcvt.d.s}, rd: FPRegs, rs1: FPRegs, extra: NonRoundingMode }
  relation:
    op: holds
    expr: encode_fcvt_fp([Reg(rd), Reg(rs1), extra], funct7(mn), rs2(mn)).is_err()
expected_error: String
generators:
  mn: { gen: oneof, items: ["fcvt.s.d", "fcvt.d.s"] }
  rd: { gen: string }
  rs1: { gen: string }
  extra: { gen: string }
evidence: src/backend/riscv/assembler/parser.rs:41
```
