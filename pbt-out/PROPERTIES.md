# Properties: encode_c_add

## encode_c_add_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc assembling the same `c.add` mnemonic. State machine rejected: encode_c_add is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree C.ADD decoder. try_compress_rv64 rejected by same-job gate (post-encode compress of 32-bit ADD, not the `c.add` mnemonic). llvm-mc is an independent assembler; mapping is `[Reg(rd), Reg(rs2)]` <-> `c.add rd, rs2` with rs2 ≠ x0.
- Doc contract: compressed.rs:42 "c.add rd, rs2" — asserted fingerprint 1ff430ca
- Seed: (none) — no project-owned encode_c_add unit test; pattern generalized from encode_c_mv_pbt.rs llvm-mc differential
- Formal: ∀ rd ∈ GPR-names, rs2 ∈ GPR-names\{x0,zero}. encode_c_add([Reg(rd), Reg(rs2)]) = Half(llvm-mc("c.add rd, rs2") LE u16)
- Test file: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_add
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rs2]
  domain: { rd: gpr_name, rs2: gpr_nz_name }
  relation:
    op: eq
    lhs: sut_half([Reg(rd), Reg(rs2)])
    rhs: llvm_mc_half("c.add {rd}, {rs2}")
generators:
  rd: { gen: string }
  rs2: { gen: string }
evidence: "encoder/mod.rs:927 c.add => encode_c_add; llvm-mc -triple=riscv64 -mattr=+c"
```

## encode_c_add_cr_type_fields
- Tier: 4
- Rationale: RISC-V Unprivileged ISA CR-type layout for C.ADD is an independent structural invariant (not copied from the SUT body). Stronger differential already covers value agreement; this pins field placement so a swapped rd/rs2 still fails. Round-trip rejected (no decoder).
- Doc contract: compressed.rs:42 "c.add rd, rs2" — asserted fingerprint 1ff430ca
- Seed: (none)
- Formal: ∀ rd ∈ {0..31}, rs2 ∈ {1..31}. let h = encode_c_add([Reg(x{rd}), Reg(x{rs2})]).h in (h&0b11)=0b10 ∧ ((h>>12)&0b1111)=0b1001 ∧ ((h>>7)&0x1f)=rd ∧ ((h>>2)&0x1f)=rs2
- Test file: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_add
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rs2]
  domain: { rd: 0..31, rs2: 1..31 }
  body: "unpack_c_add(sut_half([Reg(x{rd}), Reg(x{rs2})])) == (op=0b10, funct4=0b1001, rd, rs2)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 1, max: 31, type: u32 }
evidence: "RISC-V Unprivileged ISA C.ADD CR-type; compress.rs:202 C.ADD add rd, rd, rs2"
```

## encode_c_add_abi_xn_alias
- Tier: 4
- Rationale: ABI names (zero/ra/sp/a0/…) and the xN spelling are the same GPR. Metamorphic: encoding is invariant under the ABI↔xN rename. Stronger differential already uses mixed names; this isolates the alias law. Round-trip rejected (no decoder).
- Doc contract: compressed.rs:42 "c.add rd, rs2" — asserted fingerprint 1ff430ca
- Seed: (none)
- Formal: ∀ n ∈ {0..31}, m ∈ {1..31}. encode_c_add([Reg(ABI[n]), Reg(ABI[m])]) = encode_c_add([Reg(x{n}), Reg(x{m})]); additionally n=8 ⇒ fp aliases x8 as rd, m=8 ⇒ fp aliases x8 as rs2
- Test file: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_add
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: 0..31, m: 1..31 }
  relation:
    op: eq
    lhs: sut_half([Reg(ABI[n]), Reg(ABI[m])])
    rhs: sut_half([Reg(x{n}), Reg(x{m})])
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 1, max: 31, type: u32 }
evidence: "encoder/mod.rs:419 get_reg via reg_num; RISC-V ABI GPR names"
```

## encode_c_add_field_isolation
- Tier: 4
- Rationale: CR-type fields are independent: changing rs2 must not alter bits[11:7] (rd); changing rd must not alter bits outside [11:7]. Metamorphic on a behavior-preserving field split. Stronger differential does not pin isolation.
- Doc contract: compressed.rs:42 "c.add rd, rs2" — asserted fingerprint 1ff430ca
- Seed: (none)
- Formal: ∀ rd ∈ {0..31}, rs2_a,rs2_b ∈ {1..31}. ((encode(rd,rs2_a)>>7)&0x1f) = ((encode(rd,rs2_b)>>7)&0x1f); ∀ rd_a,rd_b ∈ {0..31}, rs2 ∈ {1..31}. encode(rd_a,rs2) & ~0x0f80 = encode(rd_b,rs2) & ~0x0f80
- Test file: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_add
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rs2_a, rs2_b, rd_a, rd_b, rs2]
  domain: { rd: 0..31, rs2_a: 1..31, rs2_b: 1..31, rd_a: 0..31, rd_b: 0..31, rs2: 1..31 }
  body: "((h(rd,rs2_a)>>7)&0x1f)==((h(rd,rs2_b)>>7)&0x1f) && (h(rd_a,rs2)&~0x0f80)==(h(rd_b,rs2)&~0x0f80)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rs2_a: { gen: int, min: 1, max: 31, type: u32 }
  rs2_b: { gen: int, min: 1, max: 31, type: u32 }
  rd_a: { gen: int, min: 0, max: 31, type: u32 }
  rd_b: { gen: int, min: 0, max: 31, type: u32 }
  rs2: { gen: int, min: 1, max: 31, type: u32 }
evidence: "RISC-V Unprivileged ISA C.ADD CR-type field split"
```

