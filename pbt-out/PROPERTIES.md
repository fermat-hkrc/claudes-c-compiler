# Properties: encode_rbit

## encode_rbit_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree RBIT decoder). Sibling encode_neon_rbit rejected for scalar (vector class; dispatch splits). Sibling encode_clz/encode_cls/encode_rev rejected (different opcode). Doc evidence: README.md:11 GNU-style assembly; README.md:240 lists rbit; encoder/mod.rs:902-909 dispatch; ARM ARM Data-processing (1 source) RBIT.
- Seed: bitfield.rs encode_clz_pbt encode_clz_diff_valid_gpr
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31. encode_rbit([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))]) = Word(llvm-mc("rbit gpr(is_64,rd), gpr(is_64,rn)"))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_rbit([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))])
    rhs: llvm_mc("rbit " + gpr(is_64,rd) + ", " + gpr(is_64,rn))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:11 GNU-style assembly; ARM ARM RBIT sf 1 0 11010110 00000 000000 Rn Rd
```

## encode_rbit_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM encoding, weaker than differential (already used above) but pins each field independently. Stronger rejected as above. Doc evidence: ARM ARM Data-processing (1 source) RBIT opcode=000000.
- Seed: bitfield.rs encode_clz_pbt encode_clz_arm_fields
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31. let w = encode_rbit([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))]). w = (sf<<31)|(1<<30)|(0b011010110<<21)|(rn<<5)|rd ∧ w[31]=sf ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=11010110 ∧ w[20:16]=00000 ∧ w[15:10]=000000 ∧ w[9:5]=rn ∧ w[4:0]=rd where sf = [is_64]
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_rbit([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))])
    rhs: (sf(is_64)<<31)|(1<<30)|(0b011010110<<21)|(rn<<5)|rd
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (1 source) RBIT opcode=000000
```

## encode_rbit_metamorphic_rd_rn_sf
- Tier: 3
- Rationale: Algebraic metamorphic — Rd/Rn occupy disjoint 5-bit fields; W vs X flips only sf. Stronger rejected as above. Doc evidence: ARM ARM field layout Rd[4:0] Rn[9:5] sf[31].
- Seed: bitfield.rs encode_clz_pbt encode_clz_metamorphic_rd_rn
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..30. let b = encode_rbit(Rd,Rn). encode_rbit(Rd+1,Rn)[4:0]=rd+1 ∧ other bits equal to b; encode_rbit(Rd,Rn+1)[9:5]=rn+1 ∧ other bits equal to b; encode_rbit(!is_64,Rd,Rn) xor b = 1<<31
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd: 0..30, rn: 0..30 }
  body: encode_rbit(rd+1,rn) updates only bits[4:0] and encode_rbit(rd,rn+1) updates only bits[9:5] and W-vs-X xor is 1<<31
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM RBIT Rd bits[4:0] Rn bits[9:5] sf bit31
```

## encode_rbit_neg_arity
- Tier: 5
- Rationale: Negative/error contract — llvm-mc rejects too few operands ("too few operands for instruction"). Stronger oracles do not cover the invalid-arity domain.
- Seed: bitfield.rs encode_clz_pbt encode_clz_neg_arity
- Formal: ∀ len ∈ 0..1, is_64 ∈ {false,true}, rd,rn ∈ 0..31. encode_rbit(prefix of [Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn))] of length len) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: 0..1, is_64: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: holds
    expr: encode_rbit(ops[..len]).is_err()
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction" for `rbit w0`
```

## encode_rbit_neg_extra_operand
- Tier: 5
- Rationale: Negative/error contract — llvm-mc rejects a 3rd operand ("invalid operand for instruction"). RBIT is a 2-operand instruction. Stronger oracles do not cover extra operands.
- Seed: bitfield.rs encode_clz_pbt encode_clz_neg_extra_operand
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, extra ∈ ExtraOperand. encode_rbit([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, extra=Reg("x0")  (rbit w0, w0, x0)
- Bug report: pbt-out/bug_reports/encode_rbit_extra_operand.md

```property
function: encoder.bitfield.encode_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, extra]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, extra: ExtraOperand }
  relation:
    op: holds
    expr: encode_rbit([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), extra]).is_err()
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects `rbit w0, w1, w2` and `rbit x0, x1, #0`
```

## encode_rbit_neg_sp
- Tier: 5
- Rationale: Negative/error contract — ARM ARM register 31 is ZR not SP; llvm-mc rejects SP/WSP as RBIT operands. Stronger oracles do not cover this invalid domain.
- Seed: bitfield.rs encode_clz_pbt encode_clz_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, is_64 ∈ {false,true}, other ∈ 0..30. encode_rbit(ops with slot which = Reg(sp)) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0  (rbit wsp, w0)
- Bug report: pbt-out/bug_reports/encode_rbit_sp.md

```property
function: encoder.bitfield.encode_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp, is_64, other]
  domain: { which: 0..1, sp: {sp,wsp}, is_64: bool, other: 0..30 }
  relation:
    op: holds
    expr: encode_rbit(ops_with_slot(which, Reg(sp))).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp: { gen: oneof, items: ["sp", "wsp"] }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc rejects `rbit sp, x0` and `rbit wsp, w0`; ARM ARM Rd/Rn are ZR not SP at 31
```

## encode_rbit_neg_mixed_width
- Tier: 5
- Rationale: Negative/error contract — llvm-mc rejects mixed W/X (`rbit x0, w1` / `rbit w0, x1`). ARM ARM requires matching 32-bit or 64-bit pair.
- Seed: bitfield.rs encode_clz_pbt encode_clz_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_rbit([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn))]) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false  (rbit x0, w0)
- Bug report: pbt-out/bug_reports/encode_rbit_mixed_width.md

```property
function: encoder.bitfield.encode_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd: 0..31, rn: 0..31, rd64: bool, rn64: bool }
  relation:
    op: holds
    expr: encode_rbit([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn))]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: String
