# Properties: encode_ccmp_ccmn

## encode_ccmp_ccmn_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential against llvm-mc (independent AArch64 assembler) for the ARM ARM immediate form `ccmp/ccmn Rn, #imm5, #nzcv, cond`. State machine rejected (pure function, no lifecycle). In-tree CCMP decoder does not exist so algebraic round-trip is unavailable. encode_cmp / encode_cmn are different jobs (SUBS/ADDS aliases without cond/nzcv) and fail the same-job sibling gate as a differential reference. Doc evidence: assembler README gas-compat; ARM ARM Conditional compare (immediate) sf op S 11010010 imm5 cond 1 0 Rn 0 nzcv; dispatch ccmp/ccmn.
- Seed: src/backend/arm/assembler/encoder/compare_branch.rs encode_cbz_pbt::encode_cbz_diff_imm_llvm_mc
- Formal: ∀ rn ∈ {x0..x30,xzr,lr,w0..w30,wzr}, is_ccmp ∈ {true,false}, imm5 ∈ {0..31}, nzcv ∈ {0..15}, cond ∈ Cond16. encode_ccmp_ccmn([Reg(rn), Imm(imm5), Imm(nzcv), Cond(cond)], is_ccmp) = Word(llvm-mc(mnemonic(is_ccmp) + " " + rn + ", #" + imm5 + ", #" + nzcv + ", " + cond)).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, is_ccmp, imm5, nzcv, cond]
  domain: { rn: gpr_name, is_ccmp: bool, imm5: u5, nzcv: u4, cond: cond_code }
  relation:
    op: eq
    lhs: encode_ccmp_ccmn([Reg(rn), Imm(imm5), Imm(nzcv), Cond(cond)], is_ccmp) as Word
    rhs: llvm_mc(ccmp_mnemonic(is_ccmp) + " " + rn + ", #" + imm5 + ", #" + nzcv + ", " + cond)
generators:
  rn: { gen: string }
  is_ccmp: { gen: bool }
  imm5: { gen: int, min: 0, max: 31, type: i64 }
  nzcv: { gen: int, min: 0, max: 15, type: i64 }
  cond: { gen: string }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; README.md:218 Compare lists ccmp; encoder/mod.rs:304-305 ccmp/ccmn dispatch; compare_branch.rs:53-54 CCMP/CCMN Rn #imm5 #nzcv cond; ARM ARM Conditional compare (immediate)
```

## encode_ccmp_ccmn_diff_reg_llvm_mc
- Tier: 2
- Rationale: Differential against llvm-mc for the ARM ARM register form `ccmp/ccmn Rn, Rm, #nzcv, cond` with matching width. Stronger state-machine and round-trip rejected as above. Same-width GPR is required by gas (llvm-mc rejects mixed x/w). Doc evidence: parser.rs:1987 `ccmp x10, x13, 0, eq`; ARM ARM Conditional compare (register).
- Seed: encode_blr_pbt::encode_blr_diff_xn_llvm_mc
- Formal: ∀ (rn, rm) same-width GPR pair, is_ccmp ∈ bool, nzcv ∈ {0..15}, cond ∈ Cond16. encode_ccmp_ccmn([Reg(rn), Reg(rm), Imm(nzcv), Cond(cond)], is_ccmp) = Word(llvm-mc(mnemonic(is_ccmp) + " " + rn + ", " + rm + ", #" + nzcv + ", " + cond)).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm, is_ccmp, nzcv, cond]
  domain: { rn: gpr_name, rm: gpr_name_same_width, is_ccmp: bool, nzcv: u4, cond: cond_code }
  relation:
    op: eq
    lhs: encode_ccmp_ccmn([Reg(rn), Reg(rm), Imm(nzcv), Cond(cond)], is_ccmp) as Word
    rhs: llvm_mc(ccmp_mnemonic(is_ccmp) + " " + rn + ", " + rm + ", #" + nzcv + ", " + cond)
generators:
  rn: { gen: string }
  rm: { gen: string }
  is_ccmp: { gen: bool }
  nzcv: { gen: int, min: 0, max: 15, type: i64 }
  cond: { gen: string }
