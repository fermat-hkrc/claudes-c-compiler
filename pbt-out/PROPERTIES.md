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
