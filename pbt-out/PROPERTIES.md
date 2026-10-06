# Properties: encode_smc

## encode_smc_diff_imm
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the valid SMC immediate domain. README.md:12 claims GNU-gas-compatible assembly; encoder/mod.rs:3 claims 32-bit AArch64 words; encode() at encoder/mod.rs:985 routes `"smc"` with operands passed through. ARM ARM SMC encoding 1101 0100 000 imm16 00011 is independently specified. Stronger rejected: State machine (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SMC decoder). Sibling encode_svc/encode_hvc/encode_brk rejected by same-job gate (different op2 / group). Weaker available: algebraic.metamorphic, algebraic.invariant, negative_error.
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:213 (llvm-mc differential over imm16)
- Formal: ∀ imm ∈ 0..=65535. encode_smc([Imm(imm)]) = Word(llvm-mc("smc #imm")) ∧ llvm-mc("smc #imm") = 0xD4000003 | (imm << 5)
- Test file: src/backend/arm/assembler/encoder/encode_smc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_smc
oracle: differential
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=65535 }
  relation:
    op: eq
    lhs: encode_smc([Imm(imm)])
    rhs: Word(llvm_mc("smc #imm"))
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: src/backend/arm/assembler/README.md:12; encoder/mod.rs:3; encoder/mod.rs:985; ARM ARM SMC 0xD4000003|(imm16<<5); llvm-mc -triple=aarch64 -show-encoding
```

## encode_smc_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM SMC field layout (not the SUT mask). bits[31:21]=0b11010100000, bits[20:5]=imm16, bits[4:0]=00011, word = 0xD4000003 | (imm16 << 5). Stronger rejected: State machine (no lifecycle); Differential is the primary oracle (this is a field-level check that does not need llvm-mc); Round-trip (no decoder).
- Doc contract: src/backend/arm/assembler/encoder/mod.rs:3 "//! Encodes AArch64 instructions into 32-bit machine code words." — asserted fingerprint e88e8208
- Seed: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:223 (ARM layout invariant)
- Formal: ∀ imm ∈ 0..=65535. let w = encode_smc([Imm(imm)]). Word(w) ⇒ (w >> 21 = 0b11010100000) ∧ ((w >> 5) & 0xFFFF = imm) ∧ (w & 0x1F = 0b00011) ∧ (w = 0xD4000003 | (imm << 5))
- Test file: src/backend/arm/assembler/encoder/encode_smc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_smc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=65535 }
  relation:
    op: eq
    lhs: encode_smc([Imm(imm)])
    rhs: Word(0xD4000003 | ((imm as u32) << 5))
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: ARM ARM SMC encoding 1101 0100 000 imm16 00011; encoder/mod.rs:3
```

## encode_smc_meta_imm_isolation
- Tier: 4
- Rationale: Metamorphic: two valid imms may differ only in bits[20:5]; encode(imm) XOR encode(0) = imm << 5. Independent of llvm-mc. Stronger rejected: State machine; Round-trip (no decoder); Differential already covers value agreement.
- Doc contract: src/backend/arm/assembler/encoder/mod.rs:3 "//! Encodes AArch64 instructions into 32-bit machine code words." — asserted fingerprint e88e8208
- Seed: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:234 (imm isolation)
- Formal: ∀ imm1, imm2 ∈ 0..=65535. let w1,w2,w0 = encode_smc of each. ((w1 ⊕ w2) & ¬0x1FFFE0 = 0) ∧ (imm1 ≠ imm2 ⇒ w1 ≠ w2) ∧ (w1 ⊕ w0 = imm1 << 5)
- Test file: src/backend/arm/assembler/encoder/encode_smc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_smc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [imm1, imm2]
  domain: { imm1: 0..=65535, imm2: 0..=65535 }
  relation:
    op: eq
    lhs: (encode_smc([Imm(imm1)]) XOR encode_smc([Imm(0)]))
    rhs: (imm1 as u32) << 5
generators:
  imm1: { gen: int, min: 0, max: 65535, type: i64 }
  imm2: { gen: int, min: 0, max: 65535, type: i64 }
