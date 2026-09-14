# Properties: encode_ubfx

## encode_ubfx_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree UBFX decoder). Sibling encode_ubfm/encode_ubfiz rejected as differential (same-job gate: raw immr/imms vs insert alias vs extract alias). Sibling encode_sbfx/encode_bfxil rejected (opc 00/01, different instruction). Doc evidence: README.md:11 GNU-style assembly; encoder/mod.rs:885 dispatch; ARM ARM Unsigned Bitfield Extract UBFX.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_diff_valid_gpr (signed twin)
- Formal: ∀ is_64 ∈ {false,true}, Rd,Rn ∈ 0..31, lsb,width with 0≤lsb<R, 1≤width≤R-lsb (R=64 if is_64 else 32). encode_ubfx([Reg(gpr(is_64,Rd)), Reg(gpr(is_64,Rn)), Imm(lsb), Imm(width)]) = llvm-mc("ubfx gpr(is_64,Rd), gpr(is_64,Rn), #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb, R: 64 if is_64 else 32 }
  relation:
    op: eq
    lhs: "encode_ubfx([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])"
    rhs: "llvm_mc_word(format!(\"ubfx {}, {}, #{}, #{}\", gpr(is_64,rd), gpr(is_64,rn), lsb, width))"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:885; ARM ARM UBFX alias of UBFM
```

## encode_ubfx_alias_ubfm
- Tier: 4c
- Rationale: Algebraic metamorphic alias. ARM ARM and bitfield.rs:6 state UBFX Rd,Rn,#lsb,#width is UBFM Rd,Rn,#lsb,#(lsb+width-1). encode_ubfm is a same-encoding sibling after that mapping (not a same-job differential). Stronger differential already used above.
- Seed: bitfield.rs encode_ubfm_pbt encode_ubfm_alias_ubfx
- Formal: ∀ valid (is_64,Rd,Rn,lsb,width) with 0≤lsb<R, 1≤width≤R-lsb. encode_ubfx([Rd,Rn,lsb,width]) = encode_ubfm([Rd,Rn,lsb,lsb+width-1])
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: "encode_ubfx([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])"
    rhs: "encode_ubfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(lsb+width-1)])"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: bitfield.rs:6 purpose comment; ARM ARM UBFX alias of UBFM
```

## encode_ubfx_arm_fields
- Tier: 4d
- Rationale: Algebraic invariant from ARM ARM Bitfield Move UBFM encoding (UBFX alias). Success-path word is sf 10 100110 N immr imms Rn Rd with N=sf, immr=lsb, imms=lsb+width-1. Stronger differential already used.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_arm_fields
- Formal: ∀ valid (is_64,Rd,Rn,lsb,width). let w = encode_ubfx(...). w[31]=sf, w[30:29]=10, w[28:23]=100110, w[22]=sf, w[21:16]=lsb, w[15:10]=lsb+width-1, w[9:5]=Rn, w[4:0]=Rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: 0..R-1, width: 1..R-lsb }
  relation:
    op: eq
    lhs: "encode_ubfx([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])"
    rhs: "(sf<<31)|(0b10<<29)|(0b100110<<23)|(sf<<22)|(lsb<<16)|((lsb+width-1)<<10)|(rn<<5)|rd"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: ARM ARM Bitfield Move UBFM/UBFX; bitfield.rs:14 comment UBFM sf 10 100110 N immr imms Rn Rd
```

## encode_ubfx_metamorphic_rd_rn
- Tier: 4c
- Rationale: ARM ARM field layout: Rd is bits[4:0], Rn is bits[9:5]. Changing one register must not change other fields. Stronger differential already used.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_metamorphic_rd_rn
- Formal: ∀ is_64, Rd,Rn ∈ 0..30, lsb∈{0,1}, width=1. encode_ubfx(Rd+1,Rn) & ~0x1f = encode_ubfx(Rd,Rn) & ~0x1f ∧ encode_ubfx(Rd,Rn+1) & ~(0x1f<<5) = encode_ubfx(Rd,Rn) & ~(0x1f<<5)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb]
  domain: { is_64: bool, rd: 0..30, rn: 0..30, lsb: {0,1}, width: 1 }
  relation:
    op: holds
    expr: "(encode_ubfx(rd+1,rn) & !0x1f) == (encode_ubfx(rd,rn) & !0x1f) && ((encode_ubfx(rd,rn+1) >> 5) & 0x1f) == rn+1"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  lsb: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Bitfield Move Rd bits[4:0] Rn bits[9:5]
```

