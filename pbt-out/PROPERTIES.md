# Properties: encode_pop16

## encode_pop16_diff_r16
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc i686; GP r16 short-form POP is Intel 0x58+rw with operand-size override 0x66 under popw. State machine N/A (pure encode). Round-trip N/A (no decoder).
- Doc contract: system.rs:324 "Encode popw (16-bit pop)" — asserted fingerprint b2a11c22
- Seed: (none) — sibling encode_mov_seg_pbt pattern
- Formal: ∀ r ∈ {ax,cx,dx,bx,sp,bp,si,di}. encode_pop16([r]) = llvm_mc("popw %r")
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop16
oracle: differential
predicate:
  quantifier: forall
  vars: [r16]
  domain: { r16: r16_gp }
  relation:
    op: eq
    lhs: "sut_encode(popw, [Register(r16)])"
    rhs: "llvm_mc(popw %r16)"
generators:
  r16: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"], type: "&str" }
evidence: system.rs:340-346; Intel POP r16 short form 0x58+rw + 0x66
```

## encode_pop16_diff_sreg
- Tier: 5
- Rationale: Differential vs llvm-mc for popw to ES/SS/DS/FS/GS. Comment at system.rs:333 claims no 0x66; llvm-mc emits 0x66 for popw (not popl). Keep domain full; failure is the finding.
- Doc contract: system.rs:333 "Segment register pops don't use 0x66 prefix" — limitation fingerprint (code asserts no 0x66; popw contract requires it)
- Seed: (none)
- Formal: ∀ s ∈ {es,ss,ds,fs,gs}. encode_pop16([s]) = llvm_mc("popw %s")
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: failing
- Counterexample: popw %es → SUT [0x07], llvm-mc [0x66, 0x07]
- Bug report: bug_reports/encode_pop16_sreg_missing_66.md

```property
function: encode_pop16
oracle: differential
predicate:
  quantifier: forall
  vars: [sreg]
  domain: { sreg: poppable_sreg }
  relation:
    op: eq
    lhs: "sut_encode(popw, [Register(sreg)])"
    rhs: "llvm_mc(popw %sreg)"
generators:
  sreg: { gen: oneof, values: ["es","ss","ds","fs","gs"], type: "&str" }
evidence: system.rs:331-336; llvm-mc popw %es = [66,07]
```

## encode_pop16_diff_mem
- Tier: 5
- Rationale: Intel POP r/m16 memory form is 0x8F /0 with 0x66 under popw. SUT currently rejects Memory. Differential against llvm-mc (base/disp/SIB/abs + segmented).
- Doc contract: system.rs:324 "Encode popw (16-bit pop)" — asserted fingerprint b2a11c22
- Seed: (none)
- Formal: ∀ mem ∈ valid_i686_mem. encode_pop16([mem]) = llvm_mc("popw mem")
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: failing
- Counterexample: popw (%eax) → SUT Err("unsupported popw operand"), llvm-mc [0x66, 0x8f, 0x00]
- Bug report: bug_reports/encode_pop16_mem_unsupported.md

```property
function: encode_pop16
oracle: differential
predicate:
  quantifier: forall
  vars: [mem]
  domain: { mem: i686_mem }
  relation:
    op: eq
    lhs: "sut_encode(popw, [Memory(mem)])"
    rhs: "llvm_mc(popw mem)"
generators:
  mem: { gen: custom, type: "MemoryOperand" }
evidence: Intel POP r/m16 = 8F /0; llvm-mc popw (%eax) = [66,8f,00]
```

## encode_pop16_invariant_r16
- Tier: 4
- Rationale: Algebraic invariant for GP short form independent of llvm-mc: bytes = [0x66, 0x58+reg_num].
- Doc contract: system.rs:324 "Encode popw (16-bit pop)" — asserted fingerprint b2a11c22
- Seed: (none)
- Formal: ∀ r ∈ R16. encode_pop16([r]) = [0x66, 0x58+reg_num(r)]
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop16
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r16]
  domain: { r16: r16_gp }
  relation:
    op: eq
    lhs: "sut_encode(popw, [Register(r16)])"
    rhs: "[0x66, 0x58 + reg_num(r16)]"
generators:
  r16: { gen: oneof, values: ["ax","cx","dx","bx","sp","bp","si","di"], type: "&str" }
evidence: system.rs:340-346
```

