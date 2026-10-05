# Properties: encode_dsb

## encode_dsb_diff_named
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc (LLVM 15.0.6, -triple=aarch64 -show-encoding) on the 12 ARM-named DSB options. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree DSB decoder). encode_dmb rejected as same-job sibling (DMB uses bits[7:5]=101, DSB uses 100). Evidence: README.md:12 gas-compatible assembly; README.md:239 lists dsb; encoder/mod.rs:967 passes operands through; ARM named CRm map; llvm-mc and GNU as agree on the 12 names. Case folding is included because SUT lowercases and llvm-mc accepts mixed case.
- Doc contract: src/backend/arm/assembler/encoder/system.rs:49 "DSB: 0xD503309F | (option << 8)" — asserted fingerprint d96979ad
- Seed: encode_dmb_pbt.rs:216 llvm-mc named-option differential
- Formal: ∀ name ∈ {sy,st,ld,ish,ishst,ishld,nsh,nshst,nshld,osh,oshst,oshld}, ∀ case ∈ CaseFold(name), ∀ as_symbol ∈ Bool. encode_dsb([Barrier(case) | Symbol(case)]) = llvm-mc("dsb " + case) as Word
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dsb
oracle: differential
predicate:
  quantifier: forall
  vars: [name, case_name]
  domain: { name: named_dsb_option, case_name: case_fold(name) }
  relation:
    op: eq
    lhs: encode_dsb([Barrier(case_name)])
    rhs: llvm_mc_word("dsb " + case_name)
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dsb_diff_imm
- Tier: 2
- Rationale: GNU as and llvm-mc both accept `dsb #imm` / `dsb imm` for imm in 0..=15 and encode CRm=imm. Parser produces Operand::Imm for `#n` and bare integers (parser.rs:1991, 2020). SUT only matches Barrier/Symbol, so Imm falls through to SY. Differential vs llvm-mc is the strongest evidenced oracle on this domain. ARM ARM documents the `#<imm>` assembler form. Documented bound 0..=15 is sampled exactly (generator min 0 max 15).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:49 "DSB: 0xD503309F | (option << 8)" — asserted fingerprint d96979ad (option/CRm is the encoding field that #imm fills)
- Seed: encode_dmb_pbt.rs:236 llvm-mc #imm differential generalized to DSB #imm
- Formal: ∀ crm ∈ {0,…,15}. encode_dsb([Imm(crm)]) = llvm-mc("dsb #" + crm) as Word
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: failing
- Counterexample: crm = 0; encode_dsb([Imm(0)]) = Word(0xd5033f9f) vs llvm-mc dsb #0 = Word(0xd503309f)
- Bug report: pbt-out/bug_reports/encode_dsb_imm_ignored.md

```property
function: encoder.encode_dsb
oracle: differential
predicate:
  quantifier: forall
  vars: [crm]
  domain: { crm: 0..=15 }
  relation:
    op: eq
    lhs: encode_dsb([Imm(crm as i64)])
    rhs: llvm_mc_word("dsb #" + crm)
generators:
  crm: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dsb_meta_barrier_eq_symbol
- Tier: 4c
- Rationale: SUT matches Barrier and Symbol with the same arm (system.rs:32). Parser emits Barrier for the 12 names and Symbol for other tokens; callers and tests also construct Symbol. Metamorphic: encoding must not depend on which of those two variants carries the same string. Stronger differential is used on the named/imm domain separately; this isolates the Barrier/Symbol split. Round-trip rejected (no decoder).
- Doc contract: src/backend/arm/assembler/parser.rs:50 "Barrier option for dmb/dsb: ish, ishld, ishst, sy, etc." — asserted fingerprint fe3a9532 (parser classifies the names; encoder treats Barrier and Symbol alike)
- Seed: encode_dmb_pbt.rs:243 Barrier vs Symbol metamorphic
- Formal: ∀ name ∈ NamedDsb, ∀ case ∈ CaseFold(name). encode_dsb([Barrier(case)]) = encode_dsb([Symbol(case)])
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dsb
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [name, case_name]
  domain: { name: named_dsb_option, case_name: case_fold(name) }
  relation:
    op: eq
    lhs: encode_dsb([Barrier(case_name)])
    rhs: encode_dsb([Symbol(case_name)])
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
evidence: src/backend/arm/assembler/encoder/system.rs:32
```

## encode_dsb_inv_arm_layout
- Tier: 4d
- Rationale: ARM ARM DSB encoding is 1101 0101 0000 0011 0011 CRm 100 11111. For a named option the CRm nibble is the documented map and all other bits are fixed. Independent of llvm-mc (which is the differential property); this pins the architectural field layout so a swapped nibble or DMB opcode leak cannot hide behind a passing named-option KAT that used the same wrong table. CRm isolation: two different names differ only in bits[11:8].
- Doc contract: src/backend/arm/assembler/encoder/system.rs:49 "DSB: 0xD503309F | (option << 8)" — asserted fingerprint d96979ad
- Seed: encode_dmb_pbt.rs:255 ARM layout invariant
- Formal: ∀ name ∈ NamedDsb. let w = encode_dsb([Barrier(name)]) in Word. w = 0xD503309F | (crm(name) << 8) ∧ bits[31:12] = 0xD5033 ∧ bits[7:5] = 0b100 ∧ bits[4:0] = 0b11111 ∧ bits[11:8] = crm(name)
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dsb
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: named_dsb_option }
  relation:
    op: eq
    lhs: encode_dsb([Barrier(name)])
    rhs: 0xd503309f | (crm(name) << 8)
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
evidence: src/backend/arm/assembler/encoder/system.rs:49
```

## encode_dsb_neg_unknown
- Tier: 4e
- Rationale: system.rs:45 returns Err("unknown dsb option: {b}") for Barrier/Symbol names outside the 12-option set. llvm-mc rejects unknown barrier option names. Documented error contract; generator draws names that are not in the closed set (including empty, numeric-as-symbol, near-miss spellings).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:45 "unknown dsb option: {}" — domain-restriction fingerprint 88d9be28
- Seed: encode_dmb_pbt.rs:281 unknown-name negative
- Formal: ∀ s ∉ NamedDsb (as Barrier or Symbol, any case). encode_dsb([Barrier(s)]) is Err ∧ encode_dsb([Symbol(s)]) is Err ∧ the error contains "unknown dsb option"
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dsb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: string not in named_dsb_option (case-insensitive) }
  relation:
    op: throws
    expr: encode_dsb([Barrier(s)])
generators:
  s: { gen: string, minLen: 1, maxLen: 8, type: String }
expected_error: unknown dsb option
evidence: src/backend/arm/assembler/encoder/system.rs:45
```

