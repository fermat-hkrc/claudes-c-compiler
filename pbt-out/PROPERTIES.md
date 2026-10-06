# Properties: encode_ic

## encode_ic_diff_valid
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the valid IC domain. README.md:12 claims GNU-gas-compatible assembly; encoder/mod.rs:3 claims 32-bit AArch64 words; encode_instruction at encoder/mod.rs:983 routes `"ic"` with raw_operands passed through. ARM ARM IC is SYS: IALLUIS = SYS #0, C7, C1, #0; IALLU = SYS #0, C7, C5, #0; IVAU = SYS #3, C7, C5, #1, Xt. llvm-mc and gas accept those three (ASCII case-insensitive) with Xt restricted to 64-bit GPR / XZR / LR. Stronger rejected: State machine (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree IC decoder). Sibling encode_dc / encode_tlbi / encode_at / encode_sys rejected by same-job gate (different SYS encodings and operand grammars). Weaker available: algebraic.metamorphic, algebraic.invariant, negative_error.
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_bti_pbt.rs:269 (llvm-mc differential over the valid domain)
- Formal: ∀ op ∈ {ialluis, iallu}. ∀ case ∈ ASCII-case-fold(op). ∀ pad ∈ {ε, space, tab}*. encode_ic(pad · case · pad) = Word(llvm-mc("ic " · case)) ∧ llvm-mc("ic " · op) = ARM_SYS(op, Rt=31). ∀ t ∈ 0..31 ∪ {xzr, lr}. ∀ case_op, case_reg. encode_ic(pad · case_fold("ivau") · pad · "," · pad · case_fold(xt(t)) · pad) = Word(llvm-mc("ic ivau, " · xt(t))) ∧ that word = ARM_SYS(op1=3, CRn=7, CRm=5, op2=1, Rt=num(t))
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_ic
oracle: differential
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: ialluis|iallu with ASCII case and surrounding whitespace, or "ivau, Xt" for Xt in x0..x30|xzr|x31|lr with ASCII case and surrounding whitespace }
  relation:
    op: eq
    lhs: encode_ic(raw)
    rhs: Word(llvm_mc("ic " + raw))
generators:
  raw: { gen: string, type: String }
evidence: src/backend/arm/assembler/README.md:12; encoder/mod.rs:3; encoder/mod.rs:983; ARM ARM IC as SYS #op1, C7, Cm, #op2; llvm-mc -triple=aarch64 -show-encoding
```

## encode_ic_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM SYS field layout (not the SUT match table). bits[31:21]=0b11010101000, L=0, IALLUIS (op1=0,CRn=7,CRm=1,op2=0,Rt=31), IALLU (op1=0,CRn=7,CRm=5,op2=0,Rt=31), IVAU (op1=3,CRn=7,CRm=5,op2=1,Rt=t). Independent formula word = 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt. Stronger rejected: State machine (no lifecycle); Differential is the primary oracle (this is a field-level check that does not need llvm-mc); Round-trip (no decoder).
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_bti_pbt.rs:279 (ARM layout invariant)
- Formal: ∀ op ∈ {ialluis, iallu}. let w = encode_ic(op). Word(w) ⇒ w = ARM_SYS(op, 31) ∧ (w >> 21) = 0b11010101000 ∧ (w & 0x1F) = 31. ∀ t ∈ 0..31. let w = encode_ic("ivau, x"·t). Word(w) ⇒ w = ARM_SYS(3,7,5,1,t) ∧ ((w >> 16) & 7) = 3 ∧ ((w >> 12) & 0xF) = 7 ∧ ((w >> 8) & 0xF) = 5 ∧ ((w >> 5) & 7) = 1 ∧ (w & 0x1F) = t
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_ic
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op, rt]
  domain: { op: {ialluis, iallu, ivau}, rt: 0..31 }
  relation:
    op: eq
    lhs: encode_ic(ic_raw(op, rt))
    rhs: Word(arm_sys(op, rt))
generators:
  op: { gen: oneof, of: ["ialluis", "iallu", "ivau"], type: String }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM IC encoding SYS op1/CRn/CRm/op2/Rt; ARM ARM SYS 0xD5080000 template
```

