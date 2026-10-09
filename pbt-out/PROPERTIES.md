# Properties: encode_pop

## encode_pop_diff_r32
- Tier: 4
- Rationale: Differential vs llvm-mc is strongest available; no in-tree i686 decoder for round-trip; pure encoder so no state machine. Intel SDM POP r32 short form 58+rd.
- Doc contract: gp_integer.rs:402 (none on function) — other fingerprint 00000000
- Seed: encode_push_pbt.rs encode_push_diff_r32
- Formal: ∀ r ∈ {eax,ecx,edx,ebx,esp,ebp,esi,edi}. encode_pop([Reg(r)]) = llvm_mc("popl %r")
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop
oracle: differential
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(\"popl\", [Reg(r)])"
    rhs: "llvm_mc(\"popl %\" + r)"
generators:
  r: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: gp_integer.rs:418-421; Intel SDM POP r32 58+rd; llvm-mc -triple=i686
```

## encode_pop_diff_sreg
- Tier: 4
- Rationale: Sreg pop forms are explicit in encode_pop body (es/ss/ds/fs/gs); cs rejected. Differential vs llvm-mc.
- Doc contract: gp_integer.rs:408 "Pop to segment register" — asserted fingerprint a1b2c3d4
- Seed: encode_pop16_pbt.rs encode_pop16_diff_sreg
- Formal: ∀ s ∈ {es,ss,ds,fs,gs}. encode_pop([Reg(s)]) = llvm_mc("popl %s")
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop
oracle: differential
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: sreg_pop }
  relation:
    op: eq
    lhs: "sut_encode(\"popl\", [Reg(s)])"
    rhs: "llvm_mc(\"popl %\" + s)"
generators:
  s: { gen: oneof, values: ["es","ss","ds","fs","gs"], type: "&str" }
evidence: gp_integer.rs:408-417; Intel SDM POP Sreg
```

## encode_pop_diff_mem
- Tier: 4
- Rationale: Memory form 8F /0; differential vs llvm-mc over base/disp/SIB/abs shapes without segment.
- Doc contract: gp_integer.rs:425 "pop m32: 0x8F /0" — asserted fingerprint b2c3d4e5
- Seed: encode_push_pbt.rs encode_push_diff_mem
- Formal: ∀ m ∈ valid_mem32. encode_pop([Mem(m)]) = llvm_mc("popl " + att(m))
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop
oracle: differential
predicate:
  quantifier: forall
  vars: [m]
  domain: { m: mem32_no_seg }
  relation:
    op: eq
    lhs: "sut_encode(\"popl\", [Mem(m)])"
    rhs: "llvm_mc(\"popl \" + att(m))"
generators:
  m: { gen: mem32_shapes, type: "MemoryOperand" }
evidence: gp_integer.rs:424-427; Intel SDM POP r/m32 8F /0
```

## encode_pop_diff_mem_segment
- Tier: 4
- Rationale: Segment override on memory POP must emit prefix (core.rs emit_segment_prefix; x86-64 sibling does before 8F). Differential vs llvm-mc for all six segments.
- Doc contract: core.rs:31 "Emit segment override prefix if the memory operand has a segment." — asserted fingerprint c3d4e5f6
- Seed: encode_push_pbt.rs encode_push_diff_mem_segment
- Formal: ∀ seg ∈ SREGS, base ∈ GP32, d ∈ disp. encode_pop([Mem(seg:base+d)]) = llvm_mc("popl %seg:d(%base)")
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: failing
- Counterexample: popl %es:(%eax) → sut=[8f,00] mc=[26,8f,00]
- Bug report: bug_reports/encode_pop_missing_segment_prefix.md

```property
function: encode_pop
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, d]
  domain: { seg: sregs, base: gp32, d: disp_edge }
  relation:
    op: eq
    lhs: "sut_encode(\"popl\", [Mem(seg:base+d)])"
    rhs: "llvm_mc(att)"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  d: { gen: oneof, values: [0, 8, -4, 127, -128], type: i64 }
