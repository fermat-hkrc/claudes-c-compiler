# Properties: encode_cneg

## encode_cneg_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (gas-compat README). State machine rejected (pure function, no lifecycle). Round-trip rejected (no CNEG decoder). encode_csneg fails the same-job sibling gate as a differential reference (4-operand architectural CSNEG, no invert); used only as a metamorphic transform. encode_cinc / encode_cinv are different jobs (CSINC / CSINV aliases).
- Seed: encode_cinc_pbt encode_cinc_diff_llvm_mc (compare_branch.rs)
- Formal: ∀ (rd, rn) same-width GPR (x0–x30/xzr/lr or w0–w30/wzr), ∀ cond ∈ Cond14∪{hs,lo}. llvm-mc("cneg rd, rn, cond") succeeds ⇒ encode_cneg([Reg(rd), Reg(rn), Cond(cond)]) = Word(llvm-mc word).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cneg
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: gpr, rn: same_width_gpr, cond: cond14 }
  relation:
    op: eq
    lhs: encode_cneg([Reg(rd), Reg(rn), Cond(cond)])
    rhs: llvm_mc_word("cneg " + rd + ", " + rn + ", " + cond)
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; encoder/mod.rs:897 cneg dispatch; compare_branch.rs:275 CNEG->CSNEG invert(cond)
```

## encode_cneg_meta_vs_csneg
- Tier: 4
- Rationale: ARM ARM defines CNEG as the CSNEG alias with Rm=Rn and invert(cond). encode_csneg is independently implemented (compare_branch.rs:127) and is a same-job architectural expansion, not a copy of encode_cneg. Stronger differential vs llvm-mc is already property 1; this is the required metamorphic angle.
- Seed: encode_cinc_pbt encode_cinc_meta_vs_csinc
- Formal: ∀ (rd, rn) same-width GPR, ∀ cond ∈ Cond14∪{hs,lo}. encode_cneg([Rd, Rn, cond]) = encode_csneg([Rd, Rn, Rn, invert(cond)]). invert(eq)=ne, invert(hs)=lo, invert(al)=nv, etc. (XOR 1 on the 4-bit cond encoding).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cneg
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: gpr, rn: same_width_gpr, cond: cond14 }
  relation:
    op: eq
    lhs: encode_cneg([Reg(rd), Reg(rn), Cond(cond)])
    rhs: encode_csneg([Reg(rd), Reg(rn), Reg(rn), Cond(invert(cond))])
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
evidence: compare_branch.rs:275 CNEG Rd, Rn, cond -> CSNEG Rd, Rn, Rn, invert(cond); ARM ARM Conditional Negate alias of CSNEG
```

## encode_cneg_word_layout
- Tier: 4
- Rationale: ARM ARM CSNEG field layout is an exact structural invariant on every success-path word. Weaker than differential/metamorphic but pins each field independently (sf, op=1, S=0, bits[28:21]=11010100, Rm=Rn, invert(cond), op2=01, Rd).
- Seed: encode_cinc_pbt encode_cinc_word_layout
- Formal: ∀ rd_n, rn_n ∈ 0..31, ∀ is_64 ∈ {0,1}, ∀ cond_enc ∈ 0..13. encode_cneg([gpr(rd_n,is_64), gpr(rn_n,is_64), Cond14[cond_enc]]) = Word(w) where w[31]=is_64, w[30]=1, w[29]=0, w[28:21]=0b11010100, w[20:16]=rn_n, w[15:12]=cond_enc XOR 1, w[11:10]=0b01, w[9:5]=rn_n, w[4:0]=rd_n.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cneg
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, rn_n, is_64, cond_enc]
  domain: { rd_n: 0..31, rn_n: 0..31, is_64: bool, cond_enc: 0..13 }
  relation:
    op: holds
    expr: word_fields_match_csneg_alias(encode_cneg([gpr(rd_n, is_64), gpr(rn_n, is_64), Cond14[cond_enc]]))
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  rn_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  cond_enc: { gen: int, min: 0, max: 13, type: u32 }
evidence: compare_branch.rs:283-286 CSNEG sf 1 0 11010100 Rm cond 0 1 Rn Rd with Rm=Rn; ARM ARM CSNEG
```

## encode_cneg_neg_arity
- Tier: 4
- Rationale: llvm-mc rejects CNEG with fewer than 3 operands ("too few operands"). Documented error contract of the gas-compat assembler. Bounds 0, 1, 2 sampled exactly.
- Seed: encode_cinc_pbt encode_cinc_neg_arity
- Formal: ∀ arity ∈ {0,1,2}. encode_cneg(ops) is Err when |ops| = arity.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..2 }
  relation:
    op: throws
    expr: encode_cneg(ops_of_len(arity))
generators:
  arity: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects bare cneg / cneg x0 / cneg x0, x1 (too few operands)
```

