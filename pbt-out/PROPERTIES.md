# Properties: encode_sbfx

## encode_sbfx_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. State machine rejected (pure function). Round-trip rejected (no in-tree SBFX decoder). Sibling encode_sbfm rejected as differential (same-job gate: raw immr/imms vs alias lsb/width). Sibling encode_ubfx/encode_bfxil rejected (different opc). Doc evidence: README.md:11 GNU-style assembly; encoder/mod.rs:886 dispatch; ARM ARM Signed Bitfield Extract SBFX.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, Rd,Rn ∈ [0,31], lsb ∈ [0,R), width ∈ [1,R-lsb] where R=64 if is_64 else 32. encode_sbfx([Reg(gpr(is_64,Rd)), Reg(gpr(is_64,Rn)), Imm(lsb), Imm(width)]) = Word(llvm-mc("sbfx gpr(Rd), gpr(Rn), #lsb, #width"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    lsb: 0..R-1
    width: 1..R-lsb
  relation:
    op: eq
    lhs: "encode_sbfx([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])"
    rhs: "llvm_mc(sbfx gpr(is_64,rd), gpr(is_64,rn), #lsb, #width)"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: README.md:11 GNU-style assembly; encoder/mod.rs:886; ARM ARM SBFX alias of SBFM
```

## encode_sbfx_alias_sbfm
- Tier: 4
- Rationale: Algebraic metamorphic alias. ARM ARM and bitfield.rs:21 state SBFX Rd,Rn,#lsb,#width is SBFM Rd,Rn,#lsb,#(lsb+width-1). encode_sbfm is a same-encoding sibling after that mapping (not a same-job differential). Stronger differential already used above.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_alias_bfm
- Formal: ∀ valid (is_64,Rd,Rn,lsb,width). encode_sbfx([Rd,Rn,lsb,width]) = encode_sbfm([Rd,Rn,lsb,lsb+width-1])
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    lsb: 0..R-1
    width: 1..R-lsb
  relation:
    op: eq
    lhs: "encode_sbfx([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])"
    rhs: "encode_sbfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(lsb+width-1)])"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: bitfield.rs:21 SBFX maps to SBFM lsb, lsb+width-1; ARM ARM SBFX alias of SBFM
```

## encode_sbfx_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM Bitfield Move SBFM encoding (SBFX alias): sf 00 100110 N immr imms Rn Rd with N=sf, immr=lsb, imms=lsb+width-1. Weaker than differential; pins each field independently.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_arm_fields
- Formal: ∀ valid (is_64,Rd,Rn,lsb,width). let w = encode_sbfx(...). w[31]=sf, w[30:29]=00, w[28:23]=100110, w[22]=sf, w[21:16]=lsb, w[15:10]=lsb+width-1, w[9:5]=Rn, w[4:0]=Rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    lsb: 0..R-1
    width: 1..R-lsb
  relation:
    op: eq
    lhs: "encode_sbfx([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])"
    rhs: "(sf<<31)|(0b100110<<23)|(sf<<22)|(lsb<<16)|((lsb+width-1)<<10)|(rn<<5)|rd"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM Bitfield Move SBFM encoding sf 00 100110 N immr imms Rn Rd
```

## encode_sbfx_metamorphic_rd_rn
- Tier: 4
- Rationale: Algebraic metamorphic: Rd lives only in bits[4:0] and Rn only in bits[9:5]. ARM ARM field layout. Weaker than differential.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_metamorphic_rd_rn
- Formal: ∀ is_64, Rd,Rn ∈ [0,30], lsb∈{0,1}, width=1. encode_sbfx(Rd+1,...) differs only in bits[4:0]; encode_sbfx(...,Rn+1,...) differs only in bits[9:5]
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb]
  domain:
    is_64: bool
    rd: 0..30
    rn: 0..30
    lsb: 0..1
  relation:
    op: holds
    expr: "(encode_sbfx(rd+1,rn,lsb,1) & 0x1f == rd+1) && (encode_sbfx(rd+1,rn,lsb,1) & ~0x1f == encode_sbfx(rd,rn,lsb,1) & ~0x1f) && ((encode_sbfx(rd,rn+1,lsb,1)>>5) & 0x1f == rn+1)"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  lsb: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM SBFM Rd at bits 4:0, Rn at bits 9:5
```

## encode_sbfx_neg_arity
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects SBFX with fewer than 4 operands. README.md:11 GNU-style assembly. get_reg/get_imm return Err on missing slots.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_arity
- Formal: ∀ len ∈ [0,3], valid dummy regs. encode_sbfx(ops[0..len]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
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
    expr: "encode_sbfx(truncate([Reg(rd), Reg(rn), Imm(0), Imm(1)], len))"
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects sbfx with too few operands; README.md:11
```

## encode_sbfx_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 5th SBFX operand. README.md:11. Documented bound: exactly 4 operands.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_extra_operand
- Formal: ∀ valid 4-operand SBFX and extra operand. encode_sbfx(ops ++ [extra]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: [Reg("w0"), Reg("w0"), Imm(0), Imm(1), Reg("x0")]
- Bug report: pbt-out/bug_reports/encode_sbfx_extra_operand.md

```property
function: encoder.bitfield.encode_sbfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, extra]
  domain:
    is_64: bool
    rd: 0..31
    extra: Operand
  relation:
    op: throws
    expr: "encode_sbfx(ops4 ++ [extra])"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof }
