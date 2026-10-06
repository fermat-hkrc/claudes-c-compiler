# encode_tlbi

## encode_tlbi_diff_valid
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the ARM TLBI grammar for the ops the encoder implements. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree TLBI decoder). Sibling encode_ic/encode_dc/encode_at/encode_sys rejected (same-job gate: named aliases of fixed SYS encodings, different operand grammar). ARM ARM field formula is an independent layout check, not a substitute for llvm-mc agreement. Weaker available: metamorphic Rt isolation / case-ws; ARM layout invariant; negative_error.
- Doc contract: system.rs:486 "TLBI encoding: SYS instruction with fixed fields" — other fingerprint a3610a45
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:1 (llvm-mc differential on SYS helper with mixed Xt grammar)
- Formal: ∀ op ∈ ImplementedTlbi, xt ∈ ValidXt∪{ε}. well_formed(op, xt) ⇒ encode_tlbi([], asm(op, xt)) = Word(w) ∧ llvm_mc("tlbi " + asm(op, xt)) = w ∧ w = arm_sys(op, rt(xt))
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_tlbi
oracle: differential
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: implemented_tlbi_ops, xt: valid_xt_or_omitted }
  relation:
    op: eq
    lhs: encode_tlbi([], asm(op, xt))
    rhs: llvm_mc("tlbi " + asm(op, xt))
generators:
  op: { gen: element, of: implemented_tlbi_ops, type: String }
  xt: { gen: element, of: valid_xt_tokens, type: String }
evidence: README.md:12 llvm-mc -triple=aarch64 -show-encoding; ARM ARM TLBI is SYS CRn=8
```

## encode_tlbi_inv_arm_layout
- Tier: 4
- Rationale: ARM ARM SYS field layout is an independent structural invariant (bits[31:21]=0b11010101000, CRn=8, (op1,CRm,op2,Rt) from the ARM TLBI table). Stronger differential is the sibling property; this pins the architectural packing independently of llvm-mc.
- Doc contract: system.rs:486 "TLBI encoding: SYS instruction with fixed fields" — other fingerprint a3610a45
- Seed: src/backend/arm/assembler/encoder/encode_at_pbt.rs encode_at_inv_arm_layout
- Formal: ∀ op ∈ ImplementedTlbi, t ∈ 0..31. well_formed(op, t) ⇒ let w = encode_tlbi([], asm(op, t)) in w>>21 = 0b11010101000 ∧ ((w>>12)&0xF)=8 ∧ w = arm_sys(op1(op), 8, crm(op), op2(op), rt(op,t))
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_tlbi
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op, t]
  domain: { op: implemented_tlbi_ops, t: u32_0_31 }
  relation:
    op: eq
    lhs: encode_tlbi([], asm(op, t))
    rhs: arm_sys(op1(op), 8, crm(op), op2(op), rt(op, t))
generators:
  op: { gen: element, of: implemented_tlbi_ops, type: String }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM SYS encoding; system.rs:486 SYS instruction with fixed fields
```

## encode_tlbi_meta_rt_isolation
- Tier: 4
- Rationale: Metamorphic: encodings of the same Xt-required op differ only in Rt bits[4:0]; no-Xt ops always encode Rt=31. Stronger differential is the sibling; this isolates the Rt patch (system.rs:537).
- Doc contract: system.rs:536 "Replace Rt field (bits 4:0)" — asserted fingerprint 4485e9a5
- Seed: src/backend/arm/assembler/encoder/encode_at_pbt.rs encode_at_meta_rt_isolation
- Formal: ∀ op ∈ XtRequired, t ∈ 0..31. encode(op, xt) ⊕ encode(op, x0) = t ∧ (encode(op, xt) & ~0x1F) = (encode(op, x0) & ~0x1F). ∀ op ∈ NoXt. encode(op) & 0x1F = 31
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_tlbi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op, t]
  domain: { op: xt_required_ops, t: u32_0_31 }
  relation:
    op: eq
    lhs: encode_tlbi([], op + ", x" + t) XOR encode_tlbi([], op + ", x0")
    rhs: t
generators:
  op: { gen: element, of: xt_required_ops, type: String }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: system.rs:536 Replace Rt field (bits 4:0)
