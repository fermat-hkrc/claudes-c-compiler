# Property ledger: encode_ubfiz

## encode_ubfiz_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (README.md:11 gas-compatible assembler; encoder/mod.rs encodes AArch64 32-bit words). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree UBFIZ decoder). encode_ubfm same-job gate fails (raw immr/imms vs alias lsb/width).
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, lsb ∈ 0..R-1, width ∈ 1..R-lsb (R=64 if is_64 else 32). encode_ubfiz([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)]) = Word(w) ∧ w = llvm-mc("ubfiz gpr(rd), gpr(rn), #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: encode_ubfiz([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])
    rhs: llvm_mc_word("ubfiz gpr(rd), gpr(rn), #lsb, #width")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:889; ARM ARM UBFIZ alias of UBFM
```

## encode_ubfiz_alias_ubfm
- Tier: 4c
- Rationale: Purpose comment bitfield.rs:75 states UBFIZ is an alias for UBFM Rd, Rn, #(-lsb MOD regsize), #(width-1). Not a same-job differential (different operand jobs); algebraic metamorphic after the ARM mapping.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_alias_sbfm
- Formal: ∀ valid (is_64,rd,rn,lsb,width). encode_ubfiz(Rd,Rn,#lsb,#width) = encode_ubfm(Rd,Rn,#(-lsb rem_euclid R),#(width-1))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: encode_ubfiz([Reg(gpr(rd)), Reg(gpr(rn)), Imm(lsb), Imm(width)])
    rhs: encode_ubfm([Reg(gpr(rd)), Reg(gpr(rn)), Imm((-lsb).rem_euclid(R)), Imm(width-1)])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: src/backend/arm/assembler/encoder/bitfield.rs:75 purpose comment; ARM ARM UBFIZ alias of UBFM
```

## encode_ubfiz_arm_fields
- Tier: 4d
- Rationale: ARM ARM Bitfield Move UBFM field layout is the encoding contract claimed by the assembler (encoder/mod.rs:3). Weaker than differential; still pins each field independently so a matching llvm-mc word cannot hide a swapped field that happens to agree on a sparse sample.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_arm_fields
- Formal: ∀ valid (is_64,rd,rn,lsb,width). let w = encode_ubfiz(...). w[31]=sf ∧ w[30:29]=10 ∧ w[28:23]=100110 ∧ w[22]=sf ∧ w[21:16]=(-lsb MOD R) ∧ w[15:10]=width-1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: encode_ubfiz([Reg(gpr(rd)), Reg(gpr(rn)), Imm(lsb), Imm(width)])
    rhs: (sf<<31)|(0b10<<29)|(0b100110<<23)|(sf<<22)|(((-lsb).rem_euclid(R))<<16)|((width-1)<<10)|(rn<<5)|rd
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM UBFM encoding sf 10 100110 N immr imms Rn Rd; bitfield.rs:75
```

## encode_ubfiz_metamorphic_rd_rn
- Tier: 4c
- Rationale: ARM field layout places Rd in bits[4:0] and Rn in bits[9:5]; incrementing one register must change only that field (metamorphic independence).
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_metamorphic_rd_rn
- Formal: ∀ is_64, rd,rn ∈ 0..30, lsb∈{0,1}, width=1. let b=encode_ubfiz(rd,rn,...); let d=encode_ubfiz(rd+1,rn,...); let n=encode_ubfiz(rd,rn+1,...). (d & 0x1f = rd+1) ∧ (d & !0x1f = b & !0x1f) ∧ ((n>>5)&0x1f = rn+1) ∧ (n & !(0x1f<<5) = b & !(0x1f<<5))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb]
  domain: { is_64: bool, rd: 0..30, rn: 0..30, lsb: {0,1}, width: 1 }
  relation:
    op: holds
    expr: "(encode_ubfiz(rd+1,rn) & 0x1f == rd+1) && (encode_ubfiz(rd+1,rn) & !0x1f == encode_ubfiz(rd,rn) & !0x1f) && ((encode_ubfiz(rd,rn+1)>>5)&0x1f == rn+1) && (encode_ubfiz(rd,rn+1) & !(0x1f<<5) == encode_ubfiz(rd,rn) & !(0x1f<<5))"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  lsb: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM UBFM Rd bits[4:0] Rn bits[9:5]
```

## encode_ubfiz_neg_arity
- Tier: 4e
- Rationale: llvm-mc/gas reject UBFIZ with fewer than 4 operands (README.md:11 gas-compatible). get_reg/get_imm return Err on missing slots. Stronger oracles do not apply to the invalid domain.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_arity
- Formal: ∀ len ∈ 0..3, is_64, rd,rn ∈ 0..31. encode_ubfiz(ops[0..len] of a valid 4-tuple) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: 0..3, is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: throws
    expr: encode_ubfiz(truncate(valid_ops, len))
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc rejects too few operands; README.md:11; get_reg/get_imm Err on missing idx
```

## encode_ubfiz_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects `ubfiz Rd, Rn, #lsb, #width, extra` as unrecognized. ARM ARM UBFIZ has exactly four operands. Extra operand must Err, not silently ignore.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_extra_operand
- Formal: ∀ valid 4-tuple ops, extra ∈ Operand. encode_ubfiz(ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, lsb=0, width=1, extra=Reg("x0") — ubfiz w0, w0, #0, #1, x0
- Bug report: pbt-out/bug_reports/encode_ubfiz_extra_operand.md

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, extra]
  domain: { valid UBFIZ 4-tuple, extra: Operand }
  relation:
    op: throws
    expr: encode_ubfiz(ops4 ++ [extra])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
  extra: { gen: oneof, options: [Reg, Imm, Shift, RegArrangement] }
expected_error: Err
evidence: llvm-mc "unrecognized instruction mnemonic" for 5-operand ubfiz; ARM ARM UBFIZ arity 4
```

## encode_ubfiz_neg_sp
- Tier: 4e
- Rationale: ARM ARM Bitfield Move uses ZR for register 31, not SP. llvm-mc rejects `ubfiz wsp/sp, ...` and `ubfiz ..., wsp/sp, ...`. parse_reg_num maps sp/wsp to 31, so the SUT currently encodes them as ZR — that is the contract under test, not Doc evidence of intent.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, is_64, other ∈ 0..30. encode_ubfiz with slot `which` replaced by Reg(sp) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0 — ubfiz wsp, w0, #0, #1
- Bug report: pbt-out/bug_reports/encode_ubfiz_sp.md

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: {0,1}, sp: {sp,wsp}, is_64: bool, other: 0..30 }
  relation:
    op: throws
    expr: encode_ubfiz(ops with slot which = Reg(sp))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: Err
evidence: ARM ARM UBFM register 31 is ZR not SP; llvm-mc "invalid operand for instruction" on wsp/sp
```

## encode_ubfiz_neg_lsb_width
- Tier: 4e
- Rationale: ARM ARM UBFIZ constraints 0<=lsb<datasize, 1<=width<=datasize-lsb. llvm-mc: "expected integer in range [1, 32]" for width=0; "[0, 31]" for lsb=32 on W form. Boundary generators pin lsb=R, width=0, width=-1, lsb=-1, width=R-lsb+1.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_lsb_width
- Formal: ∀ is_64, rd,rn ∈ 0..31, (lsb,width) ∉ {0<=lsb<R ∧ 1<=width<=R-lsb}. encode_ubfiz(...) = Err (or panic counts as failure of the Err contract)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, lsb=0, width=0 — ubfiz w0, w0, #0, #0 (debug overflow at bitfield.rs:85)
- Bug report: pbt-out/bug_reports/encode_ubfiz_lsb_width.md

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { (lsb,width) outside 0<=lsb<R, 1<=width<=R-lsb }
  relation:
    op: throws
    expr: encode_ubfiz([Reg(gpr(rd)), Reg(gpr(rn)), Imm(lsb), Imm(width)])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: -1, max: 65, type: i64 }
  width: { gen: int, min: -1, max: 65, type: i64 }
