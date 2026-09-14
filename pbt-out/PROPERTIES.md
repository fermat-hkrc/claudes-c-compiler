# Properties: encode_eon

## encode_eon_diff_reg_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc. State machine rejected — encode_eon is a pure function with no lifecycle. Algebraic round-trip rejected — no in-tree EON decoder. Same-job sibling gate: encode_orn/encode_bics/encode_logical(EOR) implement different opcodes, not EON. README claims gas-compatible AArch64 text; llvm-mc is an independent assembler of that contract. Shift amount bounds [0,31] (W) and [0,63] (X) are sampled at 0, 1, max-1, max.
- Seed: encode_bics_pbt::encode_bics_diff_reg_llvm_mc at data_processing.rs:3274
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, use_shift ∈ Bool, amt ∈ [0, 31] if ¬is_64 else [0, 63]. let names = gpr(is_64, ·) using xzr/wzr for 31. encode_eon([Reg(rd),Reg(rn),Reg(rm)] ++ optional Shift(kind,amt)) = Word(w) ∧ w = llvm-mc("eon rd, rn, rm{, kind #amt}")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_eon
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, use_shift, amt]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31", amt: "0..=63 filtered by width" }
  relation:
    op: eq
    lhs: encode_eon([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(gpr(is_64,rm))] ++ opt_shift)
    rhs: llvm_mc_word("eon", gpr(is_64,rd), gpr(is_64,rn), gpr(is_64,rm), opt_shift)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  use_shift: { gen: bool }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:236 eon dispatch; ARM ARM EON (shifted register)
```

## encode_eon_diff_imm_llvm_mc
- Tier: 2
- Rationale: GNU as / llvm-mc treat `eon Rd, Rn, #imm` as the assembler alias of `eor Rd, Rn, #~imm` (ARM ARM logical-immediate). Same differential reference as the register form. encode_bic in this file already implements the analogous BIC #imm = AND #~imm alias, evidencing that inverted-immediate logical aliases are in-scope for this encoder. Stronger state machine / round-trip rejected as for the register form.
- Seed: encode_bics_pbt::encode_bics_diff_imm_llvm_mc at data_processing.rs:3314
- Formal: ∀ rd,rn ∈ {0..30}×{0..31}, is_64 ∈ Bool, m a valid AArch64 bitmask immediate of width. let imm = ~m (width-truncated). encode_eon([Reg(rd),Reg(rn),Imm(imm)]) = Word(w) ∧ w = llvm-mc("eon rd, rn, #imm")
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: eon w0, w0, #0xaaaaaaaa (rd=0, rn=0, is_64=false, seed=0) — SUT Err("expected register at operand 2, got Some(Imm(2863311530))") vs llvm-mc 0x5200f000
- Bug report: pbt-out/bug_reports/encode_eon_imm_alias.md

```property
function: encoder.data_processing.encode_eon
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, is_64, imm]
  domain: { rd: "0..=31", rn: "0..=31", imm: "bitwise-NOT of a valid bitmask immediate" }
  relation:
    op: eq
    lhs: encode_eon([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Imm(imm)])
    rhs: llvm_mc_word("eon rd, rn, #imm")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  seed: { gen: int, min: 0, max: 9999, type: u32 }
evidence: llvm-mc eon x0,x1,#1 -> eor x0,x1,#~1; ARM ARM EON (immediate) alias of EOR (immediate); encode_bic immediate path data_processing.rs:1034-1046
```

## encode_eon_metamorphic_n_bit_vs_eor
- Tier: 4
- Rationale: ARM ARM Logical (shifted register) documents EOR opc=10 N=0 vs EON opc=10 N=1, i.e. they differ only by N at bit 21. Stronger differential already covers EON independently; this metamorphic checks the documented sibling transform without copying the SUT body. encode_logical(opc=0b10) is EOR (different job — same-job sibling gate rejects it as a differential reference).
- Seed: encode_bics_pbt::encode_bics_meta_opc_vs_bic at data_processing.rs:3346
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ valid range. encode_eon(ops) XOR encode_logical(ops, 0b10) = 1<<21
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_eon
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { rd: "0..=31", rn: "0..=31", rm: "0..=31" }
  relation:
    op: eq
    lhs: encode_eon(ops) XOR encode_logical(ops, 0b10)
    rhs: 1u32 << 21
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: data_processing.rs:975 comment "opc=10, N=1"; ARM ARM Logical (shifted register) EOR N=0 / EON N=1
```

## encode_eon_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout for Logical (shifted register) EON is an exact structural predicate on the output word. Weaker than differential (does not pin the full 32-bit value against an independent assembler) but independently evidenced.
- Seed: encode_bics_pbt::encode_bics_word_layout at data_processing.rs:3391
- Formal: ∀ rd,rn,rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ valid range. let w = encode_eon([Reg,Reg,Reg,Shift]). w[4:0]=rd ∧ w[9:5]=rn ∧ w[20:16]=rm ∧ w[31]=sf(is_64) ∧ w[30:29]=0b10 ∧ w[28:24]=0b01010 ∧ w[23:22]=shift(kind) ∧ w[21]=1 ∧ w[15:10]=amt
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_eon
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { rd: "0..=31" }
  body: fields of encode_eon match ARM ARM Logical (shifted register) EON layout
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Logical (shifted register) EON sf opc=10 01010 shift N=1 Rm imm6 Rn Rd; data_processing.rs:975
```

