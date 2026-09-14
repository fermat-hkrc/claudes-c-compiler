# Properties: encode_fmadd_fmsub

## encode_fmadd_fmsub_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FMADD/FMSUB decoder). Sibling encode_fnmadd_fnmsub rejected (same-job gate: o1=1 negated fused class). Sibling encode_fp_arith rejected (2-source FP). Sibling encode_madd rejected (integer MADD). Doc evidence: README.md:11 GNU-style gas contract; README.md:223 lists fmadd/fmsub; encoder/mod.rs:435-436 dispatch; ARM ARM Floating-point data-processing (3 source).
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_diff_valid
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_d ∈ Bool, is_sub ∈ Bool, dest/src spellings ∈ {sN|dN|SN|DN}. let mnem = is_sub ? fmsub : fmadd. encode_fmadd_fmsub([Reg(Rd),Reg(Rn),Reg(Rm),Reg(Ra)], is_sub) = Word(w) ∧ w = llvm-mc("-triple=aarch64 -show-encoding", "{mnem} Rd, Rn, Rm, Ra")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d, is_sub]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, is_d: bool, is_sub: bool }
  relation:
    op: eq
    lhs: encode_fmadd_fmsub([Reg(fp(is_d,rd)),Reg(fp(is_d,rn)),Reg(fp(is_d,rm)),Reg(fp(is_d,ra))], is_sub)
    rhs: llvm_mc_word("{fmadd|fmsub} fp(is_d,rd), fp(is_d,rn), fp(is_d,rm), fp(is_d,ra)")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  is_sub: { gen: bool }
evidence: README.md:11 GNU-style gas contract; encoder/mod.rs:435-436; ARM ARM FP 3-source
```

## encode_fmadd_fmsub_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM 3-source field layout, independent of the SUT body. Stronger differential covers the valid domain separately; this pins each named field (M/S/11111/ftype/o1/Rm/o0/Ra/Rn/Rd) so a swapped Ra/Rn packing cannot hide behind a matching total word. Weaker than differential because it does not compare against an independent assembler. Doc evidence: ARM ARM Floating-point data-processing (3 source); fp_scalar.rs:128 purpose comment (format, not the producing assignment).
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_arm_fields
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_d ∈ Bool, is_sub ∈ Bool. let w = encode_fmadd_fmsub([s/d rd,rn,rm,ra], is_sub) as Word. bits[31:24]=00011111 ∧ bits[23:22]=ftype(is_d) ∧ bit[21]=0 ∧ bits[20:16]=rm ∧ bit[15]=is_sub ∧ bits[14:10]=ra ∧ bits[9:5]=rn ∧ bits[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d, is_sub]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, is_d: bool, is_sub: bool }
  relation:
    op: eq
    lhs: encode_fmadd_fmsub([Reg(fp(is_d,rd)),Reg(fp(is_d,rn)),Reg(fp(is_d,rm)),Reg(fp(is_d,ra))], is_sub)
    rhs: (0b00011111<<24)|(ftype<<22)|(rm<<16)|(o0<<15)|(ra<<10)|(rn<<5)|rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  is_sub: { gen: bool }
evidence: ARM ARM FP 3-source M=0 S=0 11111 ftype o1 Rm o0 Ra Rn Rd; fp_scalar.rs:128
```

