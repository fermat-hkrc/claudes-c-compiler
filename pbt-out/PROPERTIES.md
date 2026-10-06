# Properties: encode_hvc

## encode_hvc_diff_imm
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, the independent AArch64 assembler the README claims gas-compatibility with. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree HVC decoder). Sibling encode_svc/encode_smc/encode_brk rejected (same-job gate: SVC op2=001, SMC op2=011, BRK different group). Domain is ARM imm16 0..=65535; llvm-mc success must agree on the word. Documented bounds sampled exactly (generator min/max pin 0 and 65535). Required metamorphic/differential property.
- Doc contract: (none) — encode_hvc has no rustdoc or body comment
- Seed: encode_svc_pbt.rs encode_svc_diff_imm; README.md:12
- Formal: ∀ imm ∈ 0..=65535. encode_hvc([Imm(imm)]) = Ok(Word(llvm-mc("hvc #"+imm)))
- Test file: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hvc
oracle: differential
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=65535 }
  relation:
    op: eq
    lhs: encode_hvc([Imm(imm)])
    rhs: Word(llvm_mc("hvc #"+imm))
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: "README.md:12 gas-compatible textual assembly; encoder/mod.rs:980 hvc => encode_hvc; ARM ARM HVC imm16"
```

## encode_hvc_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM HVC encoding (1101 0100 000 imm16 00010), independent of llvm-mc. Stronger differential already used as primary. Round-trip rejected (no decoder). Predicate is the ARM field layout, not a restatement of the SUT mask. Documented bounds sampled exactly (0 and 65535).
- Doc contract: (none) — encode_hvc has no rustdoc or body comment
- Seed: encode_svc_pbt.rs encode_svc_inv_arm_layout
- Formal: ∀ imm ∈ 0..=65535. let w = encode_hvc([Imm(imm)]); w = 0xD4000002 | (imm << 5) ∧ (w >> 21) = 0b11010100000 ∧ ((w >> 5) & 0xFFFF) = imm ∧ (w & 0x1F) = 0b00010
- Test file: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hvc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=65535 }
  body: let w = encode_hvc([Imm(imm)]); w == 0xD4000002 | ((imm as u32) << 5) && (w >> 21) == 0b11010100000 && ((w >> 5) & 0xFFFF) == (imm as u32) && (w & 0x1F) == 0b00010
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: "ARM ARM HVC encoding 11010100 000 imm16 00010; encoder/mod.rs:3 32-bit machine code words"
```

## encode_hvc_meta_imm_isolation
- Tier: 4
- Rationale: Metamorphic: two valid HVC encodings differ only in the imm16 field (bits[20:5]). Independent of the absolute opcode. Stronger differential already used as primary. Round-trip rejected (no decoder). Idempotence does not apply (encoder is not a normalizer).
- Doc contract: (none) — encode_hvc has no rustdoc or body comment
- Seed: encode_svc_pbt.rs encode_svc_meta_imm_isolation
- Formal: ∀ imm1, imm2 ∈ 0..=65535. (encode_hvc([Imm(imm1)]) XOR encode_hvc([Imm(imm2)])) & !0x1FFFE0 = 0 ∧ (imm1 ≠ imm2 ⇒ encode_hvc([Imm(imm1)]) ≠ encode_hvc([Imm(imm2)])) ∧ encode_hvc([Imm(imm1)]) XOR encode_hvc([Imm(0)]) = imm1 << 5
- Test file: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hvc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [imm1, imm2]
  domain: { imm1: 0..=65535, imm2: 0..=65535 }
  body: let w1 = encode_hvc([Imm(imm1)]); let w2 = encode_hvc([Imm(imm2)]); let w0 = encode_hvc([Imm(0)]); (w1 ^ w2) & !0x1FFFE0 == 0 && (imm1 != imm2 implies w1 != w2) && (w1 ^ w0) == ((imm1 as u32) << 5)
generators:
  imm1: { gen: int, min: 0, max: 65535, type: i64 }
  imm2: { gen: int, min: 0, max: 65535, type: i64 }
