# Properties: encode_br

## encode_br_diff_xn_llvm_mc
- Tier: 7
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler) for the ARM ARM register form `br Xn`. State machine rejected (pure function, no lifecycle). In-tree BR decoder does not exist so algebraic round-trip is unavailable. encode_blr is a different job (BLR, with link) and fails the same-job sibling gate as a differential reference. Doc evidence: assembler README gas-compat; ARM ARM Unconditional branch (register) BR bits[31:25]=1101011 opc=0000 Rn at [9:5]; codegen emits `br x0` and `br x17`.
- Seed: src/backend/arm/assembler/encoder/compare_branch.rs encode_blr_pbt::encode_blr_diff_xn_llvm_mc
- Formal: ∀ n ∈ {0..30} ∪ {xzr, lr}. encode_br([Reg(name(n))]) = Word(llvm-mc("br " + name(n))).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_br
oracle: differential
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: x_reg_or_xzr_or_lr }
  relation:
    op: eq
    lhs: encode_br([Reg(name(n))]) as Word
    rhs: llvm_mc("br " + name(n))
generators:
  n: { gen: int, min: 0, max: 32, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; README.md:220 Branches lists br; encoder/mod.rs:318 br dispatch; compare_branch.rs:214 BR 1101011 0000 11111 Rn; ARM ARM Unconditional branch (register); codegen/emit.rs:1760 br x0; emit.rs:1808 br x17
```

## encode_br_word_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM Unconditional branch (register) BR encoding: fixed bits 0xd61f0000 with Rn occupying [9:5] and op4 [4:0] zero. Stronger differential already covers the happy path against llvm-mc; this pins the field split independently of the assembler. encode_blr is not a same-job sibling.
- Seed: encode_blr_pbt::encode_blr_word_layout
- Formal: ∀ n ∈ 0..31. let w = encode_br([Reg(xn)]). w = 0xd61f0000 | (n << 5) ∧ (w >> 25) = 0b1101011 ∧ ((w >> 21) & 0xF) = 0b0000 ∧ (w & 0x1F) = 0 ∧ ((w >> 5) & 0x1F) = n.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_br
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: u32 in 0..=31 }
  relation:
    op: eq
    lhs: encode_br([Reg("x"+n)]).word
    rhs: 0xd61f0000 | (n << 5)
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: compare_branch.rs:214 BR 1101011 0000 11111 000000 Rn 00000; ARM ARM Unconditional branch (register) opc=0000 op2=11111 op4=00000
```

## encode_br_meta_vs_blr
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM BLR is BR with opc bit 21 set (0001 vs 0000 at bits[24:21]). encode_blr is not a same-job differential reference; the relation is the documented opcode pair. Stronger differential already covers the happy path; this isolates the BR-vs-BLR contract independently of llvm-mc.
- Seed: encode_blr_pbt::encode_blr_meta_vs_br
- Formal: ∀ n ∈ 0..31. encode_blr([Reg(xn)]).word XOR encode_br([Reg(xn)]).word = 1<<21.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_br
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: u32 in 0..=31 }
  relation:
    op: eq
    lhs: encode_blr(ops).word XOR encode_br(ops).word
    rhs: 1 << 21
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: compare_branch.rs:214 BR 1101011 0000 11111; compare_branch.rs:221 BLR 1101011 0001 11111; ARM ARM Unconditional branch (register) opc bit 21
```

## encode_br_neg_arity
- Tier: 4
- Rationale: Negative/error contract: llvm-mc and gas reject bare `br` (too few operands). ARM ARM BR requires Rn. Stronger differential does not apply on the empty domain.
- Seed: encode_blr_pbt::encode_blr_neg_arity
- Formal: encode_br([]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_br
oracle: negative_error
predicate:
  quantifier: forall
  vars: []
  domain: {}
  relation:
    op: throws
    expr: encode_br([])
    error: String
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects bare br (too few operands); assembler README.md:5-14 gas-compat; ARM ARM BR requires Xn
```

