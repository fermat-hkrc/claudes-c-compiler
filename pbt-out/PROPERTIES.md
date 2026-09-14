# Properties: encode_movk

## encode_movk_diff_imm_shift
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree MOVK decoder). encode_movz/encode_movn rejected by same-job sibling gate (opc 10/00 vs 11). Doc evidence: README.md:5-14 gas-compatible assembly; encoder/mod.rs:1-7 32-bit words; ARM ARM Move wide (immediate) MOVK `sf 11 100101 hw imm16 Rd`; llvm-mc `-triple=aarch64`.
- Seed: encode_madd_pbt::encode_madd_diff_gpr (same-file differential vs llvm-mc); codegen emit.rs:906-928
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ H(is_64). encode_movk([Reg(gpr(is_64,rd)), Imm(imm)] ++ shift(hw)) = llvm-mc("movk Rd, #imm[, lsl #(16*hw)]") where H(false)={0,1}, H(true)={0,1,2,3}, gpr(_,31)=xzr/wzr, and shift(0) may be omitted or `lsl #0`.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movk
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, is_64, imm, hw]
  domain:
    rd: 0..31
    is_64: bool
    imm: 0..65535
    hw: valid_hw(is_64)
  relation:
    op: eq
    lhs: encode_movk(reg_imm_optional_lsl(rd, is_64, imm, hw))
    rhs: llvm_mc(movk_asm(rd, is_64, imm, hw))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14 encoder/mod.rs:221 ARM ARM Move wide immediate MOVK
```

## encode_movk_metamorphic_sf
- Tier: 4
- Rationale: ARM ARM sf bit is the sole 32/64-bit distinguisher of otherwise-identical MOVK encodings. Stronger differential covers full-word agreement; this metamorphic isolates sf. Round-trip rejected (no decoder). Doc evidence: ARM ARM `sf 11 100101 hw imm16 Rd`; README.md size-inference `x`/`w` prefix.
- Seed: encode_madd_pbt::encode_madd_metamorphic_sf
- Formal: ∀ rd ∈ {0..31}, imm ∈ {0..65535}, hw ∈ {0,1}. encode_movk(X-ops) XOR encode_movk(W-ops) = 1<<31 at equal rd/imm/hw.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movk
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, imm, hw]
  domain:
    rd: 0..31
    imm: 0..65535
    hw: 0..1
  relation:
    op: eq
    lhs: encode_movk(x_ops) XOR encode_movk(w_ops)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Move wide immediate sf at bit 31 README.md size inference
```

## encode_movk_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an exact structural predicate on every success-path word. Differential is stronger for the whole word; this invariant pins each field so a single-bit drift is localizable. Doc evidence: ARM ARM `sf 11 100101 hw imm16 Rd`.
- Seed: encode_madd_pbt::encode_madd_invariant_arm_fields
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ H(is_64). let w = encode_movk(...). (w>>31)&1 = sf(is_64) ∧ (w>>23)&0xFF = 0b11100101 ∧ (w>>21)&3 = hw ∧ (w>>5)&0xFFFF = imm ∧ w&0x1F = rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movk
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, is_64, imm, hw]
  domain:
    rd: 0..31
    is_64: bool
    imm: 0..65535
    hw: valid_hw(is_64)
  body: word_fields_match_arm_movk(encode_movk(ops), rd, is_64, imm, hw)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: ARM ARM Move wide (immediate) MOVK sf 11 100101 hw imm16 Rd