## encode_ic_meta_rt_isolation
- Tier: 4
- Rationale: Metamorphic from ARM ARM: IVAU encodings differ only in Rt bits[4:0]; IALLUIS and IALLU differ only in CRm (1 vs 5) with Rt fixed at 31. encode("ivau, xt") XOR encode("ivau, x0") = t; encode("iallu") XOR encode("ialluis") = (5 XOR 1) << 8. Independent of llvm-mc. Stronger rejected: State machine; Round-trip (no decoder); Differential already covers value agreement.
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_bti_pbt.rs:294 (flag-bit independence)
- Formal: ∀ t ∈ 0..31. (encode_ic("ivau, x"·t) ⊕ encode_ic("ivau, x0") = t) ∧ ((encode_ic("ivau, x"·t) & !0x1F) = (encode_ic("ivau, x0") & !0x1F)). (encode_ic("iallu") ⊕ encode_ic("ialluis") = 0x400) ∧ (encode_ic("ialluis") & 0x1F = 31) ∧ (encode_ic("iallu") & 0x1F = 31)
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_ic
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [t]
  domain: { t: 0..31 }
  relation:
    op: eq
    lhs: encode_ic("ivau, x" + t) XOR encode_ic("ivau, x0")
    rhs: t
generators:
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM IC IVAU Rt field bits[4:0]; IALLUIS CRm=1 vs IALLU CRm=5
```

## encode_ic_meta_case_ws
- Tier: 4
- Rationale: Metamorphic: ASCII case-fold and surrounding space/tab on a valid IC operand string are behavior-preserving (parser stores raw_operands unlowercased; encode_ic trims and lowercases). Stronger rejected: State machine; Round-trip; Differential already covers value agreement with llvm-mc (this isolates the SUT's own case/ws invariance without spawning llvm-mc per case).
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_bti_pbt.rs:312 (case/whitespace invariance)
- Formal: ∀ raw ∈ valid_ic_raw. encode_ic(raw) = encode_ic(canonical(raw)) where canonical = trim ∘ ASCII-lowercase of op and Xt with a single comma separator
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_ic
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: valid IC operand with ASCII case and surrounding whitespace }
  relation:
    op: eq
    lhs: encode_ic(raw)
    rhs: encode_ic(canonical(raw))
generators:
  raw: { gen: string, type: String }
evidence: system.rs:403 op_name = parts[0].trim().to_lowercase(); parser.rs:1746 raw_operands stored unlowercased
```

## encode_ic_neg_unknown_op
- Tier: 3
- Rationale: Negative/error contract: llvm-mc and gas reject unknown IC operation names ("invalid operand for IC instruction" / "unknown or missing operation name"). encode_ic's `_` arm returns Err("unsupported ic operation: {}"). Empty operand string, near-miss names, and space-separated extras that become the op token are in the documented-invalid domain. Stronger rejected: State machine; Round-trip; Differential on the valid domain is a different property.
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_bti_pbt.rs:321 (unknown target)
- Formal: ∀ s ∈ UnknownIcOps (empty, near-miss names, garbage tokens; llvm-mc("ic " · s) = Err). encode_ic(s) = Err ∧ (error contains "unsupported ic operation" ∨ error contains "invalid register")
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_ic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: string that is not a valid IC op after trim+casefold, including empty and near-miss names }
  relation:
    op: throws
    lhs: encode_ic(s)
    rhs: unsupported ic operation | invalid register
generators:
  s: { gen: string, type: String }
expected_error: String
evidence: system.rs:424 unsupported ic operation; llvm-mc "invalid operand for IC instruction"; gas "unknown or missing operation name"
```

## encode_ic_neg_iallu_with_reg
- Tier: 3
- Rationale: Negative/error contract: ARM syntax for IALLUIS/IALLU has no Xt. llvm-mc rejects `ic ialluis, xN` / `ic iallu, xN` ("specified ic op does not use a register"); gas rejects ("extraneous register at operand 2"). The SUT match arms do not declare a register out of domain — they still patch Rt — so the input stays in the generator. Stronger rejected: State machine; Round-trip; Differential on the valid (no-Xt) domain is a different property.
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_bti_pbt.rs:340 (extra operand)
- Formal: ∀ op ∈ {ialluis, iallu}. ∀ t ∈ 0..31 ∪ {xzr, lr, sp}. llvm-mc("ic " · op · ", " · xt(t)) = Err ⇒ encode_ic(op · ", " · xt(t)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: failing
- Counterexample: encode_ic("ialluis, x0")
- Bug report: bug_reports/encode_ic_iallu_with_reg.md

```property
function: encoder.system.encode_ic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op, xt]
  domain: { op: {ialluis, iallu}, xt: x0..x30|xzr|lr|sp }
  relation:
    op: throws
    lhs: encode_ic(op + ", " + xt)
    rhs: error
