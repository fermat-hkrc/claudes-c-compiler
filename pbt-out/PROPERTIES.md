# Properties: encode_cmp

## encode_cmp_diff_imm_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (gas-compat README). State machine rejected (pure function). Round-trip rejected (no CMP decoder). encode_cmn / encode_add_sub fail the same-job sibling gate as differential references (CMN is ADDS-XZR; SUBS is 3-operand architectural form).
- Seed: encode_cmn_pbt encode_cmn_diff_imm_llvm_mc (compare_branch.rs)
- Formal: ∀ rn ∈ GPR∪{sp,wsp,lr}, ∀ imm ∈ Imm12Domain. llvm-mc("cmp rn, #imm") succeeds ⇒ encode_cmp([Reg(rn), Imm(imm)]) = Word(llvm-mc word). Imm12Domain = {0..4095} ∪ {N<<12 | N∈1..4095} ∪ explicit lsl#12. Bounds 0, 1, 4095, 4096, 16773120 forced.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, imm]
  domain: { rn: gpr_or_sp, imm: imm12_or_shifted }
  relation:
    op: eq
    lhs: encode_cmp([Reg(rn), Imm(imm)])
    rhs: llvm_mc_word("cmp " + rn + ", #" + imm)
generators:
  rn: { gen: string }
  imm: { gen: int, min: 0, max: 16773120, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compat; README.md:218 cmp; compare_branch.rs:7 CMP->SUBS XZR
```

## encode_cmp_diff_reg_llvm_mc
- Tier: 2
- Rationale: Same differential oracle for the shifted-register form. Same-width GPR (XZR/WZR allowed as Rn/Rm; SP as Rm is invalid without extend and is excluded from this domain). Shift kind in {lsl,lsr,asr} with amount in 0..63 (X) / 0..31 (W), including the documented bounds 0/31/63.
- Seed: encode_cmn_pbt encode_cmn_diff_reg_llvm_mc
- Formal: ∀ (rn, rm) same-width GPR, ∀ shift ∈ {ε} ∪ {(lsl|lsr|asr, amt in range)}. llvm-mc("cmp rn, rm{, shift}") succeeds ⇒ encode_cmp matches that word.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm, shift_kind, shift_amt]
  domain: { rn: same_width_gpr, rm: same_width_gpr }
  relation:
    op: eq
    lhs: encode_cmp(reg_or_shifted(rn, rm, shift_kind, shift_amt))
    rhs: llvm_mc_word("cmp " + rn + ", " + rm + shift_suffix)
generators:
  rn: { gen: string }
  rm: { gen: string }
  shift_kind: { gen: string }
  shift_amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14; compare_branch.rs:7; ARM ARM CMP (shifted register)
```

## encode_cmp_meta_vs_subs
- Tier: 4c
- Rationale: Documented alias CMP Rn, op → SUBS XZR/WZR, Rn, op (function comment + ARM ARM). encode_add_sub is not a same-job differential (different mnemonic/arity) so it is used only as a metamorphic transform. Stronger differential is the llvm-mc properties above.
- Seed: encode_cmn_pbt encode_cmn_meta_vs_adds
- Formal: ∀ ops of the form [Reg(rn), Imm|Reg, optional Shift|Extend]. encode_cmp(ops) = encode_add_sub([Reg(ZR)] ++ ops, is_sub=true, set_flags=true) where ZR = WZR if is_32bit_reg(rn) else XZR.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rn, op2]
  domain: { rn: gpr_or_sp }
  relation:
    op: eq
    lhs: encode_cmp([Reg(rn), op2])
    rhs: encode_add_sub([Reg(zr_of(rn)), Reg(rn), op2], true, true)
generators:
  rn: { gen: string }