## encode_ubfx_neg_arity
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects UBFX with fewer than 4 operands ("too few operands"). get_reg/get_imm fail when the slot is missing. Documented by llvm-mc and README GNU-style assembly.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_arity
- Formal: ∀ len ∈ 0..3, valid Rd,Rn. encode_ubfx(ops[0..len]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: 0..3, is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: throws
    expr: "encode_ubfx(truncate([Reg(Rd),Reg(Rn),Imm(0),Imm(1)], len))"
expected_error: String
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects too few operands; README.md:11 GNU-style assembly
```

## encode_ubfx_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects a 5th UBFX operand. GNU-style assembly (README.md:11) has no 5th operand for UBFX. encode_ubfx currently ignores extra operands (no len check) — expected to fail.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_extra_operand
- Formal: ∀ valid UBFX ops, extra operand. encode_ubfx(ops ++ [extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: ubfx w0, w0, #0, #1 plus extra Reg("x0") — encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_ubfx_extra_operand.md

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, extra]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    expr: "encode_ubfx([Rd,Rn,lsb,width,extra])"
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
evidence: llvm-mc rejects extra operand; README.md:11
```

## encode_ubfx_neg_sp
- Tier: 4e
- Rationale: Negative/error contract. ARM ARM register 31 is ZR not SP for UBFM/UBFX. llvm-mc rejects sp/wsp as UBFX operands. parse_reg_num maps sp/wsp to 31, so SUT currently encodes them as ZR.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_ubfx with slot `which` = SP = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp=wsp, is_64=false, other=0 — UBFX wsp, w0, #0, #1 encodes as wzr
- Bug report: pbt-out/bug_reports/encode_ubfx_sp.md

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: 0..1, sp64: bool, is_64: bool, other: 0..30 }
  relation:
    op: throws
    expr: "encode_ubfx(ops with slot which = SP/WSP)"
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM UBFM/UBFX Rd/Rn are ZR not SP; llvm-mc rejects sp/wsp
```

## encode_ubfx_neg_lsb_width
- Tier: 4e
- Rationale: Negative/error contract. ARM ARM constraints 0≤lsb<datasize, 1≤width≤datasize-lsb. llvm-mc rejects width=0, negative, lsb>=R, width>R-lsb. SUT casts i64 as u32 with no range check and computes lsb+width-1 (debug underflow on width=0).
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_lsb_width
- Formal: ∀ lsb,width outside 0≤lsb<R ∧ 1≤width≤R-lsb. encode_ubfx(...) is Err (or does not panic)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, lsb=0, width=0 — debug panic (subtract overflow) instead of Err
- Bug report: pbt-out/bug_reports/encode_ubfx_lsb_width.md

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, lsb: i64, width: i64 }
  relation:
    op: throws
    expr: "encode_ubfx([Rd,Rn,Imm(lsb),Imm(width)])"
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: -1, max: 65, type: i64 }
  width: { gen: int, min: -1, max: 65, type: i64 }
evidence: ARM ARM UBFX 0<=lsb<datasize, 1<=width<=datasize-lsb; llvm-mc rejects out-of-range
```

## encode_ubfx_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for alternate register spellings (x31/w31, XZR/WZR, LR, uppercase). Sweep of documented GNU-style aliases. Stronger oracles already used on canonical names.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_diff_alt_spellings
- Formal: ∀ valid (is_64,Rd,Rn,lsb,width), dest_spell,src_spell ∈ 0..4. encode_ubfx([Reg(spell(Rd)), Reg(spell(Rn)), Imm(lsb), Imm(width)]) = llvm-mc("ubfx spell(Rd), spell(Rn), #lsb, #width")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, lsb, width, dest_spell, src_spell]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, dest_spell: 0..4, src_spell: 0..4 }
  relation:
    op: eq
    lhs: "encode_ubfx([Reg(spell(is_64,rd,dest_spell)), Reg(spell(is_64,rn,src_spell)), Imm(lsb), Imm(width)])"
    rhs: "llvm_mc_word(format!(\"ubfx {}, {}, #{}, #{}\", spell(is_64,rd,dest_spell), spell(is_64,rn,src_spell), lsb, width))"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style assembly; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_ubfx_neg_mixed_width
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects mixed W/X UBFX (e.g. x0, w1). GNU-style assembly requires matching register width. encode_ubfx takes sf from Rd only.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_mixed_width
- Formal: ∀ Rd,Rn ∈ 0..31, rd64 ≠ rn64. encode_ubfx([Reg(gpr(rd64,Rd)), Reg(gpr(rn64,Rn)), Imm(0), Imm(1)]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — ubfx x0, w0, #0, #1 encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_ubfx_mixed_width.md

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd: 0..31, rn: 0..31, rd64: bool, rn64: bool }
  relation:
    op: throws
    expr: "encode_ubfx([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(1)])"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
evidence: llvm-mc rejects mixed W/X; README.md:11 GNU-style assembly
```

## encode_ubfx_neg_fp
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects FP/SIMD prefixes (d/s/q/v/h/b) as UBFX operands. parse_reg_num accepts those prefixes as GPR numbers.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_ubfx with slot which = prefix+n = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — ubfx d0, x1, #0, #1 encodes as w0
- Bug report: pbt-out/bug_reports/encode_ubfx_fp.md

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: [d,s,q,v,h,b], n: 0..31 }
  relation:
    op: throws
    expr: "encode_ubfx(ops with slot which = format!(\"{}{}\", prefix, n))"
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects FP/SIMD UBFX operands; README.md:11
```

## encode_ubfx_neg_nonreg
- Tier: 4e
- Rationale: Negative/error contract. Slots 0-1 must be registers; slots 2-3 must be immediates. Shift/Mem/Label/Symbol/Cond/RegArrangement are invalid. get_reg/get_imm already Err on wrong kind.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_nonreg
- Formal: ∀ which ∈ 0..3, bad operand of wrong kind for that slot. encode_ubfx(...) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..3, bad: Operand }
  relation:
    op: throws
    expr: "encode_ubfx(ops with slot which = bad)"
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: get_reg/get_imm error on wrong kind; llvm-mc rejects non-reg/non-imm
```

## encode_ubfx_neg_invalid_name
- Tier: 4e
- Rationale: Negative/error contract. Invalid register names (foo, x32, empty, r0) must Err. parse_reg_num returns None.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_ubfx with slot which = name = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ubfx
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid register name }
  relation:
    op: throws
    expr: "encode_ubfx(ops with slot which = Reg(name))"
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: parse_reg_num returns None for invalid names; llvm-mc rejects them
```
