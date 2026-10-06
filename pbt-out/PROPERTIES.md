# Properties: encode_c_jr

## encode_c_jr_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential against llvm-mc assembling the same `c.jr` mnemonic. State machine rejected: encode_c_jr is a pure function with no lifecycle. Algebraic round-trip rejected: no in-tree C.JR decoder. try_compress_rv64 rejected by same-job gate (post-encode compress of 32-bit JALR, not the `c.jr` mnemonic). llvm-mc is an independent assembler; mapping is `[Reg(rs1)]` <-> `c.jr rs1` with rs1 ≠ x0.
- Doc contract: compressed.rs:49 "c.jr rs1" — asserted fingerprint e9aaba9d
- Seed: compress.rs:805 test_compress_ret (c.jr ra = 0x8082); pattern generalized from encode_c_mv_pbt.rs llvm-mc differential
- Formal: ∀ rs1 ∈ GPR-names\{x0,zero}. encode_c_jr([Reg(rs1)]) = Half(llvm-mc("c.jr rs1") LE u16)
- Test file: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jr
oracle: differential
predicate:
  quantifier: forall
  vars: [rs1]
  domain: { rs1: gpr_nz_name }
  relation:
    op: eq
    lhs: sut_half([Reg(rs1)])
    rhs: llvm_mc_half("c.jr {rs1}")
generators:
  rs1: { gen: string }
evidence: "encoder/mod.rs:930 c.jr => encode_c_jr; llvm-mc -triple=riscv64 -mattr=+c"
```

## encode_c_jr_cr_type_fields
- Tier: 4
- Rationale: RISC-V Unprivileged ISA CR-type layout for C.JR is an independent structural invariant (not copied from the SUT body). Stronger differential already covers value agreement; this pins field placement so a non-zero rs2 or wrong funct4 still fails. Round-trip rejected (no decoder).
- Doc contract: compressed.rs:49 "c.jr rs1" — asserted fingerprint e9aaba9d
- Seed: compress.rs:568 C.JR packing comment
- Formal: ∀ rs1 ∈ {1..31}. let h = encode_c_jr([Reg(x{rs1})]).h in (h&0b11)=0b10 ∧ ((h>>12)&0b1111)=0b1000 ∧ ((h>>7)&0x1f)=rs1 ∧ ((h>>2)&0x1f)=0
- Test file: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rs1]
  domain: { rs1: 1..31 }
  body: "unpack_c_jr(sut_half([Reg(x{rs1})])) == (op=0b10, funct4=0b1000, rs1, rs2=0)"
generators:
  rs1: { gen: int, min: 1, max: 31, type: u32 }
evidence: "RISC-V Unprivileged ISA C.JR CR-type; compress.rs:568 C.JR jalr x0, 0(rs1)"
```

## encode_c_jr_abi_xn_alias
- Tier: 4
- Rationale: ABI names (ra/sp/a0/…) and the xN spelling are the same GPR. Metamorphic: encoding is invariant under the ABI↔xN rename. Stronger differential already uses mixed names; this isolates the alias law. Round-trip rejected (no decoder). Domain excludes x0 (reserved for C.JR).
- Doc contract: compressed.rs:49 "c.jr rs1" — asserted fingerprint e9aaba9d
- Seed: (none)
- Formal: ∀ n ∈ {1..31}. encode_c_jr([Reg(ABI[n])]) = encode_c_jr([Reg(x{n})]); additionally n=8 ⇒ fp aliases x8
- Test file: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jr
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
evidence: "encoder/mod.rs:421 get_reg via reg_num; RISC-V ABI GPR names"
```

## encode_c_jr_field_isolation
- Tier: 4
- Rationale: CR-type fields are independent: changing rs1 must not alter bits outside [11:7] (op, funct4, rs2=0 stay fixed). Metamorphic on a behavior-preserving field split. Stronger differential does not pin isolation. Documented bound: rs1 occupies bits[11:7] exactly.
- Doc contract: compressed.rs:49 "c.jr rs1" — asserted fingerprint e9aaba9d
- Seed: (none)
- Formal: ∀ rs1_a, rs1_b ∈ {1..31}. encode(rs1_a) & ~0x0f80 = encode(rs1_b) & ~0x0f80; ∀ rs1 ∈ {1..31}. ((encode(rs1)>>7)&0x1f) = rs1
- Test file: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jr
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
evidence: "RISC-V Unprivileged ISA C.JR CR-type field split"
```

