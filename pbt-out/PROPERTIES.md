# Properties: encode_svc

## encode_svc_diff_imm
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, the independent AArch64 assembler the README claims gas-compatibility with. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree SVC decoder). Sibling encode_hvc/encode_smc/encode_brk rejected (same-job gate: HVC op2=010, SMC op2=011, BRK different group). Domain is ARM imm16 0..=65535; llvm-mc success must agree on the word. Documented bounds sampled exactly (generator min/max pin 0 and 65535). Required metamorphic/differential property.
- Doc contract: (none) — encode_svc has no rustdoc or body comment
- Seed: encode_dmb_pbt.rs llvm-mc differential; README.md:12
- Formal: ∀ imm ∈ 0..=65535. encode_svc([Imm(imm)]) = Ok(Word(llvm-mc("svc #"+imm)))
- Test file: src/backend/arm/assembler/encoder/encode_svc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_svc
oracle: differential
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=65535 }
  relation:
    op: eq
    lhs: encode_svc([Imm(imm)])
    rhs: Word(llvm_mc("svc #"+imm))
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: "README.md:12 gas-compatible textual assembly; encoder/mod.rs:977 svc => encode_svc; ARM ARM SVC imm16"
```

## encode_svc_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM SVC encoding (1101 0100 000 imm16 00001), independent of llvm-mc. Stronger differential already used as primary. Round-trip rejected (no decoder). Predicate is the ARM field layout, not a restatement of the SUT mask. Documented bounds sampled exactly (0 and 65535).
- Doc contract: (none) — encode_svc has no rustdoc or body comment
- Seed: encode_dmb_pbt.rs encode_dmb_inv_arm_layout
- Formal: ∀ imm ∈ 0..=65535. let w = encode_svc([Imm(imm)]); w = 0xD4000001 | (imm << 5) ∧ (w >> 21) = 0b11010100000 ∧ ((w >> 5) & 0xFFFF) = imm ∧ (w & 0x1F) = 0b00001
- Test file: src/backend/arm/assembler/encoder/encode_svc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_svc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: 0..=65535 }
  body: let w = encode_svc([Imm(imm)]); w == 0xD4000001 | ((imm as u32) << 5) && (w >> 21) == 0b11010100000 && ((w >> 5) & 0xFFFF) == (imm as u32) && (w & 0x1F) == 0b00001
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: "ARM ARM SVC encoding 11010100 000 imm16 00001; encoder/mod.rs:3 32-bit machine code words"
```

## encode_svc_meta_imm_isolation
- Tier: 4
- Rationale: Metamorphic: two valid SVC encodings differ only in the imm16 field (bits[20:5]). Independent of the absolute opcode. Stronger differential already used as primary. Round-trip rejected (no decoder). Idempotence does not apply (encoder is not a normalizer).
- Doc contract: (none) — encode_svc has no rustdoc or body comment
- Seed: encode_msr_pbt.rs encode_msr_inv_arm_layout (Rt isolation generalized to imm16 isolation)
- Formal: ∀ imm1, imm2 ∈ 0..=65535. (encode_svc([Imm(imm1)]) XOR encode_svc([Imm(imm2)])) & !0x1FFFE0 = 0 ∧ (imm1 ≠ imm2 ⇒ encode_svc([Imm(imm1)]) ≠ encode_svc([Imm(imm2)])) ∧ encode_svc([Imm(imm1)]) XOR encode_svc([Imm(0)]) = imm1 << 5
- Test file: src/backend/arm/assembler/encoder/encode_svc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_svc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [imm1, imm2]
  domain: { imm1: 0..=65535, imm2: 0..=65535 }
  body: let w1 = encode_svc([Imm(imm1)]); let w2 = encode_svc([Imm(imm2)]); let w0 = encode_svc([Imm(0)]); (w1 ^ w2) & !0x1FFFE0 == 0 && (imm1 != imm2 implies w1 != w2) && (w1 ^ w0) == ((imm1 as u32) << 5)
generators:
  imm1: { gen: int, min: 0, max: 65535, type: i64 }
  imm2: { gen: int, min: 0, max: 65535, type: i64 }
