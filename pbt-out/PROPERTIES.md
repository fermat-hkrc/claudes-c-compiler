# Properties: encode_bl

## encode_bl_diff_imm_llvm_mc
- Tier: 7
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler) for the ARM ARM immediate form `bl #imm`. State machine rejected (pure function, no lifecycle). In-tree BL decoder does not exist so algebraic round-trip is unavailable. encode_branch is a different job (B/Jump26, no link) and fails the same-job sibling gate as a differential reference. Doc evidence: assembler README gas-compat; ARM ARM BL bits[31:26]=100101, imm26 = offset/4; llvm-mc accepts aligned offsets in [-134217728, 134217724].
- Seed: src/backend/arm/assembler/encoder/load_store.rs encode_adr_pbt::encode_adr_diff_imm_llvm_mc (sibling PC-relative immediate generalization)
- Formal: ∀ imm ∈ {k·4 | k ∈ ℤ, -2^25 ≤ k ≤ 2^25-1}. encode_bl([Imm(imm)]) = Word(llvm-mc("bl #imm")).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: imm = -134217728 (bl #-134217728); also Imm(0), Imm(4)
- Bug report: pbt-out/bug_reports/encode_bl_imm_offset.md

```property
function: encoder.compare_branch.encode_bl
oracle: differential
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: aligned_i64_in_pm_128MiB }
  relation:
    op: eq
    lhs: encode_bl([Imm(imm)]) as Word
    rhs: llvm_mc("bl #imm")
generators:
  imm: { gen: int, min: -134217728, max: 134217724, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:317 bl dispatch; compare_branch.rs:186 BL 100101 imm26; ARM ARM Unconditional branch (immediate)
```

## encode_bl_symbol_reloc
- Tier: 4
- Rationale: Algebraic invariant from the documented reloc contract: BL to a symbol/label/symbol+addend returns WordWithReloc { word = 0b100101<<26, Call26, symbol, addend } with imm26 left 0 for the assembler/linker to fill. Stronger differential via llvm-mc -show-encoding is unavailable on this path (encoding uses A placeholders / a CALL26 fixup, not a numeric word). encode_branch is not a same-job sibling.
- Seed: src/backend/arm/assembler/encoder/load_store.rs encode_adr_pbt::encode_adr_symbol_reloc
- Formal: ∀ s ∈ ident, ∀ addend ∈ i64. encode_bl([Symbol(s)]) = encode_bl([Label(s)]) = WordWithReloc{word:0x94000000, Call26, s, 0} ∧ encode_bl([SymbolOffset(s,addend)]) = WordWithReloc{word:0x94000000, Call26, s, addend} ∧ Call26.elf_type()=283.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_bl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [s, addend]
  domain: { s: ident, addend: i64 }
  relation:
    op: holds
    expr: word_is_0x94000000_and_reloc_is_Call26_with_symbol_and_addend
generators:
  suffix: { gen: int, min: 0, max: 1000, type: u32 }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: README.md:247-253 Call26 ELF 283 for bl; encoder/mod.rs:49 R_AARCH64_CALL26; compare_branch.rs:186-192 WordWithReloc Call26 word 0b100101<<26
```

## encode_bl_meta_vs_b
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM BL is B with bit 31 set (100101 vs 000101) and reloc Call26 vs Jump26. encode_branch is not a same-job differential reference; the relation is the documented opcode/reloc pair. Stronger differential already covers the immediate happy path; this isolates the BL-vs-B contract independently of llvm-mc.
- Seed: encode_adc_pbt S-bit XOR; encode_bics opc XOR
- Formal: ∀ s ∈ ident, ∀ addend ∈ i64. let bl = encode_bl([SymbolOffset(s,addend)]); let b = encode_branch([SymbolOffset(s,addend)]). bl.word XOR b.word = 1<<31 ∧ bl.reloc_type=Call26 ∧ b.reloc_type=Jump26 ∧ bl.symbol=b.symbol=s ∧ bl.addend=b.addend=addend.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_bl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [s, addend]
  domain: { s: ident, addend: i64 }
  relation:
    op: eq
    lhs: encode_bl(ops).word XOR encode_branch(ops).word
    rhs: 1 << 31
generators:
  suffix: { gen: int, min: 0, max: 1000, type: u32 }
  addend: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: compare_branch.rs:173 B 000101 Jump26; compare_branch.rs:186 BL 100101 Call26; ARM ARM Unconditional branch (immediate) op bit 31
```

## encode_bl_word_layout
- Tier: 4
- Rationale: Algebraic invariant pinning each field of the reloc-form word so a swapped opcode would fail even if llvm-mc were unavailable: bits[31:26]=100101, bits[25:0]=0 (imm26 filled later). Stronger differential already covers numeric equality on the immediate path.
- Seed: (none)
- Formal: ∀ s ∈ ident. let W = encode_bl([Symbol(s)]).word. W[31:26]=0b100101 ∧ W[25:0]=0.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_bl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [s]
  domain: { s: ident }
  relation:
    op: holds
    expr: ((word >> 26) & 0x3F) == 0b100101 && (word & 0x3FFFFFF) == 0
generators:
  suffix: { gen: int, min: 0, max: 1000, type: u32 }
evidence: compare_branch.rs:186 BL 100101 imm26; ARM ARM Unconditional branch (immediate); README.md:376 JUMP26/CALL26 encode imm26 field later
```

## encode_bl_neg_arity
- Tier: 3
- Rationale: Negative/error contract: BL requires a target operand. llvm-mc reports "too few operands for instruction" for bare `bl`. Stronger oracles do not apply to the missing-operand path.
- Seed: encode_adr_pbt::encode_adr_neg_bad_operands
- Formal: ∀ ops with |ops|=0. encode_bl(ops) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_bl
oracle: negative_error
predicate:
  quantifier: forall
  vars: []
  domain: { ops: empty }
  relation:
    op: throws
    expr: encode_bl([])
expected_error: String
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
evidence: llvm-mc rejects `bl`; README.md:5-14 gas-compat; get_symbol errors when operand 0 is missing
```

## encode_bl_neg_imm_unaligned_oor
- Tier: 3
- Rationale: Negative/error contract from ARM ARM / llvm-mc: the PC offset must be a multiple of 4 and in [-2^27, 2^27-4]. llvm-mc rejects #1, #134217728, #-134217732. Bounds -134217728 / 134217724 and bound±4 (and unaligned 1) are sampled exactly.
- Seed: encode_adr_pbt::encode_adr_neg_imm_range
- Formal: ∀ imm ∈ {-134217732, -134217729, -1, 1, 2, 3, 5, 134217725, 134217728, i64::MIN, i64::MAX}. encode_bl([Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_bl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [imm]
  domain: { imm: unaligned_or_out_of_26bit_range }
  relation:
    op: throws
    expr: encode_bl([Imm(imm)])
expected_error: String
generators:
  imm: { gen: int, min: -134217732, max: 134217728, type: i64 }
evidence: llvm-mc rejects `bl #1` and `bl #134217728`; ARM ARM imm26 range ±128MB, offset multiple of 4; README.md:5-14 gas-compat
```

## encode_bl_neg_extra_operand
- Tier: 3
- Rationale: Negative/error contract: BL takes a single target. llvm-mc rejects `bl foo, x0`. Stronger differential does not apply to invalid encodings.
- Seed: encode_bics_pbt::encode_bics_neg_extra_operand
- Formal: ∀ s ∈ ident, ∀ extra ∈ {Reg, Imm, Symbol, Mem}. encode_bl([Symbol(s), extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: suffix=0, which=0 (bl labl0, x0)
- Bug report: pbt-out/bug_reports/encode_bl_extra_operand.md

```property
function: encoder.compare_branch.encode_bl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [s, extra]
  domain: { extra: non_empty_second_operand }
  relation:
    op: throws
    expr: encode_bl([Symbol(s), extra])
expected_error: String
generators:
  suffix: { gen: int, min: 0, max: 1000, type: u32 }
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc rejects `bl foo, x0`; README.md:5-14 gas-compat; ARM ARM BL has a single label/imm operand
```

## encode_bl_neg_bad_operand
- Tier: 3
- Rationale: Negative/error contract: Mem/Shift/Extend/RegArrangement/Modifier are not BL targets. llvm-mc rejects `bl :lo12:foo` and register-offset memory; ARM ARM BL operand is a label or PC offset. Modifier is accepted by get_symbol (kind dropped) which would violate the gas-compat contract.
- Seed: encode_adr_pbt::encode_adr_neg_modifier
- Formal: ∀ bad ∈ {Mem, Shift, Extend, RegArrangement, Modifier, ModifierOffset}. encode_bl([bad]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: which=4 (Operand::Modifier { kind: lo12, symbol: foo })
- Bug report: pbt-out/bug_reports/encode_bl_modifier.md

```property
function: encoder.compare_branch.encode_bl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad]
  domain: { bad: Mem|Shift|Extend|RegArrangement|Modifier|ModifierOffset }
  relation:
    op: throws
    expr: encode_bl([bad])
expected_error: String
generators:
  which: { gen: int, min: 0, max: 5, type: u32 }
evidence: llvm-mc rejects `bl :lo12:foo`; ARM ARM BL operand is label or encodable integer pc offset; README.md:5-14 gas-compat
```

## encode_bl_symbol_misclassified
- Tier: 4
- Rationale: Algebraic invariant covering get_symbol parser-misclassification arms (coverage sweep). Doc evidence: encoder/mod.rs:982-986 states that symbol names colliding with register/cond/barrier names are valid symbols in context. llvm-mc accepts `bl eq` and `bl sy` as Call26 labels. Stronger differential via -show-encoding is unavailable (A placeholders).
- Seed: encode_adr_pbt::encode_adr_symbol_misclassified
- Formal: ∀ which ∈ {Reg,Cond,Barrier}, ∀ name ∈ {eq,ne,lt,gt,sy,ish,st,ld}. encode_bl([which(name)]) = WordWithReloc{word:0x94000000, Call26, name, 0}.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_bl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [which, name]
  domain: { which: {Reg,Cond,Barrier}, name: {eq,ne,lt,gt,sy,ish,st,ld} }
  relation:
    op: holds
    expr: encode_bl([which(name)]) is Call26 reloc with symbol=name addend=0 word=0x94000000
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  name: { gen: oneof, options: [eq, ne, lt, gt, sy, ish, st, ld] }
evidence: encoder/mod.rs:982-986 parser-misclassified Reg/Cond/Barrier are valid symbols in context; llvm-mc accepts `bl eq` as Call26
```
