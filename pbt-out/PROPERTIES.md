# Properties: encode_bics

## encode_bics_diff_reg_llvm_mc
- Tier: 7
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler). State machine rejected (pure function, no lifecycle). In-tree BICS decoder does not exist so algebraic round-trip is unavailable. encode_bic is a different job (no flag-setting) and fails the same-job sibling gate as a differential reference. Doc evidence: assembler README "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:237 bics dispatch; ARM ARM Logical (shifted register) BICS encoding.
- Seed: src/backend/arm/assembler/encoder/data_processing.rs encode_bic_pbt::encode_bic_diff_reg_llvm_mc (sibling register-form generalization)
- Formal: ∀ rd,rn,rm ∈ 0..=31, ∀ is_64 ∈ Bool, ∀ kind ∈ {lsl,lsr,asr,ror}, ∀ amt ∈ [0, 31] if ¬is_64 else [0, 63], ∀ use_shift ∈ Bool. encode_bics([Rd,Rn,Rm] {+ Shift(kind,amt) if use_shift}) = Word(llvm-mc("bics Rd, Rn, Rm{, kind #amt}")).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bics
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt, use_shift]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_64: Bool, kind: {lsl,lsr,asr,ror}, amt: in_range_for_sf, use_shift: Bool }
  relation:
    op: eq
    lhs: encode_bics([Reg(rd), Reg(rn), Reg(rm)] + optional Shift) as Word
    rhs: llvm_mc("bics Rd, Rn, Rm{, kind #amt}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, options: [lsl, lsr, asr, ror] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
  use_shift: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:237 bics dispatch; data_processing.rs:1009 ARM ARM format sf 11 01010 shift 1 Rm imm6 Rn Rd
```

## encode_bics_diff_imm_llvm_mc
- Tier: 7
- Rationale: Differential against llvm-mc for the GNU alias `bics Rd, Rn, #imm` → `ands Rd, Rn, #~imm` when ~imm is a valid AArch64 bitmask. State machine / round-trip rejected as above. Immediate form is part of the gas-compat public contract (llvm-mc accepts it); encode_bics is the sole dispatcher for the bics mnemonic.
- Seed: encode_bic_pbt::encode_bic_diff_imm_llvm_mc
- Formal: ∀ rd,rn ∈ 0..=31, ∀ is_64 ∈ Bool, ∀ imm such that ~imm is a valid AArch64 bitmask. encode_bics([Rd,Rn,Imm(imm)]) = Word(llvm-mc("bics Rd, Rn, #imm")).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, is_64=false, imm=0xaaaaaaaa (bics w0, w0, #0xaaaaaaaa)
- Bug report: pbt-out/bug_reports/encode_bics_imm_alias.md

```property
function: encoder.data_processing.encode_bics
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, imm]
  domain: { rd: u32_0_31, rn: u32_0_31, is_64: Bool, imm: inverted_valid_bitmask }
  relation:
    op: eq
    lhs: encode_bics([Reg(rd), Reg(rn), Imm(imm)]) as Word
    rhs: llvm_mc("bics Rd, Rn, #imm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  seed: { gen: int, min: 0, max: 9999, type: u32 }
evidence: llvm-mc -triple=aarch64 accepts bics Rd, Rn, #1 as ands Rd, Rn, #~1; README.md:5-14 gas-compatible; encoder/mod.rs:237 bics dispatch
```

## encode_bics_meta_opc_vs_bic
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM Logical (shifted register) BICS is BIC with opc=11 instead of 00 (N=1 in both). encode_bic is not a same-job differential reference; the relation is the documented opc-field XOR. Stronger differential already covers the happy path; this isolates the opc contract independently of llvm-mc.
- Seed: encode_adc_pbt S-bit XOR; encode_bic register form
- Formal: ∀ same-width GPR 3-reg operands (optional in-range shift). encode_bics(ops) XOR encode_bic(ops) = 0b11 << 29, and both succeed.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bics
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt, use_shift]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_64: Bool, shift: optional_in_range }
  relation:
    op: eq
    lhs: encode_bics(ops) XOR encode_bic(ops)
    rhs: 0b11 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, options: [lsl, lsr, asr, ror] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
  use_shift: { gen: bool }
