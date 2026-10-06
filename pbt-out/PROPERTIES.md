# Properties: encode_c_li

## encode_c_li_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential against llvm-mc (LLVM 15.0.6), the independent RISC-V assembler this tree already uses as the encoding reference. State machine rejected: encode_c_li is a pure function with no lifecycle. Algebraic round-trip rejected: no C.LI decoder in-tree. try_compress_rv64 / 32-bit ADDI rejected by the same-job gate (post-encode compress pass, not the `c.li` mnemonic encoder). Domain is llvm-mc's accepted C.LI immediate set [-32, 31] with rd ∈ {x0..x31} (rd=x0 is HINT, accepted by llvm-mc).
- Doc contract: compressed.rs:17 "c.li rd, imm" — asserted fingerprint f85aaea5
- Seed: compress.rs:832 C.LI a0, 5 = 0x4515; KAT vectors taken from llvm-mc
- Formal: ∀ rd ∈ GPR, ∀ imm ∈ [-32,31]. encode_c_li([Reg(rd), Imm(imm)]) = Half(llvm-mc("c.li rd, imm", -triple=riscv64 -mattr=+c))
- Test file: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_li
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_0_31, imm: simm6 }
  relation:
    op: eq
    lhs: sut_half([Reg(rd), Imm(imm)])
    rhs: llvm_mc_half("c.li {rd}, {imm}")
generators:
  rd: { gen: string, type: String }
  imm: { gen: int, min: -32, max: 31, type: i64 }
evidence: encoder/mod.rs:918 "c.li" => encode_c_li; README.md:13 C (compressed 16-bit); llvm-mc 15.0.6
```

## encode_c_li_ci_type_fields
- Tier: 4
- Rationale: RISC-V Unprivileged ISA CI-type layout for C.LI is an exact structural invariant independent of llvm-mc. Weaker than the differential but pins opcode/funct3/rd/imm bit placement so a packing off-by-one cannot hide behind a matching reference if the mapping were wrong. Round-trip rejected (no decoder).
- Doc contract: compressed.rs:17 "c.li rd, imm" — asserted fingerprint f85aaea5
- Seed: compress.rs:832 010 | 0 | 01010 | 00101 | 01
- Formal: ∀ rd ∈ {0..31}, ∀ imm ∈ [-32,31]. let h = encode_c_li([Reg(xN), Imm(imm)]).as_half(). (h & 0b11) = 0b01 ∧ ((h >> 13) & 0b111) = 0b010 ∧ ((h >> 7) & 0x1F) = rd ∧ sign_extend6(((h >> 12) & 1) << 5 | ((h >> 2) & 0x1F)) = imm
- Test file: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_li
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_0_31, imm: simm6 }
  relation:
    op: holds
    expr: ci_li_fields(sut_half([Reg(xN), Imm(imm)])) == (op=01, funct3=010, rd, imm6)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -32, max: 31, type: i32 }
evidence: compress.rs:99 signed 6-bit range; RISC-V ISA C.LI CI-type
```

## encode_c_li_abi_xn_alias
- Tier: 4
- Rationale: ABI names (zero, ra, sp, a0, t6, fp/s0, …) and xN must encode the same halfword. Metamorphic under a behavior-preserving rename. Stronger differential already covers xN vs llvm-mc; this pins the alias table.
- Doc contract: compressed.rs:17 "c.li rd, imm" — asserted fingerprint f85aaea5
- Seed: encode_c_lui_pbt.rs encode_c_lui_abi_xn_alias
- Formal: ∀ n ∈ {0..31}, ∀ imm ∈ [-32,31]. encode_c_li([Reg("x"+n), Imm(imm)]) = encode_c_li([Reg(ABI[n]), Imm(imm)]) ∧ (n=8 ⇒ encode_c_li([Reg("fp"), Imm(imm)]) = that halfword)
- Test file: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_li
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, imm]
  domain: { n: gpr_0_31, imm: simm6 }
  relation:
    op: eq
    lhs: sut_half([Reg(xN), Imm(imm)])
    rhs: sut_half([Reg(ABI[n]), Imm(imm)])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -32, max: 31, type: i64 }
