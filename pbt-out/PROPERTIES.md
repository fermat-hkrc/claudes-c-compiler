# Properties: encode_neon_ext

## encode_neon_ext_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the valid EXT domain. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree EXT decoder). Sibling TBL/TBX/ZIP/UZP rejected (same-job gate: different opcodes). ARM/gas/llvm-mc agree on encodings for T in {8b,16b}, matching arrangements, in-range index.
- Doc contract: neon.rs:404 "Encode NEON EXT Vd.T, Vn.T, Vm.T, #index" — asserted fingerprint 94586e3d
- Seed: encode_neon_umov_pbt.rs llvm-mc differential
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, i ∈ [0, imax(T)]. encode_neon_ext([Vd.T, Vn.T, Vm.T, #i]) = llvm-mc("ext Vd.T, Vn.T, Vm.T, #i")
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ext
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, i]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b}, i: 0..=imax(t) }
  relation:
    op: eq
    lhs: encode_neon_ext([Vd.t, Vn.t, Vm.t, Imm(i)])
    rhs: llvm_mc("ext Vd.t, Vn.t, Vm.t, #i")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ext_metamorphic_rd_rn_rm
- Tier: 3
- Rationale: Weaker than differential; ARM encoding places Rd at bits[4:0], Rn at bits[9:5], Rm at bits[20:16]. Changing only one register must differ only in that field. Complements the llvm-mc differential (required metamorphic).
- Doc contract: neon.rs:416 "Encoding: 0 Q 10 1110 00 0 Rm 0 imm4 0 Rn Rd" — asserted fingerprint b512cb1d
- Seed: encode_neon_umov_pbt.rs metamorphic_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}, T ∈ {8b,16b}, i ∈ [0, imax(T)]. (encode(rd1,rn1,rm1) ⊕ encode(rd2,rn1,rm1)) & ~0x1F = 0 ∧ (encode(rd1,rn1,rm1) ⊕ encode(rd1,rn2,rm1)) & ~(0x1F<<5) = 0 ∧ (encode(rd1,rn1,rm1) ⊕ encode(rd1,rn1,rm2)) & ~(0x1F<<16) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ext
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2, t, i]
  domain: { rd1: vreg, rd2: vreg, rn1: vreg, rn2: vreg, rm1: vreg, rm2: vreg, t: {8b,16b}, i: 0..=imax(t) }
  relation:
    op: eq
    lhs: (w111 xor w211) and not 0x1F
    rhs: 0
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:416
```

## encode_neon_ext_invariant_arm_fields
- Tier: 3
- Rationale: ARM Advanced SIMD extract layout is an exact structural predicate on the success-path word. Weaker than differential (does not check the reference encoding of imm4/Q jointly with llvm-mc) but pins bit fields independently of the assembler.
- Doc contract: neon.rs:416 "Encoding: 0 Q 10 1110 00 0 Rm 0 imm4 0 Rn Rd" — asserted fingerprint b512cb1d
- Seed: encode_neon_umov_pbt.rs invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, i ∈ [0, imax(T)]. let w = encode_neon_ext(...). bit31(w)=0 ∧ Q(w)=(T=16b) ∧ bits[29:24]=101110 ∧ bits[23:21]=000 ∧ Rm=rm ∧ bit15=0 ∧ imm4=i ∧ bit10=0 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ext
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, i]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b}, i: 0..=imax(t) }
  relation:
    op: eq
    lhs: (w >> 24) and 0x3F
    rhs: 0b101110
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:416
```