evidence: "ARM ARM HVC only imm16 occupies bits[20:5]; opcode bits[31:21] and bits[4:0]=00010 are fixed"
```

## encode_hvc_neg_extra
- Tier: 4
- Rationale: Negative/error contract from gas/llvm-mc: extra operands after the imm16 are rejected ("unexpected characters following instruction" / "invalid operand for instruction"). Stronger differential covers the valid domain. encode_hvc's own comments do not declare extra operands invalid, so they stay in the generator. README gas-compatibility is the contract.
- Doc contract: (none) — encode_hvc has no rustdoc or body comment
- Seed: encode_svc_pbt.rs encode_svc_neg_extra
- Formal: ∀ imm ∈ 0..=65535. ∀ extra ∈ Operand. llvm-mc("hvc #"+imm+", "+extra) is Err ∧ encode_hvc([Imm(imm), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
- Status: failing
- Counterexample: imm = 0, extra = Reg("x0"); SUT Ok(Word(0xd4000002)) vs llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_hvc_extra_operand.md

```property
function: encoder.system.encode_hvc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm, extra]
  domain: { imm: 0..=65535, extra: Operand }
  relation:
    op: throws
    expr: encode_hvc([Imm(imm), extra])
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  extra: { gen: oneof, of: [Reg, Imm, Barrier, Cond, Label, Symbol] }
expected_error: String
evidence: "gas: unexpected characters following instruction; llvm-mc: invalid operand for instruction; README.md:12"
```

## encode_hvc_neg_oob_imm
- Tier: 4
- Rationale: Negative/error contract from ARM ARM / gas / llvm-mc: imm16 must be in 0..=65535. llvm-mc: "immediate must be an integer in range [0, 65535]". gas: "immediate value out of range 0 to 65535". encode_hvc's own comments do not declare oob Imm invalid; keep it in the generator. Documented bounds sampled at bound±1 (-1, 65536).
- Doc contract: (none) — encode_hvc has no rustdoc or body comment
- Seed: encode_svc_pbt.rs encode_svc_neg_oob_imm
- Formal: ∀ imm ∈ i64 \\ 0..=65535. llvm-mc("hvc #"+imm) is Err ∧ encode_hvc([Imm(imm)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
- Status: failing
- Counterexample: imm = -1; SUT Ok(Word(0xd41fffe2)) vs llvm-mc Err. Also imm = 65536 → Ok(Word(0xd4000002))
- Bug report: pbt-out/bug_reports/encode_hvc_oob_imm.md

```property
function: encoder.system.encode_hvc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: i64 \\ 0..=65535 }
  relation:
    op: throws
    expr: encode_hvc([Imm(imm)])
generators:
  imm: { gen: int, min: -4096, max: 70000, type: i64 }
expected_error: String
evidence: "ARM ARM HVC imm16 0..=65535; llvm-mc range [0, 65535]; gas immediate value out of range 0 to 65535"
```

## encode_hvc_neg_empty
- Tier: 4
- Rationale: Negative/error contract from gas/llvm-mc: omitted immediate is rejected (gas "missing immediate expression"; llvm-mc "too few operands"). get_imm on empty slice returns Err; this is the documented assembler contract, not a restatement of get_imm.
- Doc contract: (none) — encode_hvc has no rustdoc or body comment
- Seed: encode_svc_pbt.rs encode_svc_neg_empty
- Formal: ∀ (). llvm-mc("hvc") is Err ∧ encode_hvc([]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hvc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..8 }
  relation:
    op: throws
    expr: encode_hvc([])
generators:
  n: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: "gas: missing immediate expression at operand 1; llvm-mc: too few operands for instruction; README.md:12"
```

## encode_hvc_neg_wrong_kind
- Tier: 4
- Rationale: Negative/error contract from gas/llvm-mc: first operand must be an immediate. Non-Imm kinds (Reg/Symbol/Barrier/Cond/Label/Mem/Shift/Extend) are rejected. encode_hvc's comments do not declare these invalid; they stay in the generator.
- Doc contract: (none) — encode_hvc has no rustdoc or body comment
- Seed: encode_svc_pbt.rs encode_svc_neg_wrong_kind
- Formal: ∀ op ∈ Operand \\ Imm. llvm-mc("hvc "+op) is Err ∧ encode_hvc([op]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_hvc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_hvc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: Operand \\ Imm }
  relation:
    op: throws
    expr: encode_hvc([op])
generators:
  op: { gen: oneof, of: [Reg, Symbol, Barrier, Cond, Label, Mem, Shift, Extend] }
expected_error: String
evidence: "llvm-mc: immediate must be an integer in range [0, 65535]; gas: immediate operand required; README.md:12"
```
