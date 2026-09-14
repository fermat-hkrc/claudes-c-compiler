# Properties: encode_smulh

## encode_smulh_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function). Round-trip rejected (no in-tree SMULH decoder). encode_umulh rejected as same-job sibling (unsigned vs signed). Doc evidence: README.md:11 GNU-style assembly; README.md:214 lists smulh; encoder/mod.rs:275 dispatch; ARM ARM Data-processing (3 source) SMULH Xd,Xn,Xm.
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_diff_valid_gpr
- Formal: ∀ rd,rn,rm ∈ {0..31}, spell ∈ {xN, xzR-if-31, lr-if-30}. encode_smulh([Reg(spell(rd)), Reg(spell(rn)), Reg(spell(rm))]) = Word(w) ∧ llvm-mc("smulh "+asm) = w
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smulh([Reg(xreg(rd)), Reg(xreg(rn)), Reg(xreg(rm))])
    rhs: llvm_mc_word("smulh " + xreg(rd) + ", " + xreg(rn) + ", " + xreg(rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:11 GNU-style assembly; encoder/mod.rs:275 "smulh" => encode_smulh; ARM ARM SMULH Xd,Xn,Xm
```

## encode_smulh_xor_umulh_u_bit
- Tier: 4c
- Rationale: Algebraic metamorphic. SMULH and UMULH share the Data-processing (3 source) layout and differ only in op31 bit U (bit 23). Not a same-job differential (signed vs unsigned). Round-trip rejected. Doc evidence: ARM ARM op31=010 (SMULH) vs op31=110 (UMULH); data_processing.rs:692/701 purpose comments.
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_xor_smulh_u_bit
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_smulh(ops) XOR encode_umulh(ops) = 1<<23 where ops=[xrd,xrn,xrm]
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smulh(ops) XOR encode_umulh(ops)
    rhs: 1u32 << 23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) SMULH op31=010 vs UMULH op31=110 (bit 23)
```

## encode_smulh_arm_fields
- Tier: 4d
- Rationale: Algebraic invariant from ARM ARM field layout. Stronger differential already covered by encode_smulh_diff_valid_gpr. This pins the documented bitfields independently of llvm-mc. Doc evidence: data_processing.rs:701 purpose comment; ARM ARM sf=1 op54=00 11011 op31=010 o0=0 Ra=11111.
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}. encode_smulh([xrd,xrn,xrm]) = 0x9B407C00 | (rm<<16) | (rn<<5) | rd ∧ sf=1 ∧ bits[30:21]=0b0011011010 ∧ o0=0 ∧ Ra=31
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smulh([Reg(xreg(rd)), Reg(xreg(rn)), Reg(xreg(rm))])
    rhs: 0x9B407C00 | (rm << 16) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) SMULH; data_processing.rs:701
```

## encode_smulh_neg_arity
- Tier: 4e
- Rationale: Negative/error contract. SMULH requires exactly three register operands. llvm-mc rejects too few operands. Doc evidence: ARM ARM SMULH Xd, Xn, Xm; README.md:11 gas-compatible assembly.
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_neg_arity
- Formal: ∀ ops with |ops| ∈ {0,1,2} and each element a valid X register. encode_smulh(ops) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..2 }
  relation:
    op: throws
    expr: encode_smulh(ops_of_len(len))
generators:
  len: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: ARM ARM SMULH Xd, Xn, Xm (exactly three registers); llvm-mc too few operands
```

