# Properties: encode_mneg

## encode_mneg_diff_gpr
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (GNU-style assembler README.md:12). State machine rejected (pure function). Round-trip rejected (no in-tree MNEG decoder). encode_msub rejected as independent differential (same-job gate: 4-operand vs 3-operand alias; shared get_reg / same TU).
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: data_processing.rs:8109 encode_msub_diff_gpr / data_processing.rs encode_mul_diff_gpr
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool. encode_mneg([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))]) = llvm-mc("mneg Rd, Rn, Rm")
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_mneg([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))])
    rhs: llvm_mc("mneg Rd, Rn, Rm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: README.md:12 gas-compat; encoder/mod.rs:409 mneg dispatch; llvm-mc 15.0.6
```

## encode_mneg_alias_msub_zr
- Tier: 4
- Rationale: Documented alias (rustdoc "MNEG -> MSUB ... XZR"; ARM ARM MNEG is MSUB with Ra=ZR). Not an independent differential (encode_msub shares get_reg / same TU). Metamorphic: SUT MNEG equals SUT MSUB with ZR and both equal llvm-mc.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: data_processing.rs:8089 encode_msub_kat_llvm_mc_ra_zr_is_mneg
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool. encode_mneg([Rd,Rn,Rm]) = encode_msub([Rd,Rn,Rm,ZR]) = llvm-mc("mneg Rd, Rn, Rm") = llvm-mc("msub Rd, Rn, Rm, ZR")
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_mneg([Rd,Rn,Rm])
    rhs: encode_msub([Rd,Rn,Rm,ZR])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: data_processing.rs:676 rustdoc alias; ARM ARM MNEG = MSUB Ra=ZR
```

## encode_mneg_metamorphic_sf_bit
- Tier: 4
- Rationale: ARM ARM sf is bit 31 of Data-processing (3 source); same Rd/Rn/Rm numbers in X vs W form differ only by sf. Weaker than differential; still an independent layout identity.
- Doc contract: data_processing.rs:682 "MSUB with Ra=XZR: sf 00 11011 000 Rm 1 11111 Rn Rd" — asserted fingerprint cd805a03
- Seed: data_processing.rs:8164 encode_msub_metamorphic_sf_bit
- Formal: ∀ rd, rn, rm ∈ {0..31}. encode_mneg(X-form) XOR encode_mneg(W-form) = 1<<31
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_mneg(Xform) XOR encode_mneg(Wform)
    rhs: 1u32 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (3 source) sf bit 31; data_processing.rs:682
```

## encode_mneg_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout of MNEG/MSUB-ZR is an exact structural invariant independent of llvm-mc: sf, bits[30:21]=0011011000, Rm, o0=1, Ra=31, Rn, Rd.
- Doc contract: data_processing.rs:682 "MSUB with Ra=XZR: sf 00 11011 000 Rm 1 11111 Rn Rd" — asserted fingerprint cd805a03
- Seed: data_processing.rs:8196 encode_msub_invariant_arm_fields
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool. word = encode_mneg([Rd,Rn,Rm]) ⇒ (word>>31)=sf ∧ (word>>21)&0x3FF=0b0011011000 ∧ (word>>16)&0x1F=rm ∧ (word>>15)&1=1 ∧ (word>>10)&0x1F=31 ∧ (word>>5)&0x1F=rn ∧ word&0x1F=rd
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool }
  body: let w = encode_mneg([Rd,Rn,Rm]) in (w>>31)==sf && ((w>>21)&0x3FF)==0b0011011000 && ((w>>16)&0x1F)==rm && ((w>>15)&1)==1 && ((w>>10)&0x1F)==31 && ((w>>5)&0x1F)==rn && (w&0x1F)==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM Data-processing (3 source) MNEG = MSUB Ra=ZR; data_processing.rs:682
```

## encode_mneg_neg_too_few
- Tier: 3
- Rationale: llvm-mc reports "too few operands for instruction" for arity < 3. get_reg on a missing index returns Err. Documented error contract of the GNU-style assembler.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: data_processing.rs:8253 encode_msub_neg_too_few
- Formal: ∀ ops with |ops| ∈ {0,1,2}. encode_mneg(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: operand lists of length 0..2 }
  relation:
    op: throws
    expr: encode_mneg(ops)
generators:
  len: { gen: int, min: 0, max: 2, type: usize }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; get_reg missing index
```

## encode_mneg_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc rejects a 4th operand ("invalid operand for instruction"). README.md:12 gas-compat. Body has no arity upper bound so this is a documented assembler contract the encoder must honour; generator keeps the extra operand.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: data_processing.rs:8274 encode_msub_neg_extra_operand
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool, extra ∈ Operand. encode_mneg([Rd,Rn,Rm,extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: failing
- Counterexample: encode_mneg([Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")]) -> Ok(Word(0x1b00fc00))
- Bug report: pbt-out/bug_reports/encode_mneg_extra_operand.md

```property
function: encode_mneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, extra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool, extra: Operand }
  relation:
    op: throws
    expr: encode_mneg([Rd,Rn,Rm,extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: llvm-mc rejects 4th operand; README.md:12 gas-compat
```

## encode_mneg_neg_mixed_width
- Tier: 3
- Rationale: ARM ARM and llvm-mc require Rd, Rn, Rm the same width (all X or all W). llvm-mc "invalid operand". Body takes sf only from Rd and discards Rn/Rm width; mixed width stays in the generator.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: data_processing.rs:8296 encode_msub_neg_mixed_width
- Formal: ∀ rd, rn, rm ∈ {0..30}, rd64, rn64, rm64 ∈ Bool. ¬(rd64=rn64=rm64) ⇒ encode_mneg([gpr(rd64,rd), gpr(rn64,rn), gpr(rm64,rm)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: failing
- Counterexample: encode_mneg([Reg("w0"), Reg("w0"), Reg("x0")]) -> Ok(Word(0x1b00fc00))
- Bug report: pbt-out/bug_reports/encode_mneg_mixed_width.md

```property
function: encode_mneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, rd64: bool, rn64: bool, rm64: bool }
  relation:
    op: throws
    expr: encode_mneg(mixed-width triple)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: String
evidence: llvm-mc mixed W/X "invalid operand"; ARM ARM same-width GPR
```

## encode_mneg_neg_sp
- Tier: 3
- Rationale: ARM ARM register 31 in Data-processing (3 source) is ZR not SP. llvm-mc rejects sp/wsp as MNEG operands. parse_reg_num maps both to 31; the function does not declare SP invalid, so SP stays in the generator.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: data_processing.rs:8324 encode_msub_neg_sp
- Formal: ∀ which ∈ {0,1,2}, is_64 ∈ Bool, a, b ∈ {0..30}. encode_mneg(triple with slot `which` = sp/wsp) is Err
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: failing
- Counterexample: encode_mneg([Reg("wsp"), Reg("w0"), Reg("w0")]) -> Ok(Word(0x1b00fc1f))
- Bug report: pbt-out/bug_reports/encode_mneg_sp_as_zr.md

```property
function: encode_mneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b]
  domain: { which: 0..2, is_64: bool, a: 0..30, b: 0..30 }
  relation:
    op: throws
    expr: encode_mneg(triple with SP/WSP at slot which)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" for sp/wsp; ARM ARM Ra/Rd/Rn/Rm use ZR not SP
