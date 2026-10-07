# Properties: encode_v_crypto_vi

## encode_v_crypto_vi_spec_opcode
- Tier: 5
- Rationale: Strongest evidenced oracle is Reference against RISC-V Cryptography Extensions Volume II, which the SUT claims (README.md:14 Zvksh/Zvksed; encoder/mod.rs:445 "per RVV Crypto spec"). The ratified vector-crypto encoding uses major opcode OP-V (1010111), not OP-P (1110111). llvm-mc 15.0.6 cannot be a differential — it does not recognize vsm3c.vi/vsm4k.vi. State machine rejected (pure function). Round-trip rejected (no decoder). encode_v_arith_vi rejected as sibling (same-job gate: OPIVI funct3=011 / OP-V, not crypto VI).
- Doc contract: vector.rs:189-190 "Encode Zvksh/Zvksed crypto instructions with VI format" — asserted fingerprint e8d38bc7
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, (mnem,funct6) ∈ {(vsm3c.vi,101011),(vsm4k.vi,100001)}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) = Word(w) ∧ (w & 0x7F) = 0b1010111
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, uimm=0, kind=0 (vsm3c.vi); SUT opcode 0b1110111 vs OP-V 0b1010111; Word(0xae002077)
- Bug report: pbt-out/bug_reports/encode_v_crypto_vi_spec_opcode.md

```property
function: encoder.encode_v_crypto_vi
oracle: reference
predicate:
  quantifier: forall
  vars: [vd, vs2, uimm, funct6]
  domain: { vd: 0..31, vs2: 0..31, uimm: 0..31, funct6: {0b101011, 0b100001} }
  body: (encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) as Word & 0x7F) == 0b1010111
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: 0, max: 31, type: i64 }
  funct6: { gen: oneof, items: [0b101011, 0b100001], type: u32 }
evidence: encoder/mod.rs:445 "Vector crypto (Zvk*) — uses OP-P encoding space per RVV Crypto spec"; assembler/README.md:14 Zvksh/Zvksed; RISC-V Cryptography Extensions Volume II OP-V=1010111
```

## encode_v_crypto_vi_format_fields
- Tier: 4
- Rationale: Algebraic invariant from the function rustdoc packing (vd, funct3=010, uimm5, vs2, vm=1, funct6). Opcode is checked by the Reference property. This property pins every other field so an opcode bug cannot hide packing defects.
- Doc contract: vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 40191257
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, funct6 ∈ {0..63}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) = Word(w) ∧ unpack(w).vd=vd ∧ unpack(w).funct3=010 ∧ unpack(w).uimm5=uimm ∧ unpack(w).vs2=vs2 ∧ unpack(w).vm=1 ∧ unpack(w).funct6=funct6
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vi
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [vd, vs2, uimm, funct6]
  domain: { vd: 0..31, vs2: 0..31, uimm: 0..31, funct6: 0..63 }
  body: unpack(encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6)) == (vd, 0b010, uimm, vs2, 1, funct6)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: 0, max: 31, type: i64 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:190 packing comment
```

## encode_v_crypto_vi_field_isolation
- Tier: 4
- Rationale: Metamorphic isolation — changing one of vd/vs2/uimm/funct6 must not alter the other fields. Required metamorphic angle at standard tier. Stronger Reference/round-trip rejected as above.
- Doc contract: vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 40191257
- Seed: (none)
- Formal: ∀ vd_a, vd_b, vs2_a, vs2_b, uimm_a, uimm_b ∈ {0..31}, f6_a, f6_b ∈ {0..63}. let wa = encode(vd_a, vs2_a, uimm_a, f6_a). Changing only vd (resp. vs2, uimm, funct6) flips only bits [11:7] (resp. [24:20], [19:15], [31:26]).
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd_a, vd_b, vs2_a, vs2_b, uimm_a, uimm_b, f6_a, f6_b]
  domain: { vd_a: 0..31, vd_b: 0..31, vs2_a: 0..31, vs2_b: 0..31, uimm_a: 0..31, uimm_b: 0..31, f6_a: 0..63, f6_b: 0..63 }
  body: (encode(vd_a,vs2_a,uimm_a,f6_a) & !vd_mask) == (encode(vd_b,vs2_a,uimm_a,f6_a) & !vd_mask)
generators:
  vd_a: { gen: int, min: 0, max: 31, type: u32 }
  vd_b: { gen: int, min: 0, max: 31, type: u32 }
  vs2_a: { gen: int, min: 0, max: 31, type: u32 }
  vs2_b: { gen: int, min: 0, max: 31, type: u32 }
  uimm_a: { gen: int, min: 0, max: 31, type: i64 }
  uimm_b: { gen: int, min: 0, max: 31, type: i64 }
  f6_a: { gen: int, min: 0, max: 63, type: u32 }
  f6_b: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:190 disjoint field layout
```

## encode_v_crypto_vi_vd_vs2_swap
- Tier: 4
- Rationale: Metamorphic — swapping the two vector-register operands (still v-regs) must swap bits [11:7] and [24:20] and preserve every other bit, matching assembly order vd, vs2.
- Doc contract: vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 40191257
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, funct6 ∈ {0..63}. let w = encode([v{vd}, v{vs2}, Imm(uimm)], funct6); let w' = encode([v{vs2}, v{vd}, Imm(uimm)], funct6). (w >> 7 & 0x1F) = vd ∧ (w >> 20 & 0x1F) = vs2 ∧ (w' >> 7 & 0x1F) = vs2 ∧ (w' >> 20 & 0x1F) = vd ∧ (w & ~(vd_mask|vs2_mask)) = (w' & ~(vd_mask|vs2_mask))
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [vd, vs2, uimm, funct6]
  domain: { vd: 0..31, vs2: 0..31, uimm: 0..31, funct6: 0..63 }
  body: (encode([v{vd}, v{vs2}, Imm(uimm)], funct6) >> 7 & 0x1F) == vd && (encode([v{vs2}, v{vd}, Imm(uimm)], funct6) >> 7 & 0x1F) == vs2
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: 0, max: 31, type: i64 }
  funct6: { gen: int, min: 0, max: 63, type: u32 }
