# Properties: encode_smaddl

## encode_smaddl_diff_valid_gpr
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler, same GNU-style text). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SMADDL decoder). encode_umaddl rejected as same-job sibling (U=1 unsigned). encode_smull rejected as independent differential (shared get_reg / same TU). ARM ARM field unpack is a weaker invariant used in encode_smaddl_arm_fields.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: src/backend/arm/assembler/encoder/data_processing.rs encode_umaddl_pbt llvm-mc differential
- Formal: ∀ rd,ra ∈ 0..31, rn,rm ∈ 0..31. encode_smaddl([Xd(rd), Wn(rn), Wm(rm), Xa(ra)]) = llvm-mc("smaddl Xd, Wn, Wm, Xa")
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31 }
  relation:
    op: eq
    lhs: encode_smaddl([Xd(rd), Wn(rn), Wm(rm), Xa(ra)])
    rhs: llvm_mc("smaddl", Xd(rd), Wn(rn), Wm(rm), Xa(ra))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:652 rustdoc; encoder/mod.rs:405 dispatch; llvm-mc -triple=aarch64 -show-encoding
```

## encode_smaddl_alias_smull_xzr
- Tier: 4
- Rationale: ARM ARM and the SUT rustdoc of encode_smull state SMULL Xd, Wn, Wm is the alias of SMADDL Xd, Wn, Wm, XZR. That is a metamorphic equality, not an independent differential (shared get_reg / same TU). llvm-mc is used as a second check that both encodings match the assembler. Round-trip rejected (no decoder).
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_alias_umull_xzr
- Formal: ∀ rd,rn,rm ∈ 0..31. encode_smaddl([Xd(rd), Wn(rn), Wm(rm), XZR]) = encode_smull([Xd(rd), Wn(rn), Wm(rm)]) = llvm-mc("smaddl Xd, Wn, Wm, xzr") = llvm-mc("smull Xd, Wn, Wm")
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_smaddl([Xd(rd), Wn(rn), Wm(rm), XZR])
    rhs: encode_smull([Xd(rd), Wn(rn), Wm(rm)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:630 Encode SMULL Xd, Wn, Wm -> SMADDL Xd, Wn, Wm, XZR; ARM ARM SMULL alias
```

## encode_smaddl_xor_umaddl_u_bit
- Tier: 4
- Rationale: Metamorphic: SMADDL and UMADDL of the same registers differ only in bit 23 (U). encode_umaddl is not a same-job differential (unsigned); the U-bit XOR is the evidenced structural relation from ARM ARM. Round-trip rejected (no decoder).
- Doc contract: data_processing.rs:658 "SMADDL: 1 00 11011 001 Rm 0 Ra Rn Rd" — asserted fingerprint e2a9b663
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_xor_smaddl_u_bit
- Formal: ∀ rd,rn,rm,ra ∈ 0..31. encode_smaddl(ops) XOR encode_umaddl(ops) = 1<<23
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31 }
  relation:
    op: eq
    lhs: encode_smaddl(ops) XOR encode_umaddl(ops)
    rhs: 1<<23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:658 vs 670 U bit (bit 23); ARM ARM Data-processing (3 source)
```

## encode_smaddl_arm_fields
- Tier: 4
- Rationale: ARM ARM SMADDL layout is an exact structural invariant independent of llvm-mc. Weaker than differential; kept as a second oracle so a llvm-mc mapping bug cannot hide a field-layout error. Round-trip rejected (no decoder).
- Doc contract: data_processing.rs:658 "SMADDL: 1 00 11011 001 Rm 0 Ra Rn Rd" — asserted fingerprint e2a9b663
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_arm_fields
- Formal: ∀ rd,rn,rm,ra ∈ 0..31. let w = encode_smaddl([Xd(rd), Wn(rn), Wm(rm), Xa(ra)]). w[31]=1 ∧ w[30:21]=0011011001 ∧ w[20:16]=rm ∧ w[15]=0 ∧ w[14:10]=ra ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31 }
  body: fields(encode_smaddl(valid_ops)) match ARM SMADDL layout for (sf=1, U=0, o0=0, rd, rn, rm, ra)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: data_processing.rs:658 encoding comment; ARM ARM Data-processing (3 source) SMADDL
```

