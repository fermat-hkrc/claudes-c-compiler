# Properties: encode_fmv_x_f

## encode_fmv_x_f_diff_2op_llvm_mc
- Tier: 5
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_fmv_x_f is a pure function with no lifecycle. Algebraic round-trip via in-tree decoder rejected: no OP-FP decoder. encode_r / encode_fclass as differential sibling rejected by same-job gate (private packer / FCLASS uses funct3=001). Reference ISA field layout is used as a weaker invariant property, not this one.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:162 "Float to integer register move" — asserted fingerprint b97c6a3e
- Seed: encode_fclass_pbt.rs:encode_fclass_diff_2op_llvm_mc (same 2-op GPR-dest/FP-src OP-FP shape)
- Formal: ∀ mn ∈ {fmv.x.w, fmv.x.s, fmv.x.d}, rd ∈ GPRNames, rs1 ∈ FPNames. encode_fmv_x_f([Reg(rd), Reg(rs1)], funct7(mn), 0) = Word(llvm-mc(mn rd, rs1))
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_x_f
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1]
  domain: { mn: {fmv.x.w, fmv.x.s, fmv.x.d}, rd: gpr_names, rs1: fp_names }
  relation:
    op: eq
    lhs: encode_fmv_x_f([Reg(rd), Reg(rs1)], funct7(mn), 0)
    rhs: llvm_mc(mn + " " + rd + ", " + rs1)
generators:
  mn: { gen: oneof, items: ["fmv.x.w", "fmv.x.s", "fmv.x.d"] }
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/README.md:308; encoder/mod.rs:759; encoder/mod.rs:789
```

## encode_fmv_x_f_r_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R-type layout and ISA FMV.X hardwires (opcode OP-FP, funct3=000, rs2=0). Stronger differential is a sibling property. Round-trip rejected (no decoder).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:333 "R-type: funct7[31:25] | rs2[24:20] | rs1[19:15] | funct3[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint 34009d12
- Seed: encode_fclass_pbt.rs:encode_fclass_r_type_fields
- Formal: ∀ rd, rs1 ∈ 0..31, f7 ∈ {0b1110000, 0b1110001}. unpack_r(encode_fmv_x_f([Reg(x{rd}), Reg(f{rs1})], f7, 0)) = (OP_OP_FP, 0b000, rd, rs1, 0, f7)
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_x_f
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, f7]
  domain: { rd: 0..31, rs1: 0..31, f7: {0b1110000, 0b1110001} }
  body: unpack_r(word) == (0b1010011, 0b000, rd, rs1, 0, f7)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  f7: { gen: oneof, items: [112, 113] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:333
```

## encode_fmv_x_f_abi_xn_fn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names and xN/fN names are aliases of the same 5-bit register index (parser/reg_num, freg_num). Stronger differential is a sibling property over mixed names.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:162 "Float to integer register move" — asserted fingerprint b97c6a3e
- Seed: encode_fclass_pbt.rs:encode_fclass_abi_xn_fn_alias
- Formal: ∀ n, m ∈ 0..31, f7 ∈ {0b1110000, 0b1110001}. encode_fmv_x_f([Reg(x{n}), Reg(f{m})], f7, 0) = encode_fmv_x_f([Reg(GABI[n]), Reg(FABI[m])], f7, 0)
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_x_f
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, f7]
  domain: { n: 0..31, m: 0..31, f7: {0b1110000, 0b1110001} }
  relation:
    op: eq
    lhs: encode_fmv_x_f([Reg(xn(n)), Reg(fn(m))], f7, 0)
    rhs: encode_fmv_x_f([Reg(gabi(n)), Reg(fabi(m))], f7, 0)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  f7: { gen: oneof, items: [112, 113] }
evidence: src/backend/riscv/assembler/encoder/mod.rs:204
```

## encode_fmv_x_f_s_vs_d_fmt
- Tier: 4
- Rationale: Metamorphic: FMV.X.W vs FMV.X.D differ only in funct7 bit 0 (fmt), which is bit 25 of the word. Documented by dispatch 0b1110000 vs 0b1110001.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:759 `"fmv.x.w" | "fmv.x.s" => encode_fmv_x_f(operands, 0b1110000, 0b00)` — asserted fingerprint 6b6cd8b8
- Seed: encode_fclass_pbt.rs:encode_fclass_s_vs_d_fmt
- Formal: ∀ rd, rs1 ∈ 0..31. encode_fmv_x_f(ops, 0b1110000, 0) xor encode_fmv_x_f(ops, 0b1110001, 0) = 1<<25
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_x_f
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1]
  domain: { rd: 0..31, rs1: 0..31 }
  relation:
    op: eq
    lhs: encode_fmv_x_f(ops, 0b1110000, 0) xor encode_fmv_x_f(ops, 0b1110001, 0)
    rhs: 1 << 25
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:759; encoder/mod.rs:789
```

## encode_fmv_x_f_w_vs_s_alias
- Tier: 4
- Rationale: Metamorphic alias: dispatcher maps both fmv.x.w and fmv.x.s to the same funct7=0b1110000. llvm-mc canonicalizes fmv.x.s to fmv.x.w with identical encoding. Property checks that the SUT word for that shared funct7 equals llvm-mc of both mnemonics.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:759 `"fmv.x.w" | "fmv.x.s" => encode_fmv_x_f(operands, 0b1110000, 0b00)` — asserted fingerprint 6b6cd8b8
- Seed: (none) — alias is dispatcher-documented
- Formal: ∀ rd ∈ GPRNames, rs1 ∈ FPNames. encode_fmv_x_f([Reg(rd), Reg(rs1)], 0b1110000, 0) = llvm-mc("fmv.x.w rd, rs1") = llvm-mc("fmv.x.s rd, rs1")
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_x_f
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1]
  domain: { rd: gpr_names, rs1: fp_names }
  body: sut_word(ops, 0b1110000) == llvm_mc("fmv.x.w "+rd+", "+rs1) == llvm_mc("fmv.x.s "+rd+", "+rs1)
