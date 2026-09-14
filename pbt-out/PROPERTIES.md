# Properties: encode_fp_1src

## encode_fp_1src_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree FRINT decoder). encode_fneg/encode_fabs/encode_fsqrt rejected as same-job siblings (FNEG/FABS/FSQRT, hardcoded opcodes). encode_neon_float_two_misc rejected (vector form). Doc evidence: README.md:11 GNU-style assembly; README.md:223 lists frintn/p/m/z/a/x/i; encoder/mod.rs:414-434 dispatch; ARM ARM Floating-point data-processing (1 source) FRINT* Sd/Dd, Sn/Dn.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_diff_valid
- Formal: ∀ rd,rn ∈ {0..31}, is_d ∈ {S,D}, (opcode,mnem) ∈ FRINT, spell ∈ {sN/dN, uppercase}. encode_fp_1src([Reg(spell(rd)), Reg(spell(rn))], opcode) = Word(w) ∧ llvm-mc(mnem+" "+asm) = w
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_1src
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, opcode]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, opcode: FRINT opcodes }
  relation:
    op: eq
    lhs: encode_fp_1src([Reg(fp(is_d, rd)), Reg(fp(is_d, rn))], opcode)
    rhs: llvm_mc_word(mnem + " " + fp(is_d, rd) + ", " + fp(is_d, rn))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  opcode: { gen: int, min: 0, max: 6, type: u32 }
evidence: README.md:11 GNU-style assembly; encoder/mod.rs:414-434 frint* => encode_fp_1src; ARM ARM FRINTN/P/M/Z/A/X/I Sd|Dd, Sn|Dn
```

## encode_fp_1src_arm_fields
- Tier: 4d
- Rationale: Algebraic invariant from ARM ARM field layout. Stronger differential already covered by encode_fp_1src_diff_valid. This pins the documented bitfields independently of llvm-mc. Doc evidence: fp_scalar.rs:115-116 purpose comment "Format: 0 00 11110 ftype 1 opcode 10000 Rn Rd"; ARM ARM M=0 S=0 11110 ftype 1 opcode 10000 Rn Rd. ftype 00=S 01=D.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, is_d ∈ {S,D}, opcode ∈ FRINT. encode_fp_1src(ops, opcode) = (0b00011110<<24)|(ftype<<22)|(1<<21)|(opcode<<15)|(0b10000<<10)|(rn<<5)|rd ∧ bits[31:24]=00011110 ∧ bits[14:10]=10000
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_1src
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, opcode]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, opcode: FRINT opcodes }
  relation:
    op: eq
    lhs: encode_fp_1src([Reg(fp(is_d, rd)), Reg(fp(is_d, rn))], opcode)
    rhs: (0b00011110u32 << 24) | (ftype << 22) | (1 << 21) | (opcode << 15) | (0b10000 << 10) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  opcode: { gen: int, min: 0, max: 6, type: u32 }
evidence: fp_scalar.rs:115-116 purpose comment; ARM ARM Floating-point data-processing (1 source)
```

## encode_fp_1src_metamorphic_fields
- Tier: 4c
- Rationale: Algebraic metamorphic. Rd/Rn/ftype/opcode occupy disjoint fields, so incrementing one must flip only that field. Round-trip rejected. Not a same-job differential vs encode_fneg (different mnemonic). Doc evidence: ARM ARM field layout; encoder/mod.rs:416-434 opcode table FRINTN=001000 FRINTP=001001 (bit 0 of opcode = bit 15).
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_metamorphic_fields
- Formal: ∀ rd,rn ∈ {0..30}, is_d ∈ {S,D}. let w=encode_fp_1src(rd,rn,is_d,FRINTN). encode(rd+1)=w+1 ∧ encode(rn+1)=w+(1<<5) ∧ encode(!is_d) XOR w = 1<<22 ∧ encode(FRINTP) XOR w = 1<<15
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_1src
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_d]
  domain: { rd: 0..30, rn: 0..30, is_d: bool }
  relation:
    op: eq
    lhs: encode_fp_1src(rd+1, rn, is_d, FRINTN)
    rhs: encode_fp_1src(rd, rn, is_d, FRINTN) + 1
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_d: { gen: bool }
evidence: ARM ARM FP 1-source Rd bits[4:0] Rn bits[9:5] ftype bits[23:22] opcode bits[20:15]; encoder/mod.rs:416 FRINTN vs 419 FRINTP
```

## encode_fp_1src_neg_arity
- Tier: 4e
- Rationale: Negative/error contract. FRINT* is a two-operand instruction; llvm-mc / gas reject fewer than 2 operands. get_reg(idx) must Err when the slot is missing. Doc evidence: README.md:11 gas-compatible; ARM ARM FRINTN <Sd>, <Sn>.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_arity
- Formal: ∀ len ∈ {0,1}, n ∈ {0..31}. encode_fp_1src(ops[0..len], FRINTN) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_1src
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n]
  domain: { len: 0..1, n: 0..31 }
  relation:
    op: throws
    lhs: encode_fp_1src(ops_of_len(len), 0b001000)
    rhs: Err
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:11 gas-compatible; ARM ARM FRINTN requires Sd, Sn; llvm-mc rejects too few operands
```

