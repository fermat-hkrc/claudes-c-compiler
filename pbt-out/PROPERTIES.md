# Properties: encode_ubfm

## encode_ubfm_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree UBFM decoder). Sibling encode_ubfx/encode_ubfiz rejected as differential (same-job gate: alias lsb/width vs raw immr/imms). Sibling encode_sbfm/encode_bfm rejected (opc 00/01, different instruction). Doc evidence: README.md:11 GNU-style assembly; encoder/mod.rs:887 dispatch; ARM ARM Bitfield Move UBFM.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, Rd,Rn ∈ [0,31], immr,imms ∈ [0,R) where R=64 if is_64 else 32. encode_ubfm([Reg(gpr(is_64,Rd)), Reg(gpr(is_64,Rn)), Imm(immr), Imm(imms)]) = Word(llvm-mc("ubfm gpr(Rd), gpr(Rn), #immr, #imms"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
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
    lhs: "encode_ubfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(immr), Imm(imms)])"
    rhs: "llvm_mc(ubfm gpr(is_64,rd), gpr(is_64,rn), #immr, #imms)"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
evidence: README.md:11 GNU-style assembly; encoder/mod.rs:887; ARM ARM UBFM sf 10 100110 N immr imms Rn Rd
```

## encode_ubfm_alias_ubfx
- Tier: 4
- Rationale: Algebraic metamorphic alias. ARM ARM and bitfield.rs:6 state UBFX Rd,Rn,#lsb,#width is UBFM Rd,Rn,#lsb,#(lsb+width-1). encode_ubfx is a same-encoding sibling after that mapping (not a same-job differential). Stronger differential already used above.
- Seed: bitfield.rs encode_sbfx_pbt encode_sbfx_alias_sbfm
- Formal: ∀ valid (is_64,Rd,Rn,lsb,width) with 0<=lsb<R, 1<=width<=R-lsb. encode_ubfm([Rd,Rn,lsb,lsb+width-1]) = encode_ubfx([Rd,Rn,lsb,width])
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
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
    lhs: "encode_ubfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(lsb+width-1)])"
    rhs: "encode_ubfx([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(lsb), Imm(width)])"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  lsb: { gen: int, min: 0, max: 63, type: u32 }
  width: { gen: int, min: 1, max: 64, type: u32 }
evidence: bitfield.rs:6 UBFX maps to UBFM lsb, lsb+width-1; ARM ARM UBFX alias of UBFM
```

## encode_ubfm_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM Bitfield Move UBFM encoding: sf 10 100110 N immr imms Rn Rd with N=sf. Weaker than differential; pins each field independently. Documented bounds 0 and R-1 are sampled exactly.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_arm_fields
- Formal: ∀ valid (is_64,Rd,Rn,immr,imms). let w = encode_ubfm(...). w[31]=sf, w[30:29]=10, w[28:23]=100110, w[22]=sf, w[21:16]=immr, w[15:10]=imms, w[9:5]=Rn, w[4:0]=Rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
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
    lhs: "encode_ubfm([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(immr), Imm(imms)])"
    rhs: "(sf<<31)|(0b10<<29)|(0b100110<<23)|(sf<<22)|(immr<<16)|(imms<<10)|(rn<<5)|rd"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Bitfield Move UBFM sf 10 100110 N immr imms Rn Rd; bitfield.rs:36 purpose comment
```

## encode_ubfm_metamorphic_rd_rn
- Tier: 4
- Rationale: Algebraic metamorphic field independence. ARM ARM places Rd in bits[4:0] and Rn in bits[9:5]; changing one register must not alter opcode/imm/the other register. Stronger differential already used.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_metamorphic_rd_rn
- Formal: ∀ is_64, Rd,Rn ∈ [0,30], imm ∈ {0,1} with imm<R. encode_ubfm(Rd+1,Rn,imm,imm) differs from encode_ubfm(Rd,Rn,imm,imm) only in bits[4:0]; encode_ubfm(Rd,Rn+1,imm,imm) differs only in bits[9:5]
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, imm]
  domain:
    is_64: bool
    rd: 0..30
    rn: 0..30
    imm: 0..1
  relation:
    op: holds
    expr: "(encode_ubfm(rd+1,rn) & !0x1f) == (encode_ubfm(rd,rn) & !0x1f) && (encode_ubfm(rd,rn+1) & !(0x1f<<5)) == (encode_ubfm(rd,rn) & !(0x1f<<5))"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM UBFM Rd bits[4:0] Rn bits[9:5]; encoder/mod.rs bit packing
```