evidence: compare_branch.rs:7 CMP Rn, op -> SUBS XZR, Rn, op; ARM ARM CMP alias of SUBS
```

## encode_cmp_word_layout_imm
- Tier: 4d
- Rationale: ARM ARM Add/subtract (immediate) SUBS field layout with Rd=XZR. Weaker than differential; pins opcode bits independently of llvm-mc.
- Seed: encode_cmn_pbt encode_cmn_word_layout_imm
- Formal: ∀ rn ∈ GPR∪{sp,wsp,lr}, ∀ imm ∈ 0..4095. encode_cmp([Reg(rn), Imm(imm)]) = Word(w) ⇒ Rd[4:0]=31 ∧ S[29]=1 ∧ op[30]=1 ∧ bits[28:24]=0b10001 ∧ sf[31]=sf(rn) ∧ sh[22]=0 ∧ imm12[21:10]=imm ∧ Rn[9:5]=reg_num(rn).
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rn, imm]
  domain: { rn: gpr_or_sp, imm: 0..4095 }
  relation:
    op: holds
    expr: word_layout_subs_imm(encode_cmp([Reg(rn), Imm(imm)]), rn, imm)
generators:
  rn: { gen: string }
  imm: { gen: int, min: 0, max: 4095, type: i64 }
evidence: ARM ARM Add/subtract (immediate) SUBS; compare_branch.rs:7 Rd=XZR
```

## encode_cmp_neg_arity
- Tier: 4e
- Rationale: llvm-mc and gas reject cmp with fewer than 2 operands ("too few operands"). encode_add_sub requires 3 operands after ZR is prepended, so 0 or 1 user operands must Err.
- Seed: encode_cmn_pbt encode_cmn_neg_arity
- Formal: ∀ arity ∈ {0,1}. encode_cmp(ops) with |ops|=arity is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [arity]
  domain: { arity: 0..1 }
  relation:
    op: throws
    expr: encode_cmp(ops_of_len(arity))
expected_error: String
generators:
  arity: { gen: int, min: 0, max: 1, type: u32 }
evidence: llvm-mc "too few operands"; encode_add_sub requires 3 operands
```

## encode_cmp_neg_imm_oor
- Tier: 4e
- Rationale: ARM ARM / llvm-mc reject immediates outside imm12 and (imm12<<12). Documented bounds sampled at 4097, 8191, 16773121, i64::MAX, i64::MIN, and negative values that do not rewrite to an encodable CMN.
- Seed: encode_cmn_pbt encode_cmn_neg_imm_oor
- Formal: ∀ rn ∈ GPR∪{sp,wsp,lr}, ∀ imm ∉ Imm12Domain ∪ NegImm12Domain. encode_cmp([Reg(rn), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: rn = "x0", imm = -9223372036854775808 (i64::MIN); panics in encode_add_sub at data_processing.rs:314
- Bug report: pbt-out/bug_reports/encode_cmp_imm_min_overflow.md

```property
function: encoder.compare_branch.encode_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, imm]
  domain: { rn: gpr_or_sp, imm: unencodable_imm }
  relation:
    op: throws
    expr: encode_cmp([Reg(rn), Imm(imm)])
expected_error: String
generators:
  rn: { gen: string }
  imm: { gen: int, type: i64 }
evidence: ARM ARM ADD/SUB imm12; llvm-mc "integer in range [0, 4095]"
```

## encode_cmp_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc rejects a third register/imm/symbol/mem operand after `cmp Rn, Rm` ("expected sxtx/uxtx or lsl"). Extra operands must Err, not be silently dropped.
- Seed: encode_cmn_pbt encode_cmn_neg_extra_operand
- Formal: ∀ (rn, rm) same-width GPR, ∀ extra ∈ {Reg, Imm, Symbol, Mem}. encode_cmp([Reg(rn), Reg(rm), extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: pair = ("x0", "x0"), which = 0 → [Reg("x0"), Reg("x0"), Reg("x2")]
- Bug report: pbt-out/bug_reports/encode_cmp_extra_operand.md

```property
function: encoder.compare_branch.encode_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, rm, extra]
  domain: { rn: same_width_gpr, rm: same_width_gpr }
  relation:
    op: throws
    expr: encode_cmp([Reg(rn), Reg(rm), extra])
expected_error: String
generators:
  rn: { gen: string }
  rm: { gen: string }
