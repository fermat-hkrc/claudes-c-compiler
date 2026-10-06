# Properties: encode_fma

## encode_fma_diff_4op_llvm_mc
- Tier: 5
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent RISC-V assembler). State machine rejected: encode_fma is a pure function with no lifecycle. Algebraic round-trip via in-tree decoder rejected: no R4-type decoder. encode_r / encode_fp_arith as differential sibling rejected by same-job gate (R-type packer / 3-operand OP-FP, not R4 FMA). Reference ISA field layout is used as a weaker invariant property, not this one.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:189 "R4-type: rs3[31:27] | fmt[26:25] | rs2[24:20] | rs1[19:15] | rm[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint d1a3f01c
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_diff_3op_llvm_mc (same llvm-mc FP encode shape; FMA is 4-reg R4)
- Formal: ∀ mn ∈ {fmadd.s, fmsub.s, fnmsub.s, fnmadd.s, fmadd.d, fmsub.d, fnmsub.d, fnmadd.d}, rd, rs1, rs2, rs3 ∈ FPNames. encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3)], opcode(mn), fmt(mn)) = Word(llvm-mc(mn rd, rs1, rs2, rs3))
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fma
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rs3]
  domain: { mn: {fmadd.s, fmsub.s, fnmsub.s, fnmadd.s, fmadd.d, fmsub.d, fnmsub.d, fnmadd.d}, rd: fp_names, rs1: fp_names, rs2: fp_names, rs3: fp_names }
  relation:
    op: eq
    lhs: encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3)], opcode(mn), fmt(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rs2 + ", " + rs3)
