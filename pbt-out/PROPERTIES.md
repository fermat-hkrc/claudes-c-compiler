# Properties: encode_fabs

## encode_fabs_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure encoder, no lifecycle). Algebraic round-trip rejected (no in-tree FABS decoder). Sibling encode_fneg/encode_fsqrt rejected (same-job gate: different ARM opcodes). Sibling encode_fp_1src rejected (FRINT*). Sibling encode_neon_float_two_misc rejected (vector form). Doc evidence: README.md:11 gas-compatible assembly; encoder/mod.rs:408-411 scalar fabs dispatch; ARM ARM FP 1-source FABS.
- Seed: fp_scalar.rs encode_fp_1src_diff_valid
- Formal: ∀ rd,rn ∈ 0..31, is_d ∈ {false,true}, dest/src spellings ∈ {sN/dN, SN/DN}. encode_fabs([Reg(dest), Reg(src)]) = llvm-mc("fabs dest, src") as little-endian Word.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fabs
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, dest_kind, src_kind]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, dest_kind: 0..1, src_kind: 0..1 }
  relation:
    op: eq
    lhs: encode_fabs([Reg(fp_spelling(is_d, rd, dest_kind)), Reg(fp_spelling(is_d, rn, src_kind))])
    rhs: llvm_mc("fabs " + dest + ", " + src)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  dest_kind: { gen: int, min: 0, max: 1, type: u32 }
  src_kind: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:11; encoder/mod.rs:408-411; ARM ARM FP 1-source FABS
```

## encode_fabs_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM FP 1-source layout for FABS (opcode=000001). Stronger differential is a sibling property; this pins field placement independently of llvm-mc. Doc evidence: fp_scalar.rs:98 purpose comment; ARM ARM Floating-point data-processing (1 source).
- Seed: fp_scalar.rs encode_fp_1src_arm_fields
- Formal: ∀ rd,rn ∈ 0..31, is_d ∈ {false,true}. let w = encode_fabs([Reg(fp(is_d,rd)), Reg(fp(is_d,rn))]). Then bits[31:24]=00011110 ∧ bits[23:22]=ftype(is_d) ∧ bit21=1 ∧ bits[20:15]=000001 ∧ bits[14:10]=10000 ∧ bits[9:5]=rn ∧ bits[4:0]=rd.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fabs
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_d]
  domain: { rd: 0..31, rn: 0..31, is_d: bool }
  relation:
    op: eq
    lhs: encode_fabs([Reg(fp(is_d, rd)), Reg(fp(is_d, rn))])
    rhs: (0b00011110u32 << 24) | (ftype(is_d) << 22) | (1 << 21) | (0b000001 << 15) | (0b10000 << 10) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
evidence: fp_scalar.rs:98 purpose comment; ARM ARM FP 1-source FABS opcode 000001
```

## encode_fabs_metamorphic_fields
- Tier: 4
- Rationale: Algebraic metamorphic: Rd+1 / Rn+1 / S-vs-D must flip only the corresponding field. Required metamorphic property at standard tier. Doc evidence: ARM ARM field layout Rd[4:0] Rn[9:5] ftype[23:22].
- Seed: fp_scalar.rs encode_fp_1src_metamorphic_fields
- Formal: ∀ rd,rn ∈ 0..30, is_d ∈ {false,true}. let w = encode_fabs(rd,rn,is_d). encode_fabs(rd+1,rn,is_d) = w+1 ∧ encode_fabs(rd,rn+1,is_d) = w+(1<<5) ∧ encode_fabs(rd,rn,!is_d) XOR w = (1<<22).
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fabs
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_d]
  domain: { rd: 0..30, rn: 0..30, is_d: bool }
  body: encode_fabs(rd+1,rn,is_d)==w+1 && encode_fabs(rd,rn+1,is_d)==w+(1<<5) && (encode_fabs(rd,rn,!is_d)^w)==(1<<22)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_d: { gen: bool }
evidence: ARM ARM FP 1-source Rd[4:0] Rn[9:5] ftype[23:22]
```

## encode_fabs_neg_arity
- Tier: 4
- Rationale: Negative/error contract: FABS requires two operands. llvm-mc rejects too-few-operands; README.md:11 gas-compatible. get_reg errors on missing slots.
- Seed: fp_scalar.rs encode_fp_1src_neg_arity
- Formal: ∀ len ∈ {0,1}, n ∈ 0..31. encode_fabs(ops) is Err when |ops|=len < 2 with FP registers.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fabs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n]
  domain: { len: 0..1, n: 0..31 }
  relation:
    op: throws
    lhs: encode_fabs(ops_of_len(len, n))
    rhs: Err
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:11; llvm-mc too few operands; get_reg missing-slot Err
```