evidence: ARM ARM SMC imm16 occupies bits[20:5] only; encoder/mod.rs:3
```

## encode_smc_neg_extra
- Tier: 3
- Rationale: Negative/error contract: gas and llvm-mc reject extra operands ("unexpected characters following instruction" / "invalid operand"). README.md:12 claims gas-compatible assembly. encode_smc's own comments do not declare extra operands invalid, so they stay in the generator. Stronger rejected: State machine; Round-trip; Differential on extra operands has no encoding (reference rejects).
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:254 (extra operand)
- Formal: ∀ imm ∈ 0..=65535. ∀ extra ∈ Operand. llvm-mc("smc #imm, extra") is Err ⇒ encode_smc([Imm(imm), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smc_pbt.rs
- Status: failing
- Counterexample: imm = 0, extra = Reg("x0"); SUT Ok(Word(0xd4000003)) vs llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_smc_extra_operand.md

```property
function: encoder.system.encode_smc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm, extra]
  domain: { imm: 0..=65535, extra: Operand }
  relation:
    op: throws
    expr: encode_smc([Imm(imm), extra])
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  extra: { gen: operand_kind }
expected_error: extra operand must Err
evidence: README.md:12; gas "unexpected characters following instruction"; llvm-mc "invalid operand for instruction"
```

## encode_smc_neg_oob_imm
- Tier: 3
- Rationale: Negative/error contract: gas ("immediate value out of range 0 to 65535") and llvm-mc ("immediate must be an integer in range [0, 65535]") reject Imm outside 0..=65535. ARM ARM imm16 field is 16 bits. encode_smc's comments do not declare oob Imm invalid; stay in generator. Documented bound 0 and 65535 plus bound±1 must be sampled. Stronger rejected: State machine; Round-trip; Differential has no encoding on oob.
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:270 (oob imm)
- Formal: ∀ imm ∈ i64 excluding 0..=65535. llvm-mc("smc #imm") is Err ⇒ encode_smc([Imm(imm)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_smc_pbt.rs
- Status: failing
- Counterexample: imm = -1; SUT Ok(Word(0xd41fffe3)) vs llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_smc_oob_imm.md

```property
function: encoder.system.encode_smc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: i64_excluding_0_to_65535 }
  relation:
    op: throws
    expr: encode_smc([Imm(imm)])
generators:
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: imm outside 0..=65535 must Err
evidence: README.md:12; gas "immediate value out of range 0 to 65535"; llvm-mc "immediate must be an integer in range [0, 65535]"; ARM ARM imm16
```

## encode_smc_neg_empty
- Tier: 3
- Rationale: Negative/error contract: gas ("missing immediate expression") and llvm-mc ("too few operands") reject omitted SMC immediate. get_imm on empty slice returns Err. Stronger rejected: State machine; Round-trip; Differential has no encoding.
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:286 (empty operands)
- Formal: encode_smc([]) is Err ∧ llvm-mc("smc") is Err
- Test file: src/backend/arm/assembler/encoder/encode_smc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_smc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..8 }
  relation:
    op: throws
    expr: encode_smc([])
generators:
  n: { gen: int, min: 0, max: 7, type: u32 }
expected_error: empty operands must Err
evidence: README.md:12; gas "missing immediate expression"; llvm-mc "too few operands for instruction"
```

## encode_smc_neg_wrong_kind
- Tier: 3
- Rationale: Negative/error contract: gas ("immediate operand required") and llvm-mc reject a non-Imm first operand. get_imm returns Err for non-Imm. Stronger rejected: State machine; Round-trip; Differential has no encoding.
- Doc contract: src/backend/arm/assembler/README.md:12 "not enabled).  It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint b5a34d5b
- Seed: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs:297 (wrong kind)
- Formal: ∀ op ∈ Operand excluding Imm. encode_smc([op]) is Err ∧ (when assemblable) llvm-mc("smc <op>") is Err
- Test file: src/backend/arm/assembler/encoder/encode_smc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_smc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: Operand_excluding_Imm }
  relation:
    op: throws
    expr: encode_smc([op])
generators:
  op: { gen: non_imm_operand }
expected_error: non-Imm first operand must Err
evidence: README.md:12; gas "immediate operand required"; get_imm encoder/mod.rs:1074
```
