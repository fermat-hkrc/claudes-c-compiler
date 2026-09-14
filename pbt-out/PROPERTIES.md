# Properties: encode_csinv

## encode_csinv_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential against llvm-mc (independent AArch64 assembler). State machine rejected: encode_csinv is a pure function with no lifecycle. Round-trip rejected: no in-tree CSINV decoder. encode_csel / encode_csneg / encode_cinv fail the same-job sibling gate as differential references (different op/op2 or mnemonic/arity). Doc evidence: README.md:5-14 gas-compat; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:310 csinv dispatch; ARM ARM CSINV encoding.
- Seed: encode_csinc_pbt::encode_csinc_diff_llvm_mc (compare_branch.rs:9377)
- Formal: ∀ rd, rn, rm ∈ SameWidthGpr, c ∈ Cond16. encode_csinv([Reg(rd), Reg(rn), Reg(rm), Cond(c)]) = Word(w) ∧ w = llvm-mc("csinv rd, rn, rm, c")
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: cond16 }
  relation:
    op: eq
    lhs: encode_csinv([Reg(rd), Reg(rn), Reg(rm), Cond(c)])
    rhs: llvm_mc("csinv {rd}, {rn}, {rm}, {c}")
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; encoder/mod.rs:310 csinv dispatch; ARM ARM CSINV sf 1 0 11010100 Rm cond 00 Rn Rd
```

## encode_csinv_meta_vs_csel
- Tier: 3
- Rationale: ARM ARM Conditional Select family: CSINV is CSEL with op=1 instead of 0, so the encoded words differ only at bit 30. Stronger Differential already used for the valid domain; this metamorphic checks the architectural sibling relation independently of llvm-mc. encode_csel is not same-job (different mnemonic/op) so not a differential reference.
- Seed: encode_csel_pbt::encode_csel_meta_vs_csinv (compare_branch.rs:7188)
- Formal: ∀ rd, rn, rm ∈ SameWidthGpr, c ∈ Cond16. encode_csinv(ops) XOR encode_csel(ops) = 1<<30 where ops = [Reg(rd), Reg(rn), Reg(rm), Cond(c)]
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: cond16 }
  body: encode_csinv(ops) XOR encode_csel(ops) == (1u32 << 30)
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
evidence: ARM ARM CSEL op=0 vs CSINV op=1 at bit 30; compare_branch.rs:93 CSEL word vs compare_branch.rs:121 CSINV word
```

## encode_csinv_meta_vs_cinv
- Tier: 3
- Rationale: ARM ARM CINV is the documented alias CSINV Rd, Rn, Rn, invert(cond) (invalid for AL/NV). Metamorphic: encode_csinv([Rd, Rn, Rn, invert(c)]) equals encode_cinv([Rd, Rn, c]) over Cond14. encode_cinv is not same-job (3-operand alias) so not a differential reference.
- Seed: encode_cinv_pbt (compare_branch.rs:4152 CINV vs CSINV alias)
- Formal: ∀ rd, rn ∈ SameWidthGpr, c ∈ Cond14. encode_csinv([Reg(rd), Reg(rn), Reg(rn), Cond(invert(c))]) = encode_cinv([Reg(rd), Reg(rn), Cond(c)])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, c: cond14 }
  relation:
    op: eq
    lhs: encode_csinv([Reg(rd), Reg(rn), Reg(rn), Cond(invert(c))])
    rhs: encode_cinv([Reg(rd), Reg(rn), Cond(c)])
generators:
  rd: { gen: string }
  rn: { gen: string }
  c: { gen: string }
evidence: compare_branch.rs:308 CINV Rd, Rn, cond -> CSINV Rd, Rn, Rn, invert(cond); ARM ARM Conditional Invert alias of CSINV
```

## encode_csinv_word_layout
- Tier: 4
- Rationale: ARM ARM CSINV field layout is an exact structural invariant of every successful encoding. Weaker than Differential/Metamorphic but pins each field independently of llvm-mc and siblings.
- Seed: encode_csinc_pbt::encode_csinc_word_layout (compare_branch.rs:9499)
- Formal: ∀ rd,rn,rm ∈ 0..31, sf ∈ {0,1}, cond ∈ 0..15. encode_csinv([gpr(rd,sf), gpr(rn,sf), gpr(rm,sf), Cond(COND16[cond])]) = Word(w) ⇒ (w[31]=sf ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=0b11010100 ∧ w[20:16]=rm ∧ w[15:12]=cond ∧ w[11:10]=0b00 ∧ w[9:5]=rn ∧ w[4:0]=rd)
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, rn_n, rm_n, is_64, cond_enc]
  domain: { rd_n: 0..31, rn_n: 0..31, rm_n: 0..31, is_64: bool, cond_enc: 0..15 }
  body: word fields match ARM ARM CSINV layout (sf, op=1, S=0, 0b11010100, Rm, cond, op2=00, Rn, Rd)
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  rn_n: { gen: int, min: 0, max: 31, type: u32 }
  rm_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  cond_enc: { gen: int, min: 0, max: 15, type: u32 }
evidence: ARM ARM CSINV sf 1 0 11010100 Rm cond 00 Rn Rd; compare_branch.rs:121-123
```

## encode_csinv_neg_arity
- Tier: 4
- Rationale: llvm-mc and gas reject CSINV with fewer than 4 operands. Documented error contract of the gas-compat assembler surface. Invalid domain: arity in {0,1,2,3}.
- Seed: encode_csinc_pbt::encode_csinc_neg_arity (compare_branch.rs:9541)
- Formal: ∀ arity ∈ {0,1,2,3}. encode_csinv(ops) is Err when |ops| = arity (missing Rm and/or cond)
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..3 }
  relation:
    op: throws
    expr: encode_csinv(prefix_of_csinv(arity))
