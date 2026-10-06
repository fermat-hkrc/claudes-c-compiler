# Properties: encode_fmv_f_x

## encode_fmv_f_x_diff_2op_llvm_mc
- Tier: 5
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_fmv_f_x is a pure function with no lifecycle. Algebraic round-trip via in-tree decoder rejected: no OP-FP decoder. encode_r / encode_fclass / encode_fmv_x_f as differential sibling rejected by same-job gate (private packer / FCLASS uses funct3=001 / FMV.X.W is the opposite direction). Reference ISA field layout is used as a weaker invariant property, not this one.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:169 "Integer to float register move" — asserted fingerprint 2b1739f2
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_diff_2op_llvm_mc (same 2-op OP-FP FMV shape, opposite register classes)
- Formal: ∀ mn ∈ {fmv.w.x, fmv.s.x, fmv.d.x}, rd ∈ FPNames, rs1 ∈ GPRNames. encode_fmv_f_x([Reg(rd), Reg(rs1)], funct7(mn), 0) = Word(llvm-mc(mn rd, rs1))
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_f_x
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: {fmv.w.x, fmv.s.x, fmv.d.x}, rd: fp_names, rs1: gpr_names }
  relation:
    op: eq
    lhs: encode_fmv_f_x([Reg(rd), Reg(rs1)], funct7(mn), 0)
    rhs: llvm_mc(mn + " " + rd + ", " + rs1)
