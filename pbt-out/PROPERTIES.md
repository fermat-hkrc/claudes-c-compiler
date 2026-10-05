# Properties: encode_neon_pmul

## encode_neon_pmul_diff_llvm_mc
- Tier: 3
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree PMUL decoder). Sibling encode_neon_three_same rejected (independence: shared get_neon_reg / dest-only Q). encode_neon_mul rejected (same-job: integer MUL, U=0). encode_neon_pmull rejected (same-job: widening three-different). llvm-mc is an independent AArch64 assembler; README.md:12 claims gas-compatible encodings. Doc evidence: neon.rs:335, README.md:12/224, ARM three-same PMUL T in {8B,16B}.
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}. llvm_mc("pmul Vd.t, Vn.t, Vm.t") = encode_neon_pmul([Vd.t, Vn.t, Vm.t]) as Word
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmul
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_pmul([Vd.t, Vn.t, Vm.t])
    rhs: llvm_mc("pmul Vd.t, Vn.t, Vm.t")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
evidence: neon.rs:335 neon.rs:341 ARM-ARM-PMUL-T-8B-16B
```

## encode_neon_pmul_metamorphic_rd_rn_rm
- Tier: 4
- Rationale: ARM three-same layout isolates Rd[4:0], Rn[9:5], Rm[20:16]. Weaker than differential but independently checks field packing. Stronger oracles applied on the valid domain above. Doc evidence: neon.rs:341 encoding diagram.
- Doc contract: neon.rs:341 "PMUL: 0 Q 1 01110 00 1 Rm 10011 1 Rn Rd (size=00 for bytes, U=1)" — asserted fingerprint 7bc199b2
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_metamorphic_rd_rn_rm
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}. let w(rd,rn,rm)=encode_neon_pmul([Vd.8b,Vn.8b,Vm.8b]). (w(rd1,rn1,rm1) ⊕ w(rd2,rn1,rm1)) ∧ ¬0x1F = 0 ∧ w(rd2,rn1,rm1)∧0x1F = rd2 ∧ (w(rd1,rn1,rm1) ⊕ w(rd1,rn2,rm1)) ∧ ¬(0x1F≪5) = 0 ∧ (w(rd1,rn2,rm1)≫5)∧0x1F = rn2 ∧ (w(rd1,rn1,rm1) ⊕ w(rd1,rn1,rm2)) ∧ ¬(0x1F≪16) = 0 ∧ (w(rd1,rn1,rm2)≫16)∧0x1F = rm2
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmul
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2]
  domain: { rd1: v0_v31, rd2: v0_v31, rn1: v0_v31, rn2: v0_v31, rm1: v0_v31, rm2: v0_v31 }
  body: changing only Rd/Rn/Rm differs only in bits[4:0]/[9:5]/[20:16]
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:341 ARM-ARM-three-same-Rd-Rn-Rm
```

## encode_neon_pmul_invariant_arm_fields
- Tier: 4
- Rationale: ARM PMUL word layout is fully specified: bit31=0, Q from T, U=1, bits[28:24]=01110, size=00, bit21=1, Rm, bits[15:10]=100111, Rn, Rd. Doc evidence: neon.rs:341.
- Doc contract: neon.rs:341 "PMUL: 0 Q 1 01110 00 1 Rm 10011 1 Rn Rd (size=00 for bytes, U=1)" — asserted fingerprint 7bc199b2
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}. let w=encode_neon_pmul([Vd.t,Vn.t,Vm.t]). w[31]=0 ∧ w[30]=⟦t=16b⟧ ∧ w[29]=1 ∧ w[28:24]=01110 ∧ w[23:22]=00 ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:10]=100111 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmul
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, t: {8b,16b} }
  body: ARM PMUL field layout holds (U=1, size=00, opcode=100111, Q from T)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
evidence: neon.rs:341 ARM-ARM-PMUL-encoding
```

## encode_neon_pmul_neg_arity
- Tier: 4
- Rationale: GNU/llvm-mc reject PMUL with fewer than 3 operands ("too few operands"). README.md:12 gas-compatible assembler. Negative/error contract. Doc evidence: neon.rs:335 (three-operand form Vd.T, Vn.T, Vm.T); llvm-mc rejects arity 0/1/2.
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn,rm ∈ {0..31}. llvm_mc(arity-n pmul) is Err ⇒ encode_neon_pmul(ops[:n]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm]
  domain: { n: 0..2, rd: v0_v31, rn: v0_v31, rm: v0_v31 }
  relation:
    op: holds
    expr: encode_neon_pmul(ops.take(n)).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:335 llvm-mc-too-few-operands
```

