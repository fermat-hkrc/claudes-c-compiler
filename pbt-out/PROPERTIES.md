# Property ledger: encode_rev

## encode_rev_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree REV decoder). Sibling encode_rev16/encode_rev32/encode_rbit/encode_clz/encode_cls rejected (different opcode). Doc evidence: README.md:11 GNU-style assembly; README.md:240 lists rev; encoder/mod.rs:910 dispatch; ARM ARM Data-processing (1 source) REV Wd,Wn / Xd,Xn with opc=000010 (W) / 000011 (X).
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_diff_valid_gpr
- Formal: ∀ is_64 ∈ Bool, rd,rn ∈ {0..31}. encode_rev([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))]) = llvm-mc("rev Rd, Rn") as LE word
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: u32 0..=31, rn: u32 0..=31 }
  relation:
    op: eq
    lhs: encode_rev([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn))])
    rhs: llvm_mc_word("rev " + gpr(is_64, rd) + ", " + gpr(is_64, rn))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:11 GNU-style gas contract; ARM ARM REV Wd/Xd; llvm-mc -triple=aarch64 -show-encoding
```

## encode_rev_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM encoding diagram. Stronger rejected as above. Weaker than differential but pins each field independently, including the sf-dependent opcode (000010 W / 000011 X) that distinguishes REV from REV16/REV32.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_arm_fields
- Formal: ∀ is_64 ∈ Bool, rd,rn ∈ {0..31}. let w = encode_rev([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))]), sf = ⟦is_64⟧, opc = is_64 ? 000011 : 000010. w[31]=sf ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=11010110 ∧ w[20:16]=00000 ∧ w[15:10]=opc ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: u32 0..=31, rn: u32 0..=31 }
  relation:
    op: eq
    lhs: encode_rev([Reg(gpr(is_64, rd)), Reg(gpr(is_64, rn))])
    rhs: (sf << 31) | (1 << 30) | (0b011010110 << 21) | (opc << 10) | (rn << 5) | rd
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (1 source) REV encoding sf 1 0 11010110 00000 opc Rn Rd; opc=000010 (32-bit) / 000011 (64-bit)
```

## encode_rev_metamorphic_rd_rn_sf
- Tier: 4
- Rationale: Algebraic metamorphic: incrementing Rd/Rn must touch only that 5-bit field. W vs X must flip sf and the opcode LSB (bit 10), not only sf — unlike REV16/CLZ/RBIT whose opcode is width-invariant. Stronger rejected as above.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_metamorphic_rd_rn_sf
- Formal: ∀ is_64 ∈ Bool, rd,rn ∈ {0..30}. let b = encode_rev(gpr(is_64,rd), gpr(is_64,rn)). encode_rev(rd+1,rn) & 0x1f = rd+1 ∧ agrees with b outside bits[4:0]; encode_rev(rd,rn+1) bits[9:5] = rn+1 ∧ agrees with b outside bits[9:5]; encode_rev(!is_64, rd, rn) xor b = (1<<31)|(1<<10)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: u32 0..=30, rn: u32 0..=30 }
  relation:
    op: holds
    expr: field_independence(encode_rev, is_64, rd, rn) && (encode_rev(!is_64,rd,rn) xor encode_rev(is_64,rd,rn) == (1<<31)|(1<<10))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM REV field layout Rd bits[4:0] Rn bits[9:5] sf bit31 opc bits[15:10] (000010 W / 000011 X)
```

## encode_rev_neg_arity
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects too few operands ("too few operands for instruction"). get_reg on a missing slot returns Err. Stronger rejected as above.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_arity
- Formal: ∀ is_64 ∈ Bool, rd,rn ∈ {0..31}, len ∈ {0,1}. encode_rev(prefix([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))], len)) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: usize 0..=1, is_64: bool, rd: u32 0..=31, rn: u32 0..=31 }
  relation:
    op: holds
    expr: encode_rev(ops[0..len]).is_err()
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction" on `rev w0`; get_reg missing slot -> Err
```

