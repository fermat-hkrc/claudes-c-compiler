# Properties: encode_fsqrt

## encode_fsqrt_diff_valid
- Tier: 2
- Rationale: Differential vs llvm-mc is the strongest evidenced oracle. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree FSQRT decoder). Sibling encode_fabs/encode_fneg rejected (same-job gate: ARM opcodes 000001 / 000010, not FSQRT 000011). Sibling encode_fp_1src rejected (FRINT*). Sibling encode_neon_float_two_misc rejected (vector form). Doc evidence: README.md:12 GNU-style assembly contract; README.md:223 lists scalar fsqrt; encoder/mod.rs:1-7 32-bit AArch64 words; encoder/mod.rs:411-413 scalar dispatch.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_diff_valid
- Formal: ∀ rd,rn ∈ [0,31], is_d ∈ Bool, dest_kind,src_kind ∈ {0,1}. encode_fsqrt([Reg(spell(is_d,rd,dest_kind)), Reg(spell(is_d,rn,src_kind))]) = Word(w) ∧ llvm-mc("fsqrt " ++ spell ++ ", " ++ spell) = w
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, dest_kind, src_kind]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, dest_kind: 0..1, src_kind: 0..1 }
  relation:
    op: eq
    lhs: encode_fsqrt([Reg(fp_spelling(is_d, rd, dest_kind)), Reg(fp_spelling(is_d, rn, src_kind))])
    rhs: llvm_mc_word("fsqrt " ++ fp_spelling(is_d, rd, dest_kind) ++ ", " ++ fp_spelling(is_d, rn, src_kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  dest_kind: { gen: int, min: 0, max: 1, type: u32 }
  src_kind: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:12 GNU-style assembly; README.md:223 fsqrt; encoder/mod.rs:411-413 scalar dispatch; llvm-mc -triple=aarch64 -show-encoding
```

## encode_fsqrt_arm_fields
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM 1-source field layout, independent of llvm-mc. Stronger differential already owned by encode_fsqrt_diff_valid. Round-trip rejected (no decoder). Doc evidence: ARM ARM Floating-point data-processing (1 source) FSQRT opcode=000011; purpose comment fp_scalar.rs:111 names the ARM layout (not the producing assignment).
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_arm_fields
- Formal: ∀ rd,rn ∈ [0,31], is_d ∈ Bool. encode_fsqrt([Reg(fp(is_d,rd)), Reg(fp(is_d,rn))]) = Word(w) ∧ w = (0b00011110<<24)|(ftype<<22)|(1<<21)|(0b000011<<15)|(0b10000<<10)|(rn<<5)|rd where ftype=01 if is_d else 00
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, is_d]
  domain: { rd: 0..31, rn: 0..31, is_d: bool }
  relation:
    op: eq
    lhs: encode_fsqrt([Reg(fp(is_d, rd)), Reg(fp(is_d, rn))])
    rhs: (0b00011110u32 << 24) | (ftype << 22) | (1 << 21) | (0b000011 << 15) | (0b10000 << 10) | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
evidence: ARM ARM FP 1-source M=0 S=0 11110 ftype 1 opcode=000011 10000 Rn Rd; fp_scalar.rs:111 purpose comment
```

## encode_fsqrt_metamorphic_fields
- Tier: 4
- Rationale: Algebraic metamorphic: independent Rd/Rn/ftype fields. Rd+1 increments bits[4:0] only; Rn+1 increments bits[9:5] only; S vs D flips only ftype bit 22. ARM ARM 1-source layout. Weaker than differential; required metamorphic angle.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_metamorphic_fields
- Formal: ∀ rd,rn ∈ [0,30], is_d ∈ Bool. let w = encode_fsqrt(rd,rn,is_d). encode_fsqrt(rd+1,rn,is_d) = w+1 ∧ encode_fsqrt(rd,rn+1,is_d) = w+(1<<5) ∧ encode_fsqrt(rd,rn,¬is_d) XOR w = (1<<22)
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fsqrt
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
evidence: ARM ARM FP 1-source independent Rd/Rn/ftype fields; fp_scalar.rs:111 purpose comment
```

## encode_fsqrt_neg_arity
- Tier: 5
- Rationale: Negative/error contract. llvm-mc rejects fsqrt with fewer than 2 operands ("too few operands"). get_reg on missing slots returns Err. Stronger oracles do not apply to the invalid-arity domain.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_arity
- Formal: ∀ len ∈ {0,1}, n ∈ [0,31]. encode_fsqrt(ops) = Err when |ops|=len < 2
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [len, n]
  domain: { len: 0..1, n: 0..31 }
  relation:
    op: throws
    expr: encode_fsqrt(ops_of_len(len, n))
generators:
  len: { gen: int, min: 0, max: 1, type: usize }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; get_reg missing-slot Err; README.md:12 gas-compatible
```

## encode_fsqrt_neg_extra_operand
- Tier: 5
- Rationale: Negative/error contract. llvm-mc rejects a 3rd operand ("invalid operand for instruction"). ARM ARM FSQRT is a 1-source 2-register encoding. SUT currently ignores extra operands via get_reg (no len check) — that is a contract violation, not a caveat. The producing body is not Doc evidence.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_extra_operand
- Formal: ∀ rd,rn ∈ [0,31], is_d ∈ Bool, extra ∈ Operand. encode_fsqrt([Reg(fp(is_d,rd)), Reg(fp(is_d,rn)), extra]) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: [Reg("s0"), Reg("s0"), Reg("s0")]
- Bug report: pbt-out/bug_reports/encode_fsqrt_extra_operand.md

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, is_d, extra]
  domain: { rd: 0..31, rn: 0..31, is_d: bool, extra: Operand }
  relation:
    op: throws
    expr: encode_fsqrt([Reg(fp(is_d, rd)), Reg(fp(is_d, rn)), extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_d: { gen: bool }
  extra: { gen: oneof, of: [Reg, Imm, Shift, RegArrangement] }
expected_error: String
evidence: llvm-mc extra-operand error; ARM ARM 1-source 2-register FSQRT; README.md:12 gas-compatible
```

## encode_fsqrt_neg_wrong_types
- Tier: 5
- Rationale: Negative/error contract. llvm-mc rejects mixed S/D, GPR, Q/V/B, and SP/WSP as FSQRT operands. ARM ARM requires matching Sd,Sn or Dd,Dn (or Hd,Hn). parse_reg_num accepting those names is not a license to encode them as FSQRT.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_wrong_types
- Formal: ∀ (dest,src) ∈ wrong_type_pair. encode_fsqrt([Reg(dest), Reg(src)]) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: [Reg("s0"), Reg("d0")]
- Bug report: pbt-out/bug_reports/encode_fsqrt_wrong_types.md

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, src]
  domain: { dest: wrong_type_name, src: wrong_type_name }
  relation:
    op: throws
    expr: encode_fsqrt([Reg(dest), Reg(src)])
generators:
  dest: { gen: string }
  src: { gen: string }
expected_error: String
evidence: llvm-mc invalid operand; ARM ARM matching Sd/Sn Dd/Dn Hd/Hn; README.md:12 gas-compatible
```

## encode_fsqrt_diff_half
- Tier: 2
- Rationale: Differential vs llvm-mc +fullfp16 for half-precision FSQRT. ARM ARM ftype=11 for H. SUT currently treats H as S (starts_with('d') only). Valid public-API input (scalar Hd,Hn is dispatched to encode_fsqrt, not the vector form). Stronger state machine / round-trip rejected as above.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_diff_half
- Formal: ∀ rd,rn ∈ [0,31]. encode_fsqrt([Reg("h"++rd), Reg("h"++rn)]) = Word(w) ∧ llvm-mc -mattr=+fullfp16 ("fsqrt h{rd}, h{rn}") = w
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: failing
- Counterexample: [Reg("h0"), Reg("h0")] (fsqrt h0, h0 → SUT 0x1e21c000 vs llvm-mc 0x1ee1c000)
- Bug report: pbt-out/bug_reports/encode_fsqrt_half_ftype.md

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_fsqrt([Reg("h" ++ rd), Reg("h" ++ rn)])
    rhs: llvm_mc_fp16_word("fsqrt h" ++ rd ++ ", h" ++ rn)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 GNU-style; ARM ARM ftype=11 H; llvm-mc -mattr=+fullfp16; encoder/mod.rs:411-413 scalar (non-RegArrangement) dispatch
```

## encode_fsqrt_neg_nonreg
- Tier: 5
- Rationale: Negative/error contract. Non-register operand kinds (Imm, Symbol, Label, Mem, Cond, Shift) must Err. get_reg documents "expected register". llvm-mc rejects non-register FSQRT operands.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_nonreg
- Formal: ∀ which ∈ {0,1}, kind ∈ {Imm, Symbol, Label, Mem, Cond, Shift}. encode_fsqrt(ops with slot which = kind) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, kind]
  domain: { which: 0..1, kind: nonreg_operand }
  relation:
    op: throws
    expr: encode_fsqrt(ops_with_nonreg(which, kind))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  kind: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: get_reg "expected register"; llvm-mc invalid operand; README.md:12 gas-compatible
```

## encode_fsqrt_neg_invalid_name
- Tier: 5
- Rationale: Sweep — documented parse_reg_num error path (None for names outside x/w/d/s/q/v/h/b 0-31, plus sp/wsp/xzr/wzr/lr). llvm-mc rejects foo/s32/d32/h32/x32/r0/s/d/empty. Stronger oracles do not apply to the invalid-name domain. Added after coverage_gaps had no LLVM profraw; manual arm audit of encode_fsqrt.
- Seed: fp_scalar.rs encode_fabs_pbt encode_fabs_neg_invalid_name
- Formal: ∀ which ∈ {0,1}, name ∈ {foo, s32, d32, h32, x32, r0, s, d, empty}. encode_fsqrt(ops with slot which = Reg(name)) = Err
- Test file: src/backend/arm/assembler/encoder/fp_scalar.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.fp_scalar.encode_fsqrt
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: 0..1, name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_fsqrt(ops_with_invalid_name(which, name))
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: string }
expected_error: String
evidence: parse_reg_num None for out-of-range / unknown prefix; llvm-mc invalid operand; encoder/mod.rs:131-148
```