```

## encode_tlbi_meta_case_ws
- Tier: 4
- Rationale: Metamorphic invariance: ASCII case-fold and surrounding space/tab are behavior-preserving (trim + to_lowercase on the op; parse_reg_num lowercases). Documented by the body, independently required by GNU-style assembly.
- Doc contract: system.rs:479 (trim + to_lowercase on op_name) — other fingerprint a3610a45
- Seed: src/backend/arm/assembler/encoder/encode_at_pbt.rs encode_at_meta_case_ws
- Formal: ∀ raw ∈ ValidTlbiWithCaseWs. encode_tlbi([], raw) = encode_tlbi([], canonical(raw))
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_tlbi
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: valid_tlbi_case_ws }
  relation:
    op: eq
    lhs: encode_tlbi([], raw)
    rhs: encode_tlbi([], canonical(raw))
generators:
  raw: { gen: string, type: String }
evidence: system.rs:479 op_name = parts[0].trim().to_lowercase(); README.md:12 GNU-style assembly
```

## encode_tlbi_diff_arm_ops
- Tier: 5
- Rationale: Differential vs llvm-mc on ARM TLBI ops the default CPU accepts that the SUT match table omits (alle2, alle3, alle3is, vae3, vae3is, vale3, vale3is). encoder/mod.rs:4 admits a codegen subset — a documented limitation on accepted assembler input, not a domain restriction (README.md:12 gas-compat and README.md:239 list tlbi). Keep these ops in the generator.
- Doc contract: encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen." — limitation fingerprint 323827bd
- Seed: src/backend/arm/assembler/encoder/encode_at_pbt.rs encode_at_diff_arm_ops
- Formal: ∀ op ∈ ArmDefaultTlbi, xt ∈ ValidXt∪{ε}. well_formed(op, xt) ⇒ encode_tlbi([], asm(op, xt)) = Word(llvm_mc("tlbi " + asm(op, xt)))
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: failing
- Counterexample: encode_tlbi([], "vae3is, x0") -> Err("unsupported tlbi operation: vae3is"); llvm-mc accepts tlbi vae3is, x0. Also alle2, alle3, alle3is, vae3, vale3, vale3is.
- Bug report: bug_reports/encode_tlbi_unimplemented_arm_ops.md

```property
function: encoder.encode_tlbi
oracle: differential
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: arm_default_tlbi_ops, xt: valid_xt_or_omitted }
  relation:
    op: eq
    lhs: encode_tlbi([], asm(op, xt))
    rhs: llvm_mc("tlbi " + asm(op, xt))
generators:
  op: { gen: element, of: arm_default_tlbi_ops, type: String }
  xt: { gen: element, of: valid_xt_tokens, type: String }
evidence: README.md:12 gas-compat; README.md:239 lists tlbi; llvm-mc accepts alle2/alle3/vae3 on the default CPU
```

## encode_tlbi_neg_missing_xt
- Tier: 3
- Rationale: Negative/error contract: ARM/llvm-mc require Xt for VA*/ASIDE*/IPAS2*/R* ops ("specified tlbi op requires a register"). The body defaults missing Xt to Rt=31 rather than declaring it invalid — that default is a producing statement, not a domain restriction. Keep missing Xt in the generator.
- Doc contract: system.rs:486 "TLBI encoding: SYS instruction with fixed fields" — other fingerprint a3610a45
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs IVAU missing Xt; encode_at_pbt.rs encode_at_neg_missing_reg
- Formal: ∀ op ∈ XtRequired. llvm_mc("tlbi " + op) = Err ⇒ encode_tlbi([], op) = Err
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: failing
- Counterexample: encode_tlbi([], "vale1is") -> Word(0xd50883bf) (Rt=XZR) instead of Err; llvm-mc: specified tlbi op requires a register
- Bug report: bug_reports/encode_tlbi_missing_xt.md

```property
function: encoder.encode_tlbi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: xt_required_ops }
  relation:
    op: throws
    lhs: encode_tlbi([], op)
    rhs: Err
generators:
  op: { gen: element, of: xt_required_ops, type: String }
expected_error: String
evidence: llvm-mc "specified tlbi op requires a register"; ARM ARM TLBI VA*/ASIDE*/IPAS2* take Xt
```

## encode_tlbi_neg_extra_xt
- Tier: 3
- Rationale: Negative/error contract: ARM/llvm-mc reject a register on VMALLE*/ALLE*/VMALLS12E1* ("specified tlbi op does not use a register"). The SUT still parses Rt and patches bits[4:0]. Extra Xt stays in the generator.
- Doc contract: system.rs:486 "TLBI encoding: SYS instruction with fixed fields" — other fingerprint a3610a45
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs extra Xt on IALLU*
- Formal: ∀ op ∈ NoXt, xt ∈ ValidXt. llvm_mc("tlbi " + op + ", " + xt) = Err ⇒ encode_tlbi([], op + ", " + xt) = Err
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: failing
- Counterexample: encode_tlbi([], "vmalle1is, x0") -> Word(0xd5088300) instead of Err; llvm-mc: specified tlbi op does not use a register
- Bug report: bug_reports/encode_tlbi_extra_xt.md

