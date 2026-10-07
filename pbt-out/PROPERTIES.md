# Properties: encode_v_crypto_vs

## encode_v_crypto_vs_spec_opcode
- Tier: 5
- Rationale: Strongest evidenced oracle is Reference against RISC-V Cryptography Extensions Volume II, which the SUT claims (README.md:14 Zvksh/Zvksed; encoder/mod.rs:449 "per RVV Crypto spec"). The ratified vector-crypto encoding uses major opcode OP-V (1010111), not OP-P (1110111). llvm-mc 15.0.6 is not a differential — it does not assemble vsm4r.vs. State machine rejected (pure function). Round-trip rejected (no decoder). encode_v_arith_vv rejected as sibling (same-job gate: OPIVV funct3=000 / OP-V with vs1 a register, not crypto VS funct3=010 / vs1=10000).
- Doc contract: vector.rs:209 "Encode Zvksed crypto instructions with VS format" — asserted fingerprint b4cc0348
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}. encode_v_crypto_vs([v{vd}, v{vs2}], 0b101001) = Word(w) ∧ (w & 0x7F) = 0b1010111 ∧ w = spec_word(vd, vs2, 0b101001)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0; SUT opcode 0b1110111 vs OP-V 0b1010111; Word(0xa6082077)
- Bug report: pbt-out/bug_reports/encode_v_crypto_vs_spec_opcode.md

```property
function: encoder.encode_v_crypto_vs
oracle: reference
predicate:
  quantifier: forall
  vars: [vd, vs2]
  domain: { vd: 0..31, vs2: 0..31 }
  body: (encode_v_crypto_vs([v{vd}, v{vs2}], 0b101001) as Word) == spec_word(vd, vs2, 0b101001)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:449 "Vector crypto (Zvk*) — uses OP-P encoding space per RVV Crypto spec"; assembler/README.md:14 Zvksh/Zvksed; RISC-V Cryptography Extensions Volume II OP-V=1010111 funct6 vsm4r.vs=101001 vs1=10000
```

## encode_v_crypto_vs_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the function rustdoc packing (vd, funct3=010, vs1=10000, vs2, vm=1, funct6). Opcode is checked by the Reference property. This property pins every other field so an opcode bug cannot hide packing defects, including the VS-specific hardcoded vs1=10000.
- Doc contract: vector.rs:210 "vsm4r.vs: funct6 | vm=1 | vs2 | 10000 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 3087a9b4
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, funct6 ∈ {0..63}. encode_v_crypto_vs([v{vd}, v{vs2}], funct6) = Word(w) ∧ unpack(w).vd=vd ∧ unpack(w).funct3=010 ∧ unpack(w).vs1=0b10000 ∧ unpack(w).vs2=vs2 ∧ unpack(w).vm=1 ∧ unpack(w).funct6=funct6
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vs
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs2, funct6]
  domain: { vd: 0..31, vs2: 0..31, funct6: 0..63 }
  body: unpack(encode_v_crypto_vs([v{vd}, v{vs2}], funct6)) == (vd, 0b010, 0b10000, vs2, 1, funct6)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:210 packing comment
```

## encode_v_crypto_vs_field_isolation
- Tier: 4
- Rationale: Metamorphic isolation — changing one of vd/vs2/funct6 must not alter the other fields, and vs1 bits [19:15] stay 10000. Required metamorphic angle at standard tier. Stronger Reference/round-trip rejected as above.
- Doc contract: vector.rs:210 "vsm4r.vs: funct6 | vm=1 | vs2 | 10000 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 3087a9b4
- Seed: (none)
- Formal: ∀ vd_a, vd_b, vs2_a, vs2_b ∈ {0..31}, f6_a, f6_b ∈ {0..63}. let wa = encode(vd_a, vs2_a, f6_a). Changing only vd (resp. vs2, funct6) flips only bits [11:7] (resp. [24:20], [31:26]). Bits [19:15] remain 10000.
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vs
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, vs2_a, vs2_b, f6_a, f6_b]
  domain: { vd_a: 0..31, vd_b: 0..31, vs2_a: 0..31, vs2_b: 0..31, f6_a: 0..63, f6_b: 0..63 }
  body: (encode(vd_a,vs2_a,f6_a) & !vd_mask) == (encode(vd_b,vs2_a,f6_a) & !vd_mask)
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  vs2_a: { gen: int, min: 0, max: 31, type: u32 }
  vs2_b: { gen: int, min: 0, max: 31, type: u32 }
  f6_a: { gen: int, min: 0, max: 63, type: u32 }
  f6_b: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:210 packing comment
```

