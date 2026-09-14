# Properties: encode_branch

## encode_branch_diff_imm_llvm_mc
- Tier: 7
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler) for the ARM ARM immediate form `b #imm`. State machine rejected (pure function, no lifecycle). In-tree B decoder does not exist so algebraic round-trip is unavailable. encode_bl is a different job (BL, with link / Call26) and fails the same-job sibling gate as a differential reference. Doc evidence: assembler README gas-compat; ARM ARM Unconditional branch (immediate) B bits[31:26]=000101, imm26=offset/4; README Jump26 for `b`.
- Seed: src/backend/arm/assembler/encoder/compare_branch.rs encode_bl_pbt::encode_bl_diff_imm_llvm_mc
- Formal: ∀ imm ∈ {k*4 | k ∈ ℤ, -2^27 ≤ imm ≤ 2^27-4}. encode_branch([Imm(imm)]) = Word(llvm-mc("b #" + imm)).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: Imm(-134217728)  (also Imm(0), Imm(4))
- Bug report: pbt-out/bug_reports/encode_branch_imm_offset.md

```property
function: encoder.compare_branch.encode_branch
oracle: differential
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: aligned_pc_offset_26 }
  relation:
    op: eq
    lhs: encode_branch([Imm(imm)]) as Word
    rhs: llvm_mc("b #" + imm)
generators:
  imm: { gen: int, min: -134217728, max: 134217724, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; README.md:220 Branches lists b; encoder/mod.rs:316 b dispatch; compare_branch.rs:173 B 000101 imm26; ARM ARM Unconditional branch (immediate)
```

## encode_branch_symbol_reloc
- Tier: 4
- Rationale: Algebraic invariant from README Jump26 / R_AARCH64_JUMP26 ELF 282 and the encoder contract that symbol/label targets leave imm26=0 for the assembler/linker to fill. Stronger differential cannot compare a concrete word for unresolved labels (llvm-mc emits a fixup, not a numeric encoding). encode_bl is not a same-job sibling.
- Seed: encode_bl_pbt::encode_bl_symbol_reloc
- Formal: ∀ s ∈ ident, a ∈ i64. encode_branch([Symbol(s)]) = WordWithReloc{word=0x14000000, Jump26, s, 0} ∧ encode_branch([Label(s)]) = same ∧ encode_branch([SymbolOffset(s,a)]) = WordWithReloc{word=0x14000000, Jump26, s, a} ∧ elf_type(Jump26)=282 ∧ (word>>26)=0b000101 ∧ (word & 0x03ffffff)=0.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_branch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [s, a]
  domain: { s: label_ident, a: i64 }
  body: encode_branch([SymbolOffset(s, a)]) == WordWithReloc(0x14000000, Jump26, s, a)
generators:
  s: { gen: string }
  a: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: encoder/mod.rs:51-52 R_AARCH64_JUMP26 for B; encoder/mod.rs:102 Jump26 => 282; README.md:254 Jump26 282 b; compare_branch.rs:173-181 imm26 filled by linker/assembler
```

## encode_branch_meta_vs_bl
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM BL is B with bit 31 set (100101 vs 000101 at bits[31:26]). encode_bl is not a same-job differential reference; the relation is the documented opcode pair. Stronger differential already covers the happy path; this isolates the B-vs-BL contract independently of llvm-mc.
- Seed: encode_bl_pbt::encode_bl_meta_vs_b
- Formal: ∀ s ∈ ident, a ∈ i64. encode_bl([SymbolOffset(s,a)]).word XOR encode_branch([SymbolOffset(s,a)]).word = 1<<31 ∧ reloc types are Call26 vs Jump26 ∧ same symbol and addend.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_branch
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [s, a]
  domain: { s: label_ident, a: i64 }
  relation:
    op: eq
    lhs: encode_bl(ops).word XOR encode_branch(ops).word
    rhs: 1 << 31
generators:
  s: { gen: string }
  a: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: compare_branch.rs:173 B 000101 imm26; compare_branch.rs:186 BL 100101 imm26; ARM ARM Unconditional branch (immediate) bit 31
```

## encode_branch_word_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM Unconditional branch (immediate) B encoding: bits[31:26]=000101, imm26 field zero until reloc fill. Stronger differential already covers Imm against llvm-mc; this pins the field split on the reloc path independently of the assembler.
- Seed: encode_bl_pbt::encode_bl_word_layout
- Formal: ∀ s ∈ ident. let (w, r) = encode_branch([Symbol(s)]). (w >> 26) & 0x3f = 0b000101 ∧ (w & 0x03ffffff) = 0 ∧ r is Jump26.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_branch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: label_ident }
  relation:
    op: eq
    lhs: encode_branch([Symbol(s)]).word >> 26
    rhs: 0b000101
