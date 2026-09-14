# Properties: encode_extr

## encode_extr_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree EXTR decoder). encode_shift ROR rejected as differential sibling (same-job gate: ROR is a 3-operand shift mnemonic; EXTR is 4-operand extract). Doc evidence: assembler README.md:11 (same textual assembly as gas); README.md:216 lists extr; encoder/mod.rs:894 dispatch; ARM ARM Extract EXTR encoding.
- Seed: bitfield.rs encode_bfi_pbt llvm-mc differential
- Formal: ∀ is_64 ∈ {false,true}, rd,rn,rm ∈ 0..31, lsb ∈ 0..(R-1) where R=32 if ¬is_64 else 64. encode_extr([Reg(Rd), Reg(Rn), Reg(Rm), Imm(lsb)]) = Word(w) ∧ w = llvm-mc("extr Rd, Rn, Rm, #lsb") where registers are W if ¬is_64 else X (31 spelled wzr/xzr)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, rm, lsb]
  domain: { is_64: bool, rd,rn,rm: 0..31 matching W or X, lsb: 0..R-1 }
  relation:
    op: eq
    lhs: encode_extr([Reg(Rd), Reg(Rn), Reg(Rm), Imm(lsb)])
    rhs: llvm_mc("extr Rd, Rn, Rm, #lsb")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; README.md:216; encoder/mod.rs:894; ARM ARM Extract EXTR
```

## encode_extr_arm_fields
- Tier: 4d
- Rationale: ARM ARM Extract EXTR encoding sf 00 100111 N 0 Rm imms Rn Rd with N=sf, imms=lsb. Weaker than differential (already used). Invariant is the architectural field layout, unpacked independently of the producing statement. Documented bounds lsb=0 and lsb=R-1 are sampled exactly.
- Seed: bitfield.rs encode_bfi_arm_fields
- Formal: ∀ is_64 ∈ {false,true}, rd,rn,rm ∈ 0..31, lsb ∈ 0..(R-1). let w = encode_extr(...).Word. w[31]=sf ∧ w[30:23]=00100111 ∧ w[22]=sf ∧ w[21]=0 ∧ w[20:16]=rm ∧ w[15:10]=lsb ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, rm, lsb]
  domain: { is_64: bool, rd,rn,rm: 0..31 matching W or X, lsb: 0..R-1 }
  relation:
    op: holds
    expr: arm_extr_fields(encode_extr([Reg(Rd), Reg(Rn), Reg(Rm), Imm(lsb)]))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Extract EXTR sf 00 100111 N 0 Rm imms Rn Rd; assembler README.md:11
```

## encode_extr_metamorphic_rd_rn_rm
- Tier: 4c
- Rationale: ARM encoding places Rd in bits[4:0], Rn in bits[9:5], Rm in bits[20:16]; sf at bit 31. Incrementing one register must change only that field. Weaker than differential. Required metamorphic companion.
- Seed: bitfield.rs encode_bfi_metamorphic_rd_rn
- Formal: ∀ is_64 ∈ {false,true}, rd,rn,rm ∈ 0..30, lsb ∈ {0,1}. encode_extr(rd+1) xor encode_extr(rd) has only bits[4:0] updated to rd+1; Rn+1 only bits[9:5]; Rm+1 only bits[20:16]
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, rm, lsb]
  domain: { is_64: bool, rd,rn,rm in 0..30, lsb in {0,1} }
  relation:
    op: holds
    expr: field_independence(encode_extr, rd, rn, rm)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  lsb: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Extract EXTR Rd bits[4:0] Rn bits[9:5] Rm bits[20:16]
