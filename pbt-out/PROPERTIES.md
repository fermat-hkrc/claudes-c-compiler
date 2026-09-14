# Property ledger: encode_sbfiz

## encode_sbfiz_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:11 gas-compatible assembler; encoder/mod.rs encodes AArch64 32-bit words). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SBFIZ decoder). encode_sbfm same-job gate fails (raw immr/imms vs alias lsb/width).
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, lsb ∈ 0..R-1, width ∈ 1..R-lsb (R=64 if is_64 else 32). encode_sbfiz([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)]) = Word(w) ∧ w = llvm-mc("sbfiz gpr(rd), gpr(rn), #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: encode_sbfiz([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])
    rhs: llvm_mc_word("sbfiz gpr(rd), gpr(rn), #lsb, #width")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:890; ARM ARM SBFIZ alias of SBFM
```

## encode_sbfiz_alias_sbfm
- Tier: 4c
- Rationale: Purpose comment bitfield.rs:60 states SBFIZ is an alias for SBFM Rd, Rn, #(-lsb MOD regsize), #(width-1). Not a same-job differential (different operand jobs); algebraic metamorphic after the ARM mapping.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_alias_bfm
- Formal: ∀ valid (is_64,rd,rn,lsb,width). encode_sbfiz(Rd,Rn,#lsb,#width) = encode_sbfm(Rd,Rn,#(-lsb rem_euclid R),#(width-1))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: encode_sbfiz([Reg(gpr(rd)), Reg(gpr(rn)), Imm(lsb), Imm(width)])
    rhs: encode_sbfm([Reg(gpr(rd)), Reg(gpr(rn)), Imm((-lsb).rem_euclid(R)), Imm(width-1)])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: src/backend/arm/assembler/encoder/bitfield.rs:60 purpose comment; ARM ARM SBFIZ alias of SBFM
```

## encode_sbfiz_arm_fields
- Tier: 4d
- Rationale: ARM ARM Bitfield Move SBFM field layout is the encoding contract claimed by the assembler (encoder/mod.rs:3). Weaker than differential; still pins each field independently so a matching llvm-mc word cannot hide a swapped field that happens to agree on a sparse sample.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_arm_fields
- Formal: ∀ valid (is_64,rd,rn,lsb,width). let w = encode_sbfiz(...). w[31]=sf ∧ w[30:29]=00 ∧ w[28:23]=100110 ∧ w[22]=sf ∧ w[21:16]=(-lsb MOD R) ∧ w[15:10]=width-1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: encode_sbfiz([Reg(gpr(rd)), Reg(gpr(rn)), Imm(lsb), Imm(width)])
    rhs: (sf<<31)|(0b100110<<23)|(sf<<22)|(((-lsb).rem_euclid(R))<<16)|((width-1)<<10)|(rn<<5)|rd
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM SBFM encoding sf 00 100110 N immr imms Rn Rd; bitfield.rs:60
```

## encode_sbfiz_metamorphic_rd_rn
- Tier: 4c
- Rationale: ARM field layout places Rd in bits[4:0] and Rn in bits[9:5]; incrementing one register must change only that field (metamorphic independence).
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_metamorphic_rd_rn
- Formal: ∀ is_64, rd,rn ∈ 0..30, lsb∈{0,1}, width=1. let b=encode_sbfiz(rd,rn,...); let d=encode_sbfiz(rd+1,rn,...); let n=encode_sbfiz(rd,rn+1,...). (d & 0x1f = rd+1) ∧ (d & !0x1f = b & !0x1f) ∧ ((n>>5)&0x1f = rn+1) ∧ (n & !(0x1f<<5) = b & !(0x1f<<5))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb]
  domain: { is_64: bool, rd: 0..30, rn: 0..30, lsb: {0,1}, width: 1 }
  relation:
    op: holds
    expr: "(encode_sbfiz(rd+1,rn) & 0x1f == rd+1) && (encode_sbfiz(rd+1,rn) & !0x1f == encode_sbfiz(rd,rn) & !0x1f) && ((encode_sbfiz(rd,rn+1)>>5)&0x1f == rn+1) && (encode_sbfiz(rd,rn+1) & !(0x1f<<5) == encode_sbfiz(rd,rn) & !(0x1f<<5))"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  lsb: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM SBFM Rd bits[4:0] Rn bits[9:5]
```

## encode_sbfiz_neg_arity
- Tier: 4e
- Rationale: llvm-mc rejects `sbfiz` with fewer than 4 operands ("unrecognized instruction mnemonic"). README.md:11 gas-compatible. get_reg/get_imm fail on missing slots; contract is Err not Word.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_arity
- Formal: ∀ len ∈ 0..3, valid prefix registers. encode_sbfiz(ops truncated to len) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: 0..3, is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: throws
    expr: encode_sbfiz(ops4.truncate(len))
expected_error: String
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects too-few-operand sbfiz; README.md:11
```

## encode_sbfiz_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects `sbfiz w0, w1, #0, #1, x0` ("unrecognized instruction mnemonic"). GNU-style SBFIZ has exactly four operands. Extra operand must Err, not be silently ignored.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_extra_operand
- Formal: ∀ valid 4-operand SBFIZ, extra ∈ Operand. encode_sbfiz(ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, lsb=0, width=1, extra=Reg("x0") — sbfiz w0, w0, #0, #1, x0 encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_sbfiz_extra_operand.md

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, extra]
  domain: { valid SBFIZ 4-tuple, extra: Operand }
  relation:
    op: throws
    expr: encode_sbfiz(ops4 ++ [extra])
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, options: [Reg, Imm, Shift, RegArrangement] }
evidence: llvm-mc extra-operand rejection; README.md:11
```

## encode_sbfiz_neg_sp
- Tier: 4e
- Rationale: ARM ARM SBFIZ uses GPR/ZR; register 31 is ZR not SP. llvm-mc rejects `sbfiz wsp, w0, #0, #1` and `sbfiz w0, wsp, #0, #1`. parse_reg_num maps sp/wsp to 31, which would silently encode ZR.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_sbfiz with ops[which]=Reg(sp) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0 — sbfiz wsp, w0, #0, #1 encodes as wzr instead of Err
- Bug report: pbt-out/bug_reports/encode_sbfiz_sp.md

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: 0..1, sp64: bool, is_64: bool, other: 0..30 }
  relation:
    op: throws
    expr: encode_sbfiz(ops_with_sp_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
evidence: llvm-mc "invalid operand for instruction" on SP/WSP; ARM ARM register 31 is ZR
```

## encode_sbfiz_neg_lsb_width
- Tier: 4e
- Rationale: ARM/llvm-mc require 0<=lsb<R and 1<=width<=R-lsb. llvm-mc: width=0 → "expected integer in range [1, 32]"; lsb=32 → "expected integer in range [0, 31]". Out-of-range must Err, not panic or encode an illegal immr/imms. Documented bounds sampled at 0, R-1, R, R+1, width=0/-1, width=R-lsb+1.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_lsb_width
- Formal: ∀ is_64, rd,rn, (lsb,width) ∉ valid ARM range. encode_sbfiz(...) ∈ {Err} (no panic)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, lsb=0, width=0 — sbfiz w0, w0, #0, #0 panics (debug overflow at width-1) rather than Err
- Bug report: pbt-out/bug_reports/encode_sbfiz_lsb_width.md

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { (lsb,width) outside 0<=lsb<R and 1<=width<=R-lsb }
  relation:
    op: throws
    expr: encode_sbfiz([Reg(gpr(rd)), Reg(gpr(rn)), Imm(lsb), Imm(width))
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: -1, max: 65, type: i64 }
  width: { gen: int, min: -1, max: 65, type: i64 }
evidence: llvm-mc range errors; ARM ARM SBFIZ 0<=lsb<datasize, 1<=width<=datasize-lsb
```

## encode_sbfiz_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for x31/w31, XZR/WZR, LR, and uppercase spellings of the same GPR. README.md:11 gas-compatible; parse_reg_num lowercases names.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_diff_alt_spellings
- Formal: ∀ valid (is_64,rd,rn,lsb,width), dest_spell,src_spell ∈ 0..4. encode_sbfiz([Reg(spell(rd)), Reg(spell(rn)), Imm(lsb), Imm(width)]) = llvm-mc("sbfiz spell(rd), spell(rn), #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, dest_spell, src_spell]
  domain: { valid SBFIZ 4-tuple, dest_spell: 0..4, src_spell: 0..4 }
  relation:
    op: eq
    lhs: encode_sbfiz([Reg(spell(rd)), Reg(spell(rn)), Imm(lsb), Imm(width)])
    rhs: llvm_mc_word("sbfiz spell(rd), spell(rn), #lsb, #width")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11; parse_reg_num lowercases; llvm-mc accepts x31/XZR/LR
```

## encode_sbfiz_neg_mixed_width
- Tier: 4e
- Rationale: llvm-mc rejects mixed W/X (`sbfiz x0, w0, #0, #1` invalid operand). ARM SBFIZ requires matching datasize. encode_sbfiz takes sf from Rd and discards Rn width.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_sbfiz([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — sbfiz x0, w0, #0, #1 encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_sbfiz_mixed_width.md

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd: 0..31, rn: 0..31, rd64 != rn64 }
  relation:
    op: throws
    expr: encode_sbfiz([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
evidence: llvm-mc mixed W/X rejection; ARM ARM matching datasize
```

## encode_sbfiz_neg_fp
- Tier: 4e
- Rationale: llvm-mc rejects FP/SIMD registers as SBFIZ operands (`sbfiz d0, x1, #0, #1`). parse_reg_num accepts d/s/q/v/h/b prefixes as GPR numbers.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_sbfiz with ops[which]=Reg(prefix n) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — sbfiz d0, x1, #0, #1 encodes as w0 instead of Err
- Bug report: pbt-out/bug_reports/encode_sbfiz_fp.md

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_sbfiz(ops_with_fp_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc "invalid operand for instruction" on FP/SIMD; ARM ARM GPR-only SBFIZ
```

## encode_sbfiz_neg_nonreg
- Tier: 4e
- Rationale: get_reg/get_imm require Reg at slots 0/1 and Imm at slots 2/3. Wrong Operand kind must Err. llvm-mc rejects non-register/non-imm tokens.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad ∈ Operand \ required kind at slot. encode_sbfiz(ops with bad at which) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: Operand of wrong kind for slot }
  relation:
    op: throws
    expr: encode_sbfiz(ops_with_bad_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: get_reg/get_imm error paths; llvm-mc operand-kind rejection
```

## encode_sbfiz_neg_invalid_name
- Tier: 4e
- Rationale: parse_reg_num returns None for foo/x32/empty/r0. llvm-mc rejects invalid register names. Contract is Err.
- Seed: bitfield.rs encode_bfi_pbt encode_bfi_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_sbfiz with ops[which]=Reg(name) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid GPR spelling }
  relation:
    op: throws
    expr: encode_sbfiz(ops_with_invalid_name_at(which))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: parse_reg_num None; llvm-mc invalid register
```
