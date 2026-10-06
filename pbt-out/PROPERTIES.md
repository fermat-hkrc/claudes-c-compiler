# Properties: encode_bti

## encode_bti_diff_targets
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler) on the valid BTI target domain. README.md:12 claims GNU-gas-compatible assembly; encoder/mod.rs:3 claims 32-bit AArch64 words; encode_instruction at encoder/mod.rs:972 routes `"bti"` with raw_operands passed through. ARM ARM BTI is HINT with CRm=0b0100, op2=0bxx0: omitted/c/j/jc → HINT #32/#34/#36/#38. llvm-mc and gas accept those four (ASCII case-insensitive). Stronger rejected: State machine (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree BTI decoder). Sibling encode_hint / NOP/YIELD/WFE/WFI/SEV/SEVL rejected by same-job gate (different mnemonics, different operand grammar). Weaker available: algebraic.metamorphic, algebraic.invariant, negative_error.
- Doc contract: src/backend/arm/assembler/encoder/system.rs:541 "Encode HINT #imm (system hint instruction)" — other fingerprint 87550030
- Seed: src/backend/arm/assembler/encoder/encode_hint_pbt.rs:229 (llvm-mc differential over the valid domain)
- Formal: ∀ t ∈ {ε, c, j, jc}. ∀ case ∈ ASCII-case-fold(t). ∀ pad ∈ {ε, space, tab}*. encode_bti(pad · case · pad) = Word(llvm-mc("bti" · opt(case))) ∧ llvm-mc("bti" · opt(t)) = 0xD503241F | (j(t) << 7) | (c(t) << 6)
- Test file: src/backend/arm/assembler/encoder/encode_bti_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_bti
oracle: differential
predicate:
  quantifier: forall
  vars: [target]
  domain: { target: { "", "c", "j", "jc" } with ASCII case and surrounding whitespace }
  relation:
    op: eq
    lhs: encode_bti(target)
    rhs: Word(llvm_mc("bti" + opt(target)))
generators:
  target: { gen: oneof, of: ["", "c", "j", "jc"], type: String }
evidence: src/backend/arm/assembler/README.md:12; encoder/mod.rs:3; encoder/mod.rs:972; ARM ARM BTI HINT #32/#34/#36/#38; llvm-mc -triple=aarch64 -show-encoding
```

## encode_bti_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM BTI field layout (not the SUT match table). bits[31:12]=0xD5032, bits[11:8]=0b0100 (CRm), bit5=0 (op2[0]), bits[4:0]=11111, word = 0xD503241F | (j<<7) | (c<<6). Stronger rejected: State machine (no lifecycle); Differential is the primary oracle (this is a field-level check that does not need llvm-mc); Round-trip (no decoder).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:545 "bti (no target)" — asserted fingerprint 37377fe2
- Seed: src/backend/arm/assembler/encoder/encode_hint_pbt.rs:239 (ARM layout invariant)
- Formal: ∀ t ∈ {ε, c, j, jc}. let w = encode_bti(t). Word(w) ⇒ (w >> 12 = 0xD5032) ∧ ((w >> 8) & 0xF = 0b0100) ∧ ((w >> 5) & 1 = 0) ∧ (w & 0x1F = 0b11111) ∧ (w = 0xD503241F | (j(t) << 7) | (c(t) << 6))
- Test file: src/backend/arm/assembler/encoder/encode_bti_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_bti
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [target]
  domain: { target: { "", "c", "j", "jc" } }
  relation:
    op: eq
    lhs: encode_bti(target)
    rhs: Word(0xD503241F | (j_flag << 7) | (c_flag << 6))
generators:
  target: { gen: oneof, of: ["", "c", "j", "jc"], type: String }
evidence: ARM ARM BTI encoding HINT CRm=0100 op2=xx0; system.rs:545-548
```

## encode_bti_meta_jc_bits
- Tier: 4
- Rationale: Metamorphic from ARM ARM: j and c independently set op2 bits (bit7 and bit6). encode("jc") = encode("j") XOR encode("c") XOR encode(""); encode("j") XOR encode("") = 1<<7; encode("c") XOR encode("") = 1<<6; the four targets produce four distinct words. Independent of llvm-mc. Stronger rejected: State machine; Round-trip (no decoder); Differential already covers value agreement.
- Doc contract: src/backend/arm/assembler/encoder/system.rs:548 "bti jc" — asserted fingerprint a0145091
- Seed: src/backend/arm/assembler/encoder/encode_hint_pbt.rs:249 (imm isolation / field independence)
- Formal: let w0,wc,wj,wjc = encode_bti of ε,c,j,jc. (wj ⊕ w0 = 1<<7) ∧ (wc ⊕ w0 = 1<<6) ∧ (wjc = wj ⊕ wc ⊕ w0) ∧ |{w0,wc,wj,wjc}| = 4
- Test file: src/backend/arm/assembler/encoder/encode_bti_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_bti
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: []
  domain: { the four ARM BTI targets }
  relation:
    op: eq
    lhs: encode_bti("jc")
    rhs: encode_bti("j") XOR encode_bti("c") XOR encode_bti("")