evidence: llvm-mc rejects `rbit x0, w1` and `rbit w0, x1`
```

## encode_rbit_neg_fp
- Tier: 5
- Rationale: Negative/error contract — llvm-mc rejects FP/SIMD prefixes as scalar RBIT operands (`rbit d0, d1`). Scalar RBIT is GPR-only.
- Seed: bitfield.rs encode_clz_pbt encode_clz_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_rbit(ops with slot which = Reg(prefix+n)) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0  (rbit d0, x1)
- Bug report: pbt-out/bug_reports/encode_rbit_fp.md

```property
function: encoder.bitfield.encode_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: {d,s,q,v,h,b}, n: 0..31 }
  relation:
    op: holds
    expr: encode_rbit(ops_with_slot(which, Reg(prefix+n))).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, items: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects `rbit d0, d1` and `rbit s0, s1`
```

## encode_rbit_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc on x31/w31, XZR/WZR, LR, and uppercase aliases. Strengthening/sweep of the valid-GPR differential. Same rejection chain as encode_rbit_diff_valid_gpr.
- Seed: bitfield.rs encode_clz_pbt encode_clz_diff_alt_spellings
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, dest_spell,src_spell ∈ 0..4. encode_rbit([Reg(spell(is_64,rd,dest_spell)), Reg(spell(is_64,rn,src_spell))]) = Word(llvm-mc("rbit " + spell + ", " + spell))
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, dest_spell, src_spell]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, dest_spell: 0..4, src_spell: 0..4 }
  relation:
    op: eq
    lhs: encode_rbit([Reg(spell(is_64,rd,dest_spell)), Reg(spell(is_64,rn,src_spell))])
    rhs: llvm_mc("rbit " + spell(is_64,rd,dest_spell) + ", " + spell(is_64,rn,src_spell))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: README.md:11; llvm-mc accepts x31/XZR/LR/uppercase as ZR/X30 aliases
```

## encode_rbit_neg_nonreg
- Tier: 5
- Rationale: Negative/error contract — non-register kinds at Rd/Rn must Err (get_reg / llvm-mc). Sweep of operand-kind error path.
- Seed: bitfield.rs encode_clz_pbt encode_clz_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Imm, Shift, Mem, Label, Symbol, Cond}. encode_rbit(ops with slot which = bad) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: NonRegOperand }
  relation:
    op: holds
    expr: encode_rbit(ops_with_slot(which, bad)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, items: [Imm, Shift, Mem, Label, Symbol, Cond] }
expected_error: String
evidence: get_reg requires Operand::Reg; llvm-mc rejects non-register RBIT operands
```

## encode_rbit_neg_invalid_name
- Tier: 5
- Rationale: Negative/error contract — parse_reg_num rejects foo/x32/empty/r0. Sweep of the parse_reg_num failure arm.
- Seed: bitfield.rs encode_clz_pbt encode_clz_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, "", x-1, x99, w}. encode_rbit(ops with slot which = Reg(name)) is Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: InvalidName }
  relation:
    op: holds
    expr: encode_rbit(ops_with_slot(which, Reg(name))).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, items: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: parse_reg_num returns None for non w/x/d/s/q/v/h/b names and numbers > 31
```

## encode_rbit_diff_valid_neon
- Tier: 2
- Rationale: Differential vs llvm-mc for the in-function NEON arm (RegArrangement 8b/16b). Sweep to reach the documented vector encoding in encode_rbit. Sibling encode_neon_rbit is the dispatch path for this form (not a same-job differential for scalar); here we exercise encode_rbit directly. State machine / round-trip rejected as above.
- Seed: neon.rs encode_neon_rbit_pbt (vector RBIT KAT)
- Formal: ∀ q16 ∈ {false,true}, rd,rn ∈ 0..31. encode_rbit([RegArrangement(v{rd}, T), RegArrangement(v{rn}, T)]) = Word(llvm-mc("rbit v{rd}.T, v{rn}.T")) where T = 16b if q16 else 8b
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_rbit
oracle: differential
predicate:
  quantifier: forall
  vars: [q16, rd, rn]
  domain: { q16: bool, rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_rbit([RegArrangement(v{rd}, T), RegArrangement(v{rn}, T)])
    rhs: llvm_mc("rbit v{rd}.T, v{rn}.T")
generators:
  q16: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: bitfield.rs:168-176 NEON vector form comment; ARM ARM Advanced SIMD two-register miscellaneous RBIT T in {8B,16B}
```