## encode_neon_ext_diff_alt_spellings
- Tier: 2
- Rationale: parse_reg_num lowercases; llvm-mc and gas accept uppercase V and T. Differential over uppercase spellings of otherwise-valid EXT.
- Doc contract: neon.rs:404 "Encode NEON EXT Vd.T, Vn.T, Vm.T, #index" — asserted fingerprint 94586e3d
- Seed: encode_neon_umov_pbt.rs diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, i ∈ [0, imax(T)]. encode_neon_ext([V{rd}.T, V{rn}.T, V{rm}.T, #i]) = llvm-mc("ext V{rd}.T, V{rn}.T, V{rm}.T, #i")
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ext
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, i]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b}, i: 0..=imax(t) }
  relation:
    op: eq
    lhs: encode_neon_ext(uppercase V regs)
    rhs: llvm_mc(uppercase V/T asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ext_neg_arity
- Tier: 4
- Rationale: Documented arity error "ext requires 4 operands"; llvm-mc and gas reject fewer than 4. Negative/error contract: too few operands must Err.
- Doc contract: neon.rs:407 "ext requires 4 operands" — domain-restriction fingerprint bd0b0ba5
- Seed: encode_neon_umov_pbt.rs neg_arity_sp_fp
- Formal: ∀ n ∈ {0,1,2,3}, ops prefix of a valid EXT of length n. encode_neon_ext(ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ext
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, t, i]
  domain: { n: 0..=3, rd: vreg, rn: vreg, rm: vreg, t: {8b,16b}, i: 0..=imax(t) }
  relation:
    op: eq
    lhs: encode_neon_ext(valid_ops[..n]).is_err()
    rhs: true
expected_error: String
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:407
```

## encode_neon_ext_neg_extra_operand
- Tier: 4
- Rationale: gas and llvm-mc reject a fifth operand. README claims gas compatibility. The arity comment only guards `< 4`; extras are still invalid assembly (not a documented exclusion of the fifth operand).
- Doc contract: neon.rs:404 "Encode NEON EXT Vd.T, Vn.T, Vm.T, #index" — asserted fingerprint 94586e3d
- Seed: encode_neon_umov_pbt.rs neg_extra_operand
- Formal: ∀ valid 4-operand EXT ops, extra ∈ Operand. encode_neon_ext(ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: failing
- Counterexample: encode_neon_ext([v0.8b, v0.8b, v0.8b, #0, v0.8b]) = Ok(Word(0x2e000000))
- Bug report: bug_reports/encode_neon_ext_extra_operand.md

```property
function: encoder.encode_neon_ext
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, i, extra]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b}, i: 0..=imax(t), extra: vreg }
  relation:
    op: eq
    lhs: encode_neon_ext(ops ++ [extra]).is_err()
    rhs: true
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b"] }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ext_neg_invalid_t
- Tier: 4
- Rationale: ARM/gas/llvm-mc accept only T in {8B,16B}. Arrangements 8h/4h/4s/2s/2d/1d/b/h/s/d are invalid. SUT sets Q=1 iff arr_d=="16b" and otherwise encodes Q=0, so invalid T is currently accepted. This is the finding, not a generator exclusion.
- Doc contract: neon.rs:415 "EXT Vd.T, Vn.T, Vm.T, #index" — asserted fingerprint 8214fdf0
- Seed: encode_neon_tbx_pbt.rs invalid Ta; encode_neon_umov unsupported elem_size
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∉ {8b,16b} among ARM arrangement tokens, i ∈ [0,15]. encode_neon_ext([Vd.T, Vn.T, Vm.T, #i]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: failing
- Counterexample: encode_neon_ext([v0.8h, v0.8h, v0.8h, #0]) = Ok(Word(0x2e000000))
- Bug report: bug_reports/encode_neon_ext_invalid_t.md

```property
function: encoder.encode_neon_ext
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, i]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8h,4h,4s,2s,2d,1d,b,h,s,d}, i: 0..=15 }
  relation:
    op: eq
    lhs: encode_neon_ext([Vd.t, Vn.t, Vm.t, Imm(i)]).is_err()
    rhs: true
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8h", "4h", "4s", "2s", "2d", "1d", "b", "h", "s", "d"] }
  i: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ext_neg_index_oor
- Tier: 4
- Rationale: gas rejects index outside 0-7 (8B) / 0-15 (16B) ("immediate value out of range"). ARM: Q=0 and imm4<3>!=0 is UNALLOCATED. Field masking (`index & 0xF`) is the classic assembler bug class. llvm-mc wraps; README claims gas, so the contract is Err not wrap. Documented bounds sampled at imax+1 and beyond.
- Doc contract: neon.rs:404 "Encode NEON EXT Vd.T, Vn.T, Vm.T, #index" — asserted fingerprint 94586e3d
- Seed: encode_neon_umov_pbt.rs neg_index_oor; pbt-patterns generator-design assembler range contracts
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b}, i > imax(T) ∨ i < 0. encode_neon_ext([Vd.T, Vn.T, Vm.T, #i]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: failing
- Counterexample: encode_neon_ext([v0.8b, v0.8b, v0.8b, #8]) = Ok(Word(0x2e004000))
- Bug report: bug_reports/encode_neon_ext_index_oor.md

```property
function: encoder.encode_neon_ext
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, i]
  domain: { rd: vreg, rn: vreg, rm: vreg, t: {8b,16b}, i: (imax(t)+1).. OR i < 0 }
  relation:
    op: eq
    lhs: encode_neon_ext([Vd.t, Vn.t, Vm.t, Imm(i)]).is_err()
    rhs: true
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b"] }
  over: { gen: int, min: 1, max: 16, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ext_neg_mismatched_t
- Tier: 4
- Rationale: gas and llvm-mc require matching T on Vd, Vn, Vm. SUT discards source arrangements. Not a documented exclusion.
- Doc contract: neon.rs:415 "EXT Vd.T, Vn.T, Vm.T, #index" — asserted fingerprint 8214fdf0
- Seed: encode_neon_tbx_pbt.rs mismatched T
- Formal: ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {8b,16b}, i ∈ [0,7]. (Td ≠ Tn ∨ Td ≠ Tm) ⇒ encode_neon_ext([Vd.Td, Vn.Tn, Vm.Tm, #i]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: failing
- Counterexample: encode_neon_ext([v0.8b, v0.8b, v0.16b, #0]) = Ok(Word(0x2e000000))
- Bug report: bug_reports/encode_neon_ext_mismatched_t.md

```property
function: encoder.encode_neon_ext
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t_d, t_n, t_m, i]
  domain: { t_d: {8b,16b}, t_n: {8b,16b}, t_m: {8b,16b}, i: 0..=7 }
  relation:
    op: eq
    lhs: encode_neon_ext(ops).is_err()
    rhs: true
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t_d: { gen: oneof, values: ["8b", "16b"] }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_ext_neg_gpr_or_bare
- Tier: 4
- Rationale: gas and llvm-mc require Vd.T / Vn.T / Vm.T / #imm. Operand::Reg (GPR, FP, bare V) and non-Imm index are invalid. get_neon_reg accepts Operand::Reg.
- Doc contract: neon.rs:404 "Encode NEON EXT Vd.T, Vn.T, Vm.T, #index" — asserted fingerprint 94586e3d
- Seed: encode_neon_umov_pbt.rs neg_arity_sp_fp / non_lane_src
- Formal: ∀ kind ∈ {GPR dest, bare Vn, GPR Vm, bare V dest, non-Imm index}. encode_neon_ext(ops) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ext_pbt.rs
- Status: failing
- Counterexample: encode_neon_ext([x0, v0.8b, v0.8b, #0]) = Ok(Word(0x2e000000))
- Bug report: bug_reports/encode_neon_ext_gpr_dest.md

```property
function: encoder.encode_neon_ext
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, i, kind]
  domain: { kind: 0..=4 }
  relation:
    op: eq
    lhs: encode_neon_ext(ops).is_err()
    rhs: true
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
evidence: src/backend/arm/assembler/README.md:12
```
