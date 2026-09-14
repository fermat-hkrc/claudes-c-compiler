# Properties: encode_csel

## encode_csel_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (gas-compat README). State machine rejected (pure function, no lifecycle). Round-trip rejected (no CSEL decoder). encode_csinc / encode_csinv fail the same-job sibling gate as differential references (different mnemonics and op2/op bits); used only as metamorphic transforms.
- Seed: encode_cinc_pbt encode_cinc_diff_llvm_mc (compare_branch.rs)
- Formal: ∀ (rd, rn, rm) same-width GPR (x0–x30/xzr/lr or w0–w30/wzr), ∀ cond ∈ Cond16∪{hs,lo}. llvm-mc("csel rd, rn, rm, cond") succeeds ⇒ encode_csel([Reg(rd), Reg(rn), Reg(rm), Cond(cond)]) = Word(llvm-mc word).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, cond]
  domain: { rd: gpr, rn: same_width_gpr, rm: same_width_gpr, cond: cond16 }
  relation:
    op: eq
    lhs: encode_csel([Reg(rd), Reg(rn), Reg(rm), Cond(cond)])
    rhs: llvm_mc_word("csel " + rd + ", " + rn + ", " + rm + ", " + cond)
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  cond: { gen: string }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; encoder/mod.rs:308 csel dispatch; ARM ARM CSEL sf 0 0 11010100 Rm cond 00 Rn Rd
```

## encode_csel_meta_vs_csinc
- Tier: 4
- Rationale: ARM ARM CSEL and CSINC share the Conditional Select encoding class and differ only in op2[11:10] (00 vs 01). encode_csinc is independently implemented (compare_branch.rs:99) and is a different mnemonic, so it fails the same-job sibling gate as a differential reference; the XOR-bit-10 relation is the required metamorphic angle. Stronger differential vs llvm-mc is already property 1.
- Seed: encode_cinc_pbt encode_cinc_meta_vs_csinc
- Formal: ∀ (rd, rn, rm) same-width GPR, ∀ cond ∈ Cond16∪{hs,lo}. encode_csel([Rd, Rn, Rm, cond]) XOR encode_csinc([Rd, Rn, Rm, cond]) = 1<<10.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, cond]
  domain: { rd: gpr, rn: same_width_gpr, rm: same_width_gpr, cond: cond16 }
  relation:
    op: eq
    lhs: encode_csel([Reg(rd), Reg(rn), Reg(rm), Cond(cond)]) XOR encode_csinc([Reg(rd), Reg(rn), Reg(rm), Cond(cond)])
    rhs: 1 << 10
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  cond: { gen: string }
evidence: ARM ARM CSEL op2=00 vs CSINC op2=01; compare_branch.rs:99 encode_csinc independently implemented; llvm-mc csel x0,x1,x2,eq=0x9a820020 csinc=0x9a820420
```

