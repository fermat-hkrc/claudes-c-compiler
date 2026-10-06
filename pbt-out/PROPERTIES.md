# Properties: encode_fclass

## encode_fclass_diff_2op_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent RISC-V assembler). State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no OP-FP decoder in tree. Same-job sibling encode_fp_cmp / encode_fp_unary / encode_fmv_x_f / encode_r rejected — encode_fp_cmp is a 3-operand compare (rs2 live); encode_fp_unary uses rm in funct3 for FSQRT; encode_fmv_x_f uses funct3=000 (bit-move, not classify); encode_r is a private packer. llvm-mc is an independent assembler the SUT's README claims to replace for RV64GC textual assembly.
- Doc contract: (none) — encode_fclass has no function-level comment. README.md:309 "fclass" — asserted fingerprint bdb9d526
- Seed: encode_fp_cmp_pbt.rs encode_fp_cmp_diff_3op_llvm_mc
- Formal: ∀ mn ∈ {fclass.s, fclass.d}, rd ∈ GPRNames, rs1 ∈ FPRNames. encode_fclass([Reg(rd), Reg(rs1)], funct7(mn)) = Word(llvm-mc(mn rd, rs1))
- Test file: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fclass
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: {fclass.s,fclass.d}, rd: GPRNames, rs1: FPRNames }
  relation:
    op: eq
    lhs: encode_fclass([Reg(rd), Reg(rs1)], funct7(mn))
    rhs: Word(llvm_mc(mn + " " + rd + ", " + rs1))
generators:
  mn: { gen: oneof, options: ["fclass.s", "fclass.d"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: README.md:309 fclass; encoder/mod.rs:742 and :770 dispatch; RISC-V Unprivileged ISA FCLASS.S/D
```

## encode_fclass_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout. Stronger differential is p1; this pins opcode/rd/rs1/rs2=0/funct3=001/funct7 independently of llvm-mc so a mapping bug cannot hide a packer bug.
- Doc contract: encoder/mod.rs:313 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: encode_fp_cmp_pbt.rs encode_fp_cmp_r_type_fields
- Formal: ∀ rd ∈ 0..31, rs1 ∈ 0..31, f7 ∈ {0b1110000, 0b1110001}. let w = encode_fclass([Reg(x{rd}), Reg(f{rs1})], f7). unpack_r(w) = (OP_OP_FP, 0b001, rd, rs1, 0, f7)
- Test file: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fclass
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, f7]
  domain: { rd: 0..31, rs1: 0..31, f7: {0b1110000,0b1110001} }
  body: unpack_r(encode_fclass([Reg(x{rd}),Reg(f{rs1})], f7)) == (0b1010011, 0b001, rd, rs1, 0, f7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  f7: { gen: oneof, options: [112, 113] }
evidence: encoder/mod.rs:313 R-type layout; encoder/mod.rs:377 OP_OP_FP; float.rs:113 funct3=001 rs2=0
```

## encode_fclass_abi_xn_fn_alias
- Tier: 4
- Rationale: Algebraic metamorphic — ABI names and numeric names are documented aliases of the same 5-bit encoding (reg_num / freg_num). Independent of llvm-mc.
- Doc contract: parser.rs:22-23 "Register: x0-x31, zero, ra, sp, gp, tp, t0-t6, s0-s11, a0-a7, f0-f31, ft0-ft11, fs0-fs11, fa0-fa7" — asserted (alias table). encoder/mod.rs:194-248 GPR ABI; encoder/mod.rs:245-293 FPR ABI.
- Seed: encode_fp_cmp_pbt.rs encode_fp_cmp_abi_xn_fn_alias
- Formal: ∀ n,m ∈ 0..31, f7 ∈ {0b1110000, 0b1110001}. encode_fclass([Reg(x{n}), Reg(f{m})], f7) = encode_fclass([Reg(gabi(n)), Reg(fabi(m))], f7)
- Test file: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fclass
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, f7]
  domain: { n: 0..31, m: 0..31, f7: {0b1110000,0b1110001} }
  relation:
    op: eq
    lhs: encode_fclass([Reg(x{n}), Reg(f{m})], f7)
    rhs: encode_fclass([Reg(gabi(n)), Reg(fabi(m))], f7)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  f7: { gen: oneof, options: [112, 113] }
evidence: parser.rs:22-23 register alias table; encoder/mod.rs:194-293 ABI maps
```

## encode_fclass_s_vs_d_fmt
- Tier: 4
- Rationale: Algebraic metamorphic — FCLASS.S and FCLASS.D share rd/rs1/funct3/rs2/opcode and differ only in funct7 fmt bit 0 (ISA funct5+fmt). Documented by dispatch encoder/mod.rs:742 vs :770.
- Doc contract: encoder/mod.rs:742 "fclass.s" => encode_fclass(operands, 0b1110000); encoder/mod.rs:770 "fclass.d" => encode_fclass(operands, 0b1110001)
- Seed: encode_fp_cmp_pbt.rs encode_fp_cmp_r_type_fields
- Formal: ∀ rd ∈ 0..31, rs1 ∈ 0..31. let ws = encode_fclass([Reg(x{rd}), Reg(f{rs1})], 0b1110000); let wd = encode_fclass([Reg(x{rd}), Reg(f{rs1})], 0b1110001). (ws xor wd) = (1 << 25) ∧ unpack_r(ws).funct7 = 0b1110000 ∧ unpack_r(wd).funct7 = 0b1110001
- Test file: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fclass
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1]
  domain: { rd: 0..31, rs1: 0..31 }
  body: (encode_fclass(ops, 0b1110000) xor encode_fclass(ops, 0b1110001)) == (1 << 25)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:742 fclass.s funct7=1110000; encoder/mod.rs:770 fclass.d funct7=1110001
```

## encode_fclass_neg_arity_class
- Tier: 3
- Rationale: Negative/error contract. llvm-mc rejects empty, 1-operand, FP rd, GPR rs1, non-Reg rd, and Imm as rs1. Dispatch passes the operand slice through unchanged, so encode_fclass owns the rejection. get_reg/get_freg error strings are the documented failure.
- Doc contract: (none) — encode_fclass has no function-level comment. llvm-mc "too few operands" / "invalid operand for instruction"; get_reg encoder/mod.rs:397-405; get_freg encoder/mod.rs:408-414
- Seed: encode_fp_cmp_pbt.rs encode_fp_cmp_neg_arity_class
- Formal: ∀ f7 ∈ {0b1110000, 0b1110001}, fp ∈ FPRNames, gpr ∈ GPRNames, bad ∈ NonRegRd. encode_fclass([], f7) is Err ∧ encode_fclass([Reg(gpr)], f7) is Err ∧ encode_fclass([Reg(fp), Reg(fp)], f7) is Err ∧ encode_fclass([Reg(gpr), Reg(gpr)], f7) is Err ∧ encode_fclass([bad, Reg(fp)], f7) is Err ∧ encode_fclass([Reg(gpr), Imm(0)], f7) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fclass
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f7, fp, gpr, bad]
  domain: { f7: {0b1110000,0b1110001}, fp: FPRNames, gpr: GPRNames, bad: NonRegRd }
  relation:
    op: throws
    expr: encode_fclass(invalid_ops, f7)
