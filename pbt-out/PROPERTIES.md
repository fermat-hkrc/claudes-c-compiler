# Properties: encode_cmn

## encode_cmn_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (gas-compat README). State machine rejected (pure function). Round-trip rejected (no CMN decoder). encode_cmp / encode_add_sub fail the same-job sibling gate as differential references (CMP is SUBS-XZR; ADDS is 3-operand architectural form).
- Seed: encode_add_sub_pbt encode_add_sub_diff_imm (data_processing.rs)
- Formal: ∀ rn ∈ GPR∪{sp,wsp,lr}, ∀ imm ∈ Imm12Domain. llvm-mc("cmn rn, #imm") succeeds ⇒ encode_cmn([Reg(rn), Imm(imm)]) = Word(llvm-mc word). Imm12Domain = {0..4095} ∪ {N<<12 | N∈1..4095}. Bounds 0, 1, 4095, 4096, 16773120 forced.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, imm]
  domain: { rn: gpr_or_sp, imm: imm12_or_shifted }
  relation:
    op: eq
    lhs: encode_cmn([Reg(rn), Imm(imm)])
    rhs: llvm_mc_word("cmn " + rn + ", #" + imm)
generators:
  rn: { gen: string }
  imm: { gen: int, min: 0, max: 16773120, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; README.md:218 cmn; compare_branch.rs:23 CMN->ADDS XZR
```

## encode_cmn_diff_reg_llvm_mc
- Tier: 2
- Rationale: Same differential oracle for the shifted-register form. Same-width GPR (XZR/WZR allowed as Rn/Rm; SP as Rm is invalid without extend and is excluded from this domain). Shift kind in {lsl,lsr,asr} with amount in 0..63 (X) / 0..31 (W), including the documented bounds.
- Seed: encode_add_sub_pbt encode_add_sub_diff_shifted_reg
- Formal: ∀ (rn, rm) same-width GPR, ∀ shift ∈ {ε} ∪ {(lsl|lsr|asr, amt in range)}. llvm-mc("cmn rn, rm{, shift}") succeeds ⇒ encode_cmn matches that word.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm, shift_kind, shift_amt]
  domain: { rn: same_width_gpr, rm: same_width_gpr }
  relation:
    op: eq
    lhs: encode_cmn(reg_or_shifted(rn, rm, shift_kind, shift_amt))
    rhs: llvm_mc_word("cmn " + rn + ", " + rm + shift_suffix)
generators:
  rn: { gen: string }
  rm: { gen: string }
  shift_kind: { gen: string }
  shift_amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14; compare_branch.rs:23; ARM ARM CMN (shifted register)
```

## encode_cmn_meta_vs_adds
- Tier: 4c
- Rationale: Documented alias CMN Rn, op → ADDS XZR/WZR, Rn, op (function comment + ARM ARM). encode_add_sub is not a same-job differential (different mnemonic/arity) so it is used only as a metamorphic transform. Stronger differential is the llvm-mc properties above.
- Seed: encode_cinv_pbt encode_cinv_meta_vs_csinv
- Formal: ∀ ops of the form [Reg(rn), Imm|Reg, optional Shift|Extend]. encode_cmn(ops) = encode_add_sub([Reg(ZR)] ++ ops, is_sub=false, set_flags=true) where ZR = WZR if is_32bit_reg(rn) else XZR.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rn, op2]
  domain: { rn: gpr_or_sp }
  relation:
    op: eq
    lhs: encode_cmn([Reg(rn), op2])
    rhs: encode_add_sub([Reg(zr_of(rn)), Reg(rn), op2], false, true)
generators:
  rn: { gen: string }
  op2: { gen: string }