## encode_ubfm_neg_arity
- Tier: 4
- Rationale: Negative/error contract. llvm-mc and GNU as reject UBFM with fewer than 4 operands ("too few operands"). get_reg/get_imm return Err on missing index. Documented by llvm-mc error and README.md:11 gas compatibility.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_arity
- Formal: ∀ len ∈ [0,3], valid dummy regs. encode_ubfm(ops[0..len]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
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
    expr: "encode_ubfm(ops.truncate(len))"
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands"; README.md:11 gas-compatible assembly; get_reg/get_imm Err on missing idx
```

## encode_ubfm_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 5th UBFM operand ("invalid operand for instruction"). README.md:11 requires gas-compatible rejection. ARM ARM UBFM has exactly Rd, Rn, #immr, #imms.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_extra_operand
- Formal: ∀ valid 4-operand UBFM ops, extra ∈ Operand. encode_ubfm(ops ++ [extra]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: ops = [Reg("w0"), Reg("w0"), Imm(0), Imm(0), Reg("x0")]; encode_ubfm returns Ok(Word) instead of Err
- Bug report: pbt-out/bug_reports/encode_ubfm_extra_operand.md

```property
function: encoder.bitfield.encode_ubfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms, extra]
  domain:
    valid_ubfm: true
    extra: Operand
  relation:
    op: throws
    expr: "encode_ubfm(ops4 ++ [extra])"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
expected_error: String
evidence: llvm-mc rejects 5th operand; ARM ARM UBFM arity 4; README.md:11
```

## encode_ubfm_neg_sp
- Tier: 4
- Rationale: Negative/error contract. ARM ARM Bitfield Move uses ZR for register 31, not SP. llvm-mc: "invalid operand for instruction" on sp/wsp as Rd or Rn. README.md:11 gas-compatible.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_ubfm with ops[which]=Reg(sp) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp=wsp, is_64=false, other=0 — encode_ubfm([Reg("wsp"), Reg("w0"), Imm(0), Imm(0)]) returns Ok(Word) encoding Rd=31
- Bug report: pbt-out/bug_reports/encode_ubfm_sp.md

```property
function: encoder.bitfield.encode_ubfm
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
    expr: "encode_ubfm(ops with slot which = SP/WSP)"
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM UBFM register 31 is ZR not SP; llvm-mc "invalid operand"; README.md:11
```

## encode_ubfm_neg_immr_imms
- Tier: 4
- Rationale: Negative/error contract. ARM ARM UBFM requires 0<=immr,imms<datasize. llvm-mc: "immediate must be an integer in range [0, 31]" (or [0, 63]). Bounds R and R-1 and -1 are sampled exactly. README.md:11 gas-compatible.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_immr_imms
- Formal: ∀ is_64, Rd,Rn ∈ [0,31], (immr,imms) with not (0<=immr<R ∧ 0<=imms<R). encode_ubfm(...) is Err (no panic)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, immr=-1, imms=0 — encode_ubfm returns Ok(Word) via `as u32` wrap of -1 instead of Err
- Bug report: pbt-out/bug_reports/encode_ubfm_immr_imms.md

```property
function: encoder.bitfield.encode_ubfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, immr, imms]
  domain:
    is_64: bool
    rd: 0..31
    rn: 0..31
    immr_imms: invalid relative to R
  relation:
    op: throws
    expr: "encode_ubfm([Reg,Reg,Imm(immr),Imm(imms)])"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: -1, max: 65, type: i64 }
  imms: { gen: int, min: -1, max: 65, type: i64 }
