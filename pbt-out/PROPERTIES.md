# Properties: encode_neon_float_three_same

## encode_neon_float_three_same_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (same GNU-style AArch64 text the assembler claims to accept). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree FP three-same decoder. Same-job sibling gate fails for encode_neon_three_same (integer size map), encode_neon_float_cmp_zero (two-misc #0), encode_fp_arith (scalar FP).
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, (U,size_hi,opcode,mnem) ∈ ARM-correct FP three-same table. encode_neon_float_three_same([Vd.T,Vn.T,Vm.T], U, size_hi, opcode) = llvm-mc("-triple=aarch64", "{mnem} Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_three_same
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, u, size_hi, opcode, mnem]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {2s,4s,2d} }
  relation:
    op: eq
    lhs: encode_neon_float_three_same([Vd.t, Vn.t, Vm.t], u, size_hi, opcode)
    rhs: llvm_mc("{mnem} Vd.t, Vn.t, Vm.t")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:1-7; neon.rs:1388-1403; encoder/mod.rs:377-496
```

## encode_neon_float_three_same_roundtrip_arm_fields
- Tier: 4
- Rationale: Weaker algebraic invariant of the ARM ARM three-same FP layout. Differential is stronger and used above; this pins field placement independently of llvm-mc.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_roundtrip_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, U,size_hi ∈ {0,1}, opcode ∈ {0..31}. let w = encode_neon_float_three_same([Vd.T,Vn.T,Vm.T], U, size_hi, opcode). Then w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=U ∧ w[28:24]=01110 ∧ w[23:22]=(size_hi<<1)|sz(T) ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:11]=opcode ∧ w[10]=1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_three_same
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, u, size_hi, opcode]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {2s,4s,2d}, u: {0,1}, size_hi: {0,1}, opcode: 0..31 }
  body: unpack(w) matches ARM ARM 0 Q U 01110 size 1 Rm opcode 1 Rn Rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  size_hi: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1388-1391 Format 0 Q U 01110 size 1 Rm opcode 1 Rn Rd
```

## encode_neon_float_three_same_metamorphic_u_bit
- Tier: 4
- Rationale: ARM ARM places U at bit 29; flipping U with other inputs fixed must XOR only that bit. Algebraic metamorphic, weaker than differential.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_metamorphic_u_bit
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, size_hi ∈ {0,1}, opcode ∈ {0..31}. encode(..., U=0, ...) XOR encode(..., U=1, ...) = 1<<29
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_three_same
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, size_hi, opcode]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {2s,4s,2d}, size_hi: {0,1}, opcode: 0..31 }
  relation:
    op: eq
    lhs: encode(ops, 0, size_hi, opcode) XOR encode(ops, 1, size_hi, opcode)
    rhs: 1 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  size_hi: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1388-1391 U at bit 29
```

## encode_neon_float_three_same_metamorphic_size_hi
- Tier: 4
- Rationale: ARM ARM places size[1] at bit 23; flipping size_hi with other inputs fixed must XOR only that bit.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_metamorphic_size_hi
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, U ∈ {0,1}, opcode ∈ {0..31}. encode(..., size_hi=0, ...) XOR encode(..., size_hi=1, ...) = 1<<23
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_three_same
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, u, opcode]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {2s,4s,2d}, u: {0,1}, opcode: 0..31 }
  relation:
    op: eq
    lhs: encode(ops, u, 0, opcode) XOR encode(ops, u, 1, opcode)
    rhs: 1 << 23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1389-1401 size[1]=size_hi at bit 23
```

## encode_neon_float_three_same_metamorphic_q
- Tier: 4
- Rationale: ARM ARM places Q at bit 30 from T; 2s (Q=0,sz=0) vs 4s (Q=1,sz=0) at equal register numbers and params must XOR only bit 30. Documented bound T ∈ {2s,4s,2d}.
- Seed: encode_neon_sli_pbt Q-pair metamorphic (8b vs 16b)
- Formal: ∀ rd,rn,rm ∈ {0..31}, U,size_hi ∈ {0,1}, opcode ∈ {0..31}. encode(2s, ...) XOR encode(4s, ...) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_three_same
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, u, size_hi, opcode]
  domain: { rd: vreg, rn: vreg, rm: vreg, u: {0,1}, size_hi: {0,1}, opcode: 0..31 }
  relation:
    op: eq
    lhs: encode(2s) XOR encode(4s)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  size_hi: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1396-1398 2s=>Q=0, 4s=>Q=1
```

## encode_neon_float_three_same_neg_unsupported_arrangement
- Tier: 5
- Rationale: Documented T domain is 2s/4s/2d (body match + llvm-mc rejects 8b/16b/4h/8h/1d without +fullfp16). Negative/error contract.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_unsupported_arrangement
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∉ {2s,4s,2d} among {8b,16b,4h,8h,1d,1s,3s,8s,"",b,h}, U,size_hi ∈ {0,1}, opcode ∈ {0..31}. encode([Vd.T,Vn.T,Vm.T], ...) = Err ∧ llvm-mc rejects "{mnem} Vd.T, Vn.T, Vm.T" when T nonempty
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, u, size_hi, opcode]
  domain: { t: {8b,16b,4h,8h,1d,1s,3s,8s,"",b,h} }
  relation:
    op: throws
    expr: encode_neon_float_three_same([Vd.t, Vn.t, Vm.t], u, size_hi, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "1d", "1s", "3s", "8s", "", "b", "h"] }
evidence: neon.rs:1396-1398; llvm-mc rejects 8b/16b/4h/8h/1d
```

