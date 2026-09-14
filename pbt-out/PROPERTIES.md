# Properties: encode_clz

## encode_clz_diff_valid_gpr
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree CLZ decoder). encode_cls rejected as differential sibling (same-job gate: CLS is count-leading-sign-bits, opcode 000101 not 000100). Doc evidence: assembler README.md:11 (same textual assembly as gas); README.md:240 lists clz; encoder/mod.rs:573-575 dispatch; ARM ARM CLZ encoding.
- Seed: bitfield.rs encode_cls_pbt llvm-mc differential
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31. encode_clz([Reg(Rd), Reg(Rn)]) = Word(w) ∧ w = llvm-mc("clz Rd, Rn") where Rd/Rn are Wd/Wn if ¬is_64 else Xd/Xn (31 spelled wzr/xzr)
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_clz
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd,rn: 0..31 matching W or X }
  relation:
    op: eq
    lhs: encode_clz([Reg(Rd), Reg(Rn)])
    rhs: llvm_mc("clz Rd, Rn")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; README.md:240; encoder/mod.rs:573-575; ARM ARM CLZ
```

## encode_clz_arm_fields
- Tier: 4d
- Rationale: ARM ARM Data-processing (1 source) CLZ encoding sf 1 0 11010110 00000 000100 Rn Rd. Weaker than differential (already used). Invariant is the architectural field layout, unpacked independently of the producing statement.
- Seed: bitfield.rs encode_cls_arm_fields
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31. let w = encode_clz([Reg(Rd), Reg(Rn)]).Word. w[31]=sf ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=11010110 ∧ w[20:16]=00000 ∧ w[15:10]=000100 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_clz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd,rn: 0..31 matching W or X }
  relation:
    op: holds
    expr: arm_clz_fields(encode_clz([Reg(Rd), Reg(Rn)]))
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Data-processing (1 source) CLZ sf 1 0 11010110 00000 000100 Rn Rd; assembler README.md:11
```

## encode_clz_metamorphic_rd_rn
- Tier: 4c
- Rationale: ARM encoding places Rd in bits[4:0] and Rn in bits[9:5]; sf at bit 31. Incrementing one register or flipping W/X must change only that field. Weaker than differential. Required metamorphic companion.
- Seed: bitfield.rs encode_cls_metamorphic_rd_rn
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..30. encode_clz(rd+1,rn) xor encode_clz(rd,rn) has only bits[4:0] updated to rd+1; Rn+1 only bits[9:5]; encode_clz(X,X) xor encode_clz(W,W) = 1<<31
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_clz
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_64, rd, rn]
  domain: { is_64: bool, rd,rn in 0..30 }
  relation:
    op: holds
    expr: field_independence(encode_clz, rd, rn) && (encode_clz(X) xor encode_clz(W) == 1<<31)
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM CLZ Rd bits[4:0] Rn bits[9:5] sf bit 31
```

## encode_clz_neg_arity
- Tier: 4e
- Rationale: llvm-mc reports "too few operands" for `clz w0`. ARM CLZ is a two-operand instruction. get_reg on a missing slot must Err.
- Seed: bitfield.rs encode_cls_neg_arity
- Formal: ∀ len ∈ 0..1, is_64, rd,rn ∈ 0..31. encode_clz(ops[0..len]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_clz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_64, rd, rn]
  domain: { len: 0..1, is_64: bool, rd,rn: 0..31 }
  relation:
    op: holds
    expr: encode_clz(ops.truncate(len)).is_err()
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; ARM ARM CLZ <Wd>, <Wn>
```