```

## encode_extr_alias_ror
- Tier: 4c
- Rationale: ARM ARM documents ROR (immediate) as the alias of EXTR when Rn=Rm: ROR Rd, Rs, #shift <=> EXTR Rd, Rs, Rs, #shift. Not a same-job differential sibling (encode_shift implements a different mnemonic). Metamorphic: encode_extr(Rd,Rn,Rn,#lsb) equals llvm-mc("ror Rd, Rn, #lsb").
- Seed: (none) — ARM alias, no existing EXTR test
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, lsb ∈ 0..(R-1). encode_extr([Reg(Rd), Reg(Rn), Reg(Rn), Imm(lsb)]) = llvm-mc("ror Rd, Rn, #lsb")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb]
  domain: { is_64: bool, rd,rn: 0..31 matching W or X, lsb: 0..R-1 }
  relation:
    op: eq
    lhs: encode_extr([Reg(Rd), Reg(Rn), Reg(Rn), Imm(lsb)])
    rhs: llvm_mc("ror Rd, Rn, #lsb")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM ROR (immediate) alias of EXTR when Rn=Rm; assembler README.md:11
```

## encode_extr_neg_arity
- Tier: 4e
- Rationale: llvm-mc reports "too few operands for instruction" for EXTR with fewer than 4 operands. Negative/error contract from the assembler contract (README.md:11).
- Seed: bitfield.rs encode_bfi_neg_arity
- Formal: ∀ len ∈ 0..3, is_64, rd,rn,rm ∈ 0..31. encode_extr(ops truncated to len) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn, rm]
  domain: { len: 0..3, is_64: bool, rd,rn,rm: 0..31 }
  relation:
    op: throws
    lhs: encode_extr(ops[0..len])
    error: Err
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc "too few operands for instruction"; assembler README.md:11
```

## encode_extr_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc reports "invalid operand for instruction" for a 5th EXTR operand. Negative/error contract from the assembler contract.
- Seed: bitfield.rs encode_bfi_neg_extra_operand
- Formal: ∀ valid EXTR ops, extra ∈ Operand. encode_extr(ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: extr w0, w0, w0, #0 plus extra Reg("x0") encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_extr_extra_operand.md

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, rm, lsb, extra]
  domain: { valid EXTR 4-tuple, extra: Operand }
  relation:
    op: throws
    lhs: encode_extr(ops ++ [extra])
    error: Err
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: Err
evidence: llvm-mc "invalid operand for instruction" on 5th EXTR operand; assembler README.md:11
```

## encode_extr_neg_sp
- Tier: 4e
- Rationale: ARM ARM Extract uses ZR not SP at register 31. llvm-mc rejects SP/WSP in any of Rd/Rn/Rm. Negative/error contract.
- Seed: bitfield.rs encode_bfi_neg_sp
- Formal: ∀ which ∈ {0,1,2}, sp ∈ {sp,wsp}, other GPRs. encode_extr with ops[which]=SP = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: extr wsp, w0, w0, #0 encodes instead of Err (which=0, sp=wsp)
- Bug report: pbt-out/bug_reports/encode_extr_sp.md

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: 0..2, sp: sp|wsp }
  relation:
    op: throws
    lhs: encode_extr(ops with SP at which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: Err
evidence: ARM ARM Extract register 31 is ZR not SP; llvm-mc rejects SP/WSP; assembler README.md:11
```

## encode_extr_neg_lsb
- Tier: 4e
- Rationale: ARM ARM and llvm-mc require 0 <= lsb <= 31 (W) / 63 (X). Documented bounds sampled at -1, 0-1 (via valid), R, R+1. Negative/error contract. Generator pins the documented edges.
- Seed: bitfield.rs encode_bfi_neg_lsb_width
- Formal: ∀ is_64, rd,rn,rm ∈ 0..31, lsb ∉ 0..(R-1). encode_extr([Rd,Rn,Rm,Imm(lsb)]) = Err (no panic)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: extr w0, w0, w0, #-1 panics or encodes instead of Err (lsb=-1, R=32)
- Bug report: pbt-out/bug_reports/encode_extr_lsb.md

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, rm, lsb]
  domain: { lsb in {-1, R, R+1, -2, 128} union values outside 0..R-1 }
  relation:
    op: throws
    lhs: encode_extr([Reg(Rd), Reg(Rn), Reg(Rm), Imm(lsb)])
    error: Err
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: -2, max: 128, type: i64 }
expected_error: Err
evidence: llvm-mc "immediate must be an integer in range [0, 31]" / "[0, 63]"; ARM ARM Extract imms range; assembler README.md:11
```

## encode_extr_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — GNU/llvm-mc accept x31/w31, XZR/WZR, LR, and uppercase as aliases of the same GPR encoding. Differential vs llvm-mc. Documented by assembler README.md:11.
- Seed: bitfield.rs encode_clz_diff_alt_spellings
- Formal: ∀ valid EXTR, spellings ∈ {x31/w31, XZR/WZR, LR, uppercase, canonical}. encode_extr(spelled ops) = llvm-mc(spelled asm)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, rm, lsb, dest_spell, src_n_spell, src_m_spell]
  domain: { valid EXTR plus alt spellings }
  relation:
    op: eq
    lhs: encode_extr(spelled ops)
    rhs: llvm_mc(spelled asm)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: assembler README.md:11; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_extr_neg_mixed_width
