# Properties: encode_sys

## encode_sys_diff_valid
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the ARM SYS grammar. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SYS decoder). Sibling encode_ic/encode_dc/encode_tlbi/encode_at rejected (same-job gate: named aliases of fixed SYS encodings, different operand grammar). ARM ARM field formula is an independent layout check, not a substitute for llvm-mc agreement. Weaker available: metamorphic Rt isolation / omitted-Xt≡xzr / case-ws; ARM layout invariant; negative_error.
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_ic_pbt.rs: encode_ic_diff_valid / encode_dc_pbt.rs: encode_dc_diff_valid
- Formal: ∀ op1 ∈ [0,7], CRn ∈ [0,15], CRm ∈ [0,15], op2 ∈ [0,7], Xt ∈ {x0..x30, xzr, x31, lr, fp, omitted}, case ∈ ASCII, ws ∈ {space,tab}*. encode_sys(raw) = llvm-mc("sys "+raw) = 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt  where Rt(omitted)=31, Rt(xzr)=Rt(x31)=31, Rt(lr)=30, Rt(fp)=29
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: failing
- Counterexample: encode_sys("#0, c0, c0, #0, fp")
- Bug report: bug_reports/encode_sys_fp_alias.md

```property
function: encoder.system.encode_sys
oracle: differential
predicate:
  quantifier: forall
  vars: [op1, crn, crm, op2, xt]
  domain: { op1: u32[0,7], crn: u32[0,15], crm: u32[0,15], op2: u32[0,7], xt: X64_or_omitted }
  relation:
    op: eq
    lhs: encode_sys(raw(op1,crn,crm,op2,xt))
    rhs: llvm_mc("sys " + raw(op1,crn,crm,op2,xt))
generators:
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  xt: { gen: oneof, args: [{ gen: int, min: 0, max: 31, type: u32 }, { gen: const, value: xzr }, { gen: const, value: lr }, { gen: const, value: fp }, { gen: const, value: omitted }] }
evidence: README.md:11 gas-compat; encoder/mod.rs:997 sys dispatch; ARM ARM SYS 0xD5080000 template; llvm-mc -triple=aarch64 -show-encoding
```

## encode_sys_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM SYS encoding (not the SUT match table): bits[31:21]=0b11010101000, L=0, fields op1/CRn/CRm/op2/Rt extracted from the word equal the generated inputs. Stronger differential is p1; this pins the field layout independently of llvm-mc aliasing SYS to DC/IC.
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_ic_pbt.rs ARM SYS layout property
- Formal: ∀ op1 ∈ [0,7], CRn ∈ [0,15], CRm ∈ [0,15], op2 ∈ [0,7], t ∈ [0,31]. let w = encode_sys("#op1, cCRn, cCRm, #op2, xt"). w>>21 = 0b11010101000 ∧ (w>>16)&7 = op1 ∧ (w>>12)&0xF = CRn ∧ (w>>8)&0xF = CRm ∧ (w>>5)&7 = op2 ∧ w&0x1F = t ∧ w = 0xD5080000|(op1<<16)|(CRn<<12)|(CRm<<8)|(op2<<5)|t
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_sys
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [op1, crn, crm, op2, t]
  domain: { op1: u32[0,7], crn: u32[0,15], crm: u32[0,15], op2: u32[0,7], t: u32[0,31] }
  relation:
    op: eq
    lhs: encode_sys(raw(op1,crn,crm,op2,xt))
    rhs: 0xd5080000 | (op1<<16) | (crn<<12) | (crm<<8) | (op2<<5) | t
generators:
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM SYS encoding 0xD5080000 | (op1<<16) | (CRn<<12) | (CRm<<8) | (op2<<5) | Rt
```

## encode_sys_meta_rt_isolation
- Tier: 4
- Rationale: Metamorphic: encodings of the same (op1,CRn,CRm,op2) differ only in Rt bits[4:0]; omitted Xt encodes as XZR (Rt=31) per ARM optional-Xt and body comment system.rs:464. Stronger differential is p1.
- Doc contract: system.rs:464 "xzr if no register specified" — asserted fingerprint 4572c11f
- Seed: encode_ic_pbt.rs / encode_dc_pbt.rs Rt isolation
- Formal: ∀ op1 ∈ [0,7], CRn ∈ [0,15], CRm ∈ [0,15], op2 ∈ [0,7], t ∈ [0,31]. encode_sys(..., xt) XOR encode_sys(..., x0) = t ∧ encode_sys(..., omitted) = encode_sys(..., xzr) ∧ encode_sys(..., xzr) = encode_sys(..., x31)
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_sys
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op1, crn, crm, op2, t]
  domain: { op1: u32[0,7], crn: u32[0,15], crm: u32[0,15], op2: u32[0,7], t: u32[0,31] }
  relation:
    op: eq
    lhs: encode_sys(raw(op1,crn,crm,op2,xt)) xor encode_sys(raw(op1,crn,crm,op2,x0))
    rhs: t
generators:
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM SYS Rt bits[4:0]; system.rs:464 omitted Xt defaults to xzr
```