evidence: "ARM ARM SVC only imm16 occupies bits[20:5]; opcode bits[31:21] and bits[4:0]=00001 are fixed"
```

## encode_svc_neg_extra
- Tier: 4
- Rationale: Negative/error contract from gas/llvm-mc (README gas-compatibility): extra operands are invalid. encode() passes operands through unchanged so extras are caller-reachable. Stronger differential already covers the valid domain. No documented exclusion of extra operands on encode_svc itself, so extras stay in the generator; SUT Err is the required result.
- Doc contract: (none) — encode_svc has no rustdoc or body comment
- Seed: encode_dmb_pbt.rs encode_dmb_neg_extra
- Formal: ∀ imm ∈ 0..=65535, extra ∈ Operand. llvm-mc("svc #"+imm+", "+asm(extra))=Err ⇒ encode_svc([Imm(imm), extra])=Err
- Test file: src/backend/arm/assembler/encoder/encode_svc_pbt.rs
- Status: failing
- Counterexample: imm = 0, extra = Reg("x0"); SUT Ok(Word(0xd4000001)) vs llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_svc_extra_operand.md

```property
function: encoder.system.encode_svc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm, extra]
  domain: { imm: 0..=65535, extra: Operand }
  relation:
    op: throws
    lhs: encode_svc([Imm(imm), extra])
    rhs: extra_operand_rejected
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  extra: { gen: oneof, items: [Reg, Imm, Barrier, Cond, Label] }
expected_error: String
evidence: "README.md:12 gas-compatible; llvm-mc rejects 'svc #0, x0'; gas 'unexpected characters following instruction'; encoder/mod.rs:977 operands passed through"
```

## encode_svc_neg_oob_imm
- Tier: 4
- Rationale: Negative/error contract from ARM ARM imm16 and gas/llvm-mc: immediate must be in [0, 65535]. Documented bounds sampled at bound±1 (-1 and 65536 pinned). Stronger differential already covers the in-range half. encode_svc does not declare out-of-range Imm invalid, so the domain stays open; SUT Err is the required result. Wrapping values (0x10000, 0x1FFFF) are generated because field masking is the suspected bug class.
- Doc contract: (none) — encode_svc has no rustdoc or body comment
- Seed: encode_dmb_pbt.rs encode_dmb_neg_wrong_kind_imm_oob
- Formal: ∀ imm ∈ ℤ \ [0, 65535]. llvm-mc("svc #"+imm)=Err ⇒ encode_svc([Imm(imm)])=Err
- Test file: src/backend/arm/assembler/encoder/encode_svc_pbt.rs
- Status: failing
- Counterexample: imm = -1; SUT Ok(Word(0xd41fffe1)) vs llvm-mc Err
- Bug report: pbt-out/bug_reports/encode_svc_oob_imm.md

```property
function: encoder.system.encode_svc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: i64 \ [0, 65535] }
  relation:
    op: throws
    lhs: encode_svc([Imm(imm)])
    rhs: imm_out_of_range
generators:
  imm: { gen: oneof, items: [-1, 65536, i64::MIN, i64::MAX, 65537, -65536, 0x10000, 0x1FFFF] }
expected_error: String
evidence: "ARM ARM SVC imm16; llvm-mc 'immediate must be an integer in range [0, 65535]'; gas 'immediate value out of range 0 to 65535'"
```

## encode_svc_neg_empty
- Tier: 4
- Rationale: Negative/error contract from gas/llvm-mc: omitted operand is invalid (`svc` alone). get_imm on an empty slice returns Err; the property asserts that documented assembler rejection, not the helper's current string. Stronger differential already covers the valid domain.
- Doc contract: (none) — encode_svc has no rustdoc or body comment
- Seed: encode_dmb_pbt.rs encode_dmb_neg_empty
- Formal: encode_svc([])=Err ∧ llvm-mc("svc")=Err
- Test file: src/backend/arm/assembler/encoder/encode_svc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_svc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..=7 }
  relation:
    op: throws
    lhs: encode_svc([])
    rhs: missing_immediate
generators:
  n: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: "README.md:12 gas-compatible; llvm-mc 'too few operands for instruction'; gas 'missing immediate expression at operand 1'"
```

## encode_svc_neg_wrong_kind
- Tier: 4
- Rationale: Negative/error contract from gas/llvm-mc: first operand must be an immediate. Non-Imm kinds (Reg/Symbol/Barrier/Cond/Mem/Shift/Label/Extend) must Err. Stronger differential already covers Imm. First Evidence is gas/llvm-mc, not get_imm's error string.
- Doc contract: (none) — encode_svc has no rustdoc or body comment
- Seed: encode_dmb_pbt.rs encode_dmb_neg_wrong_kind_imm_oob
- Formal: ∀ op ∈ Operand \ Imm. llvm-mc("svc "+asm(op))=Err ⇒ encode_svc([op])=Err
- Test file: src/backend/arm/assembler/encoder/encode_svc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_svc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: Operand \ Imm }
  relation:
    op: throws
    lhs: encode_svc([op])
    rhs: expected_immediate
generators:
  op: { gen: oneof, items: [Reg, Symbol, Barrier, Cond, Mem, Shift, Label, Extend] }
expected_error: String
evidence: "README.md:12 gas-compatible; llvm-mc 'immediate must be an integer in range [0, 65535]' for 'svc x0'; gas 'immediate operand required at operand 1'"
```
