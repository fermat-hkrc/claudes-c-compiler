# Property ledger: encode_csetm

## encode_csetm_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is Differential against llvm-mc `-triple=aarch64 -show-encoding`. State machine rejected: encode_csetm is a pure function with no lifecycle. Round-trip rejected: no in-tree CSETM decoder. encode_csinv / encode_cinv fail the same-job sibling gate as differential references (different mnemonics/arity). SUT-boundary: internal-helper of the GNU-style assembler; public contract is gas-compatible AArch64 text (README.md:5-14). Mapping: `[Reg(rd), Cond(c)]` <-> `csetm rd, c`.
- Seed: encode_cset_pbt::encode_cset_diff_llvm_mc; encode_cinv_pbt::encode_cinv_kat_csetm_alias (compare_branch.rs:4466)
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ Cond14. encode_csetm([Reg(rd), Cond(c)]) = Word(w) ∧ llvm-mc("csetm rd, c") = w
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo }
  relation:
    op: eq
    lhs: encode_csetm([Reg(rd), Cond(c)])
    rhs: llvm_mc("csetm " + rd + ", " + c)
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; ARM ARM CSETM alias of CSINV; llvm-mc -triple=aarch64
```

## encode_csetm_meta_vs_csinv
- Tier: 4c
- Rationale: Algebraic metamorphic: ARM ARM and the SUT comment state CSETM Rd, cond is CSINV Rd, ZR, ZR, invert(cond). encode_csinv is not a same-job differential sibling (4-operand CSINV mnemonic) so it is used only as this alias transform. Stronger differential vs llvm-mc is a separate property. Required metamorphic/differential property for STANDARD tier.
- Seed: compare_branch.rs:156 CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond); encode_cset_pbt::encode_cset_meta_vs_csinc
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ Cond14. encode_csetm([Reg(rd), Cond(c)]) = encode_csinv([Reg(rd), Reg(ZR(rd)), Reg(ZR(rd)), Cond(invert(c))])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo }
  relation:
    op: eq
    lhs: encode_csetm([Reg(rd), Cond(c)])
    rhs: encode_csinv([Reg(rd), Reg(ZR(rd)), Reg(ZR(rd)), Cond(invert(c))])
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
evidence: compare_branch.rs:156 CSETM Rd, cond -> CSINV Rd, XZR, XZR, invert(cond); ARM ARM CSETM alias
```

## encode_csetm_meta_vs_cinv
- Tier: 4c
- Rationale: Algebraic metamorphic: CINV Rd, ZR, cond is the same CSINV encoding as CSETM Rd, cond (Rm=Rn=ZR). encode_cinv is not a same-job differential sibling (3-operand CINV).
- Seed: encode_cinv_pbt::encode_cinv_meta_vs_csetm (compare_branch.rs:4562)
- Formal: ∀ rd ∈ GPR64∪GPR32, c ∈ Cond14. encode_csetm([Reg(rd), Cond(c)]) = encode_cinv([Reg(rd), Reg(ZR(rd)), Cond(c)])
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo }
  relation:
    op: eq
    lhs: encode_csetm([Reg(rd), Cond(c)])
    rhs: encode_cinv([Reg(rd), Reg(ZR(rd)), Cond(c)])
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
evidence: ARM ARM CINV with Rn=ZR is CSETM; encode_cinv_pbt::encode_cinv_meta_vs_csetm
```

## encode_csetm_word_layout
- Tier: 4d
- Rationale: Algebraic invariant from ARM ARM CSINV encoding with Rm=Rn=31, op2=00, op=1. Weaker than differential/metamorphic; pins each field independently so a swapped cond/Rd cannot hide behind a matching sibling.
- Seed: encode_cset_pbt::encode_cset_word_layout; ARM ARM CSINV sf 1 0 11010100 Rm cond 00 Rn Rd
- Formal: ∀ rd_n ∈ 0..31, is_64 ∈ Bool, c ∈ 0..13. encode_csetm([Reg(gpr(rd_n,is_64)), Cond(COND14[c])]) = Word(w) ⇒ w[31]=is_64 ∧ w[30]=1 ∧ w[29]=0 ∧ w[28:21]=0b11010100 ∧ w[20:16]=31 ∧ w[15:12]=c XOR 1 ∧ w[11:10]=00 ∧ w[9:5]=31 ∧ w[4:0]=rd_n
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, is_64, c]
  domain: { rd_n: 0..31, is_64: bool, c: 0..13 }
  relation:
    op: holds
    expr: "let Word(w) = encode_csetm([Reg(gpr(rd_n,is_64)), Cond(COND14[c])])?; ((w>>31)&1)==is_64 && ((w>>30)&1)==1 && ((w>>29)&1)==0 && ((w>>21)&0xFF)==0b11010100 && ((w>>16)&0x1F)==31 && ((w>>12)&0xF)==(c^1) && ((w>>10)&0x3)==0 && ((w>>5)&0x1F)==31 && (w&0x1F)==rd_n"
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  c: { gen: int, min: 0, max: 13, type: u32 }
evidence: ARM ARM CSINV encoding; compare_branch.rs:156-165
```

