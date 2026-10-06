# Properties: encode_dc

## encode_dc_diff_valid
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc AArch64 assembler on the six implemented DC ops. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree DC decoder). Sibling encode_ic / encode_tlbi / encode_at / encode_sys rejected (same-job gate: different SYS encodings and operand grammars). README.md:12 gas-compat plus ARM ARM SYS encodings in body comments.
- Doc contract: system.rs:591 "DC CIVAC: sys #3, c7, c14, #1, Xt" — asserted fingerprint b25dc9fa
- Seed: encode_ic_pbt.rs:encode_ic_diff_valid
- Formal: ∀ op ∈ {civac,cvac,cvap,cvau,ivac,zva}, ∀ xt ∈ {x0..x30,xzr,x31,lr}, ∀ case/ws variants. encode_dc([Symbol(op), Reg(xt)], raw) = Word(w) ∧ w = llvm-mc("dc op, xt" with +ccpp for cvap) = ARM SYS(op, Rt(xt))
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dc
oracle: differential
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: {civac,cvac,cvap,cvau,ivac,zva}, xt: x0..x30|xzr|x31|lr }
  relation:
    op: eq
    lhs: encode_dc([Symbol(op), Reg(xt)], raw)
    rhs: llvm_mc("dc " + op + ", " + xt)
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
  xt: { gen: oneof, options: ["x0..x30", "xzr", "x31", "lr"] }
evidence: README.md:12 gas-compat; system.rs:591-607 ARM SYS encodings; encoder/mod.rs:983
```

## encode_dc_inv_arm_layout
- Tier: 4
- Rationale: ARM ARM SYS field layout is an independent structural invariant (bits[31:21]=0b11010101000, op1/CRn/CRm/op2/Rt per named op). Stronger differential is the sibling property; this pins the documented SYS formula even if llvm-mc were unavailable.
- Doc contract: system.rs:591 "DC CIVAC: sys #3, c7, c14, #1, Xt" — asserted fingerprint b25dc9fa
- Seed: encode_ic_pbt.rs:encode_ic_inv_arm_layout
- Formal: ∀ op ∈ {civac,cvac,cvap,cvau,ivac,zva}, ∀ t ∈ 0..=31. encode_dc([Symbol(op), Reg(x{t})], raw) = Word(w) ∧ w = 0xD5080000 | (op1<<16) | (7<<12) | (CRm<<8) | (1<<5) | t with (op1,CRm) = (3,14)/(3,10)/(3,12)/(3,11)/(0,6)/(3,4)
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op, t]
  domain: { op: six DC names, t: 0..=31 }
  relation:
    op: eq
    lhs: encode_dc([Symbol(op), Reg("x"+t)], raw)
    rhs: arm_sys(op1(op), 7, crm(op), 1, t)
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: system.rs:591-607; ARM ARM SYS template 0xD5080000
```

## encode_dc_meta_rt_isolation
- Tier: 4
- Rationale: Metamorphic: changing only Xt must change only Rt bits[4:0]; distinct ops must produce distinct base words. Weaker than differential; still an independent algebraic check on the SYS packing.
- Doc contract: system.rs:565 "Check for the operation type in the operands or raw string" — other fingerprint 8360abc6
- Seed: encode_ic_pbt.rs:encode_ic_meta_rt_isolation
- Formal: ∀ op ∈ six names, ∀ t ∈ 0..=31. encode_dc(op, x{t}) XOR encode_dc(op, x0) = t ∧ (encode_dc(op, x{t}) & !0x1F) = (encode_dc(op, x0) & !0x1F). Distinct ops have distinct bases.
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op, t]
  domain: { op: six DC names, t: 0..=31 }
  relation:
    op: eq
    lhs: encode_dc(op, xt) XOR encode_dc(op, x0)
    rhs: t
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM SYS Rt in bits[4:0]; system.rs:588 (word = base | rt)
```

## encode_dc_meta_case_ws
- Tier: 4
- Rationale: Metamorphic invariance: ASCII case-fold and surrounding space/tab on the valid domain are behavior-preserving (trim+to_lowercase on Symbol; parser does not lowercase). README gas-compat: gas/llvm-mc accept mixed case and padding.
- Doc contract: system.rs:565 "Check for the operation type in the operands or raw string" — other fingerprint 8360abc6
- Seed: encode_ic_pbt.rs:encode_ic_meta_case_ws
- Formal: ∀ op ∈ six names, ∀ xt ∈ X-regs, ∀ case/ws variant v of (op, xt). encode_dc(v) = encode_dc(canonical(op, xt))
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: six DC names, xt: X-regs, variant: ascii case + space/tab pad }
  relation:
    op: eq
    lhs: encode_dc(variant)
    rhs: encode_dc(canonical(op, xt))
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
  xt: { gen: oneof, options: ["x0..x30", "xzr", "lr"] }
evidence: system.rs:567 s.to_lowercase(); parser.rs:1734-1746 does not lowercase raw_operands
```

