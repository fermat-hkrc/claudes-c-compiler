# Properties: encode_fp_arith

## encode_fp_arith_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FP 2-source decoder). Same-job sibling gate fails for encode_neon_float_three_same (vector), encode_fmadd_fmsub (4-operand fused), encode_fp_1src / encode_fneg (1-source). Doc evidence: README.md:11 GNU-style gas contract; README.md:223 lists the eight scalar mnemonics; encoder/mod.rs:377-404 dispatch; ARM ARM FP data-processing (2 source).
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_diff_valid
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_d ∈ Bool, idx ∈ {0..7}, dest_kind,src_kind,rm_kind ∈ {0,1}. let mnem,opcode = ARITH[idx]; let Rd = fp_spelling(is_d,rd,dest_kind); Rn = fp_spelling(is_d,rn,src_kind); Rm = fp_spelling(is_d,rm,rm_kind). llvm-mc(mnem Rd, Rn, Rm) = encode_fp_arith([Reg(Rd),Reg(Rn),Reg(Rm)], opcode) as Word.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_arith
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_d, idx]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_d: bool, idx: 0..7 }
  relation:
    op: eq
    lhs: encode_fp_arith([Reg(fp(is_d,rd)), Reg(fp(is_d,rn)), Reg(fp(is_d,rm))], ARITH[idx].opcode)
    rhs: llvm_mc(ARITH[idx].mnem + " " + fp(is_d,rd) + ", " + fp(is_d,rn) + ", " + fp(is_d,rm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  idx: { gen: int, min: 0, max: 7, type: usize }
evidence: README.md:11 GNU-style gas contract; encoder/mod.rs:377-404; ARM ARM FP 2-source
```

## encode_fp_arith_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM 2-source field layout. Stronger differential already used on the valid domain; this pins each bit-field independently so a coincidental 32-bit match cannot hide a swapped Rn/Rm.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_d ∈ Bool, idx ∈ {0..7}. let opcode = ARITH[idx].opcode; ftype = is_d ? 0b01 : 0b00; w = encode_fp_arith([S/D rd,rn,rm], opcode). Then w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(rm<<16)|(opcode<<12)|(0b10<<10)|(rn<<5)|rd, and bits[31:24]=00011110, bits[23:22]=ftype, bit21=1, bits[20:16]=rm, bits[15:12]=opcode, bits[11:10]=10, bits[9:5]=rn, bits[4:0]=rd.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_arith
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_d, idx]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_d: bool, idx: 0..7 }
  relation:
    op: eq
    lhs: encode_fp_arith([Reg(fp(is_d,rd)), Reg(fp(is_d,rn)), Reg(fp(is_d,rm))], ARITH[idx].opcode)
    rhs: (0b00011110u32 << 24) | (ftype << 22) | (1 << 21) | (rm << 16) | (opcode << 12) | (0b10 << 10) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  idx: { gen: int, min: 0, max: 7, type: usize }
evidence: ARM ARM Floating-point data-processing (2 source); fp_scalar.rs:76 comment
```

## encode_fp_arith_metamorphic_fields
- Tier: 4
- Rationale: Algebraic metamorphic: independent field updates must flip only the corresponding bits. Documented bounds 0..31 sampled at 30 and 31 via rd in 0..30 plus +1. Weaker than differential; used to isolate field packing bugs.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_metamorphic_fields
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_d ∈ Bool. let w = encode_fp_arith(S/D rd,rn,rm, FADD). Then encode(rd+1,rn,rm)=w+1; encode(rd,rn+1,rm)=w+(1<<5); encode(rd,rn,rm+1)=w+(1<<16); encode(S↔D) xor w = 1<<22; encode(FSUB) xor w = 1<<12.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_arith
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_d]
  domain: { rd: 0..30, rn: 0..30, rm: 0..30, is_d: bool }
  body: encode(rd+1)=w+1 AND encode(rn+1)=w+(1<<5) AND encode(rm+1)=w+(1<<16) AND (S xor D)=1<<22 AND (FADD xor FSUB)=1<<12
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_d: { gen: bool }
evidence: ARM ARM FP 2-source field positions Rd[4:0] Rn[9:5] Rm[20:16] opcode[15:12] ftype[23:22]
```

## encode_fp_arith_neg_arity
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc/gas reject fewer than 3 operands ("too few operands"). get_reg on missing index must Err. Documented 3-operand form in ARM ARM and README.md:223.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_arity
- Formal: ∀ len ∈ {0,1,2}, n ∈ {0..31}. encode_fp_arith(ops of length len using S-regs, FADD) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n]
  domain: { len: 0..2, n: 0..31 }
  relation:
    op: throws
    expr: encode_fp_arith(ops[0..len], 0b0010)
expected_error: String
generators:
  len: { gen: int, min: 0, max: 2, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc/gas reject too few operands; ARM ARM FADD Sd, Sn, Sm is 3-operand
```

## encode_fp_arith_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc/gas reject a 4th operand. The GNU-style assembler contract (README.md:11) requires the same rejection. encode_fp_arith currently ignores extras via get_reg by index only.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_extra_operand
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_d ∈ Bool, extra ∈ ExtraOperand. encode_fp_arith([S/D rd,rn,rm, extra], FADD) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, is_d=false, extra=Reg("s0") — encode_fp_arith([s0,s0,s0,s0], FADD) is Ok(Word); serial reconfirm PBT_TEST_JOBS=1
- Bug report: pbt-out/bug_reports/encode_fp_arith_extra_operand.md

```property
function: encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_d, extra]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, is_d: bool, extra: ExtraOperand }
  relation:
    op: throws
    expr: encode_fp_arith([Reg(fp(is_d,rd)), Reg(fp(is_d,rn)), Reg(fp(is_d,rm)), extra], 0b0010)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  extra: { gen: oneof, options: [Reg(sN), Imm(0), Imm(1), Imm(32), Shift, RegArrangement] }
evidence: llvm-mc/gas reject extra operands for scalar FADD; README.md:11
```

## encode_fp_arith_neg_wrong_types
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc/gas reject mixed S/D, GPR, Q/V/B, and SP/WSP for scalar FP 2-source. ARM ARM requires matching Sd,Sn,Sm or Dd,Dn,Dm (or Hd,Hn,Hm).
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_wrong_types
- Formal: ∀ (Rd,Rn,Rm) drawn from mixed-S/D, GPR, Q/V/B, or SP/WSP triples. encode_fp_arith([Reg(Rd),Reg(Rn),Reg(Rm)], FADD) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: dest="d0", src="s0", src2="s0" — mixed S/D encodes instead of Err; serial reconfirm PBT_TEST_JOBS=1. Also GPR (x0,s1,s2) and SP (sp,s1,s2).
- Bug report: pbt-out/bug_reports/encode_fp_arith_wrong_types.md

```property
function: encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm]
  domain: { (rd,rn,rm): mixed_sd | gpr | qvb | sp }
  relation:
    op: throws
    expr: encode_fp_arith([Reg(rd), Reg(rn), Reg(rm)], 0b0010)
expected_error: String
generators:
  triple: { gen: oneof, options: [mixed_sd, gpr_slot, qvb_slot, sp_slot] }
evidence: ARM ARM FADD matching-type 3-reg; llvm-mc rejects mixed/GPR/QVB/SP
```

## encode_fp_arith_diff_half
- Tier: 2
- Rationale: Differential vs llvm-mc +fullfp16 for the documented H form (ARM ftype=11). README.md:11 gas contract; ARM ARM ftype 11=H. SUT currently treats any non-d prefix as ftype=00 (S), so H is a documented-valid path that must match llvm-mc, not a crash-only check.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_diff_half
- Formal: ∀ rd,rn,rm ∈ {0..31}, idx ∈ {0..7}. llvm-mc -mattr=+fullfp16 (mnem Hd, Hn, Hm) = encode_fp_arith([Reg(h{rd}),Reg(h{rn}),Reg(h{rm})], opcode) as Word.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, idx=0 — fmul h0,h0,h0: SUT 0x1e200800 (ftype=00) vs llvm-mc 0x1ee00800 (ftype=11); serial reconfirm PBT_TEST_JOBS=1
- Bug report: pbt-out/bug_reports/encode_fp_arith_half_ftype.md

```property
function: encode_fp_arith
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, idx]
  domain: { rd: 0..31, rn: 0..31, rm: 0..31, idx: 0..7 }
  relation:
    op: eq
    lhs: encode_fp_arith([Reg("h"+rd), Reg("h"+rn), Reg("h"+rm)], ARITH[idx].opcode)
    rhs: llvm_mc_fp16(ARITH[idx].mnem + " h" + rd + ", h" + rn + ", h" + rm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 7, type: usize }
evidence: ARM ARM ftype 11=H; llvm-mc -mattr=+fullfp16; README.md:11
```

## encode_fp_arith_neg_nonreg
- Tier: 4e
- Rationale: Negative/error contract. Non-register operand kinds (Imm/Symbol/Label/Mem/Cond/Shift) at any of the three slots must Err. get_reg already returns Err for non-Reg; this pins the contract rather than crash-only.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_nonreg
- Formal: ∀ which ∈ {0,1,2}, kind ∈ {Imm,Symbol,Label,Mem,Cond,Shift}. encode_fp_arith(S-reg triple with slot `which` replaced by kind, FADD) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..2, kind: {Imm,Symbol,Label,Mem,Cond,Shift} }
  relation:
    op: throws
    expr: encode_fp_arith(s_triple_with_slot(which, kind), 0b0010)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
evidence: get_reg expected-register error; llvm-mc rejects non-register operands
```

## encode_fp_arith_neg_invalid_name
- Tier: 4e
- Rationale: Sweep of the parse_reg_num failure arm (documented invalid names s32/empty/foo). Negative/error contract: llvm-mc rejects these as invalid operands. Stronger oracles do not apply to the invalid-name domain.
- Seed: fp_scalar.rs encode_fp_1src_pbt::encode_fp_1src_neg_invalid_name
- Formal: ∀ which ∈ {0,1,2}, name ∈ {foo,s32,d32,h32,x32,r0,s,d,""}. encode_fp_arith(S-reg triple with slot `which` = Reg(name), FADD) is Err.
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_arith
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..2, name: {foo,s32,d32,h32,x32,r0,s,d,""} }
  relation:
    op: throws
    expr: encode_fp_arith(s_triple_with_slot(which, Reg(name)), 0b0010)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: oneof, options: [foo, s32, d32, h32, x32, r0, s, d, empty] }
evidence: parse_reg_num returns None for these names; llvm-mc rejects them
```
