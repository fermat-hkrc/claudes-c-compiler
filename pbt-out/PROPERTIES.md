# Properties: encode_c_mv

## encode_c_mv_diff_llvm_mc
- Tier: 5
- Rationale: Strongest oracle is differential against llvm-mc (LLVM 15.0.6), the independent RISC-V assembler this tree already uses as the encoding reference. State machine rejected: encode_c_mv is a pure function with no lifecycle. Algebraic round-trip rejected: no C.MV decoder in-tree. try_compress_rv64 / 32-bit ADD rejected by the same-job gate (post-encode compress pass, not the `c.mv` mnemonic encoder). Domain is llvm-mc's accepted C.MV set: rd ∈ {x0..x31}, rs2 ∈ {x1..x31} (rd=x0 is HINT, accepted by llvm-mc; rs2=x0 is C.JR and is rejected).
- Doc contract: compressed.rs:35 "c.mv rd, rs2" — asserted fingerprint 2f247e2c
- Seed: compress.rs:764-769 C.MV t1, t0 = 0x8316; KAT vectors taken from llvm-mc
- Formal: ∀ rd ∈ GPR, ∀ rs2 ∈ GPR\{x0}. encode_c_mv([Reg(rd), Reg(rs2)]) = Half(llvm-mc("c.mv rd, rs2", -triple=riscv64 -mattr=+c))
- Test file: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_mv
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs2]
  domain: { rd: gpr_0_31, rs2: gpr_1_31 }
  relation:
    op: eq
    lhs: sut_half([Reg(rd), Reg(rs2)])
    rhs: llvm_mc_half("c.mv {rd}, {rs2}")
generators:
  rd: { gen: string, type: String }
  rs2: { gen: string, type: String }
evidence: encoder/mod.rs:924 "c.mv" => encode_c_mv; README.md:13 C (compressed 16-bit); llvm-mc 15.0.6
```

## encode_c_mv_cr_type_fields
- Tier: 4
- Rationale: RISC-V Unprivileged ISA CR-type layout for C.MV is an exact structural invariant independent of llvm-mc. Weaker than the differential but pins opcode/funct4/rd/rs2 bit placement so a packing off-by-one cannot hide behind a matching reference if the mapping were wrong. Round-trip rejected (no decoder).
- Doc contract: compressed.rs:35 "c.mv rd, rs2" — asserted fingerprint 2f247e2c
- Seed: compress.rs:207-210 100_0 | rd | rs2 | 10
- Formal: ∀ rd ∈ {0..31}, ∀ rs2 ∈ {1..31}. let h = encode_c_mv([Reg(xN), Reg(xM)]).as_half(). (h & 0b11) = 0b10 ∧ ((h >> 12) & 0b1111) = 0b1000 ∧ ((h >> 7) & 0x1F) = rd ∧ ((h >> 2) & 0x1F) = rs2
- Test file: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_mv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs2]
  domain: { rd: gpr_0_31, rs2: gpr_1_31 }
  relation:
    op: holds
    expr: cr_mv_fields(sut_half([Reg(xN), Reg(xM)])) == (op=10, funct4=1000, rd, rs2)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 1, max: 31, type: u32 }
evidence: compress.rs:207 C.MV CR-type; RISC-V ISA C.MV CR-type
```

## encode_c_mv_abi_xn_alias
- Tier: 4
- Rationale: ABI names (zero, ra, sp, a0, t6, fp/s0, …) and xN must encode the same halfword. Metamorphic under a behavior-preserving rename. Stronger differential already covers xN vs llvm-mc; this pins the alias table. rs2≠x0 so the encoding stays C.MV (not C.JR).
- Doc contract: compressed.rs:35 "c.mv rd, rs2" — asserted fingerprint 2f247e2c
- Seed: encode_c_addi_pbt.rs encode_c_addi_abi_xn_alias
- Formal: ∀ n ∈ {0..31}, ∀ m ∈ {1..31}. encode_c_mv([Reg("x"+n), Reg("x"+m)]) = encode_c_mv([Reg(ABI[n]), Reg(ABI[m])]) ∧ (n=8 ⇒ encode_c_mv([Reg("fp"), Reg("x"+m)]) = that halfword) ∧ (m=8 ⇒ encode_c_mv([Reg("x"+n), Reg("fp")]) = that halfword)
- Test file: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_mv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: gpr_0_31, m: gpr_1_31 }
  relation:
    op: eq
    lhs: sut_half([Reg(xN), Reg(xM)])
    rhs: sut_half([Reg(ABI[n]), Reg(ABI[m])])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 1, max: 31, type: u32 }
