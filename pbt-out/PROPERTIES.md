# Property ledger: encode_cset

## encode_cset_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is Differential against llvm-mc `-triple=aarch64 -show-encoding`. State machine rejected: encode_cset is a pure function with no lifecycle. Round-trip rejected: no in-tree CSET decoder. encode_csinc / encode_cinc fail the same-job sibling gate as differential references (different mnemonics/arity). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text (README.md:5-14). Mapping: `[Reg(rd), Cond(c)]` <-> `cset rd, c`.
- Seed: encode_cinc_pbt::encode_cinc_kat_cset_alias (compare_branch.rs:3757) and encode_cinc_diff_llvm_mc
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ Cond14. encode_cset([Reg(rd), Cond(c)]) = Word(w) ∧ llvm-mc("cset rd, c") = w
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo }
  relation:
    op: eq
    lhs: encode_cset([Reg(rd), Cond(c)])
    rhs: llvm_mc("cset " + rd + ", " + c)
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; ARM ARM CSET alias of CSINC; llvm-mc -triple=aarch64
```

## encode_cset_meta_vs_csinc
- Tier: 4c
- Rationale: Algebraic metamorphic: ARM ARM and the SUT comment state CSET Rd, cond is CSINC Rd, ZR, ZR, invert(cond). encode_csinc is not a same-job differential sibling (4-operand CSINC mnemonic) so it is used only as this alias transform. Stronger differential vs llvm-mc is a separate property.
- Seed: encode_cinc_pbt::encode_cinc_meta_vs_csinc (compare_branch.rs:3814); comment compare_branch.rs:142
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ Cond14. encode_cset([Reg(rd), Cond(c)]) = encode_csinc([Reg(rd), Reg(ZR(rd)), Reg(ZR(rd)), Cond(invert(c))])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo }
  relation:
    op: eq
    lhs: encode_cset([Reg(rd), Cond(c)])
    rhs: encode_csinc([Reg(rd), Reg(ZR(rd)), Reg(ZR(rd)), Cond(invert(c))])
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
evidence: compare_branch.rs:142 CSET Rd, cond -> CSINC Rd, XZR, XZR, invert(cond); ARM ARM CSET alias
```

## encode_cset_meta_vs_cinc
- Tier: 4c
- Rationale: Algebraic metamorphic: CINC Rd, ZR, cond is the same CSINC encoding as CSET Rd, cond (Rm=Rn=ZR). encode_cinc is not a same-job differential sibling (3-operand CINC). Required metamorphic/differential property for STANDARD tier.
- Seed: encode_cinc_pbt::encode_cinc_meta_vs_cset (compare_branch.rs:3853)
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ Cond14. encode_cset([Reg(rd), Cond(c)]) = encode_cinc([Reg(rd), Reg(ZR(rd)), Cond(c)])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo }
  relation:
    op: eq
    lhs: encode_cset([Reg(rd), Cond(c)])
    rhs: encode_cinc([Reg(rd), Reg(ZR(rd)), Cond(c)])
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
evidence: ARM ARM CSET = CSINC Rd,ZR,ZR,invert(cond) = CINC Rd,ZR,cond; encode_cinc_pbt seed
```

## encode_cset_word_layout
- Tier: 4d
- Rationale: Algebraic invariant from ARM ARM CSINC field layout with Rm=Rn=31 and cond=invert(user). Weaker than differential/metamorphic; pins each field independently so a coincidental word match cannot hide a swapped field.
- Seed: encode_cinc_pbt::encode_cinc_word_layout (compare_branch.rs:3886)
- Formal: ∀ rd_n ∈ 0..31, is_64 ∈ Bool, cond_enc ∈ 0..13. encode_cset([Reg(gpr(rd_n,is_64)), Cond(COND14[cond_enc])]) = Word(w) ⇒ (w[31]=sf) ∧ (w[30]=0) ∧ (w[29]=0) ∧ (w[28:21]=0b11010100) ∧ (w[20:16]=31) ∧ (w[15:12]=cond_enc⊕1) ∧ (w[11:10]=0b01) ∧ (w[9:5]=31) ∧ (w[4:0]=rd_n)
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, is_64, cond_enc]
  domain: { rd_n: 0..31, is_64: bool, cond_enc: 0..13 }
  body: let w = encode_cset([Reg(gpr(rd_n,is_64)), Cond(COND14[cond_enc])]).word; (w>>31)&1 == sf(is_64) && (w>>30)&1 == 0 && (w>>29)&1 == 0 && (w>>21)&0xFF == 0b11010100 && (w>>16)&0x1F == 31 && (w>>12)&0xF == (cond_enc^1) && (w>>10)&0x3 == 0b01 && (w>>5)&0x1F == 31 && w&0x1F == rd_n
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  cond_enc: { gen: int, min: 0, max: 13, type: u32 }
evidence: ARM ARM CSINC encoding sf 0 0 11010100 Rm cond 01 Rn Rd with Rm=Rn=31 for CSET
```

## encode_cset_neg_arity
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects `cset` and `cset x0` as too few operands. ARM ARM CSET requires Rd and cond.
- Seed: encode_cinc_pbt::encode_cinc_neg_arity
- Formal: ∀ ops with |ops| < 2. encode_cset(ops) = Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..1 }
  relation:
    op: throws
    lhs: encode_cset(ops_of_len(arity))
    rhs: String
generators:
  arity: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects bare cset and cset x0 (too few operands); ARM ARM CSET Rd, cond
