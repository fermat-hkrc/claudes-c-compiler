# Properties: encode_bfm

## encode_bfm_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. State machine rejected (pure function). Round-trip rejected (no in-tree BFM decoder). Sibling encode_bfi/encode_bfxil rejected as differential (same-job gate: alias lsb/width vs raw immr/imms). Doc evidence: README.md:11 GNU-style assembly; encoder/mod.rs:891 dispatch; ARM ARM Bitfield Move BFM.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, immr,imms ∈ 0..(R-1) where R=32+32·is_64. encode_bfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(immr), Imm(imms)]) = Word(llvm-mc("bfm Rd, Rn, #immr, #imms"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bfm
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    immr: 0..R-1
    imms: 0..R-1
  relation:
    op: eq
    lhs: encode_bfm(ops4(is_64, rd, rn, immr, imms))
    rhs: llvm_mc_word(asm)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
evidence: README.md:11; encoder/mod.rs:891; ARM ARM Bitfield Move BFM
```

## encode_bfm_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM encoding of BFM (sf 01 100110 N immr imms Rn Rd, N=sf). Stronger differential is the sibling property; this pins the field layout independently of llvm-mc disassembly aliases (BFI/BFXIL).
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_arm_fields
- Formal: ∀ valid BFM operands. let w = encode_bfm(...). w = (sf<<31)|(0b01<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd ∧ w[31]=sf ∧ w[30:29]=01 ∧ w[28:23]=100110 ∧ w[22]=sf ∧ w[21:16]=immr ∧ w[15:10]=imms ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bfm
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    immr: 0..R-1
    imms: 0..R-1
  relation:
    op: eq
    lhs: encode_bfm(ops4(is_64, rd, rn, immr, imms))
    rhs: arm_bfm_word(sf, rd, rn, immr, imms)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Bitfield Move BFM encoding sf 01 100110 N immr imms Rn Rd
```

## encode_bfm_metamorphic_rd_rn
- Tier: 4
- Rationale: ARM encoding places Rd in bits[4:0] and Rn in bits[9:5] independently. Metamorphic: incrementing Rd/Rn updates only that field. Stronger round-trip rejected (no decoder).
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_metamorphic_rd_rn
- Formal: ∀ is_64, rd,rn ∈ 0..30, immr,imms ∈ {0,1} ∩ 0..(R-1). encode_bfm(...,rd+1,...)[4:0] = rd+1 ∧ other bits equal; encode_bfm(...,rn+1,...)[9:5] = rn+1 ∧ other bits equal
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bfm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms]
  domain:
    rd: 0..30
    rn: 0..30
  relation:
    op: holds
    expr: (w_rd & 0x1f == rd + 1) && (w_rd & !0x1f == base & !0x1f) && (((w_rn >> 5) & 0x1f) == rn + 1)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  immr: { gen: int, min: 0, max: 1, type: u32 }
  imms: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM BFM Rd bits[4:0] Rn bits[9:5]
```

## encode_bfm_neg_arity
- Tier: 4
- Rationale: Negative/error contract. llvm-mc reports too few operands for instruction. get_reg/get_imm fail when index is missing. Documented by llvm-mc and GNU as (README.md:11).
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_arity
- Formal: ∀ len ∈ 0..3, valid prefix operands. encode_bfm(ops[0..len]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain:
    len: 0..3
  relation:
    op: throws
    expr: encode_bfm(ops_truncated_to(len))
expected_error: String
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc too few operands; README.md:11
```

## encode_bfm_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 5th operand. GNU-style assembler must reject it. encode_bfm does not check operands.len().
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_extra_operand
- Formal: ∀ valid 4-operand BFM ops, extra ∈ Operand. encode_bfm(ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: [Reg("w0"), Reg("w0"), Imm(0), Imm(0), Reg("x0")]
- Bug report: pbt-out/bug_reports/encode_bfm_extra_operand.md

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, extra]
  domain:
    ops: valid_bfm_4
    extra: Operand
  relation:
    op: throws
    expr: encode_bfm(ops_plus_extra)