expected_error: String
evidence: llvm-mc rejects sbfx x0, x1, #0, #1, x2; README.md:11
```

## encode_sbfx_neg_sp
- Tier: 4
- Rationale: Negative/error contract. ARM ARM SBFX register 31 is ZR not SP. llvm-mc rejects sbfx sp and sbfx wsp.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_sbfx with slot which = SP/WSP is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0 — sbfx wsp, w0, #0, #1
- Bug report: pbt-out/bug_reports/encode_sbfx_sp.md

```property
function: encoder.bitfield.encode_sbfx
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
    expr: "encode_sbfx(ops with slot which = SP/WSP)"
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand for sbfx sp / wsp; ARM ARM SBFX Rd/Rn are W/X not SP
```

## encode_sbfx_neg_lsb_width
- Tier: 4
- Rationale: Negative/error contract. ARM ARM 0<=lsb<R and 1<=width<=R-lsb. llvm-mc rejects width=0, lsb=-1, lsb=R, width=R-lsb+1. Documented bounds sampled at bound and bound plus/minus 1.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_lsb_width
- Formal: ∀ (lsb,width) outside 0<=lsb<R ∧ 1<=width<=R-lsb. encode_sbfx(...) is Err (no panic)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, lsb=0, width=0 — sbfx w0, w0, #0, #0 (debug overflow at bitfield.rs:30)
- Bug report: pbt-out/bug_reports/encode_sbfx_lsb_width.md

```property
function: encoder.bitfield.encode_sbfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain:
    is_64: bool
    lsb: invalid
    width: invalid
  relation:
    op: throws
    expr: "encode_sbfx([Reg(rd), Reg(rn), Imm(lsb), Imm(width)])"
generators:
  is_64: { gen: bool }
  lsb: { gen: int, min: -1, max: 65, type: i64 }
  width: { gen: int, min: -1, max: 65, type: i64 }
expected_error: String
evidence: ARM ARM SBFX 0<=lsb<R, 1<=width<=R-lsb; llvm-mc rejects width 0 and overflow extract
```

## encode_sbfx_diff_alt_spellings
- Tier: 2
- Rationale: Strengthening / contract-surface sweep. Differential vs llvm-mc for x31/w31, XZR/WZR, LR, and uppercase spellings. Same oracle as encode_sbfx_diff_valid_gpr.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_diff_alt_spellings
- Formal: ∀ valid (is_64,Rd,Rn,lsb,width) and spelling kinds in {0..4}. encode_sbfx([Reg(spell(Rd)), Reg(spell(Rn)), Imm(lsb), Imm(width)]) = Word(llvm-mc("sbfx spell(Rd), spell(Rn), #lsb, #width"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, dest_spell, src_spell]
  domain:
    is_64: bool
    rd: 0..31
    dest_spell: 0..4
    src_spell: 0..4
  relation:
    op: eq
    lhs: "encode_sbfx([Reg(spell(dest)), Reg(spell(src)), Imm(lsb), Imm(width)])"
    rhs: "llvm_mc(sbfx spell(dest), spell(src), #lsb, #width)"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style assembly; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_sbfx_neg_mixed_width
- Tier: 4
- Rationale: Strengthening. Negative/error contract. llvm-mc rejects mixed W/X (`sbfx x0, w0, #0, #1`). README.md:11.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_mixed_width
- Formal: ∀ Rd,Rn ∈ [0,31], rd64 ≠ rn64. encode_sbfx([Reg(gpr(rd64,Rd)), Reg(gpr(rn64,Rn)), Imm(0), Imm(1)]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — sbfx x0, w0, #0, #1
- Bug report: pbt-out/bug_reports/encode_sbfx_mixed_width.md

```property
function: encoder.bitfield.encode_sbfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain:
    rd: 0..31
    rn: 0..31
    rd64: bool
    rn64: bool
  relation:
    op: throws
    expr: "encode_sbfx([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)]) where rd64 != rn64"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: String
evidence: llvm-mc invalid operand for sbfx x0, w0; README.md:11
```

## encode_sbfx_neg_fp
- Tier: 4
- Rationale: Strengthening / contract-surface sweep. Negative/error contract. llvm-mc rejects S/D/Q/V/H/B as SBFX operands.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ [0,31]. encode_sbfx with slot which = prefix+n is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — sbfx d0, x1, #0, #1
- Bug report: pbt-out/bug_reports/encode_sbfx_fp.md

```property
function: encoder.bitfield.encode_sbfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain:
    which: 0..1
    prefix: d|s|q|v|h|b
    n: 0..31
  relation:
    op: throws
    expr: "encode_sbfx(ops with slot which = FP register prefix+n)"
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand for sbfx d0; ARM ARM SBFX is GPR-only
```

## encode_sbfx_neg_nonreg
- Tier: 4
- Rationale: Strengthening / contract-surface sweep. Negative/error contract. Wrong operand kinds (Imm/Shift/Mem/Label/Symbol/Cond/RegArrangement) in any of the four slots must Err (Imm allowed only in slots 2-3).
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_nonreg
- Formal: ∀ which ∈ [0,3], bad kind not matching the slot. encode_sbfx(...) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..3
    bad: Operand
  relation:
    op: throws
    expr: "encode_sbfx(ops with slot which = bad)"
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: get_reg/get_imm type checks; llvm-mc rejects non-register/non-imm
```

## encode_sbfx_neg_invalid_name
- Tier: 4
- Rationale: Strengthening / contract-surface sweep. Negative/error contract. Invalid register names (foo, x32, w32, empty, r0, x, x-1, x99, w) must Err.
- Seed: bitfield.rs encode_bfxil_pbt encode_bfxil_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ invalid set. encode_sbfx with slot which = name is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_sbfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain:
    which: 0..1
    name: invalid register name
  relation:
    op: throws
    expr: "encode_sbfx(ops with slot which = Reg(name))"
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: parse_reg_num returns None for foo/x32/empty; llvm-mc rejects them
```
