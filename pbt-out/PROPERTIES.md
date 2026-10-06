# Properties: encode_at

## encode_at_diff_valid
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the ARM AT grammar for the four ops the encoder implements. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree AT decoder). Sibling encode_ic/encode_dc/encode_tlbi/encode_sys rejected (same-job gate: named aliases of fixed SYS encodings, different operand grammar). ARM ARM field formula is an independent layout check, not a substitute for llvm-mc agreement. Weaker available: metamorphic Rt isolation / case-ws; ARM layout invariant; negative_error.
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_diff_valid
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ xt ∈ {x0..x30,xzr,x31,lr}, ∀ case/ws variants. encode_at([], raw) = Word(w) ∧ llvm-mc("at "+raw) = w ∧ w = ARM_SYS(0,7,8,op2(op),Rt(xt))
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_at
oracle: differential
predicate:
  quantifier: forall
  vars: [op, xt, raw]
  domain: { op: {s1e1r,s1e1w,s1e0r,s1e0w}, xt: x0..x30|xzr|x31|lr, raw: case_ws(op, xt) }
  relation:
    op: eq
    lhs: encode_at([], raw)
    rhs: llvm_mc("at " + raw)
generators:
  op: { gen: oneof, options: ["s1e1r", "s1e1w", "s1e0r", "s1e0w"] }
  xt: { gen: oneof, options: ["x0..x30", "xzr", "x31", "lr"] }
evidence: README.md:12 gas-compat; ARM ARM AT = SYS #0, C7, C8, #op2, Xt; encoder/mod.rs:998
```

## encode_at_diff_arm_ops
- Tier: 2
- Rationale: Differential vs llvm-mc over the ARM AT ops the reference accepts on the default CPU (including S1E2/S1E3/S12E*). README.md:12 gas-compat is the assembler contract. encoder/mod.rs:4 admits a subset — a documented limitation on inputs the API accepts, not an exclusion. Domain includes the unimplemented names; the property fails there.
- Doc contract: encoder/mod.rs:4 "This covers the subset of instructions emitted by our codegen." — limitation fingerprint 323827bd
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_diff_valid
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w,s1e2r,s1e2w,s1e3r,s1e3w,s12e1r,s12e1w,s12e0r,s12e0w}, ∀ t ∈ 0..31. encode_at([], op+", xt") = llvm-mc("at "+op+", xt")
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: failing
- Counterexample: encode_at(&[], "s1e2r, x0") → Err("unsupported at operation: s1e2r")
- Bug report: bug_reports/encode_at_unimplemented_ops.md

```property
function: encoder.encode_at
oracle: differential
predicate:
  quantifier: forall
  vars: [op, t]
  domain: { op: ARM AT ops llvm-mc accepts, t: 0..31 }
  relation:
    op: eq
    lhs: encode_at([], op+", x"+t)
    rhs: llvm_mc("at "+op+", x"+t)
generators:
  op: { gen: oneof, options: ["s1e1r", "s1e1w", "s1e0r", "s1e0w", "s1e2r", "s1e2w", "s1e3r", "s1e3w", "s12e1r", "s12e1w", "s12e0r", "s12e0w"] }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 gas-compat; llvm-mc accepts s1e2r; encoder/mod.rs:4 subset limitation
