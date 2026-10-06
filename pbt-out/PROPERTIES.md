# Properties: encode_sc

## encode_sc_diff_llvm_mc
- Tier: 4
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent RISC-V assembler (A-extension). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SC decoder). encode_r / encode_sc_suffixed / encode_lr / encode_amo rejected as same-job siblings (private packer / aqrl-suffixed mnemonic / load-reserved / AMO with different funct5). Domain is llvm-mc-valid unsuffixed SC: sc.{w,d} with GPR rd, rs2 and mem (rs1) / 0(rs1).
- Doc contract: atomics.rs:17 "SC: 00011 | aq=0 | rl=0" — asserted fingerprint 724962ed
- Seed: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:encode_amo_diff_llvm_mc
- Formal: ∀ mn ∈ {sc.w, sc.d}, ∀ rd, rs2, rs1 ∈ GPRNames. encode_sc([Reg(rd), Reg(rs2), Mem{base: rs1, offset: 0}], funct3(mn)) = llvm-mc("-triple=riscv64 -mattr=+a", "mn rd, rs2, (rs1)")
- Test file: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sc
oracle: differential
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1]
  domain: { mn: sc_w_or_d, rd: GPRNames, rs2: GPRNames, rs1: GPRNames }
  body: encode_sc([Reg(rd), Reg(rs2), Mem(rs1, 0)], funct3(mn)) == llvm_mc("mn rd, rs2, (rs1)")
generators:
  mn: { gen: oneof, values: ["sc.w", "sc.d"] }
  rd: { gen: string }
  rs2: { gen: string }
  rs1: { gen: string }
evidence: README.md:306 assembler A-extension sc.w/d; encoder/mod.rs:659 unsuffixed sc.w/d dispatch; RISC-V Unprivileged ISA SC
```

## encode_sc_r_type_fields
- Tier: 3
- Rationale: Algebraic invariant from RISC-V R-type / A-extension SC layout (README.md:352, encoder/mod.rs:304, atomics.rs:17). Differential is stronger and is the sibling property; this pins opcode/funct3/rd/rs1/rs2/funct5/aq/rl independently of llvm-mc. Round-trip rejected (no decoder).
- Doc contract: atomics.rs:17 "SC: 00011 | aq=0 | rl=0" — asserted fingerprint 724962ed
- Seed: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:encode_amo_r_type_fields
- Formal: ∀ f3 ∈ {0b010, 0b011}, ∀ rd, rs2, rs1 ∈ 0..31. let w = encode_sc([Reg(xN(rd)), Reg(xN(rs2)), Mem{base: xN(rs1), offset: 0}], f3) in unpack(w) = (opcode=0b0101111, funct3=f3, rd, rs1, rs2, funct5=0b00011, aq=0, rl=0)
- Test file: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [f3, rd, rs2, rs1]
  domain: { f3: sc_funct3, rd: u32_0_31, rs2: u32_0_31, rs1: u32_0_31 }
  body: unpack(encode_sc([Reg(xN(rd)), Reg(xN(rs2)), Mem(xN(rs1), 0)], f3)) == (0b0101111, f3, rd, rs1, rs2, 0b00011, 0, 0)
generators:
  f3: { gen: oneof, values: [2, 3], type: u32 }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 0, max: 31, type: u32 }
  rs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:304 R-type layout; encoder/mod.rs:360 OP_AMO; atomics.rs:17 SC funct7; README.md:352 R-type
```

## encode_sc_abi_xn_alias
- Tier: 3
- Rationale: Algebraic metamorphic: ABI names, xN, and fp=s0/x8 are the same GPR (parser Operand::Reg comment). Differential already covers each spelling vs llvm-mc; this pins SUT alias equality without the external tool. Round-trip rejected (no decoder).
- Doc contract: atomics.rs:17 "SC: 00011 | aq=0 | rl=0" — asserted fingerprint 724962ed
- Seed: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:encode_amo_abi_xn_alias
- Formal: ∀ f3 ∈ {0b010, 0b011}, ∀ n, m, k ∈ 0..31. encode_sc([Reg(ABI(n)), Reg(ABI(m)), Mem{base: ABI(k), offset: 0}], f3) = encode_sc([Reg(xN(n)), Reg(xN(m)), Mem{base: xN(k), offset: 0}], f3) ∧ (n=8 ⇒ encode_sc([Reg("fp"), …], f3) equals the xN form) ∧ (m=8 ⇒ fp as rs2) ∧ (k=8 ⇒ fp as base)
- Test file: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [f3, n, m, k]
  domain: { f3: sc_funct3, n: u32_0_31, m: u32_0_31, k: u32_0_31 }
  body: encode_sc(ABI names) == encode_sc(xN names)
generators:
  f3: { gen: oneof, values: [2, 3], type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
  k: { gen: int, min: 0, max: 31, type: u32 }
evidence: parser.rs:22 Register ABI and xN names
```

## encode_sc_neg_extra
- Tier: 2
- Rationale: Negative/error contract from llvm-mc ("invalid operand for instruction" on a fourth operand). encode_instruction passes operands through; extra operands are not documented as valid for SC. Stronger differential on this domain is inapplicable (reference rejects the input).
- Doc contract: atomics.rs:17 "SC: 00011 | aq=0 | rl=0" — asserted fingerprint 724962ed — not an input-domain restriction on extra operands
- Seed: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:encode_amo_neg_extra
- Formal: ∀ mn ∈ {sc.w, sc.d}, ∀ rd, rs2, rs1 ∈ GPRNames, ∀ extra ∈ Operand. encode_sc([Reg(rd), Reg(rs2), Mem{base: rs1, offset: 0}, extra], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
- Status: failing
- Counterexample: encode_sc([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 0 }, Imm(0)], funct3=0b010) -> Ok(Word(0x1800202f))
- Bug report: pbt-out/bug_reports/encode_sc_extra_operand.md

```property
function: encoder.encode_sc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1, extra]
  domain: { mn: sc_w_or_d, rd: GPRNames, rs2: GPRNames, rs1: GPRNames, extra: Operand }
  body: encode_sc([Reg(rd), Reg(rs2), Mem(rs1, 0), extra], funct3(mn)).is_err()
