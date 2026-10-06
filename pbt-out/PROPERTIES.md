# Properties: encode_tst

## encode_tst_diff_valid_reg
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the valid TST shifted-register domain. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree TST decoder). encode_logical rejected as same-job sibling (3-operand ANDS mnemonic/arity). encode_cmp/encode_cmn rejected (SUBS/ADDS aliases). Weaker: ARM field invariant, field metamorphic, negative_error.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: compare_branch.rs encode_cmp_diff_reg_llvm_mc
- Formal: ∀ rn,rm ∈ GPR_W ∪ GPR_X (same width, n∈0..31 including ZR/LR), shift ∈ {ε} ∪ {lsl,lsr,asr,ror}×{0..max}, spellings ∈ {lower,UPPER,x31≡xzr}. encode_tst([Rn,Rm{,Shift}]) = llvm-mc("tst Rn, Rm{, shift #amt}") as u32 LE word
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm, is_64]
  domain: { rn: "0..31", rm: "0..31", is_64: bool }
  relation:
    op: eq
    lhs: encode_tst([Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm)), Shift?])
    rhs: llvm_mc_word("tst rn, rm{, shift}")
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: compare_branch.rs:38 asserted TST/ANDS alias; encoder/mod.rs:433 dispatch
```

## encode_tst_diff_valid_imm
- Tier: 5
- Rationale: Same llvm-mc differential on the bitmask-immediate form. Independent bitmask_from_fields constructor (ARM ARM, not encode_bitmask_imm).
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: data_processing.rs encode_logical_pbt bitmask_from_fields
- Formal: ∀ rn ∈ GPR_W ∪ GPR_X, (esize,ones,rot) a valid ARM bitmask. encode_tst([Rn, Imm(bitmask)]) = llvm-mc("tst Rn, #imm")
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, is_64, imm]
  domain: { rn: "0..31", is_64: bool, imm: "valid ARM bitmask" }
  relation:
    op: eq
    lhs: encode_tst([Reg(gpr(is_64,rn)), Imm(imm)])
    rhs: llvm_mc_word("tst rn, #imm")
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 1, max: 255, type: i64 }
evidence: compare_branch.rs:38; ARM ARM Logical (immediate) ANDS Rd=31
```

## encode_tst_arm_fields
- Tier: 4
- Rationale: ARM ARM Logical (shifted register) ANDS field layout with Rd=31, opc=11 is an exact structural invariant of every successful register-form encoding.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: compare_branch.rs encode_cmp_word_layout_imm
- Formal: ∀ rn,rm ∈ 0..31, is_64 ∈ {false,true}, (st,amt) valid shift. let w = encode_tst(...). bits[4:0]=31 ∧ bits[30:29]=11 ∧ bits[28:24]=01010 ∧ bit21=0 ∧ bits[31]=sf ∧ bits[9:5]=rn ∧ bits[20:16]=rm ∧ bits[23:22]=st ∧ bits[15:10]=amt
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rn, rm, is_64]
  domain: { rn: "0..31", rm: "0..31", is_64: bool }
  relation:
    op: holds
    expr: word_fields_match_arm_ands_rd31(encode_tst(ops))
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: compare_branch.rs:38 ARM ANDS XZR alias
```

## encode_tst_meta_vs_ands
- Tier: 4
- Rationale: Documented alias TST Rn, op = ANDS XZR/WZR, Rn, op. encode_logical is not a same-job primary differential; used only as a metamorphic alias transform.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: compare_branch.rs encode_cmp_meta_vs_subs
- Formal: ∀ ops a valid TST operand list. encode_tst(ops) = encode_logical([ZR]++ops, 0b11) where ZR is wzr iff Rn is 32-bit else xzr
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rn, rm, is_64]
  domain: { rn: "0..31", rm: "0..31", is_64: bool }
  relation:
    op: eq
    lhs: encode_tst([Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))])
    rhs: encode_logical([Reg(zr), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))], 0b11)
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: compare_branch.rs:38 alias comment
```