```

## encode_at_inv_arm_layout
- Tier: 4
- Rationale: Every AT word must satisfy the ARM SYS field layout (bits[31:21]=0b11010101000, op1=0, CRn=7, CRm=8, op2 in {0,1,2,3}, Rt = t). Stronger differential is the sibling encode_at_diff_valid; this is the independent ARM formula check. State machine / round-trip rejected as above.
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_inv_arm_layout
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ t ∈ 0..31. let w = encode_at([], "op, xt"). w>>21 = 0b11010101000 ∧ (w>>16)&7 = 0 ∧ (w>>12)&0xF = 7 ∧ (w>>8)&0xF = 8 ∧ (w>>5)&7 = op2(op) ∧ w&0x1F = t ∧ w = 0xD5080000 | (8<<8) | (op2<<5) | t
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_at
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op, t]
  domain: { op: {s1e1r,s1e1w,s1e0r,s1e0w}, t: 0..31 }
  body: word_fields(encode_at([], op+", x"+t)) == {hi:0b11010101000, op1:0, crn:7, crm:8, op2:op2(op), rt:t}
generators:
  op: { gen: oneof, options: ["s1e1r", "s1e1w", "s1e0r", "s1e0w"] }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM AT = SYS #0, C7, C8, #op2, Xt; system.rs:435
```

## encode_at_meta_rt_isolation
- Tier: 3
- Rationale: Metamorphic: encodings of the same AT op must differ only in Rt bits[4:0]; the four ops encode distinctly (op2 bits). Stronger differential is encode_at_diff_valid. No true inverse.
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_meta_rt_isolation
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ t ∈ 0..31. encode_at(op, xt) ⊕ encode_at(op, x0) = t ∧ (encode_at(op, xt) & !0x1F) = (encode_at(op, x0) & !0x1F). The four op bases (Rt=0) are pairwise distinct and differ only in op2 bits[7:5].
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_at
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op, t]
  domain: { op: {s1e1r,s1e1w,s1e0r,s1e0w}, t: 0..31 }
  body: (encode_at(op, xt) ^ encode_at(op, x0)) == t
generators:
  op: { gen: oneof, options: ["s1e1r", "s1e1w", "s1e0r", "s1e0w"] }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM AT Rt is bits[4:0]; op2 distinguishes the four ops
```

## encode_at_meta_case_ws
- Tier: 3
- Rationale: ASCII case-fold and surrounding space/tab are behavior-preserving (trim + to_lowercase). Metamorphic transform of a valid encoding. Stronger differential already covers cased inputs vs llvm-mc; this asserts SUT(raw) = SUT(canonical(raw)).
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_meta_case_ws
- Formal: ∀ raw in case/ws variants of valid (op, xt). encode_at([], raw) = encode_at([], canonical(raw))
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_at
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: case_ws(valid_at) }
  relation:
    op: eq
    lhs: encode_at([], raw)
    rhs: encode_at([], canonical(raw))
generators:
  raw: { gen: string }
evidence: system.rs:427-428 trim + to_lowercase
```

## encode_at_neg_unknown_op
- Tier: 4
- Rationale: Names that llvm-mc rejects as an AT operation ("invalid operand for AT instruction") must return Err. Negative-error contract evidenced by llvm-mc/gas rejection of the same strings, not by the SUT match table. Generator is co-filtered by llvm-mc.err so the domain is the reference-rejected set (empty, foo, ialluis, civac, sy, #0, x0, …).
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_neg_unknown_op
- Formal: ∀ s. llvm-mc("at "+s) fails ⇒ encode_at([], s) = Err(e) ∧ ("unsupported at operation" ∈ e ∨ "invalid register" ∈ e)
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_at
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: unknown_at_op_rejected_by_llvm_mc }
  relation:
    op: throws
    expr: encode_at([], s)
expected_error: String
generators:
  s: { gen: string }
evidence: system.rs:441; llvm-mc "invalid operand for AT instruction"
```

## encode_at_neg_missing_reg
- Tier: 4
- Rationale: ARM AT requires Xt. llvm-mc: "specified at op requires a register"; gas: "comma expected between operands at operand 2". The SUT currently defaults missing Xt to Rt=31; that is in the generator (documented domain, not the passing one).
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_neg_ivau_missing_reg
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ case/ws of op with no comma. llvm-mc("at "+raw) fails ∧ encode_at([], raw) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: failing
- Counterexample: encode_at(&[], "s1e1r") → Ok(Word(0xd508781f))
- Bug report: bug_reports/encode_at_missing_xt.md

```property
function: encoder.encode_at
oracle: negative_error
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: case_ws of s1e1r|s1e1w|s1e0r|s1e0w with no comma }
  relation:
    op: throws
    expr: encode_at([], raw)
