# Properties: encode_v_crypto_vv

## encode_v_crypto_vv_spec_opcode
- Tier: 5
- Rationale: Strongest evidenced oracle is Reference against RISC-V Cryptography Extensions Volume II, which the SUT claims (README.md:14 Zvksh/Zvksed; encoder/mod.rs:447 "per RVV Crypto spec"). The ratified vector-crypto encoding uses major opcode OP-V (1010111), not OP-P (1110111). llvm-mc 15.0.6 is not a differential — it does not assemble vsm3me.vv. State machine rejected (pure function). Round-trip rejected (no decoder). encode_v_arith_vv rejected as sibling (same-job gate: OPIVV funct3=000 / OP-V, not crypto VV funct3=010).
- Doc contract: vector.rs:199 "Encode Zvksh crypto instructions with VV format" — asserted fingerprint d0550a05
- Seed: (none)
- Formal: ∀ vd, vs2, vs1 ∈ {0..31}. encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}], 0b100000) = Word(w) ∧ (w & 0x7F) = 0b1010111 ∧ w = spec_word(vd, vs2, vs1, 0b100000)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, vs1=0; SUT opcode 0b1110111 vs OP-V 0b1010111; Word(0x82002077)
- Bug report: pbt-out/bug_reports/encode_v_crypto_vv_spec_opcode.md

```property
function: encoder.encode_v_crypto_vv
oracle: reference
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31 }
  body: (encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}], 0b100000) as Word) == spec_word(vd, vs2, vs1, 0b100000)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
evidence: encoder/mod.rs:447 "Vector crypto (Zvk*) — uses OP-P encoding space per RVV Crypto spec"; assembler/README.md:14 Zvksh/Zvksed; RISC-V Cryptography Extensions Volume II OP-V=1010111 funct6 vsm3me.vv=100000
```

## encode_v_crypto_vv_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the function rustdoc packing (vd, funct3=010, vs1, vs2, vm=1, funct6). Opcode is checked by the Reference property. This property pins every other field so an opcode bug cannot hide packing defects.
- Doc contract: vector.rs:200 "vsm3me.vv: funct6 | vm=1 | vs2 | vs1 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 635e5dc3
- Seed: (none)
- Formal: ∀ vd, vs2, vs1 ∈ {0..31}, funct6 ∈ {0..63}. encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}], funct6) = Word(w) ∧ unpack(w).vd=vd ∧ unpack(w).funct3=010 ∧ unpack(w).vs1=vs1 ∧ unpack(w).vs2=vs2 ∧ unpack(w).vm=1 ∧ unpack(w).funct6=funct6
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, funct6]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31, funct6: 0..63 }
  body: unpack(encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}], funct6)) == (vd, 0b010, vs1, vs2, 1, funct6)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:200 packing comment
```

## encode_v_crypto_vv_field_isolation
- Tier: 4
- Rationale: Metamorphic isolation — changing one of vd/vs2/vs1/funct6 must not alter the other fields. Required metamorphic angle at standard tier. Stronger Reference/round-trip rejected as above.
- Doc contract: vector.rs:200 "vsm3me.vv: funct6 | vm=1 | vs2 | vs1 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 635e5dc3
- Seed: (none)
- Formal: ∀ vd_a, vd_b, vs2_a, vs2_b, vs1_a, vs1_b ∈ {0..31}, f6_a, f6_b ∈ {0..63}. let wa = encode(vd_a, vs2_a, vs1_a, f6_a). Changing only vd (resp. vs2, vs1, funct6) flips only bits [11:7] (resp. [24:20], [19:15], [31:26]).
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, vs2_a, vs2_b, vs1_a, vs1_b, f6_a, f6_b]
  domain: { vd_a: 0..31, vd_b: 0..31, vs2_a: 0..31, vs2_b: 0..31, vs1_a: 0..31, vs1_b: 0..31, f6_a: 0..63, f6_b: 0..63 }
  body: (encode(vd_a,vs2_a,vs1_a,f6_a) & !vd_mask) == (encode(vd_b,vs2_a,vs1_a,f6_a) & !vd_mask)
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  vs2_a: { gen: int, min: 0, max: 31, type: u32 }
  vs2_b: { gen: int, min: 0, max: 31, type: u32 }
  vs1_a: { gen: int, min: 0, max: 31, type: u32 }
  vs1_b: { gen: int, min: 0, max: 31, type: u32 }
  f6_a: { gen: int, min: 0, max: 63, type: u32 }
  f6_b: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:200 packing comment
```