evidence: compare_branch.rs:23 CMN Rn, op -> ADDS XZR, Rn, op; encoder/mod.rs:226 adds dispatch; ARM ARM CMN alias of ADDS
```

## encode_cmn_word_layout_imm
- Tier: 4d
- Rationale: ARM ARM Add/subtract (immediate) with S=1, op=0, Rd=31 (XZR/WZR). Weaker than differential; pins field layout independently of llvm-mc parsing.
- Seed: encode_cinv_pbt encode_cinv_word_layout
- Formal: ∀ rn ∈ {x0..x30,sp,lr,w0..w30,wsp}, ∀ imm ∈ 0..4095. encode_cmn([Reg(rn), Imm(imm)]) = Word(w) where w[4:0]=31, w[29]=1 (S), w[30]=0 (op ADD), w[28:24]=0b10001, w[31]=sf(rn), w[21:10]=imm, w[9:5]=regnum(rn).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rn, imm]
  domain: { rn: gpr_or_sp, imm: 0..=4095 }
  relation:
    op: holds
    expr: word_fields_match_cmn_imm(encode_cmn([Reg(rn), Imm(imm)]), rn, imm)
generators:
  rn: { gen: string }
  imm: { gen: int, min: 0, max: 4095, type: i64 }
evidence: compare_branch.rs:23; data_processing.rs:347 ADD-immediate word; ARM ARM Add/subtract (immediate) Rd=XZR
```

## encode_cmn_neg_arity
- Tier: 4e
- Rationale: llvm-mc rejects bare `cmn` and `cmn x0` ("too few operands"). encode_add_sub requires 3 operands after ZR prepend, so 0 or 1 input operands must Err. Documented error contract from gas-compat.
- Seed: encode_cinv_pbt encode_cinv_neg_arity
- Formal: ∀ ops with |ops| ∈ {0,1}. encode_cmn(ops) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..=1 }
  relation:
    op: throws
    expr: encode_cmn(ops_of_len(arity))
generators:
  arity: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: llvm-mc "too few operands"; data_processing.rs:292 add/sub requires 3 operands
```

## encode_cmn_neg_imm_oor
- Tier: 4e
- Rationale: ARM ARM / llvm-mc reject immediates that are not an unshifted imm12 and not (imm12 << 12). Bounds 4097, 16773121 (=0xFFF<<12 + 1), and negative values whose absolute value is likewise unencodable. Documented range [0,4095] unshifted or N<<12.
- Seed: encode_add_sub_pbt encode_add_sub_neg_imm_out_of_range
- Formal: ∀ rn ∈ GPR, ∀ imm ∈ UnencodableImm. encode_cmn([Reg(rn), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: rn="x0", imm=-9223372036854775808 (i64::MIN) — panics in encode_add_sub at data_processing.rs:314 instead of Err
- Bug report: pbt-out/bug_reports/encode_cmn_imm_min_overflow.md

```property
function: encoder.compare_branch.encode_cmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, imm]
  domain: { imm: unencodable_add_imm }
  relation:
    op: throws
    expr: encode_cmn([Reg(rn), Imm(imm)])
generators:
  rn: { gen: string }
  imm: { gen: int, type: i64 }
expected_error: String
evidence: llvm-mc "integer in range [0, 4095]"; data_processing.rs:334 immediate does not fit in add/sub imm12 encoding
```

## encode_cmn_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects a trailing extra operand that is not a valid shift/extend (`cmn x0, x1, x2`, extra Imm/Mem/Symbol). Gas-compat requires Err.
- Seed: encode_cinv_pbt encode_cinv_neg_extra_operand
- Formal: ∀ valid 2-operand CMN ops, ∀ extra ∈ {Reg, Imm, Symbol, Mem}. encode_cmn(ops ++ [extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: [Reg("x0"), Reg("x0"), Reg("x2")]  (pair=("x0","x0"), which=0)
- Bug report: pbt-out/bug_reports/encode_cmn_extra_operand.md

```property
function: encoder.compare_branch.encode_cmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, rm, extra]
  domain: { extra: non_shift_extend_operand }
  relation:
    op: throws
    expr: encode_cmn([Reg(rn), Reg(rm), extra])
generators:
  rn: { gen: string }
  rm: { gen: string }
  extra: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc "invalid operand" / "expected sxtx uxtx or lsl"; README.md:5-14 gas-compat
