# Properties: encode_negs

## encode_negs_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree NEGS/SUBS decoder. encode_neg rejected (same-job gate: SUB / S=0). encode_add_sub shares get_reg/sf_bit so is not an independent differential. SUT-boundary: internal-helper of the GNU-style AArch64 assembler; mapping operands <-> `negs Rd, Rm{, shift}`.
- Seed: (none)
- Formal: ∀ rd, rm ∈ {0..31}, w ∈ {W,X}, sh ∈ {LSL,LSR,ASR} ∪ {ε}, amt ∈ [0, max_imm6(w)]. encode_negs([Reg(rd_w), Reg(rm_w), Shift?]) = Word(v) ∧ llvm-mc(-triple=aarch64, "negs Rd, Rm{, sh #amt}") = v
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_negs
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, use_shift, amt]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, kind: lsl_lsr_asr, amt: 0..63 }
  relation:
    op: eq
    lhs: encode_negs(ops)
    rhs: llvm_mc_word("negs Rd, Rm{, kind #amt}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr"] }
  use_shift: { gen: bool }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:14 same textual assembly as gas; encoder/mod.rs:279 "negs" => encode_negs; ARM ARM NEGS alias of SUBS shifted-register
```

## encode_negs_diff_subs_alias
- Tier: 2
- Rationale: Documented alias NEGS Rd, Rm = SUBS Rd, ZR, Rm (data_processing.rs:728 purpose comment; llvm-mc canonicalizes subs Rd, ZR, Rm to negs). Independent differential vs llvm-mc SUBS, not in-tree encode_add_sub. Same stronger-oracle rejections as encode_negs_diff_llvm_mc. This is the required metamorphic/differential alias identity.
- Seed: (none)
- Formal: ∀ rd, rm ∈ {0..31}, w ∈ {W,X}, sh ∈ {LSL,LSR,ASR} ∪ {ε}, amt ∈ [0, max_imm6(w)]. encode_negs([Reg(rd_w), Reg(rm_w), Shift?]) = llvm-mc("negs …") = llvm-mc("subs Rd, ZR, Rm{, sh #amt}")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_negs
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, use_shift, amt]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, kind: lsl_lsr_asr, amt: in_range }
  relation:
    op: eq
    lhs: encode_negs(ops)
    rhs: llvm_mc_word("subs Rd, ZR, Rm{, kind #amt}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr"] }
  use_shift: { gen: bool }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: data_processing.rs:728 NEGS -> SUBS Rd, XZR, Rm; llvm-mc canonicalizes the pair; ARM ARM C6 NEGS alias
```

## encode_negs_metamorphic_sf_xor
- Tier: 4c
- Rationale: ARM ARM sf is bit 31 of Add/subtract (shifted register). Same register numbers and in-range shift on X vs W must differ only in sf. Stronger differential already used on the valid domain; this is an independent field metamorphic.
- Seed: (none)
- Formal: ∀ rd, rm ∈ {0..31}, sh ∈ {LSL,LSR,ASR}, amt ∈ [0,31]. encode_negs(X-ops) XOR encode_negs(W-ops) = 1<<31
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_negs
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rm, kind, amt]
  domain: { rd: 0..31, rm: 0..31, kind: lsl_lsr_asr, amt: 0..31 }
  relation:
    op: eq
    lhs: encode_negs(x_ops) XOR encode_negs(w_ops)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr"] }
  amt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Add/subtract (shifted register) sf at bit 31; DESIGN_DOC.md:338 fixed 32-bit encoding
```

## encode_negs_invariant_arm_fields
- Tier: 4d
- Rationale: ARM ARM field layout of NEGS (shifted register): sf op=1 S=1 01011 shift 0 Rm imm6 Rn=31 Rd. Weaker than differential (does not check agreement with an independent assembler) but pins each field.
- Seed: (none)
- Formal: ∀ valid NEGS ops. let w = encode_negs(ops). (w>>31)&1=sf ∧ (w>>30)&1=1 ∧ (w>>29)&1=1 ∧ (w>>24)&0x1f=0b01011 ∧ (w>>22)&3=st ∧ (w>>21)&1=0 ∧ (w>>16)&0x1f=rm ∧ (w>>10)&0x3f=amt ∧ (w>>5)&0x1f=31 ∧ w&0x1f=rd
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_negs
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, amt]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, kind: lsl_lsr_asr, amt: in_range }
  relation:
    op: holds
    expr: fields(encode_negs(ops)) match ARM ARM NEGS layout with Rn=31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Add/subtract (shifted register) NEGS; data_processing.rs:728 Rn=XZR
```

## encode_negs_neg_too_few
- Tier: 4e
- Rationale: llvm-mc rejects `negs x0` ("too few operands"); ARM ARM NEGS requires Rd and Rm. get_reg(1) is the SUT error path. Negative/error contract from the public assembler (gas/llvm-mc) contract in README.md:14.
- Seed: (none)
- Formal: ∀ ops with |ops| < 2. encode_negs(ops) = Err(_)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_negs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, is_64, r0]
  domain: { n: 0..1, is_64: bool, r0: 0..31 }
  relation:
    op: throws
    expr: encode_negs(ops[..n])
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  is_64: { gen: bool }
  r0: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc "too few operands for instruction"; ARM ARM NEGS two-register form; README.md:14 gas-compatible