## encode_eon_neg_arity
- Tier: 5
- Rationale: Body and llvm-mc both require 3 operands (`eon requires 3 operands`). Documented error contract for too-few operands. Stronger oracles do not apply to the invalid-arity domain.
- Seed: encode_bics_pbt::encode_bics_neg_arity at data_processing.rs:3430
- Formal: ∀ n ∈ {0,1,2}, ops a length-n register list. encode_eon(ops) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_eon
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, is_64, r]
  domain: { n: "0..=2" }
  relation:
    op: throws
    expr: encode_eon(ops_of_len_n)
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  is_64: { gen: bool }
  r: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: data_processing.rs:953-955 "eon requires 3 operands"; llvm-mc rejects `eon x0, x1`
```

## encode_eon_neg_extra_operand
- Tier: 5
- Rationale: llvm-mc rejects a 4th non-shift operand (`eon x0, x1, x2, x3`). Gas-compatible assembler must reject it. Stronger oracles do not apply to this invalid domain.
- Seed: encode_bics_pbt::encode_bics_neg_extra_operand at data_processing.rs:3579
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64 ∈ Bool, extra ∈ {Reg, Imm, Mem, Symbol}. encode_eon([Reg,Reg,Reg, extra]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: eon w0, w0, w0, w0 (is_64=false, rd=rn=rm=0, which=0 extra=Reg)
- Bug report: pbt-out/bug_reports/encode_eon_extra_operand.md

```property
function: encoder.data_processing.encode_eon
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, extra]
  domain: { extra: "Reg|Imm|Mem|Symbol" }
  relation:
    op: throws
    expr: encode_eon([Reg, Reg, Reg, extra])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  which: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc rejects `eon x0, x1, x2, x3`; README.md:5-14 gas-compatible
```

## encode_eon_neg_mixed_width
- Tier: 5
- Rationale: ARM ARM EON requires all three registers to be Wt or all Xt. llvm-mc rejects mixed x/w. Gas-compatible assembler must reject it.
- Seed: encode_bics_pbt::encode_bics_neg_mixed_width at data_processing.rs:3444
- Formal: ∀ rd,rn,rm ∈ {0..30}, rd64,rn64,rm64 ∈ Bool. ¬(rd64=rn64=rm64) ⇒ encode_eon([Reg(gpr(rd64,rd)), …]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: eon w0, w0, x0 (rd=rn=rm=0, rd64=false, rn64=false, rm64=true)
- Bug report: pbt-out/bug_reports/encode_eon_mixed_width.md

```property
function: encoder.data_processing.encode_eon
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, rd64, rn64, rm64]
  domain: { rd: "0..=30", not_all_same_width: true }
  relation:
    op: throws
    expr: encode_eon([Reg(gpr(rd64,rd)), Reg(gpr(rn64,rn)), Reg(gpr(rm64,rm))])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  rd64: { gen: bool }
  rn64: { gen: bool }
  rm64: { gen: bool }
expected_error: String
evidence: ARM ARM EON Wt,Wt,Wm / Xt,Xt,Xm; llvm-mc rejects `eon x0, w1, x2`
```

## encode_eon_neg_sp_fp
- Tier: 5
- Rationale: ARM ARM EON uses XZR/WZR for register 31, never SP/WSP; Wt/Xt only (no FP/SIMD). llvm-mc rejects `eon sp, ...` and `eon d0, ...`. Gas-compatible assembler must reject them.
- Seed: encode_bics_pbt::encode_bics_neg_sp_fp at data_processing.rs:3471
- Formal: ∀ which ∈ {0,1,2}, bad ∈ {sp,wsp,dN,sN,qN,vN,hN,bN}. encode_eon with bad at operand `which` and valid GPRs elsewhere = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: eon wsp, w0, w0 (which=0, is_64=false, kind=0); also eon d0, x1, x2
- Bug report: pbt-out/bug_reports/encode_eon_sp_register_form.md ; pbt-out/bug_reports/encode_eon_fp_reg.md

```property
function: encoder.data_processing.encode_eon
oracle: negative_error
predicate:
  quantifier: forall
  vars: [which, is_64, a, b, kind, fp_n]
  domain: { which: "0..=2", kind: "sp|wsp|d|s|q|v|h|b" }
  relation:
    op: throws
    expr: encode_eon(ops_with_bad_at_which)