## encode_dsb_neg_extra
- Tier: 4e
- Rationale: GNU as rejects `dsb sy, x0` ("unexpected characters following instruction"); llvm-mc rejects extra operands. encode() passes the full operand slice through (mod.rs:967). SUT reads only operands.first() and ignores the rest, so extra operands silently encode. Documented gas/llvm-mc rejection is the error contract; extra operands stay in the domain.
- Doc contract: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438 (gas rejects extra operands)
- Seed: encode_dmb_pbt.rs:297 extra operand negative
- Formal: ∀ name ∈ NamedDsb, ∀ extra ∈ Operand. llvm-mc("dsb " + name + ", …") is Err ⇒ encode_dsb([Barrier(name), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: failing
- Counterexample: name = "sy", extra = Reg("x0"); encode_dsb([Barrier("sy"), Reg("x0")]) = Ok(Word(0xd5033f9f))
- Bug report: pbt-out/bug_reports/encode_dsb_extra_operand.md

```property
function: encoder.encode_dsb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, extra]
  domain: { name: named_dsb_option, extra: Operand }
  relation:
    op: throws
    expr: encode_dsb([Barrier(name), extra])
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
expected_error: extra operand
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dsb_neg_empty
- Tier: 4e
- Rationale: GNU as rejects omitted-operand `dsb` ("missing immediate expression"); llvm-mc rejects "too few operands for instruction dsb". SUT empty-slice falls through `_ => 0b1111` and silently encodes SY. Documented gas/llvm-mc rejection is the error contract; empty stays in the domain. The `_ => 0b1111` default is a producing statement, not a domain restriction.
- Doc contract: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438 (gas rejects omitted option)
- Seed: encode_dmb_pbt.rs:323 empty-operand negative
- Formal: encode_dsb([]) is Err ∧ llvm-mc("dsb") is Err
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: failing
- Counterexample: encode_dsb([]) = Ok(Word(0xd5033f9f)); llvm-mc("dsb") is Err
- Bug report: pbt-out/bug_reports/encode_dsb_empty_defaults_sy.md

```property
function: encoder.encode_dsb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..8 }
  relation:
    op: throws
    expr: encode_dsb([])
generators:
  n: { gen: int, min: 0, max: 7, type: u32 }
expected_error: omitted operand
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dsb_neg_wrong_kind_imm_oob
- Tier: 4e
- Rationale: llvm-mc rejects registers, memory, labels, condition codes, shifts, extends, and `#imm` outside 0..=15 as DSB operands. SUT only matches Barrier/Symbol; any other first-operand kind falls through to SY. Documented gas/llvm-mc rejection is the error contract. Imm 0..=15 is the valid #imm domain (separate differential); this generator covers Imm outside 0..=15 plus non-barrier kinds.
- Doc contract: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_dmb_pbt.rs:332 wrong-kind / oob-imm negative
- Formal: ∀ op ∈ WrongKind ∪ Imm(ℤ \ {0,…,15}). llvm-mc(asm(op)) is Err ⇒ encode_dsb([op]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_dsb_pbt.rs
- Status: failing
- Counterexample: op = Imm(-1); encode_dsb([Imm(-1)]) = Ok(Word(0xd5033f9f)); llvm-mc("dsb #-1") is Err
- Bug report: pbt-out/bug_reports/encode_dsb_wrong_kind_defaults_sy.md

```property
function: encoder.encode_dsb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: wrong_kind_or_imm_oob }
  relation:
    op: throws
    expr: encode_dsb([op])
generators:
  op: { gen: oneof, values: ["imm_oob","reg","mem","cond","shift","label","extend"] }
expected_error: invalid operand
evidence: src/backend/arm/assembler/README.md:12
```