generators:
  op: { gen: oneof, of: ["ialluis", "iallu"], type: String }
  xt: { gen: string, type: String }
expected_error: String
evidence: llvm-mc "specified ic op does not use a register"; gas "extraneous register at operand 2"; ARM ARM IC IALLUIS/IALLU have no Xt
```

## encode_ic_neg_ivau_missing_reg
- Tier: 3
- Rationale: Negative/error contract: ARM syntax for IVAU requires Xt. llvm-mc rejects `ic ivau` ("specified ic op requires a register"); gas rejects ("missing register at operand 2"). encode_ic defaults a missing register to Rt=31 (XZR) without a domain-restriction comment, so the input stays in the generator. Stronger rejected: State machine; Round-trip; Differential on IVAU-with-Xt is a different property.
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_bti_pbt.rs:321 (omitted-invalid analogue)
- Formal: ∀ pad ∈ {ε, space, tab}*. ∀ case ∈ ASCII-case-fold("ivau"). llvm-mc("ic " · pad · case · pad) = Err ⇒ encode_ic(pad · case · pad) = Err
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: failing
- Counterexample: encode_ic("ivau")
- Bug report: bug_reports/encode_ic_ivau_missing_reg.md

```property
function: encoder.system.encode_ic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: case/ws variants of "ivau" with no comma/register }
  relation:
    op: throws
    lhs: encode_ic(raw)
    rhs: error
generators:
  raw: { gen: string, type: String }
expected_error: String
evidence: llvm-mc "specified ic op requires a register"; gas "missing register at operand 2"; ARM ARM IC IVAU requires Xt
```

## encode_ic_neg_wrong_reg_class
- Tier: 3
- Rationale: Negative/error contract: IVAU Xt must be a 64-bit integer register (Xn / XZR / LR). llvm-mc rejects W/SP/WZR/WSP/SIMD/FP ("invalid operand for instruction"); gas rejects ("operand mismatch" / "operand 2 must be an integer register"). parse_reg_num accepts W/SP/D/S/Q/V/H/B, and encode_ic has no class check, so those inputs stay in the generator. Stronger rejected: State machine; Round-trip; Differential on the valid X-register domain is a different property.
- Doc contract: (none)
- Seed: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs extra_operand / wrong_kind
- Formal: ∀ bad ∈ {w0..w30, wzr, wsp, sp, d0, s0, q0, v0, h0, b0}. llvm-mc("ic ivau, " · bad) = Err ⇒ encode_ic("ivau, " · bad) = Err
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: failing
- Counterexample: encode_ic("ivau, w0")
- Bug report: bug_reports/encode_ic_wrong_reg_class.md

```property
function: encoder.system.encode_ic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: W-register | SP | WSP | WZR | SIMD/FP register }
  relation:
    op: throws
    lhs: encode_ic("ivau, " + bad)
    rhs: error
generators:
  bad: { gen: string, type: String }
expected_error: String
evidence: llvm-mc "invalid operand for instruction"; gas "operand mismatch" / "must be an integer register"; ARM ARM IC IVAU Xt is 64-bit GPR
```

## encode_ic_neg_invalid_reg
- Tier: 3
- Rationale: Sweep for the documented error path `ic: invalid register '{}'` (system.rs:406) that the first-batch generators did not reliably reach. llvm-mc rejects malformed Xt (x32, empty, extra operands, `#0`). Stronger rejected: State machine; Round-trip; Differential on the valid X-register domain is a different property.
- Doc contract: (none)
- Seed: encode_ic_pbt.rs encode_ic_neg_wrong_reg_class (malformed vs wrong-class)
- Formal: ∀ xt ∈ {x32..x99, ε, "x0, x1", "#0", "foo", "31", "x"}. llvm-mc("ic ivau, " · xt) = Err ⇒ encode_ic("ivau, " · xt) = Err ∧ error contains "invalid register"
- Test file: src/backend/arm/assembler/encoder/encode_ic_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_ic
oracle: negative_error
predicate:
  quantifier: forall
  vars: [xt]
  domain: { xt: x32..x99 | empty | extra-operand | #imm | non-register token }
  relation:
    op: throws
    lhs: encode_ic("ivau, " + xt)
    rhs: invalid register
generators:
  xt: { gen: string, type: String }
expected_error: String
evidence: system.rs:406 "ic: invalid register"; llvm-mc "expected register operand" / "unexpected token"
```