## encode_csetm_neg_arity
- Tier: 4e
- Rationale: Negative/error contract from llvm-mc (too few operands) and gas-compat README. CSETM requires Rd and cond.
- Seed: encode_cset_pbt::encode_cset_neg_arity
- Formal: ∀ ops. |ops| < 2 ⇒ encode_csetm(ops) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..1 }
  relation:
    op: throws
    expr: encode_csetm(ops_of_len(arity))
generators:
  arity: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands for instruction"; README.md:5-14 gas-compat
```

## encode_csetm_neg_extra_operand
- Tier: 4e
- Rationale: Negative/error: llvm-mc rejects a third operand (`invalid operand`). Documented gas-compat contract. Bound: exactly 2 operands.
- Seed: encode_cset_pbt::encode_cset_neg_extra_operand
- Formal: ∀ rd ∈ GPR, c ∈ Cond14, extra ∈ ExtraOperand. encode_csetm([Reg(rd), Cond(c), extra]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Cond("eq"), Reg("x2")]
- Bug report: pbt-out/bug_reports/encode_csetm_extra_operand.md

```property
function: encoder.compare_branch.encode_csetm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, c, extra]
  domain: { rd: GPR64_or_GPR32, c: Cond14_with_hs_lo, extra: ExtraOperand }
  relation:
    op: throws
    expr: encode_csetm([Reg(rd), Cond(c), extra])
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [eq, ne, cs, hs, cc, lo, mi, pl, vs, vc, hi, ls, ge, lt, gt, le] }
  extra: { gen: oneof, options: [Reg(x2), Imm(0), Symbol(bar), Mem] }
expected_error: String
evidence: llvm-mc "invalid operand" for csetm x0, eq, x2; README.md:5-14 gas-compat
```

## encode_csetm_neg_al_nv
- Tier: 4e
- Rationale: Negative/error: ARM ARM CSETM alias is not valid when cond is AL or NV; llvm-mc rejects with "condition codes AL and NV are invalid for this instruction". Bound sampled exactly at AL=14 and NV=15.
- Seed: encode_cset_pbt::encode_cset_neg_al_nv
- Formal: ∀ rd ∈ GPR, c ∈ {al, nv}. encode_csetm([Reg(rd), Cond(c)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Cond("al")]
- Bug report: pbt-out/bug_reports/encode_csetm_al_nv.md

```property
function: encoder.compare_branch.encode_csetm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: {al, nv} }
  relation:
    op: throws
    expr: encode_csetm([Reg(rd), Cond(c)])
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [al, nv] }
expected_error: String
evidence: ARM ARM CSETM not valid for AL/NV; llvm-mc "condition codes AL and NV are invalid for this instruction"
```

## encode_csetm_neg_wrong_reg
- Tier: 4e
- Rationale: Negative/error: llvm-mc rejects SP/WSP (register 31 is XZR/WZR for CSETM) and FP/SIMD names; parse_reg_num None for invalid names. Bound: SP/WSP plus d/s/q/v/h/b prefixes and invalid names.
- Seed: encode_cset_pbt::encode_cset_neg_wrong_reg
- Formal: ∀ name ∈ {sp, wsp} ∪ FP_SIMD ∪ InvalidReg. encode_csetm([Reg(name), Cond("eq")]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("sp"), Cond("eq")] (kind=0, n=0); also [Reg("d0"), Cond("eq")]
- Bug report: pbt-out/bug_reports/encode_csetm_sp_as_zr.md; pbt-out/bug_reports/encode_csetm_fp_reg.md

```property
function: encoder.compare_branch.encode_csetm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: SP_or_WSP_or_FP_or_invalid }
  relation:
    op: throws
    expr: encode_csetm([Reg(name), Cond("eq")])