## encode_sys_meta_case_ws
- Tier: 4
- Rationale: Metamorphic invariance: ASCII case on Cn/Cm/Xt and surrounding space/tab (and optional `#`) must not change the encoding. gas/llvm-mc accept those variants. Stronger differential is p1.
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_dc_pbt.rs encode_dc_meta_case_ws
- Formal: ∀ valid SYS fields, case-fold and surrounding whitespace pads. encode_sys(padded/cased raw) = encode_sys(canonical raw) (both Ok with equal words, or both Err)
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_sys
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [op1, crn, crm, op2, xt, pads, case]
  domain: { op1: u32[0,7], crn: u32[0,15], crm: u32[0,15], op2: u32[0,7], xt: X64, pads: ws, case: ascii }
  relation:
    op: eq
    lhs: encode_sys(cased_padded(raw))
    rhs: encode_sys(canonical(raw))
generators:
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  t: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:12 gas-compat; llvm-mc accepts SYS case/whitespace; SUT to_lowercase/trim
```

## encode_sys_neg_too_few
- Tier: 4
- Rationale: Negative/error contract: ARM SYS requires four fields (op1, Cn, Cm, op2). llvm-mc/gas reject fewer than 4 operands ("too few operands" / "comma expected"). Body returns Err when parts.len() < 4. Domain includes 0, 1, 2, 3 comma-separated tokens including empty string.
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_dc_pbt.rs encode_dc_neg_missing_xt
- Formal: ∀ s with fewer than 4 comma-separated operands. encode_sys(s) = Err ∧ llvm-mc("sys "+s) fails
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_sys
oracle: negative_error
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: sys_operand_strings with < 4 comma parts }
  relation:
    op: throws
    lhs: encode_sys(raw)
    rhs: String
generators:
  nparts: { gen: int, min: 0, max: 3, type: usize }
expected_error: String
evidence: ARM ARM SYS requires op1,Cn,Cm,op2; llvm-mc "too few operands"; gas "comma expected"; system.rs:451 parts.len() < 4
```