## encode_smaddl_neg_arity
- Tier: 4
- Rationale: llvm-mc / gas reject SMADDL with fewer than 4 operands ("too few operands"). get_reg on a missing slot returns Err, which is the documented assembler contract. Extra-operand case is a separate property because the body has no upper bound.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_neg_arity
- Formal: ∀ ops. |ops| ∈ {0,1,2,3} ⇒ encode_smaddl(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand slices of length 0..3 }
  relation:
    op: throws
    expr: encode_smaddl(ops)
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
expected_error: Err
evidence: llvm-mc too few operands; data_processing.rs:652 four named operands
```

## encode_smaddl_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc / gas reject a 5th SMADDL operand ("invalid operand for instruction"). The rustdoc names four operands. Body has no operands.len() upper bound, so the input stays in the generator.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_neg_extra_operand
- Formal: ∀ rd,rn,rm,ra ∈ 0..31, extra ∈ Operand. encode_smaddl([Xd(rd), Wn(rn), Wm(rm), Xa(ra), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, extra=Reg("x0") → Ok(Word(0x9b200000))
- Bug report: bug_reports/encode_smaddl_extra_operand.md

```property
function: encode_smaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, extra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_smaddl([Xd(rd), Wn(rn), Wm(rm), Xa(ra), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc invalid operand for instruction; data_processing.rs:652 four named operands
```

## encode_smaddl_neg_wrong_width
- Tier: 4
- Rationale: ARM ARM and the rustdoc require Xd, Wn, Wm, Xa. llvm-mc rejects any other W/X mix. get_reg returns width but encode_smaddl discards it, so the input stays in the generator.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_neg_wrong_width
- Formal: ∀ rd,rn,rm,ra ∈ 0..30, rd64,rn64,rm64,ra64 ∈ bool. ¬(rd64 ∧ ¬rn64 ∧ ¬rm64 ∧ ra64) ⇒ encode_smaddl([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm), gpr(ra64,ra)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, rd64=false, rn64=false, rm64=false, ra64=false (w0,w0,w0,w0) → Ok(Word(0x9b200000))
- Bug report: bug_reports/encode_smaddl_wrong_width.md

```property
function: encode_smaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, rd64, rn64, rm64, ra64]
  domain: { rd,rn,rm,ra: 0..30, widths: not the valid X/W/W/X mix }
  relation:
    op: throws
    expr: encode_smaddl(mixed_width_ops)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
  ra64: { gen: bool }
expected_error: Err
evidence: data_processing.rs:652 Xd, Wn, Wm, Xa; llvm-mc invalid operand for instruction
```

## encode_smaddl_neg_sp
- Tier: 4
- Rationale: ARM ARM register 31 in SMADDL is XZR/WZR, not SP/WSP. llvm-mc rejects SP/WSP in any slot. parse_reg_num maps both to 31, so the input stays in the generator.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_neg_sp
- Formal: ∀ which ∈ 0..3, is_64 ∈ bool, a,b ∈ 0..30. encode_smaddl(valid_ops with slot which replaced by SP if is_64 else WSP) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: failing
- Counterexample: which=0, is_64=false, a=0, b=0 (wsp, w0, w0, x0) → Ok(Word(0x9b20001f))
- Bug report: bug_reports/encode_smaddl_sp_as_zr.md

```property
function: encode_smaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b]
  domain: { which: 0..3, is_64: bool, a,b: 0..30 }
  relation:
    op: throws
    expr: encode_smaddl(ops_with_sp_in_slot)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: Err