generators:
  s: { gen: string }
evidence: compare_branch.rs:173 B 000101 imm26; ARM ARM Unconditional branch (immediate) bits[31:26]=000101
```

## encode_branch_neg_arity
- Tier: 3
- Rationale: Negative/error contract from llvm-mc / gas: bare `b` is "too few operands". Stronger oracles do not apply to the empty-operand domain.
- Seed: encode_bl_pbt::encode_bl_neg_arity
- Formal: encode_branch([]) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dummy]
  domain: { dummy: u32 }
  relation:
    op: holds
    expr: encode_branch([]).is_err()
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects bare b with too few operands; README.md:5-14 gas-compat
```

## encode_branch_neg_imm_unaligned_oor
- Tier: 3
- Rationale: Negative/error contract from ARM ARM / llvm-mc: B offset must be a 4-byte-aligned integer in [-2^27, 2^27-4]. llvm-mc rejects `b #1` as "expected label or encodable integer pc offset".
- Seed: encode_bl_pbt::encode_bl_neg_imm_unaligned_oor
- Formal: ∀ imm ∉ aligned_pc_offset_26. encode_branch([Imm(imm)]) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: unaligned_or_oor_pc_offset }
  relation:
    op: holds
    expr: encode_branch([Imm(imm)]).is_err()
generators:
  imm: { gen: int, min: -134217729, max: 134217725, type: i64 }
expected_error: String
evidence: ARM ARM Unconditional branch (immediate) imm26 = offset/4, range plus/minus 128 MiB; llvm-mc rejects b #1
```

## encode_branch_neg_extra_operand
- Tier: 3
- Rationale: Negative/error contract from llvm-mc / gas: B takes a single target; a second operand is "invalid operand for instruction".
- Seed: encode_bl_pbt::encode_bl_neg_extra_operand
- Formal: ∀ s ∈ ident, extra ∈ {Reg, Imm, Symbol, Mem}. encode_branch([Symbol(s), extra]) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Symbol("labl0"), Reg("x0")]
- Bug report: pbt-out/bug_reports/encode_branch_extra_operand.md

```property
function: encoder.compare_branch.encode_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s, extra]
  domain: { s: label_ident, extra: Operand }
  relation:
    op: holds
    expr: encode_branch([Symbol(s), extra]).is_err()
generators:
  s: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects b foo, x0 with invalid operand; README.md:5-14 gas-compat
```

## encode_branch_neg_bad_operand
- Tier: 3
- Rationale: Negative/error contract: B operand is a label or encodable integer PC offset, not Mem/Shift/Extend/RegArrangement/Modifier. llvm-mc does not treat `:lo12:` as a B target.
- Seed: encode_bl_pbt::encode_bl_neg_bad_operand
- Formal: ∀ bad ∈ {Mem, Shift, Extend, RegArrangement, Modifier, ModifierOffset}. encode_branch([bad]) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: Modifier { kind: "lo12", symbol: "foo" }  (which=4); also ModifierOffset
- Bug report: pbt-out/bug_reports/encode_branch_modifier.md

```property
function: encoder.compare_branch.encode_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: non_branch_operand }
  relation:
    op: holds
    expr: encode_branch([bad]).is_err()
generators:
  bad: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: llvm-mc rejects b :lo12:foo; ARM ARM B operand is label or pc offset; README.md:5-14 gas-compat
```

## encode_branch_symbol_misclassified
- Tier: 4
- Rationale: Algebraic invariant from get_symbol's documented parser-misclassification workaround: names that collide with registers, condition codes, or barrier options are valid branch symbols in context. Coverage sweep of get_symbol Reg/Cond/Barrier arms that encode_branch otherwise never hits. Stronger differential cannot compare a concrete word for unresolved labels.
- Seed: encode_bl_pbt::encode_bl_symbol_misclassified
- Formal: ∀ name ∈ {eq,ne,lt,gt,sy,ish,st,ld}, kind ∈ {Reg,Cond,Barrier}. encode_branch([kind(name)]) = WordWithReloc{word=0x14000000, Jump26, name, 0}.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_branch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: u32 in 0..=2, name: colliding_ident }
  relation:
    op: eq
    lhs: encode_branch([misclassified(which, name)]).word
    rhs: 0x14000000
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: string }
evidence: encoder/mod.rs:982-986 parser misclassifies symbol names that collide with register names, condition codes, or barrier names; these are valid symbols in context
```
