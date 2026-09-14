# Properties: encode_csneg

## encode_csneg_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential against llvm-mc (independent AArch64 assembler). State machine rejected: encode_csneg is a pure function with no lifecycle. Round-trip rejected: no in-tree CSNEG decoder. encode_csinc / encode_csinv / encode_csel / encode_cneg fail the same-job sibling gate as differential references (different op/op2 or mnemonic/arity). Doc evidence: README.md:5-14 gas-compat; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:311 csneg dispatch; ARM ARM CSNEG encoding.
- Seed: encode_csinc_pbt::encode_csinc_diff_llvm_mc (compare_branch.rs:9377)
- Formal: ∀ rd, rn, rm ∈ SameWidthGpr, c ∈ Cond16. encode_csneg([Reg(rd), Reg(rn), Reg(rm), Cond(c)]) = Word(w) ∧ w = llvm-mc("csneg rd, rn, rm, c")
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: cond16 }
  relation:
    op: eq
    lhs: encode_csneg([Reg(rd), Reg(rn), Reg(rm), Cond(c)])
    rhs: llvm_mc("csneg {rd}, {rn}, {rm}, {c}")
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; encoder/mod.rs:311 csneg dispatch; ARM ARM CSNEG sf 1 0 11010100 Rm cond 01 Rn Rd
```

## encode_csneg_meta_vs_csinc
- Tier: 3
- Rationale: ARM ARM Conditional Select family: CSNEG is CSINC with op=1 instead of 0, so the encoded words differ only at bit 30. Stronger Differential already used for the valid domain; this metamorphic checks the architectural sibling relation independently of llvm-mc. encode_csinc is not same-job (different mnemonic/op) so not a differential reference.
- Seed: encode_csinc_pbt::encode_csinc_meta_vs_csel (compare_branch.rs:9436)
- Formal: ∀ rd, rn, rm ∈ SameWidthGpr, c ∈ Cond16. encode_csneg(ops) XOR encode_csinc(ops) = 1<<30 where ops = [Reg(rd), Reg(rn), Reg(rm), Cond(c)]
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: cond16 }
  body: encode_csneg(ops) XOR encode_csinc(ops) == (1u32 << 30)
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
evidence: ARM ARM CSINC op=0 vs CSNEG op=1 at bit 30; compare_branch.rs:107 CSINC word vs compare_branch.rs:136 CSNEG word
```

## encode_csneg_meta_vs_cneg
- Tier: 3
- Rationale: ARM ARM CNEG is the documented alias CSNEG Rd, Rn, Rn, invert(cond) (invalid for AL/NV). Metamorphic: encode_csneg([Rd, Rn, Rn, invert(c)]) equals encode_cneg([Rd, Rn, c]) over Cond14. encode_cneg is not same-job (3-operand alias) so not a differential reference.
- Seed: encode_cneg_pbt::encode_cneg_meta_vs_csneg (compare_branch.rs:6884)
- Formal: ∀ rd, rn ∈ SameWidthGpr, c ∈ Cond14. encode_csneg([Reg(rd), Reg(rn), Reg(rn), Cond(invert(c))]) = encode_cneg([Reg(rd), Reg(rn), Cond(c)])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, c: cond14 }
  relation:
    op: eq
    lhs: encode_csneg([Reg(rd), Reg(rn), Reg(rn), Cond(invert(c))])
    rhs: encode_cneg([Reg(rd), Reg(rn), Cond(c)])
generators:
  rd: { gen: string }
  rn: { gen: string }
  c: { gen: string }
evidence: compare_branch.rs:275 CNEG Rd, Rn, cond -> CSNEG Rd, Rn, Rn, invert(cond); ARM ARM Conditional Negate alias of CSNEG
```

## encode_csneg_word_layout
- Tier: 4
- Rationale: ARM ARM CSNEG field layout is an exact structural invariant of the success-path word. Stronger Differential already used; this pins each field independently of llvm-mc. Predicate from ARM ARM encoding, not from the SUT body.
- Seed: encode_csinc_pbt::encode_csinc_word_layout (compare_branch.rs:9505)
- Formal: ∀ rd_n, rn_n, rm_n ∈ 0..31, is_64 ∈ Bool, cond_enc ∈ 0..15. encode_csneg([Reg(gpr(rd_n,is_64)), Reg(gpr(rn_n,is_64)), Reg(gpr(rm_n,is_64)), Cond(COND16[cond_enc])]) = Word(w) ∧ w[31]=sf(is_64) ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=0b11010100 ∧ w[20:16]=rm_n ∧ w[15:12]=cond_enc ∧ w[11:10]=01 ∧ w[9:5]=rn_n ∧ w[4:0]=rd_n
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, rn_n, rm_n, is_64, cond_enc]
  domain: { rd_n: 0..31, rn_n: 0..31, rm_n: 0..31, is_64: bool, cond_enc: 0..15 }
  body: Word fields match ARM ARM CSNEG layout
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  rn_n: { gen: int, min: 0, max: 31, type: u32 }
  rm_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  cond_enc: { gen: int, min: 0, max: 15, type: u32 }
evidence: ARM ARM CSNEG sf 1 0 11010100 Rm cond 01 Rn Rd; compare_branch.rs:286
```

