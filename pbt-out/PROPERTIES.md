# Properties: encode_dmb

## encode_dmb_diff_named
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc (LLVM 15.0.6, -triple=aarch64 -show-encoding) on the 12 ARM-named DMB options. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree DMB decoder). encode_dsb rejected as same-job sibling (DSB uses bits[7:5]=100, DMB uses 101). Evidence: README.md:12 gas-compatible assembly; README.md:239 lists dmb; encoder/mod.rs:964 passes operands through; ARM named CRm map; llvm-mc and GNU as agree on the 12 names. Case folding is included because SUT lowercases and llvm-mc accepts mixed case.
- Doc contract: src/backend/arm/assembler/encoder/system.rs:25 "DMB: 0xD50330BF | (CRm << 8)" — asserted fingerprint ee074f29
- Seed: encode_cnt_pbt.rs:170 llvm-mc differential
- Formal: ∀ name ∈ {sy,st,ld,ish,ishst,ishld,nsh,nshst,nshld,osh,oshst,oshld}, ∀ case ∈ CaseFold(name), ∀ as_symbol ∈ Bool. encode_dmb([Barrier(case) | Symbol(case)]) = llvm-mc("dmb " + case) as Word
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dmb
oracle: differential
predicate:
  quantifier: forall
  vars: [name, case_name]
  domain: { name: named_dmb_option, case_name: case_fold(name) }
  relation:
    op: eq
    lhs: encode_dmb([Barrier(case_name)])
    rhs: llvm_mc_word("dmb " + case_name)
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dmb_diff_imm
- Tier: 2
- Rationale: GNU as and llvm-mc both accept `dmb #imm` / `dmb imm` for imm in 0..=15 and encode CRm=imm. Parser produces Operand::Imm for `#n` and bare integers (parser.rs:1991, 2020). SUT only matches Barrier/Symbol, so Imm falls through to SY. Differential vs llvm-mc is the strongest evidenced oracle on this domain. ARM ARM documents the `#<imm>` assembler form. Documented bound 0..=15 is sampled exactly (generator min 0 max 15).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:25 "DMB: 0xD50330BF | (CRm << 8)" — asserted fingerprint ee074f29 (CRm is the encoding field that #imm fills)
- Seed: encode_cnt_pbt.rs:170 llvm-mc differential generalized to DMB #imm
- Formal: ∀ crm ∈ {0,…,15}. encode_dmb([Imm(crm)]) = llvm-mc("dmb #" + crm) as Word
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: failing
- Counterexample: crm = 0; encode_dmb([Imm(0)]) = Word(0xd5033fbf) vs llvm-mc dmb #0 = Word(0xd50330bf)
- Bug report: pbt-out/bug_reports/encode_dmb_imm_ignored.md

```property
function: encoder.encode_dmb
oracle: differential
predicate:
  quantifier: forall
  vars: [crm]
  domain: { crm: 0..=15 }
  relation:
    op: eq
    lhs: encode_dmb([Imm(crm as i64)])
    rhs: llvm_mc_word("dmb #" + crm)
generators:
  crm: { gen: int, min: 0, max: 15, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dmb_meta_barrier_eq_symbol
- Tier: 4c
- Rationale: SUT matches Barrier and Symbol with the same arm (system.rs:8). Parser emits Barrier for the 12 names and Symbol for other tokens; callers and tests also construct Symbol. Metamorphic: encoding must not depend on which of those two variants carries the same string. Stronger differential is used on the named/imm domain separately; this isolates the Barrier/Symbol split. Round-trip rejected (no decoder).
- Doc contract: src/backend/arm/assembler/parser.rs:50 "Barrier option for dmb/dsb: ish, ishld, ishst, sy, etc." — asserted fingerprint fe3a9532 (parser classifies the names; encoder treats Barrier and Symbol alike)
- Seed: encode_cnt_pbt.rs:182 Rd/Rn isolation metamorphic generalized to Barrier vs Symbol
- Formal: ∀ name ∈ NamedDmb, ∀ case ∈ CaseFold(name). encode_dmb([Barrier(case)]) = encode_dmb([Symbol(case)])
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dmb
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [name, case_name]
  domain: { name: named_dmb_option, case_name: case_fold(name) }
  relation:
    op: eq
    lhs: encode_dmb([Barrier(case_name)])
    rhs: encode_dmb([Symbol(case_name)])
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
evidence: src/backend/arm/assembler/encoder/system.rs:8
```

## encode_dmb_inv_arm_layout
- Tier: 4d
- Rationale: ARM ARM DMB encoding is 1101 0101 0000 0011 0011 CRm 101 11111. For a named option the CRm nibble is the documented map and all other bits are fixed. Independent of llvm-mc (which is the differential property); this pins the architectural field layout so a swapped nibble or DSB opcode leak cannot hide behind a passing named-option KAT that used the same wrong table. CRm isolation: two different names differ only in bits[11:8].
- Doc contract: src/backend/arm/assembler/encoder/system.rs:25 "DMB: 0xD50330BF | (CRm << 8)" — asserted fingerprint ee074f29
- Seed: encode_cnt_pbt.rs:207 ARM layout invariant
- Formal: ∀ name ∈ NamedDmb. let w = encode_dmb([Barrier(name)]) in Word. w = 0xD50330BF | (crm(name) << 8) ∧ bits[31:12] = 0xD5033 ∧ bits[7:5] = 0b101 ∧ bits[4:0] = 0b11111 ∧ bits[11:8] = crm(name)
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dmb
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: named_dmb_option }
  relation:
    op: eq
    lhs: encode_dmb([Barrier(name)])
    rhs: 0xd50330bf | (crm(name) << 8)
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
evidence: src/backend/arm/assembler/encoder/system.rs:24
```

## encode_dmb_neg_unknown
- Tier: 4e
- Rationale: system.rs:21 returns Err("unknown dmb option: {b}") for Barrier/Symbol names outside the 12-option set. llvm-mc rejects unknown barrier option names. Documented error contract; generator draws names that are not in the closed set (including empty, numeric-as-symbol, near-miss spellings).
- Doc contract: src/backend/arm/assembler/encoder/system.rs:21 "unknown dmb option: {}" — domain-restriction fingerprint 90176615
- Seed: encode_cnt_pbt.rs:260 invalid T negative
- Formal: ∀ s ∉ NamedDmb (as Barrier or Symbol, any case). encode_dmb([Barrier(s)]) is Err ∧ encode_dmb([Symbol(s)]) is Err ∧ the error contains "unknown dmb option"
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_dmb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: string not in named_dmb_option (case-insensitive) }
  relation:
    op: throws
    expr: encode_dmb([Barrier(s)])
generators:
  s: { gen: string, minLen: 1, maxLen: 8, type: String }
expected_error: unknown dmb option
evidence: src/backend/arm/assembler/encoder/system.rs:21
```

