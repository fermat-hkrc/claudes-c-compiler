# Properties: encode_cinv

## encode_cinv_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential against llvm-mc (gas-compat README.md:5-14). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree CINV decoder. encode_csinv fails same-job sibling gate (different mnemonic/arity) so is not the differential reference.
- Seed: compare_branch.rs encode_cinc_diff_llvm_mc
- Formal: ∀ rd, rn ∈ same-width GPR (x0–x30/xzr/lr or w0–w30/wzr), ∀ cond ∈ Cond14∪{hs,lo}. encode_cinv([Reg(rd), Reg(rn), Cond(cond)]) = Word(w) ∧ w = llvm-mc("cinv rd, rn, cond")
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: same_width_gpr, rn: same_width_gpr, cond: cond14_with_aliases }
  relation:
    op: eq
    lhs: encode_cinv([Reg(rd), Reg(rn), Cond(cond)])
    rhs: llvm_mc("cinv {rd}, {rn}, {cond}")
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; encoder/mod.rs:899 cinv dispatch; ARM ARM CINV alias of CSINV
```

## encode_cinv_meta_vs_csinv
- Tier: 3
- Rationale: Documented alias: CINV Rd, Rn, cond -> CSINV Rd, Rn, Rn, invert(cond) (compare_branch.rs:308). Metamorphic, not differential: encode_csinv has a different job (4-operand CSINV). Round-trip rejected: no decoder.
- Seed: compare_branch.rs encode_cinc_meta_vs_csinc
- Formal: ∀ rd, rn ∈ same-width GPR, ∀ cond ∈ Cond14∪{hs,lo}. encode_cinv([Rd, Rn, cond]) = encode_csinv([Rd, Rn, Rn, invert(cond)])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: same_width_gpr, rn: same_width_gpr, cond: cond14_with_aliases }
  relation:
    op: eq
    lhs: encode_cinv([Reg(rd), Reg(rn), Cond(cond)])
    rhs: encode_csinv([Reg(rd), Reg(rn), Reg(rn), Cond(invert(cond))])
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
evidence: compare_branch.rs:308 CINV Rd, Rn, cond -> CSINV Rd, Rn, Rn, invert(cond); ARM ARM CINV alias of CSINV
```

## encode_cinv_meta_vs_csetm
- Tier: 3
- Rationale: CSETM Rd, cond is CSINV Rd, XZR, XZR, invert(cond), which is CINV Rd, ZR, cond (compare_branch.rs:155). Same-width ZR co-generated from Rd.
- Seed: compare_branch.rs encode_cinc_meta_vs_cset
- Formal: ∀ rd ∈ GPR, ∀ cond ∈ Cond14∪{hs,lo}. encode_cinv([Rd, ZR(rd), cond]) = encode_csetm([Rd, cond])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, cond]
  domain: { rd: gpr, cond: cond14_with_aliases }
  relation:
    op: eq
    lhs: encode_cinv([Reg(rd), Reg(zr_of(rd)), Cond(cond)])
    rhs: encode_csetm([Reg(rd), Cond(cond)])
generators:
  rd: { gen: string }
  cond: { gen: string }
evidence: compare_branch.rs:155 CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond); ARM ARM CSETM alias
```

## encode_cinv_word_layout
- Tier: 4
- Rationale: ARM ARM CSINV field layout is an exact structural invariant of a successful encode. Weaker than differential/metamorphic but pins each bit field independently.
- Seed: compare_branch.rs encode_cinc_word_layout
- Formal: ∀ rd_n, rn_n ∈ 0..31, ∀ is_64 ∈ Bool, ∀ cond_enc ∈ 0..13. encode_cinv([gpr(rd_n,is_64), gpr(rn_n,is_64), Cond(COND14[cond_enc])]) = Word(w) ∧ w[31]=sf ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=0b11010100 ∧ w[20:16]=rn_n ∧ w[15:12]=cond_enc XOR 1 ∧ w[11:10]=00 ∧ w[9:5]=rn_n ∧ w[4:0]=rd_n
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, rn_n, is_64, cond_enc]
  domain: { rd_n: 0..31, rn_n: 0..31, is_64: bool, cond_enc: 0..13 }
  body: fields of encode_cinv Word match ARM ARM CSINV with Rm=Rn and invert(cond)
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  rn_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  cond_enc: { gen: int, min: 0, max: 13, type: u32 }
evidence: compare_branch.rs:317 CSINV sf 1 0 11010100 Rm cond 0 0 Rn Rd; ARM ARM Conditional select
```

## encode_cinv_neg_arity
- Tier: 4
- Rationale: llvm-mc / gas reject too few operands. Documented error contract for the assembler (README.md:5-14).
- Seed: compare_branch.rs encode_cinc_neg_arity
- Formal: ∀ ops with |ops| ∈ {0,1,2}. encode_cinv(ops) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..2 }
  relation:
    op: holds
    expr: encode_cinv(ops_of_arity(arity)).is_err()
generators:
  arity: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: llvm-mc too few operands; README.md:5-14 gas-compat
