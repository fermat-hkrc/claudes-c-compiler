# Properties: encode_sbfm

## encode_sbfm_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. State machine rejected (pure function). Round-trip rejected (no in-tree SBFM decoder). Sibling encode_sbfiz/encode_sbfx rejected as differential (same-job gate: alias lsb/width vs raw immr/imms). Sibling encode_ubfm/encode_bfm rejected (different opc). Doc evidence: README.md:11 GNU-style assembly; encoder/mod.rs:888 dispatch; ARM ARM Bitfield Move SBFM.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, immr,imms ∈ 0..(R-1) where R=32+32·is_64. encode_sbfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(immr), Imm(imms)]) = Word(llvm-mc("sbfm Rd, Rn, #immr, #imms"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbfm
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
    lhs: encode_sbfm(ops4(is_64, rd, rn, immr, imms))
    rhs: llvm_mc_word(asm)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
evidence: README.md:11; encoder/mod.rs:888; ARM ARM Bitfield Move SBFM
```

## encode_sbfm_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM encoding of SBFM (sf 00 100110 N immr imms Rn Rd, N=sf). Stronger differential is the sibling property; this pins the field layout independently of llvm-mc disassembly aliases (SBFIZ/SBFX/ASR).
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_arm_fields
- Formal: ∀ valid SBFM operands. let w = encode_sbfm(...). w = (sf<<31)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd ∧ w[31]=sf ∧ w[30:29]=00 ∧ w[28:23]=100110 ∧ w[22]=sf ∧ w[21:16]=immr ∧ w[15:10]=imms ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbfm
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
    lhs: encode_sbfm(ops4(is_64, rd, rn, immr, imms))
    rhs: arm_sbfm_word(sf, rd, rn, immr, imms)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Bitfield Move SBFM encoding sf 00 100110 N immr imms Rn Rd
```

## encode_sbfm_metamorphic_rd_rn
- Tier: 4
- Rationale: ARM encoding places Rd in bits[4:0] and Rn in bits[9:5] independently. Metamorphic: incrementing Rd/Rn updates only that field. Stronger round-trip rejected (no decoder).
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_metamorphic_rd_rn
- Formal: ∀ is_64, rd,rn ∈ 0..30, immr,imms ∈ {0,1} ∩ 0..(R-1). encode_sbfm(...,rd+1,...)[4:0] = rd+1 ∧ other bits equal; encode_sbfm(...,rn+1,...)[9:5] = rn+1 ∧ other bits equal
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbfm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms]
  domain:
    is_64: bool
    rd: 0..30
    rn: 0..30
    immr: {0,1}
    imms: {0,1}
  relation:
    op: holds
    expr: encode_sbfm(rd+1).bits_4_0 == rd+1 && encode_sbfm(rd+1).other == encode_sbfm(rd).other && encode_sbfm(rn+1).bits_9_5 == rn+1 && encode_sbfm(rn+1).other == encode_sbfm(rn).other
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM SBFM Rd bits[4:0] Rn bits[9:5]
```

## encode_sbfm_neg_arity
- Tier: 4
- Rationale: Negative/error contract: SBFM requires 4 operands. llvm-mc "too few operands". get_reg/get_imm return Err on missing index. Stronger oracles do not apply to the invalid domain.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_arity
- Formal: ∀ len ∈ 0..3, valid prefix operands. encode_sbfm(ops[0..len]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain:
    len: 0..3
    is_64: bool
    rd: 0..31
    rn: 0..31
  relation:
    op: throws
    expr: encode_sbfm(ops.truncate(len))
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; get_reg/get_imm Err on missing index
```

## encode_sbfm_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract: SBFM has no 5th operand. llvm-mc "invalid operand". Documented GNU-style assembler contract (README.md:11).
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_extra_operand
- Formal: ∀ valid SBFM ops, extra ∈ Operand. encode_sbfm(ops ++ [extra]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: sbfm w0, w0, #0, #0 plus extra Reg("x0") encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_sbfm_extra_operand.md

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms, extra]
  domain:
    valid: sbfm_valid
    extra: Operand
  relation:
    op: throws
    expr: encode_sbfm(ops4 ++ [extra])
generators:
  is_64: { gen: bool }
  extra: extra_operand