## encode_neon_float_three_same_neg_extra_operands
- Tier: 5
- Rationale: llvm-mc and gas require exactly three operands for vector FADD-class. Extra operand must Err. Seed from sibling extra-operand property.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_extra_operands
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {2s,4s,2d}, insn ∈ ARM table, extra_kind ∈ {RegArrangement, Imm, Reg, Mem}. llvm-mc rejects four-operand asm ⇒ encode([Vd.T,Vn.T,Vm.T, extra]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, t="2s", insn=fadd (U=0,size_hi=0,opcode=26), extra_kind=0 — fadd v0.2s, v0.2s, v0.2s, v0.2s encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_float_three_same_extra_operand.md

```property
function: encoder.neon.encode_neon_float_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t, insn, extra_kind]
  domain: { t: {2s,4s,2d} }
  relation:
    op: throws
    expr: encode_neon_float_three_same([Vd.t, Vn.t, Vm.t, extra], u, size_hi, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra_kind: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc rejects fadd v0.4s, v1.4s, v2.4s, v3.4s; README.md:5-14 gas-compatible
```

## encode_neon_float_three_same_neg_arity_and_shape
- Tier: 5
- Rationale: llvm-mc rejects too-few operands, non-register slots, invalid names (v32/foo/empty/v/v-1/v99), dest Operand::Reg (no arrangement). get_neon_reg error paths.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_arity_and_shape
- Formal: ∀ n ∈ {0,1,2}, invalid dest/src/Vm ∈ {Imm,Mem,Symbol,Shift,Cond,Label}, bad name ∈ {v32,foo,"",v,v-1,v99}, dest Operand::Reg. encode(...) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, which, bad]
  domain: { n: 0..2, bad: {v32,foo,"",v,v-1,v99} }
  relation:
    op: throws
    expr: encode_neon_float_three_same(invalid_ops, u, size_hi, opcode)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
evidence: get_neon_reg neon.rs:7-21; parse_reg_num encoder/mod.rs:131-148
```

## encode_neon_float_three_same_neg_src_reg_no_arrangement
- Tier: 5
- Rationale: llvm-mc rejects `fadd v0.2s, v0, v0.2s` (source needs arrangement T). Source Operand::Reg must Err.
- Seed: encode_neon_sli_pbt::test_encode_neon_sli_regression_src_reg_no_arrangement
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, insn ∈ ARM table, slot ∈ {Vn,Vm}. encode with Operand::Reg at slot = Err ∧ llvm-mc rejects the corresponding asm
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t="2s", insn=fadd, which=1 — fadd v0.2s, v0, v0.2s encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_float_three_same_src_reg_no_arrangement.md

```property
function: encoder.neon.encode_neon_float_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, insn, which]
  domain: { t: {2s,4s,2d}, which: {1,2} }
  relation:
    op: throws
    expr: encode_neon_float_three_same(ops_with_Reg_source, u, size_hi, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  which: { gen: int, min: 1, max: 2, type: u32 }
evidence: llvm-mc rejects fadd v0.2s, v0, v0.2s; README.md:5-14 gas-compatible
```

## encode_neon_float_three_same_neg_non_v_prefix
- Tier: 5
- Rationale: llvm-mc rejects non-V register prefixes (x/w/d/s/q/h/b) on vector FP three-same. V register required.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_non_v_prefix
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}, prefix ∈ {x,w,d,s,q,h,b}, slot ∈ {0,1,2}, insn ∈ ARM table. llvm-mc rejects ⇒ encode with that prefix = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t="2s", insn=fadd, prefix="x", slot=0 — fadd x0.2s, v0.2s, v0.2s encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_float_three_same_non_v_prefix.md

```property
function: encoder.neon.encode_neon_float_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, insn, prefix, slot]
  domain: { prefix: {x,w,d,s,q,h,b}, slot: 0..2 }
  relation:
    op: throws
    expr: encode_neon_float_three_same(ops_with_non_v_prefix, u, size_hi, opcode)
expected_error: String
generators:
  prefix: { gen: oneof, values: ["x", "w", "d", "s", "q", "h", "b"] }
  slot: { gen: int, min: 0, max: 2, type: u32 }
evidence: llvm-mc rejects fadd x0.2s, v0.2s, v0.2s; README.md:5-14 gas-compatible
```

## encode_neon_float_three_same_neg_arrangement_mismatch
- Tier: 5
- Rationale: llvm-mc requires dest T = Vn T = Vm T. Mismatched arrangements must Err.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_arrangement_mismatch
- Formal: ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {2s,4s,2d} with Td≠Tn ∨ Td≠Tm, insn ∈ ARM table. llvm-mc rejects ⇒ encode([Vd.Td,Vn.Tn,Vm.Tm], ...) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, td="2d", tn="2s", tm="2s", insn=fadd — fadd v0.2d, v0.2s, v0.2s encodes as 2d form instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_float_three_same_arrangement_mismatch.md

```property
function: encoder.neon.encode_neon_float_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm, insn]
  domain: { td,tn,tm: {2s,4s,2d} }
  relation:
    op: throws
    expr: encode_neon_float_three_same([Vd.td, Vn.tn, Vm.tm], u, size_hi, opcode)
expected_error: String
generators:
  td: { gen: oneof, values: ["2s", "4s", "2d"] }
  tn: { gen: oneof, values: ["2s", "4s", "2d"] }
  tm: { gen: oneof, values: ["2s", "4s", "2d"] }
evidence: llvm-mc rejects fadd v0.4s, v1.2s, v2.4s; README.md:5-14 gas-compatible
```