expected_error: String
generators:
  raw: { gen: string }
evidence: llvm-mc "specified at op requires a register"; ARM ARM AT Xt required
```

## encode_at_neg_wrong_reg_class
- Tier: 4
- Rationale: Xt must be a 64-bit GPR. llvm-mc/gas reject W/SP/SIMD ("invalid operand" / "operand mismatch" / "must be an integer register"). parse_reg_num currently accepts those prefixes; they stay in the generator.
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_neg_wrong_reg_class
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ bad ∈ {w0..w30,wzr,wsp,sp,w31,dN,sN,qN,vN,hN,bN}. llvm-mc("at op, bad") fails ∧ encode_at([], op+", "+bad) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: failing
- Counterexample: encode_at(&[], "s1e1r, w0") → Ok(Word(0xd5087800))
- Bug report: bug_reports/encode_at_wrong_reg_class.md

```property
function: encoder.encode_at
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, bad]
  domain: { op: four AT ops, bad: W/SP/SIMD register token }
  relation:
    op: throws
    expr: encode_at([], op+", "+bad)
expected_error: String
generators:
  op: { gen: oneof, options: ["s1e1r", "s1e1w", "s1e0r", "s1e0w"] }
  bad: { gen: string }
evidence: llvm-mc "invalid operand for instruction"; gas "operand mismatch"
```

## encode_at_neg_extra_operand
- Tier: 4
- Rationale: Extra operands after Xt are rejected by llvm-mc ("unexpected token in argument list") and gas ("unexpected characters following instruction"). splitn(2) leaves the extra text inside the register token, which parse_reg_num should fail.
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_neg_invalid_reg (x0, x1 extra)
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ extra ∈ {x0..x30,xzr,lr,#0,w0,x32}. llvm-mc("at op, x0, extra") fails ∧ encode_at([], op+", x0, "+extra) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_at
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, extra]
  domain: { op: four AT ops, extra: extra trailing operand }
  relation:
    op: throws
    expr: encode_at([], op+", x0, "+extra)
expected_error: String
generators:
  op: { gen: oneof, options: ["s1e1r", "s1e1w", "s1e0r", "s1e0w"] }
  extra: { gen: string }
evidence: llvm-mc "unexpected token in argument list"; gas "unexpected characters following instruction"
```

## encode_at_neg_invalid_reg
- Tier: 4
- Rationale: Sweep — parse_reg_num None arm (system.rs:431) for malformed Xt (x32, empty, foo, #0). llvm-mc rejects ("expected register operand"). Documented error path not reached by extra-operand alone when the second field is a single invalid token.
- Doc contract: system.rs:435 "AT encoding: SYS instruction. Base words from GCC:" — other fingerprint ebf834b6
- Seed: src/backend/arm/assembler/encoder/encode_ic_pbt.rs:encode_ic_neg_invalid_reg
- Formal: ∀ op ∈ {s1e1r,s1e1w,s1e0r,s1e0w}, ∀ xt ∈ {x32, x33, empty, foo, #0, 31, x, x32..x99}. llvm-mc("at op, xt") fails ∧ encode_at([], op+", "+xt) = Err(e) ∧ ("invalid register" ∈ e ∨ "unsupported at operation" ∈ e)
- Test file: src/backend/arm/assembler/encoder/encode_at_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_at
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: four AT ops, xt: malformed Xt token }
  relation:
    op: throws
    expr: encode_at([], op+", "+xt)
expected_error: String
generators:
  op: { gen: oneof, options: ["s1e1r", "s1e1w", "s1e0r", "s1e0w"] }
  xt: { gen: string }
evidence: system.rs:431 parse_reg_num None; llvm-mc "expected register operand"
```