```

## encode_cinv_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a fourth operand. Gas-compat requires Err, not silent drop.
- Seed: compare_branch.rs encode_cinc_neg_extra_operand
- Formal: ∀ rd, rn ∈ same-width GPR, ∀ cond ∈ Cond14∪{hs,lo}, ∀ extra ∈ {Reg, Imm, Symbol, Mem}. encode_cinv([Rd, Rn, Cond, extra]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x0"), Cond("eq"), Reg("x2")]
- Bug report: pbt-out/bug_reports/encode_cinv_extra_operand.md

```property
function: encoder.compare_branch.encode_cinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, cond, extra]
  domain: { rd: same_width_gpr, rn: same_width_gpr, cond: cond14_with_aliases, extra: extra_operand }
  relation:
    op: holds
    expr: encode_cinv([Reg(rd), Reg(rn), Cond(cond), extra]).is_err()
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand for fourth arg; README.md:5-14 gas-compat
```

## encode_cinv_neg_al_nv
- Tier: 4
- Rationale: ARM ARM CINV alias is not valid for cond AL or NV. llvm-mc: "condition codes AL and NV are invalid for this instruction".
- Seed: compare_branch.rs encode_cinc_neg_al_nv
- Formal: ∀ rd, rn ∈ same-width GPR, ∀ cond ∈ {al, nv}. encode_cinv([Rd, Rn, Cond(cond)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x0"), Cond("al")]
- Bug report: pbt-out/bug_reports/encode_cinv_al_nv.md

```property
function: encoder.compare_branch.encode_cinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: same_width_gpr, rn: same_width_gpr, cond: {al, nv} }
  relation:
    op: holds
    expr: encode_cinv([Reg(rd), Reg(rn), Cond(cond)]).is_err()
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
expected_error: String
evidence: ARM ARM CINV not a valid alias for AL/NV; llvm-mc error text
```

## encode_cinv_neg_wrong_reg
- Tier: 4
- Rationale: llvm-mc rejects SP/WSP (reg 31 is XZR/WZR), mixed x/w, FP/SIMD prefixes, and invalid names. Gas-compat negative contract.
- Seed: compare_branch.rs encode_cinc_neg_wrong_reg
- Formal: ∀ kind ∈ {sp-rd, sp-rn, wsp-rd, mixed-xw, d-reg, s-reg, q-reg, v-reg, invalid-name}, ∀ n ∈ 0..31. encode_cinv(bad_ops(kind, n)) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind=0 n=0 ([Reg("sp"), Reg("x0"), Cond("eq")]); also mixed x/w and FP names
- Bug report: pbt-out/bug_reports/encode_cinv_sp_as_zr.md; pbt-out/bug_reports/encode_cinv_mixed_width.md; pbt-out/bug_reports/encode_cinv_fp_reg.md

```property
function: encoder.compare_branch.encode_cinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: 0..8, n: 0..31 }
  relation:
    op: holds
    expr: encode_cinv(bad_ops(kind, n)).is_err()
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc invalid operand for SP/FP/mixed; README.md:5-14 gas-compat
```

## encode_cinv_neg_unknown_cond
- Tier: 4
- Rationale: Coverage sweep of encode_cond None arm. Unknown condition names must Err (encode_cond returns None).
- Seed: compare_branch.rs encode_cinc_neg_unknown_cond
- Formal: ∀ rd, rn ∈ same-width GPR, ∀ cond ∈ {zz, foo, eqq, "", "eq ", always}. encode_cinv([Rd, Rn, Cond(cond)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, which]
  domain: { rd: same_width_gpr, rn: same_width_gpr, which: 0..5 }
  relation:
    op: holds
    expr: encode_cinv([Reg(rd), Reg(rn), Cond(unknown_cond(which))]).is_err()
generators:
  rd: { gen: string }
  rn: { gen: string }
  which: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: encoder/mod.rs:169 encode_cond returns None for unknown names; compare_branch.rs:313 unknown condition
```

## encode_cinv_neg_invalid_name
- Tier: 4
- Rationale: Coverage sweep of get_reg parse_reg_num None arm. Invalid register names must Err.
- Seed: compare_branch.rs encode_cinc_neg_invalid_name
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_cinv([Reg(name), Reg("x0"), Cond("eq")]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which]
  domain: { which: 0..7 }
  relation:
    op: holds
    expr: encode_cinv([Reg(invalid_name(which)), Reg("x0"), Cond("eq")]).is_err()
generators:
  which: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: encoder/mod.rs:131 parse_reg_num returns None for x32/foo/empty; get_reg maps that to Err
```

## encode_cinv_neg_bad_operand_kind
- Tier: 4
- Rationale: Coverage sweep of get_reg non-Reg and cond-not-Cond arms. Imm/Mem/Symbol/Shift/Label in any of the three slots must Err.
- Seed: compare_branch.rs encode_cinc_neg_bad_operand_kind
- Formal: ∀ slot ∈ {0,1,2}, ∀ kind ∈ {Imm, Mem, Symbol, Shift, Label}. encode_cinv(ops with slot replaced by that kind) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, which]
  domain: { slot: 0..2, which: 0..4 }
  relation:
    op: holds
    expr: encode_cinv(ops_with_bad_kind(slot, which)).is_err()
generators:
  slot: { gen: int, min: 0, max: 2, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956 get_reg requires Operand::Reg; compare_branch.rs:312 third operand must be Cond
```