## encode_pop16_metamorphic_popw_vs_popl_gp
- Tier: 4
- Rationale: Metamorphic — popw r16 must equal 0x66 concatenated with popl r32 short-form opcode byte (same register number family).
- Doc contract: system.rs:324 "Encode popw (16-bit pop)" — asserted fingerprint b2a11c22
- Seed: (none)
- Formal: ∀ (r16,r32) paired. encode_pop16([r16]) = [0x66] ‖ encode_pop([r32])
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop16
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [pair]
  domain: { pair: r16_r32_pairs }
  relation:
    op: eq
    lhs: "encode_pop16([r16])"
    rhs: "[0x66] ++ encode_pop([r32])"
generators:
  pair: { gen: oneof, values: [["ax","eax"],["cx","ecx"],["dx","edx"],["bx","ebx"],["sp","esp"],["bp","ebp"],["si","esi"],["di","edi"]] }
evidence: system.rs:340-346 vs gp_integer.rs:416-419
```

## encode_pop16_metamorphic_sreg_vs_popl
- Tier: 4
- Rationale: Metamorphic — popw Sreg should be 0x66 ‖ popl Sreg bytes. Same root cause as encode_pop16_diff_sreg (b1); retired as duplicate ledger entry so one property owns the bug. Test still present in harness as corroborating witness.
- Doc contract: system.rs:333 "Segment register pops don't use 0x66 prefix" — limitation (asserts wrong no-prefix behavior for popw)
- Seed: (none)
- Formal: ∀ s ∈ {es,ss,ds,fs,gs}. encode_pop16([s]) = [0x66] ‖ encode_pop([s])
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: retired
- Counterexample: popw %es → [0x07], expect [0x66,0x07] (owned by p2/b1 for bug linkage)
- Bug report: bug_reports/encode_pop16_sreg_missing_66.md
- Retired reason: duplicate metamorphic witness of b1 (p2 diff_sreg); one propertyId↔bugId pair in report.json
- Re-verified: cargo test --lib encode_pop16_metamorphic_sreg_vs_popl -- --test-threads=1 → FAIL (sreg="es", left=[07] right=[66,07]); still reproduces on real SUT; retired only as duplicate ledger entry of b1, not as a pass

```property
function: encode_pop16
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [sreg]
  domain: { sreg: poppable_sreg }
  relation:
    op: eq
    lhs: "encode_pop16([sreg])"
    rhs: "[0x66] ++ encode_pop([sreg])"
generators:
  sreg: { gen: oneof, values: ["es","ss","ds","fs","gs"], type: "&str" }
evidence: llvm-mc popw %es=[66,07] popl %es=[07]
```

## encode_pop16_neg_arity_cs
- Tier: 3
- Rationale: Negative/error — arity ≠ 1 → Err; cs is not a valid POP destination (Intel / llvm-mc).
- Doc contract: system.rs:324 "Encode popw (16-bit pop)" — asserted fingerprint b2a11c22
- Seed: (none)
- Formal: ∀ n ≠ 1. encode_pop16(n ops) is Err ∧ encode_pop16([cs]) is Err
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: arity_ne_1_or_cs }
  relation:
    op: holds
    expr: "sut_encode(popw, bad).is_err()"
generators:
  bad: { gen: custom }
expected_error: String
evidence: system.rs:327-328,336 cs; llvm-mc rejects popw %cs
```

## encode_pop16_neg_wrong_width
- Tier: 3
- Rationale: Negative/error — r32/r8 must be rejected for popw (llvm-mc rejects; reg_num aliases them onto r16 encoding).
- Doc contract: system.rs:324 "Encode popw (16-bit pop)" — asserted fingerprint b2a11c22
- Seed: (none)
- Formal: ∀ r ∈ R32 ∪ R8. encode_pop16([r]) is Err
- Test file: src/backend/i686/assembler/encoder/encode_pop16_pbt.rs
- Status: failing
- Counterexample: popw %eax → Ok([0x66, 0x58]); popw %al → Ok([0x66, 0x58])
- Bug report: bug_reports/encode_pop16_wrong_width_gp.md

```property
function: encode_pop16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: r32_or_r8 }
  relation:
    op: holds
    expr: "sut_encode(popw, [Register(r)]).is_err()"
generators:
  r: { gen: oneof, values: ["eax","ecx","al","cl","ah"], type: "&str" }
expected_error: String
evidence: llvm-mc rejects popw %eax/%al; registers.rs:4-15 reg_num aliasing
```