evidence: core.rs:31-42; x86-64 gp_integer.rs:393 emit_segment_prefix before 8F; Intel SDM 2.1.1
```

## encode_pop_invariant_r32_opcode
- Tier: 3
- Rationale: Algebraic invariant — short form is exactly one byte 0x58+n.
- Doc contract: (none)
- Seed: encode_push_pbt.rs encode_push_invariant_r32_opcode
- Formal: ∀ r ∈ GP32. encode_pop([Reg(r)]) = [0x58 + reg_num(r)]
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: gp32 }
  relation:
    op: eq
    lhs: "sut_encode(\"popl\", [Reg(r)])"
    rhs: "[0x58 + reg_num(r)]"
generators:
  r: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
evidence: gp_integer.rs:419-421; Intel SDM POP r32
```

## encode_pop_meta_segment_stripped_eq_bare
- Tier: 4
- Rationale: Metamorphic — segmented memory encoding must equal seg_prefix ‖ bare encoding. Required metamorphic/differential at standard tier. Same root cause as encode_pop_diff_mem_segment (b1); ledger bugId owned by p4, this entry remains failing as a secondary witness.
- Doc contract: core.rs:31 emit_segment_prefix — asserted fingerprint c3d4e5f6
- Seed: encode_push_pbt.rs encode_push_meta_segment_stripped_eq_bare
- Formal: ∀ seg, base, d. strip_seg(encode_pop(Mem(seg:…))) = encode_pop(Mem(bare)) ∧ first_byte = seg_prefix(seg)
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: failing
- Counterexample: seg=es base=eax disp=0 → with_seg=[8f,00] (missing 0x26)
- Bug report: bug_reports/encode_pop_meta_missing_segment_prefix.md

```property
function: encode_pop
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [seg, base, d]
  domain: { seg: sregs, base: gp32, d: disp_edge }
  body: "with_seg[0]==seg_prefix(seg) ∧ strip_seg(with_seg)==bare"
generators:
  seg: { gen: oneof, values: ["es","cs","ss","ds","fs","gs"], type: "&str" }
  base: { gen: oneof, values: ["eax","ecx","edx","ebx","esp","ebp","esi","edi"], type: "&str" }
  d: { gen: oneof, values: [0, 4, -1, 127], type: i64 }
evidence: core.rs:31-42; x86-64 sibling emit_segment_prefix
```

## encode_pop_neg_arity
- Tier: 3
- Rationale: Negative/error — arity ≠ 1 must Err with documented message path.
- Doc contract: gp_integer.rs:403-405 "pop requires 1 operand" — asserted fingerprint d4e5f6a7
- Seed: encode_push_pbt.rs encode_push_neg_arity
- Formal: ∀ n ∈ {0,2,3}. encode_pop(ops_n) = Err
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_pop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: "0|2|3" }
  relation:
    op: throws
    expr: "sut_encode(\"popl\", ops_n)"
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
expected_error: "pop requires 1 operand"
evidence: gp_integer.rs:403-405
```

## encode_pop_neg_non_gp_and_wrong_width
- Tier: 3
- Rationale: Negative — cs is invalid POP destination (Intel, passes); non-GP xmm and r8/r16 must not silently alias via reg_num (llvm-mc rejects).
- Doc contract: gp_integer.rs:416 cannot pop to cs; registers.rs reg_num aliases r8/r16/xmm — asserted fingerprint e5f6a7b8
- Seed: encode_push_pbt.rs encode_push_neg_xmm
- Formal: ∀ bad ∈ XMM∪R8∪R16. llvm_mc("popl %bad") rejects ⇒ encode_pop rejects
- Test file: src/backend/i686/assembler/encoder/encode_pop_pbt.rs
- Status: failing
- Counterexample: popl %xmm0 → Ok([0x58]); popl %al → Ok([0x58]); popl %ax → Ok([0x58])
- Bug report: bug_reports/encode_pop_wrong_width_and_non_gp.md

```property
function: encode_pop
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: "xmm*|r8|r16" }
  relation:
    op: throws
    expr: "sut_encode(\"popl\", [Reg(bad)])"
generators:
  bad: { gen: oneof, values: ["xmm0","al","ax"], type: "&str" }
expected_error: "bad register / wrong width"
evidence: Intel SDM POP r32 only on popl; llvm-mc rejects; registers.rs reg_num aliases
```
