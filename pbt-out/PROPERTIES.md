# Properties: encode_bfi

## encode_bfi_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree BFI decoder). encode_bfm rejected as differential sibling (raw immr/imms form, different assembly syntax). encode_ubfiz/encode_sbfiz rejected (UBFM/SBFM opc). Doc evidence: assembler README.md:11 (same textual assembly as gas); README.md:216 lists bfi; encoder/mod.rs:892 dispatch; ARM ARM BFI alias of BFM.
- Seed: data_processing.rs encode_smulh_pbt llvm-mc differential
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, lsb ∈ 0..R-1, width ∈ 1..R-lsb (R=64 if is_64 else 32). encode_bfi([Reg(gpr), Reg(gpr), Imm(lsb), Imm(width)]) = Word(w) ∧ w = llvm-mc("bfi Rd, Rn, #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb, R: 32|64 }
  relation:
    op: eq
    lhs: encode_bfi([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])
    rhs: llvm_mc("bfi Rd, Rn, #lsb, #width")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; README.md:216; encoder/mod.rs:892; ARM ARM BFI alias
```

## encode_bfi_alias_bfm
- Tier: 4c
- Rationale: ARM ARM and bitfield.rs:103 purpose comment define BFI as the BFM alias with immr=(-lsb MOD R), imms=width-1. Not differential (encode_bfm is in-tree, different syntax). Stronger round-trip rejected (no decoder).
- Seed: (none) — ARM ARM BFI <=> BFM alias
- Formal: ∀ valid (is_64, rd, rn, lsb, width). encode_bfi([Rd,Rn,#lsb,#width]) = encode_bfm([Rd,Rn, #((-lsb) mod R), #(width-1)])
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { valid BFI W/X domain }
  relation:
    op: eq
    lhs: encode_bfi([Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)])
    rhs: encode_bfm([Reg(Rd), Reg(Rn), Imm((-lsb) mod R), Imm(width-1)])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: bitfield.rs:103 purpose comment; ARM ARM BFI alias of BFM
```

## encode_bfi_arm_fields
- Tier: 4d
- Rationale: ARM ARM Bitfield encoding sf 01 100110 N immr imms Rn Rd with N=sf, opc=01, immr=(-lsb MOD R), imms=width-1. Weaker than differential (already used). Invariant is the architectural field layout, not the producing statement.
- Seed: data_processing.rs encode_smulh_arm_fields
- Formal: ∀ valid (is_64, rd, rn, lsb, width). let w = encode_bfi(...).Word. w[31]=sf ∧ w[30:29]=01 ∧ w[28:23]=100110 ∧ w[22]=sf ∧ w[21:16]=(-lsb mod R) ∧ w[15:10]=width-1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { valid BFI W/X domain }
  relation:
    op: holds
    expr: arm_bfm_fields(encode_bfi(ops))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM Bitfield BFM/BFI encoding; assembler README.md:11
```

## encode_bfi_metamorphic_rd_rn
- Tier: 4c
- Rationale: ARM encoding places Rd in bits[4:0] and Rn in bits[9:5]; incrementing one register number must change only that field. Weaker than differential.
- Seed: data_processing.rs encode_smulh_metamorphic_fields
- Formal: ∀ valid BFI with rd,rn ∈ 0..30. encode_bfi(rd+1,...) xor encode_bfi(rd,...) has only bits[4:0] changed to rd+1; encode_bfi(...,rn+1,...) xor base has only bits[9:5] changed to rn+1
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { rd,rn in 0..30, valid lsb/width }
  relation:
    op: holds
    expr: field_independence(encode_bfi, rd, rn)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM Bitfield Rd bits[4:0] Rn bits[9:5]
```

## encode_bfi_neg_arity
- Tier: 5
- Rationale: llvm-mc and gas require four operands (Rd, Rn, #lsb, #width). Fewer must Err. get_reg/get_imm on missing index is the documented helper error path.
- Seed: encode_smulh_neg_arity
- Formal: ∀ ops with |ops| < 4 and slots filled from valid-looking Reg/Imm. encode_bfi(ops) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..3 }
  relation:
    op: throws
    expr: encode_bfi(ops_of_len(len))
expected_error: String
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
evidence: llvm-mc rejects `bfi w0, w1`; ARM ARM BFI has four operands
```

## encode_bfi_neg_extra_operand
- Tier: 5
- Rationale: llvm-mc rejects a fifth operand (`unrecognized instruction mnemonic`). GNU-style assembler contract: extra operand must Err, not silently ignore.
- Seed: encode_smulh_neg_extra_operand
- Formal: ∀ valid 4-operand BFI plus extra ∈ {Reg, Imm, Shift, RegArrangement}. encode_bfi(ops++[extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: bfi w0, w0, #0, #1, x0 (extra=Reg("x0"))
- Bug report: pbt-out/bug_reports/encode_bfi_extra_operand.md

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, extra]
  domain: { valid BFI plus extra operand }
  relation:
    op: throws
    expr: encode_bfi(ops ++ [extra])
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, options: [Reg, Imm, Shift] }
evidence: llvm-mc error on `bfi w0, w1, #0, #1, x2`
```

## encode_bfi_neg_sp
- Tier: 5
- Rationale: ARM ARM bitfield register 31 is ZR not SP. llvm-mc rejects SP/WSP in either Rd or Rn. GNU-style assembler must Err.
- Seed: encode_smulh_neg_sp
- Formal: ∀ which ∈ {Rd,Rn}, sp ∈ {sp,wsp}, other a valid GPR. encode_bfi with SP/WSP in that slot = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: bfi wsp, w0, #0, #1 (which=0, sp=wsp)
- Bug report: pbt-out/bug_reports/encode_bfi_sp.md

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, sp64]
  domain: { which: Rd|Rn, sp: sp|wsp }
  relation:
    op: throws
    expr: encode_bfi(ops_with_sp)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  is_64: { gen: bool }
evidence: llvm-mc `invalid operand` on `bfi sp, x0, #0, #1`; ARM ARM Rn/Rd are W/X not SP
```

## encode_bfi_neg_lsb_width
- Tier: 5
- Rationale: ARM ARM and llvm-mc require 0 <= lsb < R and 1 <= width <= R-lsb. Documented bounds must be rejected at bound+1 and width=0. llvm-mc: expected integer in range [1, 32] for width=0; invalid lsb=32 / width=33.
- Seed: (none) — ARM ARM BFI constraints; llvm-mc range errors
- Formal: ∀ is_64, rd, rn, and (lsb,width) in {width=0, lsb=R, width=R-lsb+1 when lsb<R, lsb=-1, width=-1}. encode_bfi = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: bfi w0, w0, #0, #0 (lsb=0, width=0) — debug overflow panic at width-1
- Bug report: pbt-out/bug_reports/encode_bfi_lsb_width.md

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { out-of-range lsb/width including bound+1 and 0 }
  relation:
    op: throws
    expr: encode_bfi([Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)])
expected_error: String
generators:
  is_64: { gen: bool }
  lsb: { gen: int, min: -2, max: 65, type: i64 }
  width: { gen: int, min: -2, max: 65, type: i64 }
evidence: ARM ARM BFI 0<=lsb<R, 1<=width<=R-lsb; llvm-mc range errors
```

## encode_bfi_diff_alt_spellings
- Tier: 2
- Rationale: Strengthening of the llvm-mc differential: uppercase W/X, x31/w31 aliases of ZR, and LR must match llvm-mc. Same evidence as encode_bfi_diff_valid_gpr.
- Seed: encode_smulh_diff_alt_spellings
- Formal: ∀ valid BFI with dest/src spelled as uppercase, x31/w31, XZR/WZR, or LR. encode_bfi = llvm-mc(same spelling)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, dest_spell, src_spell]
  domain: { valid BFI plus alt spellings }
  relation:
    op: eq
    lhs: encode_bfi([Reg(dest_spell), Reg(src_spell), Imm(lsb), Imm(width)])
    rhs: llvm_mc("bfi dest, src, #lsb, #width")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: assembler README.md:11; llvm-mc accepts W0/X31/LR/XZR