evidence: parser.rs:1987 ccmp x10, x13, 0, eq; encoder/mod.rs:304-305; ARM ARM Conditional compare (register) sf op S 11010010 Rm cond 0 0 Rn 0 nzcv
```

## encode_ccmp_ccmn_meta_ccmp_vs_ccmn
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM CCMN is CCMP with bit 30 clear (op field). encode_ccmp_ccmn(..., true) is not a same-job differential reference; the relation is the documented opcode pair. Stronger differential already covers the happy path; this isolates the CCMP-vs-CCMN contract independently of llvm-mc. Doc evidence: compare_branch.rs:54 "The only difference: CCMP has bit 30 = 1, CCMN has bit 30 = 0".
- Seed: encode_cbz_pbt::encode_cbz_meta_cbz_vs_cbnz
- Formal: ∀ valid 4-operand CCMP encoding (imm or reg form). encode_ccmp_ccmn(ops, true).word XOR encode_ccmp_ccmn(ops, false).word = 1<<30.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: valid_ccmp_operands }
  relation:
    op: eq
    lhs: encode_ccmp_ccmn(ops, true).word XOR encode_ccmp_ccmn(ops, false).word
    rhs: 1 << 30
generators:
  ops: { gen: string }
evidence: compare_branch.rs:54 CCMP bit 30 = 1, CCMN bit 30 = 0; ARM ARM Conditional compare op field
```

## encode_ccmp_ccmn_word_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM field layout. Stronger differential already covers numeric agreement; this pins individual fields (sf, op, S, opcode, o2, Rn, Rm/imm5, cond, nzcv) so a coincidental word match cannot hide a field swap.
- Seed: encode_blr_pbt::encode_blr_word_layout
- Formal: ∀ rn_num ∈ 0..31, is_64 ∈ bool, is_ccmp ∈ bool, imm5 ∈ 0..31, nzcv ∈ 0..15, cond_val ∈ 0..15. let w = encode_ccmp_ccmn(imm-form).word. (w>>31)=sf ∧ ((w>>30)&1)=op ∧ ((w>>29)&1)=1 ∧ ((w>>21)&0xFF)=0b11010010 ∧ ((w>>16)&0x1F)=imm5 ∧ ((w>>12)&0xF)=cond_val ∧ ((w>>11)&1)=1 ∧ ((w>>5)&0x1F)=rn_num ∧ (w&0xF)=nzcv ∧ ((w>>10)&1)=0 ∧ ((w>>4)&1)=0. Register form identical except o2=0 and (w>>16)&0x1F = rm_num.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rn_num, is_64, is_ccmp, imm5, nzcv, cond_val]
  domain: { rn_num: u5, is_64: bool, is_ccmp: bool, imm5: u5, nzcv: u4, cond_val: u4 }
  body: word fields match ARM ARM Conditional compare (immediate)
generators:
  rn_num: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  is_ccmp: { gen: bool }
  imm5: { gen: int, min: 0, max: 31, type: i64 }
  nzcv: { gen: int, min: 0, max: 15, type: i64 }
  cond_val: { gen: int, min: 0, max: 15, type: u32 }
evidence: ARM ARM Conditional compare (immediate/register); compare_branch.rs:53-76
```

## encode_ccmp_ccmn_neg_arity
- Tier: 5
- Rationale: Negative/error contract from llvm-mc/gas: CCMP/CCMN requires four operands (Rn, Rm-or-imm5, nzcv, cond). Stronger oracles do not apply to the invalid domain. llvm-mc reports "too few operands" for 0..3 operands.
- Seed: encode_blr_pbt::encode_blr_neg_arity
- Formal: ∀ is_ccmp ∈ bool, ops with |ops| < 4. encode_ccmp_ccmn(ops, is_ccmp) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_ccmp, n]
  domain: { is_ccmp: bool, n: 0..3 }
  relation:
    op: throws
    expr: encode_ccmp_ccmn(ops_of_len(n), is_ccmp)
generators:
  is_ccmp: { gen: bool }
  n: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc aarch64 rejects ccmp with fewer than 4 operands (too few operands)
```

## encode_ccmp_ccmn_neg_imm5_nzcv_oor
- Tier: 5
- Rationale: Negative/error contract from llvm-mc/ARM ARM: imm5 must be in [0, 31] and nzcv in [0, 15]. Bounds 0/31/32/-1 and 0/15/16/-1 are sampled exactly. Stronger differential does not apply on the invalid domain. SUT currently masks with 0x1F/0xF so this is the bound-straddle property most likely to fail.
- Seed: encode_bl_pbt::encode_bl_neg_imm_unaligned_oor
- Formal: ∀ rn ∈ GPR, is_ccmp ∈ bool, cond ∈ Cond16, (imm5, nzcv) with imm5 ∉ [0,31] ∨ nzcv ∉ [0,15]. encode_ccmp_ccmn([Reg(rn), Imm(imm5), Imm(nzcv), Cond(cond)], is_ccmp) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Imm(-1), Imm(0), Cond("eq")], is_ccmp=false  (also nzcv=16)
- Bug report: pbt-out/bug_reports/encode_ccmp_ccmn_imm5_nzcv_truncated.md

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, is_ccmp, imm5, nzcv, cond]
  domain: { rn: gpr_name, is_ccmp: bool, imm5: oor_or_valid_i64, nzcv: oor_or_valid_i64, cond: cond_code }
  relation:
    op: throws
    expr: encode_ccmp_ccmn([Reg(rn), Imm(imm5), Imm(nzcv), Cond(cond)], is_ccmp)