## encode_br_neg_w_reg
- Tier: 4
- Rationale: Negative/error contract: ARM ARM Rn is a 64-bit GPR; llvm-mc rejects `br wN` / `br wzr` / `br wsp`. Gas-compat assembler must Err, not silently encode as Xn. Stronger differential does not apply on the invalid domain. Bounds w0 (invalid) vs x0 (valid) and w30 vs x30 sampled exactly.
- Seed: encode_blr_pbt::encode_blr_neg_w_reg
- Formal: ∀ n ∈ {0..30} ∪ {wzr, wsp}. encode_br([Reg(wn)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: n = 0 (br w0) -> Ok(Word(0xd61f0000))
- Bug report: pbt-out/bug_reports/encode_br_w_reg.md

```property
function: encoder.compare_branch.encode_br
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: w_reg_or_wzr_or_wsp }
  relation:
    op: throws
    expr: encode_br([Reg(wn)])
    error: String
generators:
  n: { gen: int, min: 0, max: 32, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects br w0 / wzr / wsp; ARM ARM Unconditional branch (register) Rn is Xn; README.md:5-14 gas-compat
```

## encode_br_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract: llvm-mc rejects a second operand (`br x0, x1`, `br x0, #0`). Gas-compat assembler must Err rather than silently drop extras. Stronger differential does not apply on the invalid domain.
- Seed: encode_blr_pbt::encode_blr_neg_extra_operand
- Formal: ∀ n ∈ 0..30, ∀ extra ∈ {Reg, Imm, Symbol, Mem}. encode_br([Reg(xn), extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: n = 0, which = 0 ([Reg("x0"), Reg("x1")]) -> Ok(Word(0xd61f0000))
- Bug report: pbt-out/bug_reports/encode_br_extra_operand.md

```property
function: encoder.compare_branch.encode_br
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, extra]
  domain: { n: 0..=30, extra: extra_operand }
  relation:
    op: throws
    expr: encode_br([Reg(xn), extra])
    error: String
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects br x0, x1 and br x0, #0 (invalid operand); README.md:5-14 gas-compat
```

## encode_br_neg_bad_operand
- Tier: 4
- Rationale: Negative/error contract: BR takes a 64-bit GPR, not Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol. llvm-mc rejects `br #0` and `br foo`. Stronger differential does not apply on the invalid domain.
- Seed: encode_blr_pbt::encode_blr_neg_bad_operand
- Formal: ∀ op ∈ {Imm, Mem, Shift, Extend, RegArrangement, Modifier, Symbol, Label}. encode_br([op]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_br
oracle: negative_error
predicate:
  quantifier: forall
  vars: [op]
  domain: { op: non_gpr_operand }
  relation:
    op: throws
    expr: encode_br([op])
    error: String
generators:
  which: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects br #0 and br foo; ARM ARM BR requires Xn; README.md:5-14 gas-compat
```

## encode_br_neg_wrong_reg_class
- Tier: 4
- Rationale: Negative/error contract: llvm-mc rejects SP (register 31 is XZR, not SP for BR), FP/SIMD names (d/s/q/v/h/b), and invalid names (x32, foo, empty, r0). parse_reg_num currently maps those prefixes; gas-compat requires Err. Stronger differential does not apply on the invalid domain. Bounds x30 (valid) vs x32 (invalid) and n=31 as xzr (valid) vs sp (invalid) are sampled exactly.
- Seed: encode_blr_pbt::encode_blr_neg_wrong_reg_class
- Formal: ∀ name ∈ {sp, wsp} ∪ {p+n | p∈{d,s,q,v,h,b}, n∈0..31} ∪ {x32, w32, foo, "", r0, x}. encode_br([Reg(name)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: which = 0, n = 0 (br sp) -> Ok(Word(0xd61f03e0)); also br d0 -> Ok(Word(0xd61f0000))
- Bug report: pbt-out/bug_reports/encode_br_sp_as_zr.md; pbt-out/bug_reports/encode_br_fp_reg.md

```property
function: encoder.compare_branch.encode_br
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: sp_or_fp_or_invalid }
  relation:
    op: throws
    expr: encode_br([Reg(name)])
    error: String
generators:
  which: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects br sp / d0 / x32 / r0 / foo; ARM ARM Rn is Xn not SP; README.md:5-14 gas-compat
```

## encode_br_neg_invalid_name
- Tier: 4
- Rationale: Coverage-sweep of get_reg's parse_reg_num None arm. Names that are not a valid register encoding (x32, w32, foo, empty, r0, x, x-1, x99) must Err. Distinct from SP/FP which parse_reg_num currently accepts (filed as bugs). llvm-mc rejects these names. Stronger differential does not apply on the invalid domain.
- Seed: encode_blr_pbt::encode_blr_neg_invalid_name
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_br([Reg(name)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_br
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_br([Reg(name)])
    error: String
generators:
  which: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects br x32 / foo / r0; parse_reg_num returns None for those names; README.md:5-14 gas-compat
```