generators:
  mn: { gen: oneof, items: ["fmv.w.x", "fmv.s.x", "fmv.d.x"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/README.md:308; encoder/mod.rs:764; encoder/mod.rs:794
```

## encode_fmv_f_x_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout and ISA FMV.W.X/D hardwires (opcode OP-FP, funct3=000, rs2=0). Stronger differential is a sibling property. Round-trip rejected (no decoder).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:336 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_r_type_fields
- Formal: ∀ rd, rs1 ∈ 0..31, f7 ∈ {0b1111000, 0b1111001}. unpack_r(encode_fmv_f_x([Reg(f{rd}), Reg(x{rs1})], f7, 0)) = (OP_OP_FP, 0b000, rd, rs1, 0, f7)
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_f_x
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, f7]
  domain: { rd: 0..31, rs1: 0..31, f7: {0b1111000, 0b1111001} }
  body: unpack_r(word) == (0b1010011, 0b000, rd, rs1, 0, f7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  f7: { gen: oneof, items: [120, 121] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:336
```

## encode_fmv_f_x_abi_fn_xn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names and fN/xN names are aliases of the same 5-bit register index (parser/freg_num, reg_num). Stronger differential is a sibling property over mixed names.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:169 "Integer to float register move" — asserted fingerprint 2b1739f2
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_abi_xn_fn_alias
- Formal: ∀ n, m ∈ 0..31, f7 ∈ {0b1111000, 0b1111001}. encode_fmv_f_x([Reg(f{n}), Reg(x{m})], f7, 0) = encode_fmv_f_x([Reg(FABI[n]), Reg(GABI[m])], f7, 0)
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_f_x
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, f7]
  domain: { n: 0..31, m: 0..31, f7: {0b1111000, 0b1111001} }
  relation:
    op: eq
    lhs: encode_fmv_f_x([Reg(fn(n)), Reg(xn(m))], f7, 0)
    rhs: encode_fmv_f_x([Reg(fabi(n)), Reg(gabi(m))], f7, 0)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  f7: { gen: oneof, items: [120, 121] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:207
```

## encode_fmv_f_x_s_vs_d_fmt
- Tier: 4
- Rationale: Metamorphic: FMV.W.X vs FMV.D.X differ only in funct7 bit 0 (fmt), which is bit 25 of the word. Documented by dispatch 0b1111000 vs 0b1111001.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:764 `"fmv.w.x" | "fmv.s.x" => encode_fmv_f_x(operands, 0b1111000, 0b00)` — asserted fingerprint 2f466653
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_s_vs_d_fmt
- Formal: ∀ rd, rs1 ∈ 0..31. encode_fmv_f_x(ops, 0b1111000, 0) xor encode_fmv_f_x(ops, 0b1111001, 0) = 1<<25
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_f_x
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1]
  domain: { rd: 0..31, rs1: 0..31 }
  body: encode_fmv_f_x(ops, 0b1111000, 0) xor encode_fmv_f_x(ops, 0b1111001, 0) == (1 << 25)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:764; encoder/mod.rs:794
```

## encode_fmv_f_x_w_vs_s_alias
- Tier: 4
- Rationale: Metamorphic plus differential: fmv.w.x and fmv.s.x share funct7=0b1111000 (dispatch OR-pattern). llvm-mc encodes them identically. Stronger 2-op differential is a sibling covering all three mnemonics.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:764 `"fmv.w.x" | "fmv.s.x" => encode_fmv_f_x(operands, 0b1111000, 0b00)` — asserted fingerprint 2f466653
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_w_vs_s_alias
- Formal: ∀ rd ∈ FPNames, rs1 ∈ GPRNames. encode_fmv_f_x([Reg(rd), Reg(rs1)], 0b1111000, 0) = llvm-mc(fmv.w.x rd, rs1) = llvm-mc(fmv.s.x rd, rs1)
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_f_x
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs1]
  domain: { rd: fp_names, rs1: gpr_names }
  body: sut == llvm_mc("fmv.w.x " + rd + ", " + rs1) == llvm_mc("fmv.s.x " + rd + ", " + rs1)
generators:
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/mod.rs:764
```

## encode_fmv_f_x_neg_arity_class
- Tier: 3
- Rationale: Negative/error contract: FMV.W.X/D.X is a 2-operand instruction with FP rd and integer rs1. llvm-mc rejects empty, 1-operand, GPR rd, FP rs1, and non-register rd. get_freg does not accept Imm for rd. Stronger differential does not apply on the invalid domain. These inputs are documented-invalid, not a SUT mishandling.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:169 "Integer to float register move" — asserted fingerprint 2b1739f2
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_neg_arity_class
- Formal: ∀ f7 ∈ {0b1111000, 0b1111001}, fp ∈ FPNames, gpr ∈ GPRNames, bad ∉ FPRegs. encode_fmv_f_x([], f7, 0) is Err ∧ encode_fmv_f_x([Reg(fp)], f7, 0) is Err ∧ encode_fmv_f_x([Reg(gpr), Reg(gpr)], f7, 0) is Err ∧ encode_fmv_f_x([Reg(fp), Reg(fp)], f7, 0) is Err ∧ encode_fmv_f_x([bad, Reg(gpr)], f7, 0) is Err ∧ encode_fmv_f_x([Imm(0), Reg(gpr)], f7, 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_f_x
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f7, fp, gpr, bad]
  domain: { f7: {0b1111000, 0b1111001}, fp: fp_names, gpr: gpr_names, bad: non_fp_reg }
  body: encode_fmv_f_x(invalid, f7, 0) is Err
generators:
  f7: { gen: oneof, items: [120, 121] }
  fp: { gen: string }
  gpr: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/float.rs:170
```

## encode_fmv_f_x_neg_extra
- Tier: 3
- Rationale: Negative/error contract: llvm-mc rejects a 3rd operand on fmv.w.x/s.x/d.x ("invalid operand for instruction"). The ISA form is 2-operand; extra tokens must be Err. Stronger differential does not apply on the invalid domain.
- Doc contract: src/backend/riscv/assembler/README.md:308 "fmv.x.w/d, fmv.w.x/d.x" — asserted fingerprint 8b41c309
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_neg_extra
- Formal: ∀ mn ∈ {fmv.w.x, fmv.s.x, fmv.d.x}, rd ∈ FPNames, rs1 ∈ GPRNames, extra ∈ Operand. encode_fmv_f_x([Reg(rd), Reg(rs1), extra], funct7(mn), 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: failing
- Counterexample: encode_fmv_f_x([Reg("f0"), Reg("x0"), Imm(0)], 0b1111000, 0) = Ok(Word(0xf0000053))
- Bug report: bug_reports/encode_fmv_f_x_extra_operand.md

```property
function: encoder.encode_fmv_f_x
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: {fmv.w.x, fmv.s.x, fmv.d.x}, rd: fp_names, rs1: gpr_names, extra: Operand }
  body: encode_fmv_f_x([Reg(rd), Reg(rs1), extra], funct7(mn), 0) is Err
generators:
  mn: { gen: oneof, items: ["fmv.w.x", "fmv.s.x", "fmv.d.x"] }
  rd: { gen: string }
  rs1: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:308
```

## encode_fmv_f_x_neg_rm_third
- Tier: 3
- Rationale: Negative/error contract: FMV.W.X/D.X has no rounding-mode field (funct3 hardwired 000, unlike FCVT). llvm-mc rejects a 3rd rne/rtz/rdn/rup/rmm/dyn token. Stronger differential does not apply on the invalid domain.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:169 "Integer to float register move" — asserted fingerprint 2b1739f2
- Seed: encode_fmv_x_f_pbt.rs:encode_fmv_x_f_neg_rm_third
- Formal: ∀ mn ∈ {fmv.w.x, fmv.s.x, fmv.d.x}, rd ∈ FPNames, rs1 ∈ GPRNames, rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fmv_f_x([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_f_x_pbt.rs
- Status: failing
- Counterexample: encode_fmv_f_x([Reg("f0"), Reg("x0"), RoundingMode("rne")], 0b1111000, 0) = Ok(Word(0xf0000053))
- Bug report: bug_reports/encode_fmv_f_x_rm_third.md

```property
function: encoder.encode_fmv_f_x
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: {fmv.w.x, fmv.s.x, fmv.d.x}, rd: fp_names, rs1: gpr_names, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  body: encode_fmv_f_x([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), 0) is Err
generators:
  mn: { gen: oneof, items: ["fmv.w.x", "fmv.s.x", "fmv.d.x"] }
  rd: { gen: string }
  rs1: { gen: string }
  rm: { gen: oneof, items: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/float.rs:172
```
