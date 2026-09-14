# Properties: encode_cinc

## encode_cinc_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler) for gas-compatible `cinc Rd, Rn, cond`. State machine rejected (pure function, no lifecycle). In-tree CINC decoder does not exist so algebraic round-trip is unavailable. encode_csinc / encode_cset are architectural aliases with different mnemonics/arity and fail the same-job sibling gate as a differential reference (used as metamorphic instead). Doc evidence: assembler README gas-compat; ARM ARM CINC alias of CSINC; dispatch cinc.
- Seed: src/backend/arm/assembler/encoder/compare_branch.rs encode_blr_pbt::encode_blr_diff_xn_llvm_mc
- Formal: ∀ rd, rn ∈ same-width GPR {x0..x30,xzr,lr} or {w0..w30,wzr}, cond ∈ Cond14 (eq,ne,cs,hs,cc,lo,mi,pl,vs,vc,hi,ls,ge,lt,gt,le). encode_cinc([Reg(rd), Reg(rn), Cond(cond)]) = Word(llvm-mc("cinc " + rd + ", " + rn + ", " + cond)).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: gpr_name, rn: gpr_name_same_width, cond: cond14 }
  relation:
    op: eq
    lhs: encode_cinc([Reg(rd), Reg(rn), Cond(cond)]) as Word
    rhs: llvm_mc("cinc " + rd + ", " + rn + ", " + cond)
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:898 cinc dispatch; compare_branch.rs:292 CINC Rd, Rn, cond -> CSINC Rd, Rn, Rn, invert(cond); ARM ARM Conditional Increment alias of CSINC
```

## encode_cinc_meta_vs_csinc
- Tier: 4
- Rationale: Algebraic metamorphic from the documented architectural alias: CINC Rd, Rn, cond encodes identically to CSINC Rd, Rn, Rn, invert(cond). encode_csinc is independently implemented (4-operand CSINC, no invert). Stronger differential already covers the happy path vs llvm-mc; this isolates the alias contract in-tree. invert is ARM ARM cond XOR 1 (eq<->ne, cs<->cc, ...). Same-job differential rejected: different mnemonic and arity.
- Seed: encode_ccmp_ccmn_pbt::encode_ccmp_ccmn_meta_ccmp_vs_ccmn
- Formal: ∀ rd, rn same-width GPR, cond ∈ Cond14. encode_cinc([Reg(rd), Reg(rn), Cond(cond)]) = encode_csinc([Reg(rd), Reg(rn), Reg(rn), Cond(invert(cond))]).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: gpr_name, rn: gpr_name_same_width, cond: cond14 }
  relation:
    op: eq
    lhs: encode_cinc([Reg(rd), Reg(rn), Cond(cond)])
    rhs: encode_csinc([Reg(rd), Reg(rn), Reg(rn), Cond(invert(cond))])
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
evidence: compare_branch.rs:292 Encode CINC Rd, Rn, cond -> CSINC Rd, Rn, Rn, invert(cond); ARM ARM CINC alias of CSINC; encode_csinc at compare_branch.rs:99-111
```

## encode_cinc_meta_vs_cset
- Tier: 4
- Rationale: Algebraic metamorphic special case: CSET Rd, cond is CSINC Rd, XZR, XZR, invert(cond), which is CINC Rd, XZR, cond. llvm-mc disassembles `cinc x0, xzr, eq` as `cset x0, eq` with the same encoding. Stronger differential covers general CINC; this pins the ZR-source alias. encode_cset is independently implemented.
- Seed: encode_ccmp_ccmn_pbt::encode_ccmp_ccmn_meta_ccmp_vs_ccmn
- Formal: ∀ rd ∈ GPR (X-form with xzr or W-form with wzr), cond ∈ Cond14. encode_cinc([Reg(rd), Reg(zr(rd)), Cond(cond)]) = encode_cset([Reg(rd), Cond(cond)]).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, cond]
  domain: { rd: gpr_name, cond: cond14 }
  relation:
    op: eq
    lhs: encode_cinc([Reg(rd), Reg(zr_of(rd)), Cond(cond)])
    rhs: encode_cset([Reg(rd), Cond(cond)])
generators:
  rd: { gen: string }
  cond: { gen: string }