generators: {}
evidence: ARM ARM BTI op2 bit2=j bit1=c bit0=0; system.rs:545-548
```

## encode_bti_meta_case_ws
- Tier: 4
- Rationale: Metamorphic: ASCII case-fold and surrounding whitespace are behavior-preserving on the valid domain (gas/llvm-mc accept BTI C / extra spaces; parser stores raw_operands without lowercasing so encode_bti's trim+lowercase is the contract). Stronger rejected: State machine; Round-trip; Differential already covers llvm-mc agreement including case.
- Doc contract: src/backend/arm/assembler/encoder/system.rs:541 "Encode HINT #imm (system hint instruction)" — other fingerprint 87550030
- Seed: src/backend/arm/assembler/encoder/encode_hint_pbt.rs:229 (valid-domain wrapping)
- Formal: ∀ t ∈ {ε, c, j, jc}. ∀ case ∈ ASCII-case-fold(t). ∀ pad ∈ {space, tab}*. encode_bti(pad · case · pad) = encode_bti(t)
- Test file: src/backend/arm/assembler/encoder/encode_bti_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_bti
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [target, case, pad]
  domain: { target: { "", "c", "j", "jc" }, case: ASCII-case-fold(target), pad: {space,tab}* }
  relation:
    op: eq
    lhs: encode_bti(pad + case + pad)
    rhs: encode_bti(target)
generators:
  target: { gen: oneof, of: ["", "c", "j", "jc"], type: String }
  pad: { gen: string, alphabet: " \t", maxLen: 4, type: String }
evidence: README.md:12 gas-compat; parser.rs:1746 raw_operands not lowercased; system.rs:543 trim+to_lowercase
```

## encode_bti_neg_unknown
- Tier: 3
- Rationale: Negative/error contract: gas and llvm-mc reject unknown BTI options ("unknown option to BTI" / "invalid operand"). README.md:12 claims gas-compatible assembly. encode_bti's own comments do not declare unknown names out of domain, so they stay in the generator. Stronger rejected: State machine; Round-trip; Differential on unknown names has no encoding (reference rejects).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:541 "Encode HINT #imm (system hint instruction)" — other fingerprint 87550030
- Seed: src/backend/arm/assembler/encoder/encode_hint_pbt.rs:319 (wrong-kind / unknown)
- Formal: ∀ s. trim(lower(s)) ∉ {ε, c, j, jc} ∧ llvm-mc("bti " · s) is Err ⇒ encode_bti(s) is Err ∧ error contains "unsupported bti target"
- Test file: src/backend/arm/assembler/encoder/encode_bti_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_bti
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: operand strings whose trim+lowercase is not in {"", "c", "j", "jc"} }
  relation:
    op: throws
    lhs: encode_bti(s)
    rhs: unsupported bti target
generators:
  s: { gen: string, type: String }
expected_error: String
evidence: README.md:12; gas "unknown option to BTI at operand 1"; llvm-mc "invalid operand for instruction"; system.rs:549
```

## encode_bti_neg_extra
- Tier: 3
- Rationale: Negative/error contract: gas and llvm-mc reject extra operands after a BTI target ("unexpected characters following instruction" / "invalid operand"). README.md:12 claims gas-compatible assembly. encode_bti's own comments do not declare extra operands invalid, so they stay in the generator. Stronger rejected: State machine; Round-trip; Differential on extra operands has no encoding (reference rejects).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:541 "Encode HINT #imm (system hint instruction)" — other fingerprint 87550030
- Seed: src/backend/arm/assembler/encoder/encode_hint_pbt.rs:267 (extra operand)
- Formal: ∀ t ∈ {ε, c, j, jc}. ∀ extra ∈ extra-operand-tokens. ∀ comma ∈ {true, false}. let raw = comma-or-space join of t and extra. ¬valid(raw) ∧ llvm-mc("bti" · raw) is Err ⇒ encode_bti(raw) is Err ∧ error contains "unsupported bti target"
- Test file: src/backend/arm/assembler/encoder/encode_bti_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.system.encode_bti
oracle: negative_error
predicate:
  quantifier: forall
  vars: [target, extra]
  domain: { target: { "", "c", "j", "jc" }, extra: operand tokens }
  relation:
    op: throws
    lhs: encode_bti(target + ", " + extra)
    rhs: unsupported bti target
generators:
  target: { gen: oneof, of: ["", "c", "j", "jc"], type: String }
  extra: { gen: string, type: String }
expected_error: String
evidence: README.md:12; gas "unexpected characters following instruction"; llvm-mc "invalid operand for instruction"
```