expected_error: String
generators:
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
evidence: llvm-mc rejects fifth operand; README.md:11
```

## encode_bfm_neg_sp
- Tier: 4
- Rationale: Negative/error contract. ARM ARM Bitfield Move uses ZR not SP at register 31. llvm-mc rejects sp/wsp. parse_reg_num maps sp/wsp to 31.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_bfm with slot which = SP = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0 — bfm wsp, w0, #0, #0
- Bug report: pbt-out/bug_reports/encode_bfm_sp.md

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp, is_64, other]
  domain:
    which: 0..1
    sp: [sp, wsp]
  relation:
    op: throws
    expr: encode_bfm(ops_with_sp_at_which)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM BFM register 31 is ZR; llvm-mc rejects sp/wsp
```

## encode_bfm_neg_immr_imms
- Tier: 4
- Rationale: Negative/error contract. ARM/llvm-mc: 32-bit immr,imms in [0,31]; 64-bit in [0,63]. Bounds sampled at -1, R, R+1. Documented bound must be hit exactly.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_lsb_width
- Formal: ∀ is_64, rd, rn, (immr,imms) not in [0,R-1]×[0,R-1]. encode_bfm(...) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, immr=-1, imms=0
- Bug report: pbt-out/bug_reports/encode_bfm_immr_imms.md

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms]
  domain:
    immr_imms: outside_0_to_R_minus_1
  relation:
    op: throws
    expr: encode_bfm(ops4(is_64, rd, rn, immr, imms))
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: -1, max: 64, type: i64 }
  imms: { gen: int, min: -1, max: 64, type: i64 }
evidence: llvm-mc immediate must be in range [0, 31] or [0, 63]; ARM ARM BFM
```

## encode_bfm_neg_mixed_width
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects mixed W/X. GNU-style assembler must reject it. encode_bfm takes sf from Rd only.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_bfm([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(0)]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — bfm x0, w0, #0, #0
- Bug report: pbt-out/bug_reports/encode_bfm_mixed_width.md

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain:
    rd64_ne_rn64: true
  relation:
    op: throws
    expr: encode_bfm(mixed_wx_ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
evidence: llvm-mc rejects mixed W/X; README.md:11
```

## encode_bfm_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for alternate register spellings (x31/w31, XZR/WZR, LR, uppercase). Sweep of documented GNU-style names. Stronger state-machine/round-trip rejected as for encode_bfm_diff_valid_gpr.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_diff_alt_spellings
- Formal: ∀ valid BFM operands, dest_spell,src_spell ∈ 0..4. encode_bfm([Reg(spell(Rd)), Reg(spell(Rn)), Imm(immr), Imm(imms)]) = Word(llvm-mc("bfm spell(Rd), spell(Rn), #immr, #imms"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bfm
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms, dest_spell, src_spell]
  domain:
    dest_spell: 0..4
    src_spell: 0..4
  relation:
    op: eq
    lhs: encode_bfm(ops_with_spell)
    rhs: llvm_mc_word(asm)
generators:
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style assembly; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_bfm_neg_fp
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects FP/SIMD prefixes as BFM operands. parse_reg_num accepts d/s/q/v/h/b. Sweep of documented GPR-only constraint.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_bfm with slot which = prefix n = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix=d, n=0 — bfm d0, x1, #0, #0
- Bug report: pbt-out/bug_reports/encode_bfm_fp.md

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain:
    which: 0..1
    prefix: [d, s, q, v, h, b]
    n: 0..31
  relation:
    op: throws
    expr: encode_bfm(ops_with_fp_at_which)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects d0 as BFM operand; README.md:11
```

## encode_bfm_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract. Wrong operand kinds (Imm/Shift/Mem/Label/Symbol/Cond/RegArrangement) at Rd/Rn, or non-Imm at immr/imms, must Err. get_reg/get_imm already reject wrong kinds.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad kind not matching the slot. encode_bfm(ops with slot which = bad) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..3
  relation:
    op: throws
    expr: encode_bfm(ops_with_bad_kind)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: get_reg/get_imm type checks; llvm-mc operand-kind errors
```

## encode_bfm_neg_invalid_name
- Tier: 4
- Rationale: Negative/error contract. Invalid register names (foo/x32/empty/r0) must Err. parse_reg_num returns None.
- Seed: bitfield.rs encode_bfi_pbt::encode_bfi_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_bfm with slot which = name = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_bfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain:
    which: 0..1
  relation:
    op: throws
    expr: encode_bfm(ops_with_invalid_name)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: parse_reg_num returns None for invalid names; llvm-mc rejects them
```
