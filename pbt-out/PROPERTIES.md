# Properties: encode_hint

## encode_hint_diff_imm
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the valid HINT immediate domain. README.md:12 claims GNU-gas-compatible assembly; encoder/mod.rs:3 claims 32-bit AArch64 words; encode() at encoder/mod.rs:967 routes `"hint"` with operands passed through. ARM ARM HINT encoding 1101 0101 0000 0011 0010 CRm op2 11111 is independently specified; imm ∈ 0..=127. Stronger rejected: State machine (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree HINT decoder). Sibling NOP/YIELD/WFE/WFI/SEV/SEVL/BTI rejected by same-job gate (different mnemonics, no free imm). Weaker available: algebraic.metamorphic, algebraic.invariant, negative_error.
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_brk_pbt.rs:223 (llvm-mc differential over imm)
- Formal: ∀ imm ∈ 0..=127. encode_hint([Imm(imm)]) = Word(llvm-mc("hint #imm")) ∧ llvm-mc("hint #imm") = 0xD503201F | (((imm >> 3) & 0xF) << 8) | ((imm & 7) << 5)
- Test file: src/backend/arm/assembler/encoder/encode_hint_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hint
oracle: differential
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=127 }
  relation:
    op: eq
    lhs: encode_hint([Imm(imm)])
    rhs: Word(llvm_mc("hint #imm"))
generators:
  imm: { gen: int, min: 0, max: 127, type: i64 }
evidence: src/backend/arm/assembler/README.md:12; encoder/mod.rs:3; encoder/mod.rs:967; ARM ARM HINT 0xD503201F|(CRm<<8)|(op2<<5); llvm-mc -triple=aarch64 -show-encoding
```

## encode_hint_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM HINT field layout (not the SUT mask). bits[31:12]=0b11010101000000110010, bits[11:5]=imm[6:0], bits[4:0]=11111, word = 0xD503201F | (imm << 5). Stronger rejected: State machine (no lifecycle); Differential is the primary oracle (this is a field-level check that does not need llvm-mc); Round-trip (no decoder).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:556 "HINT: 11010101 00000011 0010 CRm op2 11111" — other fingerprint d662ba6b
- Seed: src/backend/arm/assembler/encoder/encode_brk_pbt.rs:232 (ARM layout invariant)
- Formal: ∀ imm ∈ 0..=127. let w = encode_hint([Imm(imm)]). Word(w) ⇒ (w >> 12 = 0xD5032) ∧ ((w >> 5) & 0x7F = imm) ∧ (w & 0x1F = 0b11111) ∧ (w = 0xD503201F | (imm << 5))
- Test file: src/backend/arm/assembler/encoder/encode_hint_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hint
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=127 }
  relation:
    op: eq
    lhs: encode_hint([Imm(imm)])
    rhs: Word(0xD503201F | ((imm as u32) << 5))
generators:
  imm: { gen: int, min: 0, max: 127, type: i64 }
evidence: ARM ARM HINT encoding 1101 0101 0000 0011 0010 CRm op2 11111; system.rs:556
```

## encode_hint_meta_imm_isolation
- Tier: 4
- Rationale: Metamorphic: two valid imms may differ only in bits[11:5]; encode(imm) XOR encode(0) = imm << 5. Independent of llvm-mc. Stronger rejected: State machine; Round-trip (no decoder); Differential already covers value agreement.
- Doc contract: src/backend/arm/assembler/encoder/system.rs:557 "CRm = imm >> 3, op2 = imm & 7" — other fingerprint 7812959d
- Seed: src/backend/arm/assembler/encoder/encode_brk_pbt.rs:242 (imm isolation)
- Formal: ∀ imm1, imm2 ∈ 0..=127. let w1,w2,w0 = encode_hint of each. ((w1 ⊕ w2) & ¬0xFE0 = 0) ∧ (imm1 ≠ imm2 ⇒ w1 ≠ w2) ∧ (w1 ⊕ w0 = imm1 << 5)
- Test file: src/backend/arm/assembler/encoder/encode_hint_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hint
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [imm1, imm2]
  domain: { imm1: 0..=127, imm2: 0..=127 }
  relation:
    op: eq
    lhs: (encode_hint([Imm(imm1)]) XOR encode_hint([Imm(0)]))
    rhs: (imm1 as u32) << 5
generators:
  imm1: { gen: int, min: 0, max: 127, type: i64 }
  imm2: { gen: int, min: 0, max: 127, type: i64 }
