# Properties: encode_fcmp

## encode_fcmp_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (GNU-style assembler contract, README.md:11). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree decoder). encode_fp_arith / encode_neon_float_cmp_zero fail the same-job gate.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_diff_valid; codegen/comparison.rs:15-19
- Formal: ∀ rn,rm ∈ 0..31, is_d ∈ {S,D}, spelling ∈ {canonical, uppercase}. encode_fcmp([Reg(Sn|Dn), Reg(Sm|Dm)]) = Word(w) ∧ w = llvm-mc("fcmp Sn|Dn, Sm|Dm")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fcmp
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm, is_d]
  domain: { rn: 0..31, rm: 0..31, is_d: bool }
  relation:
    op: eq
    lhs: encode_fcmp([Reg(fp(is_d,rn)), Reg(fp(is_d,rm))])
    rhs: llvm_mc("fcmp " + fp(is_d,rn) + ", " + fp(is_d,rm))
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:439; codegen/comparison.rs:15-19
```

## encode_fcmp_diff_zero
- Tier: 2
- Rationale: ARM FCMP compare-to-zero form is a documented second encoding (purpose comment fp_scalar.rs:164; llvm-mc accepts `fcmp Sn|Dn, #0.0`). Operand has no float-immediate variant; Imm(0) is the encoder-level mapping of #0.0. Differential vs llvm-mc.
- Seed: fp_scalar.rs:164-167
- Formal: ∀ rn ∈ 0..31, is_d ∈ {S,D}. encode_fcmp([Reg(Sn|Dn), Imm(0)]) = Word(w) ∧ w = llvm-mc("fcmp Sn|Dn, #0.0")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fcmp
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, is_d]
  domain: { rn: 0..31, is_d: bool }
  relation:
    op: eq
    lhs: encode_fcmp([Reg(fp(is_d,rn)), Imm(0)])
    rhs: llvm_mc("fcmp " + fp(is_d,rn) + ", #0.0")
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
evidence: src/backend/arm/assembler/README.md:11; fp_scalar.rs:164; ARM ARM Floating-point compare
```

## encode_fcmp_arm_fields
- Tier: 4
- Rationale: ARM ARM Floating-point compare field layout is an exact structural invariant on success-path words. Weaker than differential (does not catch wrong class vs llvm-mc) but pins documented bit positions independently of the reference tool.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_arm_fields; fp_scalar.rs:171
- Formal: ∀ rn,rm ∈ 0..31, is_d. let w = encode_fcmp([Reg(Sn|Dn), Reg(Sm|Dm)]). Then w[31:24]=00011110, w[23:22]=ftype, w[21]=1, w[20:16]=rm, w[15:10]=001000, w[9:5]=rn, w[4:0]=00000. For Imm(0): w[20:16]=00000 and w[4:0]=01000.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fcmp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rn, rm, is_d]
  domain: { rn: 0..31, rm: 0..31, is_d: bool }
  relation:
    op: eq
    lhs: encode_fcmp([Reg(fp(is_d,rn)), Reg(fp(is_d,rm))])
    rhs: (0b00011110<<24)|(ftype<<22)|(1<<21)|(rm<<16)|(0b001000<<10)|(rn<<5)
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
evidence: fp_scalar.rs:171; ARM ARM Floating-point compare
```

## encode_fcmp_metamorphic_fields
- Tier: 4
- Rationale: Independent field packing implies Rn/Rm/ftype/opc mutations affect only their documented bits. Metamorphic (weaker than round-trip; no inverse exists). Required metamorphic property at standard tier.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_metamorphic_fields
- Formal: ∀ rn,rm ∈ 0..30, is_d. let w = encode_fcmp([Sn|Dn rn, Sm|Dm rm]). Then encode(rn+1)=w+(1<<5), encode(rm+1)=w+(1<<16), encode(is_d flip) XOR w = 1<<22, encode(Imm(0)) XOR encode(rm=0) = 1<<3.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fcmp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rn, rm, is_d]
  domain: { rn: 0..30, rm: 0..30, is_d: bool }
  body: encode(rn+1)==w+(1<<5) AND encode(rm+1)==w+(1<<16) AND encode(ftype_flip) XOR w == 1<<22 AND encode(Imm(0)) XOR encode(rm=0) == 1<<3