evidence: data_processing.rs:1009 BICS opc=11 N=1; data_processing.rs:1088 BIC opc=00 N=1; ARM ARM Logical shifted-register opc field bits[30:29]
```

## encode_bics_word_layout
- Tier: 4
- Rationale: Algebraic invariant from the documented bit layout: sf at 31, opc=11 at [30:29], bits[28:24]=01010, N=1 at 21, Rd/Rn/Rm/shift/imm6 placed as specified. Stronger differential already covers numeric equality; this pins each field so a swapped Rn/Rm would fail even if llvm-mc were unavailable.
- Seed: (none)
- Formal: ∀ valid 3-reg BICS encodings W. bit31=sf, bits[30:29]=0b11, bits[28:24]=0b01010, bit21=1, W[4:0]=rd, W[9:5]=rn, W[20:16]=rm, W[23:22]=shift_type, W[15:10]=amt&0x3F.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bics
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { rd: u32_0_31, rn: u32_0_31, rm: u32_0_31, is_64: Bool, kind: {lsl,lsr,asr,ror}, amt: in_range_for_sf }
  relation:
    op: holds
    expr: word_fields_match_arm_arm(encode_bics(ops))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, options: [lsl, lsr, asr, ror] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: data_processing.rs:1009 Format sf 11 01010 shift 1 Rm imm6 Rn Rd; ARM ARM Logical (shifted register)
```

## encode_bics_neg_arity
- Tier: 3
- Rationale: Negative/error contract: function comment and early return require 3 operands. Stronger oracles do not apply to the underspecified-arity path.
- Seed: encode_bic_pbt::encode_bic_neg_arity
- Formal: ∀ n ∈ {0,1,2}, ∀ valid-looking register names. encode_bics(ops) with |ops|=n is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bics
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, is_64, r]
  domain: { n: 0..=2, is_64: Bool, r: 0..=30 }
  relation:
    op: throws
    expr: encode_bics(ops_of_len_n)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  is_64: { gen: bool }
  r: { gen: int, min: 0, max: 30, type: u32 }
evidence: data_processing.rs:982-984 "bics requires 3 operands"
```

## encode_bics_neg_mixed_width
- Tier: 3
- Rationale: Negative/error contract from ARM ARM / llvm-mc: all three GPRs must share sf. Mixed x/w is rejected by llvm-mc ("invalid operand"). Stronger differential does not apply to invalid encodings.
- Seed: encode_bic_pbt::encode_bic_neg_mixed_width
- Formal: ∀ rd,rn,rm ∈ 0..=30, ∀ (rd64,rn64,rm64) not all equal. encode_bics([Rd,Rn,Rm]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=true (bics w0, w0, x0)
- Bug report: pbt-out/bug_reports/encode_bics_mixed_width.md

```property
function: encoder.data_processing.encode_bics
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: 0..=30, rn: 0..=30, rm: 0..=30, widths: not_all_equal }
  relation:
    op: throws
    expr: encode_bics([Reg(rd), Reg(rn), Reg(rm)])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
evidence: llvm-mc rejects `bics x0, w1, x2`; ARM ARM BICS requires same-width GPRs; README.md:5-14 gas-compat
```

## encode_bics_neg_sp_fp
- Tier: 3
- Rationale: Negative/error contract: ARM ARM BICS register 31 is XZR/WZR not SP/WSP; FP/SIMD names (d/s/q/v/h/b) are not GPRs. llvm-mc rejects `bics sp, ...` and `bics d0, ...`.
- Seed: encode_bic_pbt::encode_bic_neg_sp_fp_regform
- Formal: ∀ which ∈ {0,1,2}, ∀ bad ∈ {sp,wsp,dN,sN,qN,vN,hN,bN}. encode_bics with bad at operand `which` (other slots valid GPRs) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: which=0, is_64=false, kind=0 (bics wsp, w0, w0); also d0 at any slot
- Bug report: pbt-out/bug_reports/encode_bics_sp_register_form.md; pbt-out/bug_reports/encode_bics_fp_reg.md

```property
function: encoder.data_processing.encode_bics
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b, bad]
  domain: { which: 0..=2, bad: {sp,wsp,dN,sN,qN,vN,hN,bN} }
  relation:
    op: throws
    expr: encode_bics(ops_with_bad_at_which)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 8, type: u32 }
  fp_n: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc rejects `bics sp, x1, x2` and `bics d0, x1, x2`; ARM ARM BICS Rd/Rn/Rm are GPRs, R31=XZR/WZR
