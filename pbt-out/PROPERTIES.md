# Properties: encode_neon_mls

## encode_neon_mls_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on GNU-style `mls Vd.T, Vn.T, Vm.T`. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree vector MLS decoder). Sibling encode_neon_three_same / encode_neon_mul / encode_neon_mla rejected (independence / same-job gates). Assembler README claims GNU-style assembly (gas-compatible); llvm-mc is the independent reference for that encoding.
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_mls([Vd.t,Vn.t,Vm.t]) = llvm-mc("mls Vd.t, Vn.t, Vm.t")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mls
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: valid_mls_t }
  relation:
    op: eq
    lhs: encode_neon_mls([RegArrangement(v{rd},t), RegArrangement(v{rn},t), RegArrangement(v{rm},t)])
    rhs: llvm_mc("mls v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
evidence: neon.rs:361
```

## encode_neon_mls_metamorphic_rd_rn_rm
- Tier: 3
- Rationale: ARM three-same layout places Rd at [4:0], Rn at [9:5], Rm at [20:16]. Changing one register must differ only in that field. Weaker than differential; complements it with an independent field-isolation check that does not invoke llvm-mc.
- Doc contract: neon.rs:365 "MLS: 0 Q 1 01110 size 1 Rm 10010 1 Rn Rd (U=1)" — asserted fingerprint f7e9ecf0
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_metamorphic_rd_rn_rm
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. let w111=encode_neon_mls(rd1,rn1,rm1,8b), w211=encode_neon_mls(rd2,rn1,rm1,8b), w121=encode_neon_mls(rd1,rn2,rm1,8b), w112=encode_neon_mls(rd1,rn1,rm2,8b). (w111⊕w211)∧¬0x1F=0 ∧ w211∧0x1F=rd2 ∧ (w111⊕w121)∧¬(0x1F≪5)=0 ∧ (w121≫5)∧0x1F=rn2 ∧ (w111⊕w112)∧¬(0x1F≪16)=0 ∧ (w112≫16)∧0x1F=rm2
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mls
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, rm1: u32_0_31, rm2: u32_0_31 }
  relation:
    op: holds
    expr: rd_rn_rm_fields_isolated(encode_neon_mls)
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:365
```

## encode_neon_mls_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM Advanced SIMD three-same MLS layout is an exact structural predicate on the success-path word. Weaker than differential/metamorphic. U=1 distinguishes MLS from MLA.
- Doc contract: neon.rs:365 "MLS: 0 Q 1 01110 size 1 Rm 10010 1 Rn Rd (U=1)" — asserted fingerprint f7e9ecf0
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. let w=encode_neon_mls([Vd.t,Vn.t,Vm.t]). w[31]=0 ∧ w[30]=Q(t) ∧ w[29]=1 ∧ w[28:24]=01110 ∧ w[23:22]=size(t) ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:10]=100101 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mls
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: valid_mls_t }
  relation:
    op: holds
    expr: arm_mls_fields(w)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
evidence: neon.rs:365
```

## encode_neon_mls_neg_arity
- Tier: 5
- Rationale: GNU/gas and llvm-mc reject MLS with fewer than 3 operands. encode_neon_mls has no explicit len check; get_neon_reg on a missing index must still Err. Negative/error contract vs llvm-mc.
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}. llvm-mc rejects arity-n mls ⇒ encode_neon_mls(ops[0..n]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mls
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm]
  domain: { n: 0..2, rd: u32_0_31, rn: u32_0_31, rm: u32_0_31 }
  relation:
    op: holds
    expr: encode_neon_mls(ops.take(n)).is_err()
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:361
```