## encode_clz_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects `clz w0, w1, w2` ("invalid operand"). CLZ has exactly two operands. Extra operand must Err.
- Seed: bitfield.rs encode_cls_neg_extra_operand
- Formal: ∀ valid (is_64, rd, rn), extra ∈ Operand. encode_clz([Rd, Rn, extra]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: is_64=false, rd=0, rn=0, extra=Reg("x0")  (clz w0, w0, x0) -> Ok(Word(0x5ac01000))
- Bug report: pbt-out/bug_reports/encode_clz_extra_operand.md

```property
function: encoder.bitfield.encode_clz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, extra]
  domain: { valid CLZ GPR pair plus one extra Operand }
  relation:
    op: holds
    expr: encode_clz([Rd, Rn, extra]).is_err()
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, values: ["Reg", "Imm", "Shift", "RegArrangement"] }
expected_error: String
evidence: llvm-mc rejects clz w0, w1, w2; ARM ARM CLZ two-operand form
```

## encode_clz_neg_sp
- Tier: 4e
- Rationale: ARM CLZ register 31 is ZR not SP. llvm-mc rejects `clz wsp, w0` and `clz w0, sp`. SP/WSP must Err.
- Seed: bitfield.rs encode_cls_neg_sp
- Formal: ∀ which ∈ {0,1}, sp ∈ {sp,wsp}, other GPR. encode_clz with slot which = SP/WSP = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, sp64=false, is_64=false, other=0  (clz wsp, w0) -> Ok(Word(0x5ac0101f))
- Bug report: pbt-out/bug_reports/encode_clz_sp.md

```property
function: encoder.bitfield.encode_clz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, sp64, is_64, other]
  domain: { which: 0..1, sp64: bool, is_64: bool, other: 0..30 }
  relation:
    op: holds
    expr: encode_clz(ops_with_sp_at(which)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  sp64: { gen: bool }
  is_64: { gen: bool }
  other: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: ARM ARM CLZ register 31 is ZR not SP; llvm-mc rejects SP/WSP
```

## encode_clz_neg_mixed_width
- Tier: 4e
- Rationale: llvm-mc rejects `clz x0, w1`. ARM CLZ requires matching W/W or X/X. Mixed width must Err.
- Seed: bitfield.rs encode_cls_neg_mixed_width
- Formal: ∀ rd,rn ∈ 0..31, rd64 ≠ rn64. encode_clz([Reg(rd64?Xd:Wd), Reg(rn64?Xn:Wn)]) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: rd=0, rn=0, rd64=true, rn64=false  (clz x0, w0) -> Ok(Word(0xdac01000))
- Bug report: pbt-out/bug_reports/encode_clz_mixed_width.md

```property
function: encoder.bitfield.encode_clz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rd64, rn64]
  domain: { rd,rn: 0..31, rd64 != rn64 }
  relation:
    op: holds
    expr: encode_clz([Reg mixed W/X]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
expected_error: String
evidence: llvm-mc rejects clz x0, w1; ARM ARM CLZ Wd,Wn / Xd,Xn
```

## encode_clz_neg_fp
- Tier: 4e
- Rationale: llvm-mc rejects `clz d0, x1`. ARM scalar CLZ operands are GPRs only. FP/SIMD prefixes d/s/q/v/h/b must Err. (NEON vector CLZ is a different dispatch path via encode_neon_two_misc.)
- Seed: bitfield.rs encode_cls_neg_fp
- Formal: ∀ which ∈ {0,1}, prefix ∈ {d,s,q,v,h,b}, n ∈ 0..31. encode_clz with slot which = prefix+n = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: failing
- Counterexample: which=0, prefix="d", n=0  (clz d0, x1) -> Ok(Word(0x5ac01020))
- Bug report: pbt-out/bug_reports/encode_clz_fp.md

```property
function: encoder.bitfield.encode_clz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, n]
  domain: { which: 0..1, prefix: [d, s, q, v, h, b], n: 0..31 }
  relation:
    op: holds
    expr: encode_clz(ops_with_fp_at(which)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, values: ["d", "s", "q", "v", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects clz d0, x1; ARM ARM CLZ GPR-only; encoder/mod.rs:573-575 NEON vs scalar dispatch
```

## encode_clz_diff_alt_spellings
- Tier: 2
- Rationale: Same differential oracle as encode_clz_diff_valid_gpr, targeting documented alternate spellings llvm-mc accepts: x31/w31 (ZR), XZR/WZR uppercase, LR, and fully uppercased GPR names. Sweep: canonical gpr() generator never emits these aliases.
- Seed: bitfield.rs encode_cls_diff_alt_spellings
- Formal: ∀ is_64 ∈ {false,true}, rd,rn ∈ 0..31, dest_spell,src_spell ∈ 0..4. encode_clz([Reg(spell(Rd)), Reg(spell(Rn))]) = Word(w) ∧ w = llvm-mc("clz spell(Rd), spell(Rn)")
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_clz
oracle: differential
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, dest_spell, src_spell]
  domain: { is_64: bool, rd: 0..31, rn: 0..31, dest_spell: 0..4, src_spell: 0..4 }
  relation:
    op: eq
    lhs: encode_clz([Reg(spell(Rd)), Reg(spell(Rn))])
    rhs: llvm_mc("clz spell(Rd), spell(Rn)")
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_spell: { gen: int, min: 0, max: 4, type: u32 }
  src_spell: { gen: int, min: 0, max: 4, type: u32 }
evidence: llvm-mc accepts x31/w31, XZR/WZR, LR; parse_reg_num maps lr/xzr/wzr/x31; README.md:11
```

## encode_clz_neg_nonreg
- Tier: 4e
- Rationale: llvm-mc rejects non-register operand kinds. get_reg must Err on Imm/Shift/Mem/Label/Symbol/Cond/RegArrangement at either slot. Sweep: first batch only used Reg operands.
- Seed: bitfield.rs encode_cls_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ non-Reg Operand. encode_clz with slot which = bad = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_clz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: non-Reg Operand }
  relation:
    op: holds
    expr: encode_clz(ops_with_nonreg_at(which)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, values: ["Imm", "Shift", "Mem", "Label", "Symbol", "Cond", "RegArrangement"] }
expected_error: String
evidence: llvm-mc rejects non-register CLZ operands; get_reg expected register at operand idx
```

## encode_clz_neg_invalid_name
- Tier: 4e
- Rationale: llvm-mc rejects invalid register names (foo, x32, empty, r0). parse_reg_num returns None outside w/x 0..31 and aliases. Sweep: first batch only used valid GPR names.
- Seed: bitfield.rs encode_cls_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, x, r0, empty, x-1, x99, w}. encode_clz with slot which = Reg(name) = Err
- Test file: src/backend/arm/assembler/encoder/bitfield.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.bitfield.encode_clz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid register names }
  relation:
    op: holds
    expr: encode_clz(ops_with_invalid_name_at(which)).is_err()
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, values: ["foo", "x32", "w32", "x", "r0", "", "x-1", "x99", "w"] }
expected_error: String
evidence: llvm-mc rejects foo/x32/r0; parse_reg_num returns None for those names
```
