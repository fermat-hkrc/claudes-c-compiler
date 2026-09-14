# Properties: encode_int_to_float

## encode_int_to_float_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (GNU-style assembler contract, README.md:11). State machine rejected (pure function). Algebraic round-trip rejected (no in-tree decoder). encode_fcvt_rounding / encode_neon_float_two_misc fail the same-job gate.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_diff_valid
- Formal: ∀ rd,rn ∈ 0..31, dest_d ∈ {S,D}, src64 ∈ {W,X}, signed ∈ {scvtf,ucvtf}, spelling ∈ {canonical, uppercase, w31/x31, wzr/xzr, lr}. encode_int_to_float([Reg(Sd|Dd), Reg(Wn|Xn)], signed) = Word(w) ∧ w = llvm-mc("scvtf|ucvtf Sd|Dd, Wn|Xn")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_int_to_float
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, dest_d, src64, is_signed]
  domain: { rd: 0..31, rn: 0..31, dest_d: bool, src64: bool, is_signed: bool }
  relation:
    op: eq
    lhs: encode_int_to_float([Reg(fp(dest_d,rd)), Reg(gpr(src64,rn))], is_signed)
    rhs: llvm_mc(scvtf_or_ucvtf + " " + fp(dest_d,rd) + ", " + gpr(src64,rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_d: { gen: bool }
  src64: { gen: bool }
  is_signed: { gen: bool }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:454-459; fp_scalar.rs:211-215
```

## encode_int_to_float_arm_fields
- Tier: 4
- Rationale: ARM ARM Conversion between floating-point and integer field layout is an exact structural invariant on success-path words. Weaker than differential (does not catch wrong opcode/class vs llvm-mc) but pins documented bit positions independently of the reference tool.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_arm_fields
- Formal: ∀ rd,rn ∈ 0..31, dest_d, src64, is_signed. let w = encode_int_to_float([Reg(Sd|Dd), Reg(Wn|Xn)], is_signed). Then w[31]=sf, w[30:29]=00, w[28:24]=11110, w[23:22]=ftype, w[21]=1, w[20:19]=00, w[18:16]=opcode (010 signed / 011 unsigned), w[15:10]=0, w[9:5]=rn, w[4:0]=rd.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_int_to_float
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, dest_d, src64, is_signed]
  domain: { rd: 0..31, rn: 0..31, dest_d: bool, src64: bool, is_signed: bool }
  relation:
    op: eq
    lhs: encode_int_to_float([Reg(fp(dest_d,rd)), Reg(gpr(src64,rn))], is_signed)
    rhs: (sf<<31)|(0b11110<<24)|(ftype<<22)|(1<<21)|(opcode<<16)|(rn<<5)|rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_d: { gen: bool }
  src64: { gen: bool }
  is_signed: { gen: bool }
evidence: fp_scalar.rs:211-215; ARM ARM Conversion between floating-point and integer
```

## encode_int_to_float_metamorphic_fields
- Tier: 4
- Rationale: Independent field packing implies Rd/Rn/sf/ftype/opcode mutations affect only their documented bits. Metamorphic (weaker than round-trip; no inverse exists).
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_metamorphic_fields
- Formal: ∀ rd,rn ∈ 0..30, dest_d, src64. let w = encode_int_to_float([Sd|Dd rd, Wn|Xn rn], signed=true). Then encode(rd+1)=w+1, encode(rn+1)=w+(1<<5), encode(src64 flip) XOR w = 1<<31, encode(dest_d flip) XOR w = 1<<22, encode(unsigned) XOR w = 1<<16.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_int_to_float
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, dest_d, src64]
  domain: { rd: 0..30, rn: 0..30, dest_d: bool, src64: bool }
  body: encode(rd+1)==w+1 AND encode(rn+1)==w+(1<<5) AND encode(sf_flip) XOR w == 1<<31 AND encode(ftype_flip) XOR w == 1<<22 AND encode(unsigned) XOR w == 1<<16
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  dest_d: { gen: bool }
  src64: { gen: bool }
evidence: fp_scalar.rs:211-215
```

## encode_int_to_float_neg_arity
- Tier: 5
- Rationale: Documented 2-operand requirement (fp_scalar.rs:211-217 "scvtf/ucvtf requires 2 operands"; llvm-mc "too few operands"). Negative/error contract.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_arity
- Formal: ∀ ops with |ops| ∈ {0,1}. encode_int_to_float(ops, _) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_int_to_float
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len]
  domain: { len: 0..1 }
  relation:
    op: throws
    expr: encode_int_to_float(ops_of_len(len), true)
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: fp_scalar.rs:216-217; llvm-mc too few operands
```

## encode_int_to_float_neg_extra_operand
- Tier: 5
- Rationale: Integer SCVTF/UCVTF is a 2-operand form. A 3rd operand is the fixed-point `#fbits` form (ARM bit21=0, different encoding). llvm-mc accepts `scvtf s0, w1, #8` as fixed-point (0x1e02e020), not integer. This function's job is integer conversion (bit21=1); extra must Err rather than silently drop the scale.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_extra_operand
- Formal: ∀ rd,rn ∈ 0..31, dest_d, src64, extra ∈ Operand. encode_int_to_float([Reg(Sd|Dd), Reg(Wn|Xn), extra], _) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: [Reg("s0"), Reg("w0"), Reg("x0")] is_signed=true (shrunk rd=0,rn=0,dest_d=false,src64=false,extra=Reg("x0"); serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_int_to_float_extra_operand.md

```property
function: encode_int_to_float
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, dest_d, src64, extra]
  domain: { rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    expr: encode_int_to_float([Reg(fp), Reg(gpr), extra], true)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_d: { gen: bool }
  src64: { gen: bool }
  extra: { gen: oneof, items: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: fp_scalar.rs:216-217; ARM ARM integer vs fixed-point (bit21); llvm-mc scvtf s0,w1,#8 = 0x1e02e020
```

## encode_int_to_float_neg_sp_src
- Tier: 5
- Rationale: llvm-mc rejects `scvtf s0, sp` / `scvtf s0, wsp` ("invalid operand"). ARM integer conversion uses ZR at register 31, not SP. parse_reg_num maps sp/wsp to 31, so the SUT may encode SP as ZR — that is a contract violation.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_sp_dest
- Formal: ∀ dest_d ∈ {S,D}, sp ∈ {sp,wsp}, rd ∈ 0..31. encode_int_to_float([Reg(Sd|Dd), Reg(sp)], _) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: [Reg("s0"), Reg("wsp")] is_signed=true (shrunk is_64=false, rd=0, dest_d=false; serial reconfirm PBT_TEST_JOBS=1)
- Bug report: pbt-out/bug_reports/encode_int_to_float_sp_src.md

```property
function: encode_int_to_float
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, dest_d, is_64]
  domain: { rd: 0..31, dest_d: bool, is_64: bool }
  relation:
    op: throws
    expr: encode_int_to_float([Reg(fp(dest_d,rd)), Reg(sp_or_wsp(is_64))], true)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  dest_d: { gen: bool }
  is_64: { gen: bool }
expected_error: String
evidence: llvm-mc "invalid operand" for scvtf s0, sp / wsp; ARM ARM Rn is Wn|Xn (31=ZR not SP)
```

## encode_int_to_float_neg_wrong_types
- Tier: 5
- Rationale: Integer SCVTF/UCVTF requires FP dest (Sd|Dd) and GP source (Wn|Xn). llvm-mc rejects GP dest (`scvtf w0, s1`). FP source (`scvtf s0, s1`) is SIMD-scalar (different encoding class 0x5e21d820), not this function's integer-to-float job. Q/V/B/H-as-source are invalid for the integer form.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_wrong_types
- Formal: ∀ (dest, src) ∈ wrong-type pairs (GP dest; FP source; Q/V/B dest or source). encode_int_to_float([Reg(dest), Reg(src)], _) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: [Reg("w0"), Reg("w0")] is_signed=true (shrunk; serial reconfirm PBT_TEST_JOBS=1). Also [Reg("s0"), Reg("s1")] (SIMD-scalar, not integer form).
- Bug report: pbt-out/bug_reports/encode_int_to_float_wrong_types.md

```property
function: encode_int_to_float
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, src]
  domain: { dest, src: wrong_type_pair }
  relation:
    op: throws
    expr: encode_int_to_float([Reg(dest), Reg(src)], true)
generators:
  dest: { gen: string }
  src: { gen: string }
expected_error: String
evidence: llvm-mc invalid operand for scvtf w0,s1; SIMD-scalar scvtf s0,s1 = 0x5e21d820 (different class); fp_scalar.rs:211 integer-to-float
```

## encode_int_to_float_diff_half
- Tier: 2
- Rationale: ARM ftype=11 is half-precision dest. llvm-mc +fullfp16 accepts `scvtf h0, w1` = 0x1ee20020. Documented ftype in the purpose comment lists S/D; H is the remaining ARM ftype for this encoding class and is a valid GNU-style operand. Differential vs llvm-mc.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_diff_half
- Formal: ∀ rd,rn ∈ 0..31, src64, is_signed. encode_int_to_float([Reg(Hd), Reg(Wn|Xn)], signed) = Word(w) ∧ w = llvm-mc -mattr=+fullfp16 ("scvtf|ucvtf Hd, Wn|Xn")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: [Reg("h0"), Reg("w0")] is_signed=false (shrunk rd=0,rn=0,src64=false,is_signed=false; serial reconfirm PBT_TEST_JOBS=1). SUT 0x1e230000 vs llvm-mc 0x1ee30000.
- Bug report: pbt-out/bug_reports/encode_int_to_float_half_ftype.md

```property
function: encode_int_to_float
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, src64, is_signed]
  domain: { rd: 0..31, rn: 0..31, src64: bool, is_signed: bool }
  relation:
    op: eq
    lhs: encode_int_to_float([Reg("h"+rd), Reg(gpr(src64,rn))], is_signed)
    rhs: llvm_mc_fp16(scvtf_or_ucvtf + " h" + rd + ", " + gpr(src64,rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  src64: { gen: bool }
  is_signed: { gen: bool }
evidence: ARM ARM ftype 11=H; llvm-mc -mattr=+fullfp16 scvtf h0,w1 = 0x1ee20020; README.md:11 GNU-style assembly
```

## encode_int_to_float_neg_nonreg
- Tier: 5
- Rationale: Sweep — get_reg (mod.rs:956-965) documents Err for a non-register at the dest or source slot. llvm-mc rejects Imm/Symbol/Label/Mem/Cond/Shift as SCVTF operands.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_nonreg
- Formal: ∀ which ∈ {0,1}, bad ∈ {Imm, Symbol, Label, Mem, Cond, Shift}. encode_int_to_float(ops with slot which = bad, _) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_int_to_float
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..1, kind: 0..5 }
  relation:
    op: throws
    expr: encode_int_to_float(ops_with_nonreg_at(which, kind), true)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956-965 get_reg; llvm-mc invalid operand
```

## encode_int_to_float_neg_invalid_name
- Tier: 5
- Rationale: Sweep — parse_reg_num returns None for names outside x/w/d/s/q/v/h/b 0..31 and aliases. get_reg then Err("invalid register").
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, x32, w32, s32, d32, r0, x, s, empty}. encode_int_to_float(ops with slot which = Reg(name), _) = Err(_)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_int_to_float
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_int_to_float(ops_with_name_at(which, name), true)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num; get_reg invalid register
```