generators:
  rd: { gen: string }
  rs1: { gen: string }
evidence: src/backend/riscv/assembler/encoder/mod.rs:759
```

## encode_fmv_x_f_neg_arity_class
- Tier: 3
- Rationale: Negative/error contract: missing operands, FP in the GPR slot, GPR in the FP slot, and non-Reg tokens must Err. llvm-mc rejects those. get_reg/get_freg document the class checks. Imm(0..=31) is accepted by get_reg as a GCC bare GPR number (not in the invalid-rd domain).
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:405-409 get_reg expected-register / invalid integer register — asserted fingerprint f1b1a1fb
- Seed: encode_fclass_pbt.rs:encode_fclass_neg_arity_class
- Formal: ∀ f7 ∈ {0b1110000, 0b1110001}. encode_fmv_x_f([], f7, 0) is Err ∧ encode_fmv_x_f([gpr], f7, 0) is Err ∧ encode_fmv_x_f([fp, fp], f7, 0) is Err ∧ encode_fmv_x_f([gpr, gpr], f7, 0) is Err ∧ encode_fmv_x_f([bad, fp], f7, 0) is Err ∧ encode_fmv_x_f([gpr, Imm(0)], f7, 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fmv_x_f
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f7, fp, gpr, bad]
  domain: { f7: {0b1110000, 0b1110001}, fp: fp_names, gpr: gpr_names, bad: non_reg }
  body: encode_fmv_x_f(invalid, f7, 0) is Err
generators:
  f7: { gen: oneof, items: [112, 113] }
  fp: { gen: string }
  gpr: { gen: string }
  bad: { gen: oneof }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:405; encoder/mod.rs:416
```

## encode_fmv_x_f_neg_extra
- Tier: 3
- Rationale: Negative/error contract: FMV.X takes exactly two operands. llvm-mc errors on a 3rd operand. The SUT currently ignores extra operands (same class as encode_fclass B1) — keep the llvm-mc contract, do not collapse.
- Doc contract: src/backend/riscv/assembler/README.md:308 "fcvt (all int/float conversions), fmv.x.w/d, fmv.w.x/d.x," — asserted fingerprint 8b41c309
- Seed: encode_fclass_pbt.rs:encode_fclass_neg_extra
- Formal: ∀ mn ∈ {fmv.x.w, fmv.x.s, fmv.x.d}, rd ∈ GPRNames, rs1 ∈ FPNames, extra ∈ Operand. encode_fmv_x_f([Reg(rd), Reg(rs1), extra], funct7(mn), 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: failing
- Counterexample: encode_fmv_x_f([Reg("x0"), Reg("f0"), Imm(0)], 0b1110000, 0) → Ok(Word(0xe0000053))
- Bug report: pbt-out/bug_reports/encode_fmv_x_f_extra_operand.md

```property
function: encoder.encode_fmv_x_f
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, extra]
  domain: { mn: {fmv.x.w, fmv.x.s, fmv.x.d}, rd: gpr_names, rs1: fp_names, extra: Operand }
  body: encode_fmv_x_f([Reg(rd), Reg(rs1), extra], funct7(mn), 0) is Err
generators:
  mn: { gen: oneof, items: ["fmv.x.w", "fmv.x.s", "fmv.x.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  extra: { gen: oneof }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:308
```

## encode_fmv_x_f_neg_rm_third
- Tier: 3
- Rationale: Negative/error contract: FMV.X has no rounding-mode field (funct3 hardwired 000). llvm-mc rejects a 3rd rne token. Do not collapse if the SUT ignores it.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:162 "Float to integer register move" — asserted fingerprint b97c6a3e
- Seed: encode_fclass_pbt.rs:encode_fclass_neg_rm_third
- Formal: ∀ mn ∈ {fmv.x.w, fmv.x.s, fmv.x.d}, rd ∈ GPRNames, rs1 ∈ FPNames, rm ∈ {rne,rtz,rdn,rup,rmm,dyn}. encode_fmv_x_f([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), 0) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fmv_x_f_pbt.rs
- Status: failing
- Counterexample: encode_fmv_x_f([Reg("x0"), Reg("f0"), RoundingMode("rne")], 0b1110000, 0) → Ok(Word(0xe0000053))
- Bug report: pbt-out/bug_reports/encode_fmv_x_f_rm_third.md

```property
function: encoder.encode_fmv_x_f
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rm]
  domain: { mn: {fmv.x.w, fmv.x.s, fmv.x.d}, rd: gpr_names, rs1: fp_names, rm: {rne,rtz,rdn,rup,rmm,dyn} }
  body: encode_fmv_x_f([Reg(rd), Reg(rs1), RoundingMode(rm)], funct7(mn), 0) is Err
generators:
  mn: { gen: oneof, items: ["fmv.x.w", "fmv.x.s", "fmv.x.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rm: { gen: oneof, items: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
expected_error: String
evidence: RISC-V Unprivileged ISA FMV.X.W/D funct3 hardwired 000
```