evidence: ARM ARM HINT imm occupies bits[11:5] only; system.rs:557
```

## encode_hint_neg_extra
- Tier: 3
- Rationale: Negative/error contract: gas and llvm-mc reject extra operands ("unexpected characters following instruction" / "invalid operand"). README.md:12 claims gas-compatible assembly. encode_hint's own comments do not declare extra operands invalid, so they stay in the generator. Stronger rejected: State machine; Round-trip; Differential on extra operands has no encoding (reference rejects).
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_brk_pbt.rs:260 (extra operand)
- Formal: ∀ imm ∈ 0..=127. ∀ extra ∈ Operand. llvm-mc("hint #imm, extra") is Err ⇒ encode_hint([Imm(imm), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_hint_pbt.rs
- Status: failing
- Counterexample: imm = 0, extra = Reg("x0"); SUT Ok(Word(0xd503201f)) vs llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_hint_extra_operand.md

```property
function: encoder.system.encode_hint
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm, extra]
  domain: { imm: 0..=127, extra: Operand }
  relation:
    op: holds
    expr: encode_hint([Imm(imm), extra]).is_err()
expected_error: String
generators:
  imm: { gen: int, min: 0, max: 127, type: i64 }
  extra: { gen: oneof, variants: [Reg, Imm, Barrier, Cond, Label, Symbol] }
evidence: src/backend/arm/assembler/README.md:12; llvm-mc "invalid operand for instruction"; gas "unexpected characters following instruction"
```

## encode_hint_neg_oob_imm
- Tier: 3
- Rationale: Negative/error contract: llvm-mc and gas reject HINT immediates outside 0..=127. ARM ARM HINT packs a 7-bit imm into CRm:op2. encode_hint's comments describe the split but do not declare out-of-range Imm invalid, so oob stays in the generator. Stronger rejected: State machine; Round-trip; Differential has no encoding (reference rejects).
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_brk_pbt.rs:276 (oob imm)
- Formal: ∀ imm ∉ 0..=127. llvm-mc("hint #imm") is Err ⇒ encode_hint([Imm(imm)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_hint_pbt.rs
- Status: failing
- Counterexample: imm = -1; SUT Ok(Word(0xd5032fff)) vs llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_hint_oob_imm.md

```property
function: encoder.system.encode_hint
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: i64 \ 0..=127 }
  relation:
    op: holds
    expr: encode_hint([Imm(imm)]).is_err()
expected_error: String
generators:
  imm: { gen: int, min: -4096, max: 70000, type: i64 }
evidence: llvm-mc "immediate must be an integer in range [0, 127]"; gas "immediate value out of range 0 to 127"; ARM ARM HINT 7-bit imm
```

## encode_hint_neg_empty
- Tier: 3
- Rationale: Negative/error contract: llvm-mc ("too few operands") and gas ("missing immediate expression") reject omitted HINT immediate. get_imm returns Err on missing operand 0, matching the reference. Stronger rejected: State machine; Round-trip; Differential has no encoding.
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_brk_pbt.rs:292 (empty operands)
- Formal: ∀ _ . llvm-mc("hint") is Err ∧ encode_hint([]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_hint_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hint
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..8 }
  relation:
    op: holds
    expr: encode_hint([]).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 7, type: u32 }
evidence: llvm-mc "too few operands for instruction"; gas "missing immediate expression"
```

## encode_hint_neg_wrong_kind
- Tier: 3
- Rationale: Negative/error contract: gas requires an immediate operand ("immediate operand required"); llvm-mc rejects non-imm first operands. get_imm returns Err when operand 0 is not Imm. Stronger rejected: State machine; Round-trip; Differential has no encoding.
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_brk_pbt.rs:303 (wrong kind)
- Formal: ∀ op ∈ Operand \ Imm. encode_hint([op]) is Err ∧ (asm form of op is assemblable ⇒ llvm-mc("hint " + asm(op)) is Err)
- Test file: src/backend/arm/assembler/encoder/encode_hint_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hint
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: Operand \ Imm }
  relation:
    op: holds
    expr: encode_hint([op]).is_err()
expected_error: String
generators:
  op: { gen: oneof, variants: [Reg, Symbol, Barrier, Cond, Label, Mem, Shift, Extend] }
evidence: gas "immediate operand required"; encoder/mod.rs:1076 get_imm
```
