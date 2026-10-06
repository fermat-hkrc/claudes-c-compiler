# Properties: encode_cond_branch

## encode_cond_branch_diff_imm_llvm_mc
- Tier: 3
- Rationale: Strongest applicable is Differential vs llvm-mc (independent AArch64 assembler). State machine rejected (pure function). Round-trip rejected (no in-tree B.cond decoder). encode_branch/cbz/tbz rejected (different jobs). ARM ARM B.cond accepts a PC-relative immediate offset, multiple of 4, in ±1 MiB; gas and llvm-mc assemble `b.{cond} #imm`.
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs encode_branch_diff_imm_llvm_mc
- Formal: ∀ cond ∈ {eq,ne,cs,hs,cc,lo,mi,pl,vs,vc,hi,ls,ge,lt,gt,le,al,nv}, ∀ imm ∈ {k·4 | k ∈ ℤ, −1048576 ≤ k·4 ≤ 1048572}. encode_cond_branch(cond, [Imm(imm)]) = Word(w) ∧ w = llvm-mc(`b.{cond} #imm`)
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: failing
- Counterexample: cond = "eq", imm = -1048576 (b.eq #-1048576)
- Bug report: pbt-out/bug_reports/encode_cond_branch_imm_offset.md

```property
function: encoder.compare_branch.encode_cond_branch
oracle: differential
predicate:
  quantifier: forall
  vars: [cond, imm]
  domain: { cond: cond16, imm: aligned_imm19 }
  relation:
    op: eq
    lhs: encode_cond_branch(cond, [Imm(imm)])
    rhs: llvm_mc_word("b.{cond} #{imm}")
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  imm: { gen: int, min: -1048576, max: 1048572, type: i64 }
evidence: compare_branch.rs:200 README.md:12 README.md:220
```

## encode_cond_branch_diff_reloc_eq_imm0
- Tier: 3
- Rationale: Reloc-form word (imm19=0) must equal llvm-mc `b.{cond} #0`. Differential vs llvm-mc.
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs encode_branch_kat_symbol_foo
- Formal: ∀ cond ∈ 18 names, ∀ s. encode_cond_branch(cond, [Symbol(s)]) = WordWithReloc{word: w, …} ∧ w = llvm-mc(`b.{cond} #0`)
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: differential
predicate:
  quantifier: forall
  vars: [cond, s]
  domain: { cond: cond16, s: symbol }
  relation:
    op: eq
    lhs: word_of(encode_cond_branch(cond, [Symbol(s)]))
    rhs: llvm_mc_word("b.{cond} #0")
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  s: { gen: string, type: String }
evidence: compare_branch.rs:200 README.md:267
```

## encode_cond_branch_symbol_reloc
- Tier: 4
- Rationale: README.md:267/458 and compare_branch.rs:204-207 assert CondBr19 (ELF 280) with the symbol and addend preserved.
- Doc contract: README.md:458 "All branch-type relocations (B, BL, B.cond, CBZ/CBNZ, TBZ/TBNZ) are deferred" — asserted fingerprint f35fd13e
- Seed: compare_branch.rs encode_cbz_symbol_reloc
- Formal: ∀ cond ∈ 18 names, ∀ s, ∀ a ∈ ℤ. encode_cond_branch(cond, [Symbol(s)|Label(s)]) = WordWithReloc{reloc: CondBr19, symbol: s, addend: 0, elf_type: 280} ∧ encode_cond_branch(cond, [SymbolOffset(s,a)]) has addend = a
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [cond, s, a]
  domain: { cond: cond16, s: symbol, a: addend }
  relation:
    op: holds
    expr: reloc_is_condbr19(encode_cond_branch(cond, [Symbol(s)])) && reloc_is_condbr19(encode_cond_branch(cond, [Label(s)])) && reloc_addend(encode_cond_branch(cond, [SymbolOffset(s, a)])) == a
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  s: { gen: string, type: String }
  a: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: README.md:267 README.md:458 compare_branch.rs:204
```

## encode_cond_branch_word_layout
- Tier: 4
- Rationale: ARM ARM / compare_branch.rs:200: bits[31:24]=01010100, imm19 at [23:5] is 0 in reloc form, bit 4 is 0, cond at [3:0].
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs encode_branch_word_layout
- Formal: ∀ cond ∈ 18 names, ∀ s. let w = word(encode_cond_branch(cond, [Symbol(s)])). (w>>24)=0x54 ∧ ((w>>4)&1)=0 ∧ ((w>>5)&0x7FFFF)=0 ∧ (w&0xF)∈0..15
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [cond, s]
  domain: { cond: cond16, s: symbol }
  relation:
    op: holds
    expr: arm_bcond_layout(word_of(encode_cond_branch(cond, [Symbol(s)])))
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  s: { gen: string, type: String }
evidence: compare_branch.rs:200
```

## encode_cond_branch_meta_cond
- Tier: 4
- Rationale: ARM ARM condition aliases cs≡hs, cc≡lo; invert of a 4-bit cond is bit 0. Changing cond only mutates bits[3:0].
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs encode_cbz_meta_cbz_vs_cbnz
- Formal: ∀ s, ∀ a. encode_cond_branch("cs",[SymbolOffset(s,a)]) = encode_cond_branch("hs",[…]) ∧ encode_cond_branch("cc",[…]) = encode_cond_branch("lo",[…]) ∧ ∀ invertible cond. word(cond) XOR word(invert(cond)) = 1 ∧ ∀ cond1,cond2. (word(cond1) XOR word(cond2)) & ~0xF = 0
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [s, a]
  domain: { s: symbol, a: addend }
  relation:
    op: eq
    lhs: encode_cond_branch("cs", [SymbolOffset(s, a)])
    rhs: encode_cond_branch("hs", [SymbolOffset(s, a)])
generators:
  s: { gen: string, type: String }
  a: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: README.md:220 ARM ARM condition encodings
```

## encode_cond_branch_neg_arity
- Tier: 4
- Rationale: llvm-mc/gas reject `b.eq` with no operand. Negative/error contract.
- Doc contract: compare_branch.rs:199 "    let (sym, addend) = get_symbol(operands, 0)?;" — asserted fingerprint 0a538088
- Seed: compare_branch.rs encode_branch_neg_arity
- Formal: ∀ cond ∈ 18 names. encode_cond_branch(cond, []) is Err
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [cond]
  domain: { cond: cond16 }
  relation:
    op: throws
    expr: encode_cond_branch(cond, [])
    error: String
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
expected_error: String
evidence: README.md:12 llvm-mc too few operands
```

## encode_cond_branch_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc/gas reject `b.eq label, extra`. No operands.len() guard in the SUT; extra operands must still Err under the gas contract.
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs encode_branch_neg_extra_operand
- Formal: ∀ cond ∈ 18 names, ∀ s, ∀ extra ∈ {Reg,Imm,Symbol,Mem}. encode_cond_branch(cond, [Symbol(s), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: failing
- Counterexample: cond = "eq", suffix = 0, which = 0 (b.eq labl0, x0)
- Bug report: pbt-out/bug_reports/encode_cond_branch_extra_operand.md

```property
function: encoder.compare_branch.encode_cond_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [cond, s, extra]
  domain: { cond: cond16, s: symbol, extra: extra_operand }
  relation:
    op: throws
    expr: encode_cond_branch(cond, [Symbol(s), extra])
    error: String
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  s: { gen: string, type: String }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: README.md:12 llvm-mc invalid operand
```

## encode_cond_branch_neg_unknown_cond
- Tier: 4
- Rationale: compare_branch.rs:198 returns Err("unknown condition: {cond}") when encode_cond yields None.
- Doc contract: compare_branch.rs:198 "    let cond_val = encode_cond(cond).ok_or_else(|| format!("unknown condition: {}", cond))?;" — asserted fingerprint dd207d24
- Seed: compare_branch.rs encode_cset_neg_unknown_cond
- Formal: ∀ c ∉ 18 names (non-empty, not a case-variant of those names). encode_cond_branch(c, [Symbol("L")]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [c]
  domain: { c: unknown_cond }
  relation:
    op: throws
    expr: encode_cond_branch(c, [Symbol("L")])
    error: String
generators:
  c: { gen: string, minLen: 1, maxLen: 8, type: String }
expected_error: String
evidence: compare_branch.rs:198
```

## encode_cond_branch_neg_bad_operand
- Tier: 4
- Rationale: Sweep: Mem/Shift/Extend/RegArrangement/Expr are not B.cond operands; gas/llvm-mc reject them.
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs encode_branch_neg_bad_operand
- Formal: ∀ cond ∈ 18 names, ∀ bad ∈ {Mem, Shift, Extend, RegArrangement, Expr}. encode_cond_branch(cond, [bad]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [cond, which]
  domain: { cond: cond16, which: 0..4 }
  relation:
    op: throws
    expr: encode_cond_branch(cond, [bad_operand(which)])
    error: String
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: README.md:12 llvm-mc expected label or encodable integer pc offset
```

## encode_cond_branch_neg_modifier
- Tier: 4
- Rationale: Sweep: `:lo12:` modifiers are not valid B.cond operands; gas/llvm-mc reject them. get_symbol currently accepts Modifier.
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs test_encode_branch_regression_modifier
- Formal: ∀ cond ∈ 18 names. encode_cond_branch(cond, [Modifier{lo12, foo}|ModifierOffset{lo12, foo, 8}]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: failing
- Counterexample: cond = "eq", which = 0 (b.eq :lo12:foo)
- Bug report: pbt-out/bug_reports/encode_cond_branch_modifier.md

```property
function: encoder.compare_branch.encode_cond_branch
oracle: negative_error
predicate:
  quantifier: forall
  vars: [cond, which]
  domain: { cond: cond16, which: 0..1 }
  relation:
    op: throws
    expr: encode_cond_branch(cond, [modifier(which)])
    error: String
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  which: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: README.md:12 gas rejects relocation modifiers on B.cond
```

## encode_cond_branch_symbol_misclassified
- Tier: 4
- Rationale: Sweep: get_symbol documents that the parser misclassifies symbol names colliding with Reg/Cond/Barrier; those remain valid B.cond targets (gas accepts `b.eq x0` as a label named x0).
- Doc contract: compare_branch.rs:200 "B.cond: 01010100 imm19 0 cond" — asserted fingerprint b04d2a9b
- Seed: compare_branch.rs encode_branch_symbol_misclassified
- Formal: ∀ cond ∈ 18 names, ∀ name ∈ {eq,ne,lt,gt,sy,ish,st,ld}, ∀ kind ∈ {Reg,Cond,Barrier}. encode_cond_branch(cond, [kind(name)]) = WordWithReloc{CondBr19, symbol: name, addend: 0}
- Test file: src/backend/arm/assembler/encoder/encode_cond_branch_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cond_branch
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [cond, which, name]
  domain: { cond: cond16, which: 0..2, name: colliding_names }
  relation:
    op: holds
    expr: reloc_symbol(encode_cond_branch(cond, [misclassified(which, name)])) == name
generators:
  cond: { gen: element, of: ["eq","ne","cs","hs","cc","lo","mi","pl","vs","vc","hi","ls","ge","lt","gt","le","al","nv"], type: String }
  which: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: element, of: ["eq","ne","lt","gt","sy","ish","st","ld"], type: String }
evidence: encoder/mod.rs:1139 parser-misclassified symbols
```