generators:
  mn: { gen: oneof, items: ["fmadd.s", "fmsub.s", "fnmsub.s", "fnmadd.s", "fmadd.d", "fmsub.d", "fnmsub.d", "fnmadd.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  rs3: { gen: string }
evidence: src/backend/riscv/assembler/README.md:309; encoder/mod.rs:797
```

## encode_fma_diff_rm_llvm_mc
- Tier: 5
- Rationale: Same differential as the 4-op property, covering the optional rounding-mode operand. llvm-mc accepts rne/rtz/rdn/rup/rmm/dyn as a 5th token. Stronger state machine / round-trip rejected as above.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:492 "Parse a rounding mode to 3-bit encoding" — asserted fingerprint c9473eee
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_diff_rm_llvm_mc
- Formal: ∀ mn ∈ FmaMn, rd, rs1, rs2, rs3 ∈ FPNames, rm ∈ {rne, rtz, rdn, rup, rmm, dyn}. encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), RoundingMode(rm)], opcode(mn), fmt(mn)) = Word(llvm-mc(mn rd, rs1, rs2, rs3, rm))
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fma
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rs3, rm]
  domain: { mn: fma_mnemonics, rd: fp_names, rs1: fp_names, rs2: fp_names, rs3: fp_names, rm: {rne, rtz, rdn, rup, rmm, dyn} }
  relation:
    op: eq
    lhs: encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), RoundingMode(rm)], opcode(mn), fmt(mn))
    rhs: llvm_mc(mn + " " + rd + ", " + rs1 + ", " + rs2 + ", " + rs3 + ", " + rm)
generators:
  mn: { gen: oneof, items: ["fmadd.s", "fmsub.s", "fnmsub.s", "fnmadd.s", "fmadd.d", "fmsub.d", "fnmsub.d", "fnmadd.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  rs3: { gen: string }
  rm: { gen: oneof, items: ["rne", "rtz", "rdn", "rup", "rmm", "dyn"] }
evidence: src/backend/riscv/assembler/parser.rs:41; encoder/mod.rs:492
```

## encode_fma_r4_type_fields
- Tier: 4
- Rationale: Algebraic invariant from the documented R4-type layout (rs3 | fmt | rs2 | rs1 | rm | rd | opcode). Stronger differential is a sibling property. Round-trip rejected (no decoder).
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:189 "R4-type: rs3[31:27] | fmt[26:25] | rs2[24:20] | rs1[19:15] | rm[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint d1a3f01c
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_r_type_fields
- Formal: ∀ rd, rs1, rs2, rs3 ∈ 0..31, opc ∈ {0b1000011, 0b1000111, 0b1001011, 0b1001111}, fmt ∈ {0b00, 0b01}, rm ∈ {0,1,2,3,4,7}. unpack_r4(encode_fma([Reg(f{rd}), Reg(f{rs1}), Reg(f{rs2}), Reg(f{rs3}), RoundingMode(rm_name)], opc, fmt)) = (opc, rd, rm, rs1, rs2, fmt, rs3)
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fma
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, rs3, opc, fmt, rm]
  domain: { rd: 0..31, rs1: 0..31, rs2: 0..31, rs3: 0..31, opc: {0b1000011, 0b1000111, 0b1001011, 0b1001111}, fmt: {0, 1}, rm: {0, 1, 2, 3, 4, 7} }
  body: unpack_r4(word) == (opc, rd, rm, rs1, rs2, fmt, rs3)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rs3: { gen: int, min: 0, max: 31, type: u32 }
  opc: { gen: oneof, items: [67, 71, 75, 79] }
  fmt: { gen: int, min: 0, max: 1, type: u32 }
  rm: { gen: oneof, items: [0, 1, 2, 3, 4, 7] }
evidence: src/backend/riscv/assembler/encoder/float.rs:189
```

## encode_fma_abi_fn_alias
- Tier: 4
- Rationale: Metamorphic: ABI names (ft0/fa0/fs0/...) and fN names are aliases of the same 5-bit FP register index (freg_num). Stronger differential is a sibling property over mixed names.
- Doc contract: src/backend/riscv/assembler/encoder/float.rs:189 "R4-type: rs3[31:27] | fmt[26:25] | rs2[24:20] | rs1[19:15] | rm[14:12] | rd[11:7] | opcode[6:0]" — asserted fingerprint d1a3f01c
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_abi_fn_alias
- Formal: ∀ n, m, p, q ∈ 0..31, opc ∈ FmaOpcodes, fmt ∈ {0,1}. encode_fma([Reg(f{n}), Reg(f{m}), Reg(f{p}), Reg(f{q})], opc, fmt) = encode_fma([Reg(FABI[n]), Reg(FABI[m]), Reg(FABI[p]), Reg(FABI[q])], opc, fmt)
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fma
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m, p, q, opc, fmt]
  domain: { n: 0..31, m: 0..31, p: 0..31, q: 0..31, opc: fma_opcodes, fmt: {0, 1} }
  relation:
    op: eq
    lhs: encode_fma([Reg(fn(n)), Reg(fn(m)), Reg(fn(p)), Reg(fn(q))], opc, fmt)
    rhs: encode_fma([Reg(fabi(n)), Reg(fabi(m)), Reg(fabi(p)), Reg(fabi(q))], opc, fmt)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  p: { gen: int, min: 0, max: 31, type: u32 }
  q: { gen: int, min: 0, max: 31, type: u32 }
  opc: { gen: oneof, items: [67, 71, 75, 79] }
  fmt: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/riscv/assembler/encoder/mod.rs:257
```

## encode_fma_rm_default_dyn
- Tier: 4
- Rationale: Metamorphic: omitted 5th operand equals explicit RoundingMode("dyn") and unpacks rm=111 (ISA default DYN). Stronger differential covers both forms independently.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:492 "Parse a rounding mode to 3-bit encoding" — asserted fingerprint c9473eee
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_rm_default_dyn
- Formal: ∀ rd, rs1, rs2, rs3 ∈ FPNames, opc ∈ FmaOpcodes, fmt ∈ {0,1}. encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3)], opc, fmt) = encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), RoundingMode("dyn")], opc, fmt) ∧ unpack_r4(...).rm = 0b111
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fma
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs1, rs2, rs3, opc, fmt]
  domain: { rd: fp_names, rs1: fp_names, rs2: fp_names, rs3: fp_names, opc: fma_opcodes, fmt: {0, 1} }
  relation:
    op: eq
    lhs: encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3)], opc, fmt)
    rhs: encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), RoundingMode("dyn")], opc, fmt)
generators:
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  rs3: { gen: string }
  opc: { gen: oneof, items: [67, 71, 75, 79] }
  fmt: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/riscv/assembler/encoder/float.rs:180; encoder/mod.rs:492