## encode_cneg_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a fourth operand ("invalid operand"). Gas-compat contract. Extra kinds: Reg/Imm/Symbol/Mem.
- Seed: encode_cinc_pbt encode_cinc_neg_extra_operand
- Formal: ∀ (rd, rn) same-width GPR, ∀ cond ∈ Cond14, ∀ extra ∈ {Reg, Imm, Symbol, Mem}. encode_cneg([Rd, Rn, Cond, extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x0"), Cond("eq"), Reg("x2")]  (rd="x0", rn="x0", cond="eq", which=0)
- Bug report: pbt-out/bug_reports/encode_cneg_extra_operand.md

```property
function: encoder.compare_branch.encode_cneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, cond, extra]
  domain: { rd: gpr, rn: same_width_gpr, cond: cond14, extra: extra_operand }
  relation:
    op: throws
    expr: encode_cneg([Reg(rd), Reg(rn), Cond(cond), extra])
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects cneg x0, x1, eq, x2 (invalid operand)
```

## encode_cneg_neg_al_nv
- Tier: 4
- Rationale: ARM ARM and llvm-mc: condition codes AL and NV are invalid for CNEG. Bounds al and nv sampled exactly.
- Seed: encode_cinc_pbt encode_cinc_neg_al_nv
- Formal: ∀ (rd, rn) same-width GPR, ∀ cond ∈ {al, nv}. encode_cneg([Rd, Rn, Cond(cond)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x0"), Cond("al")]  (rd="x0", rn="x0", which=0)
- Bug report: pbt-out/bug_reports/encode_cneg_al_nv.md

```property
function: encoder.compare_branch.encode_cneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: gpr, rn: same_width_gpr, cond: {al, nv} }
  relation:
    op: throws
    expr: encode_cneg([Reg(rd), Reg(rn), Cond(cond)])
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
expected_error: String
evidence: llvm-mc -triple=aarch64 "condition codes AL and NV are invalid for this instruction"; ARM ARM CNEG not valid for AL/NV
```

## encode_cneg_neg_wrong_reg
- Tier: 4
- Rationale: llvm-mc rejects SP/WSP (register 31 is XZR/WZR), mixed x/w, FP/SIMD (d/s/q/v), and invalid names (x32, w32, foo, empty, r0, x, x-1, x99). Gas-compat contract.
- Seed: encode_cinc_pbt encode_cinc_neg_wrong_reg
- Formal: ∀ kind ∈ {sp-rd, sp-rn, wsp-rd, mixed-x-w, d-reg, s-reg, q-reg, v-reg, invalid-name}, ∀ n ∈ 0..31. encode_cneg(bad_ops(kind, n)) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind=0, n=0  ([Reg("sp"), Reg("x0"), Cond("eq")]); also mixed [Reg("x0"), Reg("w0"), Cond("eq")] and FP [Reg("d0"), Reg("d0"), Cond("eq")]
- Bug report: pbt-out/bug_reports/encode_cneg_sp_as_zr.md; pbt-out/bug_reports/encode_cneg_mixed_width.md; pbt-out/bug_reports/encode_cneg_fp_reg.md

```property
function: encoder.compare_branch.encode_cneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: 0..8, n: 0..31 }
  relation:
    op: throws
    expr: encode_cneg(bad_ops(kind, n))
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects cneg sp, x0, eq; cneg x0, w1, eq; cneg d0, d1, eq
```

## encode_cneg_neg_bad_operand_kind
- Tier: 4
- Rationale: get_reg requires Operand::Reg at slots 0 and 1; slot 2 must be Operand::Cond. Non-Reg/non-Cond kinds (Imm/Mem/Symbol/Shift/Label) must Err. Coverage of the match-arm error paths.
- Seed: encode_cinc_pbt encode_cinc_neg_bad_operand_kind
- Formal: ∀ slot ∈ {0,1,2}, ∀ kind ∈ {Imm, Mem, Symbol, Shift, Label}. encode_cneg(ops with slot replaced by that kind) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, which]
  domain: { slot: 0..2, which: 0..4 }
  relation:
    op: throws
    expr: encode_cneg(ops_with_bad_kind(slot, which))
generators:
  slot: { gen: int, min: 0, max: 2, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956-966 get_reg requires Operand::Reg; compare_branch.rs:279-281 third operand must be Cond
```

## encode_cneg_neg_unknown_cond
- Tier: 4
- Rationale: Coverage sweep of encode_cond None arm. Unknown condition names must Err (llvm-mc and encode_cond both reject them). Documented error path at compare_branch.rs:279.
- Seed: encode_cinc_pbt encode_cinc_neg_unknown_cond
- Formal: ∀ (rd, rn) same-width GPR, ∀ cond ∈ {zz, foo, eqq, "", "eq ", always}. encode_cneg([Rd, Rn, Cond(cond)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, which]
  domain: { rd: gpr, rn: same_width_gpr, which: 0..5 }
  relation:
    op: throws
    expr: encode_cneg([Reg(rd), Reg(rn), Cond(unknown_cond(which))])
generators:
  rd: { gen: string }
  rn: { gen: string }
  which: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: compare_branch.rs:279 encode_cond(c).ok_or_else unknown condition
```

## encode_cneg_neg_invalid_name
- Tier: 4
- Rationale: Coverage sweep of get_reg parse_reg_num None arm. Invalid register names (x32, w32, foo, empty, r0, x, x-1, x99) must Err. Bound x32 is 31+1.
- Seed: encode_cinc_pbt encode_cinc_neg_invalid_name
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_cneg([Reg(name), Reg("x0"), Cond("eq")]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cneg
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which]
  domain: { which: 0..7 }
  relation:
    op: throws
    expr: encode_cneg([Reg(invalid_name(which)), Reg("x0"), Cond("eq")])
generators:
  which: { gen: int, min: 0, max: 7, type: u32 }
expected_error: String
evidence: encoder/mod.rs:131-148 parse_reg_num returns None for x32/non-prefix; get_reg maps that to Err
```