evidence: vector.rs:192-193 get_vreg(0)=vd, get_vreg(1)=vs2
```

## encode_v_crypto_vi_neg_arity_bad_regs
- Tier: 3
- Rationale: Negative/error contract — fewer than 3 operands, non-vector vd/vs2, or non-Imm at uimm must return Err. Evidenced by get_vreg/get_imm contracts and the 3-operand rustdoc form. Stronger oracles do not apply to the invalid domain.
- Doc contract: vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 40191257
- Seed: (none)
- Formal: ∀ ops with |ops|<3 ∨ ops[0] or ops[1] not a v-reg ∨ ops[2] not Imm, funct6 ∈ {0..63}. encode_v_crypto_vi(ops, funct6) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_v_crypto_vi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, funct6]
  domain: { ops: arity_lt_3_or_bad_vreg_or_non_imm, funct6: 0..63 }
  body: encode_v_crypto_vi(ops, funct6).is_err()
generators:
  funct6: { gen: int, min: 0, max: 63, type: u32 }
expected_error: String
evidence: vector.rs:192-194 get_vreg/get_imm; encoder/mod.rs:480-494
```

## encode_v_crypto_vi_neg_extra
- Tier: 3
- Rationale: Negative/error — a fourth operand after a complete vd, vs2, uimm5 form must Err. Public wrapper encode_instruction passes operands through; a 3-operand instruction does not accept extras. No arity check in the body (inferred contract).
- Doc contract: vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 40191257
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, extra ∈ Operand, funct6 ∈ {vsm3c.vi, vsm4k.vi}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), extra], funct6) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, uimm=0, extra=Imm(0), kind=0; Ok(Word(0xae002077))
- Bug report: pbt-out/bug_reports/encode_v_crypto_vi_extra_operand.md

```property
function: encoder.encode_v_crypto_vi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, uimm, extra, funct6]
  domain: { vd: 0..31, vs2: 0..31, uimm: 0..31, extra: Operand, funct6: {0b101011, 0b100001} }
  body: encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), extra], funct6).is_err()
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: 0, max: 31, type: i64 }
  funct6: { gen: oneof, items: [0b101011, 0b100001], type: u32 }
expected_error: String
evidence: inferred (3-operand rustdoc form; encode_instruction at encoder/mod.rs:1018,1021 passes operands through)
```

## encode_v_crypto_vi_neg_uimm_oob
- Tier: 3
- Rationale: Negative/error — rustdoc names the immediate uimm5, so the documented domain is {0..31}. Values outside that range must Err rather than wrap with `& 0x1F`. Bounds 32, -1, i64::MIN/MAX are forced.
- Doc contract: vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 40191257
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, uimm ∉ {0..31}, funct6 ∈ {vsm3c.vi, vsm4k.vi}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, uimm=-1, kind=0 (vsm3c.vi); Ok(Word(0xae0f8077))
- Bug report: pbt-out/bug_reports/encode_v_crypto_vi_uimm_oob.md

```property
function: encoder.encode_v_crypto_vi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, uimm, funct6]
  domain: { vd: 0..31, vs2: 0..31, uimm: i64_outside_0_31, funct6: {0b101011, 0b100001} }
  body: encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm)], funct6).is_err()
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: -1000, max: 1000, type: i64 }
  funct6: { gen: oneof, items: [0b101011, 0b100001], type: u32 }
expected_error: String
evidence: vector.rs:190 uimm5; vector.rs:194 `& 0x1F` with no range check
```

## encode_v_crypto_vi_neg_mask_v0t
- Tier: 3
- Rationale: Negative/error — Zvksh/Zvksed VI forms are not maskable (vm is required 1; RISC-V Crypto Volume II). A trailing v0.t token must Err, not be ignored while still emitting vm=1.
- Doc contract: vector.rs:190 "vsm3c.vi, vsm4k.vi: funct6 | vm=1 | vs2 | uimm5 | 010 | vd | OP_V_CRYPTO" — asserted fingerprint 40191257
- Seed: (none)
- Formal: ∀ vd, vs2 ∈ {0..31}, uimm ∈ {0..31}, funct6 ∈ {vsm3c.vi, vsm4k.vi}. encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), Symbol("v0.t")], funct6) = Err(_)
- Test file: src/backend/riscv/assembler/encoder/encode_v_crypto_vi_pbt.rs
- Status: failing
- Counterexample: vd=0, vs2=0, uimm=0, kind=0; Ok(Word(0xae002077))
- Bug report: pbt-out/bug_reports/encode_v_crypto_vi_mask_v0t.md

```property
function: encoder.encode_v_crypto_vi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vs2, uimm, funct6]
  domain: { vd: 0..31, vs2: 0..31, uimm: 0..31, funct6: {0b101011, 0b100001} }
  body: encode_v_crypto_vi([v{vd}, v{vs2}, Imm(uimm), Symbol("v0.t")], funct6).is_err()
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vs2: { gen: int, min: 0, max: 31, type: u32 }
  uimm: { gen: int, min: 0, max: 31, type: i64 }
  funct6: { gen: oneof, items: [0b101011, 0b100001], type: u32 }
expected_error: String
evidence: vector.rs:190 vm=1; RISC-V Cryptography Extensions Volume II (not maskable); encoder/mod.rs:962 TODO masked variants
```