evidence: encoder/mod.rs:210-256 reg_num ABI/xN/fp
```

## encode_c_mv_field_isolation
- Tier: 4
- Rationale: Changing only rs2 must leave rd bits unchanged and vice versa (field isolation). Metamorphic: encode(rd, rs2_a) and encode(rd, rs2_b) share bits[11:7]; encode(rd_a, rs2) and encode(rd_b, rs2) share rs2/op/funct4 bits. Independent of llvm-mc.
- Doc contract: compressed.rs:35 "c.mv rd, rs2" — asserted fingerprint 2f247e2c
- Seed: encode_c_addi_pbt.rs encode_c_addi_imm_isolation
- Formal: ∀ rd ∈ {0..31}, ∀ rs2_a, rs2_b ∈ {1..31}. ((encode(rd,rs2_a) >> 7) & 0x1F) = ((encode(rd,rs2_b) >> 7) & 0x1F) ∧ ∀ rd_a, rd_b ∈ {0..31}, ∀ rs2 ∈ {1..31}. encode(rd_a,rs2) & ~0x0F80 = encode(rd_b,rs2) & ~0x0F80
- Test file: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_mv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs2_a, rs2_b, rd_a, rd_b, rs2]
  domain: { rd: gpr_0_31, rs2_a: gpr_1_31, rs2_b: gpr_1_31, rd_a: gpr_0_31, rd_b: gpr_0_31, rs2: gpr_1_31 }
  relation:
    op: holds
    expr: rd_field(encode(rd,rs2_a)) == rd_field(encode(rd,rs2_b)) && (encode(rd_a,rs2) & ~0x0F80) == (encode(rd_b,rs2) & ~0x0F80)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs2_a: { gen: int, min: 1, max: 31, type: u32 }
  rs2_b: { gen: int, min: 1, max: 31, type: u32 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 1, max: 31, type: u32 }
evidence: RISC-V ISA C.MV CR-type field layout
```

## encode_c_mv_neg_rs2_x0
- Tier: 3
- Rationale: llvm-mc rejects `c.mv rd, x0` (`invalid operand for instruction`) because rs2=x0 is the C.JR encoding, not C.MV. compress.rs:205 requires rs2 != 0 for C.MV. Negative-error contract: encode_c_mv must Err on rs2=x0 rather than silently emit C.JR. Documented bound sampled at rs2=x0 / zero / Imm(0).
- Doc contract: compressed.rs:35 "c.mv rd, rs2" — asserted fingerprint 2f247e2c
- Seed: compress.rs:205 rs2 != 0; llvm-mc error on c.mv x1, x0
- Formal: ∀ rd ∈ GPR. llvm-mc("c.mv rd, x0") errors ∧ encode_c_mv([Reg(rd), Reg("x0")]) = Err(_) ∧ encode_c_mv([Reg(rd), Reg("zero")]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
- Status: failing
- Counterexample: encode_c_mv([Reg("x0"), Reg("x0")])
- Bug report: bug_reports/encode_c_mv_rs2_x0.md

```property
function: encoder.encode_c_mv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd]
  domain: { rd: gpr_name }
  relation:
    op: throws
    expr: encode_c_mv([Reg(rd), Reg("x0")])
    error: String
generators:
  rd: { gen: string, type: String }
expected_error: String
evidence: llvm-mc "invalid operand for instruction" on c.mv x1, x0; compress.rs:205 rs2 != 0; RISC-V ISA C.MV rs2≠x0
```

## encode_c_mv_neg_extra
- Tier: 3
- Rationale: C.MV is two-operand. llvm-mc rejects a third operand (`c.mv x1, x2, x3` → invalid operand). Negative-error: encode_c_mv must Err rather than silently ignore extras (get_reg only reads indices 0 and 1).
- Doc contract: compressed.rs:35 "c.mv rd, rs2" — asserted fingerprint 2f247e2c
- Seed: encode_c_addi_pbt.rs encode_c_addi_neg_extra
- Formal: ∀ rd ∈ GPR, ∀ rs2 ∈ GPR\{x0}, ∀ extra. encode_c_mv([Reg(rd), Reg(rs2), extra]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
- Status: failing
- Counterexample: encode_c_mv([Reg("x0"), Reg("x1"), Imm(0)])
- Bug report: bug_reports/encode_c_mv_extra_operand.md

```property
function: encoder.encode_c_mv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs2, extra]
  domain: { rd: gpr_name, rs2: gpr_1_31_name, extra: Operand }
  relation:
    op: throws
    expr: encode_c_mv([Reg(rd), Reg(rs2), extra])
    error: String
generators:
  rd: { gen: string, type: String }
  rs2: { gen: string, type: String }
  extra: { gen: string, type: Operand }
expected_error: String
evidence: llvm-mc rejects extra operands for c.mv; compressed.rs:35 two-operand form
```

## encode_c_mv_neg_arity_fp
- Tier: 3
- Rationale: llvm-mc rejects too few operands and FP dest/src (`c.mv fa0, a1` / `c.mv a0, fa1` → invalid operand; `c.mv x1` / `c.mv` → too few operands). Negative-error: empty/1-operand and FP register dest or src must Err.
- Doc contract: compressed.rs:35 "c.mv rd, rs2" — asserted fingerprint 2f247e2c
- Seed: encode_c_addi_pbt.rs encode_c_addi_neg_arity_fp
- Formal: ∀ ops. |ops| < 2 ⇒ encode_c_mv(ops) = Err(_) ∧ ∀ fp ∈ FPR, ∀ gpr ∈ GPR\{x0}. encode_c_mv([Reg(fp), Reg(gpr)]) = Err(_) ∧ encode_c_mv([Reg(gpr), Reg(fp)]) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_c_mv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_mv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp, gpr]
  domain: { ops: arity_lt_2, fp: fpr_name, gpr: gpr_1_31_name }
  relation:
    op: throws
    expr: encode_c_mv(ops) and encode_c_mv([Reg(fp), Reg(gpr)]) and encode_c_mv([Reg(gpr), Reg(fp)])
    error: String
generators:
  ops: { gen: list, maxLen: 1 }
  fp: { gen: string, type: String }
  gpr: { gen: string, type: String }
expected_error: String
evidence: llvm-mc "too few operands" / "invalid operand"; get_reg rejects FP names
```