evidence: compare_branch.rs:141 CSET Rd, cond -> CSINC Rd, XZR, XZR, invert(cond); compare_branch.rs:292 CINC -> CSINC with Rm=Rn; llvm-mc disassembles cinc x0, xzr, eq as cset x0, eq
```

## encode_cinc_word_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM CSINC encoding of the CINC alias: bits[31]=sf (from Rd width), [30]=op=0, [29]=S=0, [28:21]=11010100, [20:16]=Rn, [15:12]=invert(cond)=cond^1, [11]=0, [10]=1, [9:5]=Rn, [4:0]=Rd. Stronger differential already checks the whole word vs llvm-mc; this names the field contract. invert from ARM ARM (LSB flip of 4-bit cond), not from the SUT body as First Evidence.
- Seed: encode_blr_pbt::encode_blr_word_layout
- Formal: ∀ rd_n, rn_n ∈ {0..31}, is_64 ∈ bool, cond_enc ∈ {0..13}. let w = encode_cinc([Reg(name(rd_n,is_64)), Reg(name(rn_n,is_64)), Cond(cond_name(cond_enc))]). w[31]=is_64, w[30:21]=0b0_0_11010100, w[20:16]=rn_n, w[15:12]=cond_enc XOR 1, w[11:10]=0b01, w[9:5]=rn_n, w[4:0]=rd_n.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd_n, rn_n, is_64, cond_enc]
  domain: { rd_n: u5, rn_n: u5, is_64: bool, cond_enc: 0..13 }
  relation:
    op: holds
    expr: word_matches_csinc_alias(encode_cinc([Reg(name(rd_n,is_64)), Reg(name(rn_n,is_64)), Cond(cond_name(cond_enc))]), rd_n, rn_n, is_64, cond_enc xor 1)
generators:
  rd_n: { gen: int, min: 0, max: 31, type: u32 }
  rn_n: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  cond_enc: { gen: int, min: 0, max: 13, type: u32 }
evidence: ARM ARM CSINC sf 00 11010100 Rm cond 01 Rn Rd; ARM ARM CINC alias Rm=Rn invert(cond); encoder/mod.rs:1-7 32-bit AArch64 words
```

## encode_cinc_neg_arity
- Tier: 5
- Rationale: Negative/error contract from gas: too few operands is rejected. llvm-mc reports "too few operands for instruction" for `cinc`, `cinc x0`, `cinc x0, x1`. Stronger oracles do not apply to the invalid-arity domain.
- Seed: encode_blr_pbt::encode_blr_neg_arity
- Formal: ∀ prefix of [Reg(rd), Reg(rn)] of length 0..2. encode_cinc(prefix) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..2 }
  relation:
    op: holds
    expr: encode_cinc(ops_of_len(arity)).is_err()
generators:
  arity: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: llvm-mc -triple=aarch64 rejects bare/1-op/2-op cinc (too few operands); README.md:5-14 gas-compat