## encode_tst_metamorphic_fields
- Tier: 4
- Rationale: A single-field increment of Rn/Rm/shift-amount or W↔X must flip only the corresponding ARM bit.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: encode_fnmadd_fnmsub_metamorphic_fields
- Formal: ∀ rn,rm ∈ 0..30, is_64, amt ∈ 0..max-1. let w = TST(rn,rm,lsl#amt). TST(rn+1)=w+(1<<5) ∧ TST(rm+1)=w+(1<<16) ∧ TST(amt+1)=w+(1<<10) ∧ (TST(¬is_64) XOR w)=1<<31
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rn, rm, is_64, amt]
  domain: { rn: "0..30", rm: "0..30", is_64: bool, amt: "0..30" }
  relation:
    op: holds
    expr: field_increments_isolate(encode_tst)
generators:
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  amt: { gen: int, min: 0, max: 30, type: u32 }
evidence: ARM ARM Logical (shifted register) disjoint Rn/Rm/imm6/sf
```

## encode_tst_neg_arity
- Tier: 3
- Rationale: llvm-mc/gas reject TST with fewer than 2 operands. encode_logical requires 3 operands after the ZR prepend, so arity 0 and 1 must Err.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: compare_branch.rs encode_cmp_neg_arity
- Formal: ∀ arity ∈ {0,1}, n ∈ 0..30. encode_tst(ops with arity operands) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity, n]
  domain: { arity: "0..1", n: "0..30" }
  relation:
    op: throws
    expr: encode_tst(ops_of_len(arity))
expected_error: String
generators:
  arity: { gen: int, min: 0, max: 1, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
evidence: compare_branch.rs:38 alias requires Rn and op
```

## encode_tst_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc rejects a third non-shift operand. Body does not check operands.len() after the ZR prepend; extra non-Shift is ignored. The input is accepted by the API and not documented invalid.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: compare_branch.rs encode_cmp_neg_extra_operand
- Formal: ∀ rn,rm same-width GPR, extra ∉ {valid Shift}. encode_tst([Rn,Rm,extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: failing
- Counterexample: encode_tst([Reg("w0"), Reg("w0"), Reg("x0")])
- Bug report: pbt-out/bug_reports/encode_tst_extra_operand.md

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, rm, is_64]
  domain: { rn: "0..31", rm: "0..31", is_64: bool }
  relation:
    op: throws
    expr: encode_tst([Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm)), extra])
expected_error: String
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
evidence: compare_branch.rs:38 two-operand alias
```

## encode_tst_neg_wrong_reg
- Tier: 3
- Rationale: Split during Test into encode_tst_neg_sp / encode_tst_neg_mixed_width / encode_tst_neg_fp_reg so each bug class has its own shrinking witness.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: compare_branch.rs encode_cmp_neg_wrong_reg
- Formal: (retired — split)
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: retired
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: "0..8", n: "0..31" }
  relation:
    op: throws
    expr: encode_tst(wrong_reg_ops(kind, n))
expected_error: String
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: compare_branch.rs:38 Rn/op are GPRs
```

## encode_tst_neg_sp
- Tier: 3
- Rationale: llvm-mc rejects SP/WSP as Rn or Rm. parse_reg_num maps both to 31 (same as ZR). Not documented invalid on encode_tst.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: encode_tst_neg_wrong_reg kind=0
- Formal: ∀ which ∈ {SP-Rn, SP-Rm, WSP-Rn, WSP-Rm}, n ∈ 0..30. encode_tst(sp_ops(which,n)) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: failing
- Counterexample: encode_tst([Reg("sp"), Reg("x0")])
- Bug report: pbt-out/bug_reports/encode_tst_sp_as_zr.md

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, n]
  domain: { which: "0..3", n: "0..30" }
  relation:
    op: throws
    expr: encode_tst(wrong_reg_ops(which, n))
expected_error: String
generators:
  which: { gen: int, min: 0, max: 3, type: u32 }
  n: { gen: int, min: 0, max: 30, type: u32 }
