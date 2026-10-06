# Properties: encode_fnmadd_fnmsub

## encode_fnmadd_fnmsub_diff_valid
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the valid FNMADD/FNMSUB S/D domain. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FNMADD decoder). encode_fmadd_fmsub rejected as same-job sibling (o1=0 non-negated 3-source class). encode_fp_arith / encode_madd rejected (different ARM class). Weaker: ARM field invariant, field metamorphic, negative_error.
- Doc contract: fp_scalar.rs:143 "Encode FNMADD/FNMSUB: Rd = -Ra +/- (Rn * Rm)" — asserted fingerprint d2bebfec
- Seed: fp_scalar.rs encode_fmadd_fmsub_diff_valid
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_d ∈ {S,D}, is_sub ∈ {false,true}, spellings ∈ {lower,UPPER}. encode_fnmadd_fnmsub([Rd,Rn,Rm,Ra], is_sub) = llvm-mc("fnmadd|fnmsub Rd, Rn, Rm, Ra") as u32 LE word
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d, is_sub]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", ra: "0..31", is_d: bool, is_sub: bool }
  relation:
    op: eq
    lhs: encode_fnmadd_fnmsub([Reg(fp(is_d,rd)), Reg(fp(is_d,rn)), Reg(fp(is_d,rm)), Reg(fp(is_d,ra))], is_sub)
    rhs: llvm_mc_word(fnmadd_or_fnmsub(is_sub), fp(is_d,rd), fp(is_d,rn), fp(is_d,rm), fp(is_d,ra))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  is_sub: { gen: bool }
evidence: fp_scalar.rs:143 asserted FNMADD/FNMSUB encoding; encoder/mod.rs:565-566 dispatch
```

## encode_fnmadd_fnmsub_arm_fields
- Tier: 4
- Rationale: ARM ARM Floating-point data-processing (3 source) field layout with o1=1 is an exact structural invariant of every successful encoding. Stronger differential is p1; this pins each field independently so a coincidental 32-bit match cannot hide a swapped Rm/Ra.
- Doc contract: fp_scalar.rs:144 "Format: 0 00 11111 ftype 1 Rm o1 Ra Rn Rd" — asserted fingerprint 759209f1
- Seed: fp_scalar.rs encode_fmadd_fmsub_arm_fields
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_d ∈ {S,D}, is_sub ∈ {false,true}. let w = encode_fnmadd_fnmsub(...). bits[31:24]=0b00011111 ∧ bits[23:22]=ftype(is_d) ∧ bit21=1 ∧ bits[20:16]=rm ∧ bit15=is_sub ∧ bits[14:10]=ra ∧ bits[9:5]=rn ∧ bits[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d, is_sub]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", ra: "0..31", is_d: bool, is_sub: bool }
  body: word_fields_match_arm_fp3src_o1_1(encode_fnmadd_fnmsub(ops, is_sub))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  is_sub: { gen: bool }
evidence: fp_scalar.rs:144 ARM format comment
```

## encode_fnmadd_fnmsub_metamorphic_fields
- Tier: 4
- Rationale: A single-field increment of Rd/Rn/Rm/Ra, S↔D, or FNMADD↔FNMSUB must flip only the corresponding ARM bit. Evidenced by the ARM 3-source layout (disjoint fields). Stronger differential is p1; this catches field-aliasing bugs a whole-word match can miss.
- Doc contract: fp_scalar.rs:144 "Format: 0 00 11111 ftype 1 Rm o1 Ra Rn Rd" — asserted fingerprint 759209f1
- Seed: fp_scalar.rs encode_fmadd_fmsub_metamorphic_fields
- Formal: ∀ rd,rn,rm,ra ∈ {0..30}, is_d ∈ {S,D}. let w = FNMADD(rd,rn,rm,ra,is_d). FNMADD(rd+1,...)=w+1 ∧ FNMADD(...,rn+1,...)=w+(1<<5) ∧ FNMADD(...,ra+1,...)=w+(1<<10) ∧ FNMADD(...,rm+1,...)=w+(1<<16) ∧ (FNMADD(...,¬is_d) XOR w)=1<<22 ∧ (FNMSUB XOR FNMADD)=1<<15
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d]
  domain: { rd: "0..30", rn: "0..30", rm: "0..30", ra: "0..30", is_d: bool }
  body: field_isolation(encode_fnmadd_fnmsub)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
  is_d: { gen: bool }