generators:
  rn: { gen: string }
  is_ccmp: { gen: bool }
  imm5: { gen: int, min: -2, max: 33, type: i64 }
  nzcv: { gen: int, min: -2, max: 17, type: i64 }
  cond: { gen: string }
expected_error: String
evidence: llvm-mc aarch64 imm5 in range 0 to 31 and nzcv in range 0 to 15; ARM ARM imm5 5-bit unsigned, nzcv 4-bit
```

## encode_ccmp_ccmn_neg_extra_operand
- Tier: 5
- Rationale: Negative/error contract from llvm-mc: a fifth operand is invalid. Stronger oracles do not apply. SUT only inspects indices 0..3 so extra operands are the documented extra-operand bug class from sibling encoders.
- Seed: encode_blr_pbt::encode_blr_neg_extra_operand
- Formal: ∀ valid 4-operand CCMP ops, extra ∈ Operand, is_ccmp ∈ bool. encode_ccmp_ccmn(ops ++ [extra], is_ccmp) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Imm(0), Imm(0), Cond("eq"), Reg("x1")], is_ccmp=false
- Bug report: pbt-out/bug_reports/encode_ccmp_ccmn_extra_operand.md

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops, extra, is_ccmp]
  domain: { ops: valid_ccmp_operands, extra: Operand, is_ccmp: bool }
  relation:
    op: throws
    expr: encode_ccmp_ccmn(ops ++ [extra], is_ccmp)
generators:
  is_ccmp: { gen: bool }
  extra: { gen: string }
  ops: { gen: string }
expected_error: String
evidence: llvm-mc aarch64 rejects ccmp x0, #0, #0, eq, x1 with invalid operand
```

## encode_ccmp_ccmn_neg_wrong_reg_class
- Tier: 5
- Rationale: Negative/error contract from llvm-mc/ARM ARM: Rn/Rm are Wt/Xt (XZR/WZR for 31), never SP/WSP, never FP/SIMD, never mixed x/w. Invalid names (x32, foo) also Err. Stronger oracles do not apply on the invalid domain.
- Seed: encode_blr_pbt::encode_blr_neg_wrong_reg_class
- Formal: ∀ is_ccmp ∈ bool, name ∈ {sp, wsp, dN, sN, qN, vN, hN, bN, x32, w32, foo, "", r0, x, x-1, x99}. encode_ccmp_ccmn([Reg(name), Imm(0), Imm(0), Cond("eq")], is_ccmp) = Err. Also mixed-width Rn/Rm (xN, wM) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("sp"), Imm(0), Imm(0), Cond("eq")], is_ccmp=false  (also d0 FP)
- Bug report: pbt-out/bug_reports/encode_ccmp_ccmn_sp_as_zr.md ; pbt-out/bug_reports/encode_ccmp_ccmn_fp_reg.md

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, is_ccmp]
  domain: { name: invalid_or_wrong_class_reg, is_ccmp: bool }
  relation:
    op: throws
    expr: encode_ccmp_ccmn([Reg(name), Imm(0), Imm(0), Cond("eq")], is_ccmp)
generators:
  name: { gen: string }
  is_ccmp: { gen: bool }