expected_error: String
evidence: llvm-mc extra operand rejected; README.md:11 same textual assembly as gas
```

## encode_sbfm_neg_sp
- Tier: 4
- Rationale: ARM ARM Bitfield Move uses ZR for register 31, not SP. llvm-mc rejects sp/wsp as SBFM operands.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_sbfm with SP/WSP at slot which is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: sbfm wsp, w0, #0, #0 encodes as wzr instead of Err
- Bug report: pbt-out/bug_reports/encode_sbfm_sp.md

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain:
    which: 0..1
    sp64: bool
    is_64: bool
    other: 0..30
  relation:
    op: throws
    expr: encode_sbfm(ops_with_sp)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM SBFM Rd/Rn are W/X including ZR, not SP; llvm-mc rejects sp/wsp
```

## encode_sbfm_neg_immr_imms
- Tier: 4
- Rationale: ARM ARM and llvm-mc require 0<=immr,imms<datasize. Boundary sampled at -1, R, R+1. Documented GNU-style assembler contract.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_immr_imms
- Formal: ∀ is_64, rd,rn, (immr,imms) ∉ [0,R)². encode_sbfm(...) is Err (or panic-as-fail)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: sbfm w0, w0, #-1, #0 encodes instead of Err (immr=-1 as u32 overflows the 6-bit field)
- Bug report: pbt-out/bug_reports/encode_sbfm_immr_imms.md

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    immr_imms: invalid_immr_imms
  relation:
    op: throws
    expr: encode_sbfm(ops4)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM SBFM 0<=immr,imms<datasize; llvm-mc "immediate must be an integer in range [0, 31/63]"
```

## encode_sbfm_neg_mixed_width
- Tier: 4
- Rationale: llvm-mc/gas reject mixed W/X (sbfm x0, w0 / sbfm w0, x0). GNU-style assembler contract. Stronger differential does not apply to invalid domain.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_sbfm([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(0)]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: sbfm x0, w0, #0, #0 encodes using sf from Rd and Rn number from Wn
- Bug report: pbt-out/bug_reports/encode_sbfm_mixed_width.md

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain:
    rd: 0..31
    rn: 0..31
    rd64: bool
    rn64: bool
    pre: rd64 != rn64
  relation:
    op: throws
    expr: encode_sbfm(mixed_ops)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: String
evidence: llvm-mc "invalid operand" for mixed W/X; README.md:11
```

## encode_sbfm_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for alternate GNU spellings (x31/w31, XZR/WZR, LR, uppercase). Same contract as encode_sbfm_diff_valid_gpr; generators skew to spelling variants.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_diff_alt_spellings
- Formal: ∀ valid SBFM operands, dest_spell,src_spell ∈ 0..4. encode_sbfm([Reg(spell(Rd)), Reg(spell(Rn)), Imm(immr), Imm(imms)]) = Word(llvm-mc("sbfm spell(Rd), spell(Rn), #immr, #imms"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbfm
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms, dest_spell, src_spell]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    immr: 0..R-1
    imms: 0..R-1
    dest_spell: 0..4
    src_spell: 0..4
  relation:
    op: eq
    lhs: encode_sbfm(ops_spelled)
    rhs: llvm_mc_word(asm_spelled)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style assembly; parse_reg_num lowercases names; lr=30
```

## encode_sbfm_neg_fp
- Tier: 4
- Rationale: Negative/error contract: SBFM operands are GPRs. llvm-mc rejects d/s/q/v/h/b prefixes. parse_reg_num currently accepts them.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_sbfm with FP/SIMD register at slot which is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: sbfm d0, x1, #0, #0 encodes as 32-bit SBFM w0 instead of Err
- Bug report: pbt-out/bug_reports/encode_sbfm_fp.md

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain:
    which: 0..1
    prefix: {d,s,q,v,h,b}
    n: 0..31
  relation:
    op: throws
    expr: encode_sbfm(ops_with_fp)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" for d0; ARM ARM SBFM uses W/X GPRs
```

## encode_sbfm_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract: slots 0-1 must be Reg, slots 2-3 must be Imm. get_reg/get_imm return Err on wrong kind.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad ∈ Operand \ expected_kind(which). encode_sbfm(ops with bad at which) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..3
    bad: non_reg_or_non_imm
  relation:
    op: throws
    expr: encode_sbfm(ops_with_bad_kind)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: get_reg/get_imm Err on wrong Operand kind; llvm-mc invalid operand
```

## encode_sbfm_neg_invalid_name
- Tier: 4
- Rationale: Negative/error contract: parse_reg_num returns None for foo/x32/empty/r0. llvm-mc rejects them.
- Seed: bitfield.rs encode_bfm_pbt::encode_bfm_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_sbfm with Reg(name) at slot which is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_sbfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain:
    which: 0..1
    name: invalid_gpr_name
  relation:
    op: throws
    expr: encode_sbfm(ops_with_invalid_name)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: parse_reg_num None for non-GPR names; llvm-mc invalid operand
```