generators:
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_d: { gen: bool }
evidence: fp_scalar.rs:164-175; ARM ARM Floating-point compare opc/ftype/Rm/Rn
```

## encode_fcmp_neg_arity
- Tier: 5
- Rationale: ARM FCMP always has two operands (Sn, Sm or Sn, #0.0). llvm-mc rejects `fcmp s0` with "too few operands". README.md:11 gas-compatibility. Negative/error contract. The SUT currently treats len<2 as #0.0 — producing statement is not Doc evidence that 1-operand is valid.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_arity
- Formal: ∀ len ∈ {0,1}, n ∈ 0..31, is_d. encode_fcmp(ops of length len) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: len=1, n=0, is_d=false — [Reg("s0")] encodes as Word(0x1e202008) (fcmp s0, #0.0)
- Bug report: pbt-out/bug_reports/encode_fcmp_arity.md

```property
function: encode_fcmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n, is_d]
  domain: { len: 0..1, n: 0..31, is_d: bool }
  relation:
    op: throws
    expr: encode_fcmp(ops_of_len(len))
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
expected_error: String
evidence: src/backend/arm/assembler/README.md:11; llvm-mc "too few operands for instruction"; ARM ARM FCMP two-operand syntax
```

## encode_fcmp_neg_extra_operand
- Tier: 5
- Rationale: Scalar FCMP has no third operand (FCCMP is a different mnemonic). llvm-mc rejects `fcmp s0, s1, s2` ("invalid operand"). Extra operand must Err.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_extra_operand
- Formal: ∀ rn,rm ∈ 0..31, is_d, extra ∈ Operand. encode_fcmp([Reg(Sn|Dn), Reg(Sm|Dm), extra]) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rn=0, rm=0, is_d=false, extra=Reg("s0") — [Reg("s0"), Reg("s0"), Reg("s0")] encodes as Word(0x1e202000)
- Bug report: pbt-out/bug_reports/encode_fcmp_extra_operand.md

```property
function: encode_fcmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, rm, is_d, extra]
  domain: { rn: 0..31, rm: 0..31, is_d: bool, extra: Operand }
  relation:
    op: throws
    expr: encode_fcmp([Reg(fp(is_d,rn)), Reg(fp(is_d,rm)), extra])
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  extra: { gen: oneof, items: [Reg, Imm(0), Imm(1), Shift, RegArrangement] }
expected_error: String
evidence: src/backend/arm/assembler/README.md:11; llvm-mc "invalid operand for instruction"; encoder/mod.rs:439 fcmp vs fccmp
```

## encode_fcmp_neg_wrong_types
- Tier: 5
- Rationale: ARM FCMP requires matching Sn,Sm or Dn,Dm (or Hn,Hm with fp16). llvm-mc rejects mixed S/D, GPR, Q/V/B, SP/WSP. Negative/error contract under the gas-compatibility README.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_wrong_types
- Formal: ∀ (a,b) ∈ mixed-S/D ∪ GPR ∪ QVB ∪ SP. encode_fcmp([Reg(a), Reg(b)]) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: a="s0", b="d0" — encodes as Word(0x1e202000) (same bits as fcmp s0, s0)
- Bug report: pbt-out/bug_reports/encode_fcmp_wrong_types.md

```property
function: encode_fcmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [a, b]
  domain: { a: wrong_fp_name, b: wrong_fp_name }
  relation:
    op: throws
    expr: encode_fcmp([Reg(a), Reg(b)])
generators:
  a: { gen: string }
  b: { gen: string }
expected_error: String
evidence: src/backend/arm/assembler/README.md:11; llvm-mc "invalid operand for instruction"; ARM ARM FCMP Sn/Sm or Dn/Dm
```

## encode_fcmp_diff_half
- Tier: 2
- Rationale: ARM ARM ftype=11 for half-precision FCMP; llvm-mc -mattr=+fullfp16 accepts `fcmp h0, h1` / `fcmp h0, #0.0`. parse_reg_num accepts h0-h31. Differential vs llvm-mc. Same half-ftype contract as neighbouring fp_scalar encoders.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_diff_half
- Formal: ∀ rn,rm ∈ 0..31. encode_fcmp([Reg(Hn), Reg(Hm)]) = Word(w) ∧ w = llvm-mc-fp16("fcmp Hn, Hm"). Also encode_fcmp([Reg(Hn), Imm(0)]) = llvm-mc-fp16("fcmp Hn, #0.0").
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rn=0, rm=0, zero=false — SUT Word(0x1e202000) vs llvm-mc Word(0x1ee02000)
- Bug report: pbt-out/bug_reports/encode_fcmp_half_ftype.md

```property
function: encode_fcmp
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm]
  domain: { rn: 0..31, rm: 0..31 }
  relation:
    op: eq
    lhs: encode_fcmp([Reg("h"+rn), Reg("h"+rm)])
    rhs: llvm_mc_fp16("fcmp h" + rn + ", h" + rm)
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; ARM ARM ftype=11 half; parse_reg_num h0-h31; llvm-mc -mattr=+fullfp16
```

## encode_fcmp_neg_nonzero_imm
- Tier: 5
- Rationale: ARM FCMP immediate form allows only #0.0. llvm-mc rejects `#1.0` ("expected floating-point constant #0.0") and `#0` (integer). Documented bound Imm(0) vs bound±1. Sweep property.
- Seed: llvm-mc `fcmp s0, #1.0`
- Formal: ∀ rn ∈ 0..31, is_d, imm ∈ {-1,1,MIN,MAX,2..32}. encode_fcmp([Reg(Sn|Dn), Imm(imm)]) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fcmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, is_d, imm]
  domain: { rn: 0..31, is_d: bool, imm: nonzero i64 }
  relation:
    op: throws
    expr: encode_fcmp([Reg(fp(is_d,rn)), Imm(imm)])
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  imm: { gen: int, min: -1, max: 32, type: i64 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:11; llvm-mc "expected floating-point constant #0.0"; ARM ARM FCMP #0.0 only
```

## encode_fcmp_neg_nonreg
- Tier: 5
- Rationale: get_reg requires Operand::Reg. Non-register kinds at either slot (except Imm(0) at slot 1, which is the #0.0 form) must Err. Sweep of documented error path.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_nonreg
- Formal: ∀ which ∈ {0,1}, kind ∈ {Imm(1), Symbol, Label, Mem, Cond, Shift}. encode_fcmp with that kind at slot which = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fcmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..1, kind: nonreg Operand }
  relation:
    op: throws
    expr: encode_fcmp(ops_with_nonreg_at(which, kind))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956-966 get_reg; llvm-mc invalid operand
```

## encode_fcmp_neg_invalid_name
- Tier: 5
- Rationale: parse_reg_num rejects names outside x/w/d/s/q/v/h/b 0..31 and sp/xzr/lr. Invalid names must Err. Sweep of documented error path.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,s32,d32,h32,x32,r0,s,d,""}. encode_fcmp with Reg(name) at slot which = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fcmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid register spelling }
  relation:
    op: throws
    expr: encode_fcmp(ops_with_name_at(which, name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num; llvm-mc invalid operand
```