expected_error: String
evidence: llvm-mc aarch64 rejects ccmp sp, wsp, d0 and mixed-width ccmp x0, w1; ARM ARM Rn/Rm are GPR Wt/Xt with 31=XZR/WZR
```

## encode_ccmp_ccmn_neg_mixed_width
- Tier: 5
- Rationale: Negative/error contract from llvm-mc: CCMP/CCMN register form requires matching GPR width. Mixed x/w is rejected by gas. Stronger oracles do not apply on the invalid domain.
- Seed: encode_ccmp_ccmn_neg_wrong_reg_class
- Formal: ∀ n,m ∈ 0..30, is_ccmp ∈ bool, nzcv ∈ [0,15], cond ∈ Cond16. encode_ccmp_ccmn([Reg(xN), Reg(wM), Imm(nzcv), Cond(cond)], is_ccmp) = Err ∧ encode_ccmp_ccmn([Reg(wN), Reg(xM), ...], is_ccmp) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("w0"), Reg("x0"), Imm(0), Cond("eq")], is_ccmp=false
- Bug report: pbt-out/bug_reports/encode_ccmp_ccmn_mixed_width.md

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, m, is_ccmp, nzcv, cond]
  domain: { n: 0..30, m: 0..30, is_ccmp: bool, nzcv: u4, cond: cond_code }
  relation:
    op: throws
    expr: encode_ccmp_ccmn([Reg(wN), Reg(xM), Imm(nzcv), Cond(cond)], is_ccmp)
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  m: { gen: int, min: 0, max: 30, type: u32 }
  is_ccmp: { gen: bool }
  nzcv: { gen: int, min: 0, max: 15, type: i64 }
  cond: { gen: string }
expected_error: String
evidence: llvm-mc aarch64 rejects mixed-width ccmp x0, w1 and ccmn w0, x0
```

## encode_ccmp_ccmn_neg_invalid_cond
- Tier: 5
- Rationale: Coverage-sweep negative/error contract for the encode_cond None arm. llvm-mc reports invalid condition code. Stronger oracles do not apply on the invalid domain.
- Seed: encode_ccmp_ccmn_neg_arity
- Formal: ∀ rn ∈ GPR, is_ccmp ∈ bool, use_imm ∈ bool, cond ∉ Cond16. encode_ccmp_ccmn([Reg(rn), Imm|Reg, Imm(0), Cond(cond)], is_ccmp) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, is_ccmp, cond]
  domain: { rn: gpr_name, is_ccmp: bool, cond: invalid_cond }
  relation:
    op: throws
    expr: encode_ccmp_ccmn([Reg(rn), Imm(0), Imm(0), Cond(cond)], is_ccmp)
generators:
  rn: { gen: string }
  is_ccmp: { gen: bool }
  cond: { gen: string }
expected_error: String
evidence: llvm-mc aarch64 rejects ccmp x0, #0, #0, xx with invalid condition code; encode_cond returns None for unknown names
```

## encode_ccmp_ccmn_neg_invalid_rm
- Tier: 5
- Rationale: Coverage-sweep negative/error contract for parse_reg_num None on Rm. Invalid Rm names must Err.
- Seed: encode_blr_pbt::encode_blr_neg_invalid_name
- Formal: ∀ n ∈ 0..30, is_ccmp ∈ bool, rm ∈ {x32, w32, foo, "", r0, x, x-1, x99}. encode_ccmp_ccmn([Reg(xN), Reg(rm), Imm(0), Cond("eq")], is_ccmp) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, is_ccmp, rm]
  domain: { n: 0..30, is_ccmp: bool, rm: invalid_reg_name }
  relation:
    op: throws
    expr: encode_ccmp_ccmn([Reg(xN), Reg(rm), Imm(0), Cond("eq")], is_ccmp)
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
  is_ccmp: { gen: bool }
  rm: { gen: string }
expected_error: String
evidence: parse_reg_num returns None for x32/foo/empty; llvm-mc rejects those Rm names
```

## encode_ccmp_ccmn_neg_bad_operand_kind
- Tier: 5
- Rationale: Coverage-sweep negative/error contract for the unsupported-operands fallthrough when slot 1/2/3 is Mem/Symbol/Shift/Extend/Label/Barrier rather than Imm/Reg/Cond.
- Seed: encode_cbz_pbt::encode_cbz_neg_bad_label_kind
- Formal: ∀ is_ccmp ∈ bool, slot ∈ {1,2,3}, bad ∈ {Mem, Symbol, Shift, Extend, Label, Barrier}. encode_ccmp_ccmn(ops with slot replaced by bad, is_ccmp) = Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_ccmp_ccmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_ccmp, slot, which]
  domain: { is_ccmp: bool, slot: 1..3, which: 0..5 }
  relation:
    op: throws
    expr: encode_ccmp_ccmn(ops_with_bad_kind(slot, which), is_ccmp)
generators:
  is_ccmp: { gen: bool }
  slot: { gen: int, min: 1, max: 3, type: u32 }
  which: { gen: int, min: 0, max: 5, type: u32 }
expected_error: String
evidence: compare_branch.rs:79-80 unsupported ccmp/ccmn operands when neither immediate nor register form matches
```
