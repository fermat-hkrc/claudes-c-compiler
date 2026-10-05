# Properties: encode_neon_faddp

## encode_neon_faddp_diff_llvm_mc_vector
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the documented vector domain FADDP Vd.T, Vn.T, Vm.T with T in {2s,4s,2d}. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FADDP decoder). Sibling encode_neon_float_three_same rejected (same-job gate: FADD vs pairwise FADDP, different opcode). Doc evidence: README.md:12 gas-compatible assembly; README.md:226 lists faddp; neon.rs:1682 vector form.
- Doc contract: neon.rs:1682 "Vector form: FADDP Vd.T, Vn.T, Vm.T" — asserted fingerprint 69be30d9
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}. encode_neon_faddp([Vd.T, Vn.T, Vm.T]) = Word(w) ∧ w = llvm-mc("faddp Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_faddp
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: {2s,4s,2d} }
  relation:
    op: eq
    lhs: encode_neon_faddp([arr(rd,t), arr(rn,t), arr(rm,t)])
    rhs: llvm_mc("faddp v{rd}.{t}, v{rn}.{t}, v{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_neon_faddp_diff_llvm_mc_scalar
- Tier: 2
- Rationale: Documented scalar form FADDP Sd, Vn.2S / FADDP Dd, Vn.2D must agree with llvm-mc. Same stronger-oracle rejection chain as the vector differential. Doc evidence: neon.rs:1684.
- Doc contract: neon.rs:1684 "Scalar form: FADDP Sd, Vn.2S  or FADDP Dd, Vn.2D" — asserted fingerprint edfd218d
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, (dest,T) ∈ {(s,2s),(d,2d)}. encode_neon_faddp([Reg(dest rd), Vn.T]) = Word(w) ∧ w = llvm-mc("faddp {dest}rd, Vn.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_faddp
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest, t]
  domain: { rd: u32_0_31, rn: u32_0_31, (dest,t): {(s,2s),(d,2d)} }
  relation:
    op: eq
    lhs: encode_neon_faddp([Reg(dest rd), arr(rn,t)])
    rhs: llvm_mc("faddp {dest}{rd}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest: { gen: element, of: ["s", "d"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:1684
```

## encode_neon_faddp_meta_rd_rn_rm
- Tier: 3
- Rationale: ARM three-same layout isolates Rd[4:0], Rn[9:5], Rm[20:16]. Changing one register must differ only in that field. Weaker than differential (already used for value agreement) but independently checks field placement. Doc evidence: neon.rs:1683.
- Doc contract: neon.rs:1683 "Format: 0 Q 1 01110 0 sz 1 Rm 110101 Rn Rd" — asserted fingerprint 9590c88c
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_meta_rd_rn_rm
- Formal: ∀ rd1,rd2,rn1,rn2,rm1,rm2 ∈ {0..31}, T ∈ {2s,4s,2d}. let w(rd,rn,rm)=encode_neon_faddp([Vd.T,Vn.T,Vm.T]). (w(rd1,rn1,rm1) ⊕ w(rd2,rn1,rm1)) ∧ ¬0x1F = 0 ∧ w&0x1F=rd; (w(rd1,rn1,rm1) ⊕ w(rd1,rn2,rm1)) ∧ ¬(0x1F<<5) = 0; (w(rd1,rn1,rm1) ⊕ w(rd1,rn1,rm2)) ∧ ¬(0x1F<<16) = 0
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_faddp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, rm1, rm2, t]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31, rm1: u32_0_31, rm2: u32_0_31, t: {2s,4s,2d} }
  relation:
    op: holds
    expr: "((sut_word(&[arr(rd1,t),arr(rn1,t),arr(rm1,t)])? ^ sut_word(&[arr(rd2,t),arr(rn1,t),arr(rm1,t)])?) & !0x1Fu32) == 0"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  rm1: { gen: int, min: 0, max: 31, type: u32 }
  rm2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:1683
```

## encode_neon_faddp_inv_layout
- Tier: 3
- Rationale: Documented ARM bit layout for vector (0 Q 1 01110 0 sz 1 Rm 110101 Rn Rd) and scalar (01 1 11110 0 sz 11000 01101 10 Rn Rd) must hold exactly, including Q/sz for each T. Doc evidence: neon.rs:1683 and neon.rs:1685.
- Doc contract: neon.rs:1683 "Format: 0 Q 1 01110 0 sz 1 Rm 110101 Rn Rd" — asserted fingerprint 9590c88c
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_inv_layout
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}. encode_neon_faddp([Vd.T,Vn.T,Vm.T]) = Word(ARM_FADDP_VEC(rd,rn,rm,T)). ∀ rd,rn ∈ {0..31}, (sz,T) ∈ {(0,2s),(1,2d)}. encode_neon_faddp([Reg(s|d rd), Vn.T]) = Word(ARM_FADDP_SISD(rd,rn,sz))
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_faddp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: {2s,4s,2d} }
  relation:
    op: eq
    lhs: encode_neon_faddp([arr(rd,t), arr(rn,t), arr(rm,t)])
    rhs: arm_faddp_vec(rd, rn, rm, t)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:1683