## encode_dmb_neg_extra
- Tier: 4e
- Rationale: GNU as rejects `dmb sy, x0` ("unexpected characters following instruction"); llvm-mc rejects extra operands. encode() passes the full operand slice through (mod.rs:964). SUT reads only operands.first() and ignores the rest, so extra operands silently encode. Documented gas/llvm-mc rejection is the error contract; extra operands stay in the domain.
- Doc contract: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438 (gas rejects extra operands)
- Seed: encode_cnt_pbt.rs:232 extra operand negative
- Formal: ∀ name ∈ NamedDmb, ∀ extra ∈ Operand. llvm-mc("dmb " + name + ", …") is Err ⇒ encode_dmb([Barrier(name), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: failing
- Counterexample: name = "sy", extra = Reg("x0"); encode_dmb([Barrier("sy"), Reg("x0")]) = Ok(Word(0xd5033fbf))
- Bug report: pbt-out/bug_reports/encode_dmb_extra_operand.md

```property
function: encoder.encode_dmb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, extra]
  domain: { name: named_dmb_option, extra: Operand }
  relation:
    op: throws
    expr: encode_dmb([Barrier(name), extra])
generators:
  name: { gen: oneof, values: ["sy","st","ld","ish","ishst","ishld","nsh","nshst","nshld","osh","oshst","oshld"], type: "&str" }
expected_error: extra operand
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dmb_neg_empty
- Tier: 4e
- Rationale: GNU as rejects omitted operand ("missing immediate expression at operand 1"); llvm-mc rejects `dmb` with no operand ("too few operands"). SUT empty-slice arm defaults to SY (CRm=15). ARM ARM notes option default SY, but the assembler's claimed contract is gas (README.md:12), and gas requires an operand. Empty stays in the domain.
- Doc contract: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_cnt_pbt.rs:226 arity negative
- Formal: encode_dmb([]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: failing
- Counterexample: encode_dmb([]) = Ok(Word(0xd5033fbf))
- Bug report: pbt-out/bug_reports/encode_dmb_empty_defaults_sy.md

```property
function: encoder.encode_dmb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [unused]
  domain: { unused: unit }
  relation:
    op: throws
    expr: encode_dmb([])
generators:
  unused: { gen: const, value: 0, type: u32 }
expected_error: missing operand
evidence: src/backend/arm/assembler/README.md:12
```

## encode_dmb_neg_wrong_kind_imm_oob
- Tier: 4e
- Rationale: GNU as / llvm-mc reject a register, memory operand, or `#imm` outside 0..=15 as the DMB operand. Parser can produce Imm, Reg, Mem, Cond, etc. SUT treats every non-Barrier/non-Symbol first operand as SY, including Imm(16), Imm(-1), and Reg("x0"). Documented bound 0..=15 is sampled at 0, 15, 16, -1, and i64 extremes. Wrong kinds stay in the domain.
- Doc contract: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" — asserted fingerprint f00ab438
- Seed: encode_cnt_pbt.rs:278 GPR/bare/sp negative
- Formal: ∀ op ∈ {Imm(n) | n ∉ 0..=15} ∪ {Reg, Mem, Cond, Shift, Label, …}. llvm-mc rejects the corresponding assembly ⇒ encode_dmb([op]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_dmb_pbt.rs
- Status: failing
- Counterexample: op = Imm(-1); encode_dmb([Imm(-1)]) = Ok(Word(0xd5033fbf))
- Bug report: pbt-out/bug_reports/encode_dmb_wrong_kind_defaults_sy.md

```property
function: encoder.encode_dmb
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind]
  domain: { kind: imm_oob | reg | mem | cond }
  relation:
    op: throws
    expr: encode_dmb([wrong_kind_operand(kind)])
generators:
  kind: { gen: int, min: 0, max: 3, type: u8 }
expected_error: invalid dmb operand
evidence: src/backend/arm/assembler/README.md:12
```