## encode_smulh_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract. A fourth operand is invalid; llvm-mc rejects it. encode_smulh only reads indices 0..2 so extra operands are a documented-contract risk. Doc evidence: ARM ARM three-operand SMULH; README.md:11.
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_neg_extra_operand
- Formal: ∀ rd,rn,rm ∈ {0..31}, extra ∈ ExtraOperand. encode_smulh([xrd,xrn,xrm,extra]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, extra=Reg("x0") — serial reconfirm PBT_TEST_JOBS=1
- Bug report: pbt-out/bug_reports/encode_smulh_extra_operand.md

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, extra: ExtraOperand }
  relation:
    op: throws
    expr: encode_smulh([Reg(xreg(rd)), Reg(xreg(rn)), Reg(xreg(rm)), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, options: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: ARM ARM SMULH Xd, Xn, Xm (no 4th operand); llvm-mc rejects extra operand
```

## encode_smulh_neg_wrong_width
- Tier: 4e
- Rationale: Negative/error contract. SMULH has no 32-bit form; all three operands must be 64-bit X registers. llvm-mc rejects W operands. Documented bound: 64-bit only. Doc evidence: ARM ARM SMULH Xd,Xn,Xm; llvm-mc "invalid operand".
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_neg_wrong_width
- Formal: ∀ rd,rn,rm ∈ {0..30}, (rd64,rn64,rm64) ≠ (true,true,true). encode_smulh([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=false — smulh w0, w0, w0; serial reconfirm PBT_TEST_JOBS=1
- Bug report: pbt-out/bug_reports/encode_smulh_wrong_width.md

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30 }
  relation:
    op: throws
    expr: encode_smulh([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: String
evidence: ARM ARM SMULH Xd, Xn, Xm (64-bit only); llvm-mc rejects W registers
```

## encode_smulh_neg_sp
- Tier: 4e
- Rationale: Negative/error contract. Register 31 in SMULH is XZR, not SP/WSP. llvm-mc rejects SP/WSP. Doc evidence: ARM ARM SMULH uses Xd|XZR not SP; llvm-mc "invalid operand".
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_neg_sp
- Formal: ∀ which ∈ {0,1,2}, is_64 ∈ Bool, a,b ∈ {0..30}. encode_smulh(ops with slot which = SP/WSP) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, a=0, b=0 — smulh wsp, x0, x0; serial reconfirm PBT_TEST_JOBS=1
- Bug report: pbt-out/bug_reports/encode_smulh_sp_as_zr.md

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b]
  domain: { which: 0..2, a: 0..30, b: 0..30 }
  relation:
    op: throws
    expr: encode_smulh(ops_with_sp_at(which))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM SMULH Xd/Xn/Xm use XZR not SP; llvm-mc rejects sp/wsp
```

## encode_smulh_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc over documented alternate spellings of the same registers (x31=XZR, uppercase, LR=x30). Documented bounds sampled: Rd/Rn/Rm = 0, 30, 31. Doc evidence: parse_reg_num maps lr/xzr/x31; README.md:11 case-insensitive GNU assembly.
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_diff_alt_spellings
- Formal: ∀ rd,rn,rm ∈ {0..31}, dest/src spellings ∈ {xN, x31-if-31, XZR-if-31, LR-if-30, UPPER}. encode_smulh(ops) = llvm-mc(asm)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, dest_spell, src_n_spell, src_m_spell]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, dest_spell: 0..4, src_n_spell: 0..4, src_m_spell: 0..4 }
  relation:
    op: eq
    lhs: encode_smulh([Reg(dest), Reg(src_n), Reg(src_m)])
    rhs: llvm_mc_word("smulh " + dest + ", " + src_n + ", " + src_m)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_n_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_m_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style assembly; encoder/mod.rs:131 parse_reg_num lr/xzr/x31
```

## encode_smulh_metamorphic_fields
- Tier: 4c
- Rationale: Algebraic metamorphic. Incrementing Rd/Rn/Rm by 1 (staying in 0..30) must flip only that field's LSB. Strengthens the ARM field invariant independently of llvm-mc. Doc evidence: ARM ARM SMULH Rd bits[4:0], Rn bits[9:5], Rm bits[20:16].
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..30}. let w=encode_smulh(rd,rn,rm). encode_smulh(rd+1,rn,rm) differs from w only in Rd=rd+1; encode_smulh(rd,rn+1,rm) differs only in Rn=rn+1; encode_smulh(rd,rn,rm+1) differs only in Rm=rm+1
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30 }
  relation:
    op: holds
    expr: field_independent(encode_smulh(rd,rn,rm), encode_smulh(rd+1,rn,rm), encode_smulh(rd,rn+1,rm), encode_smulh(rd,rn,rm+1))
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM SMULH Rd[4:0] Rn[9:5] Rm[20:16]
```

## encode_smulh_neg_fp
- Tier: 4e
- Rationale: Negative/error contract. FP/SIMD registers (d/s/q/v/h/b) are not SMULH operands. llvm-mc rejects them. parse_reg_num accepts those prefixes so this is a documented-contract risk. Doc evidence: ARM ARM SMULH Xd,Xn,Xm; llvm-mc "invalid operand".
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_neg_fp
- Formal: ∀ which ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_smulh(ops with slot which = prefix+n) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — smulh d0, x1, x2; serial reconfirm PBT_TEST_JOBS=1
- Bug report: pbt-out/bug_reports/encode_smulh_fp_as_gpr.md

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..2, n: 0..31 }
  relation:
    op: throws
    expr: encode_smulh(ops_with_fp_at(which, prefix, n))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM SMULH Xd,Xn,Xm (GPR only); llvm-mc rejects d/s/q/v/h/b
```