generators:
  mn: { gen: oneof, values: ["sc.w", "sc.d"] }
  rd: { gen: string }
  rs2: { gen: string }
  rs1: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: llvm-mc extra operand error; encoder/mod.rs:659 unsuffixed sc.w/d dispatch
```

## encode_sc_neg_nonzero_offset
- Tier: 2
- Rationale: Negative/error contract from llvm-mc ("optional integer offset must be 0") and RISC-V A-extension SC address form. Bound 0 is sampled at ±1 and extremes. Stronger differential on this domain is inapplicable (reference rejects the input).
- Doc contract: atomics.rs:17 "SC: 00011 | aq=0 | rl=0" — asserted fingerprint 724962ed — not an offset-domain restriction; encode_sc binds `_offset` and ignores it
- Seed: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:encode_amo_neg_nonzero_offset
- Formal: ∀ mn ∈ {sc.w, sc.d}, ∀ rd, rs2, rs1 ∈ GPRNames, ∀ off ∈ ℤ\{0}. encode_sc([Reg(rd), Reg(rs2), Mem{base: rs1, offset: off}], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
- Status: failing
- Counterexample: encode_sc([Reg("x0"), Reg("x0"), Mem { base: "x0", offset: 1 }], funct3=0b010) -> Ok(Word(0x1800202f))
- Bug report: pbt-out/bug_reports/encode_sc_nonzero_offset.md

```property
function: encoder.encode_sc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, rs1, off]
  domain: { mn: sc_w_or_d, rd: GPRNames, rs2: GPRNames, rs1: GPRNames, off: nonzero_i64 }
  body: encode_sc([Reg(rd), Reg(rs2), Mem(rs1, off)], funct3(mn)).is_err()
generators:
  mn: { gen: oneof, values: ["sc.w", "sc.d"] }
  rd: { gen: string }
  rs2: { gen: string }
  rs1: { gen: string }
  off: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc optional integer offset must be 0; RISC-V Unprivileged ISA SC
```

## encode_sc_neg_arity_fp
- Tier: 2
- Rationale: Negative/error contract from llvm-mc ("too few operands" / "invalid operand" on FP regs) and get_reg/get_mem error paths. Empty, missing rs2/mem, missing mem, FP dest, FP rs2, FP base must Err.
- Doc contract: atomics.rs:17 "SC: 00011 | aq=0 | rl=0" — asserted fingerprint 724962ed
- Seed: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:encode_amo_neg_arity_fp
- Formal: ∀ f3 ∈ {0b010, 0b011}, ∀ rd, rs2, rs1 ∈ GPRNames, ∀ fp ∈ FPNames. encode_sc([], f3) is Err ∧ encode_sc([Reg(rd)], f3) is Err ∧ encode_sc([Reg(rd), Reg(rs2)], f3) is Err ∧ encode_sc([Reg(fp), Reg(rs2), Mem{base: rs1, offset: 0}], f3) is Err ∧ encode_sc([Reg(rd), Reg(fp), Mem{base: rs1, offset: 0}], f3) is Err ∧ encode_sc([Reg(rd), Reg(rs2), Mem{base: fp, offset: 0}], f3) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [f3, rd, rs2, rs1, fp]
  domain: { f3: sc_funct3, rd: GPRNames, rs2: GPRNames, rs1: GPRNames, fp: FPNames }
  body: encode_sc([], f3).is_err() and missing-operand and FP-GPR cases are Err
generators:
  f3: { gen: oneof, values: [2, 3], type: u32 }
  rd: { gen: string }
  rs2: { gen: string }
  rs1: { gen: string }
  fp: { gen: string }
expected_error: String
evidence: llvm-mc too-few / invalid FP operand; get_reg get_mem
```

## encode_sc_neg_non_mem
- Tier: 2
- Rationale: Negative/error contract from llvm-mc ("expected '(' or optional integer offset") when slot 2 is not a memory operand. get_mem returns Err for non-Mem.
- Doc contract: atomics.rs:17 "SC: 00011 | aq=0 | rl=0" — asserted fingerprint 724962ed
- Seed: src/backend/riscv/assembler/encoder/encode_amo_pbt.rs:encode_amo_neg_non_mem
- Formal: ∀ mn ∈ {sc.w, sc.d}, ∀ rd, rs2 ∈ GPRNames, ∀ bad ∈ Operand \ Mem. encode_sc([Reg(rd), Reg(rs2), bad], funct3(mn)) is Err
- Test file: src/backend/riscv/assembler/encoder/encode_sc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_sc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [mn, rd, rs2, bad]
  domain: { mn: sc_w_or_d, rd: GPRNames, rs2: GPRNames, bad: non_Mem_Operand }
  body: encode_sc([Reg(rd), Reg(rs2), bad], funct3(mn)).is_err()
generators:
  mn: { gen: oneof, values: ["sc.w", "sc.d"] }
  rd: { gen: string }
  rs2: { gen: string }
  bad: { gen: string }
expected_error: String
evidence: llvm-mc expected memory operand; get_mem encoder/mod.rs:437
```
