# Properties: encode_movz

## encode_movz_diff_imm_shift
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree MOVZ decoder). encode_movk/encode_movn rejected by same-job sibling gate (opc 11/00 vs 10). Doc evidence: README.md:5-14 gas-compatible assembly; encoder/mod.rs:1-7 32-bit words; ARM ARM Move wide (immediate) MOVZ `sf 10 100101 hw imm16 Rd`; llvm-mc `-triple=aarch64`.
- Seed: encode_movk_pbt::encode_movk_diff_imm_shift
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ H(is_64). encode_movz([Reg(gpr(is_64,rd)), Imm(imm)] ++ shift(hw)) = llvm-mc("movz Rd, #imm[, lsl #(16*hw)]") where H(false)={0,1}, H(true)={0,1,2,3}, gpr(_,31)=xzr/wzr, and shift(0) may be omitted or `lsl #0`.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
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
    lhs: encode_movz(reg_imm_optional_lsl(rd, is_64, imm, hw))
    rhs: llvm_mc(movz_asm(rd, is_64, imm, hw))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14 encoder/mod.rs:220 ARM ARM Move wide immediate MOVZ
```

## encode_movz_metamorphic_sf
- Tier: 4
- Rationale: ARM ARM sf bit is the sole 32/64-bit distinguisher of otherwise-identical MOVZ encodings. Stronger differential covers full-word agreement; this metamorphic isolates sf. Round-trip rejected (no decoder). Doc evidence: ARM ARM `sf 10 100101 hw imm16 Rd`; README.md size-inference `x`/`w` prefix.
- Seed: encode_movk_pbt::encode_movk_metamorphic_sf
- Formal: ∀ rd ∈ {0..31}, imm ∈ {0..65535}, hw ∈ {0,1}. encode_movz(X-ops) XOR encode_movz(W-ops) = 1<<31 at equal rd/imm/hw.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
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
    lhs: encode_movz(x_ops) XOR encode_movz(w_ops)
    rhs: 1 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Move wide immediate sf at bit 31 README.md size inference
```

## encode_movz_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an exact structural predicate on every success-path word. Differential is stronger for the whole word; this invariant pins each field so a single-bit drift is localizable. Doc evidence: ARM ARM `sf 10 100101 hw imm16 Rd`.
- Seed: encode_movk_pbt::encode_movk_invariant_arm_fields
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ H(is_64). let w = encode_movz(...). (w>>31)&1 = sf(is_64) ∧ (w>>23)&0xFF = 0b10100101 ∧ (w>>21)&3 = hw ∧ (w>>5)&0xFFFF = imm ∧ w&0x1F = rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, is_64, imm, hw]
  domain:
    rd: 0..31
    is_64: bool
    imm: 0..65535
    hw: valid_hw(is_64)
  body: let w = encode_movz(...); (w>>31)&1 == sf(is_64) && (w>>23)&0xFF == 0b10100101 && (w>>21)&3 == hw && (w>>5)&0xFFFF == imm && (w&0x1F) == rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: ARM ARM Move wide immediate MOVZ sf 10 100101 hw imm16 Rd