## encode_rev_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects a 3rd operand ("invalid operand for instruction"). GNU-style assembler contract (README.md:11). encode_rev does not check operands.len(), so extra operands are currently ignored — expected to fail.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_extra_operand
- Formal: ∀ is_64 ∈ Bool, rd,rn ∈ {0..31}, extra ∈ Operand. encode_rev([Reg(Rd), Reg(Rn), extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: [Reg("w0"), Reg("w0"), Reg("x0")]
- Bug report: pbt-out/bug_reports/encode_rev_extra_operand.md

```property
function: encode_rev
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, extra]
  domain: { is_64: bool, rd: u32 0..=31, rn: u32 0..=31, extra: Operand }
  relation:
    op: holds
    expr: encode_rev([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), extra]).is_err()
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects `rev w0, w1, w2` and `rev w0, w1, #0`; README.md:11 GNU-style gas contract
```

## encode_rev_neg_sp
- Tier: 4
- Rationale: Negative/error contract. ARM ARM Data-processing (1 source) uses ZR at register 31, not SP. llvm-mc rejects SP/WSP as REV operands. parse_reg_num maps sp/wsp to 31, so the SUT currently encodes them as ZR — expected to fail.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, is_64 ∈ Bool, other ∈ {0..30}. encode_rev(ops with slot `which` = Reg(sp)) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: [Reg("wsp"), Reg("w0")]
- Bug report: pbt-out/bug_reports/encode_rev_sp.md

```property
function: encode_rev
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: u32 0..=1, sp64: bool, is_64: bool, other: u32 0..=30 }
  relation:
    op: holds
    expr: encode_rev(ops_with_sp).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM REV register 31 is ZR not SP; llvm-mc rejects `rev sp, x0` and `rev wsp, w0`
```

## encode_rev_neg_mixed_width
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects mixed W/X (`rev w0, x1` / `rev x0, w1`). encode_rev takes sf from Rd and ignores Rn width — expected to fail.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_mixed_width
- Formal: ∀ rd,rn ∈ {0..31}, rd64,rn64 ∈ Bool. rd64 ≠ rn64 ⇒ encode_rev([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn))]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("w0")]
- Bug report: pbt-out/bug_reports/encode_rev_mixed_width.md

```property
function: encode_rev
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd: u32 0..=31, rn: u32 0..=31, rd64: bool, rn64: bool }
  relation:
    op: holds
    expr: (rd64 != rn64) => encode_rev([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn))]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: String
evidence: llvm-mc rejects `rev w0, x1` and `rev x0, w1`; ARM ARM REV requires matching W/W or X/X
```

## encode_rev_neg_fp
- Tier: 4
- Rationale: Negative/error contract. llvm-mc rejects FP/SIMD prefixes as REV operands. parse_reg_num accepts d/s/q/v/h/b, so the SUT currently encodes them as GPR numbers — expected to fail.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_rev(ops with slot `which` = Reg(prefix{n})) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: [Reg("d0"), Reg("x1")]
- Bug report: pbt-out/bug_reports/encode_rev_fp.md

```property
function: encode_rev
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: u32 0..=1, prefix: {d,s,q,v,h,b}, n: u32 0..=31 }
  relation:
    op: holds
    expr: encode_rev(ops_with_fp).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, variants: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects `rev d0, d1`; ARM ARM REV operands are W/X GPRs; README.md:11
```

## encode_rev_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for GNU-style alternate spellings (x31/w31, XZR/WZR, LR, uppercase). README.md:11 GNU-style gas contract. Sweep: documented spelling surface not covered by the wzr/xzr-only valid-GPR generator.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_diff_alt_spellings
- Formal: ∀ is_64 ∈ Bool, rd,rn ∈ {0..31}, dest_spell,src_spell ∈ {0..4}. encode_rev([Reg(spell(is_64,rd,dest_spell)), Reg(spell(is_64,rn,src_spell))]) = llvm-mc("rev " + spell + ", " + spell)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, dest_spell, src_spell]
  domain: { is_64: bool, rd: u32 0..=31, rn: u32 0..=31, dest_spell: u32 0..=4, src_spell: u32 0..=4 }
  relation:
    op: eq
    lhs: encode_rev([Reg(spell(is_64, rd, dest_spell)), Reg(spell(is_64, rn, src_spell))])
    rhs: llvm_mc_word("rev " + spell(is_64, rd, dest_spell) + ", " + spell(is_64, rn, src_spell))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11 GNU-style gas contract; llvm-mc accepts x31/w31, XZR/WZR, LR, uppercase
```

## encode_rev_neg_nonreg
- Tier: 4
- Rationale: Negative/error contract. get_reg requires Operand::Reg; llvm-mc rejects non-register operand kinds. Sweep: Imm/Shift/Mem/Label/Symbol/Cond not in first-batch generators.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Imm, Shift, Mem, Label, Symbol, Cond}. encode_rev(ops with slot which = bad) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: u32 0..=1, bad: non_reg Operand }
  relation:
    op: holds
    expr: encode_rev(ops_with_nonreg).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, variants: [Imm, Shift, Mem, Label, Symbol, Cond] }
expected_error: String
evidence: get_reg expected register; llvm-mc rejects non-register REV operands
```

## encode_rev_neg_invalid_name
- Tier: 4
- Rationale: Negative/error contract. parse_reg_num returns None for foo/x32/empty/r0. llvm-mc rejects invalid register names. Sweep: invalid-name domain not in first batch.
- Seed: bitfield.rs encode_rev16_pbt::encode_rev16_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_rev(ops with slot which = Reg(name)) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_rev
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: u32 0..=1, name: invalid register spelling }
  relation:
    op: holds
    expr: encode_rev(ops_with_bad_name).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, variants: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: parse_reg_num None for those names; llvm-mc rejects invalid register names
```