expected_error: String
evidence: ARM ARM UBFM 0<=immr,imms<datasize; llvm-mc range error; README.md:11
```

## encode_ubfm_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for alternate GPR spellings (x31/w31, XZR/WZR, LR, uppercase). parse_reg_num lowercases and maps lr to 30. README.md:11 gas-compatible. Sweep: documented spelling aliases not covered by the canonical gpr() generator.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_diff_alt_spellings
- Formal: ∀ valid (is_64,Rd,Rn,immr,imms), dest_spell,src_spell ∈ [0,4]. encode_ubfm([Reg(spell(Rd)), Reg(spell(Rn)), Imm(immr), Imm(imms)]) = Word(llvm-mc("ubfm spell(Rd), spell(Rn), #immr, #imms"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
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
    lhs: "encode_ubfm([Reg(spell(rd)), Reg(spell(rn)), Imm(immr), Imm(imms)])"
    rhs: "llvm_mc(ubfm spell(rd), spell(rn), #immr, #imms)"
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  immr: { gen: int, min: 0, max: 63, type: u32 }
  imms: { gen: int, min: 0, max: 63, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11; parse_reg_num maps lr/XZR/x31; llvm-mc accepts those spellings
```

## encode_ubfm_neg_mixed_width
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects mixed W/X ("invalid operand for instruction"). ARM ARM UBFM requires Rd and Rn the same datasize (sf from both). README.md:11 gas-compatible. Sweep: encode_ubfm takes sf from Rd only and never checks Rn width.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_mixed_width
- Formal: ∀ Rd,Rn ∈ [0,31], rd64 ≠ rn64. encode_ubfm([Reg(gpr(rd64,Rd)), Reg(gpr(rn64,Rn)), Imm(0), Imm(0)]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false — encode_ubfm([Reg("x0"), Reg("w0"), Imm(0), Imm(0)]) returns Ok(Word)
- Bug report: pbt-out/bug_reports/encode_ubfm_mixed_width.md

```property
function: encoder.bitfield.encode_ubfm
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
    expr: "encode_ubfm([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Imm(0), Imm(0)]) where rd64 != rn64"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: String
evidence: llvm-mc mixed W/X invalid operand; ARM ARM UBFM same datasize; README.md:11
```

## encode_ubfm_neg_fp
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects d/s/q/v/h/b registers as UBFM operands. ARM ARM UBFM is a GPR instruction. parse_reg_num accepts those prefixes; encode_ubfm does not call is_fp_reg. README.md:11 gas-compatible. Sweep: FP prefix path untested by GPR generators.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ [0,31]. encode_ubfm with ops[which]=Reg(prefix+n) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 — encode_ubfm([Reg("d0"), Reg("x1"), Imm(0), Imm(0)]) returns Ok(Word)
- Bug report: pbt-out/bug_reports/encode_ubfm_fp.md

```property
function: encoder.bitfield.encode_ubfm
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
    expr: "encode_ubfm(ops with slot which = FP/SIMD register)"
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc FP invalid operand; ARM ARM UBFM GPR-only; README.md:11
```

## encode_ubfm_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract. get_reg requires Operand::Reg at slots 0-1; get_imm requires Operand::Imm at slots 2-3. llvm-mc rejects non-register/non-imm kinds. Sweep: wrong-kind operands.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_nonreg
- Formal: ∀ which ∈ [0,3], bad ∈ {Imm (slots 0-1 only), Shift, Mem, Label, Symbol, Cond, RegArrangement}. encode_ubfm with ops[which]=bad is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain:
    which: 0..3
    bad: non-matching Operand kind
  relation:
    op: throws
    expr: "encode_ubfm(ops with slot which = bad kind)"
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: get_reg/get_imm type errors; llvm-mc invalid operand; README.md:11
```

## encode_ubfm_neg_invalid_name
- Tier: 4
- Rationale: Negative/error contract. parse_reg_num returns None for foo/x32/w32/empty/r0/x/x-1/x99/w. get_reg then Err("invalid register"). Sweep: invalid names vs the 0..=31 GPR generator.
- Seed: bitfield.rs encode_sbfm_pbt encode_sbfm_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,x32,w32,x,r0,"",x-1,x99,w}. encode_ubfm with ops[which]=Reg(name) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_ubfm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain:
    which: 0..1
    name: invalid register names
  relation:
    op: throws
    expr: "encode_ubfm(ops with slot which = Reg(invalid name))"
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: parse_reg_num None; llvm-mc unknown token / invalid operand; README.md:11
```