evidence: compare_branch.rs:38 Rn/op are GPRs not SP
```

## encode_tst_neg_mixed_width
- Tier: 3
- Rationale: llvm-mc rejects mixed W/X. encode_logical takes Rm's number only. Not documented invalid.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: encode_tst_neg_wrong_reg mixed
- Formal: ∀ n ∈ 0..30. encode_tst([Reg("xN"), Reg("wN")]) is Err ∧ encode_tst([Reg("wN"), Reg("xN")]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: failing
- Counterexample: encode_tst([Reg("w0"), Reg("x0")])
- Bug report: pbt-out/bug_reports/encode_tst_mixed_width.md

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: "0..30" }
  relation:
    op: throws
    expr: encode_tst([Reg("w"+n), Reg("x"+n)])
expected_error: String
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
evidence: compare_branch.rs:38 matching-width GPR pair
```

## encode_tst_neg_fp_reg
- Tier: 3
- Rationale: llvm-mc rejects FP/SIMD registers. parse_reg_num accepts d/s/q/v/h/b prefixes. Not documented invalid.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: encode_tst_neg_wrong_reg FP
- Formal: ∀ n,m ∈ 0..31, p ∈ {d,s,q,v,h,b}. encode_tst([Reg(pN), Reg(pM)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: failing
- Counterexample: encode_tst([Reg("d0"), Reg("d0")])
- Bug report: pbt-out/bug_reports/encode_tst_fp_as_gpr.md

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, m]
  domain: { n: "0..31", m: "0..31" }
  relation:
    op: throws
    expr: encode_tst([Reg("d"+n), Reg("d"+m)])
expected_error: String
generators:
  n: { gen: int, min: 0, max: 31, type: u32 }
  m: { gen: int, min: 0, max: 31, type: u32 }
evidence: compare_branch.rs:38 Rn/op are GPRs
```

## encode_tst_neg_invalid_name
- Tier: 3
- Rationale: Sweep: unparsable names (x32, foo, empty, r0) must Err. parse_reg_num returns None; encode_logical get_reg fails.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: (none) — coverage sweep
- Formal: ∀ name ∈ {x32,w32,foo,"",r0,x,x-1,x99}. encode_tst([Reg(name), Reg("x0")]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which]
  domain: { which: "0..7" }
  relation:
    op: throws
    expr: encode_tst([Reg(invalid_name(which)), Reg("x0")])
expected_error: String
generators:
  which: { gen: int, min: 0, max: 7, type: u32 }
evidence: compare_branch.rs:38; parse_reg_num rejects x32
```

## encode_tst_neg_shift_oor
- Tier: 3
- Rationale: llvm-mc rejects shift amounts outside 0..31 (W) / 0..63 (X). encode_logical masks with `& 0x3F`. Not documented invalid.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: (none) — strengthening / sweep
- Formal: ∀ rn,rm ∈ 0..31, is_64, amt > max(is_64). encode_tst([Rn,Rm,Shift(lsl,amt)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: failing
- Counterexample: encode_tst([Reg("w0"), Reg("w0"), Shift { kind: "lsl", amount: 32 }])
- Bug report: pbt-out/bug_reports/encode_tst_shift_oor.md

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, rm, amt]
  domain: { rn: "0..31", rm: "0..31", amt: "32..128" }
  relation:
    op: throws
    expr: encode_tst([Reg("w"+rn), Reg("w"+rm), Shift("lsl", amt)])
expected_error: String
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  amt: { gen: int, min: 32, max: 128, type: u32 }
evidence: ARM ARM 32-bit imm6 < 32; llvm-mc range
```

## encode_tst_neg_invalid_imm
- Tier: 3
- Rationale: Sweep: non-bitmask immediates llvm-mc rejects (#0, #-1, …) must Err. encode_bitmask_imm already returns None for 0 and all-ones.
- Doc contract: compare_branch.rs:38 "TST Rn, op -> ANDS XZR, Rn, op" — asserted fingerprint 73596118
- Seed: (none) — coverage sweep
- Formal: ∀ rn ∈ 0..31, imm such that llvm-mc("tst Rn, #imm") errors. encode_tst([Rn, Imm(imm)]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_tst_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_tst
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, is_64, imm]
  domain: { rn: "0..31", is_64: bool, imm: "non-bitmask" }
  relation:
    op: throws
    expr: encode_tst([Reg(gpr(is_64,rn)), Imm(imm)])
expected_error: String
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: -1, max: 0, type: i64 }
evidence: ARM ARM logical immediate excludes 0 and all-ones
```
