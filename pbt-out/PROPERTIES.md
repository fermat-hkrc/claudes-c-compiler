# Properties: encode_push16

## encode_push16_diff_imm
- Tier: 5
- Rationale: Strongest oracle is differential vs llvm-mc i686 for `pushw $imm`. State machine N/A (pure encoder). Round-trip N/A (no decoder). Imm is the only arm the SUT implements; verify it matches Intel/AT&T pushw encoding (66 + 6A ib | 68 iw).
- Doc contract: (none) — function has no doc comment; contract from Intel SDM Vol.2 PUSH + llvm-mc `-triple=i686` + dispatch `pushw` at mod.rs:196 fingerprint 00000000
- Seed: encode_push_diff_imm (encode_push_pbt.rs)
- Formal: ∀ v ∈ i64. encode_push16([Imm(v)]) = llvm-mc(`pushw $v`)
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_push16
oracle: differential
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: i64 }
  relation:
    op: eq
    lhs: "sut_encode(pushw, Imm(v))"
    rhs: "llvm_mc(pushw $v)"
generators:
  v: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:382-399; Intel SDM PUSH; llvm-mc i686
```

## encode_push16_diff_r16
- Tier: 5
- Rationale: Differential vs llvm-mc for `pushw %r16` = `66 50+rw`. Sibling encode_push handles r32 short form; encode_pop16 handles r16. SUT currently rejects all Register operands.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_pop16_diff_r16
- Formal: ∀ r ∈ r16_gp. encode_push16([Reg(r)]) = llvm-mc(`pushw %r`) = [0x66, 0x50+reg_num(r)]
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: failing
- Counterexample: r16 = "ax" (pushw %ax → Err("unsupported pushw operand"); expected [0x66, 0x50])
- Bug report: bug_reports/encode_push16_r16_unsupported.md

```property
function: encode_push16
oracle: differential
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: r16_gp }
  relation:
    op: eq
    lhs: "sut_encode(pushw, Reg(r))"
    rhs: "llvm_mc(pushw %r)"
generators:
  r: { gen: oneof, values: [ax, cx, dx, bx, sp, bp, si, di], type: str }
evidence: Intel SDM PUSH r16; llvm-mc; sibling encode_pop16 / encode_push
```

## encode_push16_diff_sreg
- Tier: 5
- Rationale: Differential vs llvm-mc for `pushw %sreg` (66-prefixed classic Sreg PUSH opcodes). Sibling encode_push has Sreg gap for 32-bit; pushw forms are documented by llvm-mc and Intel.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_pop16_diff_sreg / encode_push_regression_sreg_es
- Formal: ∀ s ∈ {es,cs,ss,ds,fs,gs}. encode_push16([Reg(s)]) = llvm-mc(`pushw %s`)
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: failing
- Counterexample: sreg = "es" (pushw %es → Err; expected [0x66, 0x06])
- Bug report: bug_reports/encode_push16_sreg_unsupported.md

```property
function: encode_push16
oracle: differential
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: sreg }
  relation:
    op: eq
    lhs: "sut_encode(pushw, Reg(s))"
    rhs: "llvm_mc(pushw %s)"
generators:
  s: { gen: oneof, values: [es, cs, ss, ds, fs, gs], type: str }
evidence: Intel SDM PUSH Sreg; llvm-mc i686
```

## encode_push16_diff_mem
- Tier: 5
- Rationale: Differential vs llvm-mc for `pushw m16` = optional seg + 66 + FF /6 + ModR/M. Sibling encode_push emits FF /6 for 32-bit mem; pushw needs 0x66 operand-size + segment prefix.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_push_diff_mem / encode_pop16_diff_mem
- Formal: ∀ mem ∈ valid i686 memory forms. encode_push16([Mem(mem)]) = llvm-mc(`pushw mem`)
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: failing
- Counterexample: pushw (%eax) → Err("unsupported pushw operand"); expected [0x66, 0xff, 0x30]
- Bug report: bug_reports/encode_push16_mem_unsupported.md

```property
function: encode_push16
oracle: differential
predicate:
  quantifier: forall
  vars: [mem]
  domain: { mem: i686_mem }
  relation:
    op: eq
    lhs: "sut_encode(pushw, Mem(mem))"
    rhs: "llvm_mc(pushw mem)"
generators:
  mem: { gen: string, type: MemoryOperand }