evidence: ARM ARM Rd/Ra are XZR not SP; llvm-mc invalid operand for instruction; data_processing.rs:652
```

## encode_smaddl_diff_alt_spellings
- Tier: 5
- Rationale: Sweep: x31/XZR/LR/uppercase aliases that llvm-mc accepts must match. Differential vs llvm-mc. Strengthens the valid-domain generator toward spelling edges.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_diff_alt_spellings
- Formal: ∀ rd,rn,rm,ra ∈ 0..31 and alt spellings in {x31, XZR, LR, uppercase}. encode_smaddl(ops) = llvm-mc(asm)
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, dest_spell, src_spell, acc_spell]
  domain: { registers 0..31, alt spellings x31/XZR/LR/uppercase }
  relation:
    op: eq
    lhs: encode_smaddl(ops)
    rhs: llvm_mc(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:267 parse_reg_num xzr/wzr/lr/casefold; llvm-mc -triple=aarch64
```

## encode_smaddl_neg_fp
- Tier: 4
- Rationale: Sweep: llvm-mc rejects FP/SIMD prefixes (d/s/q/v/h/b) as SMADDL operands. parse_reg_num accepts those prefixes, so the input stays in the generator.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_neg_fp
- Formal: ∀ which ∈ 0..3, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_smaddl(valid_ops with slot which = prefix∥n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 (d0, w1, w2, x3) → Ok(Word(0x9b220c20))
- Bug report: bug_reports/encode_smaddl_fp_as_gpr.md

```property
function: encode_smaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..3, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_smaddl(ops_with_fp_in_slot)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc invalid operand for instruction; data_processing.rs:652 Xd/Wn/Wm/Xa are GPRs
```

## encode_smaddl_neg_nonreg
- Tier: 4
- Rationale: Sweep: non-register operand kinds (Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement) must Err. get_reg already rejects non-Reg.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad ∈ non-Reg Operand. encode_smaddl(valid_ops with slot which = bad) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: non-Reg Operand }
  relation:
    op: throws
    expr: encode_smaddl(ops_with_nonreg)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: Err
evidence: encoder/mod.rs:1092 get_reg expected register; data_processing.rs:654-657
```

## encode_smaddl_neg_invalid_name
- Tier: 4
- Rationale: Sweep: unparsable names (foo, x32, empty, r0) must Err. parse_reg_num returns None.
- Doc contract: data_processing.rs:652 "Encode SMADDL Xd, Wn, Wm, Xa (signed multiply-add long)" — asserted fingerprint db6834ac
- Seed: data_processing.rs encode_umaddl_pbt encode_umaddl_neg_invalid_name
- Formal: ∀ which ∈ 0..3, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_smaddl(valid_ops with slot which = Reg(name)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..3, name: unparsable register names }
  relation:
    op: throws
    expr: encode_smaddl(ops_with_bad_name)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: Err
evidence: encoder/mod.rs:267 parse_reg_num; encoder/mod.rs:1092 invalid register
```

## encode_smaddl_meta_rd_rn_rm_ra
- Tier: 4
- Rationale: Strengthening / sweep: incrementing Rd/Rn/Rm/Ra by 1 updates only that 5-bit field. Independent of llvm-mc. Round-trip rejected (no decoder).
- Doc contract: data_processing.rs:658 "SMADDL: 1 00 11011 001 Rm 0 Ra Rn Rd" — asserted fingerprint e2a9b663
- Seed: encode_crc32_pbt encode_crc32_meta_rd_rn_rm
- Formal: ∀ rd,rn,rm,ra ∈ 0..30. encode_smaddl(..., rd+1, ...) differs from base only in bits[4:0]; rn+1 only in bits[9:5]; rm+1 only in bits[20:16]; ra+1 only in bits[14:10]
- Test file: src/backend/arm/assembler/encoder/encode_smaddl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_smaddl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, ra: 0..30 }
  body: mutating one of rd/rn/rm/ra by +1 flips only that 5-bit field
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
evidence: data_processing.rs:658 Rd/Rn/Rm/Ra field positions
```