## encode_fabs_neg_extra_operand
- Tier: 4
- Rationale: Negative/error: FABS has no 3rd operand. llvm-mc rejects `fabs s0, s1, s2`. README.md:11 gas-compatible. encode_fabs currently does not check operands.len() — expected to fail.
- Seed: fp_scalar.rs encode_fp_1src_neg_extra_operand
- Formal: ∀ rd,rn ∈ 0..31, is_d ∈ {false,true}, extra ∈ Operand. encode_fabs([Reg(fp(is_d,rd)), Reg(fp(is_d,rn)), extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_d=false, extra=Reg("s0")
- Bug report: pbt-out/bug_reports/encode_fabs_extra_operand.md

```property
function: encoder.fp_scalar.encode_fabs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, extra]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, extra: Operand }
  relation:
    op: throws
    lhs: encode_fabs([Reg(fp(is_d, rd)), Reg(fp(is_d, rn)), extra])
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  extra: { gen: oneof, items: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: README.md:11; llvm-mc extra operand error
```

## encode_fabs_neg_wrong_types
- Tier: 4
- Rationale: Negative/error: FABS requires matching Sd,Sn or Dd,Dn (or Hd,Hn). Mixed S/D, GPR, Q/V/B, SP/WSP are invalid (llvm-mc "invalid operand"). README.md:11.
- Seed: fp_scalar.rs encode_fp_1src_neg_wrong_types
- Formal: ∀ (dest, src) ∈ mixed-S/D ∪ GPR∪FP ∪ Q/V/B ∪ SP/WSP. encode_fabs([Reg(dest), Reg(src)]) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: dest="s0", src="d0"
- Bug report: pbt-out/bug_reports/encode_fabs_wrong_types.md

```property
function: encoder.fp_scalar.encode_fabs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, src]
  domain: { dest: wrong_fp_name, src: wrong_fp_name }
  relation:
    op: throws
    lhs: encode_fabs([Reg(dest), Reg(src)])
    rhs: Err
generators:
  dest: { gen: string }
  src: { gen: string }
expected_error: String
evidence: README.md:11; llvm-mc invalid operand for mixed/GPR/SP; ARM FABS Sd,Sn | Dd,Dn | Hd,Hn
```

## encode_fabs_diff_half
- Tier: 2
- Rationale: Differential vs llvm-mc +fullfp16 for half-precision FABS (ftype=11). ARM FABS Hd,Hn is a documented valid form. encode_fabs currently treats non-D as ftype=00 — expected to fail.
- Seed: fp_scalar.rs encode_fp_1src_diff_half
- Formal: ∀ rd,rn ∈ 0..31. encode_fabs([Reg("h"+rd), Reg("h"+rn)]) = llvm-mc -mattr=+fullfp16 ("fabs h{rd}, h{rn}").
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0 (fabs h0, h0: SUT=0x1e20c000 llvm-mc=0x1ee0c000)
- Bug report: pbt-out/bug_reports/encode_fabs_half_ftype.md

```property
function: encoder.fp_scalar.encode_fabs
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_fabs([Reg("h"+rd), Reg("h"+rn)])
    rhs: llvm_mc_fp16("fabs h{rd}, h{rn}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM FABS Hd,Hn ftype=11; llvm-mc -mattr=+fullfp16; README.md:223 fabs
```

## encode_fabs_neg_nonreg
- Tier: 4
- Rationale: Negative/error: non-register operand kinds must Err. get_reg returns "expected register" for Imm/Symbol/Label/Mem/Cond/Shift.
- Seed: fp_scalar.rs encode_fp_1src_neg_nonreg
- Formal: ∀ which ∈ {0,1}, kind ∈ {Imm, Symbol, Label, Mem, Cond, Shift}. encode_fabs(ops with slot which replaced) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fabs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..1, kind: 0..5 }
  relation:
    op: throws
    lhs: encode_fabs(ops_with_nonreg(which, kind))
    rhs: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: get_reg encoder/mod.rs:956-965 expected register
```

## encode_fabs_neg_invalid_name
- Tier: 4
- Rationale: Sweep — documented parse_reg_num/get_reg error path for names outside x/w/d/s/q/v/h/b0-31. Not reached by nonreg (wrong Operand kind) or wrong_types (valid prefixes). Doc evidence: encoder/mod.rs:131-147 parse_reg_num returns None for foo/s32/empty.
- Seed: fp_scalar.rs encode_fp_1src_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, s32, d32, h32, x32, r0, s, d, ""}. encode_fabs(ops with slot which = Reg(name)) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fabs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_reg_name }
  relation:
    op: throws
    lhs: encode_fabs(ops_with_invalid_name(which, name))
    rhs: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num None for out-of-range/unknown names
```
