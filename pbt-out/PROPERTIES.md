# Properties: encode_neon_mla

## encode_neon_mla_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on GNU-style `mla Vd.T, Vn.T, Vm.T`. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree vector MLA decoder). Sibling encode_neon_three_same / encode_neon_mul rejected (independence / same-job gates). Assembler README claims GNU-style assembly (gas-compatible); llvm-mc is the independent reference for that encoding.
- Doc contract: neon.rs:348 "Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)" — asserted fingerprint 2fb74a86
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_mla([Vd.t,Vn.t,Vm.t]) = llvm-mc("mla Vd.t, Vn.t, Vm.t")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mla
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: valid_mla_t }
  relation:
    op: eq
    lhs: encode_neon_mla([RegArrangement(v{rd},t), RegArrangement(v{rn},t), RegArrangement(v{rm},t)])
    rhs: llvm_mc("mla v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
evidence: neon.rs:348
```

## encode_neon_mla_metamorphic_rd_rn_rm
- Tier: 3
- Rationale: ARM three-same layout places Rd at [4:0], Rn at [9:5], Rm at [20:16]. Changing one register must differ only in that field. Weaker than differential; complements it with an independent field-isolation check that does not invoke llvm-mc.
- Doc contract: neon.rs:354 "MLA: 0 Q 0 01110 size 1 Rm 10010 1 Rn Rd" — asserted fingerprint 279e1522
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_metamorphic_rd_rn_rm
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. let w111=encode_neon_mla(rd1,rn1,rm1,8b), w211=encode_neon_mla(rd2,rn1,rm1,8b), w121=encode_neon_mla(rd1,rn2,rm1,8b), w112=encode_neon_mla(rd1,rn1,rm2,8b). (w111⊕w211)∧¬0x1F=0 ∧ w211∧0x1F=rd2 ∧ (w111⊕w121)∧¬(0x1F≪5)=0 ∧ (w121≫5)∧0x1F=rn2 ∧ (w111⊕w112)∧¬(0x1F≪16)=0 ∧ (w112≫16)∧0x1F=rm2
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mla
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, rm1: u32_0_31, rm2: u32_0_31 }
  relation:
    op: holds
    expr: rd_rn_rm_fields_isolated(encode_neon_mla)
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:354
```

## encode_neon_mla_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM Advanced SIMD three-same MLA layout is an exact structural predicate on the success-path word. Weaker than differential/metamorphic.
- Doc contract: neon.rs:354 "MLA: 0 Q 0 01110 size 1 Rm 10010 1 Rn Rd" — asserted fingerprint 279e1522
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. let w=encode_neon_mla([Vd.t,Vn.t,Vm.t]). w[31]=0 ∧ w[30]=Q(t) ∧ w[29]=0 ∧ w[28:24]=01110 ∧ w[23:22]=size(t) ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:10]=100101 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mla
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: valid_mla_t }
  relation:
    op: holds
    expr: arm_mla_layout(encode_neon_mla(rd,rn,rm,t))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
evidence: neon.rs:354
```

## encode_neon_mla_neg_arity
- Tier: 4
- Rationale: GNU/gas and llvm-mc reject MLA with fewer than 3 operands. The function comment names three operands Vd.T, Vn.T, Vm.T. get_neon_reg on a missing slot returns Err; this pins the documented error contract vs llvm-mc.
- Doc contract: neon.rs:348 "Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)" — asserted fingerprint 2fb74a86
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}. llvm-mc rejects arity-n MLA ⇒ encode_neon_mla(first n of [Vd.8b,Vn.8b,Vm.8b]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mla
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm]
  domain: { n: 0..2, rd: u32_0_31, rn: u32_0_31, rm: u32_0_31 }
  relation:
    op: holds
    expr: encode_neon_mla(ops.take(n)).is_err()
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:348
```