## encode_fmadd_fmsub_metamorphic_fields
- Tier: 3
- Rationale: Algebraic metamorphic: incrementing one register or flipping ftype/o0 must change only that field. Independent of reconstructing the full encoding word. Stronger differential already covers agreement with llvm-mc; this catches field-packing bugs that a constant offset error might still match a wrong-but-consistent layout. Doc evidence: ARM ARM field positions Rd[4:0] Rn[9:5] Ra[14:10] o0[15] Rm[20:16] ftype[23:22].
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_metamorphic_fields
- Formal: ∀ rd,rn,rm,ra ∈ {0..30}, is_d ∈ Bool. let w = encode_fmadd_fmsub(rd,rn,rm,ra,is_d,false). encode(..rd+1..) = w+1 ∧ encode(..rn+1..) = w+(1<<5) ∧ encode(..ra+1..) = w+(1<<10) ∧ encode(..rm+1..) = w+(1<<16) ∧ encode(..!is_d..) XOR w = 1<<22 ∧ encode(..is_sub=true..) XOR w = 1<<15
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, ra: 0..30, is_d: bool }
  relation:
    op: holds
    expr: encode(rd+1)==w+1 && encode(rn+1)==w+(1<<5) && encode(ra+1)==w+(1<<10) && encode(rm+1)==w+(1<<16) && (encode(!is_d)^w)==(1<<22) && (encode(is_sub=true)^w)==(1<<15)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  ra: { gen: int, min: 0, max: 30, type: u32 }
  is_d: { gen: bool }
evidence: ARM ARM FP 3-source field positions Rd[4:0] Rn[9:5] Ra[14:10] o0[15] Rm[20:16] ftype[23:22]
```

## encode_fmadd_fmsub_neg_arity
- Tier: 5
- Rationale: Negative/error contract: llvm-mc rejects FMADD/FMSUB with fewer than 4 operands ("too few operands"). get_reg on a missing index must Err. Stronger oracles do not apply to the invalid domain. Doc evidence: llvm-mc / GNU gas arity; ARM ARM FMADD <Sd>, <Sn>, <Sm>, <Sa> is a 4-operand instruction.
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_neg_arity
- Formal: ∀ len ∈ {0,1,2,3}, n ∈ {0..31}, is_sub ∈ Bool. encode_fmadd_fmsub(ops of length len with S-regs, is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n, is_sub]
  domain: { len: 0..3, n: 0..31, is_sub: bool }
  relation:
    op: throws
    expr: encode_fmadd_fmsub(ops[0..len], is_sub)
generators:
  len: { gen: int, min: 0, max: 3, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; ARM ARM FMADD 4-operand form
```