## encode_v_crypto_vs_operand_swap
- Tier: 4
- Rationale: Metamorphic — swapping assembly operands 0/1 (vd/vs2) must swap only bits [11:7] and [24:20] and preserve all other bits, including the hardcoded vs1=10000 field. Confirms operand-to-field mapping independently of the packing formula.
- Doc contract: vector.rs:210 "vsm4r.vs: funct6 | vm=1 | vs2 | 10000 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 3087a9b4
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, funct6 ∈ {0..63}. let w = encode([v{vd}, v{vs2}], funct6). encode([v{vs2}, v{vd}], funct6) has bits [11:7] and [24:20] swapped vs w and all other bits equal.
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vs
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, vs2, funct6]
  domain: { vd: 0..31, vs2: 0..31, funct6: 0..63 }
  body: (encode([v{vs2}, v{vd}], funct6) & !(vd_mask|vs2_mask)) == (encode([v{vd}, v{vs2}], funct6) & !(vd_mask|vs2_mask))
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:210 packing comment; encoder/mod.rs:1026 assembly order vd, vs2
```

## encode_v_crypto_vs_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract. get_vreg returns Err for missing operands and non-vector names. Volume II vsm4r.vs requires two vector registers. Stronger oracles do not apply to the invalid domain.
- Doc contract: vector.rs:209 "Encode Zvksed crypto instructions with VS format" — asserted fingerprint b4cc0348
- Seed: (none)
- Formal: ∀ ops with |ops| < 2 ∨ ops[i] not a v-reg for some i ∈ {0,1}, funct6 ∈ {0..63}. encode_v_crypto_vs(ops, funct6) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, funct6]
  domain: { ops: short_or_bad_vreg, funct6: 0..63 }
  body: encode_v_crypto_vs(ops, funct6).is_err()
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
expected_error: String
evidence: encoder/mod.rs:484 get_vreg Err on missing/non-vector; Volume II vsm4r.vs vd, vs2
```

## encode_v_crypto_vs_neg_extra
- Tier: 3
- Rationale: Negative/error contract. Volume II assembly is `vsm4r.vs vd, vs2` (exactly two operands). encode_instruction passes operands through, so extra tokens must be rejected by this encoder. Stronger oracles do not apply to the invalid domain.
- Doc contract: vector.rs:209 "Encode Zvksed crypto instructions with VS format" — asserted fingerprint b4cc0348
- Seed: encode_v_crypto_vv_pbt.rs extra-operand property (same encoder family)
- Formal: ∀ vd, vs2 ∈ {0..31}, extra. encode_v_crypto_vs([v{vd}, v{vs2}, extra], 0b101001) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, extra=Imm(0); Ok(Word(0xa6082077))
- Bug report: pbt-out/bug_reports/encode_v_crypto_vs_extra_operand.md

```property
function: encoder.encode_v_crypto_vs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, extra]
  domain: { vd: 0..31, vs2: 0..31, extra: Operand }
  body: encode_v_crypto_vs([v{vd}, v{vs2}, extra], 0b101001).is_err()
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: [{ gen: int, min: -1, max: 208, type: i64 }, { gen: string }] }
expected_error: String
evidence: Volume II vsm4r.vs vd, vs2; encoder/mod.rs:1026 operands passed through
```

## encode_v_crypto_vs_neg_mask_v0t
- Tier: 3
- Rationale: Negative/error contract. Volume II Zvksed VS is not maskable (vm required 1). A trailing v0.t is invalid assembly and must be Err, not silently encoded as unmasked.
- Doc contract: vector.rs:210 "vsm4r.vs: funct6 | vm=1 | vs2 | 10000 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 3087a9b4
- Seed: encode_v_crypto_vv_pbt.rs v0.t property (same encoder family)
- Formal: ∀ vd, vs2 ∈ {0..31}. encode_v_crypto_vs([v{vd}, v{vs2}, Symbol("v0.t")], 0b101001) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vs_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, Symbol("v0.t"); Ok(Word(0xa6082077))
- Bug report: pbt-out/bug_reports/encode_v_crypto_vs_mask_v0t.md

```property
function: encoder.encode_v_crypto_vs
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2]
  domain: { vd: 0..31, vs2: 0..31 }
  body: encode_v_crypto_vs([v{vd}, v{vs2}, Symbol("v0.t")], 0b101001).is_err()
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: vector.rs:210 vm=1; Volume II Zvksed VS not maskable
```