expected_error: Err
evidence: ARM ARM UBFIZ 0<=lsb<datasize, 1<=width<=datasize-lsb; llvm-mc range errors
```

## encode_ubfiz_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for alternate GPR spellings (x31/w31, XZR/WZR, LR, uppercase). README.md:11 gas-compatible; parse_reg_num lowercases names and maps lr/x31.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_diff_alt_spellings
- Formal: ∀ valid (is_64,rd,rn,lsb,width), dest_spell,src_spell ∈ 0..4. encode_ubfiz([Reg(spell(rd)), Reg(spell(rn)), Imm(lsb), Imm(width)]) = llvm-mc("ubfiz spell(rd), spell(rn), #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, dest_spell, src_spell]
  domain: { valid UBFIZ 4-tuple, dest_spell: 0..4, src_spell: 0..4 }
  relation:
    op: eq
    lhs: encode_ubfiz([Reg(spell(rd)), Reg(spell(rn)), Imm(lsb), Imm(width)])
    rhs: llvm_mc_word("ubfiz spell(rd), spell(rn), #lsb, #width")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11; parse_reg_num lowercases and maps lr/x31; llvm-mc accepts x31/w31/LR/uppercase
```

## encode_ubfiz_neg_mixed_width
- Tier: 4e
- Rationale: llvm-mc rejects mixed W/X (`ubfiz x0, w0, #0, #1` invalid operand). ARM ARM requires matching datasize for Rd and Rn. encode_ubfiz takes sf from Rd only.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_ubfiz([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — ubfiz x0, w0, #0, #1
- Bug report: pbt-out/bug_reports/encode_ubfiz_mixed_width.md

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd: 0..31, rn: 0..31, rd64 != rn64 }
  relation:
    op: throws
    expr: encode_ubfiz([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: Err
evidence: llvm-mc "invalid operand for instruction" on mixed W/X; ARM ARM matching datasize
```

## encode_ubfiz_neg_fp
- Tier: 4e
- Rationale: llvm-mc rejects FP/SIMD prefixes (d/s/q/v/h/b) as UBFIZ operands. parse_reg_num accepts them as GPR numbers. Not a valid UBFIZ register class.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_ubfiz with slot which = Reg(prefix||n) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — ubfiz d0, x1, #0, #1
- Bug report: pbt-out/bug_reports/encode_ubfiz_fp.md

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: {0,1}, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_ubfiz(ops with slot which = Reg(prefix||n))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc "invalid operand for instruction" on d0; ARM ARM GPR-only UBFIZ
```

## encode_ubfiz_neg_nonreg
- Tier: 4e
- Rationale: get_reg/get_imm return Err on wrong operand kind. llvm-mc rejects non-register/non-imm at those slots.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad ∈ Operand \ expected-kind-at-slot. encode_ubfiz with slot which = bad = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: non-matching Operand kind }
  relation:
    op: throws
    expr: encode_ubfiz(ops with slot which = bad)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: Err
evidence: get_reg/get_imm Err on wrong kind; llvm-mc rejects non-reg/non-imm
```

## encode_ubfiz_neg_invalid_name
- Tier: 4e
- Rationale: parse_reg_num returns None for foo/x32/empty/r0; get_reg then Err. llvm-mc rejects invalid names.
- Seed: bitfield.rs encode_sbfiz_pbt encode_sbfiz_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_ubfiz with slot which = Reg(name) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfiz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: {0,1}, name: invalid GPR names }
  relation:
    op: throws
    expr: encode_ubfiz(ops with slot which = Reg(name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: Err
evidence: parse_reg_num None for foo/x32; llvm-mc invalid operand
```