## encode_fp_1src_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract. Scalar FRINT* has exactly 2 operands; llvm-mc rejects a 3rd. Extra operands must Err, not be ignored. Doc evidence: README.md:11; ARM ARM two-operand syntax.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_extra_operand
- Formal: ∀ rd,rn ∈ {0..31}, is_d ∈ {S,D}, extra ∈ Operand. encode_fp_1src([fp(rd), fp(rn), extra], FRINTN) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_d=false, extra=Reg("s0")  [frintn s0, s0, s0]
- Bug report: pbt-out/bug_reports/encode_fp_1src_extra_operand.md

```property
function: encode_fp_1src
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, extra]
  domain: { rd: 0..31, rn: 0..31, extra: Operand }
  relation:
    op: throws
    lhs: encode_fp_1src([Reg(fp(rd)), Reg(fp(rn)), extra], 0b001000)
    rhs: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: string }
expected_error: String
evidence: README.md:11 gas-compatible; ARM ARM FRINTN two-operand; llvm-mc rejects extra operand
```

## encode_fp_1src_neg_wrong_types
- Tier: 4e
- Rationale: Negative/error contract. Scalar FRINT* requires matching FP registers Sd,Sn or Dd,Dn (not GPR, SP, Q/V/B, mixed S/D). llvm-mc / gas reject those. Doc evidence: ARM ARM FRINTN <Sd>,<Sn> | <Dd>,<Dn>; parse_reg_num accepting x/w/q/v/h/b/sp is not a license to encode them as S/D.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_wrong_types
- Formal: ∀ (dest,src) ∈ wrong_type_pair. encode_fp_1src([Reg(dest), Reg(src)], FRINTN) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: dest="s0", src="d0"  [frintn s0, d0]
- Bug report: pbt-out/bug_reports/encode_fp_1src_wrong_types.md

```property
function: encode_fp_1src
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, src]
  domain: { dest,src: GPR | SP | QVB | mixed S/D }
  relation:
    op: throws
    lhs: encode_fp_1src([Reg(dest), Reg(src)], 0b001000)
    rhs: Err
generators:
  dest: { gen: string }
  src: { gen: string }
expected_error: String
evidence: ARM ARM FRINTN Sd,Sn or Dd,Dn; README.md:11 gas-compatible; llvm-mc rejects GPR/SP/QVB/mixed
```

## encode_fp_1src_diff_half
- Tier: 2
- Rationale: Differential vs llvm-mc +fullfp16. ARM ARM ftype=11 for half-precision FRINT*. If the SUT accepts H registers (parse_reg_num maps h0-h31), the word must use ftype=11, not silently encode as single (ftype=00). Same-job as scalar FRINT*, not the NEON vector form. Doc evidence: ARM ARM ftype 00=S 01=D 11=H; llvm-mc `frintn h0, h1` = 0x1ee44020.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_diff_half
- Formal: ∀ rd,rn ∈ {0..31}, (opcode,mnem) ∈ FRINT. encode_fp_1src([Reg(h{rd}), Reg(h{rn})], opcode) = llvm-mc-fp16(mnem+" h{rd}, h{rn}")
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, idx=0  [frintn h0, h0]; SUT=0x1e244000 llvm-mc=0x1ee44000
- Bug report: pbt-out/bug_reports/encode_fp_1src_half_ftype.md

```property
function: encode_fp_1src
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, opcode]
  domain: { rd: 0..31, rn: 0..31, opcode: FRINT opcodes }
  relation:
    op: eq
    lhs: encode_fp_1src([Reg("h"+rd), Reg("h"+rn)], opcode)
    rhs: llvm_mc_fp16_word(mnem + " h" + rd + ", h" + rn)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  opcode: { gen: int, min: 0, max: 6, type: u32 }
evidence: ARM ARM FP 1-source ftype=11 half; llvm-mc -mattr=+fullfp16 frintn h0,h1 = 0x1ee44020
```

## encode_fp_1src_neg_nonreg_invalid_name
- Tier: 4e
- Rationale: Negative/error contract. Non-register operand kinds and invalid names (foo, s32, d32, empty, r0) must Err. llvm-mc rejects them. Doc evidence: get_reg requires Operand::Reg; parse_reg_num returns None outside x/w/d/s/q/v/h/b 0..31 and sp/xzr/lr aliases.
- Seed: fp_scalar.rs encode_fcvt_rounding_pbt::encode_fcvt_rounding_neg_nonreg / encode_fcvt_rounding_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, bad ∈ nonreg_kinds ∪ invalid_names. encode_fp_1src(ops with slot which = bad, FRINTN) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_fp_1src
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, bad]
  domain: { which: 0..1, bad: Imm|Symbol|Label|Mem|Cond|Shift|foo|s32|d32|empty|r0 }
  relation:
    op: throws
    lhs: encode_fp_1src(ops_with_slot(which, bad), 0b001000)
    rhs: Err
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: string }
expected_error: String
evidence: get_reg requires Operand::Reg; parse_reg_num None for invalid names; llvm-mc rejects non-register FRINT operands
```