## encode_v_crypto_vv_operand_swap
- Tier: 4
- Rationale: Metamorphic — swapping assembly operands 0/1/2 (vd/vs2/vs1) must swap only the corresponding 5-bit fields and preserve all other bits. Confirms operand-to-field mapping independently of the packing formula.
- Doc contract: vector.rs:200 "vsm3me.vv: funct6 | vm=1 | vs2 | vs1 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 635e5dc3
- Seed: (none)
- Formal: ∀ vd, vs2, vs1 ∈ {0..31}, funct6 ∈ {0..63}. let w = encode([v{vd}, v{vs2}, v{vs1}], funct6). encode([v{vs2}, v{vd}, v{vs1}], funct6) has bits [11:7] and [24:20] swapped vs w and all other bits equal. encode([v{vd}, v{vs1}, v{vs2}], funct6) has bits [24:20] and [19:15] swapped vs w and all other bits equal.
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, funct6]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31, funct6: 0..63 }
  body: (encode([v{vs2}, v{vd}, v{vs1}], funct6) & !(vd_mask|vs2_mask)) == (encode([v{vd}, v{vs2}, v{vs1}], funct6) & !(vd_mask|vs2_mask))
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:200 packing comment; encoder/mod.rs:1021 assembly order vd, vs2, vs1
```

## encode_v_crypto_vv_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract. get_vreg returns Err for missing operands and non-vector names. Volume II vsm3me.vv requires three vector registers. Stronger oracles do not apply to the invalid domain.
- Doc contract: vector.rs:199 "Encode Zvksh crypto instructions with VV format" — asserted fingerprint d0550a05
- Seed: (none)
- Formal: ∀ ops with |ops| < 3 ∨ ops[i] not a v-reg for some i ∈ {0,1,2}. encode_v_crypto_vv(ops, funct6) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, funct6]
  domain: { ops: arity<3 or non-vreg at 0/1/2, funct6: 0..63 }
  body: encode_v_crypto_vv(ops, funct6).is_err()
generators:
  ops: { gen: oneof, items: [empty, one_vreg, two_vregs, three_with_bad_vreg] }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
expected_error: String
evidence: encoder/mod.rs:482 get_vreg Err on missing/non-vreg; Volume II vsm3me.vv vd, vs2, vs1
```

## encode_v_crypto_vv_neg_extra
- Tier: 3
- Rationale: Negative/error contract. Volume II assembly is exactly three operands. encode_instruction passes operands through, so extra operands are caller-reachable. The encoder must reject a fourth operand rather than silently ignore it.
- Doc contract: vector.rs:199 "Encode Zvksh crypto instructions with VV format" — asserted fingerprint d0550a05
- Seed: (none)
- Formal: ∀ vd, vs2, vs1 ∈ {0..31}, extra ∈ Operand, funct6 ∈ {0b100000}. encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}, extra], funct6) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, vs1=0, extra=Imm(0); Ok(Word(0x82002077))
- Bug report: pbt-out/bug_reports/encode_v_crypto_vv_extra_operand.md

```property
function: encoder.encode_v_crypto_vv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1, extra]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31, extra: Operand }
  body: encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}, extra], 0b100000).is_err()
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: [Imm, Reg, Symbol, Label, Mem, FenceArg, Csr, RoundingMode] }
expected_error: String
evidence: assembler/README.md:14 Zvksh; RISC-V Cryptography Extensions Volume II vsm3me.vv vd, vs2, vs1 (exactly 3 operands); encoder/mod.rs:1021 operands passed through
```

## encode_v_crypto_vv_neg_mask_v0t
- Tier: 3
- Rationale: Negative/error contract. Volume II Zvksh vsm3me.vv is not maskable (vm=1 always). A trailing v0.t must be rejected, not silently encoded unmasked.
- Doc contract: vector.rs:200 "vsm3me.vv: funct6 | vm=1 | vs2 | vs1 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 635e5dc3
- Seed: (none)
- Formal: ∀ vd, vs2, vs1 ∈ {0..31}. encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}, Symbol("v0.t")], 0b100000) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vv_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, vs1=0, Symbol("v0.t"); Ok(Word(0x82002077))
- Bug report: pbt-out/bug_reports/encode_v_crypto_vv_mask_v0t.md

```property
function: encoder.encode_v_crypto_vv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, vs1]
  domain: { vd: 0..31, vs2: 0..31, vs1: 0..31 }
  body: encode_v_crypto_vv([v{vd}, v{vs2}, v{vs1}, Symbol("v0.t")], 0b100000).is_err()
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  vs1: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: RISC-V Cryptography Extensions Volume II vsm3me.vv not maskable vm=1; vector.rs:200 vm=1
```