```

## encode_cinc_neg_extra_operand
- Tier: 5
- Rationale: Negative/error contract from gas: a fourth operand is invalid. llvm-mc reports "invalid operand for instruction" for `cinc x0, x1, eq, x2`. encode_cinc currently inspects only indices 0..2 — this is a documented-valid rejection the SUT may miss.
- Seed: encode_blr_pbt::encode_blr_neg_extra_operand
- Formal: ∀ rd, rn same-width GPR, cond ∈ Cond14, extra ∈ {Reg, Imm, Symbol, Mem}. encode_cinc([Reg(rd), Reg(rn), Cond(cond), extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: rd="x0", rn="x0", cond="eq", which=0 (extra=Reg("x2"))
- Bug report: pbt-out/bug_reports/encode_cinc_extra_operand.md

```property
function: encoder.compare_branch.encode_cinc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, cond, extra]
  domain: { rd: gpr_name, rn: gpr_name_same_width, cond: cond14, extra: extra_operand }
  relation:
    op: holds
    expr: encode_cinc([Reg(rd), Reg(rn), Cond(cond), extra]).is_err()
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects cinc x0, x1, eq, x2 (invalid operand); README.md:5-14 gas-compat
```

## encode_cinc_neg_al_nv
- Tier: 5
- Rationale: Negative/error contract from ARM ARM / gas: CINC is not a valid alias when cond is AL or NV. llvm-mc reports "condition codes AL and NV are invalid for this instruction". Bound: cond encodings 14 and 15 (and names al/nv). Stronger oracles do not apply to this invalid-cond domain.
- Seed: encode_blr_pbt::encode_blr_neg_wrong_reg_class
- Formal: ∀ rd, rn same-width GPR, cond ∈ {al, nv}. encode_cinc([Reg(rd), Reg(rn), Cond(cond)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: rd="x0", rn="x0", which=0 (cond="al")
- Bug report: pbt-out/bug_reports/encode_cinc_al_nv.md

```property
function: encoder.compare_branch.encode_cinc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: gpr_name, rn: gpr_name_same_width, cond: {al, nv} }
  relation:
    op: holds
    expr: encode_cinc([Reg(rd), Reg(rn), Cond(cond)]).is_err()
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
expected_error: String
evidence: ARM ARM CINC alias not valid for AL/NV; llvm-mc error "condition codes AL and NV are invalid for this instruction"; README.md:5-14 gas-compat
```

## encode_cinc_neg_wrong_reg
- Tier: 5
- Rationale: Negative/error contract from gas: SP/WSP, mixed x/w, FP/SIMD, and invalid names are rejected. llvm-mc: "invalid operand for instruction" for `cinc sp, x1, eq`, `cinc x0, w1, eq`, `cinc d0, d1, eq`, `cinc x32, x1, eq`. Register 31 is XZR/WZR never SP. Stronger oracles do not apply to this invalid-register domain.
- Seed: encode_blr_pbt::encode_blr_neg_wrong_reg_class
- Formal: ∀ (rd, rn, cond) where at least one of: rd or rn is SP/WSP/FP/invalid-name, or rd/rn mixed x/w. encode_cinc([Reg(rd), Reg(rn), Cond(eq)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind=0, n=0 (Rd="sp", Rn="x0", cond="eq")
- Bug report: pbt-out/bug_reports/encode_cinc_sp_as_zr.md

```property
function: encoder.compare_branch.encode_cinc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: sp|wsp|fp|mixed|invalid, n: 0..31 }
  relation:
    op: holds
    expr: encode_cinc(bad_ops(kind, n)).is_err()
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects cinc sp / mixed x/w / d0 / x32; ARM ARM CSEL-group register 31 is XZR not SP; README.md:5-14 gas-compat
```

## encode_cinc_neg_unknown_cond
- Tier: 5
- Rationale: Coverage sweep — encode_cond None arm. Unknown condition strings must be rejected. Documented by encode_cond returning None and the error "unknown condition". llvm-mc also rejects non-cond tokens.
- Seed: (none) — coverage sweep of encode_cond None
- Formal: ∀ rd, rn same-width GPR, cond ∈ {zz, foo, eqq, "", "eq ", always}. encode_cinc([Reg(rd), Reg(rn), Cond(cond)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, cond]
  domain: { rd: gpr_name, rn: gpr_name_same_width, cond: unknown_cond }
  relation:
    op: holds
    expr: encode_cinc([Reg(rd), Reg(rn), Cond(cond)]).is_err()
generators:
  rd: { gen: string }
  rn: { gen: string }
  cond: { gen: string }
expected_error: String
evidence: encoder/mod.rs:169-188 encode_cond returns None for unknown names; compare_branch.rs:298 unknown condition error; README.md:5-14 gas-compat
```

## encode_cinc_neg_invalid_name
- Tier: 5
- Rationale: Coverage sweep — get_reg parse_reg_num None arm, which encode_cinc_neg_wrong_reg never reached (shrunk to SP). Invalid names x32/w32/foo/empty/r0/x/x-1/x99 must be Err.
- Seed: encode_blr_pbt::encode_blr_neg_invalid_name
- Formal: ∀ name ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_cinc([Reg(name), Reg("x0"), Cond("eq")]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name]
  domain: { name: invalid_reg_name }
  relation:
    op: holds
    expr: encode_cinc([Reg(name), Reg("x0"), Cond("eq")]).is_err()
generators:
  name: { gen: string }
expected_error: String
evidence: encoder/mod.rs:131-148 parse_reg_num returns None for x32/foo/empty; get_reg encoder/mod.rs:956-966; llvm-mc rejects x32
```

## encode_cinc_neg_bad_operand_kind
- Tier: 5
- Rationale: Coverage sweep — get_reg non-Reg arm and cond match `_` with Some(non-Cond). CINC operands 0 and 1 must be registers; operand 2 must be a condition. Imm/Mem/Symbol/Shift/Label in any slot must Err.
- Seed: encode_blr_pbt::encode_blr_neg_bad_operand
- Formal: ∀ slot ∈ {0,1,2}, bad ∈ {Imm, Mem, Symbol, Shift, Label}. encode_cinc(ops with ops[slot]=bad) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cinc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, which]
  domain: { slot: 0..2, which: 0..4 }
  relation:
    op: holds
    expr: encode_cinc(ops_with_bad_at(slot, which)).is_err()
generators:
  slot: { gen: int, min: 0, max: 2, type: u32 }
  which: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: get_reg encoder/mod.rs:956-966 expected register; compare_branch.rs:296-298 expected condition code as third operand
```