evidence: encoder/mod.rs:210-256 reg_num ABI/xN/fp
```

## encode_c_li_imm_isolation
- Tier: 4
- Rationale: Changing only the immediate must leave rd bits unchanged and vice versa (field isolation). Metamorphic: encode(rd, imm_a) and encode(rd, imm_b) share bits[11:7]; encode(rd_a, imm) and encode(rd_b, imm) share the imm packing. Independent of llvm-mc.
- Doc contract: compressed.rs:17 "c.li rd, imm" — asserted fingerprint f85aaea5
- Seed: (none)
- Formal: ∀ rd ∈ {0..31}, ∀ imm_a, imm_b ∈ [-32,31]. ((encode(rd,imm_a) >> 7) & 0x1F) = ((encode(rd,imm_b) >> 7) & 0x1F) ∧ ∀ rd_a, rd_b, ∀ imm ∈ [-32,31]. encode(rd_a,imm) & ~0x0F80 = encode(rd_b,imm) & ~0x0F80
- Test file: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_li
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, imm_a, imm_b, rd_a, rd_b, imm]
  domain: { rd: gpr_0_31, imm_a: simm6, imm_b: simm6, rd_a: gpr_0_31, rd_b: gpr_0_31, imm: simm6 }
  relation:
    op: holds
    expr: rd_field(encode(rd,imm_a)) == rd_field(encode(rd,imm_b)) && (encode(rd_a,imm) & !0x0F80) == (encode(rd_b,imm) & !0x0F80)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm_a: { gen: int, min: -32, max: 31, type: i64 }
  imm_b: { gen: int, min: -32, max: 31, type: i64 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: -32, max: 31, type: i64 }
evidence: RISC-V ISA C.LI CI-type rd in bits[11:7], imm in bits[12]+[6:2]
```

## encode_c_li_neg_imm_oob
- Tier: 3
- Rationale: llvm-mc rejects immediates outside [-32, 31] with "immediate must be an integer in the range [-32, 31]". compress.rs:99 states signed 6-bit. Documented bound must be sampled at bound±1 (32, -33). SUT must Err rather than truncate.
- Doc contract: compressed.rs:17 "c.li rd, imm" — asserted fingerprint f85aaea5
- Seed: encode_c_lui_pbt.rs encode_c_lui_neg_imm_oob
- Formal: ∀ rd ∈ GPR, ∀ imm ∉ [-32,31]. llvm-mc rejects "c.li rd, imm" ⇒ encode_c_li([Reg(rd), Imm(imm)]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
- Status: failing
- Counterexample: encode_c_li([Reg("x0"), Imm(32)]) -> Ok(Half(0x5001))
- Bug report: bug_reports/encode_c_li_imm_oob.md

```property
function: encoder.encode_c_li
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm]
  domain: { rd: gpr_0_31, imm: not_in_simm6 }
  relation:
    op: throws
    expr: encode_c_li([Reg(rd), Imm(imm)])
generators:
  rd: { gen: string, type: String }
  imm: { gen: int, type: i64 }
expected_error: String
evidence: llvm-mc range [-32, 31]; compress.rs:99
```

## encode_c_li_neg_extra
- Tier: 3
- Rationale: C.LI is two-operand. llvm-mc rejects a third operand ("invalid operand for instruction"). encode_c_li currently reads only indices 0 and 1, so extra operands are a documented-by-reference error path.
- Doc contract: compressed.rs:17 "c.li rd, imm" — asserted fingerprint f85aaea5
- Seed: encode_c_lui_pbt.rs encode_c_lui_neg_extra
- Formal: ∀ rd ∈ GPR, ∀ imm ∈ [-32,31], ∀ extra. encode_c_li([Reg(rd), Imm(imm), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
- Status: failing
- Counterexample: encode_c_li([Reg("x0"), Imm(0), Imm(0)]) -> Ok(Half(0x4001))
- Bug report: bug_reports/encode_c_li_extra_operand.md

```property
function: encoder.encode_c_li
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm, extra]
  domain: { rd: gpr_0_31, imm: simm6, extra: Operand }
  relation:
    op: throws
    expr: encode_c_li([Reg(rd), Imm(imm), extra])
generators:
  rd: { gen: string, type: String }
  imm: { gen: int, min: -32, max: 31, type: i64 }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: compressed.rs:17 two-operand form; llvm-mc invalid operand for extra
```

## encode_c_li_neg_arity_fp
- Tier: 3
- Rationale: llvm-mc rejects too-few operands and FP dest (`c.li f0, 1` invalid operand). Empty/1-operand and FP register names must Err.
- Doc contract: compressed.rs:17 "c.li rd, imm" — asserted fingerprint f85aaea5
- Seed: encode_c_lui_pbt.rs encode_c_lui_neg_arity_fp
- Formal: ∀ ops with |ops| < 2. encode_c_li(ops) = Err ∧ ∀ fp ∈ FPR, ∀ imm ∈ [-32,31]. encode_c_li([Reg(fp), Imm(imm)]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_li_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_li
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp, imm]
  domain: { ops: arity_lt_2, fp: fpr_name, imm: simm6 }
  relation:
    op: throws
    expr: encode_c_li(ops)
generators:
  ops: { gen: list, type: Vec<Operand> }
  fp: { gen: string, type: String }
  imm: { gen: int, min: -32, max: 31, type: i64 }
expected_error: String
evidence: llvm-mc too few operands / invalid operand for FP dest
```