## encode_c_jr_neg_arity_fp
- Tier: 3
- Rationale: llvm-mc rejects too-few operands and FP registers as invalid operands for `c.jr`. Negative/error contract: SUT must Err. Stronger oracles do not apply on the invalid domain.
- Doc contract: compressed.rs:49 "c.jr rs1" — asserted fingerprint e9aaba9d
- Seed: (none)
- Formal: ∀ ops. |ops|<1 ⇒ encode_c_jr(ops)=Err; ∀ fp ∈ FP-names. encode_c_jr([Reg(fp)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_c_jr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, fp]
  domain: { ops: empty_ops, fp: fp_name }
  body: "encode_c_jr(ops).is_err() && encode_c_jr([Reg(fp)]).is_err()"
generators:
  ops: { gen: list, maxLen: 0 }
  fp: { gen: string }
expected_error: String
evidence: "llvm-mc too few operands / invalid operand; encoder/mod.rs:421 get_reg"
```

## encode_c_jr_neg_extra
- Tier: 3
- Rationale: C.JR is one-operand. llvm-mc rejects a second operand (`invalid operand for instruction`). Public dispatcher passes the operand slice through unchanged (mod.rs:930). SUT must Err on extra operands. Documented bound: exactly one operand (`c.jr rs1`).
- Doc contract: compressed.rs:49 "c.jr rs1" — asserted fingerprint e9aaba9d
- Seed: encode_c_mv_pbt.rs extra-operand negative (same CR-type arity contract)
- Formal: ∀ rs1 ∈ GPR-names\{x0}, extra ∈ Operand. encode_c_jr([Reg(rs1), extra]) = Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
- Status: failing
- Counterexample: encode_c_jr([Reg("x1"), Imm(0)]) → Ok(Half(0x8082))
- Bug report: bug_reports/encode_c_jr_extra_operand.md

```property
function: encoder.encode_c_jr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rs1, extra]
  domain: { rs1: gpr_nz_name, extra: extra_operand }
  body: "encode_c_jr([Reg(rs1), extra]).is_err()"
generators:
  rs1: { gen: string }
  extra: { gen: string }
expected_error: String
evidence: "llvm-mc rejects c.jr ra, x1; compressed.rs:49 one-operand form"
```

## encode_c_jr_neg_rs1_x0
- Tier: 3
- Rationale: ISA: C.JR is only valid when rs1≠x0; the code point with rs1=x0 (halfword 0x8002) is reserved. llvm-mc rejects `c.jr x0` and `c.jr zero`. compress.rs:566 requires rs1 != 0 for C.JR. SUT must Err. Domain is the documented reserved input, not a silent-narrowing of the generator.
- Doc contract: compressed.rs:49 "c.jr rs1" — asserted fingerprint e9aaba9d
- Seed: encode_c_mv_pbt.rs rs2=x0 negative (CR-type x0 collision with C.JR)
- Formal: ∀ name ∈ {x0, zero}. llvm-mc rejects `c.jr name` ∧ encode_c_jr([Reg(name)])=Err
- Test file: src/backend/riscv/assembler/encoder/encode_c_jr_pbt.rs
- Status: failing
- Counterexample: encode_c_jr([Reg("x0")]) → Ok(Half(0x8002))
- Bug report: bug_reports/encode_c_jr_rs1_x0.md

```property
function: encoder.encode_c_jr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: {x0, zero} }
  body: "encode_c_jr([Reg(name)]).is_err()"
generators:
  name: { gen: string }
expected_error: String
evidence: "RISC-V Unprivileged ISA C.JR rs1≠x0 reserved; llvm-mc rejects c.jr x0; compress.rs:566 rs1 != 0"
```