evidence: fp_scalar.rs:144 disjoint ARM fields
```

## encode_fnmadd_fnmsub_neg_arity
- Tier: 3
- Rationale: llvm-mc / gas reject FNMADD/FNMSUB with fewer than 4 operands ("too few operands"). get_reg on a missing index returns Err. No rustdoc restriction; the four-operand form is the function's own contract (Rd, Rn, Rm, Ra).
- Doc contract: fp_scalar.rs:143 "Encode FNMADD/FNMSUB: Rd = -Ra +/- (Rn * Rm)" — asserted fingerprint d2bebfec
- Seed: fp_scalar.rs encode_fmadd_fmsub_neg_arity
- Formal: ∀ len ∈ {0,1,2,3}, is_sub ∈ {false,true}. encode_fnmadd_fnmsub(ops[0..len], is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, is_sub]
  domain: { len: "0..3", is_sub: bool }
  relation:
    op: throws
    expr: encode_fnmadd_fnmsub(ops_of_len(len), is_sub)
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  is_sub: { gen: bool }
expected_error: String
evidence: fp_scalar.rs:143 four named registers Rd Rn Rm Ra; get_reg missing index Err
```

## encode_fnmadd_fnmsub_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc rejects a 5th operand ("invalid operand for instruction"). The function names four registers (Rd, Rn, Rm, Ra). Body does not check operands.len(); extra stays in the generator. Not a documented domain restriction.
- Doc contract: fp_scalar.rs:143 "Encode FNMADD/FNMSUB: Rd = -Ra +/- (Rn * Rm)" — asserted fingerprint d2bebfec
- Seed: fp_scalar.rs encode_fmadd_fmsub_neg_extra_operand
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_d ∈ {S,D}, is_sub ∈ {false,true}, extra ∈ ExtraOperand. encode_fnmadd_fnmsub([Rd,Rn,Rm,Ra,extra], is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: failing
- Counterexample: encode_fnmadd_fnmsub([Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0"), Reg("s0")], false) → Ok(Word) (serial, --test-threads=1)
- Bug report: bug_reports/encode_fnmadd_fnmsub_extra_operand.md

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d, is_sub, extra]
  domain: { extra: "Reg|Imm|Shift|RegArrangement" }
  relation:
    op: throws
    expr: encode_fnmadd_fnmsub([Rd,Rn,Rm,Ra,extra], is_sub)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  is_sub: { gen: bool }
  extra: { gen: oneof, items: ["Reg(sN)", "Imm(0)", "Imm(1)", "Imm(32)", "Shift", "RegArrangement"] }
expected_error: String
evidence: fp_scalar.rs:143 four named registers; llvm-mc extra operand error
```

## encode_fnmadd_fnmsub_neg_wrong_types
- Tier: 3
- Rationale: llvm-mc rejects mixed S/D, GPR, Q/V/B, and SP/WSP in any of the four slots. ARM 3-source FNMADD requires matching Sd/Dd/Hd quadruples. No rustdoc restriction. Body only inspects dest prefix for ftype and parse_reg_num accepts GP/SP/Q/V/B.
- Doc contract: fp_scalar.rs:143 "Encode FNMADD/FNMSUB: Rd = -Ra +/- (Rn * Rm)" — asserted fingerprint d2bebfec
- Seed: fp_scalar.rs encode_fmadd_fmsub_neg_wrong_types
- Formal: ∀ (dest,n,m,a) ∈ WrongTypeQuad, is_sub ∈ {false,true}. encode_fnmadd_fnmsub([dest,n,m,a], is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: failing
- Counterexample: encode_fnmadd_fnmsub([Reg("d0"), Reg("s0"), Reg("s0"), Reg("s0")], false) → Ok(Word) (serial, --test-threads=1)
- Bug report: bug_reports/encode_fnmadd_fnmsub_wrong_types.md

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, src_n, src_m, src_a, is_sub]
  domain: { dest,src_n,src_m,src_a: "mixed S/D | GPR | Q/V/B | SP/WSP | all-GPR" }
  relation:
    op: throws
    expr: encode_fnmadd_fnmsub([Reg(dest),Reg(src_n),Reg(src_m),Reg(src_a)], is_sub)
generators:
  dest: { gen: string }
  src_n: { gen: string }
  src_m: { gen: string }
  src_a: { gen: string }
  is_sub: { gen: bool }
expected_error: String
evidence: fp_scalar.rs:143 FP fused form; ARM ARM matching Sd/Dd/Hd quadruples
```

## encode_fnmadd_fnmsub_diff_half
- Tier: 5
- Rationale: ARM ftype=11 (H) is a documented encoding of the same 3-source class. The format comment names ftype without restricting it to S/D. llvm-mc with +fullfp16 accepts matching H quadruples. Body `starts_with('d')` else ftype=00 does not declare H invalid. Differential vs llvm-mc is the strongest oracle on this sub-domain.
- Doc contract: fp_scalar.rs:144 "Format: 0 00 11111 ftype 1 Rm o1 Ra Rn Rd" — asserted fingerprint 759209f1
- Seed: fp_scalar.rs encode_fmadd_fmsub_diff_half
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_sub ∈ {false,true}. encode_fnmadd_fnmsub([h_rd,h_rn,h_rm,h_ra], is_sub) = llvm-mc -mattr=+fullfp16 ("fnmadd|fnmsub Hd, Hn, Hm, Ha")
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: failing
- Counterexample: encode_fnmadd_fnmsub([Reg("h0"), Reg("h0"), Reg("h0"), Reg("h0")], false)
- Bug report: bug_reports/encode_fnmadd_fnmsub_half_ftype.md

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_sub]
  domain: { rd: "0..31", rn: "0..31", rm: "0..31", ra: "0..31", is_sub: bool }
  relation:
    op: eq
    lhs: encode_fnmadd_fnmsub([Reg(h{rd}),Reg(h{rn}),Reg(h{rm}),Reg(h{ra})], is_sub)
    rhs: llvm_mc_fp16_word(fnmadd_or_fnmsub(is_sub), h{rd}, h{rn}, h{rm}, h{ra})
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
evidence: fp_scalar.rs:144 ftype field; ARM ARM ftype=11 half
```

## encode_fnmadd_fnmsub_neg_nonreg
- Tier: 3
- Rationale: get_reg requires Operand::Reg; llvm-mc rejects immediate/mem/shift/cond/symbol/label in any of the four slots. Missing-register Err from get_reg is the documented failure shape.
- Doc contract: fp_scalar.rs:143 "Encode FNMADD/FNMSUB: Rd = -Ra +/- (Rn * Rm)" — asserted fingerprint d2bebfec
- Seed: fp_scalar.rs encode_fmadd_fmsub_neg_nonreg
- Formal: ∀ which ∈ {0,1,2,3}, bad ∈ {Imm, Symbol, Label, Mem, Cond, Shift}, is_sub ∈ {false,true}. encode_fnmadd_fnmsub(ops with slot `which` = bad, is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind, is_sub]
  domain: { which: "0..3", kind: "Imm|Symbol|Label|Mem|Cond|Shift", is_sub: bool }
  relation:
    op: throws
    expr: encode_fnmadd_fnmsub(ops_with_nonreg_at(which, kind), is_sub)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
  is_sub: { gen: bool }
expected_error: String
evidence: get_reg expected register at operand; fp_scalar.rs:143 register operands
```

## encode_fnmadd_fnmsub_neg_invalid_name
- Tier: 3
- Rationale: Sweep — unparsable register names (foo, s32, empty, r0) must Err via parse_reg_num. Contract surface of get_reg used by this function. Added after the first full run to cover the remaining documented error path.
- Doc contract: fp_scalar.rs:143 "Encode FNMADD/FNMSUB: Rd = -Ra +/- (Rn * Rm)" — asserted fingerprint d2bebfec
- Seed: fp_scalar.rs encode_fmadd_fmsub_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2,3}, name ∈ {foo,s32,d32,h32,x32,r0,s,d,""}, is_sub ∈ {false,true}. encode_fnmadd_fnmsub(ops with slot `which` = Reg(name), is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/encode_fnmadd_fnmsub_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fnmadd_fnmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name, is_sub]
  domain: { which: "0..3", name: "{foo,s32,d32,h32,x32,r0,s,d,empty}", is_sub: bool }
  relation:
    op: throws
    expr: encode_fnmadd_fnmsub(ops_with_name_at(which, name), is_sub)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  name: { gen: string }
  is_sub: { gen: bool }
expected_error: String
evidence: parse_reg_num None → get_reg Err invalid register
```