## encode_sys_neg_oob_fields
- Tier: 4
- Rationale: Negative/error contract: ARM/llvm-mc/gas require op1,op2 ∈ [0,7] and Cn,Cm ∈ C0–C15. Bound±1 must be sampled (op1=8, CRn=16, CRm=16, op2=8). SUT currently masks with &7/&0xF rather than rejecting; that is the candidate bug, not a domain restriction (the function's own comments do not declare oob input invalid).
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_svc_pbt.rs oob-imm (same assembler gas-compat contract)
- Formal: ∀ (op1,CRn,CRm,op2,Xt) valid except exactly one field out of range at bound+1 or above. llvm-mc("sys "+raw) fails ∧ encode_sys(raw) = Err
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: failing
- Counterexample: encode_sys("#8, c0, c0, #0, x0")
- Bug report: bug_reports/encode_sys_oob_fields.md

```property
function: encoder.system.encode_sys
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op1, crn, crm, op2, t, which]
  domain: { one of op1 in [8,255], crn in [16,255], crm in [16,255], op2 in [8,255]; others in range; includes bound+1 }
  relation:
    op: throws
    lhs: encode_sys(raw)
    rhs: String
generators:
  op1: { gen: int, min: 8, max: 255, type: u32 }
  crn: { gen: int, min: 16, max: 255, type: u32 }
  crm: { gen: int, min: 16, max: 255, type: u32 }
  op2: { gen: int, min: 8, max: 255, type: u32 }
expected_error: String
evidence: ARM ARM SYS field widths; llvm-mc "immediate must be an integer in range [0, 7]" / "Expected cN operand where 0 <= N <= 15"; gas "immediate value out of range 0 to 7" / "C0 - C15 expected"
```

## encode_sys_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract: llvm-mc/gas reject a sixth operand ("invalid operand" / "unexpected characters following instruction"). Body splits on comma and ignores parts after Xt; extra operands stay in the generator (not declared out of domain by this function).
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_dc_pbt.rs encode_dc_neg_extra_operand
- Formal: ∀ valid SYS fields, extra ∈ {x0..x31, #imm, foo}. llvm-mc("sys "+raw+", "+extra) fails ∧ encode_sys(raw+", "+extra) = Err
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: failing
- Counterexample: encode_sys("#0, c0, c0, #0, x0, x0")
- Bug report: bug_reports/encode_sys_extra_operand.md

```property
function: encoder.system.encode_sys
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op1, crn, crm, op2, t, extra]
  domain: { valid SYS 5-operand raw, extra: extra_token }
  relation:
    op: throws
    lhs: encode_sys(raw + ", " + extra)
    rhs: String
generators:
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  t: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, args: [{ gen: const, value: x1 }, { gen: const, value: "#0" }, { gen: const, value: foo }] }
expected_error: String
evidence: llvm-mc extra operand error; gas "unexpected characters following instruction at operand 5"
```

## encode_sys_neg_wrong_reg_class
- Tier: 4
- Rationale: Negative/error contract: ARM SYS Xt is a 64-bit GPR (XZR/LR). llvm-mc/gas reject W/SP/WZR/WSP/SIMD. parse_reg_num currently accepts those names; the function's comments do not declare them out of domain. Domain is every wrong class llvm-mc rejects, including bound names w0/w31/sp/v0.
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_dc_pbt.rs encode_dc_neg_wrong_reg_class
- Formal: ∀ valid SYS fields, bad ∈ {w0..w31, wzr, wsp, sp, v0, d0, s0, q0, h0, b0}. llvm-mc("sys "+raw+", "+bad) fails ∧ encode_sys(raw+", "+bad) = Err
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: failing
- Counterexample: encode_sys("#0, c0, c0, #0, w0")
- Bug report: bug_reports/encode_sys_wrong_reg_class.md

```property
function: encoder.system.encode_sys
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op1, crn, crm, op2, bad]
  domain: { valid SYS 4-field prefix, bad: W/SP/SIMD token }
  relation:
    op: throws
    lhs: encode_sys(raw + ", " + bad)
    rhs: String
generators:
  op1: { gen: int, min: 0, max: 7, type: u32 }
  crn: { gen: int, min: 0, max: 15, type: u32 }
  crm: { gen: int, min: 0, max: 15, type: u32 }
  op2: { gen: int, min: 0, max: 7, type: u32 }
  bad: { gen: oneof, args: [{ gen: const, value: w0 }, { gen: const, value: sp }, { gen: const, value: v0 }, { gen: const, value: wzr }] }
expected_error: String
evidence: ARM ARM SYS Xt is 64-bit GPR; llvm-mc "invalid operand"; gas "operand mismatch" / "must be an integer register"
```

## encode_sys_neg_invalid_reg
- Tier: 4
- Rationale: Sweep — documented Err arm system.rs:463 "sys: invalid register" when parse_reg_num returns None (x32, foo, empty, #imm). llvm-mc/gas reject those Xt tokens. Distinct from wrong-class (parse_reg_num Some).
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_dc_pbt.rs encode_dc_neg_invalid_reg
- Formal: ∀ xt ∈ {x32, x33, foo, empty, x, #0, 31, x32..x99}. encode_sys("#0, c0, c0, #0, "+xt) = Err containing "invalid register" ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_sys
oracle: negative_error
predicate:
  quantifier: forall
  vars: [xt]
  domain: { xt: malformed Xt token parse_reg_num rejects }
  relation:
    op: throws
    lhs: encode_sys("#0, c0, c0, #0, " + xt)
    rhs: String
generators:
  xt: { gen: oneof, args: [{ gen: const, value: x32 }, { gen: const, value: foo }, { gen: const, value: "" }] }
expected_error: String
evidence: system.rs:463 "sys: invalid register"; llvm-mc invalid operand
```

## encode_sys_neg_non_numeric
- Tier: 4
- Rationale: Sweep — documented Err arms system.rs:454/456/458/460 for non-numeric op1/CRn/CRm/op2 ("sys: invalid op1/CRn/CRm/op2"). llvm-mc/gas reject those fields.
- Doc contract: system.rs:447 "Encode `sys #op1, Cn, Cm, #op2, Xt` instruction." — asserted fingerprint d6e99d55
- Seed: encode_sys parts.parse map_err arms
- Formal: ∀ raw with a non-numeric op1, CRn, CRm, or op2 token. encode_sys(raw) = Err containing "invalid op1" ∨ "invalid CRn" ∨ "invalid CRm" ∨ "invalid op2" ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/encode_sys_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_sys
oracle: negative_error
predicate:
  quantifier: forall
  vars: [raw]
  domain: { raw: SYS string with a non-numeric field }
  relation:
    op: throws
    lhs: encode_sys(raw)
    rhs: String
generators:
  raw: { gen: oneof, args: [{ gen: const, value: "foo, c0, c0, #0, x0" }, { gen: const, value: "#0, xx, c0, #0, x0" }] }
expected_error: String
evidence: system.rs:454-460 parse map_err; llvm-mc field errors
```