```

## encode_neon_faddp_neg_extra
- Tier: 3
- Rationale: Documented arity is 2 or 3 operands (neon.rs:1718 "faddp requires 2 or 3 operands"; vector form is three operands; llvm-mc rejects a fourth). The SUT uses `operands.len() >= 3` with no maximum, so this is the documented error contract. Doc evidence: neon.rs:1718; README.md:12 gas compatibility (gas/llvm-mc reject extra).
- Doc contract: neon.rs:1718 "faddp requires 2 or 3 operands" — domain-restriction fingerprint 9f4f4026
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_extra
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ {2s,4s,2d}. llvm-mc rejects "faddp Vd.T, Vn.T, Vm.T, Vextra.T" ⇒ encode_neon_faddp([Vd.T,Vn.T,Vm.T,Vextra.T]) is Err. Also ∀ n ∈ {0,1}. encode_neon_faddp(ops with n operands) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: failing
- Counterexample: encode_neon_faddp([v0.2s, v0.2s, v0.2s, v0.2s])
- Bug report: bug_reports/encode_neon_faddp_extra_operand.md

```property
function: encoder.encode_neon_faddp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, extra: u32_0_31, t: {2s,4s,2d} }
  relation:
    op: throws
    expr: encode_neon_faddp(&[arr(rd,t), arr(rn,t), arr(rm,t), arr(extra,t)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["2s", "4s", "2d"] }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1718
```

## encode_neon_faddp_neg_invalid_t
- Tier: 3
- Rationale: ARM/docs restrict vector T to {2S,4S,2D}; llvm-mc rejects 8b/16b/4h/8h/1d/etc (4h/8h need fullfp16, not claimed). Doc evidence: neon.rs:1682 vector form; README.md:12.
- Doc contract: neon.rs:1682 "Vector form: FADDP Vd.T, Vn.T, Vm.T" — asserted fingerprint 69be30d9
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∉ {2s,4s,2d} ∧ T ∈ NEON_ARR. llvm-mc rejects "faddp Vd.T, Vn.T, Vm.T" ⇒ encode_neon_faddp([Vd.T,Vn.T,Vm.T]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_faddp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: invalid_t }
  relation:
    op: throws
    expr: encode_neon_faddp(&[arr(rd,t), arr(rn,t), arr(rm,t)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b", "16b", "4h", "8h", "1d", "1s", "4d", "2h", "1q"] }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1682
```

## encode_neon_faddp_neg_mismatch_t
- Tier: 3
- Rationale: Documented vector form uses one T for Vd, Vn, Vm; llvm-mc/gas reject mismatched arrangements. Doc evidence: neon.rs:1682 "FADDP Vd.T, Vn.T, Vm.T"; README.md:12.
- Doc contract: neon.rs:1682 "Vector form: FADDP Vd.T, Vn.T, Vm.T" — asserted fingerprint 69be30d9
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_mismatch_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ {2s,4s,2d}. ¬(Td=Tn=Tm) ∧ llvm-mc rejects "faddp Vd.Td, Vn.Tn, Vm.Tm" ⇒ encode_neon_faddp([Vd.Td,Vn.Tn,Vm.Tm]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: failing
- Counterexample: encode_neon_faddp([v0.2d, v0.2s, v0.2s])
- Bug report: bug_reports/encode_neon_faddp_mismatch_t.md

```property
function: encoder.encode_neon_faddp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, td: {2s,4s,2d}, tn: {2s,4s,2d}, tm: {2s,4s,2d}, not_all_equal: true }
  relation:
    op: throws
    expr: encode_neon_faddp(&[arr(rd,td), arr(rn,tn), arr(rm,tm)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: element, of: ["2s", "4s", "2d"] }
  tn: { gen: element, of: ["2s", "4s", "2d"] }
  tm: { gen: element, of: ["2s", "4s", "2d"] }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1682
```

## encode_neon_faddp_neg_gpr_bare_scalar_dest
- Tier: 3
- Rationale: Vector FADDP requires arranged V registers; scalar FADDP requires Sd+Vn.2S or Dd+Vn.2D (neon.rs:1684). GPR, SP, bare V, and dest/source size mismatches are rejected by llvm-mc/gas. Doc evidence: neon.rs:1682, neon.rs:1684, README.md:12.
- Doc contract: neon.rs:1684 "Scalar form: FADDP Sd, Vn.2S  or FADDP Dd, Vn.2D" — asserted fingerprint edfd218d
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_neg_gpr_bare_sp
- Formal: ∀ inputs in {GPR triple, W dest + vector, SP dest, bare V triple, D dest + Vn.2S, S dest + Vn.2D, X dest + Vn.2S, S dest + Vn.4S}. llvm-mc rejects asm ⇒ encode_neon_faddp(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: failing
- Counterexample: encode_neon_faddp([Reg("d0"), v0.2s])
- Bug report: bug_reports/encode_neon_faddp_scalar_dest_size.md

```property
function: encoder.encode_neon_faddp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, kind: 0..7 }
  relation:
    op: throws
    expr: encode_neon_faddp(&ops_for(kind, rd, rn, rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 7, type: u8 }
expected_error: String
evidence: src/backend/arm/assembler/encoder/neon.rs:1684
```

## encode_neon_faddp_diff_alt_spellings
- Tier: 2
- Rationale: Sweep property: GNU-style assembly is case-insensitive on V-register prefixes (README.md:12). Uppercase Vd.T must agree with llvm-mc. Same differential oracle as the vector property.
- Doc contract: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_neon_bsl_pbt.rs:encode_neon_bsl_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {2s,4s,2d}. encode_neon_faddp([V{rd}.T, V{rn}.T, V{rm}.T]) = Word(w) ∧ w = llvm-mc("faddp Vrd.T, Vrn.T, Vrm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_faddp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_faddp
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, t: {2s,4s,2d} }
  relation:
    op: eq
    lhs: encode_neon_faddp([arr_upper(rd,t), arr_upper(rn,t), arr_upper(rm,t)])
    rhs: llvm_mc("faddp V{rd}.{t}, V{rn}.{t}, V{rm}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/README.md:12
```
