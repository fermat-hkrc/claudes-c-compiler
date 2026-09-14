# Properties: encode_fneg

## encode_fneg_diff_valid
- Tier: 2
- Rationale: Differential vs llvm-mc is the strongest evidenced oracle. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FNEG decoder). Sibling encode_fabs/encode_fsqrt rejected (same-job gate: ARM opcodes 000001 / 000011, not FNEG 000010). Sibling encode_fp_1src rejected (FRINT*). Sibling encode_neon_float_two_misc rejected (vector form). Doc evidence: README.md:12 GNU-style assembly contract; README.md:223 lists scalar fneg; encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:406-408 scalar dispatch.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_diff_valid
- Formal: ∀ rd,rn ∈ [0,31], is_d ∈ Bool, dest_kind,src_kind ∈ {0,1}. encode_fneg([Reg(spell(is_d,rd,dest_kind)), Reg(spell(is_d,rn,src_kind))]) = Word(w) ∧ llvm-mc("fneg " ++ spell ++ ", " ++ spell) = w
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fneg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, dest_kind, src_kind]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, dest_kind: 0..1, src_kind: 0..1 }
  relation:
    op: eq
    lhs: encode_fneg([Reg(fp_spelling(is_d, rd, dest_kind)), Reg(fp_spelling(is_d, rn, src_kind))])
    rhs: llvm_mc_word("fneg " ++ fp_spelling(is_d, rd, dest_kind) ++ ", " ++ fp_spelling(is_d, rn, src_kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  dest_kind: { gen: int, min: 0, max: 1, type: u32 }
  src_kind: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:12 GNU-style assembly; README.md:223 fneg; encoder/mod.rs:406-408 scalar dispatch; llvm-mc -triple=aarch64 -show-encoding
```

## encode_fneg_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM 1-source field layout, independent of llvm-mc. Stronger differential already owned by encode_fneg_diff_valid. Round-trip rejected (no decoder). Doc evidence: ARM ARM Floating-point data-processing (1 source) FNEG opcode=000010; purpose comment fp_scalar.rs:88 names the ARM layout (not the producing assignment).
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_arm_fields
- Formal: ∀ rd,rn ∈ [0,31], is_d ∈ Bool. encode_fneg([Reg(fp(is_d,rd)), Reg(fp(is_d,rn))]) = Word(w) ∧ w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b000010<<15)|(0b10000<<10)|(rn<<5)|rd where ftype=01 if is_d else 00
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fneg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_d]
  domain: { rd: 0..31, rn: 0..31, is_d: bool }
  relation:
    op: eq
    lhs: encode_fneg([Reg(fp(is_d, rd)), Reg(fp(is_d, rn))])
    rhs: (0b00011110u32 << 24) | (ftype << 22) | (1 << 21) | (0b000010 << 15) | (0b10000 << 10) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
evidence: ARM ARM FP 1-source M=0 S=0 11110 ftype 1 opcode=000010 10000 Rn Rd; fp_scalar.rs:88 purpose comment
```

## encode_fneg_metamorphic_fields
- Tier: 4
- Rationale: Algebraic metamorphic: independent Rd/Rn/ftype fields. Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D flips only ftype bit 22. ARM ARM 1-source layout. Weaker than differential; required metamorphic angle.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_metamorphic_fields
- Formal: ∀ rd,rn ∈ [0,30], is_d ∈ Bool. let w = encode_fneg(rd,rn,is_d). encode_fneg(rd+1,rn,is_d) = w+1 ∧ encode_fneg(rd,rn+1,is_d) = w+(1<<5) ∧ encode_fneg(rd,rn,¬is_d) XOR w = (1<<22)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fneg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, is_d]
  domain: { rd: 0..30, rn: 0..30, is_d: bool }
  body: w_rd == w + 1 && w_rn == w + (1 << 5) && (w_ft ^ w) == (1u32 << 22)
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  is_d: { gen: bool }
evidence: ARM ARM FP 1-source Rd bits[4:0] Rn bits[9:5] ftype bits[23:22]
```

## encode_fneg_neg_arity
- Tier: 4e
- Rationale: Negative/error contract. GNU assembler / llvm-mc reject FNEG with fewer than 2 operands. get_reg on missing slots must Err. Doc evidence: README.md:12 gas contract; llvm-mc "too few operands".
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_arity
- Formal: ∀ len ∈ {0,1}, n ∈ [0,31]. encode_fneg(ops) = Err where |ops|=len
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n]
  domain: { len: 0..1, n: 0..31 }
  relation:
    op: throws
    expr: encode_fneg(ops_of_len(len, n))
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: README.md:12 gas contract; llvm-mc too few operands for fneg with 0 or 1 operand
```

## encode_fneg_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract. ARM FNEG / llvm-mc / gas take exactly two registers; a 3rd operand is invalid. Doc evidence: README.md:12; llvm-mc "invalid operand for instruction" on `fneg s0, s1, s2`.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_extra_operand
- Formal: ∀ rd,rn ∈ [0,31], is_d ∈ Bool, extra ∈ Operand. encode_fneg([Reg(fp(is_d,rd)), Reg(fp(is_d,rn)), extra]) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_d=false, extra=Reg("s0") → fneg s0, s0, s0
- Bug report: pbt-out/bug_reports/encode_fneg_extra_operand.md

```property
function: encoder.fp_scalar.encode_fneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, extra]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, extra: Operand }
  relation:
    op: throws
    expr: encode_fneg([Reg(fp(is_d, rd)), Reg(fp(is_d, rn)), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  extra: { gen: oneof, choices: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: README.md:12 gas contract; llvm-mc invalid operand for fneg s0, s1, s2
```

## encode_fneg_neg_wrong_types
- Tier: 4e
- Rationale: Negative/error contract. ARM FNEG requires matching Sd,Sn or Dd,Dn (or Hd,Hn). Mixed S/D, GPR, Q/V/B, SP/WSP are invalid. Doc evidence: README.md:12; llvm-mc "invalid operand" on `fneg s0, d0`.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_wrong_types
- Formal: ∀ (dest,src) ∈ wrong_type_pair. encode_fneg([Reg(dest), Reg(src)]) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: dest="s0", src="d0" → fneg s0, d0
- Bug report: pbt-out/bug_reports/encode_fneg_wrong_types.md

```property
function: encoder.fp_scalar.encode_fneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, src]
  domain: { dest: wrong_type_name, src: wrong_type_name }
  relation:
    op: throws
    expr: encode_fneg([Reg(dest), Reg(src)])
generators:
  dest: { gen: string }
  src: { gen: string }
expected_error: String
evidence: README.md:12 gas contract; ARM ARM FNEG Sd,Sn or Dd,Dn; llvm-mc invalid operand for fneg s0, d0
```

## encode_fneg_diff_half
- Tier: 2
- Rationale: Differential vs llvm-mc +fullfp16 for half-precision FNEG (ftype=11). ARM ARM ftype 11=H. Bound documented at ftype encoding. Same-job as S/D scalar FNEG (same mnemonic, same 1-source class).
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_diff_half
- Formal: ∀ rd,rn ∈ [0,31]. encode_fneg([Reg("h"++rd), Reg("h"++rn)]) = Word(w) ∧ llvm-mc -mattr=+fullfp16 ("fneg h{rd}, h{rn}") = w
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: rd=0, rn=0 → fneg h0, h0; SUT Word(0x1e214000) vs llvm-mc Word(0x1ee14000)
- Bug report: pbt-out/bug_reports/encode_fneg_half_ftype.md

```property
function: encoder.fp_scalar.encode_fneg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_fneg([Reg("h" ++ rd), Reg("h" ++ rn)])
    rhs: llvm_mc_fp16_word("fneg h{rd}, h{rn}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM ftype 11=H; llvm-mc -mattr=+fullfp16 fneg h0, h1 = 0x1ee14020; README.md:223 fneg
```

## encode_fneg_neg_nonreg
- Tier: 4e
- Rationale: Negative/error contract. Non-register operand kinds (Imm, Symbol, Label, Mem, Cond, Shift) at either slot must Err. Doc evidence: README.md:12; get_reg contract encoder/mod.rs:956-965 "expected register".
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_nonreg
- Formal: ∀ which ∈ {0,1}, kind ∈ nonreg_kinds. encode_fneg(ops with slot which = nonreg(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..1, kind: 0..5 }
  relation:
    op: throws
    expr: encode_fneg(ops_with_nonreg_at(which, kind))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: README.md:12 gas contract; encoder/mod.rs:956-965 get_reg expected register
```

## encode_fneg_neg_invalid_name
- Tier: 4e
- Rationale: Negative/error contract sweep. parse_reg_num rejects names outside x/w/d/s/q/v/h/b + 0..=31 (and sp/xzr/lr aliases). Invalid names at either slot must Err. Doc evidence: encoder/mod.rs:131-147 parse_reg_num; encoder/mod.rs:956-965 get_reg "invalid register".
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo,s32,d32,h32,x32,r0,s,d,""}. encode_fneg(ops with slot which = Reg(name)) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: {foo,s32,d32,h32,x32,r0,s,d,""} }
  relation:
    op: throws
    expr: encode_fneg(ops_with_invalid_name_at(which, name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num returns None for foo/s32/empty; encoder/mod.rs:956-965 get_reg invalid register
```