## encode_csel_meta_vs_csinv
- Tier: 4
- Rationale: ARM ARM CSEL and CSINV share the Conditional Select encoding class and differ only in op[30] (0 vs 1). encode_csinv is independently implemented (compare_branch.rs:113). Stronger differential vs llvm-mc is already property 1; this is a second metamorphic angle covering the op bit.
- Seed: encode_cinc_pbt encode_cinc_meta_vs_csinc
- Formal: ∀ (rd, rn, rm) same-width GPR, ∀ cond ∈ Cond16∪{hs,lo}. encode_csel([Rd, Rn, Rm, cond]) XOR encode_csinv([Rd, Rn, Rm, cond]) = 1<<30.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, cond]
  domain: { rd: gpr, rn: same_width_gpr, rm: same_width_gpr, cond: cond16 }
  relation:
    op: eq
    lhs: encode_csel([Reg(rd), Reg(rn), Reg(rm), Cond(cond)]) XOR encode_csinv([Reg(rd), Reg(rn), Reg(rm), Cond(cond)])
    rhs: 1 << 30
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  cond: { gen: string }
evidence: ARM ARM CSEL op=0 vs CSINV op=1; compare_branch.rs:113 encode_csinv independently implemented; llvm-mc csel x0,x1,x2,eq=0x9a820020 csinv=0xda820020
```

## encode_csel_word_layout
- Tier: 4
- Rationale: ARM ARM CSEL field layout is an exact structural invariant on every success-path word. Weaker than differential/metamorphic but pins each field independently (sf, op=0, S=0, bits[28:21]=11010100, Rm, cond, op2=00, Rn, Rd). Documented bounds 0..31 for Rd/Rn/Rm and 0..15 for cond are sampled exactly.
- Seed: encode_cinc_pbt encode_cinc_word_layout
- Formal: ∀ rd_n, rn_n, rm_n ∈ 0..31, ∀ is_64 ∈ {0,1}, ∀ cond_enc ∈ 0..15. encode_csel([gpr(rd_n,is_64), gpr(rn_n,is_64), gpr(rm_n,is_64), Cond16[cond_enc]]) = Word(w) where w[31]=is_64, w[30]=0, w[29]=0, w[28:21]=0b11010100, w[20:16]=rm_n, w[15:12]=cond_enc, w[11:10]=0b00, w[9:5]=rn_n, w[4:0]=rd_n.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, rn_n, rm_n, is_64, cond_enc]
  domain: { rd_n: 0..31, rn_n: 0..31, rm_n: 0..31, is_64: bool, cond_enc: 0..15 }
  relation:
    op: holds
    expr: word_fields_match_csel(encode_csel([gpr(rd_n, is_64), gpr(rn_n, is_64), gpr(rm_n, is_64), Cond16[cond_enc]]))
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  rn_n: { gen: int, min: 0, max: 31, type: u32 }
  rm_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  cond_enc: { gen: int, min: 0, max: 15, type: u32 }
evidence: ARM ARM CSEL encoding sf 0 0 11010100 Rm cond 00 Rn Rd; encoder/mod.rs:1-7 32-bit AArch64 words
```

## encode_csel_neg_arity
- Tier: 4
- Rationale: llvm-mc rejects CSEL with fewer than 4 operands ("too few operands"). Negative/error contract from the gas-compat README. Stronger oracles do not apply to the invalid-arity domain.
- Seed: encode_cinc_pbt encode_cinc_neg_arity
- Formal: ∀ arity ∈ {0,1,2,3}. encode_csel(ops) is Err when |ops| = arity (missing Rd/Rn/Rm/cond).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..3 }
  relation:
    op: throws
    lhs: encode_csel(ops_of_len(arity))
    error: String
generators:
  arity: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; README.md:5-14 gas-compat