evidence: llvm-mc "expected sxtx uxtx or lsl"; README.md:5-14 gas-compat
```

## encode_cmp_neg_wrong_reg
- Tier: 4e
- Rationale: llvm-mc rejects XZR/WZR as immediate-form Rn (Rn=31 is SP), mixed x/w without extend, FP/SIMD names, SP as Rm without extend, and invalid register names. Gas-compat requires the same rejections.
- Seed: encode_cmn_pbt encode_cmn_neg_wrong_reg
- Formal: ∀ ops in WrongRegDomain. encode_cmp(ops) is Err. WrongRegDomain = { [xzr|#imm], [wzr|#imm], mixed-width pair, [dN|#0], [xN|dN], [xN|sp], invalid name, [sN|w0] }.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: failing
- Counterexample: kind = 0, n = 0, imm = 0 → [Reg("xzr"), Imm(0)]; also wzr-imm, mixed x/w, d0, x0/sp
- Bug report: pbt-out/bug_reports/encode_cmp_xzr_imm.md

```property
function: encoder.compare_branch.encode_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: wrong_reg_cmp }
  relation:
    op: throws
    expr: encode_cmp(ops)
expected_error: String
generators:
  ops: { gen: list, elem: { gen: string } }
evidence: llvm-mc rejects XZR-imm / mixed / FP / SP-Rm; ARM ARM CMP immediate Rn is Xn|SP
```

## encode_cmp_diff_extend_llvm_mc
- Tier: 2
- Rationale: Coverage-sweep differential for the extended-register form (sxtw/uxtw/sxtx/uxtx, amount 0..4 including bounds). Documented ARM ARM CMP (extended register) and llvm-mc-accepted.
- Seed: encode_cmn_pbt encode_cmn_diff_extend_llvm_mc
- Formal: ∀ (rn, rm, ext, amt) in ExtendDomain. llvm-mc("cmp rn, rm, ext{#amt}") succeeds ⇒ encode_cmp matches that word. Amount bounds 0 and 4 forced.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, rm, ext, amt]
  domain: { amt: 0..4 }
  relation:
    op: eq
    lhs: encode_cmp([Reg(rn), Reg(rm), Extend(ext, amt)])
    rhs: llvm_mc_word("cmp " + rn + ", " + rm + ", " + ext)
generators:
  amt: { gen: int, min: 0, max: 4, type: u32 }
evidence: ARM ARM CMP (extended register); llvm-mc accepts sxtw/uxtw/sxtx/uxtx amount 0..4
```

## encode_cmp_diff_neg_imm_llvm_mc
- Tier: 2
- Rationale: Coverage-sweep differential for gas rewrite `cmp Rn, #-N` → `cmn Rn, #N`. Documented in encode_add_sub ("Handle negative immediates") and llvm-mc.
- Seed: encode_cmn_pbt encode_cmn_diff_neg_imm_llvm_mc
- Formal: ∀ rn ∈ GPR∪{sp,wsp,lr}, ∀ N ∈ 1..4095. encode_cmp([Reg(rn), Imm(-N)]) = Word(llvm-mc("cmp rn, #-N")). Bounds 1 and 4095 forced.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, n]
  domain: { rn: gpr_or_sp, n: 1..4095 }
  relation:
    op: eq
    lhs: encode_cmp([Reg(rn), Imm(-n)])
    rhs: llvm_mc_word("cmp " + rn + ", #-" + n)
generators:
  n: { gen: int, min: 1, max: 4095, type: i64 }
evidence: data_processing.rs:312-316 negative-imm alias; llvm-mc gas rewrite cmp #-N → cmn #N
```

## encode_cmp_neg_non_reg_first
- Tier: 4e
- Rationale: Coverage-sweep negative contract for a non-register first operand. encode_cmp only special-cases Operand::Reg for width; any other first operand still prepends XZR and encode_add_sub must Err on get_reg of a non-Reg at index 1.
- Seed: encode_cmn_pbt encode_cmn_neg_non_reg_first
- Formal: ∀ first ∈ {Imm, Symbol, Mem, Cond, Shift}, ∀ second ∈ {Reg("x0"), Imm(0)}. encode_cmp([first, second]) is Err.
- Test file: src/backend/arm/assembler/encoder/compare_branch.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.compare_branch.encode_cmp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, second]
  domain: { kind: 0..4 }
  relation:
    op: throws
    expr: encode_cmp([non_reg_first(kind), second])
expected_error: String
generators:
  kind: { gen: int, min: 0, max: 4, type: u32 }
evidence: get_reg requires Operand::Reg; llvm-mc rejects non-GPR first operand
```