expected_error: String
generators:
  f7: { gen: oneof, options: [112, 113] }
  fp: { gen: string }
  gpr: { gen: string }
evidence: llvm-mc rejects missing/wrong-class operands; get_reg encoder/mod.rs:397; get_freg encoder/mod.rs:408
```

## encode_fclass_neg_extra
- Tier: 3
- Rationale: Negative/error contract. llvm-mc rejects a 3rd operand ("invalid operand for instruction"). encode_instruction passes the full slice through, so encode_fclass must reject operands.len() > 2. No documented exclusion of extra operands.
- Doc contract: (none) — encode_fclass has no function-level comment. README.md:6-7 assembler of codegen textual assembly; llvm-mc rejects extra
- Seed: encode_fp_cmp_pbt.rs encode_fp_cmp_neg_extra
- Formal: ∀ mn ∈ {fclass.s, fclass.d}, rd ∈ GPRNames, rs1 ∈ FPRNames, extra ∈ Operand. encode_fclass([Reg(rd), Reg(rs1), extra], funct7(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- Status: failing
- Counterexample: encode_fclass([Reg("x0"), Reg("f0"), Imm(0)], 0b1110000) = Ok(Word(0xe0001053))
- Bug report: bug_reports/encode_fclass_extra_operand.md

```property
function: encoder.encode_fclass
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: {fclass.s,fclass.d}, rd: GPRNames, rs1: FPRNames, extra: Operand }
  relation:
    op: throws
    expr: encode_fclass([Reg(rd), Reg(rs1), extra], funct7(mn))
expected_error: String
generators:
  mn: { gen: oneof, options: ["fclass.s", "fclass.d"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: llvm-mc rejects extra operands for fclass.s/fclass.d; encoder/mod.rs:742/770 pass operands through
```

## encode_fclass_neg_rm_third
- Tier: 3
- Rationale: Negative/error contract. FCLASS has no rounding-mode field (funct3 is hardwired 001, unlike FSQRT). llvm-mc rejects a 3rd rne/rtz/... token. A 3rd RoundingMode must Err, not be ignored.
- Doc contract: (none) — encode_fclass has no function-level comment. RISC-V Unprivileged ISA FCLASS: funct3=001, rs2=0, no rm. llvm-mc "invalid operand for instruction"
- Seed: encode_fp_cmp_pbt.rs encode_fp_cmp_neg_rm_fourth
- Formal: ∀ mn ∈ {fclass.s, fclass.d}, rd ∈ GPRNames, rs1 ∈ FPRNames, rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fclass([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fclass_pbt.rs
- Status: failing
- Counterexample: encode_fclass([Reg("x0"), Reg("f0"), RoundingMode("rne")], 0b1110000) = Ok(Word(0xe0001053))
- Bug report: bug_reports/encode_fclass_rm_third.md

```property
function: encoder.encode_fclass
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: {fclass.s,fclass.d}, rd: GPRNames, rs1: FPRNames, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  relation:
    op: throws
    expr: encode_fclass([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn))
expected_error: String
generators:
  mn: { gen: oneof, options: ["fclass.s", "fclass.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rm: { gen: oneof, options: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: RISC-V Unprivileged ISA FCLASS has no rm field; llvm-mc rejects a 3rd rne token
```
