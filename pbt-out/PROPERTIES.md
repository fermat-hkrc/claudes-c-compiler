# Properties: encode_rev16

## encode_rev16_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree REV16 decoder). Sibling encode_neon_two_misc rejected for scalar (vector class; dispatch splits). Sibling encode_rev/encode_rev32/encode_rbit/encode_clz/encode_cls rejected (different opcode). Doc evidence: README.md:11 GNU-style assembly; README.md:240 lists rev16; encoder/mod.rs:576-578 dispatch; ARM ARM Data-processing (1 source) REV16.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31. encode_rev16([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))]) = Word(llvm-mc("rev16 gpr(is_64,rd), gpr(is_64,rn)"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rev16
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_rev16([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))])
    rhs: llvm_mc("rev16 " + gpr(is_64,rd) + ", " + gpr(is_64,rn))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:11 GNU-style assembly; ARM ARM REV16 sf 1 0 11010110 00000 000001 Rn Rd
```

## encode_rev16_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM encoding, weaker than differential (already used above) but pins each field independently. Stronger rejected as above. Doc evidence: ARM ARM Data-processing (1 source) REV16 opcode=000001.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_arm_fields
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31. let w = encode_rev16([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))]). w = (sf<<31)|(1<<30)|(0b011010110<<21)|(0b000001<<10)|(rn<<5)|rd ∧ w[31]=sf ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=11010110 ∧ w[20:16]=00000 ∧ w[15:10]=000001 ∧ w[9:5]=rn ∧ w[4:0]=rd where sf = [is_64]
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rev16
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_rev16([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))])
    rhs: (sf(is_64)<<31)|(1<<30)|(0b011010110<<21)|(0b000001<<10)|(rn<<5)|rd
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (1 source) REV16 opcode=000001
```

## encode_rev16_metamorphic_rd_rn_sf
- Tier: 3
- Rationale: Algebraic metamorphic — Rd/Rn occupy disjoint 5-bit fields; W vs X flips only sf. Stronger rejected as above. Doc evidence: ARM ARM field layout Rd[4:0] Rn[9:5] sf[31].
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_metamorphic_rd_rn_sf
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..30. let b = encode_rev16(Rd,Rn). encode_rev16(Rd+1,Rn)[4:0]=rd+1 ∧ other bits equal to b; encode_rev16(Rd,Rn+1)[9:5]=rn+1 ∧ other bits equal to b; encode_rev16(!is_64,Rd,Rn) xor b = 1<<31
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rev16
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: 0..30, rn: 0..30 }
  body: encode_rev16(rd+1,rn) updates only bits[4:0] and encode_rev16(rd,rn+1) updates only bits[9:5] and W-vs-X xor is 1<<31
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM field layout Rd[4:0] Rn[9:5] sf[31]
```

## encode_rev16_neg_arity
- Tier: 4
- Rationale: Negative/error contract — REV16 requires exactly two register operands. llvm-mc rejects too few operands. Stronger oracles do not apply to the invalid domain. Doc evidence: llvm-mc "too few operands"; get_reg returns Err on missing index.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_neg_arity
- Formal: ∀ len ∈ 0..1, is_64 ∈ {false,true}, rd,rn ∈ 0..31. encode_rev16(truncate([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))], len)) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rev16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: 0..1, is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: throws
    expr: encode_rev16(truncate(ops2(is_64,rd,rn), len))
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects rev16 with fewer than 2 operands
```