```

## encode_fma_neg_arity_gpr
- Tier: 4
- Rationale: Negative/error contract: llvm-mc rejects too-few operands and GPR in an FP slot. get_freg returns Err for missing/non-FP operands. Stronger differential does not apply on the invalid domain.
- Doc contract: src/backend/riscv/assembler/encoder/mod.rs:425 "expected float register at operand {}, got {:?}" — asserted fingerprint 9575ce77
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_neg_arity_gpr
- Formal: ∀ opc ∈ FmaOpcodes, fmt ∈ {0,1}, fp ∈ FPNames, gpr ∈ GPRNames, bad ∈ NonReg. encode_fma([], opc, fmt) is Err ∧ encode_fma([fp], opc, fmt) is Err ∧ encode_fma([fp,fp], opc, fmt) is Err ∧ encode_fma([fp,fp,fp], opc, fmt) is Err ∧ encode_fma with GPR or non-Reg in any of the four slots is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_fma
oracle: negative_error
predicate:
  quantifier: forall
  vars: [opc, fmt, fp, gpr, bad]
  domain: { opc: fma_opcodes, fmt: {0, 1}, fp: fp_names, gpr: gpr_names, bad: non_reg_operands }
  relation:
    op: throws
    lhs: encode_fma(too_few_or_gpr_or_nonreg, opc, fmt)
    rhs: String
generators:
  opc: { gen: oneof, items: [67, 71, 75, 79] }
  fmt: { gen: int, min: 0, max: 1, type: u32 }
  fp: { gen: string }
  gpr: { gen: string }
  bad: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/encoder/mod.rs:420; encoder/mod.rs:425
```

## encode_fma_neg_extra
- Tier: 4
- Rationale: Negative/error contract: llvm-mc rejects a 6th operand after rd, rs1, rs2, rs3, rm. The encoder must Err rather than silently ignore extras. Stronger differential does not apply on the invalid domain.
- Doc contract: src/backend/riscv/assembler/README.md:309 "fmadd/fmsub/fnmadd/fnmsub" — asserted fingerprint f4814537
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_neg_extra
- Formal: ∀ mn ∈ FmaMn, rd, rs1, rs2, rs3 ∈ FPNames, extra ∈ Operand. encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), RoundingMode("rne"), extra], opcode(mn), fmt(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: failing
- Counterexample: encode_fma([Reg("f0"), Reg("f0"), Reg("f0"), Reg("f0"), RoundingMode("rne"), Imm(0)], 0b1000011, 0) -> Ok(Word(67))
- Bug report: pbt-out/bug_reports/encode_fma_extra_operand.md

```property
function: encoder.encode_fma
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rs3, extra]
  domain: { mn: fma_mnemonics, rd: fp_names, rs1: fp_names, rs2: fp_names, rs3: fp_names, extra: Operand }
  relation:
    op: throws
    lhs: encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), RoundingMode("rne"), extra], opcode(mn), fmt(mn))
    rhs: String
generators:
  mn: { gen: oneof, items: ["fmadd.s", "fmsub.s", "fnmsub.s", "fnmadd.s", "fmadd.d", "fmsub.d", "fnmsub.d", "fnmadd.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  rs3: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/README.md:309
```

## encode_fma_neg_non_rm_fifth
- Tier: 4
- Rationale: Negative/error contract: llvm-mc requires the optional 5th operand to be a rounding-mode mnemonic. A non-RoundingMode 5th token must Err. Stronger differential does not apply on the invalid domain.
- Doc contract: src/backend/riscv/assembler/parser.rs:41 "Rounding mode: rne, rtz, rdn, rup, rmm, dyn" — asserted fingerprint 4d950ca5
- Seed: encode_fp_arith_pbt.rs:encode_fp_arith_neg_non_rm_fourth
- Formal: ∀ mn ∈ FmaMn, rd, rs1, rs2, rs3 ∈ FPNames, extra ∈ NonRoundingModeOperand. encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), extra], opcode(mn), fmt(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_fma_pbt.rs
- Status: failing
- Counterexample: encode_fma([Reg("f0"), Reg("f0"), Reg("f0"), Reg("f0"), Imm(0)], 0b1000011, 0) -> Ok(Word(28739))
- Bug report: pbt-out/bug_reports/encode_fma_non_rm_fifth.md

```property
function: encoder.encode_fma
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs1, rs2, rs3, extra]
  domain: { mn: fma_mnemonics, rd: fp_names, rs1: fp_names, rs2: fp_names, rs3: fp_names, extra: non_rm_operands }
  relation:
    op: throws
    lhs: encode_fma([Reg(rd), Reg(rs1), Reg(rs2), Reg(rs3), extra], opcode(mn), fmt(mn))
    rhs: String
generators:
  mn: { gen: oneof, items: ["fmadd.s", "fmsub.s", "fnmsub.s", "fnmadd.s", "fmadd.d", "fmsub.d", "fnmsub.d", "fnmadd.d"] }
  rd: { gen: string }
  rs1: { gen: string }
  rs2: { gen: string }
  rs3: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: src/backend/riscv/assembler/parser.rs:41
```