```

## encode_cset_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects a third operand (`cset x0, eq, x1`). gas-compat README.md:5-14. Documented extra-operand rejection.
- Seed: encode_cinc_pbt::encode_cinc_neg_extra_operand
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ Cond14, extra ∈ ExtraOp. encode_cset([Reg(rd), Cond(c), extra]) = Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: rd = "x0", cond = "eq", which = 0 (extra = Reg("x2"))
- Bug report: pbt-out/bug_reports/encode_cset_extra_operand.md

```property
function: encoder.compare_branch.encode_cset
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, c, extra]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo, extra: ExtraOp }
  relation:
    op: throws
    lhs: encode_cset([Reg(rd), Cond(c), extra])
    rhs: String
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
  extra: { gen: oneof, options: [Reg(x2), Imm(0), Symbol(bar), Mem(x1,0)] }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects cset x0, eq, x1 (invalid operand); README.md:5-14 gas-compat
```

## encode_cset_neg_al_nv
- Tier: 4e
- Rationale: Negative/error contract. ARM ARM CSET alias is not valid for AL or NV; llvm-mc: "condition codes AL and NV are invalid for this instruction". Bound cond encodings 14 and 15 sampled exactly.
- Seed: encode_cinc_pbt::encode_cinc_neg_al_nv
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ {al, nv}. encode_cset([Reg(rd), Cond(c)]) = Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: rd = "x0", which = 0 (cond = "al")
- Bug report: pbt-out/bug_reports/encode_cset_al_nv.md

```property
function: encoder.compare_branch.encode_cset
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: {al, nv} }
  relation:
    op: throws
    lhs: encode_cset([Reg(rd), Cond(c)])
    rhs: String
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [al, nv] }
expected_error: String
evidence: ARM ARM CSET not valid for AL/NV; llvm-mc error "condition codes AL and NV are invalid for this instruction"
```

## encode_cset_neg_wrong_reg
- Tier: 4e
- Rationale: Negative/error contract. llvm-mc rejects SP/WSP (register 31 is XZR/WZR), FP/SIMD (d/s/q/v/h/b), and invalid names (x32, foo, empty). ARM ARM CSET takes Wt/Xt only.
- Seed: encode_cinc_pbt::encode_cinc_neg_wrong_reg
- Formal: ∀ name ∈ {sp, wsp} ∪ FPRegs ∪ InvalidNames. encode_cset([Reg(name), Cond("eq")]) = Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind = 0, n = 0 (name = "sp"); also d0 (kind=2)
- Bug report: pbt-out/bug_reports/encode_cset_sp_as_zr.md ; pbt-out/bug_reports/encode_cset_fp_reg.md

```property
function: encoder.compare_branch.encode_cset
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: SP_or_FP_or_Invalid }
  relation:
    op: throws
    lhs: encode_cset([Reg(name), Cond("eq")])
    rhs: String
generators:
  name: { gen: oneof, options: [sp, wsp, dN, sN, qN, vN, hN, bN, x32, w32, foo, empty, r0, x, x-1, x99] }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects cset sp/wsp/d0 (invalid operand); ARM ARM CSET Wt/Xt only, register 31 is XZR/WZR
```

## encode_cset_neg_unknown_cond
- Tier: 4e
- Rationale: Coverage-sweep negative/error contract for encode_cond None. Unknown condition names are not in encode_cond's 16-name map (mod.rs:169-190) and llvm-mc rejects them.
- Seed: encode_cinc_pbt::encode_cinc_neg_unknown_cond
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ {zz, foo, eqq, "", "eq ", always}. encode_cset([Reg(rd), Cond(c)]) = Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: UnknownCond }
  relation:
    op: throws
    lhs: encode_cset([Reg(rd), Cond(c)])
    rhs: String
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [zz, foo, eqq, empty, eq_space, always] }
expected_error: String
evidence: encoder/mod.rs:169-190 encode_cond returns None for unknown names; llvm-mc rejects unknown cond
```

## encode_cset_neg_invalid_name
- Tier: 4e
- Rationale: Coverage-sweep negative/error contract for get_reg parse_reg_num None. Invalid register names (x32, foo, empty, r0, x, x-1, x99) must be Err.
- Seed: encode_cinc_pbt::encode_cinc_neg_invalid_name
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_cset([Reg(name), Cond("eq")]) = Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: InvalidRegName }
  relation:
    op: throws
    lhs: encode_cset([Reg(name), Cond("eq")])
    rhs: String
generators:
  name: { gen: oneof, options: [x32, w32, foo, empty, r0, x, x-1, x99] }
expected_error: String
evidence: encoder/mod.rs:131-148 parse_reg_num returns None for these names; get_reg then Err
```

## encode_cset_neg_bad_operand_kind
- Tier: 4e
- Rationale: Coverage-sweep negative/error contract for get_reg non-Reg and cond-not-Cond arms. Imm/Mem/Symbol/Shift/Label in either slot must be Err.
- Seed: encode_cinc_pbt::encode_cinc_neg_bad_operand_kind
- Formal: ∀ slot ∈ {0,1}, bad ∈ {Imm, Mem, Symbol, Shift, Label}. encode_cset(ops with ops[slot]=bad) = Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cset
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, bad]
  domain: { slot: 0..1, bad: NonRegNonCond }
  relation:
    op: throws
    lhs: encode_cset(ops_with_slot(slot, bad))
    rhs: String
generators:
  slot: { gen: int, min: 0, max: 1, type: u32 }
  bad: { gen: oneof, options: [Imm(0), Mem(x0,0), Symbol(foo), Shift(lsl,0), Label(foo)] }
expected_error: String
evidence: get_reg (mod.rs:956) requires Operand::Reg; encode_cset match requires Operand::Cond at index 1
```