```

## encode_bics_neg_shift_range_unknown
- Tier: 3
- Rationale: Negative/error contract: ARM ARM imm6 range is [0,31] (32-bit) / [0,63] (64-bit); only lsl/lsr/asr/ror are valid. llvm-mc rejects bound+1 and unknown kinds. Bounds 31/32 (W) and 63/64 (X) are sampled exactly.
- Seed: encode_bic_pbt::encode_bic_neg_shift_range_neon_arr / encode_bic_neg_unknown_shift_kind
- Formal: ∀ valid 3-reg BICS, ∀ kind ∈ {lsl,lsr,asr,ror}, ∀ amt ∈ {32,33,63,64} if ¬is_64 else {64,65,128}. encode_bics(ops+Shift(kind,amt)) is Err. Also ∀ unknown kind ∈ {lslx,rrx,rol,"","asr "}. encode_bics(... Shift(kind,amt)) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: lsl #32 on 32-bit (rd=rn=rm=0); unknown kind "lslx" amount 0
- Bug report: pbt-out/bug_reports/encode_bics_shift_out_of_range.md; pbt-out/bug_reports/encode_bics_unknown_shift_kind.md

```property
function: encoder.data_processing.encode_bics
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { amt: bound_plus_one_or_unknown_kind }
  relation:
    op: throws
    expr: encode_bics([Rd,Rn,Rm,Shift(kind,amt)])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, options: [lsl, lsr, asr, ror] }
  amt_w: { gen: oneof, options: [32, 33, 63, 64] }
  amt_x: { gen: oneof, options: [64, 65, 128] }
  unknown: { gen: oneof, options: [lslx, rrx, rol, "", "asr "] }
evidence: llvm-mc rejects `bics w0, w1, w2, lsl #32` and `bics x0, x1, x2, lsl #64`; ARM ARM imm6 range; README.md:5-14 gas-compat
```

## encode_bics_neg_invalid_rm
- Tier: 3
- Rationale: Negative/error contract covering the get_reg error path (parse_reg_num None). Sweep round: coverage_gaps had no profraw; manual arm audit of encode_bics error paths. x32/w32/empty/foo/r0/x are not GPRs; llvm-mc rejects them.
- Seed: encode_bic_pbt::encode_bic_neg_invalid_rm
- Formal: ∀ rd,rn ∈ 0..=30, ∀ is_64 ∈ Bool, ∀ bad ∈ {x32,w32,x99,"",foo,r0,x}. encode_bics([Rd,Rn,Reg(bad)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_bics
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, bad]
  domain: { bad: {x32,w32,x99,"",foo,r0,x} }
  relation:
    op: throws
    expr: encode_bics([Reg(rd), Reg(rn), Reg(bad)])
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  bad: { gen: oneof, options: [x32, w32, x99, "", foo, r0, x] }
evidence: parse_reg_num returns None for num>31 and unknown prefixes; llvm-mc rejects invalid register names; README.md:5-14 gas-compat
```

## encode_bics_neg_extra_operand
- Tier: 3
- Rationale: Negative/error contract: a 4th operand that is not a valid shift is not a BICS encoding. llvm-mc rejects `bics x0, x1, x2, x3`. Sweep round: encode_bics only inspects operand 3 when it is Shift and otherwise ignores extras.
- Seed: (none)
- Formal: ∀ rd,rn,rm ∈ 0..=30, ∀ extra ∈ {Reg, Imm, Mem, Symbol}. encode_bics([Rd,Rn,Rm,extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: is_64=false, rd=rn=rm=0, which=0 (bics w0, w0, w0, w0)
- Bug report: pbt-out/bug_reports/encode_bics_extra_operand.md

```property
function: encoder.data_processing.encode_bics
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, rm, extra]
  domain: { extra: non_shift_operand }
  relation:
    op: throws
    expr: encode_bics([Rd, Rn, Rm, extra])
expected_error: String
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc rejects `bics x0, x1, x2, x3`; README.md:5-14 gas-compat; ARM ARM BICS 4th operand is optional shift only
```