```

## encode_bfi_neg_mixed_width
- Tier: 5
- Rationale: ARM ARM BFI requires Wd,Wn or Xd,Xn of the same size. llvm-mc rejects mixed W/X. Strengthening error-path.
- Seed: encode_smulh_neg_wrong_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64, valid lsb/width for dest size. encode_bfi = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: bfi x0, w0, #0, #1 (rd64=true, rn64=false, rd=0, rn=0)
- Bug report: pbt-out/bug_reports/encode_bfi_mixed_width.md

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd64, rn64, rd, rn, lsb, width]
  domain: { rd64 != rn64 }
  relation:
    op: throws
    expr: encode_bfi([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(lsb), Imm(width)])
expected_error: String
generators:
  rd64: { gen: bool }
  rn64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc invalid operand on `bfi w0, x1, #0, #1`; ARM ARM same-size W/X
```

## encode_bfi_neg_fp
- Tier: 5
- Rationale: BFI is a GPR bitfield insert. llvm-mc rejects S/D/Q/V/H/B registers. Strengthening error-path.
- Seed: encode_smulh_neg_fp
- Formal: ∀ which ∈ {Rd,Rn}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_bfi with that FP name = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: bfi d0, x1, #0, #1 (which=0, prefix=d, n=0)
- Bug report: pbt-out/bug_reports/encode_bfi_fp.md

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: Rd|Rn, prefix: d|s|q|v|h|b, n: 0..31 }
  relation:
    op: throws
    expr: encode_bfi(ops_with_fp)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc invalid operand on `bfi s0, s1, #0, #1`
```

## encode_bfi_neg_nonreg
- Tier: 5
- Rationale: Rd/Rn must be registers and lsb/width must be immediates. Non-register / non-imm kinds must Err via get_reg/get_imm.
- Seed: encode_smulh_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad a non-matching Operand kind. encode_bfi = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: Imm|Shift|Mem|Label|Symbol|Cond|RegArrangement }
  relation:
    op: throws
    expr: encode_bfi(ops_with_bad_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: get_reg/get_imm error paths; llvm-mc operand kind errors
```

## encode_bfi_neg_invalid_name
- Tier: 5
- Rationale: Sweep — parse_reg_num returns None for foo/x32/empty/r0; llvm-mc rejects them. Documented error path not yet property-tested.
- Seed: encode_smulh_neg_invalid_name
- Formal: ∀ which ∈ {Rd,Rn}, name ∈ {foo, x32, w32, x, r0, empty, x-1, x99, w}. encode_bfi with Reg(name) at that slot = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: Rd|Rn, name: invalid GPR spelling }
  relation:
    op: throws
    expr: encode_bfi(ops_with_invalid_name)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: parse_reg_num returns None for these names; llvm-mc invalid operand
```