## encode_rev16_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract — REV16 has no third operand. llvm-mc rejects extra operands. Doc evidence: llvm-mc error on `rev16 w0, w1, w2`.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_neg_extra_operand
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, extra ∈ Operand. encode_rev16([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, extra=Reg("x0")
- Bug report: pbt-out/bug_reports/encode_rev16_extra_operand.md

```property
function: encoder.bitfield.encode_rev16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, extra]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_rev16([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), extra])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects rev16 w0, w1, w2
```

## encode_rev16_neg_sp
- Tier: 4
- Rationale: Negative/error contract — ARM ARM Data-processing (1 source) uses ZR not SP at register 31. llvm-mc rejects sp/wsp. Doc evidence: ARM ARM; llvm-mc error on `rev16 sp, x0` / `rev16 wsp, w0`.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other ∈ 0..30, is_64 ∈ {false,true}. encode_rev16(ops with slot which = Reg(sp)) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0 (wsp, w0)
- Bug report: pbt-out/bug_reports/encode_rev16_sp.md

```property
function: encoder.bitfield.encode_rev16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: 0..1, sp64: bool, is_64: bool, other: 0..30 }
  relation:
    op: throws
    expr: encode_rev16(ops_with_sp(which, sp64, is_64, other))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM register 31 is ZR not SP; llvm-mc rejects sp/wsp
```

## encode_rev16_neg_mixed_width
- Tier: 4
- Rationale: Negative/error contract — W and X registers cannot be mixed. llvm-mc rejects `rev16 x0, w1`. Doc evidence: ARM ARM REV16 <Wd>,<Wn> / <Xd>,<Xn>; llvm-mc error on mixed width.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_rev16([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn))]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false (x0, w0)
- Bug report: pbt-out/bug_reports/encode_rev16_mixed_width.md

```property
function: encoder.bitfield.encode_rev16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd: 0..31, rn: 0..31, rd64: bool, rn64: bool, rd64 != rn64 }
  relation:
    op: throws
    expr: encode_rev16([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn))])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: String
evidence: llvm-mc rejects rev16 x0, w1; ARM ARM same-width W or X pair
```

## encode_rev16_neg_fp
- Tier: 4
- Rationale: Negative/error contract — FP/SIMD prefixes are not scalar REV16 operands. llvm-mc rejects `rev16 d0, d1`. Vector form uses RegArrangement, not Reg. Doc evidence: llvm-mc error on d/s/q/v/h/b as scalar REV16.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_rev16(ops with slot which = Reg(prefix+n)) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0 (d0, x1)
- Bug report: pbt-out/bug_reports/encode_rev16_fp.md

```property
function: encoder.bitfield.encode_rev16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: throws
    expr: encode_rev16(ops_with_fp(which, prefix, n))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, variants: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects rev16 d0, d1
```

## encode_rev16_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for GNU aliases (x31/w31, XZR/WZR, LR, uppercase). parse_reg_num lowercases; x31 and xzr are both 31. Doc evidence: README.md:11 GNU-style; parse_reg_num maps lr/xzr/wzr/x31.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_diff_alt_spellings
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, dest_spell,src_spell ∈ 0..4. encode_rev16([Reg(spell(is_64,rd,dest_spell)), Reg(spell(is_64,rn,src_spell))]) = Word(llvm-mc("rev16 " + spell(dest) + ", " + spell(src)))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rev16
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, dest_spell, src_spell]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, dest_spell: 0..4, src_spell: 0..4 }
  relation:
    op: eq
    lhs: encode_rev16([Reg(spell(is_64,rd,dest_spell)), Reg(spell(is_64,rn,src_spell))])
    rhs: llvm_mc("rev16 " + spell(is_64,rd,dest_spell) + ", " + spell(is_64,rn,src_spell))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style; parse_reg_num lr/xzr/wzr/x31 aliases
```

## encode_rev16_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract — get_reg requires Operand::Reg. llvm-mc rejects non-register operands. Doc evidence: get_reg Err on non-Reg; ARM ARM register operands.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Imm, Shift, Mem, Label, Symbol, Cond}. encode_rev16(ops with slot which = bad) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rev16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: non_reg_operand }
  relation:
    op: throws
    expr: encode_rev16(ops_with_kind(which, bad))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, variants: [Imm, Shift, Mem, Label, Symbol, Cond] }
expected_error: String
evidence: get_reg returns Err on non-Reg; llvm-mc rejects non-register operands
```

## encode_rev16_neg_invalid_name
- Tier: 4
- Rationale: Negative/error contract — parse_reg_num returns None for foo/x32/empty/r0. llvm-mc rejects invalid register names. Doc evidence: parse_reg_num; llvm-mc.
- Seed: bitfield.rs encode_rbit_pbt encode_rbit_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_rev16(ops with slot which = Reg(name)) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rev16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_gpr_name }
  relation:
    op: throws
    expr: encode_rev16(ops_with_name(which, name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, variants: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: parse_reg_num returns None for these names; llvm-mc rejects them
```