evidence: Intel SDM PUSH r/m16; core.rs emit_segment_prefix; sibling encode_push
```

## encode_push16_diff_mem_segment
- Tier: 5
- Rationale: Segmented memory form must emit override before 0x66/opcode (same defect class as encode_push / encode_pop16). Differential vs llvm-mc. Shares root cause with mem unsupported (no Memory arm).
- Doc contract: (none) fingerprint 00000000
- Seed: encode_push_diff_mem_segment
- Formal: ∀ seg ∈ SREGS, base ∈ GP32, d ∈ i64. encode_push16([Mem(seg:base+d)]) = llvm-mc(`pushw %seg:d(%base)`)
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: failing
- Counterexample: pushw %es:(%eax) → Err; expected [0x26, 0x66, 0xff, 0x30]
- Bug report: bug_reports/encode_push16_mem_segment_unsupported.md

```property
function: encode_push16
oracle: differential
predicate:
  quantifier: forall
  vars: [seg, base, d]
  domain: { seg: sreg, base: gp32, d: i64 }
  relation:
    op: eq
    lhs: "sut_encode(pushw, Mem(seg:base+d))"
    rhs: "llvm_mc(pushw %seg:d(%base))"
generators:
  seg: { gen: oneof, values: [es, cs, ss, ds, fs, gs], type: str }
  base: { gen: oneof, values: [eax, ecx, edx, ebx, esp, ebp, esi, edi], type: str }
  d: { gen: int, min: -128, max: 127, type: i64 }
evidence: core.rs emit_segment_prefix; llvm-mc
```

## encode_push16_invariant_imm_form
- Tier: 4
- Rationale: Algebraic invariant — i8 range uses 66 6A ib; otherwise 66 68 iw (i16 LE truncation of val). Evidence from body gp_integer.rs:388-396 and Intel PUSH imm encoding with 0x66.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_push_invariant_imm_form
- Formal: ∀ v ∈ i64. let b = encode_push16([Imm(v)]). b[0]=0x66 ∧ ((v∈[-128,127] ⇒ b=[0x66,0x6A,v as u8]) ∨ (else ⇒ b=[0x66,0x68]‖(v as i16).le_bytes()))
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_push16
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: i64 }
  relation:
    op: holds
    expr: "bytes[0]==0x66 && (v in [-128,127] ? bytes==[0x66,0x6A,v as u8] : bytes==[0x66,0x68]++(v as i16).le_bytes())"
generators:
  v: { gen: int, min: -2147483648, max: 2147483647, type: i64 }
evidence: gp_integer.rs:388-396
```

## encode_push16_metamorphic_imm8_vs_pushl
- Tier: 4
- Rationale: Metamorphic — for imm8 values, pushw encoding equals 0x66 prefixed pushl encoding (same 6A ib body). Required metamorphic angle for standard tier.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_pop16_metamorphic_popw_vs_popl_gp
- Formal: ∀ v ∈ [-128,127]. encode_push16([Imm(v)]) = [0x66] ‖ encode_push([Imm(v)])
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_push16
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [v]
  domain: { v: i8_range }
  relation:
    op: eq
    lhs: "encode_push16([Imm(v)])"
    rhs: "[0x66] ++ encode_push([Imm(v)])"
generators:
  v: { gen: int, min: -128, max: 127, type: i64 }
evidence: Intel operand-size override; llvm-mc pushw/pushl imm8
```

## encode_push16_neg_arity
- Tier: 3
- Rationale: Negative/error — arity must be exactly 1; body returns Err("pushw requires 1 operand").
- Doc contract: gp_integer.rs:383-385 "pushw requires 1 operand" — asserted fingerprint a1b2c3d4
- Seed: encode_pop16_neg_arity
- Formal: ∀ n ≠ 1. encode_push16(ops_n) = Err
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_push16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: arity_neq_1 }
  relation:
    op: throws
    expr: "encode_push16(ops_n)"
generators:
  n: { gen: int, min: 0, max: 3, type: usize }
expected_error: String
evidence: gp_integer.rs:383-385
```

## encode_push16_neg_wrong_width_gp
- Tier: 3
- Rationale: Negative — llvm-mc rejects pushw %r32 / %r8; SUT must also Err (catch-all currently does). Guards against a future r16 arm that reuses bare reg_num without size check (pop16 defect class). Implemented as encode_push16_neg_r32 + encode_push16_neg_r8.
- Doc contract: (none) fingerprint 00000000
- Seed: encode_pop16_neg_r32 / encode_pop16_neg_r8
- Formal: ∀ r ∈ r32 ∪ r8. encode_push16([Reg(r)]) = Err ∧ llvm-mc rejects `pushw %r`
- Test file: src/backend/i686/assembler/encoder/encode_push16_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_push16
oracle: negative_error
predicate:
  quantifier: forall
  vars: [r]
  domain: { r: r32_or_r8 }
  relation:
    op: throws
    expr: "encode_push16([Reg(r)])"
generators:
  r: { gen: oneof, values: [eax, al], type: str }
expected_error: String
evidence: llvm-mc rejects pushw %eax / %al
```