## encode_fmadd_fmsub_neg_extra_operand
- Tier: 5
- Rationale: Negative/error contract: llvm-mc rejects a 5th operand ("invalid operand"). GNU-style assembler must not silently ignore extra operands. Stronger oracles do not apply to the invalid domain. Doc evidence: llvm-mc rejects `fmadd s0, s1, s2, s3, s0`.
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_neg_extra_operand
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_d ∈ Bool, is_sub ∈ Bool, extra ∈ ExtraOperand. encode_fmadd_fmsub([Rd,Rn,Rm,Ra,extra], is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, is_d=false, is_sub=false, extra=Reg("s0") (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_fmadd_fmsub_extra_operand.md

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_d, is_sub, extra]
  domain: { rd: 0..31, extra: ExtraOperand }
  relation:
    op: throws
    expr: encode_fmadd_fmsub([Rd,Rn,Rm,Ra,extra], is_sub)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, variants: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc rejects fifth operand on fmadd; README.md:11 gas contract
```

## encode_fmadd_fmsub_neg_wrong_types
- Tier: 5
- Rationale: Negative/error contract: llvm-mc rejects mixed S/D, GPR, Q/V/B, and SP/WSP in any of the four slots. ARM ARM requires four matching S, D, or H registers. Stronger oracles do not apply to the invalid domain. Doc evidence: llvm-mc "invalid operand"; ARM ARM FMADD <Sd>, <Sn>, <Sm>, <Sa> (or D/H).
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_neg_wrong_types
- Formal: ∀ (rd,rn,rm,ra) in wrong-type quadruples (mixed S/D, GPR, QVB, SP in any slot, or all-GPR), is_sub ∈ Bool. encode_fmadd_fmsub([Reg(rd),Reg(rn),Reg(rm),Reg(ra)], is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: dest="d0", src_n="s0", src_m="s0", src_a="s0", is_sub=false (serial reconfirm PBT_TEST_JOBS=1). Also GPR dest x0 and SP as Ra.
- Bug report: pbt-out/bug_reports/encode_fmadd_fmsub_wrong_types.md

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_sub]
  domain: { regs: wrong_type_quad }
  relation:
    op: throws
    expr: encode_fmadd_fmsub([Reg(rd),Reg(rn),Reg(rm),Reg(ra)], is_sub)
generators:
  rd: { gen: string }
  is_sub: { gen: bool }
expected_error: String
evidence: llvm-mc invalid operand for mixed S/D, GPR, QVB, SP; ARM ARM matching S/D/H
```

## encode_fmadd_fmsub_diff_half
- Tier: 2
- Rationale: Differential vs llvm-mc with +fullfp16 for the documented H-register form (ftype=11). README.md:11 gas contract plus ARM ARM ftype 11=H. SUT currently keys ftype only on dest.starts_with('d'), so H is a documented valid encoding that must match llvm-mc, not a crash-only check. Stronger state machine / round-trip rejected as above.
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_diff_half
- Formal: ∀ rd,rn,rm,ra ∈ {0..31}, is_sub ∈ Bool. encode_fmadd_fmsub([h rd,rn,rm,ra], is_sub) = Word(w) ∧ w = llvm-mc("-triple=aarch64 -mattr=+fullfp16 -show-encoding", "{fmadd|fmsub} Hd, Hn, Hm, Ha")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, ra=0, is_sub=false — SUT 0x1f000000 vs llvm-mc 0x1fc00000 (serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_fmadd_fmsub_half_ftype.md

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, ra, is_sub]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, ra: 0..31, is_sub: bool }
  relation:
    op: eq
    lhs: encode_fmadd_fmsub([Reg(h{rd}),Reg(h{rn}),Reg(h{rm}),Reg(h{ra})], is_sub)
    rhs: llvm_mc_fp16_word("{fmadd|fmsub} h{rd}, h{rn}, h{rm}, h{ra}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  ra: { gen: int, min: 0, max: 31, type: u32 }
  is_sub: { gen: bool }
evidence: ARM ARM ftype 11=H; llvm-mc -mattr=+fullfp16; README.md:11
```

## encode_fmadd_fmsub_neg_nonreg
- Tier: 5
- Rationale: Negative/error contract: get_reg documents "expected register at operand idx"; llvm-mc rejects Imm/Symbol/Label/Mem/Cond/Shift in any of the four slots. Stronger oracles do not apply. Doc evidence: encoder/mod.rs:956-965 get_reg error; ARM ARM register operands.
- Seed: fp_scalar.rs encode_fp_arith_pbt encode_fp_arith_neg_nonreg
- Formal: ∀ which ∈ {0,1,2,3}, kind ∈ {Imm,Symbol,Label,Mem,Cond,Shift}, is_sub ∈ Bool. encode_fmadd_fmsub(four-op vector with slot `which` replaced by non-Reg, is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind, is_sub]
  domain: { which: 0..3, kind: nonreg }
  relation:
    op: throws
    expr: encode_fmadd_fmsub(ops_with_nonreg_at(which, kind), is_sub)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
  is_sub: { gen: bool }
expected_error: String
evidence: encoder/mod.rs:956-965 get_reg expected register; ARM ARM 4 register operands
```

## encode_fmadd_fmsub_neg_invalid_name
- Tier: 5
- Rationale: Sweep (coverage_gaps had no profraw; manual arm audit of parse_reg_num None). Negative/error contract: parse_reg_num returns None for names outside x|w|d|s|q|v|h|b0-31, so get_reg must Err. Doc evidence: encoder/mod.rs:131-147 parse_reg_num.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2,3}, name ∈ {foo,s32,d32,h32,x32,r0,s,d,empty}, is_sub ∈ Bool. encode_fmadd_fmsub(four-op vector with slot `which` = Reg(name), is_sub) is Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fmadd_fmsub
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name, is_sub]
  domain: { which: 0..3, name: invalid_reg_name, is_sub: bool }
  relation:
    op: throws
    expr: encode_fmadd_fmsub(ops_with_invalid_name_at(which, name), is_sub)
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  name: { gen: string }
  is_sub: { gen: bool }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num None for non x|w|d|s|q|v|h|b0-31
```