```property
function: encoder.encode_tlbi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: no_xt_ops, xt: valid_xt_tokens }
  relation:
    op: throws
    lhs: encode_tlbi([], op + ", " + xt)
    rhs: Err
generators:
  op: { gen: element, of: no_xt_ops, type: String }
  xt: { gen: element, of: valid_xt_tokens, type: String }
expected_error: String
evidence: llvm-mc "specified tlbi op does not use a register"; ARM ARM VMALLE*/ALLE*/VMALLS12E1* take no Xt
```

## encode_tlbi_neg_wrong_reg_class
- Tier: 3
- Rationale: Negative/error contract: llvm-mc/gas reject W/SP/WSP/WZR/SIMD as TLBI Xt ("invalid operand for instruction"). parse_reg_num accepts those prefixes; the encoder does not check is_64bit_reg. Keep the wrong class in the generator.
- Doc contract: system.rs:486 "TLBI encoding: SYS instruction with fixed fields" — other fingerprint a3610a45
- Seed: src/backend/arm/assembler/encoder/encode_at_pbt.rs encode_at_neg_wrong_reg_class
- Formal: ∀ op ∈ XtRequired, bad ∈ WrongRegClass. llvm_mc("tlbi " + op + ", " + bad) = Err ⇒ encode_tlbi([], op + ", " + bad) = Err
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: failing
- Counterexample: encode_tlbi([], "vale1is, w0") -> Word(0xd50883a0) (same as x0) instead of Err; llvm-mc: invalid operand for instruction
- Bug report: bug_reports/encode_tlbi_wrong_reg_class.md

```property
function: encoder.encode_tlbi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, bad]
  domain: { op: xt_required_ops, bad: wrong_reg_class }
  relation:
    op: throws
    lhs: encode_tlbi([], op + ", " + bad)
    rhs: Err
generators:
  op: { gen: element, of: xt_required_ops, type: String }
  bad: { gen: element, of: wrong_reg_class, type: String }
expected_error: String
evidence: llvm-mc "invalid operand for instruction"; ARM ARM TLBI Xt is a 64-bit GPR / XZR / LR
```

## encode_tlbi_neg_invalid_reg
- Tier: 3
- Rationale: Sweep — parse_reg_num None path (system.rs:482) was unexercised. Malformed Xt (x32, empty, #0, foo) that llvm-mc rejects must return Err containing "invalid register". ARM default-CPU unimplemented ops are not in this domain (covered by encode_tlbi_diff_arm_ops).
- Doc contract: system.rs:482 "tlbi: invalid register" — asserted fingerprint (error string)
- Seed: src/backend/arm/assembler/encoder/encode_at_pbt.rs encode_at_neg_invalid_reg
- Formal: ∀ op ∈ XtRequired, xt ∈ {x32, empty, #0, foo, x}. llvm_mc("tlbi " + op + ", " + xt) = Err ⇒ encode_tlbi([], op + ", " + xt) = Err ∧ ("invalid register" ∈ err ∨ "unsupported tlbi operation" ∈ err)
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_tlbi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: xt_required_ops, xt: malformed_xt }
  relation:
    op: throws
    lhs: encode_tlbi([], op + ", " + xt)
    rhs: invalid_register
generators:
  op: { gen: element, of: xt_required_ops, type: String }
  xt: { gen: element, of: malformed_xt, type: String }
expected_error: String
evidence: system.rs:482 tlbi: invalid register; llvm-mc expected register operand
```

## encode_tlbi_neg_unknown_op
- Tier: 3
- Rationale: Sweep — match-arm default (system.rs:534) for names llvm-mc also rejects. Domain is names that are not implemented SUT ops and not ARM default-CPU TLBI ops (those live in encode_tlbi_diff_arm_ops).
- Doc contract: system.rs:534 "unsupported tlbi operation" — asserted fingerprint (error string)
- Seed: src/backend/arm/assembler/encoder/encode_at_pbt.rs encode_at_neg_unknown_op
- Formal: ∀ s. llvm_mc("tlbi " + s) = Err ⇒ encode_tlbi([], s) = Err ∧ ("unsupported tlbi operation" ∈ err ∨ "invalid register" ∈ err)
- Test file: src/backend/arm/assembler/encoder/encode_tlbi_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_tlbi
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: unknown_tlbi_name }
  relation:
    op: throws
    lhs: encode_tlbi([], s)
    rhs: unsupported_or_invalid
generators:
  s: { gen: string, type: String }
expected_error: String
evidence: system.rs:534 unsupported tlbi operation; llvm-mc invalid operand for TLBI instruction
```