## encode_smulh_neg_nonreg
- Tier: 4e
- Rationale: Negative/error contract. Non-register operands (Imm, Mem, Label, Symbol, Cond, Shift, RegArrangement) must be rejected at any slot. Doc evidence: ARM ARM SMULH three GPRs; get_reg error "expected register".
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_neg_nonreg
- Formal: ∀ which ∈ {0,1,2}, bad ∈ NonRegOperand. encode_smulh(ops with slot which = bad) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..2 }
  relation:
    op: throws
    expr: encode_smulh(ops_with_nonreg_at(which, bad))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: ARM ARM SMULH Xd,Xn,Xm; get_reg expected register
```

## encode_smulh_neg_invalid_name
- Tier: 4e
- Rationale: Negative/error contract. Invalid register names (foo, x32, empty, r0, ...) must Err. Documented bound: GPR numbers 0..31. Doc evidence: parse_reg_num returns None outside x/w 0..31 and aliases; llvm-mc rejects unknown names.
- Seed: data_processing.rs encode_umulh_pbt::encode_umulh_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2}, name ∈ InvalidName. encode_smulh(ops with slot which = Reg(name)) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..2 }
  relation:
    op: throws
    expr: encode_smulh(ops_with_name_at(which, name))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: encoder/mod.rs:131 parse_reg_num; ARM ARM GPR 0..31
```

## encode_smulh_neg_wzr
- Tier: 4e
- Rationale: Contract-surface sweep. encode_smulh_neg_wrong_width samples rd/rn/rm in 0..30 so it never hits WZR (W form of register 31). Documented bound: SMULH is 64-bit only; register 31 is XZR not WZR. llvm-mc rejects wzr. Doc evidence: ARM ARM SMULH Xd|XZR; llvm-mc invalid operand.
- Seed: data_processing.rs encode_smulh_pbt::encode_smulh_neg_wrong_width
- Formal: ∀ which ∈ {0,1,2}, a,b ∈ {0..30}. encode_smulh(ops with slot which = wzr and other slots Xa/Xb) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, a=0, b=0 — smulh wzr, x0, x0; serial reconfirm PBT_TEST_JOBS=1
- Bug report: pbt-out/bug_reports/encode_smulh_wzr.md

```property
function: encode_smulh
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, a, b]
  domain: { which: 0..2, a: 0..30, b: 0..30 }
  relation:
    op: throws
    expr: encode_smulh(ops_with_wzr_at(which))
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM SMULH 64-bit only, register 31 is XZR not WZR; llvm-mc rejects wzr
```