generators:
  name: { gen: oneof, options: [sp, wsp, dN, sN, qN, vN, hN, bN, x32, w32, foo, empty, r0, x, x-1, x99] }
expected_error: String
evidence: llvm-mc "invalid operand" for csetm sp, eq and csetm d0, eq; ARM ARM Wt/Xt only, register 31 is XZR/WZR
```

## encode_csetm_neg_unknown_cond
- Tier: 4e
- Rationale: Coverage-sweep negative/error for encode_cond None arm. Unknown condition names must Err ("invalid cond"). Stronger oracles do not apply to this error path.
- Seed: encode_cset_pbt::encode_cset_neg_unknown_cond
- Formal: ∀ rd ∈ GPR, c ∈ {zz, foo, eqq, empty, "eq ", always}. encode_csetm([Reg(rd), Cond(c)]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, c]
  domain: { rd: GPR64_or_GPR32, c: unknown_cond }
  relation:
    op: throws
    expr: encode_csetm([Reg(rd), Cond(c)])
generators:
  rd: { gen: oneof, options: [x_gpr, w_gpr] }
  c: { gen: oneof, options: [zz, foo, eqq, empty, "eq ", always] }
expected_error: String
evidence: encode_cond None -> "invalid cond"; llvm-mc rejects unknown conditions
```

## encode_csetm_neg_invalid_name
- Tier: 4e
- Rationale: Coverage-sweep negative/error for parse_reg_num None arm. Invalid register names (x32, w32, foo, empty, r0, x, x-1, x99) must Err. Isolated from SP/FP so the None arm is actually executed (wrong_reg shrinks to SP).
- Seed: encode_cset_pbt::encode_cset_neg_invalid_name
- Formal: ∀ name ∈ {x32, w32, foo, empty, r0, x, x-1, x99}. encode_csetm([Reg(name), Cond("eq")]) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: invalid_reg_name }
  relation:
    op: throws
    expr: encode_csetm([Reg(name), Cond("eq")])
generators:
  name: { gen: oneof, options: [x32, w32, foo, empty, r0, x, x-1, x99] }
expected_error: String
evidence: parse_reg_num None -> "invalid register"; llvm-mc rejects these names
```

## encode_csetm_neg_bad_operand_kind
- Tier: 4e
- Rationale: Coverage-sweep negative/error for get_reg non-Reg and cond-not-Cond arms. Imm/Mem/Symbol/Shift/Label in either slot must Err.
- Seed: encode_cset_pbt::encode_cset_neg_bad_operand_kind
- Formal: ∀ slot ∈ {0,1}, bad ∈ {Imm, Mem, Symbol, Shift, Label}. encode_csetm(ops with slot replaced by bad) is Err
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_csetm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, which]
  domain: { slot: 0..1, which: 0..4 }
  relation:
    op: throws
    expr: encode_csetm(ops_with_bad_kind(slot, which))
generators:
  slot: { gen: int, min: 0, max: 1, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: get_reg non-Reg -> "expected register"; cond match arm -> "csetm requires condition"
```
