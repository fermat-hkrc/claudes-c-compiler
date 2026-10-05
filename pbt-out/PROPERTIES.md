# Properties: encode_neon_mul

## encode_neon_mul_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree vector MUL decoder). Sibling encode_neon_three_same rejected as primary differential (independence gate: shared get_neon_reg / neon_arr_to_q_size / dest-only Q,size). encode_neon_elem rejected (same-job gate: by-element MUL). ARM MUL vector T in {8B,16B,4H,8H,2S,4S}; size:Q=11:x reserved.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: data_processing.rs:8648 encode_mul_kat_llvm_mc_neon_v0_16b
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_mul([Vd.T,Vn.T,Vm.T]) = llvm-mc("mul Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mul
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: "v0..v31", rn: "v0..v31", rm: "v0..v31", t: "{8b,16b,4h,8h,2s,4s}" }
  relation:
    op: eq
    lhs: encode_neon_mul([RegArrangement(v{rd},t), RegArrangement(v{rn},t), RegArrangement(v{rm},t)])
    rhs: llvm_mc("mul v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
evidence: neon.rs:322
```

## encode_neon_mul_metamorphic_rd_rn_rm
- Tier: 4
- Rationale: ARM three-same layout isolates Rd[4:0], Rn[9:5], Rm[20:16]. Stronger differential is the llvm-mc property; this metamorphic check does not need the external assembler and pins field placement independently of a copied encoder body.
- Doc contract: neon.rs:329 "MUL (vector): 0 Q 0 01110 size 1 Rm 10011 1 Rn Rd" — asserted fingerprint 76e072ed
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_metamorphic_rd_rn_rm_u
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. letting w(rd,rn,rm)=encode_neon_mul([Vd.8b,Vn.8b,Vm.8b]): (w(rd1,rn1,rm1) xor w(rd2,rn1,rm1)) & ~0x1F = 0 ∧ w(rd2,rn1,rm1)&0x1F = rd2 ∧ (w(rd1,rn1,rm1) xor w(rd1,rn2,rm1)) & ~(0x1F<<5) = 0 ∧ (w(rd1,rn2,rm1)>>5)&0x1F = rn2 ∧ (w(rd1,rn1,rm1) xor w(rd1,rn1,rm2)) & ~(0x1F<<16) = 0 ∧ (w(rd1,rn1,rm2)>>16)&0x1F = rm2
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mul
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { rd1: "0..31", rd2: "0..31", rn1: "0..31", rn2: "0..31", rm1: "0..31", rm2: "0..31" }
  relation:
    op: holds
    expr: "((w(rd1,rn1,rm1) ^ w(rd2,rn1,rm1)) & !0x1F) == 0 && (w(rd2,rn1,rm1) & 0x1F) == rd2 && ((w(rd1,rn1,rm1) ^ w(rd1,rn2,rm1)) & !(0x1F << 5)) == 0 && ((w(rd1,rn2,rm1) >> 5) & 0x1F) == rn2 && ((w(rd1,rn1,rm1) ^ w(rd1,rn1,rm2)) & !(0x1F << 16)) == 0 && ((w(rd1,rn1,rm2) >> 16) & 0x1F) == rm2"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:329
```

## encode_neon_mul_invariant_arm_fields
- Tier: 4
- Rationale: ARM Advanced SIMD three-same MUL layout is an exact structural invariant of every valid encoding. Weaker than llvm-mc differential; pins U=0, opcode 10011, size:Q from T, bit21=1 independently of the reference tool.
- Doc contract: neon.rs:329 "MUL (vector): 0 Q 0 01110 size 1 Rm 10011 1 Rn Rd" — asserted fingerprint 76e072ed
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. let w = encode_neon_mul([Vd.T,Vn.T,Vm.T]). bit31(w)=0 ∧ Q(w)=Q(T) ∧ U(w)=0 ∧ bits[28:24](w)=01110 ∧ size(w)=size(T) ∧ bit21(w)=1 ∧ Rm(w)=rm ∧ bits[15:10](w)=100111 ∧ Rn(w)=rn ∧ Rd(w)=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mul
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", t: "{8b,16b,4h,8h,2s,4s}" }
  relation:
    op: holds
    expr: "((w >> 31) & 1) == 0 && ((w >> 30) & 1) == q(t) && ((w >> 29) & 1) == 0 && ((w >> 24) & 0x1F) == 0b01110 && ((w >> 22) & 3) == size(t) && ((w >> 21) & 1) == 1 && ((w >> 16) & 0x1F) == rm && ((w >> 10) & 0x3F) == 0b100111 && ((w >> 5) & 0x1F) == rn && (w & 0x1F) == rd"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
evidence: neon.rs:329
```

## encode_neon_mul_neg_arity
- Tier: 3
- Rationale: GNU gas / llvm-mc reject MUL with fewer than 3 operands ("too few operands"). get_neon_reg fails on a missing slot, so the documented 3-operand form is a negative-error contract. Stronger oracles do not apply on the invalid-arity domain.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}. llvm-mc rejects arity-n mul ⇒ encode_neon_mul(first n of [Vd.8b,Vn.8b,Vm.8b]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm]
  domain: { n: "0..2", rd: "0..31", rn: "0..31", rm: "0..31" }
  relation:
    op: throws
    expr: encode_neon_mul(take(ops3, n))
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:322
```

## encode_neon_mul_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc / gas reject a fourth operand on vector MUL. The documented form is Vd.T, Vn.T, Vm.T (three operands). encode_neon_mul has no operands.len() check, so extra operands are a documented-error path the generator must reach.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. llvm-mc rejects "mul Vd.T, Vn.T, Vm.T, Vextra.T" ⇒ encode_neon_mul([Vd.T,Vn.T,Vm.T,Vextra.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, t=8b
- Bug report: bug_reports/encode_neon_mul_extra_operand.md

```property
function: encoder.encode_neon_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", extra: "0..31", t: "{8b,16b,4h,8h,2s,4s}" }
  relation:
    op: throws
    expr: encode_neon_mul([Vd.t, Vn.t, Vm.t, Vextra.t])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
expected_error: String
evidence: neon.rs:322
```

## encode_neon_mul_neg_invalid_t
- Tier: 3
- Rationale: ARM MUL vector T is {8B,16B,4H,8H,2S,4S} matching across Vd/Vn/Vm; size:Q=11:x (1D/2D) is reserved; llvm-mc rejects mismatched T, .2d, .1d, .1q. neon_arr_to_q_size accepts 1d/2d and source arrangements are discarded, so this is the error-path property for those documented rejections.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}. ¬(valid_mul_T(Td) ∧ Td=Tn ∧ Tn=Tm) ∧ llvm-mc rejects "mul Vd.Td, Vn.Tn, Vm.Tm" ⇒ encode_neon_mul([Vd.Td,Vn.Tn,Vm.Tm]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, td=8b, tn=8b, tm=16b
- Bug report: bug_reports/encode_neon_mul_mismatched_t.md

```property
function: encoder.encode_neon_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", td: "any_t", tn: "any_t", tm: "any_t" }
  relation:
    op: throws
    expr: encode_neon_mul([Vd.td, Vn.tn, Vm.tm])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: string }
  tn: { gen: string }
  tm: { gen: string }
expected_error: String
evidence: neon.rs:322
```

## encode_neon_mul_neg_gpr_or_bare
- Tier: 3
- Rationale: gas/llvm-mc require Vd.T, Vn.T, Vm.T. Bare Vn, GPR dest, scalar d/s/q/x/w, and xN.T are rejected. get_neon_reg accepts Operand::Reg (empty arrangement) and parse_reg_num accepts x/w prefixes, so those inputs are in the API domain and must Err.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr-dest, bare-Vn, x-Rm, bare-Vd, xN.8b-dest}. llvm-mc rejects the corresponding asm ⇒ encode_neon_mul(ops(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, kind=1
- Bug report: bug_reports/encode_neon_mul_bare_src.md

```property
function: encoder.encode_neon_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", kind: "0..4" }
  relation:
    op: throws
    expr: encode_neon_mul(ops_gpr_or_bare(kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:322
```

## encode_neon_mul_diff_alt_spellings
- Tier: 5
- Rationale: Sweep: uppercase MUL/V/T must agree with llvm-mc. parse_reg_num lowercases names. Same differential oracle as encode_neon_mul_diff_llvm_mc.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_mul([V{rd}.T, V{rn}.T, V{rm}.T]) = llvm-mc("MUL Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mul
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", t: "{8b,16b,4h,8h,2s,4s}" }
  relation:
    op: eq
    lhs: encode_neon_mul([RegArrangement(V{rd}, t), RegArrangement(V{rn}, t), RegArrangement(V{rm}, t)])
    rhs: llvm_mc("MUL V{rd}.{T}, V{rn}.{T}, V{rm}.{T}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
evidence: neon.rs:322
```

## encode_neon_mul_neg_nonreg
- Tier: 3
- Rationale: Sweep: Imm/Mem/Label in any slot must Err. get_neon_reg's other arm returns Err. Documented 3-register form.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_nonreg
- Formal: ∀ rd,rn,rm ∈ {0..31}, slot ∈ {0,1,2}, kind ∈ {Imm, Mem, Label}. encode_neon_mul(ops with slot replaced by kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, slot]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", kind: "0..2", slot: "0..2" }
  relation:
    op: throws
    expr: encode_neon_mul(ops_with_nonreg(slot, kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: neon.rs:322
```

## encode_neon_mul_neg_reserved_t
- Tier: 3
- Rationale: Sweep: ARM MUL vector size:Q=11:x is reserved (no 1D/2D). llvm-mc rejects mul v0.2d / v0.1d. neon_arr_to_q_size accepts both, so this generator is pinned to the reserved bound.
- Doc contract: neon.rs:322 "Encode NEON MUL Vd.T, Vn.T, Vm.T" — asserted fingerprint f9a56f7e
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_invalid_t (ADD allows 2d; MUL does not)
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {1d,2d}. llvm-mc rejects "mul Vd.T, Vn.T, Vm.T" ⇒ encode_neon_mul([Vd.T,Vn.T,Vm.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_mul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t=1d
- Bug report: bug_reports/encode_neon_mul_reserved_t.md

```property
function: encoder.encode_neon_mul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", t: "{1d,2d}" }
  relation:
    op: throws
    expr: encode_neon_mul([Vd.t, Vn.t, Vm.t])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
expected_error: String
evidence: neon.rs:322
```
