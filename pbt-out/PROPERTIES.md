# Properties: encode_c_jalr

## encode_c_jalr_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc assembling the same `c.jalr` mnemonic. State machine rejected: encode_c_jalr is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree C.JALR decoder. try_compress_rv64 rejected by same-job gate (post-encode compress of 32-bit JALR, not the `c.jalr` mnemonic). llvm-mc is an independent assembler; mapping is `[Reg(rs1)]` <-> `c.jalr rs1` with rs1 ≠ x0.
- Doc contract: compressed.rs:55 "c.jalr rs1" — asserted fingerprint 45541b91
- Seed: compress.rs:572 C.JALR packing comment; pattern generalized from encode_c_jr_pbt.rs llvm-mc differential
- Formal: ∀ rs1 ∈ GPR-names\{x0,zero}. encode_c_jalr([Reg(rs1)]) = Half(llvm-mc("c.jalr rs1") LE u16)
- Test file: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jalr
oracle: differential
predicate:
  quantifier: forall
  vars: [rs1]
  domain: { rs1: gpr_nz_name }
  relation:
    op: eq
    lhs: sut_half([Reg(rs1)])
    rhs: llvm_mc_half("c.jalr {rs1}")
generators:
  rs1: { gen: string }
evidence: "encoder/mod.rs:933 c.jalr => encode_c_jalr; llvm-mc -triple=riscv64 -mattr=+c"
```

## encode_c_jalr_cr_type_fields
- Tier: 4
- Rationale: RISC-V Unprivileged ISA CR-type layout for C.JALR is an independent structural invariant (not copied from the SUT body). Stronger differential already covers value agreement; this pins field placement so a zero bit12 (C.JR) or non-zero rs2 still fails. Round-trip rejected (no decoder).
- Doc contract: compressed.rs:55 "c.jalr rs1" — asserted fingerprint 45541b91
- Seed: compress.rs:572 C.JALR packing comment
- Formal: ∀ rs1 ∈ {1..31}. let h = encode_c_jalr([Reg(x{rs1})]).h in (h&0b11)=0b10 ∧ ((h>>12)&0b1111)=0b1001 ∧ ((h>>7)&0x1f)=rs1 ∧ ((h>>2)&0x1f)=0
- Test file: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jalr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs1]
  domain: { rs1: 1..31 }
  body: "unpack_c_jalr(sut_half([Reg(x{rs1})])) == (op=0b10, funct4=0b1001, rs1, rs2=0)"
generators:
  rs1: { gen: int, min: 1, max: 31, type: u32 }
evidence: "RISC-V Unprivileged ISA C.JALR CR-type; compress.rs:572 C.JALR jalr x1, 0(rs1)"
```

## encode_c_jalr_abi_xn_alias
- Tier: 4
- Rationale: ABI names (ra/sp/a0/…) and the xN spelling are the same GPR. Metamorphic: encoding is invariant under the ABI↔xN rename. Stronger differential already uses mixed names; this isolates the alias law. Round-trip rejected (no decoder). Domain excludes x0 (reserved / C.EBREAK collision).
- Doc contract: compressed.rs:55 "c.jalr rs1" — asserted fingerprint 45541b91
- Seed: (none)
- Formal: ∀ n ∈ {1..31}. encode_c_jalr([Reg(ABI[n])]) = encode_c_jalr([Reg(x{n})]); additionally n=8 ⇒ fp aliases x8
- Test file: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jalr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 1..31 }
  relation:
    op: eq
    lhs: sut_half([Reg(ABI[n])])
    rhs: sut_half([Reg(x{n})])
generators:
  n: { gen: int, min: 1, max: 31, type: u32 }