```

## encode_negs_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects a third operand that is not lsl/lsr/asr (including extra GPR, extend, trailing after shift). GNU-style NEGS is Rd, Rm{, shift} only.
- Seed: (none)
- Formal: ∀ rd, rm ∈ GPR, extra ∉ Shift(lsl|lsr|asr, in-range). encode_negs([Rd, Rm, extra, …]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rm=0, is_64=false, extra=Reg("x0")
- Bug report: pbt-out/bug_reports/encode_negs_extra_operand.md

```property
function: encode_negs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, extra]
  domain: { rd: 0..30, rm: 0..30, extra: non_shift_or_trailing }
  relation:
    op: throws
    expr: encode_negs([Rd, Rm, extra])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  extra: { gen: oneof, items: ["Reg", "Imm", "Mem", "Symbol", "Cond", "Label"] }
evidence: llvm-mc "expected 'lsl', 'lsr' or 'asr'"; ARM ARM NEGS optional shift only; README.md:14
```

## encode_negs_neg_mixed_width
- Tier: 4e
- Rationale: llvm-mc rejects `negs x0, w1` and `negs w0, x1`. ARM ARM requires Rd and Rm the same width.
- Seed: (none)
- Formal: ∀ rd, rm ∈ {0..30}, rd64 ≠ rm64. encode_negs([Reg(rd_w), Reg(rm_w')]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rm=0, rd64=false, rm64=true
- Bug report: pbt-out/bug_reports/encode_negs_mixed_width.md

```property
function: encode_negs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rm, rd64, rm64]
  domain: { rd: 0..30, rm: 0..30, rd64: bool, rm64: bool }
  relation:
    op: throws
    expr: encode_negs([Reg(rd), Reg(rm)])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rm64: { gen: bool }
evidence: llvm-mc "invalid operand for instruction" on mixed X/W; ARM ARM Rd/Rm same width
```

## encode_negs_neg_sp_fp_shift
- Tier: 4e
- Rationale: Combined documented invalid domain: (1) register 31 is XZR/WZR never SP/WSP; (2) FP/SIMD prefixes are not NEGS operands; (3) imm6 range 0..31 (sf=0) / 0..63 (sf=1), and ROR/unknown shift kinds are not in {LSL,LSR,ASR}. Bounds sampled at 32, 31, 63, 64.
- Seed: (none)
- Formal: ∀ ops in {SP in either slot} ∪ {FP prefix in either slot} ∪ {shift amt out of range} ∪ {shift kind ∉ {lsl,lsr,asr}}. encode_negs(ops) = Err(_)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: SP=[Reg("wsp"), Reg("w0")]; FP=[Reg("d0"), Reg("x1")]; range=[Reg("w0"), Reg("w0"), Shift{lsl,32}]; kind=[Reg("w0"), Reg("w0"), Shift{ror,0}]
- Bug report: pbt-out/bug_reports/encode_negs_sp.md; pbt-out/bug_reports/encode_negs_fp_reg.md; pbt-out/bug_reports/encode_negs_shift_range.md; pbt-out/bug_reports/encode_negs_bad_shift_kind.md

```property
function: encode_negs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, prefix, amt_w, amt_x, kind]
  domain: { which: 0..1, prefix: fp_prefix, amt_w: out_of_range_w, amt_x: out_of_range_x, kind: invalid_shift }
  relation:
    op: throws
    expr: encode_negs(ops)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  prefix: { gen: oneof, items: ["d", "s", "q", "v", "h", "b"] }
  amt_w: { gen: oneof, items: [32, 33, 63, 64] }
  amt_x: { gen: oneof, items: [64, 65, 128] }
  kind: { gen: oneof, items: ["ror", "foo", "lslv", "rrx", "empty", "uxtw"] }
evidence: llvm-mc rejects SP/FP/ROR/out-of-range; ARM ARM register 31 is ZR, shift in LSL/LSR/ASR, imm6 range by sf
```

## encode_negs_diff_lr
- Tier: 2
- Rationale: `lr` is a documented 64-bit alias of X30 (parse_reg_num and llvm-mc). Folded into the valid-domain differential.
- Seed: (none)
- Formal: ∀ which ∈ {Rd,Rm}, other ∈ {0..30}, sh ∈ {LSL,LSR,ASR}, amt ∈ [0,63]. encode_negs(ops with lr in that slot) = llvm-mc("negs … lr …")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_negs
oracle: differential
predicate:
  quantifier: forall
  vars: [which, other, kind, amt]
  domain: { which: 0..1, other: 0..30, kind: lsl_lsr_asr, amt: 0..63 }
  relation:
    op: eq
    lhs: encode_negs(ops_with_lr)
    rhs: llvm_mc_word("negs ... lr ...")
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  other: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: encoder/mod.rs:136 "lr" => 30; llvm-mc encodes lr as x30
```

## encode_negs_neg_invalid_reg
- Tier: 4e
- Rationale: Contract-surface sweep (coverage_gaps had no profraw; manual arm audit of get_reg). parse_reg_num returns None for foo/x32/w32/x/r0/empty; get_reg Errs on non-Reg kinds. llvm-mc rejects these. Documented error path of get_reg (encoder/mod.rs:956-965) not reached by too_few (None vs Some invalid).
- Seed: (none)
- Formal: ∀ which ∈ {0,1}, name ∉ valid GPR names ∪ extra ∉ Reg. encode_negs(ops with that slot replaced) = Err(_)
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_negs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, name, extra]
  domain: { which: 0..1, name: invalid_gpr, extra: non_reg }
  relation:
    op: throws
    expr: encode_negs(ops_with_slot_replaced)
expected_error: String
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  name: { gen: oneof, items: ["foo", "x32", "w32", "x", "r0", "empty"] }
evidence: encoder/mod.rs:956-965 get_reg expected register; parse_reg_num None for invalid names; llvm-mc rejects them
```
