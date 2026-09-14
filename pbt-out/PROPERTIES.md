# Properties: encode_ret

## encode_ret_diff_xn_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler of the same GNU-style RET text). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree RET decoder. encode_br rejected (same-job gate: BR / opc=0000). encode_blr rejected (same-job gate: BLR / opc=0001). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; mapping [] <-> `ret`, [Reg("xN"|"xzr"|"lr")] <-> `ret xN`.
- Seed: src/backend/arm/codegen/prologue.rs:319 emits bare `ret`; peephole.rs:1028 classifies it
- Formal: ∀ name ∈ {⊥} ∪ {x0..x30, xzr, lr, X0}. encode_ret(ops(name)) = Word(v) ∧ llvm-mc(-triple=aarch64, asm(name)) = v, where ops(⊥)=[] and asm(⊥)="ret"
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ret
oracle: differential
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: optional X-register name or omitted }
  relation:
    op: eq
    lhs: encode_ret(ops)
    rhs: llvm_mc_word(asm)
generators:
  name: { gen: optional, elem: { gen: string } }
evidence: src/backend/arm/assembler/README.md:14 same textual assembly as gas; README.md:220 ret under Branches; compare_branch.rs:232 RET 1101011 0010 11111 Rn; ARM ARM Unconditional branch (register) RET opc=0010, omitted Xn defaults to X30
```

## encode_ret_word_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM / body comment: bits[31:25]=1101011, opc[24:21]=0010, op2=11111, op3=000000, Rn[9:5], op4=00000. Differential is stronger and used on the X-reg domain; this pins the field layout independently of llvm-mc.
- Seed: (none)
- Formal: ∀ n ∈ {0..31}. encode_ret([Reg(xn)]) = Word(w) ⇒ w = 0xd65f0000 | (n << 5) ∧ (w>>25)=0b1101011 ∧ ((w>>21)&0xF)=0b0010 ∧ (w&0x1F)=0 ∧ ((w>>5)&0x1F)=n
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ret
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..31 }
  relation:
    op: eq
    lhs: encode_ret([Reg(xn)])
    rhs: Word(0xd65f0000 | (n << 5))
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: compare_branch.rs:232 RET 1101011 0010 11111 000000 Rn 00000; ARM ARM Unconditional branch (register) RET opc=0010
```

## encode_ret_meta_default_x30_lr
- Tier: 4
- Rationale: ARM ARM / body comment: omitted Xn defaults to X30 (LR). GNU as / llvm-mc alias `ret` ≡ `ret x30` ≡ `ret lr`. Metamorphic equality of the three operand forms.
- Seed: prologue.rs:319 bare `ret`; compare_branch.rs:227-228 default Rn=30
- Formal: ∀. encode_ret([]) = encode_ret([Reg("x30")]) = encode_ret([Reg("lr")]) = Word(0xd65f03c0)
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ret
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [dummy]
  domain: { dummy: unit }
  relation:
    op: eq
    lhs: encode_ret([])
    rhs: encode_ret([Reg("x30")])
generators:
  dummy: { gen: int, min: 0, max: 0, type: u32 }
evidence: compare_branch.rs:227-228 empty operands default to x30 (LR); ARM ARM RET omitted Xn is X30; llvm-mc ret / ret x30 / ret lr all encode 0xd65f03c0
```

## encode_ret_meta_vs_br
- Tier: 4
- Rationale: ARM ARM Unconditional branch (register): RET opc=0010 vs BR opc=0000, otherwise identical. For the same Rn, RET XOR BR = bit 22. encode_br is a different-job sibling used only as a metamorphic companion, not a differential reference.
- Seed: (none)
- Formal: ∀ n ∈ {0..31}. encode_ret([Reg(xn)]) XOR encode_br([Reg(xn)]) = 1<<22
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ret
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..31 }
  relation:
    op: eq
    lhs: encode_ret([Reg(xn)]) XOR encode_br([Reg(xn)])
    rhs: 1u32 << 22
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: compare_branch.rs:214 BR 1101011 0000 11111 Rn; compare_branch.rs:232 RET 1101011 0010 11111 Rn; ARM ARM opc BR=0000 RET=0010 (bit 22)
```

## encode_ret_neg_w_reg
- Tier: 4
- Rationale: ARM ARM Rn is Xn (64-bit GPR). llvm-mc rejects `ret wN`. README.md:14 gas-compat. Negative/error contract: W-form Rn must Err.
- Seed: (none)
- Formal: ∀ n ∈ {0..32}. encode_ret([Reg(wn)]) is Err, where w32 maps to wzr/wsp
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: n=0 (ret w0 encodes as ret x0 / 0xd65f0000)
- Bug report: pbt-out/bug_reports/encode_ret_w_reg.md