```

## encode_csel_neg_extra_operand
- Tier: 4
- Rationale: llvm-mc rejects a fifth operand ("invalid operand"). Gas-compat contract requires Err. Stronger oracles do not apply to the extra-operand domain.
- Seed: encode_cinc_pbt encode_cinc_neg_extra_operand
- Formal: ∀ (rd, rn, rm) same-width GPR, ∀ cond ∈ Cond16∪{hs,lo}, ∀ extra ∈ {Reg, Imm, Symbol, Mem}. encode_csel([Rd, Rn, Rm, cond, extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x1"), Reg("x2"), Cond("eq"), Reg("x3")]
- Bug report: pbt-out/bug_reports/encode_csel_extra_operand.md

```property
function: encoder.compare_branch.encode_csel
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, cond, extra]
  domain: { rd: gpr, rn: same_width_gpr, rm: same_width_gpr, cond: cond16, extra: Operand }
  relation:
    op: throws
    lhs: encode_csel([Reg(rd), Reg(rn), Reg(rm), Cond(cond), extra])
    error: String
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  cond: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" for csel x0, x1, x2, eq, x3; README.md:5-14 gas-compat
```

## encode_csel_neg_wrong_reg
- Tier: 4
- Rationale: llvm-mc rejects SP/WSP (register 31 is XZR/WZR), mixed x/w widths, FP/SIMD names, and invalid register names. Gas-compat contract requires Err. Documented bounds: x0..x30/xzr, w0..w30/wzr; SP and FP are outside the valid domain.
- Seed: encode_cinc_pbt encode_cinc_neg_wrong_reg
- Formal: ∀ kind ∈ {sp-Rd, sp-Rn, sp-Rm, wsp-Rd, mixed-x/w, d-reg, s-reg, q-reg, invalid-name}. encode_csel(ops(kind)) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind=0 n=0 → [Reg("sp"), Reg("x0"), Reg("x0"), Cond("eq")]; also mixed [Reg("x0"), Reg("w1"), Reg("x2"), Cond("eq")]; also FP [Reg("d0"), Reg("d1"), Reg("d2"), Cond("eq")]
- Bug report: pbt-out/bug_reports/encode_csel_sp_as_zr.md; pbt-out/bug_reports/encode_csel_mixed_width.md; pbt-out/bug_reports/encode_csel_fp_reg.md

```property
function: encoder.compare_branch.encode_csel
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: 0..8, n: 0..31 }
  relation:
    op: throws
    lhs: encode_csel(bad_ops(kind, n))
    error: String
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects csel sp / mixed x/w / d0; ARM ARM CSEL Wt/Xt only, register 31 is XZR/WZR
```

## encode_csel_neg_bad_operand_kind
- Tier: 4
- Rationale: get_reg requires Operand::Reg at slots 0..2; slot 3 must be Operand::Cond. Imm/Mem/Symbol/Shift/Label in any of the four slots is outside the CSEL grammar. llvm-mc rejects non-GPR / missing-cond forms. Stronger oracles do not apply to this invalid-kind domain.
- Seed: encode_cinc_pbt encode_cinc_neg_bad_operand_kind
- Formal: ∀ slot ∈ {0,1,2,3}, ∀ which ∈ {Imm, Mem, Symbol, Shift, Label}. encode_csel(ops with that kind at slot) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, which]
  domain: { slot: 0..3, which: 0..4 }
  relation:
    op: throws
    lhs: encode_csel(ops_with_bad_kind(slot, which))
    error: String
generators:
  slot: { gen: int, min: 0, max: 3, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: encoder/mod.rs:956 get_reg expected register; compare_branch.rs:91 csel requires condition; README.md:5-14 gas-compat
```

## encode_csel_neg_unknown_cond
- Tier: 4
- Rationale: Coverage sweep — encode_cond None arm ("invalid cond") is a documented error path. bad_operand_kind hits the non-Cond `_` arm, not encode_cond None. Stronger oracles do not apply to unknown cond names.
- Seed: encode_cinc_pbt encode_cinc_neg_unknown_cond
- Formal: ∀ (rd, rn, rm) same-width GPR, ∀ cond ∈ {zz, foo, eqq, "", "eq ", always}. encode_csel([Rd, Rn, Rm, Cond(cond)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, cond]
  domain: { rd: gpr, rn: same_width_gpr, rm: same_width_gpr, cond: unknown_cond }
  relation:
    op: throws
    lhs: encode_csel([Reg(rd), Reg(rn), Reg(rm), Cond(cond)])
    error: String
generators:
  rd: { gen: string }
  rn: { gen: string }
  rm: { gen: string }
  cond: { gen: string }
expected_error: String
evidence: encoder/mod.rs:169-190 encode_cond returns None for unknown names; compare_branch.rs:90 invalid cond
```

## encode_csel_neg_invalid_name
- Tier: 4
- Rationale: Coverage sweep — parse_reg_num None arm. wrong_reg fails on SP first so the invalid-name kinds never get a passing verdict. Dedicated generator over x32/w32/foo/empty/r0/x/x-1/x99 reaches get_reg's invalid-register error.
- Seed: encode_cinc_pbt encode_cinc_neg_invalid_name
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_csel([Reg(name), Reg("x0"), Reg("x1"), Cond("eq")]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csel
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: invalid_reg }
  relation:
    op: throws
    lhs: encode_csel([Reg(name), Reg("x0"), Reg("x1"), Cond("eq")])
    error: String
generators:
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-148 parse_reg_num None; encoder/mod.rs:959 invalid register
```