## encode_dc_neg_unknown_op
- Tier: 3
- Rationale: Negative/error contract: llvm-mc/gas reject unknown DC op names (including substring supersets like gzva, civacs). SUT must Err. Domain is names that are not exactly the six implemented ops after trim+casefold; substring matches stay in the domain (contains() is not a documented exact-match contract).
- Doc contract: (none) — encode_dc has no rustdoc declaring unknown names out of domain via substring
- Seed: encode_ic_pbt.rs:encode_ic_neg_unknown_op
- Formal: ∀ name ∉ {civac,cvac,cvap,cvau,ivac,zva} (after trim+casefold of first comma field). llvm-mc("dc name, x0") = Err ⇒ encode_dc([Symbol(name), Reg("x0")], raw) = Err
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: failing
- Counterexample: encode_dc([Symbol("civacs"), Reg("x0")], "civacs, x0") = Ok(Word(0xd50b7e20))
- Bug report: pbt-out/bug_reports/encode_dc_substring_op.md

```property
function: encoder.encode_dc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: DC op names that are not exactly the six implemented ops }
  relation:
    op: holds
    expr: encode_dc([Symbol(name), Reg("x0")], name + ", x0").is_err()
generators:
  name: { gen: string, minLen: 0, maxLen: 16 }
expected_error: unsupported dc variant
evidence: llvm-mc/gas reject unknown DC ops; system.rs:611 Err unsupported dc variant
```

## encode_dc_neg_missing_xt
- Tier: 3
- Rationale: All six implemented DC ops require Xt. llvm-mc and gas reject `dc <op>` with no register. Body does not declare missing Xt out of domain; defaulting Rt to 0 would be silent wrong encoding.
- Doc contract: (none) — encode_dc has no rustdoc declaring missing Xt invalid. gas "comma expected between operands at operand 2".
- Seed: encode_ic_pbt.rs:encode_ic_neg_ivau_missing_reg
- Formal: ∀ op ∈ {civac,cvac,cvap,cvau,ivac,zva}. encode_dc([Symbol(op)], op) = Err
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: failing
- Counterexample: encode_dc([Symbol("civac")], "civac") = Ok(Word(0xd50b7e20))
- Bug report: pbt-out/bug_reports/encode_dc_missing_xt.md

```property
function: encoder.encode_dc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: six DC names }
  relation:
    op: holds
    expr: encode_dc([Symbol(op)], op).is_err()
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
expected_error: Err
evidence: llvm-mc/gas require Xt on every implemented DC op; ARM ARM SYS Xt required
```

## encode_dc_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc/gas reject a third operand (`dc civac, x0, x1`). SUT must Err. Body ignores extras if get(1) is a Reg.
- Doc contract: (none)
- Seed: encode_ic_pbt.rs extra-operand negative
- Formal: ∀ op ∈ six names, ∀ xt ∈ X-regs, ∀ extra. encode_dc([Symbol(op), Reg(xt), extra], raw) = Err
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: failing
- Counterexample: encode_dc([Symbol("civac"), Reg("x0"), Reg("x0")], "civac, x0, x0") = Ok(Word(0xd50b7e20))
- Bug report: pbt-out/bug_reports/encode_dc_extra_operand.md