## encode_neon_mls_neg_extra_operand
- Tier: 5
- Rationale: llvm-mc/gas reject a fourth operand on MLS. README.md:12 claims the assembler accepts the same textual assembly gas would consume. encode_neon_mls reads only indices 0..2; extra must still be rejected.
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. llvm-mc rejects 4-operand mls ⇒ encode_neon_mls([Vd.t,Vn.t,Vm.t,Vextra.t]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, t=8b
- Bug report: pbt-out/bug_reports/encode_neon_mls_extra_operand.md

```property
function: encoder.neon.encode_neon_mls
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, extra: u32_0_31, t: valid_mls_t }
  relation:
    op: holds
    expr: encode_neon_mls(four_ops).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
expected_error: String
evidence: assembler/README.md:12
```

## encode_neon_mls_neg_invalid_t
- Tier: 5
- Rationale: ARM Advanced SIMD three-same MLS allows T in {8B,16B,4H,8H,2S,4S} matching across Vd/Vn/Vm; size:Q=11:x (1D/2D) is reserved. llvm-mc rejects mismatched and reserved T. The function's comment asserts Vd.T,Vn.T,Vm.T (same T); it does not declare 1d/2d invalid, so those stay in the generator.
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, td,tn,tm ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}. ¬(valid_mls(td) ∧ td=tn=tm) ∧ llvm-mc rejects ⇒ encode_neon_mls([Vd.td,Vn.tn,Vm.tm]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, td=8b, tn=16b, tm=8b
- Bug report: pbt-out/bug_reports/encode_neon_mls_mismatched_t.md

```property
function: encoder.neon.encode_neon_mls
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, td: any_t, tn: any_t, tm: any_t }
  relation:
    op: holds
    expr: encode_neon_mls(mismatched_or_reserved).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tn: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tm: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
expected_error: String
evidence: neon.rs:361
```

## encode_neon_mls_neg_gpr_or_bare
- Tier: 5
- Rationale: llvm-mc/gas require Vd.T / Vn.T / Vm.T arrangement operands. GPR dest, bare V, scalar FP prefixes, and xN.T are rejected by the reference assembler. get_neon_reg currently accepts Operand::Reg and parse_reg_num accepts x/w prefixes; those must still Err under the GNU-style contract.
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {0..4}. llvm-mc rejects the corresponding non-arrangement form ⇒ encode_neon_mls(ops_kind) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, kind=1, fp_prefix=x (mls v0.8b, v0, v0.8b)
- Bug report: pbt-out/bug_reports/encode_neon_mls_gpr_or_bare.md

```property
function: encoder.neon.encode_neon_mls
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, fp_prefix]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, kind: 0..4, fp_prefix: {x,w,d,s,q,h,b} }
  relation:
    op: holds
    expr: encode_neon_mls(gpr_or_bare_ops).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
  fp_prefix: { gen: oneof, values: ["x","w","d","s","q","h","b"] }
expected_error: String
evidence: assembler/README.md:12
```

## encode_neon_mls_diff_alt_spellings
- Tier: 2
- Rationale: Parser lowercases V/X/W prefixes and arrangements; llvm-mc accepts uppercase MLS/V/T. Differential agreement on the alt-spelling surface is required by the GNU-style contract (README.md:12).
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_mls([V{rd}.t, V{rn}.t, V{rm}.t]) = llvm-mc("MLS Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mls
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: valid_mls_t }
  relation:
    op: eq
    lhs: encode_neon_mls([RegArrangement(V{rd},t), RegArrangement(V{rn},t), RegArrangement(V{rm},t)])
    rhs: llvm_mc("MLS V{rd}.{T}, V{rn}.{T}, V{rm}.{T}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
evidence: assembler/README.md:12
```

## encode_neon_mls_neg_nonreg
- Tier: 5
- Rationale: get_neon_reg's non-register arm must Err for Imm/Mem/Label at any slot. llvm-mc rejects dest-slot nonreg. Sweep: documented error path of get_neon_reg other.
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_neg_nonreg
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {Imm,Mem,Label}, slot ∈ {0,1,2}. encode_neon_mls(ops with slot replaced by nonreg) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_mls
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, slot]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, kind: 0..2, slot: 0..2 }
  relation:
    op: holds
    expr: encode_neon_mls(nonreg_ops).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: neon.rs:7
```

## encode_neon_mls_neg_reserved_t
- Tier: 5
- Rationale: ARM Advanced SIMD three-same MLS reserves size:Q=11:x (1D/2D). llvm-mc rejects. neon_arr_to_q_size accepts 1d/2d. Dedicated boundary property so the generator is guaranteed to hit the reserved encodings.
- Doc contract: neon.rs:361 "Encode NEON MLS Vd.T, Vn.T, Vm.T (multiply-subtract)" — asserted fingerprint 12a577e8
- Seed: encode_neon_mla_pbt.rs:encode_neon_mla_neg_reserved_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {1d,2d}. llvm-mc rejects mls Vd.t,Vn.t,Vm.t ⇒ encode_neon_mls is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mls_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t=1d
- Bug report: pbt-out/bug_reports/encode_neon_mls_reserved_t.md

```property
function: encoder.neon.encode_neon_mls
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: {1d,2d} }
  relation:
    op: holds
    expr: encode_neon_mls(ops_t(rd,rn,rm,t)).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["1d","2d"] }
expected_error: String
evidence: neon.rs:365
```