## encode_neon_pmul_neg_extra_operand
- Tier: 4
- Rationale: GNU/llvm-mc reject a fourth operand ("invalid operand"). The assembler contract requires exactly three vector operands. Doc evidence: neon.rs:335 three-operand form; llvm-mc rejects 4-operand pmul.
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, t ∈ {8b,16b}. llvm_mc("pmul Vd.t, Vn.t, Vm.t, Vextra.t") is Err ⇒ encode_neon_pmul([Vd.t,Vn.t,Vm.t,Vextra.t]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=0, t="8b" (pmul v0.8b, v0.8b, v0.8b, v0.8b)
- Bug report: bug_reports/encode_neon_pmul_extra_operand.md

```property
function: encoder.encode_neon_pmul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, extra: v0_v31, t: {8b,16b} }
  relation:
    op: holds
    expr: encode_neon_pmul([Vd.t, Vn.t, Vm.t, Vextra.t]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
evidence: neon.rs:335 llvm-mc-invalid-fourth-operand
```

## encode_neon_pmul_neg_invalid_t
- Tier: 4
- Rationale: ARM PMUL T is 8B or 16B only; llvm-mc rejects 4h/8h/2s/4s/1d/2d/1q and mismatched T. Doc "bytes only" asserts the domain; encoding a non-byte T as 8B is silent wrong output. Doc evidence: neon.rs:335; ARM ARM; llvm-mc.
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, td,tn,tm ∈ Arr. ¬(td∈{8b,16b} ∧ td=tn ∧ tn=tm) ⇒ llvm_mc("pmul Vd.td, Vn.tn, Vm.tm") is Err ⇒ encode_neon_pmul([Vd.td,Vn.tn,Vm.tm]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, td="8b", tn="8b", tm="16b" (pmul v0.8b, v0.8b, v0.16b)
- Bug report: bug_reports/encode_neon_pmul_mismatched_t.md

```property
function: encoder.encode_neon_pmul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, td: Arr, tn: Arr, tm: Arr }
  relation:
    op: holds
    expr: encode_neon_pmul([Vd.td, Vn.tn, Vm.tm]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  tn: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  tm: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
evidence: neon.rs:335 ARM-ARM-PMUL-T-8B-16B llvm-mc-invalid-operand
```

## encode_neon_pmul_neg_gpr_or_bare
- Tier: 4
- Rationale: GNU/llvm-mc require Vd.T / Vn.T / Vm.T arrangement operands. GPR, scalar FP, bare V, and xN.8b are rejected. get_neon_reg currently accepts Operand::Reg and parse_reg_num accepts x/w/d/s/q/v/h/b. Doc evidence: neon.rs:335; llvm-mc.
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr_dest, bare_vn, gpr_vm, bare_vd, xN_arr}. llvm_mc(kind) is Err ⇒ encode_neon_pmul(kind) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, kind=0, fp_prefix="x" (pmul x0, v0.8b, v0.8b)
- Bug report: bug_reports/encode_neon_pmul_gpr_or_bare.md

```property
function: encoder.encode_neon_pmul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, kind: {0..4} }
  relation:
    op: holds
    expr: encode_neon_pmul(gpr_or_bare(kind)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
evidence: neon.rs:335 llvm-mc-invalid-operand
```

## encode_neon_pmul_diff_alt_spellings
- Tier: 3
- Rationale: GNU-style assembler accepts uppercase mnemonic and V-register spellings. Differential vs llvm-mc on PMUL Vd.T,... with uppercase. Strengthens the valid-domain differential after the first batch. Doc evidence: neon.rs:335.
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {8b,16b}. llvm_mc("PMUL Vd.T, Vn.T, Vm.T") = encode_neon_pmul([V{rd}.t, V{rn}.t, V{rm}.t]) as Word
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmul
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_pmul([V{rd}.t, V{rn}.t, V{rm}.t])
    rhs: llvm_mc("PMUL Vd.T, Vn.T, Vm.T")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
evidence: neon.rs:335
```

## encode_neon_pmul_neg_reserved_t
- Tier: 4
- Rationale: Sweep: ARM PMUL size must be 00; T in {4H,8H,2S,4S,1D,2D,1Q} is reserved. The dest-only `arr_d == "16b"` test encodes every other arrangement as 8B. Dedicated generator so shrinking cannot hide behind mismatch. Doc evidence: neon.rs:335 "bytes only".
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_reserved_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, t ∈ {4h,8h,2s,4s,1d,2d,1q}. llvm_mc("pmul Vd.t, Vn.t, Vm.t") is Err ⇒ encode_neon_pmul([Vd.t,Vn.t,Vm.t]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, t="4h" (pmul v0.4h, v0.4h, v0.4h)
- Bug report: bug_reports/encode_neon_pmul_reserved_t.md

```property
function: encoder.encode_neon_pmul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, t: {4h,8h,2s,4s,1d,2d,1q} }
  relation:
    op: holds
    expr: encode_neon_pmul([Vd.t, Vn.t, Vm.t]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
evidence: neon.rs:335 ARM-ARM-PMUL-T-8B-16B
```

## encode_neon_pmul_neg_nonreg
- Tier: 4
- Rationale: Sweep: Imm / Mem / Label in any of the three slots is not a NEON register. get_neon_reg's `other` arm should Err. Doc evidence: neon.rs:335 Vd.T, Vn.T, Vm.T.
- Doc contract: neon.rs:335 "Encode NEON PMUL Vd.T, Vn.T, Vm.T (polynomial multiply, bytes only)" — asserted fingerprint de2000ce
- Seed: encode_neon_mul_pbt.rs:encode_neon_mul_neg_nonreg
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {Imm,Mem,Label}, slot ∈ {0,1,2}. encode_neon_pmul(ops with slot replaced by kind) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_pmul_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_pmul
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, slot]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, kind: {0..2}, slot: {0..2} }
  relation:
    op: holds
    expr: encode_neon_pmul(ops_with_nonreg).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 2, type: usize }
evidence: neon.rs:335 neon.rs:7-20
```