## encode_csneg_neg_arity
- Tier: 4
- Rationale: llvm-mc and ARM ARM require four operands (Rd, Rn, Rm, cond). Fewer than 4 must Err. Documented error contract from llvm-mc "too few operands" and SUT `get_reg`/`csneg requires condition`.
- Seed: encode_csinc_pbt::encode_csinc_neg_arity (compare_branch.rs:9547)
- Formal: ∀ arity ∈ 0..3. encode_csneg(ops) is Err when |ops| = arity
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..3 }
  relation:
    op: throws
    expr: encode_csneg(ops_of_len(arity))
generators:
  arity: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects csneg with fewer than 4 operands; compare_branch.rs:128-133 get_reg / csneg requires condition
```

## encode_csneg_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a fifth operand ("invalid operand for instruction"). Gas-compat README.md:5-14. Documented error contract: extra operand must Err.
- Seed: encode_csinc_pbt::encode_csinc_neg_extra_operand (compare_branch.rs:9566)
- Formal: ∀ rd, rn, rm ∈ SameWidthGpr, c ∈ Cond16, extra ∈ ExtraOperand. encode_csneg([Reg(rd), Reg(rn), Reg(rm), Cond(c), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x0"), Reg("x0"), Cond("eq"), Reg("x3")]
- Bug report: pbt-out/bug_reports/encode_csneg_extra_operand.md

```property
function: encoder.compare_branch.encode_csneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c, extra]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: cond16, extra: extra_operand }
  relation:
    op: throws
    expr: encode_csneg([Reg(rd), Reg(rn), Reg(rm), Cond(c), extra])
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects csneg x0, x1, x2, eq, x3; README.md:5-14 gas-compat
```

## encode_csneg_neg_wrong_reg
- Tier: 4
- Rationale: ARM ARM CSNEG takes Wt/Xt only; register 31 is XZR/WZR never SP/WSP; same-width GPRs. llvm-mc rejects SP, WSP, FP/SIMD, mixed x/w, and invalid names. Documented error contract.
- Seed: encode_csinc_pbt::encode_csinc_neg_wrong_reg (compare_branch.rs:9587)
- Formal: ∀ kind ∈ WrongRegKind, n ∈ 0..31. encode_csneg(bad_ops(kind, n)) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind=0 n=0 (csneg sp, x0, x0, eq); also mixed x/w, FP d0/s0/q0, wsp
- Bug report: pbt-out/bug_reports/encode_csneg_sp_as_zr.md, pbt-out/bug_reports/encode_csneg_mixed_width.md, pbt-out/bug_reports/encode_csneg_fp_as_gpr.md

```property
function: encoder.compare_branch.encode_csneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: 0..8, n: 0..31 }
  relation:
    op: throws
    expr: encode_csneg(bad_ops(kind, n))
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects csneg sp / wsp / d0 / mixed x/w / x32; ARM ARM CSNEG Wt/Xt, register 31 is XZR/WZR
```

## encode_csneg_neg_unknown_cond
- Tier: 4
- Rationale: encode_cond returns None for names outside the 16 architectural conditions (plus hs/lo aliases). llvm-mc "invalid condition code". Documented error contract.
- Seed: encode_csinc_pbt::encode_csinc_neg_unknown_cond (compare_branch.rs:9598)
- Formal: ∀ rd, rn, rm ∈ SameWidthGpr, c ∈ {zz, foo, eqq, "", "eq ", always}. encode_csneg([Reg(rd), Reg(rn), Reg(rm), Cond(c)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, c]
  domain: { rd: same_width_gpr, rn: same_width_gpr, rm: same_width_gpr, c: unknown_cond }
  relation:
    op: throws
    expr: encode_csneg([Reg(rd), Reg(rn), Reg(rm), Cond(c)])
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  c: { gen: string }
expected_error: String
evidence: encoder/mod.rs:169-190 encode_cond None arm; llvm-mc invalid condition code
```

## encode_csneg_neg_invalid_name
- Tier: 4
- Rationale: Coverage-sweep of get_reg parse_reg_num None arm. Names outside x0-x30/w0-w30/xzr/wzr/lr/sp must Err. llvm-mc rejects x32/foo/empty. Documented error contract from parse_reg_num returning None.
- Seed: encode_csinc_pbt::encode_csinc_neg_invalid_name (coverage sweep)
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_csneg([Reg(name), Reg("x0"), Reg("x1"), Cond("eq")]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_csneg([Reg(name), Reg("x0"), Reg("x1"), Cond("eq")])
generators:
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-148 parse_reg_num None; llvm-mc rejects x32/foo
```

## encode_csneg_neg_bad_operand_kind
- Tier: 4
- Rationale: Coverage-sweep of get_reg non-Reg arm and cond-not-Cond arm. Imm/Mem/Symbol/Shift/Label in any of the four slots must Err. Documented by get_reg expected-register and "csneg requires condition".
- Seed: encode_csinc_pbt::encode_csinc_neg_bad_operand_kind (coverage sweep)
- Formal: ∀ slot ∈ 0..3, kind ∈ {Imm, Mem, Symbol, Shift, Label}. encode_csneg(ops with slot replaced by kind) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, kind]
  domain: { slot: 0..3, kind: 0..4 }
  relation:
    op: throws
    expr: encode_csneg(ops_with_slot_replaced(slot, kind))
generators:
  slot: { gen: int, min: 0, max: 3, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956-966 get_reg non-Reg; compare_branch.rs:131-133 cond-not-Cond
```