```

## encode_movz_diff_abs_g
- Tier: 2
- Rationale: resolve_abs_g_modifier is documented for movz/movk (data_processing.rs:179-181). Constant `:abs_gN:` extracts the 16-bit chunk at shift N*16; the resolved encoding must match llvm-mc of `movz Rd, #chunk [, lsl #shift]`. Reloc form of llvm-mc on the modifier text is a different job (fixup vs resolved constant). State machine rejected. Round-trip rejected.
- Seed: encode_movk_pbt::encode_movk_diff_abs_g
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, val ∈ i64, kind ∈ G(is_64). encode_movz([Reg(gpr), Modifier{kind, symbol=val}]) = llvm-mc(movz Rd, #((val>>shift)&0xFFFF) [, lsl #shift]) where G(true)={abs_g0,abs_g0_nc,abs_g1,abs_g1_nc,abs_g2,abs_g2_nc,abs_g3}, G(false) drops g2/g3, shift=kind's halfword shift.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, is_64, val, kind]
  domain:
    rd: 0..31
    is_64: bool
    val: i64
    kind: abs_g_kind(is_64)
  body: encode_movz(abs_g_ops(rd, is_64, kind, val)) == llvm_mc(movz_resolved_asm(rd, is_64, val, kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  val: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
  kind: { gen: string }
evidence: data_processing.rs:179-181 abs_g modifiers for movz/movk
```

## encode_movz_diff_lr
- Tier: 2
- Rationale: `lr` is a documented 64-bit alias of X30 (parse_reg_num maps lr -> 30; is_64bit_reg treats lr as 64-bit; llvm-mc accepts `movz lr, #imm`). Same differential contract as X-form MOVZ. encode_movk/encode_movn same-job rejected.
- Seed: encode_movk_pbt::encode_movk_diff_lr
- Formal: ∀ imm ∈ {0..65535}, hw ∈ {0,1,2,3}. encode_movz([Reg("lr"), Imm(imm)] ++ shift(hw)) = llvm-mc("movz lr, #imm[, lsl #(16*hw)]").
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
oracle: differential
predicate:
  quantifier: forall
  vars: [imm, hw]
  domain:
    imm: 0..65535
    hw: 0..3
  relation:
    op: eq
    lhs: encode_movz(ops_imm("lr", imm, hw))
    rhs: llvm_mc(movz_asm("lr", imm, hw))
generators:
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
evidence: encoder/mod.rs:136 lr => 30; encoder/mod.rs:153 lr is 64-bit; llvm-mc movz lr
```

## encode_movz_neg_imm_oob
- Tier: 5
- Rationale: llvm-mc and ARM ARM require imm16 in [0, 65535]. SUT must Err outside that range. Differential cannot run on rejected input. Stronger oracles rejected for the invalid domain. Doc evidence: llvm-mc "immediate must be an integer in range [0, 65535]".
- Seed: encode_movk_pbt::encode_movk_neg_imm_oob
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∉ {0..65535}. encode_movz([Reg(gpr(is_64,rd)), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, is_64=false, imm=-1 (movz w0, #-1)
- Bug report: pbt-out/bug_reports/encode_movz_imm_oob.md

```property
function: encoder.data_processing.encode_movz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, imm]
  domain:
    rd: 0..31
    is_64: bool
    imm: i64 \ {0..65535}
  relation:
    op: throws
    expr: encode_movz(reg_imm(rd, is_64, imm))
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
evidence: llvm-mc immediate must be an integer in range [0, 65535]; ARM ARM imm16
```

## encode_movz_neg_invalid_shift
- Tier: 5
- Rationale: llvm-mc requires optional lsl with integer 0/16 (W) or 0/16/32/48 (X). Non-lsl kinds and other amounts must Err. Doc evidence: llvm-mc "expected 'lsl' with optional integer 0, 16, 32 or 48".
- Seed: encode_movk_pbt::encode_movk_neg_invalid_shift
- Formal: ∀ rd ∈ {0..31}, imm ∈ {0..65535}, (is_64, kind, amount) ∈ invalid_shift. encode_movz([Reg(gpr), Imm(imm), Shift{kind,amount}]) is Err. invalid_shift = non-lsl kinds, or lsl amount not in {0,16} (W) / {0,16,32,48} (X).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, imm=0, is_64=false, kind="lsr", amount=0 (movz w0, #0, lsr #0)
- Bug report: pbt-out/bug_reports/encode_movz_invalid_shift.md

```property
function: encoder.data_processing.encode_movz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, imm, is_64, kind, amount]
  domain:
    rd: 0..31
    imm: 0..65535
    is_64: bool
    kind: invalid_or_non_canonical_shift
    amount: 0..64
  relation:
    op: throws
    expr: encode_movz(reg_imm_shift(rd, is_64, imm, kind, amount))
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  is_64: { gen: bool }
  kind: { gen: string }
  amount: { gen: int, min: 0, max: 64, type: u32 }
evidence: llvm-mc expected lsl with optional integer 0, 16, 32 or 48
```

## encode_movz_neg_extra_operand
- Tier: 5
- Rationale: ARM ARM and llvm-mc accept exactly Rd + imm16 + optional lsl. An extra operand after that is invalid. Doc evidence: llvm-mc "invalid operand for instruction" on `movz x0, #0, lsl #16, lsl #32` and `movz x0, #0, x1`.
- Seed: encode_movk_pbt::encode_movk_neg_extra_operand
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ H(is_64), extra ∈ Operand. encode_movz(ops_imm(gpr, imm, hw) ++ [extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, is_64=true, hw=0, imm=0, extra=Reg("x0") (movz x0, #0, x0)
- Bug report: pbt-out/bug_reports/encode_movz_extra_operand.md

```property
function: encoder.data_processing.encode_movz
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
    expr: encode_movz(ops_imm_plus_extra(rd, is_64, imm, hw, extra))
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 3, type: u32 }
  extra: { gen: string }
evidence: llvm-mc extra operand after optional lsl is invalid; ARM ARM exactly Rd+imm16+optional lsl
```

## encode_movz_neg_sp
- Tier: 5
- Rationale: ARM ARM register 31 in MOVZ is XZR/WZR, never SP/WSP. llvm-mc rejects `movz sp, #0` and `movz wsp, #0`. Doc evidence: llvm-mc "invalid operand for instruction"; ARM ARM Move wide (immediate) Rd.
- Seed: encode_movk_pbt::encode_movk_neg_sp
- Formal: ∀ is_64 ∈ {false,true}, imm ∈ {0..65535}, hw ∈ {0,1}. encode_movz(ops_imm(sp_name(is_64), imm, hw)) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: is_64=false, imm=0, hw=0 (movz wsp, #0)
- Bug report: pbt-out/bug_reports/encode_movz_sp.md

```property
function: encoder.data_processing.encode_movz
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
    expr: encode_movz(ops_imm(sp_or_wsp(is_64), imm, hw))
expected_error: String
generators:
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
  hw: { gen: int, min: 0, max: 1, type: u32 }
evidence: llvm-mc invalid operand for movz sp/wsp; ARM ARM Rd=31 is XZR/WZR
```

## encode_movz_neg_too_few
- Tier: 5
- Rationale: llvm-mc rejects `movz x0` as too few operands. get_reg/get_imm return Err when the slot is missing. Doc evidence: llvm-mc "too few operands for instruction".
- Seed: encode_movk_pbt::encode_movk_neg_too_few
- Formal: ∀ n ∈ {0,1}, rd ∈ {0..31}, is_64 ∈ {false,true}, imm ∈ {0..65535}. encode_movz([Reg, Imm][..n]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
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
    expr: encode_movz(ops_prefix(n, rd, is_64, imm))
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: llvm-mc too few operands for instruction
```

## encode_movz_neg_fp
- Tier: 5
- Rationale: llvm-mc rejects FP/SIMD names as MOVZ Rd. ARM ARM Move wide operates on GPRs. Doc evidence: llvm-mc "invalid operand" on `movz d0, #0`.
- Seed: encode_movk_pbt::encode_movk_neg_fp
- Formal: ∀ fp ∈ {d,s,q,v,h,b}{0..31}, imm ∈ {0..65535}. encode_movz([Reg(fp), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: fp="d0", imm=0 (movz d0, #0)
- Bug report: pbt-out/bug_reports/encode_movz_fp.md

```property
function: encoder.data_processing.encode_movz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [fp, imm]
  domain:
    fp: fp_simd_name
    imm: 0..65535
  relation:
    op: throws
    expr: encode_movz([Reg(fp), Imm(imm)])
expected_error: String
generators:
  fp: { gen: string }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: llvm-mc invalid operand for movz d0; ARM ARM GPR-only Rd
```

## encode_movz_neg_invalid_name
- Tier: 5
- Rationale: Invalid register names (foo, x32, w32, x, r0, empty) must Err. llvm-mc rejects them. Doc evidence: llvm-mc "invalid operand" on `movz foo, #0` and `movz x32, #0`.
- Seed: encode_movk_pbt::encode_movk_neg_invalid_name
- Formal: ∀ name ∈ {foo, x32, w32, x, r0, ""}, imm ∈ {0..65535}. encode_movz([Reg(name), Imm(imm)]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [name, imm]
  domain:
    name: invalid_gpr_name
    imm: 0..65535
  relation:
    op: throws
    expr: encode_movz([Reg(name), Imm(imm)])
expected_error: String
generators:
  name: { gen: string }
  imm: { gen: int, min: 0, max: 65535, type: i64 }
evidence: llvm-mc invalid operand for movz foo / movz x32
```

## encode_movz_neg_bad_second
- Tier: 5
- Rationale: Second operand must be imm16 or a resolvable abs_g modifier. Unknown modifiers, unresolved abs_g symbols, Symbol/Label/Mem/Reg must Err. Doc evidence: get_imm expects Imm; resolve_abs_g_modifier returns None for unknown kinds / non-constant symbols, then get_imm fails.
- Seed: encode_movk_pbt::encode_movk_neg_bad_second
- Formal: ∀ rd ∈ {0..31}, is_64 ∈ {false,true}, second ∈ {Modifier(lo12), Modifier(abs_g0,foo), Symbol, Label, Mem, Reg}. encode_movz([Reg(gpr), second]) is Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_movz
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, is_64, second]
  domain:
    rd: 0..31
    is_64: bool
    second: non_imm16_non_resolvable_abs_g
  relation:
    op: throws
    expr: encode_movz([Reg(gpr(is_64, rd)), second])
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  second: { gen: string }
evidence: data_processing.rs:201-214 get_imm after unresolved modifier; llvm-mc rejects non-imm16 second operand
```