```

## encode_movk_diff_abs_g
- Tier: 2
- Rationale: GNU `:abs_gN:` / `:abs_gN_nc:` modifiers select the Nth 16-bit chunk; for a constant expression this is the same instruction as `movk Rd, #chunk, lsl #(16*N)`. llvm-mc leaves a fixup on the modifier form, so the independent reference is llvm-mc of the resolved `movk Rd, #chunk, lsl #shift`. Doc evidence: data_processing.rs:179-181 (purpose of abs_g for movk); GNU as abs_g0..g3 semantics; ARM ARM hw field.
- Seed: resolve_abs_g_modifier comment data_processing.rs:179-181
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, val ∈ i64, kind ∈ K(is_64). encode_movk([Reg(gpr), Modifier{kind, symbol: decimal(val)}]) = llvm-mc("movk Rd, #(val>>shift)&0xFFFF [, lsl #shift]") where K(false)={abs_g0,abs_g0_nc,abs_g1,abs_g1_nc}, K(true)=K(false)∪{abs_g2,abs_g2_nc,abs_g3}, shift(kind) ∈ {0,16,32,48}.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movk
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, is_64, val, kind]
  domain:
    rd: 0..31
    is_64: bool
    val: i64
    kind: abs_g_kinds(is_64)
  relation:
    op: eq
    lhs: encode_movk(reg_and_abs_g_modifier(rd, is_64, kind, val))
    rhs: llvm_mc(resolved_movk_asm(rd, is_64, val, kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  val: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  kind: { gen: string }
evidence: data_processing.rs:179-181 GNU as abs_g0..abs_g3 chunk extract ARM ARM hw
```

## encode_movk_neg_imm_oob
- Tier: 5
- Rationale: llvm-mc and ARM ARM require imm16 ∈ [0, 65535]. Negative and >65535 must be rejected. Bounds 0, 65535 are in the valid properties; this property samples 65536, -1, i64::MIN/MAX and other out-of-range values. Doc evidence: llvm-mc "immediate must be an integer in range [0, 65535]".
- Seed: llvm-mc rejection of `movk x0, #65536` and `movk x0, #-1`
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∉ [0, 65535]. encode_movk([Reg(gpr), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, is_64=false, imm=-1 (movk w0, #-1)
- Bug report: pbt-out/bug_reports/encode_movk_imm_oob.md

```property
function: encoder.data_processing.encode_movk
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, imm]
  domain:
    rd: 0..31
    is_64: bool
    imm: i64_outside_imm16
  relation:
    op: throws
    expr: encode_movk(reg_and_imm(rd, is_64, imm))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc aarch64 immediate must be an integer in range 0 to 65535 ARM ARM imm16
```

## encode_movk_neg_invalid_shift
- Tier: 5
- Rationale: llvm-mc requires lsl with amount in {0,16,32,48} (W: {0,16}). Non-multiples, other shift kinds, and W-register lsl #32/#48 must Err. Documented bounds sampled at 8, 15, 17, 31, 32, 33, 47, 48, 49, 64. Doc evidence: llvm-mc "expected 'lsl' with optional integer 0, 16, 32 or 48".
- Seed: llvm-mc rejection of `movk x0, #42, lsl #8`, `lsr #16`, `movk w0, #42, lsl #32`
- Formal: ∀ rd ∈ {0..31}, is_64, imm ∈ [0,65535], (kind, amount) ∉ valid_lsl(is_64). encode_movk([Reg, Imm, Shift{kind,amount}]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, is_64=false, imm=0, kind="lsr", amount=0 (movk w0, #0, lsr #0)
- Bug report: pbt-out/bug_reports/encode_movk_invalid_shift.md

```property
function: encoder.data_processing.encode_movk
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, imm, kind, amount]
  domain:
    rd: 0..31
    is_64: bool
    imm: 0..65535
    kind: invalid_shift_kind
    amount: invalid_shift_amount
  relation:
    op: throws
    expr: encode_movk(reg_imm_shift(rd, is_64, imm, kind, amount))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  kind: { gen: string }
  amount: { gen: int, min: 0, max: 64, type: u32 }
expected_error: String
evidence: llvm-mc aarch64 expected lsl with integer 0 16 32 or 48 for X and 0 or 16 for W
```

## encode_movk_neg_arity_sp_fp
- Tier: 5
- Rationale: llvm-mc rejects too few operands, a fourth operand, SP/WSP (Rd=31 is ZR not SP), and FP/SIMD names. Gas-compatible assembler must reject the same. Doc evidence: llvm-mc "too few operands" / "invalid operand"; ARM ARM Rd is XZR/WZR at 31; parse_reg_num comment lists d/s/q/v/h/b as FP.
- Seed: encode_madd_pbt extra/SP/FP negatives; llvm-mc `movk sp, #1`, `movk d0, #1`, `movk x0`
- Formal: ∀ invalid operand lists in {len<2, extra after valid form, Rd ∈ {sp,wsp}, Rd FP-prefixed, invalid name}. encode_movk(ops) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: extra=[Reg("x0"), Imm(0), Reg("x0")]; sp=wsp+#0; fp=d0+#0. too_few and invalid_name subcases pass.
- Bug report: pbt-out/bug_reports/encode_movk_extra_operand.md; pbt-out/bug_reports/encode_movk_sp.md; pbt-out/bug_reports/encode_movk_fp_as_gpr.md

```property
function: encoder.data_processing.encode_movk
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain:
    ops: invalid_movk_operand_list
  relation:
    op: throws
    expr: encode_movk(ops)
generators:
  ops: { gen: list, maxLen: 5 }
expected_error: String
evidence: llvm-mc aarch64 rejects SP FP extra and too-few operands ARM ARM Rd 31 is XZR or WZR
```

## encode_movk_diff_lr
- Tier: 2
- Rationale: `lr` is a 64-bit alias of X30 (is_64bit_reg and llvm-mc). Differential vs llvm-mc on `movk lr, #imm [, lsl #N]`. Doc evidence: parse_reg_num "lr" => 30; llvm-mc `movk lr, #1` = `movk x30, #1`.
- Seed: encode_madd_pbt::encode_madd_diff_lr
- Formal: ∀ imm ∈ {0..65535}, hw ∈ {0,1,2,3}. encode_movk([Reg("lr"), Imm(imm)] ++ shift(hw)) = llvm-mc("movk lr, #imm[, lsl #(16*hw)]").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movk
oracle: differential
predicate:
  quantifier: forall
  vars: [imm, hw]
  domain:
    imm: 0..65535
    hw: 0..3
  relation:
    op: eq
    lhs: encode_movk(lr_imm_optional_lsl(imm, hw))
    rhs: llvm_mc(movk_lr_asm(imm, hw))
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: encoder/mod.rs:131-155 parse_reg_num lr=>30, is_64bit_reg lr; llvm-mc lr alias
```

## encode_movk_neg_bad_second
- Tier: 5
- Rationale: Contract-surface sweep. After the first run, the Modifier-None fallthrough (unknown kind or non-constant abs_g symbol) and non-Imm second operands were untested documented error paths (resolve_abs_g_modifier returns None, then get_imm requires Imm). llvm-mc rejects lo12/symbol/mem/reg as the immediate slot.
- Seed: data_processing.rs:238-247 Modifier then get_imm; resolve_abs_g_modifier `_ => Ok(None)`
- Formal: ∀ rd, is_64, second ∈ {Modifier(kind not abs_g), Modifier(abs_g, non-constant symbol), Symbol, Label, Mem, Reg}. encode_movk([Reg(gpr), second]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movk
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, second]
  domain:
    rd: 0..31
    is_64: bool
    second: non_imm16_second_operand
  relation:
    op: throws
    expr: encode_movk(reg_and_second(rd, is_64, second))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: data_processing.rs resolve_abs_g_modifier returns None then get_imm requires Imm llvm-mc rejects non-imm16 second operand
```

# Properties: encode_movn

## encode_movn_diff_imm_shift
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree MOVN decoder). encode_movz/encode_movk rejected by same-job sibling gate (opc 10/11 vs 00; MOVZ zeros other halfwords, MOVK keeps them). Doc evidence: README.md:5-14 gas-compatible assembly; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:222 movn dispatch; ARM ARM Move wide (immediate) MOVN `sf 00 100101 hw imm16 Rd`; llvm-mc `-triple=aarch64`.
- Seed: encode_movk_pbt::encode_movk_diff_imm_shift (same-file Move-wide differential vs llvm-mc); codegen emit.rs:873-902
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ H(is_64). encode_movn([Reg(gpr(is_64,rd)), Imm(imm)] ++ shift(hw)) = llvm-mc("movn Rd, #imm[, lsl #(16*hw)]") where H(false)={0,1}, H(true)={0,1,2,3}, gpr(_,31)=xzr/wzr, and shift(0) may be omitted or `lsl #0`.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, is_64, imm, hw]
  domain:
    rd: 0..31
    is_64: bool
    imm: 0..65535
    hw: valid_hw(is_64)
  relation:
    op: eq
    lhs: encode_movn(reg_imm_optional_lsl(rd, is_64, imm, hw))
    rhs: llvm_mc(movn_asm(rd, is_64, imm, hw))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14 encoder/mod.rs:222 ARM ARM Move wide immediate MOVN
```

## encode_movn_metamorphic_sf
- Tier: 4
- Rationale: ARM ARM sf bit is the sole 32/64-bit distinguisher of otherwise-identical MOVN encodings. Stronger differential covers full-word agreement; this metamorphic isolates sf. Round-trip rejected (no decoder). Doc evidence: ARM ARM `sf 00 100101 hw imm16 Rd`; README.md size-inference `x`/`w` prefix.
- Seed: encode_movk_pbt::encode_movk_metamorphic_sf
- Formal: ∀ rd ∈ {0..31}, imm ∈ {0..65535}, hw ∈ {0,1}. encode_movn(X-ops) XOR encode_movn(W-ops) = 1<<31 at equal rd/imm/hw.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, imm, hw]
  domain:
    rd: 0..31
    imm: 0..65535
    hw: 0..1
  relation:
    op: eq
    lhs: encode_movn(x_ops) XOR encode_movn(w_ops)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Move wide immediate sf at bit 31 README.md size inference
```

## encode_movn_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an exact structural predicate on every success-path word. Differential is stronger for the whole word; this invariant pins each field so a single-bit drift is localizable. Doc evidence: ARM ARM `sf 00 100101 hw imm16 Rd` (opc=00 => bits [30:23]=00100101).
- Seed: encode_movk_pbt::encode_movk_invariant_arm_fields
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ H(is_64). let w = encode_movn(...). (w>>31)&1 = sf(is_64) ∧ (w>>23)&0xFF = 0b00100101 ∧ (w>>21)&3 = hw ∧ (w>>5)&0xFFFF = imm ∧ w&0x1F = rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, is_64, imm, hw]
  domain:
    rd: 0..31
    is_64: bool
    imm: 0..65535
    hw: valid_hw(is_64)
  relation:
    op: holds
    expr: arm_movn_fields(encode_movn(ops), rd, is_64, imm, hw)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: ARM ARM Move wide immediate MOVN sf 00 100101 hw imm16 Rd
```

## encode_movn_diff_lr
- Tier: 2
- Rationale: `lr` is a documented 64-bit alias of X30 (parse_reg_num and is_64bit_reg). Differential vs llvm-mc on that alias. Same-job sibling encode_movk has the same alias contract. Stronger state-machine/round-trip rejected as for the main differential.
- Seed: encode_movk_pbt::encode_movk_diff_lr
- Formal: ∀ imm ∈ {0..65535}, hw ∈ {0,1,2,3}. encode_movn([Reg("lr"), Imm(imm)] ++ shift(hw)) = llvm-mc("movn lr, #imm[, lsl #(16*hw)]").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movn
oracle: differential
predicate:
  quantifier: forall
  vars: [imm, hw]
  domain:
    imm: 0..65535
    hw: 0..3
  relation:
    op: eq
    lhs: encode_movn(lr_imm_optional_lsl(imm, hw))
    rhs: llvm_mc(movn_lr_asm(imm, hw))
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: encoder/mod.rs:131-155 parse_reg_num lr=>30, is_64bit_reg lr; llvm-mc lr alias
```

## encode_movn_neg_imm_oob
- Tier: 5
- Rationale: llvm-mc and ARM ARM require imm16 in [0, 65535]. encode_movn masks with `& 0xFFFF` and does not reject. Documented error contract from llvm-mc: "immediate must be an integer in range [0, 65535]". Stronger oracles do not apply to the invalid domain.
- Seed: encode_movk_pbt::encode_movk_neg_imm_oob
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∉ {0..65535}. encode_movn([Reg(gpr), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, is_64=false, imm=-1 (movn w0, #-1)
- Bug report: pbt-out/bug_reports/encode_movn_imm_oob.md

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, imm]
  domain:
    rd: 0..31
    is_64: bool
    imm: i64 \\ {0..65535}
  relation:
    op: throws
    expr: encode_movn([Reg(gpr(is_64, rd)), Imm(imm)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: llvm-mc aarch64 "immediate must be an integer in range [0, 65535]"; ARM ARM imm16
```

## encode_movn_neg_invalid_shift
- Tier: 5
- Rationale: llvm-mc requires optional lsl with 0/16 (W) or 0/16/32/48 (X). Non-lsl kinds and other amounts are rejected. encode_movn integer-divides lsl amount by 16 and defaults non-lsl to hw=0. Documented error: "expected 'lsl' with optional integer 0, 16, 32 or 48".
- Seed: encode_movk_pbt::encode_movk_neg_invalid_shift
- Formal: ∀ rd, imm ∈ {0..65535}, (is_64, kind, amount) ∈ InvalidShift. encode_movn([Reg, Imm, Shift{kind,amount}]) is Err. InvalidShift = non-lsl kind, or lsl amount not in {0,16} (W) / {0,16,32,48} (X).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, imm=0, is_64=false, kind="lsr", amount=0 (movn w0, #0, lsr #0)
- Bug report: pbt-out/bug_reports/encode_movn_invalid_shift.md

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm, is_64, kind, amount]
  domain:
    rd: 0..31
    imm: 0..65535
    is_64: bool
    kind: shift_kind
    amount: invalid_lsl_or_any_non_lsl
  relation:
    op: throws
    expr: encode_movn([Reg(gpr), Imm(imm), Shift{kind, amount}])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
expected_error: String
evidence: llvm-mc aarch64 expected lsl with 0/16 (W) or 0/16/32/48 (X)
```

## encode_movn_neg_extra_operand
- Tier: 5
- Rationale: llvm-mc rejects a fourth operand after optional lsl ("invalid operand for instruction"). MOVN encoding has exactly Rd + imm16 + optional lsl. Extra operands must Err. encode_movn ignores operands beyond index 2.
- Seed: encode_movk_pbt::encode_movk_neg_extra_operand
- Formal: ∀ rd, is_64, imm ∈ {0..65535}, hw ∈ H(is_64), extra. encode_movn(valid_ops ++ [extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, is_64=true, hw=0, imm=0, extra=Reg("x0") (movn x0, #0, x0)
- Bug report: pbt-out/bug_reports/encode_movn_extra_operand.md

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, imm, hw, extra]
  domain:
    rd: 0..31
    is_64: bool
    imm: 0..65535
    hw: valid_hw(is_64)
    extra: Operand
  relation:
    op: throws
    expr: encode_movn(valid_ops ++ [extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc aarch64 rejects extra operand after optional lsl; ARM ARM exactly Rd+imm16+optional lsl
```

## encode_movn_neg_sp
- Tier: 5
- Rationale: ARM ARM register 31 in Move-wide is XZR/WZR, never SP/WSP. llvm-mc rejects `movn sp, #0` / `movn wsp, #0`. parse_reg_num maps sp/wsp to 31, so encode_movn currently encodes ZR.
- Seed: encode_movk_pbt::encode_movk_neg_sp
- Formal: ∀ is_64, imm ∈ {0..65535}, hw ∈ {0,1}. encode_movn([Reg(sp|wsp), Imm(imm)] ++ shift(hw)) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: is_64=false, imm=0, hw=0 (movn wsp, #0)
- Bug report: pbt-out/bug_reports/encode_movn_sp.md

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, imm, hw]
  domain:
    is_64: bool
    imm: 0..65535
    hw: 0..1
  relation:
    op: throws
    expr: encode_movn(sp_imm_optional_lsl(is_64, imm, hw))
generators:
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: ARM ARM Move wide Rd=31 is XZR/WZR never SP; llvm-mc rejects movn sp/wsp
```

## encode_movn_neg_too_few
- Tier: 5
- Rationale: Contract-surface sweep. get_reg(0)/get_imm(1) fail when fewer than 2 operands. llvm-mc: "too few operands for instruction".
- Seed: encode_movk_pbt::encode_movk_neg_too_few
- Formal: ∀ n ∈ {0,1}, ops prefix of a valid pair of length n. encode_movn(ops) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, is_64, imm]
  domain:
    n: 0..1
    rd: 0..31
    is_64: bool
    imm: 0..65535
  relation:
    op: throws
    expr: encode_movn(ops[..n])
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
expected_error: String
evidence: llvm-mc aarch64 too few operands; get_reg/get_imm require operand 0 and 1
```

## encode_movn_neg_fp
- Tier: 5
- Rationale: Contract-surface sweep. llvm-mc rejects FP/SIMD names as MOVN Rd. parse_reg_num accepts d/s/q/v/h/b prefixes so encode_movn currently encodes them as GPRs.
- Seed: encode_movk_pbt::encode_movk_neg_fp
- Formal: ∀ fp ∈ {d,s,q,v,h,b}{0..31}, imm ∈ {0..65535}. encode_movn([Reg(fp), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: fp="d0", imm=0 (movn d0, #0)
- Bug report: pbt-out/bug_reports/encode_movn_fp_as_gpr.md

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp, imm]
  domain:
    fp: fp_simd_name
    imm: 0..65535
  relation:
    op: throws
    expr: encode_movn([Reg(fp), Imm(imm)])
generators:
  fp: { gen: string }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
expected_error: String
evidence: llvm-mc aarch64 rejects movn d0, #0; ARM ARM Rd is GPR
```

## encode_movn_neg_invalid_name
- Tier: 5
- Rationale: Contract-surface sweep. parse_reg_num returns None for foo/x32/w32/x/r0/empty, so get_reg Errs. llvm-mc rejects those names.
- Seed: encode_movk_pbt::encode_movk_neg_invalid_name
- Formal: ∀ name ∈ {foo, x32, w32, x, r0, ""}, imm ∈ {0..65535}. encode_movn([Reg(name), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, imm]
  domain:
    name: invalid_gpr_name
    imm: 0..65535
  relation:
    op: throws
    expr: encode_movn([Reg(name), Imm(imm)])
generators:
  name: { gen: string }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
expected_error: String
evidence: parse_reg_num None for non-GPR names; llvm-mc rejects them
```

## encode_movn_neg_bad_second
- Tier: 5
- Rationale: Contract-surface sweep. get_imm requires Operand::Imm; Modifier/Symbol/Label/Mem/Reg at slot 1 must Err. (abs_g is documented for movz/movk only, data_processing.rs:179-181.) llvm-mc rejects non-imm16 second operands (except reloc abs_g, which this encoder does not implement for movn).
- Seed: encode_movk_pbt::encode_movk_neg_bad_second
- Formal: ∀ rd, is_64, second ∈ {Modifier(lo12), Modifier(abs_g0, non-constant), Symbol, Label, Mem, Reg}. encode_movn([Reg(gpr), second]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, second]
  domain:
    rd: 0..31
    is_64: bool
    second: non_imm16_second_operand
  relation:
    op: throws
    expr: encode_movn(reg_and_second(rd, is_64, second))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
expected_error: String
evidence: get_imm requires Imm; data_processing.rs:179-181 abs_g is for movz/movk; llvm-mc rejects non-imm16 second
```