generators:
  which: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
  a: { gen: int, min: 0, max: 30, type: u32 }
  b: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 8, type: u32 }
  fp_n: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM EON Xt/Wt only, R31=XZR/WZR; llvm-mc rejects `eon sp, x0, x1` and `eon d0, d1, d2`
```

## encode_eon_neg_shift_range
- Tier: 5
- Rationale: ARM ARM EON shift amount is [0,31] for 32-bit and [0,63] for 64-bit. llvm-mc rejects lsl #32 (W) and lsl #64 (X). Documented bound must be exercised at bound+1. Stronger oracles do not apply to this invalid domain.
- Seed: encode_bics_pbt::encode_bics_neg_shift_range at data_processing.rs:3514
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}. let amt = 32|33|63 if ¬is_64 else 64|65|128. encode_eon([Reg,Reg,Reg,Shift(kind,amt)]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: eon w0, w0, w0, lsl #32 (rd=rn=rm=0, is_64=false, kind="lsl", amt_w=32)
- Bug report: pbt-out/bug_reports/encode_eon_shift_out_of_range.md

```property
function: encoder.data_processing.encode_eon
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, kind, amt]
  domain: { amt: "32|33|63 if 32-bit else 64|65|128" }
  relation:
    op: throws
    expr: encode_eon([Reg, Reg, Reg, Shift(kind, amt)])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, items: ["lsl", "lsr", "asr", "ror"] }
  amt_w: { gen: oneof, items: [32, 33, 63] }
  amt_x: { gen: oneof, items: [64, 65, 128] }
expected_error: String
evidence: ARM ARM EON amount [0,31]/[0,63]; llvm-mc rejects `eon w0, w1, w2, lsl #32` and `eon x0, x1, x2, lsl #64`
```

## encode_eon_neg_unknown_shift
- Tier: 5
- Rationale: Coverage-sweep (round 1). ARM ARM EON shift is {LSL,LSR,ASR,ROR} only; llvm-mc rejects unknown kinds. The `_ => 0b00` arm in encode_eon was not executed by the first batch. Stronger oracles do not apply to this invalid domain.
- Seed: encode_bics_pbt::encode_bics_neg_unknown_shift at data_processing.rs:3546
- Formal: ∀ rd,rn,rm ∈ {0..30}, is_64 ∈ Bool, unknown ∉ {lsl,lsr,asr,ror}, amt ∈ {0,1,31}. encode_eon([Reg,Reg,Reg,Shift(unknown,amt)]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: eon w0, w0, w0, lslx #0 (rd=rn=rm=0, is_64=false, unknown="lslx", amt_ok=0)
- Bug report: pbt-out/bug_reports/encode_eon_unknown_shift_kind.md

```property
function: encoder.data_processing.encode_eon
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, is_64, unknown, amt]
  domain: { unknown: "lslx|rrx|rol|empty|asr " }
  relation:
    op: throws
    expr: encode_eon([Reg, Reg, Reg, Shift(unknown, amt)])
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  is_64: { gen: bool }
  unknown: { gen: oneof, items: ["lslx", "rrx", "rol", "", "asr "] }
  amt: { gen: oneof, items: [0, 1, 31] }
expected_error: String
evidence: ARM ARM EON shift in {LSL,LSR,ASR,ROR}; llvm-mc rejects `eon w0, w0, w0, lslx #0`
```

## encode_eon_neg_invalid_name
- Tier: 5
- Rationale: Coverage-sweep (round 1). parse_reg_num None path (x32, w32, empty, foo, r0, x, x-1, x99) was not forced by the first batch. get_reg must Err on an invalid register name.
- Seed: encode_bics_pbt::encode_bics_neg_invalid_rm at data_processing.rs:3570
- Formal: ∀ is_64 ∈ Bool, rd,rn ∈ {0..30}, bad ∈ {x32,w32,x99,"",foo,r0,x}. encode_eon([Reg(rd),Reg(rn),Reg(bad)]) = Err
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.data_processing.encode_eon
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_64, rd, rn, bad]
  domain: { bad: "x32|w32|x99|empty|foo|r0|x" }
  relation:
    op: throws
    expr: encode_eon([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rn)), Reg(bad)])
generators:
  is_64: { gen: bool }
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  bad: { gen: oneof, items: ["x32", "w32", "x99", "", "foo", "r0", "x"] }
expected_error: String
evidence: encoder/mod.rs:131-148 parse_reg_num returns None outside x/w 0..31 and aliases
```