```

## encode_cmn_neg_wrong_reg
- Tier: 4e
- Rationale: llvm-mc rejects XZR/WZR as immediate-form Rn, mixed x/w without extend, FP/SIMD names, SP as Rm without extend, and invalid register names. Gas-compat requires Err.
- Seed: encode_cinv_pbt encode_cinv_neg_wrong_reg
- Formal: ∀ ops in WrongRegDomain. encode_cmn(ops) is Err. WrongRegDomain includes [xzr|#imm], [wzr|#imm], mixed x/w, d/s/q/v/h/b names, [xN, sp], invalid names (x32, foo, empty).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind=0, n=0, imm=0 → [Reg("xzr"), Imm(0)]  (`cmn xzr, #0`). Related witnesses: wzr+#0, mixed x/w, d0+#0, x0+sp.
- Bug report: pbt-out/bug_reports/encode_cmn_xzr_imm.md

```property
function: encoder.compare_branch.encode_cmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: 0..=8 }
  relation:
    op: throws
    expr: encode_cmn(wrong_reg_ops(kind, n))
generators:
  kind: { gen: int, min: 0, max: 8, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: llvm-mc rejects cmn xzr, #0 / cmn d0, #0 / cmn x0, w1 / cmn x0, sp; README.md:5-14 gas-compat
```

## encode_cmn_diff_extend_llvm_mc
- Tier: 2
- Rationale: Coverage sweep — ARM ARM CMN (extended register) was not exercised in the first batch. Differential vs llvm-mc. Amount bounds 0 and 4 forced.
- Seed: encode_add_sub_pbt encode_add_sub_diff_extended_and_sp
- Formal: ∀ (rn, rm, extend, amt∈0..4) in the gas-valid extended-register CMN domain. encode_cmn([Reg(rn), Reg(rm), Extend(extend, amt)]) = Word(llvm-mc("cmn rn, rm, extend #amt")).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm, extend, amt]
  domain: { amt: 0..=4 }
  relation:
    op: eq
    lhs: encode_cmn([Reg(rn), Reg(rm), Extend(extend, amt)])
    rhs: llvm_mc_word("cmn " + rn + ", " + rm + ", " + extend + " #" + amt)
generators:
  rn: { gen: string }
  rm: { gen: string }
  extend: { gen: string }
  amt: { gen: int, min: 0, max: 4, type: u32 }
evidence: ARM ARM CMN (extended register); llvm-mc cmn x0, w1, sxtw; data_processing.rs:388-407
```

## encode_cmn_diff_neg_imm_llvm_mc
- Tier: 2
- Rationale: Coverage sweep — gas/llvm-mc rewrite `cmn Rn, #-N` as `cmp Rn, #N` for N in 1..4095. First-batch imm generator was non-negative only. Differential vs llvm-mc. Bounds -1 and -4095 forced.
- Seed: encode_add_sub_pbt encode_add_sub_metamorphic_neg_imm
- Formal: ∀ rn ∈ GPR∪{sp,wsp,lr}, ∀ n ∈ 1..4095. encode_cmn([Reg(rn), Imm(-n)]) = Word(llvm-mc("cmn rn, #-n")).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, n]
  domain: { n: 1..=4095 }
  relation:
    op: eq
    lhs: encode_cmn([Reg(rn), Imm(-n)])
    rhs: llvm_mc_word("cmn " + rn + ", #-" + n)
generators:
  rn: { gen: string }
  n: { gen: int, min: 1, max: 4095, type: i64 }
evidence: llvm-mc rewrites cmn x0, #-42 as cmp x0, #42; README.md:5-14 gas-compat; data_processing.rs:312-316 negative-imm alias
```

## encode_cmn_neg_non_reg_first
- Tier: 4e
- Rationale: Coverage sweep — encode_cmn's `operands.first()` else-arm (first operand not Reg → treat as 64-bit XZR) is not hit by arity-0/1 tests that fail earlier. A non-register first operand with a second operand must still Err (llvm-mc: invalid operand).
- Seed: encode_cinv_pbt encode_cinv_neg_bad_operand_kind
- Formal: ∀ first ∈ {Imm, Symbol, Mem, Cond, Shift}, ∀ second ∈ {Reg(x0), Imm(0)}. encode_cmn([first, second]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind]
  domain: { kind: 0..=4 }
  relation:
    op: throws
    expr: encode_cmn([non_reg_first(kind), Operand::Imm(0)])
generators:
  kind: { gen: int, min: 0, max: 4, type: u32 }
expected_error: String
evidence: compare_branch.rs:25-29 else { false } when first is not Reg; llvm-mc rejects non-GPR Rn; get_reg expected register at operand 1
```