generators:
  arity: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects `csinv`, `csinv x0`, `csinv x0, x1`, `csinv x0, x1, x2` (too few operands); README.md:5-14 gas-compat
```

## encode_csinv_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a fifth operand on CSINV. Gas-compat contract requires Err. Invalid domain: valid 4-operand CSINV plus extra Reg/Imm/Symbol/Mem.
- Seed: encode_csinc_pbt::encode_csinc_neg_extra_operand (compare_branch.rs:9563)
- Formal: ∀ rd,rn,rm ∈ SameWidthGpr, c ∈ Cond16, extra ∈ {Reg, Imm, Symbol, Mem}. encode_csinv([Reg(rd), Reg(rn), Reg(rm), Cond(c), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x0"), Reg("x0"), Cond("eq"), Reg("x3")] (csinv x0, x0, x0, eq, extra which=0)
- Bug report: pbt-out/bug_reports/encode_csinv_extra_operand.md

```property
function: encoder.compare_branch.encode_csinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c, extra]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: cond16, extra: extra_operand }
  relation:
    op: throws
    expr: encode_csinv([Reg(rd), Reg(rn), Reg(rm), Cond(c), extra])
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc `csinv x0, x1, x2, eq, x3` error invalid operand; README.md:5-14 gas-compat
```

## encode_csinv_neg_wrong_reg
- Tier: 4
- Rationale: ARM ARM CSINV takes Wt/Xt only; register 31 is XZR/WZR not SP/WSP; mixed x/w is invalid. llvm-mc rejects SP, WSP, FP/SIMD, mixed width, and invalid names. Invalid domain generated by kind ∈ 0..8 covering those classes.
- Seed: encode_csinc_pbt::encode_csinc_neg_wrong_reg (compare_branch.rs:9587)
- Formal: ∀ kind ∈ 0..8, n ∈ 0..31. encode_csinv(bad_ops(kind, n)) is Err where bad_ops produces SP/WSP/FP/mixed/invalid-name operands
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind=0 n=0 → [Reg("sp"), Reg("x0"), Reg("x0"), Cond("eq")] (csinv sp, x0, x0, eq). Also fails for mixed x/w (kind=4) and FP d/s/q (kind=5..7).
- Bug report: pbt-out/bug_reports/encode_csinv_sp_as_zr.md; pbt-out/bug_reports/encode_csinv_mixed_width.md; pbt-out/bug_reports/encode_csinv_fp_reg.md

```property
function: encoder.compare_branch.encode_csinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: 0..8, n: 0..31 }
  relation:
    op: throws
    expr: encode_csinv(bad_ops(kind, n))
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects `csinv sp, x0, x1, eq`, `csinv d0, d1, d2, eq`, `csinv x0, w1, x2, eq`, `csinv wsp, w0, w1, eq`; ARM ARM Wt/Xt only, R31=XZR/WZR
```

## encode_csinv_neg_unknown_cond
- Tier: 4
- Rationale: encode_cond returns None for names outside the 16 architectural codes (plus hs/lo aliases). llvm-mc rejects unknown condition codes. Invalid domain: zz, foo, eqq, empty, "eq ", always.
- Seed: encode_csinc_pbt::encode_csinc_neg_unknown_cond (compare_branch.rs:9599)
- Formal: ∀ rd,rn,rm ∈ SameWidthGpr, c ∈ {zz, foo, eqq, "", "eq ", always}. encode_csinv([Reg(rd), Reg(rn), Reg(rm), Cond(c)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: unknown_cond }
  relation:
    op: throws
    expr: encode_csinv([Reg(rd), Reg(rn), Reg(rm), Cond(c)])
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
expected_error: String
evidence: encoder/mod.rs:169-190 encode_cond None arm; llvm-mc rejects unknown condition codes
```

## encode_csinv_neg_invalid_name
- Tier: 4
- Rationale: Coverage sweep of get_reg parse_reg_num None arm. Invalid register names (x32, w32, empty, foo, r0, x, x-1, x99) must Err. Documented by parse_reg_num returning None for out-of-range / non-matching names, and llvm-mc rejecting them.
- Seed: encode_csinc_pbt::encode_csinc_neg_invalid_name (compare_branch.rs)
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_csinv([Reg(name), Reg("x0"), Reg("x1"), Cond("eq")]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_csinv([Reg(name), Reg("x0"), Reg("x1"), Cond("eq")])
generators:
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-148 parse_reg_num None; encoder/mod.rs:956-966 get_reg invalid register
```

## encode_csinv_neg_bad_operand_kind
- Tier: 4
- Rationale: Coverage sweep of get_reg non-Reg and cond-not-Cond arms. Imm/Mem/Symbol/Shift/Label in any of the four slots must Err. Documented by get_reg requiring Operand::Reg and encode_csinv requiring Operand::Cond at index 3.
- Seed: encode_csinc_pbt::encode_csinc_neg_bad_operand_kind (compare_branch.rs)
- Formal: ∀ slot ∈ {0,1,2,3}, kind ∈ {Imm, Mem, Symbol, Shift, Label}. encode_csinv(ops with slot replaced by kind) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, which]
  domain: { slot: 0..3, which: 0..4 }
  relation:
    op: throws
    expr: encode_csinv(ops_with_bad_kind(slot, which))
generators:
  slot: { gen: int, min: 0, max: 3, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956-966 get_reg expected register; compare_branch.rs:117-119 csinv requires condition
```