```

## encode_mneg_neg_fp
- Tier: 3
- Rationale: llvm-mc rejects FP/SIMD prefixes (d/s/q/v/h/b) as MNEG operands. parse_reg_num accepts those prefixes; encode_mneg does not call is_fp_reg. Strengthening round after extra/mixed/SP failures.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: encode_smaddl_pbt.rs encode_smaddl_neg_fp
- Formal: ∀ which ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_mneg(triple with slot which = prefix||n) is Err
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: failing
- Counterexample: encode_mneg([Reg("d0"), Reg("w1"), Reg("w2")]) -> Ok(Word(0x1b02fc20))
- Bug report: pbt-out/bug_reports/encode_mneg_fp_as_gpr.md

```property
function: encode_mneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..2, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_mneg(triple with FP register at slot which)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" for d0; is_fp_reg unused by encode_mneg
```

## encode_mneg_neg_nonreg
- Tier: 3
- Rationale: get_reg requires Operand::Reg; Imm/Mem/Shift/Label/Symbol/Cond/RegArrangement must Err. Strengthening / contract-surface.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: encode_smaddl_pbt.rs encode_smaddl_neg_nonreg
- Formal: ∀ which ∈ {0,1,2}, bad ∈ non-Reg Operand. encode_mneg(triple with slot which = bad) is Err
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..2, bad: non-Reg Operand }
  relation:
    op: throws
    expr: encode_mneg(triple with non-register at slot which)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: get_reg returns Err for non-Reg; llvm-mc requires GPR operands
```

## encode_mneg_neg_invalid_name
- Tier: 3
- Rationale: parse_reg_num returns None for x32/foo/empty/r0; get_reg must Err. Strengthening / contract-surface.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: encode_smaddl_pbt.rs encode_smaddl_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2}, name ∈ {foo, x32, w32, x, r0, empty, x-1, x99, w}. encode_mneg(triple with slot which = Reg(name)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..2, name: invalid register names }
  relation:
    op: throws
    expr: encode_mneg(triple with invalid name at slot which)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: parse_reg_num None for x32/foo; llvm-mc rejects unrecognized names
```

## encode_mneg_diff_alt_spellings
- Tier: 5
- Rationale: parse_reg_num lowercases and accepts x31/w31/XZR/LR. Differential vs llvm-mc on those aliases. Strengthening / contract-surface.
- Doc contract: data_processing.rs:676 "Encode MNEG Xd, Xn, Xm -> MSUB Xd, Xn, Xm, XZR" — asserted fingerprint 19f56334
- Seed: encode_smaddl_pbt.rs encode_smaddl_diff_alt_spellings
- Formal: ∀ rd, rn, rm ∈ {0..31}, is_64 ∈ Bool, spellings ∈ {x31, XZR, LR, uppercase}. encode_mneg(spelled) = llvm-mc(spelled)
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_64: bool }
  relation:
    op: eq
    lhs: encode_mneg(alt-spellings)
    rhs: llvm_mc(alt-spellings)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: parse_reg_num to_lowercase; llvm-mc accepts x31/XZR/LR
```

## encode_mneg_meta_rd_rn_rm
- Tier: 4
- Rationale: Rd/Rn/Rm n vs n+1 must differ only in that 5-bit field (ARM layout isolation). Strengthening / contract-surface.
- Doc contract: data_processing.rs:682 "MSUB with Ra=XZR: sf 00 11011 000 Rm 1 11111 Rn Rd" — asserted fingerprint cd805a03
- Seed: encode_smaddl_pbt.rs encode_smaddl_meta_rd_rn_rm_ra
- Formal: ∀ rd, rn, rm ∈ {0..30}, is_64 ∈ Bool. (encode_mneg(rd+1) XOR encode_mneg(rd)) & ~0x1F = 0, and similarly Rn bits[9:5], Rm bits[20:16]
- Test file: src/backend/arm/assembler/encoder/encode_mneg_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_mneg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, is_64: bool }
  relation:
    op: eq
    lhs: (encode_mneg(rd+1) XOR encode_mneg(rd)) AND NOT 0x1F
    rhs: 0
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM Rd bits[4:0] Rn bits[9:5] Rm bits[20:16]
```