## encode_c_add_neg_arity_fp
- Tier: 3
- Rationale: llvm-mc rejects too-few operands and FP registers as invalid operands for `c.add`. Negative/error contract: SUT must Err. Stronger oracles do not apply on the invalid domain.
- Doc contract: compressed.rs:42 "c.add rd, rs2" — asserted fingerprint 1ff430ca
- Seed: (none)
- Formal: ∀ ops. |ops|<2 ⇒ encode_c_add(ops)=Err; ∀ fp ∈ FP-names, gpr ∈ GPR-names\{x0}. encode_c_add([Reg(fp), Reg(gpr)])=Err ∧ encode_c_add([Reg(gpr), Reg(fp)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_add
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp, gpr]
  domain: { ops: short_ops, fp: fp_name, gpr: gpr_nz_name }
  body: "encode_c_add(ops).is_err() && encode_c_add([Reg(fp),Reg(gpr)]).is_err() && encode_c_add([Reg(gpr),Reg(fp)]).is_err()"
generators:
  ops: { gen: list, maxLen: 1 }
  fp: { gen: string }
  gpr: { gen: string }
expected_error: String
evidence: "llvm-mc too few operands / invalid operand; encoder/mod.rs:419 get_reg"
```

## encode_c_add_neg_extra
- Tier: 3
- Rationale: C.ADD is two-operand. llvm-mc rejects a third operand (`invalid operand for instruction`). Public dispatcher passes the operand slice through unchanged (mod.rs:927). SUT must Err on extra operands. Documented bound: exactly two operands.
- Doc contract: compressed.rs:42 "c.add rd, rs2" — asserted fingerprint 1ff430ca
- Seed: encode_c_mv_pbt.rs extra-operand negative (same CR-type arity contract)
- Formal: ∀ rd ∈ GPR-names, rs2 ∈ GPR-names\{x0}, extra ∈ Operand. encode_c_add([Reg(rd), Reg(rs2), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
- Status: failing
- Counterexample: encode_c_add([Reg("x0"), Reg("x1"), Imm(0)]) → Ok(Half(0x9006)); regression encode_c_add([Reg("x1"), Reg("x2"), Imm(0)]) → Ok(Half(0x908a))
- Bug report: bug_reports/encode_c_add_extra_operand.md

```property
function: encoder.encode_c_add
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rs2, extra]
  domain: { rd: gpr_name, rs2: gpr_nz_name, extra: extra_operand }
  body: "encode_c_add([Reg(rd), Reg(rs2), extra]).is_err()"
generators:
  rd: { gen: string }
  rs2: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: "llvm-mc rejects c.add x1, x2, x3; compressed.rs:42 two-operand form"
```

## encode_c_add_neg_rs2_x0
- Tier: 3
- Rationale: ISA: rs2=x0 with funct4=1001 is C.JALR (rd≠0) or C.EBREAK (rd=0), not C.ADD. llvm-mc rejects `c.add x1, x0` and `c.add x0, x0`. compress.rs:200 requires rs2 != 0 for C.ADD. SUT must Err. Domain includes rd=x0 (C.EBREAK collision) and rd≠x0 (C.JALR collision).
- Doc contract: compressed.rs:42 "c.add rd, rs2" — asserted fingerprint 1ff430ca
- Seed: encode_c_mv_pbt.rs rs2=x0 negative (CR-type rs2≠x0)
- Formal: ∀ rd ∈ GPR-names. llvm-mc rejects `c.add rd, x0` ∧ encode_c_add([Reg(rd), Reg(x0)])=Err ∧ encode_c_add([Reg(rd), Reg(zero)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_add_pbt.rs
- Status: failing
- Counterexample: encode_c_add([Reg("x0"), Reg("x0")]) → Ok(Half(0x9002)) C.EBREAK; regression encode_c_add([Reg("x1"), Reg("x0")]) → Ok(Half(0x9082)) C.JALR x1
- Bug report: bug_reports/encode_c_add_rs2_x0.md

```property
function: encoder.encode_c_add
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd]
  domain: { rd: gpr_name }
  body: "llvm_mc_rejects(c.add {rd}, x0) && encode_c_add([Reg(rd), Reg(x0)]).is_err() && encode_c_add([Reg(rd), Reg(zero)]).is_err()"
generators:
  rd: { gen: string }
expected_error: String
evidence: "llvm-mc invalid operand for c.add x1, x0; compress.rs:200 rs2 != 0; ISA C.JALR/C.EBREAK overlap"
```