evidence: "encoder/mod.rs:423 get_reg via reg_num; RISC-V ABI GPR names"
```

## encode_c_jalr_field_isolation
- Tier: 4
- Rationale: CR-type fields are independent: changing rs1 must not alter bits outside [11:7] (op, funct4=1001, rs2=0 stay fixed). Metamorphic on a behavior-preserving field split. Stronger differential does not pin isolation. Documented bound: rs1 occupies bits[11:7] exactly.
- Doc contract: compressed.rs:55 "c.jalr rs1" — asserted fingerprint 45541b91
- Seed: (none)
- Formal: ∀ rs1_a, rs1_b ∈ {1..31}. encode(rs1_a) & ~0x0f80 = encode(rs1_b) & ~0x0f80; ∀ rs1 ∈ {1..31}. ((encode(rs1)>>7)&0x1f) = rs1
- Test file: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jalr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rs1_a, rs1_b, rs1]
  domain: { rs1_a: 1..31, rs1_b: 1..31, rs1: 1..31 }
  body: "(h(rs1_a)&~0x0f80)==(h(rs1_b)&~0x0f80) && ((h(rs1)>>7)&0x1f)==rs1"
generators:
  rs1_a: { gen: int, min: 1, max: 31, type: u32 }
  rs1_b: { gen: int, min: 1, max: 31, type: u32 }
  rs1: { gen: int, min: 1, max: 31, type: u32 }
evidence: "RISC-V Unprivileged ISA C.JALR CR-type field split"
```

## encode_c_jalr_neg_arity_fp
- Tier: 3
- Rationale: llvm-mc rejects too-few operands and FP registers as invalid operands for `c.jalr`. Negative/error contract: SUT must Err. Stronger oracles do not apply on the invalid domain.
- Doc contract: compressed.rs:55 "c.jalr rs1" — asserted fingerprint 45541b91
- Seed: (none)
- Formal: ∀ ops. |ops|<1 ⇒ encode_c_jalr(ops)=Err; ∀ fp ∈ FP-names. encode_c_jalr([Reg(fp)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jalr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp]
  domain: { ops: empty_ops, fp: fp_name }
  body: "encode_c_jalr(ops).is_err() && encode_c_jalr([Reg(fp)]).is_err()"
generators:
  ops: { gen: list, maxLen: 0 }
  fp: { gen: string }
expected_error: String
evidence: "llvm-mc too few operands / invalid operand; encoder/mod.rs:423 get_reg"
```

## encode_c_jalr_neg_extra
- Tier: 3
- Rationale: C.JALR is one-operand. llvm-mc rejects a second operand (`invalid operand for instruction`). Public dispatcher passes the operand slice through unchanged (mod.rs:933). SUT must Err on extra operands. Documented bound: exactly one operand (`c.jalr rs1`).
- Doc contract: compressed.rs:55 "c.jalr rs1" — asserted fingerprint 45541b91
- Seed: encode_c_jr_pbt.rs extra-operand negative (same CR-type arity contract)
- Formal: ∀ rs1 ∈ GPR-names\{x0}, extra ∈ Operand. encode_c_jalr([Reg(rs1), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
- Status: failing
- Counterexample: encode_c_jalr([Reg("x1"), Imm(0)]) → Ok(Half(0x9082))
- Bug report: bug_reports/encode_c_jalr_extra_operand.md

```property
function: encoder.encode_c_jalr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs1, extra]
  domain: { rs1: gpr_nz_name, extra: extra_operand }
  body: "encode_c_jalr([Reg(rs1), extra]).is_err()"
generators:
  rs1: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: "llvm-mc rejects c.jalr ra, x2; compressed.rs:55 one-operand form"
```

## encode_c_jalr_neg_rs1_x0
- Tier: 3
- Rationale: ISA: C.JALR is only valid when rs1≠x0; the code point with rs1=x0 (halfword 0x9002) is C.EBREAK, not C.JALR. llvm-mc rejects `c.jalr x0` and `c.jalr zero`. compress.rs:571 requires rs1 != 0 for C.JALR. SUT must Err. Domain is the documented reserved input, not a silent-narrowing of the generator.
- Doc contract: compressed.rs:55 "c.jalr rs1" — asserted fingerprint 45541b91
- Seed: encode_c_jr_pbt.rs rs1=x0 negative (CR-type x0 collision with C.EBREAK)
- Formal: ∀ name ∈ {x0, zero}. llvm-mc rejects `c.jalr name` ∧ encode_c_jalr([Reg(name)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_jalr_pbt.rs
- Status: failing
- Counterexample: encode_c_jalr([Reg("x0")]) → Ok(Half(0x9002))
- Bug report: bug_reports/encode_c_jalr_rs1_x0.md

```property
function: encoder.encode_c_jalr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: {x0, zero} }
  body: "encode_c_jalr([Reg(name)]).is_err()"
generators:
  name: { gen: string }
expected_error: String
evidence: "RISC-V Unprivileged ISA C.JALR rs1≠x0 (rs1=x0 is C.EBREAK); llvm-mc rejects c.jalr x0; compress.rs:571 rs1 != 0"
```