```property
function: encoder.encode_dc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, xt, extra]
  domain: { op: six DC names, xt: X-regs, extra: additional operand }
  relation:
    op: holds
    expr: encode_dc([Symbol(op), Reg(xt), extra], raw).is_err()
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
  xt: { gen: oneof, options: ["x0..x30", "xzr", "lr"] }
  extra: { gen: oneof, options: ["x0", "x1", "xzr", "#0", "w0"] }
expected_error: Err
evidence: llvm-mc unexpected characters following instruction; gas same
```

## encode_dc_neg_wrong_reg_class
- Tier: 3
- Rationale: llvm-mc/gas require a 64-bit integer GPR. W/SP/WSP/SIMD registers are invalid. parse_reg_num accepts them, so the SUT may silently encode the number; that is the bug class.
- Doc contract: (none)
- Seed: encode_ic_pbt.rs:encode_ic_neg_wrong_reg_class
- Formal: ∀ op ∈ six names, ∀ bad ∈ {w0..w30, wzr, wsp, sp, d/s/q/v/h/b0..31}. encode_dc([Symbol(op), Reg(bad)], raw) = Err
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: failing
- Counterexample: encode_dc([Symbol("civac"), Reg("w0")], "civac, w0") = Ok(Word(0xd50b7e20))
- Bug report: pbt-out/bug_reports/encode_dc_wrong_reg_class.md

```property
function: encoder.encode_dc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, bad]
  domain: { op: six DC names, bad: W/SP/SIMD register names }
  relation:
    op: holds
    expr: encode_dc([Symbol(op), Reg(bad)], raw).is_err()
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
  bad: { gen: oneof, options: ["w0..w30", "wzr", "wsp", "sp", "d0..d31"] }
expected_error: Err
evidence: llvm-mc invalid operand; gas operand mismatch / must be an integer register
```

## encode_dc_neg_invalid_reg
- Tier: 3
- Rationale: Sweep — drive parse_reg_num None (system.rs:573) for Xt names llvm-mc also rejects (x32, empty, #0, foo). Distinct from wrong-class (those parse_reg_num accepts).
- Doc contract: (none)
- Seed: encode_ic_pbt.rs:encode_ic_neg_invalid_reg
- Formal: ∀ op ∈ six names, ∀ xt ∈ {x32..x99, #0, ε, foo, 31, x}. llvm-mc("dc op, xt") = Err ⇒ encode_dc([Symbol(op), Reg(xt)], raw) = Err containing "invalid register" or "unsupported dc variant"
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: six DC names, xt: malformed Xt tokens }
  relation:
    op: holds
    expr: encode_dc([Symbol(op), Reg(xt)], raw).is_err()
generators:
  op: { gen: oneof, options: ["civac", "cvac", "cvap", "cvau", "ivac", "zva"] }
  xt: { gen: oneof, options: ["x32", "x33", "#0", "", "foo", "31", "x"] }
expected_error: invalid register
evidence: system.rs:573 parse_reg_num(name).ok_or("invalid register for dc"); llvm-mc rejects x32
```

## encode_dc_neg_unknown_nonsubstr
- Tier: 3
- Rationale: Sweep — names that llvm-mc rejects and that do not substring-match the six implemented ops must take the final Err arm (system.rs:611). Complements the failing substring property so the documented unsupported-variant path is executed and asserted.
- Doc contract: (none)
- Seed: encode_ic_pbt.rs:encode_ic_neg_unknown_op
- Formal: ∀ name whose trim+casefold is not an exact implemented DC op, is not {cisw,csw,isw}, and does not contain civac|cvac|cvap|cvau|ivac|zva. llvm-mc("dc name, x0") = Err ⇒ encode_dc = Err
- Test file: src/backend/arm/assembler/encoder/encode_dc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: unknown DC ops that do not substring-match implemented names }
  relation:
    op: holds
    expr: encode_dc([Symbol(name), Reg("x0")], name + ", x0").is_err()
generators:
  name: { gen: string, minLen: 0, maxLen: 16 }
expected_error: unsupported dc variant
evidence: system.rs:611 Err unsupported dc variant
```