- Tier: 4e
- Rationale: Sweep — llvm-mc rejects mixed W/X among Rd/Rn/Rm. ARM EXTR requires all three registers the same size. Negative/error contract.
- Seed: bitfield.rs encode_bfi_neg_mixed_width
- Formal: ∀ rd,rn,rm ∈ 0..31, rd64,rn64,rm64 ∈ bool not all equal. encode_extr mixed W/X = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: extr w0, x0, w0, #0 encodes instead of Err (rd64=false, rn64=true, rm64=false)
- Bug report: pbt-out/bug_reports/encode_extr_mixed_width.md

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { not all of rd64,rn64,rm64 equal }
  relation:
    op: throws
    lhs: encode_extr(mixed W/X)
    error: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: Err
evidence: llvm-mc rejects mixed W/X EXTR; ARM ARM Extract all registers same size; assembler README.md:11
```

## encode_extr_neg_fp
- Tier: 4e
- Rationale: Sweep — llvm-mc rejects FP/SIMD prefixes (d/s/q/v/h/b) as EXTR operands. ARM EXTR is GPR-only. Negative/error contract.
- Seed: bitfield.rs encode_bfi_neg_fp
- Formal: ∀ which ∈ {0,1,2}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_extr with FP at slot which = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: extr d0, x1, x2, #0 encodes instead of Err (which=0, prefix=d, n=0)
- Bug report: pbt-out/bug_reports/encode_extr_fp.md

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..2, prefix: d|s|q|v|h|b, n: 0..31 }
  relation:
    op: throws
    lhs: encode_extr(FP at which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: Err
evidence: llvm-mc rejects FP/SIMD EXTR operands; ARM ARM Extract GPR-only; assembler README.md:11
```

## encode_extr_neg_nonreg
- Tier: 4e
- Rationale: Sweep — get_reg/get_imm require Reg at 0..2 and Imm at 3. Wrong kinds must Err. Documented by encoder helper contracts used by EXTR.
- Seed: bitfield.rs encode_clz_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad kind (not Imm when which=3). encode_extr with wrong kind at which = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: non-matching Operand kind }
  relation:
    op: throws
    lhs: encode_extr(wrong kind at which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: Err
evidence: encoder/mod.rs:956 get_reg expected register; encoder/mod.rs:968 get_imm expected immediate
```

## encode_extr_neg_invalid_name
- Tier: 4e
- Rationale: Sweep — parse_reg_num returns None for foo/x32/empty/r0. get_reg must Err. Negative/error contract.
- Seed: bitfield.rs encode_clz_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_extr with invalid name at which = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_extr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..2, name: invalid GPR spelling }
  relation:
    op: throws
    lhs: encode_extr(invalid name at which)
    error: Err
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
expected_error: Err
evidence: encoder/mod.rs:131 parse_reg_num; llvm-mc rejects invalid register names
```
