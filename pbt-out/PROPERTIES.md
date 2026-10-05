# Properties: encode_neon_bsl

## encode_neon_bsl_diff_llvm_mc
- Tier: 3
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. README.md:12 claims gas-compatible textual assembly; BSL is listed in README.md:225 NEON three-same. llvm-mc is an independent AArch64 assembler. State machine rejected (pure function). Round-trip rejected (no in-tree BSL decoder). Sibling encode_neon_bic / encode_neon_bitwise_insert rejected (same-job gate: different opcodes).
- Doc contract: neon.rs:734 "Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T" — asserted fingerprint 6b1c6f73
- Seed: encode_neon_not_pbt.rs:encode_neon_not_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}. encode_neon_bsl([Vd.t, Vn.t, Vm.t]) = llvm-mc("bsl Vd.t, Vn.t, Vm.t")
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bsl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_bsl([arr(rd,t), arr(rn,t), arr(rm,t)])
    rhs: llvm_mc("bsl v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:734
```

## encode_neon_bsl_meta_rd_rn_rm
- Tier: 4
- Rationale: Metamorphic field isolation — changing only Rd/Rn/Rm must differ only in bits[4:0]/[9:5]/[20:16]. Independent of llvm-mc; grounded in ARM three-same layout cited at neon.rs:745.
- Doc contract: neon.rs:745 "BSL Vd.T, Vn.T, Vm.T: 0 Q 1 01110 01 1 Rm 000111 Rn Rd" — asserted fingerprint 173f4fb3
- Seed: encode_neon_not_pbt.rs:encode_neon_not_meta_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}, t ∈ {8b,16b}. (w(rd1,rn1,rm1,t) ⊕ w(rd2,rn1,rm1,t)) ∧ ¬0x1F = 0 ∧ w.Rd = rd; (w(rd1,rn1,rm1,t) ⊕ w(rd1,rn2,rm1,t)) ∧ ¬(0x1F≪5) = 0 ∧ w.Rn = rn; (w(rd1,rn1,rm1,t) ⊕ w(rd1,rn1,rm2,t)) ∧ ¬(0x1F≪16) = 0 ∧ w.Rm = rm
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bsl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2, t]
  domain: { rd1: vreg, rd2: vreg, rn1: vreg, rn2: vreg, rm1: vreg, rm2: vreg, t: {8b,16b} }
  relation:
    op: holds
    expr: "(w11^w21)&!0x1F==0 && (w11^w12)&!(0x1F<<5)==0 && (w11^w1m)&!(0x1F<<16)==0"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:745
```

## encode_neon_bsl_inv_layout
- Tier: 4
- Rationale: ARM Advanced SIMD three-same BSL encoding is an independent reference: 0 Q 1 01110 01 1 Rm 000111 Rn Rd = 0x2e601c00 | (Q<<30) | (Rm<<16) | (Rn<<5) | Rd. Q=1 iff T=16b.
- Doc contract: neon.rs:745 "BSL Vd.T, Vn.T, Vm.T: 0 Q 1 01110 01 1 Rm 000111 Rn Rd" — asserted fingerprint 173f4fb3
- Seed: encode_neon_not_pbt.rs:encode_neon_not_inv_layout
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}. encode_neon_bsl([Vd.t,Vn.t,Vm.t]) = 0x2e601c00 | (Q(t)<<30) | (rm≪16) | (rn≪5) | rd ∧ bits[31]=0 ∧ bits[29:10] match BSL fixed fields ∧ (w8b ⊕ w16b) = 1≪30
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bsl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_bsl([arr(rd,t), arr(rn,t), arr(rm,t)])
    rhs: 0x2e601c00 | (Q(t)<<30) | (rm<<16) | (rn<<5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:745
```

## encode_neon_bsl_neg_arity
- Tier: 4
- Rationale: Documented arity: neon.rs:737 "bsl requires 3 operands" (Err when len < 3).
- Doc contract: neon.rs:737 "bsl requires 3 operands" — domain-restriction fingerprint b6657fe1
- Seed: encode_neon_not_pbt.rs:encode_neon_not_neg_arity
- Formal: ∀ n ∈ {0,1,2}, ops with |ops|=n of valid Vd.T. encode_neon_bsl(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bsl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t]
  domain: { n: {0,1,2}, rd: vreg, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_bsl(ops_of_len(n))
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:737
```

## encode_neon_bsl_neg_extra
- Tier: 4
- Rationale: gas/llvm-mc reject a fourth operand. README.md:12 gas-compatible contract. SUT only checks len < 3.
- Doc contract: neon.rs:734 "Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T" — asserted fingerprint 6b1c6f73
- Seed: encode_neon_not_pbt.rs:encode_neon_not_neg_extra
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, t ∈ {8b,16b}. llvm-mc rejects "bsl Vd.t, Vn.t, Vm.t, Vextra.t" ⇒ encode_neon_bsl([Vd.t,Vn.t,Vm.t,Vextra.t]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, t="8b" (bsl v0.8b, v0.8b, v0.8b, v0.8b) → Ok(Word(0x2e601c00))
- Bug report: pbt-out/bug_reports/encode_neon_bsl_extra_operand.md

```property
function: encoder.encode_neon_bsl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t]
  domain: { rd: vreg, rn: vreg, rm: vreg, extra: vreg, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_bsl([arr(rd,t), arr(rn,t), arr(rm,t), arr(extra,t)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:734
```

## encode_neon_bsl_neg_invalid_t
- Tier: 4
- Rationale: ARM BSL T is 8B/16B only. llvm-mc rejects .4h/.8h/.2s/.4s/.2d/.1d. README gas-compat. Keep invalid T in the domain.
- Doc contract: neon.rs:734 "Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T" — asserted fingerprint 6b1c6f73
- Seed: encode_neon_not_pbt.rs:encode_neon_not_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {4h,8h,2s,4s,2d,1d,4b,8d,2h,1s}. llvm-mc rejects "bsl Vd.t, Vn.t, Vm.t" ⇒ encode_neon_bsl is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t="4h" (bsl v0.4h, v0.4h, v0.4h) → Ok(Word(0x2e601c00)) identical to .8b
- Bug report: pbt-out/bug_reports/encode_neon_bsl_invalid_t.md

```property
function: encoder.encode_neon_bsl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: invalid_bsl_t }
  relation:
    op: throws
    expr: encode_neon_bsl([arr(rd,t), arr(rn,t), arr(rm,t)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["4h", "8h", "2s", "4s", "2d", "1d", "4b", "8d", "2h", "1s"] }
expected_error: String
evidence: neon.rs:734
```

## encode_neon_bsl_neg_mismatch_t
- Tier: 4
- Rationale: ARM requires all three operands share T. llvm-mc rejects mixed 8b/16b. SUT discards Vn/Vm arrangements.
- Doc contract: neon.rs:734 "Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T" — asserted fingerprint 6b1c6f73
- Seed: encode_neon_not_pbt.rs:encode_neon_not_neg_mismatch_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, td,tn,tm ∈ {8b,16b} with ¬(td=tn=tm). llvm-mc rejects ⇒ encode_neon_bsl is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, td="8b", tn="8b", tm="16b" (bsl v0.8b, v0.8b, v0.16b) → Ok(Word(0x2e601c00))
- Bug report: pbt-out/bug_reports/encode_neon_bsl_mismatch_t.md

```property
function: encoder.encode_neon_bsl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm]
  domain: { rd: vreg, rn: vreg, rm: vreg, td: {8b,16b}, tn: {8b,16b}, tm: {8b,16b}, not_all_equal: true }
  relation:
    op: throws
    expr: encode_neon_bsl([arr(rd,td), arr(rn,tn), arr(rm,tm)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, items: ["8b", "16b"] }
  tn: { gen: oneof, items: ["8b", "16b"] }
  tm: { gen: oneof, items: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:734
```

## encode_neon_bsl_neg_gpr_bare_sp
- Tier: 4
- Rationale: BSL operands are arranged NEON registers. llvm-mc rejects GPR (x/w), SP, bare V (no arrangement), and scalar FP (d/s/q). get_neon_reg accepts Operand::Reg.
- Doc contract: neon.rs:734 "Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T" — asserted fingerprint 6b1c6f73
- Seed: encode_neon_not_pbt.rs:encode_neon_not_neg_gpr_bare_sp
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}, kind ∈ {x-gpr, w-dest, sp, bare-v, d-reg, s-dest, q-dest}. llvm-mc rejects ⇒ encode_neon_bsl is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t="8b", kind=0 (bsl x0, x0, x0) → Ok(Word(0x2e601c00)) same as bsl v0.8b, v0.8b, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_bsl_gpr_bare_sp.md

```property
function: encoder.encode_neon_bsl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, kind]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b}, kind: non_neon }
  relation:
    op: throws
    expr: encode_neon_bsl(non_arranged_ops(kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
  kind: { gen: int, min: 0, max: 6, type: u8 }
expected_error: String
evidence: neon.rs:734
```

## encode_neon_bsl_diff_alt_spellings
- Tier: 3
- Rationale: Sweep — parse_reg_num lowercases V prefixes; llvm-mc accepts uppercase V. Contract-surface round after coverage_gaps (no profraw; manual arm audit of uppercase V).
- Doc contract: neon.rs:734 "Encode NEON BSL (bitwise select): BSL Vd.T, Vn.T, Vm.T" — asserted fingerprint 6b1c6f73
- Seed: encode_neon_not_pbt.rs:encode_neon_not_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}. encode_neon_bsl([V{rd}.t, V{rn}.t, V{rm}.t]) = llvm-mc("bsl V{rd}.{t}, V{rn}.{t}, V{rm}.{t}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_bsl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_bsl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_bsl([arr_upper(rd,t), arr_upper(rn,t), arr_upper(rm,t)])
    rhs: llvm_mc("bsl V{rd}.{t}, V{rn}.{t}, V{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b"] }
evidence: neon.rs:734
```