```property
function: encode_ret
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..32 }
  relation:
    op: throws
    expr: encode_ret([Reg(wn)])
generators:
  n: { gen: int, min: 0, max: 32, type: u32 }
expected_error: String
evidence: ARM ARM RET Rn is Xn; llvm-mc rejects ret w0; README.md:14 same textual assembly as gas
```

## encode_ret_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects `ret xN, extra`. ARM ARM RET takes at most one Xn. README.md:14 gas-compat. Extra operand must Err.
- Seed: (none)
- Formal: ∀ n ∈ {0..30}, extra ∈ {Reg, Imm, Symbol, Mem}. encode_ret([Reg(xn), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: n=0, which=0 (ret x0, x1 encodes as ret x0 / 0xd65f0000)
- Bug report: pbt-out/bug_reports/encode_ret_extra_operand.md

```property
function: encode_ret
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, extra]
  domain: { n: 0..30, extra: one of Reg Imm Symbol Mem }
  relation:
    op: throws
    expr: encode_ret([Reg(xn), extra])
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects ret x0, x1; ARM ARM RET takes at most one Xn; README.md:14 gas-compat
```

## encode_ret_neg_bad_operand
- Tier: 4
- Rationale: RET takes a GPR or nothing. Imm/Mem/Shift/Extend/RegArrangement/Modifier/Symbol/Label are not valid RET operands (llvm-mc / ARM ARM). Must Err.
- Seed: (none)
- Formal: ∀ bad ∈ {Imm, Mem, Shift, Extend, RegArrangement, Modifier, Symbol, Label}. encode_ret([bad]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ret
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which]
  domain: { which: 0..7 }
  relation:
    op: throws
    expr: encode_ret([bad(which)])
generators:
  which: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: ARM ARM RET operand is optional Xn; get_reg at encoder/mod.rs:956 expected register; llvm-mc rejects non-GPR
```

## encode_ret_neg_wrong_reg_class
- Tier: 4
- Rationale: ARM ARM register 31 is XZR not SP; Rn is GPR not FP/SIMD. llvm-mc rejects `ret sp` / `ret d0` / invalid names. Must Err.
- Seed: (none)
- Formal: ∀ name ∈ {sp, wsp, dN, sN, qN, vN, hN, bN, x32, w32, foo, "", r0, x, x-1, x99}. encode_ret([Reg(name)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: which=0, n=0 (ret sp encodes as ret xzr / 0xd65f03e0)
- Bug report: pbt-out/bug_reports/encode_ret_sp.md

```property
function: encode_ret
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: SP or FP or invalid GPR names }
  relation:
    op: throws
    expr: encode_ret([Reg(name)])
generators:
  name: { gen: string }
expected_error: String
evidence: ARM ARM RET Rn is Xn, register 31 is XZR never SP; parse_reg_num encoder/mod.rs:131; llvm-mc rejects ret sp / ret d0
```

## encode_ret_neg_fp_reg
- Tier: 4
- Rationale: ARM ARM RET Rn is Xn. llvm-mc rejects `ret d0` and other FP/SIMD names. Dedicated generator over {d,s,q,v,h,b} so a failure shrinks to an FP witness (distinct from SP).
- Seed: (none)
- Formal: ∀ prefix ∈ {d,s,q,v,h,b}, n ∈ {0..31}. encode_ret([Reg(prefix||n)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: which=0, n=0 (ret d0 encodes as ret x0 / 0xd65f0000)
- Bug report: pbt-out/bug_reports/encode_ret_fp_reg.md

```property
function: encode_ret
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, n]
  domain: { which: 0..5, n: 0..31 }
  relation:
    op: throws
    expr: encode_ret([Reg(fp_name(which, n))])
generators:
  which: { gen: int, min: 0, max: 5, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM RET Rn is Xn; llvm-mc rejects ret d0; parse_reg_num encoder/mod.rs:141 accepts d/s/q/v/h/b
```

## encode_ret_neg_invalid_name
- Tier: 4
- Rationale: Coverage sweep of get_reg parse_reg_num None arm. Names that are not a valid register encoding (x32, w32, foo, empty, r0, x, x-1, x99) must Err with invalid register. Distinct from SP/FP which parse_reg_num accepts.
- Seed: (none)
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_ret([Reg(name)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_ret
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which]
  domain: { which: 0..7 }
  relation:
    op: throws
    expr: encode_ret([Reg(invalid_name(which))])
generators:
  which: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: get_reg encoder/mod.rs:956-961 parse_reg_num None returns invalid register; ARM ARM RET Rn is a GPR number 0-31
```
