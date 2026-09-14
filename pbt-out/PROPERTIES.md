# Properties: encode_bfxil

## encode_bfxil_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree BFXIL decoder). encode_bfm rejected as differential sibling (raw immr/imms form, different assembly syntax). encode_ubfx/encode_sbfx rejected (UBFM/SBFM opc). Doc evidence: assembler README.md:11 (same textual assembly as gas); README.md:216 lists bfxil; encoder/mod.rs:893 dispatch; ARM ARM BFXIL alias of BFM.
- Seed: bitfield.rs encode_bfi_pbt llvm-mc differential
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, lsb ∈ 0..R-1, width ∈ 1..R-lsb (R=64 if is_64 else 32). encode_bfxil([Reg(gpr), Reg(gpr), Imm(lsb), Imm(width)]) = Word(w) ∧ w = llvm-mc("bfxil Rd, Rn, #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb, R: 32|64 }
  relation:
    op: eq
    lhs: encode_bfxil([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])
    rhs: llvm_mc("bfxil Rd, Rn, #lsb, #width")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; README.md:216; encoder/mod.rs:893; ARM ARM BFXIL alias
```

## encode_bfxil_alias_bfm
- Tier: 4c
- Rationale: ARM ARM and bitfield.rs:118 purpose comment define BFXIL as the BFM alias with immr=lsb, imms=lsb+width-1. Not differential (encode_bfm is in-tree, different syntax). Stronger round-trip rejected (no decoder). Required metamorphic/differential companion to the llvm-mc differential.
- Seed: (none) — ARM ARM BFXIL <=> BFM alias
- Formal: ∀ valid (is_64, rd, rn, lsb, width). encode_bfxil([Rd,Rn,#lsb,#width]) = encode_bfm([Rd,Rn, #lsb, #(lsb+width-1)])
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { valid BFXIL W/X domain }
  relation:
    op: eq
    lhs: encode_bfxil([Reg(Rd), Reg(Rn), Imm(lsb), Imm(width)])
    rhs: encode_bfm([Reg(Rd), Reg(Rn), Imm(lsb), Imm(lsb+width-1)])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: bitfield.rs:118 purpose comment; ARM ARM BFXIL alias of BFM
```

## encode_bfxil_arm_fields
- Tier: 4d
- Rationale: ARM ARM Bitfield encoding sf 01 100110 N immr imms Rn Rd with N=sf, opc=01, immr=lsb, imms=lsb+width-1. Weaker than differential (already used). Invariant is the architectural field layout, not the producing statement.
- Seed: bitfield.rs encode_bfi_arm_fields
- Formal: ∀ valid (is_64, rd, rn, lsb, width). let w = encode_bfxil(...).Word. w[31]=sf ∧ w[30:29]=01 ∧ w[28:23]=100110 ∧ w[22]=sf ∧ w[21:16]=lsb ∧ w[15:10]=lsb+width-1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { valid BFXIL W/X domain }
  relation:
    op: holds
    expr: arm_bfm_fields(encode_bfxil(ops))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM Bitfield BFM/BFXIL encoding; assembler README.md:11
```

## encode_bfxil_metamorphic_rd_rn
- Tier: 4c
- Rationale: ARM encoding places Rd in bits[4:0] and Rn in bits[9:5]; incrementing one register number must change only that field. Weaker than differential.
- Seed: bitfield.rs encode_bfi_metamorphic_rd_rn
- Formal: ∀ valid BFXIL with rd,rn ∈ 0..30. encode_bfxil(rd+1,...) xor encode_bfxil(rd,...) has only bits[4:0] changed to rd+1; encode_bfxil(...,rn+1,...) xor base has only bits[9:5] changed to rn+1
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { rd,rn in 0..30, valid lsb/width }
  relation:
    op: holds
    expr: field_independence(encode_bfxil, rd, rn)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM Bitfield Rd bits[4:0] Rn bits[9:5]
```

## encode_bfxil_neg_arity
- Tier: 5
- Rationale: llvm-mc rejects too few operands (`bfxil w0, w1, #0` is unrecognized). get_reg/get_imm return Err when the slot is missing. Documented assembler contract (README.md:11 gas-compatible). Negative/error after stronger oracles on the valid domain.
- Seed: bitfield.rs encode_bfi_neg_arity
- Formal: ∀ len ∈ 0..3, valid dummy regs. encode_bfxil(ops[0..len]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: 0..3 }
  relation:
    op: throws
    expr: encode_bfxil(ops.truncate(len))
    error: Err
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc rejects too few operands; README.md:11 gas-compatible
```

## encode_bfxil_neg_extra_operand
- Tier: 5
- Rationale: llvm-mc rejects a 5th operand (`unrecognized instruction mnemonic`). BFXIL assembly is Rd, Rn, #lsb, #width only. Documented gas-compatible contract.
- Seed: bitfield.rs encode_bfi_neg_extra_operand
- Formal: ∀ valid BFXIL ops, extra operand. encode_bfxil(ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: [Reg("w0"), Reg("w0"), Imm(0), Imm(1), Reg("x0")]
- Bug report: pbt-out/bug_reports/encode_bfxil_extra_operand.md

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, extra]
  domain: { valid BFXIL + extra Operand }
  relation:
    op: throws
    expr: encode_bfxil(ops4 ++ [extra])
    error: Err
generators:
  extra: { gen: oneof, options: [Reg, Imm, Shift, RegArrangement] }
expected_error: Err
evidence: llvm-mc rejects extra operand; README.md:11
```

## encode_bfxil_neg_sp
- Tier: 5
- Rationale: ARM ARM Bitfield uses ZR for register 31, not SP. llvm-mc rejects `bfxil sp, ...` and `bfxil wsp, ...`. parse_reg_num maps sp/wsp to 31, so the SUT currently encodes them as ZR — that is the contract under test, not a characterizing of current behavior.
- Seed: bitfield.rs encode_bfi_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_bfxil(ops with slot which = SP) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0 — bfxil wsp, w0, #0, #1
- Bug report: pbt-out/bug_reports/encode_bfxil_sp.md

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: 0..1, sp: sp|wsp }
  relation:
    op: throws
    expr: encode_bfxil(ops with SP at slot which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: Err
evidence: ARM ARM register 31 is ZR not SP; llvm-mc rejects SP/WSP
```

## encode_bfxil_neg_lsb_width
- Tier: 5
- Rationale: ARM ARM requires 0 <= lsb < R and 1 <= width <= R-lsb. llvm-mc rejects width 0, negative, lsb out of range, and extract overflow. Bounds sampled at 0, R-1, R, R+1, width=0, width=R-lsb+1.
- Seed: bitfield.rs encode_bfi_neg_lsb_width
- Formal: ∀ is_64, rd, rn, (lsb,width) with ¬(0<=lsb<R ∧ 1<=width<=R-lsb). encode_bfxil(...) = Err (no panic)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, lsb=0, width=0 — bfxil w0, w0, #0, #0 (debug overflow at lsb+width-1)
- Bug report: pbt-out/bug_reports/encode_bfxil_lsb_width.md

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { invalid lsb/width relative to R }
  relation:
    op: throws
    expr: encode_bfxil(ops4(is_64,rd,rn,lsb,width))
    error: Err
generators:
  lsb: { gen: int, min: -1, max: 65, type: i64 }
  width: { gen: int, min: -1, max: 65, type: i64 }
expected_error: Err
evidence: ARM ARM 0<=lsb<R, 1<=width<=R-lsb; llvm-mc range errors
```

## encode_bfxil_diff_alt_spellings
- Tier: 2
- Rationale: Strengthening / contract-surface: GNU-style aliases x31/w31, XZR/WZR, LR, uppercase must match llvm-mc. Same differential oracle as encode_bfxil_diff_valid_gpr.
- Seed: bitfield.rs encode_bfi_diff_alt_spellings
- Formal: ∀ valid (is_64, rd, rn, lsb, width), dest/src spellings in {gpr, x31/w31 if 31, XZR/WZR if 31, LR if x30, uppercase}. encode_bfxil([Reg(dest), Reg(src), Imm(lsb), Imm(width)]) = llvm-mc("bfxil dest, src, #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, dest_spell, src_spell]
  domain: { valid BFXIL plus alt spellings }
  relation:
    op: eq
    lhs: encode_bfxil([Reg(spell(dest)), Reg(spell(src)), Imm(lsb), Imm(width)])
    rhs: llvm_mc("bfxil dest, src, #lsb, #width")
generators:
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 gas-compatible; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_bfxil_neg_mixed_width
- Tier: 5
- Rationale: Strengthening: llvm-mc rejects mixed W/X (`bfxil x0, w1, #0, #1` invalid operand). encode_bfxil takes is_64 only from Rd.
- Seed: bitfield.rs encode_bfi_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_bfxil([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — bfxil x0, w0, #0, #1
- Bug report: pbt-out/bug_reports/encode_bfxil_mixed_width.md

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd64 != rn64 }
  relation:
    op: throws
    expr: encode_bfxil([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)])
    error: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: Err
evidence: llvm-mc rejects mixed W/X; README.md:11
```

## encode_bfxil_neg_fp
- Tier: 5
- Rationale: Strengthening: llvm-mc rejects S/D/Q/V/H/B as BFXIL operands. parse_reg_num accepts those prefixes.
- Seed: bitfield.rs encode_bfi_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_bfxil(ops with slot which = prefix+n) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — bfxil d0, x1, #0, #1
- Bug report: pbt-out/bug_reports/encode_bfxil_fp.md

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: d|s|q|v|h|b, n: 0..31 }
  relation:
    op: throws
    expr: encode_bfxil(ops with FP at slot which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc rejects FP/SIMD; ARM ARM BFXIL is GPR-only
```

## encode_bfxil_neg_nonreg
- Tier: 5
- Rationale: Sweep: wrong operand kind (Imm at Rd/Rn, Shift/Mem/Label/Symbol/Cond/RegArrangement at any slot that is not an Imm slot) must Err via get_reg/get_imm.
- Seed: bitfield.rs encode_bfi_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad Operand of the wrong kind for that slot. encode_bfxil(...) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: non-matching Operand kind }
  relation:
    op: throws
    expr: encode_bfxil(ops with bad at slot which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: Err
evidence: get_reg/get_imm type errors; llvm-mc operand kind
```

## encode_bfxil_neg_invalid_name
- Tier: 5
- Rationale: Sweep: invalid register names (foo/x32/empty/r0) must Err via parse_reg_num.
- Seed: bitfield.rs encode_bfi_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_bfxil(ops with Reg(name) at slot which) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_bfxil
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid GPR spelling }
  relation:
    op: throws
    expr: encode_bfxil(ops with Reg(name) at slot which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: Err
evidence: parse_reg_num returns None; llvm-mc rejects invalid names
```