## encode_neon_mla_neg_extra_operand
- Tier: 4
- Rationale: gas/llvm-mc reject a fourth operand. The function comment names exactly three operands Vd.T, Vn.T, Vm.T. The SUT has no operands.len() check, so extra operands are a documented-contract error path.
- Doc contract: neon.rs:348 "Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)" — asserted fingerprint 2fb74a86
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. llvm-mc rejects 4-operand MLA ⇒ encode_neon_mla([Vd.t,Vn.t,Vm.t,Vextra.t]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, t=8b
- Bug report: bug_reports/encode_neon_mla_extra_operand.md

```property
function: encoder.neon.encode_neon_mla
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, extra: u32_0_31, t: valid_mla_t }
  relation:
    op: holds
    expr: encode_neon_mla([Vd.t,Vn.t,Vm.t,Vextra.t]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
expected_error: String
evidence: neon.rs:348
```

## encode_neon_mla_neg_invalid_t
- Tier: 4
- Rationale: ARM MLA (vector) requires matching T in {8B,16B,4H,8H,2S,4S}; 1D/2D reserved; mismatched arrangements rejected by gas/llvm-mc. The SUT uses only dest arrangement and neon_arr_to_q_size accepts 1d/2d.
- Doc contract: neon.rs:348 "Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)" — asserted fingerprint 2fb74a86
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, td,tn,tm ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}. ¬(valid_mla(td) ∧ td=tn ∧ tn=tm) ⇒ llvm-mc rejects ∧ encode_neon_mla([Vd.td,Vn.tn,Vm.tm]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, td=8b, tn=8b, tm=16b
- Bug report: bug_reports/encode_neon_mla_mismatched_t.md

```property
function: encoder.neon.encode_neon_mla
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, td: any_t, tn: any_t, tm: any_t }
  relation:
    op: holds
    expr: encode_neon_mla([Vd.td,Vn.tn,Vm.tm]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tn: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tm: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
expected_error: String
evidence: neon.rs:348
```

## encode_neon_mla_neg_gpr_or_bare
- Tier: 4
- Rationale: gas/llvm-mc require Vd.T / Vn.T / Vm.T. Operand::Reg (bare vN, xN, wN, dN) and xN.8b are not GNU MLA vector operands. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w prefixes.
- Doc contract: neon.rs:348 "Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)" — asserted fingerprint 2fb74a86
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {0..4}. llvm-mc rejects GPR/bare/non-V prefix MLA ⇒ encode_neon_mla(kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, kind=1, fp_prefix=x  (mla v0.8b, v0, v0.8b)
- Bug report: bug_reports/encode_neon_mla_gpr_or_bare.md

```property
function: encoder.neon.encode_neon_mla
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, kind: 0..4 }
  relation:
    op: holds
    expr: encode_neon_mla(gpr_or_bare(kind)).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:348
```

## encode_neon_mla_diff_alt_spellings
- Tier: 2
- Rationale: Parser lowercases V/X prefixes and arrangements; GNU/gas accept uppercase MLA/V. Differential vs llvm-mc on uppercase spellings.
- Doc contract: neon.rs:348 "Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)" — asserted fingerprint 2fb74a86
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_mla([V{rd}.t, V{rn}.t, V{rm}.t]) = llvm-mc("MLA V{rd}.{T}, V{rn}.{T}, V{rm}.{T}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mla
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: valid_mla_t }
  relation:
    op: eq
    lhs: encode_neon_mla(uppercase_V_regs)
    rhs: llvm_mc("MLA V{rd}.{T}, V{rn}.{T}, V{rm}.{T}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
evidence: neon.rs:348
```

## encode_neon_mla_neg_nonreg
- Tier: 4
- Rationale: Imm/Mem/Label at any operand slot is not a NEON register. get_neon_reg returns Err for non-Reg/RegArrangement. Pins the error contract vs llvm-mc on dest.
- Doc contract: neon.rs:348 "Encode NEON MLA Vd.T, Vn.T, Vm.T (multiply-accumulate)" — asserted fingerprint 2fb74a86
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_nonreg
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {Imm,Mem,Label}, slot ∈ {0,1,2}. encode_neon_mla(ops with slot replaced by kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mla
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, slot]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, kind: 0..2, slot: 0..2 }
  relation:
    op: holds
    expr: encode_neon_mla(ops_with_nonreg(slot,kind)).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: neon.rs:348
```

## encode_neon_mla_neg_reserved_t
- Tier: 4
- Rationale: ARM MLA (vector) size:Q=11:x is reserved (no 1D/2D). neon_arr_to_q_size maps 1d/2d to size=11 and the SUT encodes them. Documented bound must be sampled exactly.
- Doc contract: neon.rs:354 "MLA: 0 Q 0 01110 size 1 Rm 10010 1 Rn Rd" — asserted fingerprint 279e1522
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_reserved_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {1d,2d}. llvm-mc rejects MLA Vd.t,Vn.t,Vm.t ⇒ encode_neon_mla([Vd.t,Vn.t,Vm.t]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mla_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t=1d
- Bug report: bug_reports/encode_neon_mla_reserved_t.md

```property
function: encoder.neon.encode_neon_mla
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: reserved_mla_t }
  relation:
    op: holds
    expr: encode_neon_mla([Vd.t,Vn.t,Vm.t]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["1d","2d"] }
expected_error: String
evidence: neon.rs:354
```
