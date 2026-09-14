# Properties: encode_ldrsw

## encode_ldrsw_diff_unsigned_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree LDRSW decoder). Same-job siblings encode_ldr_str / encode_ldrs / encode_ldur_stur rejected (LDR/STR unsigned, LDRSB/LDRSH, generic unscaled; different jobs). Doc evidence: assembler README.md:11 "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:333 dispatch of ldrsw; ARM ARM LDRSW (immediate) unsigned offset; README.md:221 Loads/Stores table.
- Seed: load_store.rs encode_ldur_stur_pbt encode_ldur_stur_diff_llvm_mc
- Formal: ∀ rt, rn ∈ {0..31}, imm12 ∈ {0..4095}. Let offset = imm12 * 4. Then encode_ldrsw([Reg(Xt), Mem{Xn|SP, offset}]) = llvm-mc("ldrsw Xt, [Xn|SP{, #offset}]") as LE u32, where X31 dest is xzr, Rn=31 is sp.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, imm12]
  domain: { rt: "0..=31", rn: "0..=31", imm12: "0..=4095" }
  relation:
    op: eq
    lhs: encode_ldrsw(ops_unsigned(rt, rn, imm12 * 4))
    rhs: llvm_mc_word(asm_unsigned)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:333; ARM ARM LDRSW unsigned offset size=10 111 0 01 10 imm12 Rn Rt
```

## encode_ldrsw_diff_unscaled_pre_post_llvm_mc
- Tier: 2
- Rationale: llvm-mc is the independent assembler reference for unscaled (offset in [-256,255]), pre-index, and post-index forms. Stronger oracles rejected as in encode_ldrsw_diff_unsigned_llvm_mc. Documented simm9 bounds -256 and 255 sampled exactly. Writeback domain excludes Rt==Rn unless Rn is SP (llvm-mc: unpredictable).
- Seed: load_store.rs encode_ldur_stur_pbt encode_ldur_stur_diff_llvm_mc
- Formal: ∀ rt, rn ∈ {0..31}, simm ∈ [-256,255], form ∈ {unscaled, pre, post}. If form ∈ {pre, post} ⇒ (rt ≠ rn ∨ rn = 31). Then encode_ldrsw([Reg(Xt), Mem*|MemPre|MemPost {Xn|SP, simm}]) = llvm-mc of the matching LDRSW/LDURSW syntax as LE u32.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, simm, form]
  domain: { rt: "0..=31", rn: "0..=31", simm: "-256..=255", form: "unscaled|pre|post" }
  relation:
    op: eq
    lhs: encode_ldrsw(ops_form(rt, rn, simm, form))
    rhs: llvm_mc_word(asm_form)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  form: { gen: int, min: 0, max: 2, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; ARM ARM LDRSW pre/post (bits[11:10]=11/01) and LDURSW (bits[11:10]=00) simm9 in [-256,255]
```

## encode_ldrsw_diff_regoff_llvm_mc
- Tier: 2
- Rationale: llvm-mc is the independent assembler reference for LDRSW (register). ARM ARM: Xm with lsl/sxtx amount in {0,2}; Wm with uxtw/sxtw amount in {0,2}. Stronger oracles rejected as in encode_ldrsw_diff_unsigned_llvm_mc.
- Seed: load_store.rs encode_ldur_stur_pbt
- Formal: ∀ rt, rn, rm ∈ {0..31}, extend ∈ {lsl,sxtx,uxtw,sxtw}, amount ∈ {0,2}. If extend ∈ {lsl,sxtx} then index is Xm else Wm. Then encode_ldrsw([Reg(Xt), MemRegOffset{Xn|SP, index, extend, amount}]) = llvm-mc("ldrsw Xt, [Xn|SP, Rm, extend #amount]") as LE u32. Rm=31 is xzr/wzr.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, rm, extend, amount]
  domain: { rt: "0..=31", rn: "0..=31", rm: "0..=31", extend: "lsl|sxtx|uxtw|sxtw", amount: "{0,2}" }
  relation:
    op: eq
    lhs: encode_ldrsw(ops_regoff(rt, rn, rm, extend, amount))
    rhs: llvm_mc_word(asm_regoff)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extend: { gen: int, min: 0, max: 3, type: u32 }
  amount: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; ARM ARM LDRSW (register) size=10 111 0 00 10 1 Rm option S 10 Rn Rt
```

## encode_ldrsw_arm_fields
- Tier: 4
- Rationale: ARM ARM LDRSW encoding pins each field independently of llvm-mc parsing. Unsigned: size=10, bits[29:27]=111, V=0, bits[25:24]=01, opc=10, imm12=offset/4. Unscaled: bits[25:24]=00, bit21=0, bits[11:10]=00, imm9=simm. Pre bits[11:10]=11; post bits[11:10]=01. Register: bit21=1, bits[11:10]=10. Documented bounds size=10, imm12 in {0,1,4094,4095}, simm9 in {-256,-255,0,255}, Rt/Rn in {0,1,30,31} sampled exactly. Stronger differential already claimed by the three llvm-mc properties.
- Seed: load_store.rs encode_ldur_stur_pbt unpack_ldur
- Formal: ∀ rt, rn ∈ {0..31}, imm12 ∈ {0..4095}, simm ∈ [-256,255] with simm < 0 ∨ simm mod 4 ≠ 0. Let wu = encode_ldrsw(Mem unsigned). Then wu[31:30]=10 ∧ wu[29:27]=111 ∧ wu[26]=0 ∧ wu[25:24]=01 ∧ wu[23:22]=10 ∧ wu[21:10]=imm12 ∧ wu[9:5]=rn ∧ wu[4:0]=rt. Analogous field equalities hold for unscaled/pre/post/register forms.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, rn, imm12, simm]
  domain: { rt: "0..=31", rn: "0..=31", imm12: "0..=4095", simm: "-256..=255" }
  relation:
    op: holds
    expr: unpack_matches_arm(encode_ldrsw(ops))
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
evidence: ARM ARM LDRSW unsigned size=10 111 0 01 10 imm12 Rn Rt; LDURSW / pre / post / register option S
```

## encode_ldrsw_metamorphic_rt_rn_imm
- Tier: 4
- Rationale: ARM ARM places Rt at [4:0], Rn at [9:5], unsigned imm12 at [21:10]; incrementing Rt by 1 adds 1, Rn by 1 adds 32, imm12 by 1 adds 1024. Pre XOR post at equal operands = 0b10 << 10. Stronger differential already claimed. Not a same-job sibling differential.
- Seed: load_store.rs encode_ldaxr_stlxr_pbt encode_ldaxr_stlxr_metamorphic_l_size_o0
- Formal: ∀ rt ∈ {0..30}, rn ∈ {0..30}, imm12 ∈ {0..4094}. encode(rt+1,rn,imm12) − encode(rt,rn,imm12) = 1 ∧ encode(rt,rn+1,imm12) − encode(rt,rn,imm12) = 32 ∧ encode(rt,rn,imm12+1) − encode(rt,rn,imm12) = 1<<10. ∀ simm ∈ [-256,255]. encode(pre,simm) XOR encode(post,simm) = 0b10<<10.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, imm12, simm]
  domain: { rt: "0..=30", rn: "0..=30", imm12: "0..=4094", simm: "-256..=255" }
  relation:
    op: holds
    expr: "encode(rt+1)-encode(rt)=1 AND encode(rn+1)-encode(rn)=32 AND encode(imm12+1)-encode(imm12)=1<<10 AND encode(pre) XOR encode(post)=0b10<<10"
generators:
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm12: { gen: int, min: 0, max: 4094, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
evidence: ARM ARM LDRSW Rt[4:0] Rn[9:5] imm12[21:10]; pre bits[11:10]=11 vs post=01
```

## encode_ldrsw_neg_arity_kinds
- Tier: 4e
- Rationale: ARM ARM / llvm-mc require exactly two operands (Xt and a memory or literal operand). Fewer than 2 operands already Err in the body; Imm, Cond, Barrier, Shift, Extend, RegList, MemExpr, Label at the address slot are not LDRSW addressing modes and llvm-mc rejects them. Documented error contract: return Err. Stronger differential does not apply to the invalid domain.
- Seed: load_store.rs encode_ldur_stur_pbt extra_operand / arity
- Formal: ∀ ops. |ops| < 2 ∨ (ops[0] is Reg(Xt) ∧ ops[1] ∈ {Imm, Cond, Barrier, Shift, Extend, RegList, MemExpr, Label}) ⇒ encode_ldrsw(ops) is Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain: { ops: "arity<2 or address-slot not a load/store addressing mode" }
  relation:
    op: holds
    expr: encode_ldrsw(ops).is_err()
generators:
  ops: { gen: list, elem: { gen: string }, maxLen: 1 }
expected_error: String
evidence: ARM ARM LDRSW addressing modes; llvm-mc rejects Imm/Cond/Barrier as the address operand; encode_ldrsw:313 "ldrsw requires 2 operands"
```

## encode_ldrsw_neg_invalid_regs
- Tier: 4e
- Rationale: ARM ARM LDRSW dest is Xt (31=XZR), never Wt/SP/SIMD; base is Xn|SP, never W/XZR. llvm-mc rejects `ldrsw w0, [x1]`, `ldrsw sp, [x1]`, `ldrsw d0, [x1]`, `ldrsw x0, [w1]`, `ldrsw x0, [xzr]`, `ldrsw x0, [x1, w2]`, and writeback `ldrsw x0, [x0, #4]!`. Documented error: Err. Stronger differential does not apply to the invalid domain.
- Seed: load_store.rs encode_ldur_stur_pbt / encode_ldaxr_stlxr_pbt SP/W-base/FP negative contracts
- Formal: ∀ rt, rn ∈ {0..31}. encode_ldrsw([Reg(Wt), Mem{Xn,0}]) is Err ∧ encode_ldrsw([Reg("sp"), Mem{Xn,0}]) is Err ∧ encode_ldrsw([Reg(Dt|St|Qt), Mem{Xn,0}]) is Err ∧ encode_ldrsw([Reg(Xt), Mem{Wn,0}]) is Err ∧ encode_ldrsw([Reg(Xt), Mem{"xzr",0}]) is Err ∧ encode_ldrsw([Reg(Xt), MemRegOffset{Xn, Wm, None, None}]) is Err ∧ (rt ≠ 31 ⇒ encode_ldrsw([Reg(Xt), MemPreIndex{Xt, 4}]) is Err).
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, simd='d' — encode_ldrsw([Reg("w0"), Mem{x0,0}]) = Ok(Word(0xb9800000))
- Bug report: pbt-out/bug_reports/encode_ldrsw_w_dest.md

```property
function: encoder.load_store.encode_ldrsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn]
  domain: { rt: "0..=31", rn: "0..=31" }
  relation:
    op: holds
    expr: encode_ldrsw(invalid_reg_ops).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: ARM ARM LDRSW Xt, [Xn|SP]; llvm-mc rejects Wt/SP/SIMD dest, W-base, XZR-base
```

## encode_ldrsw_neg_offset_range_extra
- Tier: 4e
- Rationale: ARM ARM / llvm-mc: unsigned pimm in [0,16380] multiple of 4; otherwise simm9 in [-256,255]; pre/post simm9 in [-256,255]. Out-of-range offsets (-257, 257, 16384, pre/post 256) must be Err, not a truncated encoding. Documented bounds sampled at bound±1 (255/256/257, -256/-257, 16380/16384). Stronger differential does not apply to the invalid domain.
- Seed: load_store.rs encode_ldur_stur_pbt imm9_out_of_range
- Formal: ∀ rt, rn ∈ {0..31}, off ∉ unsigned-pimm ∧ off ∉ [-256,255]. encode_ldrsw([Reg(Xt), Mem{Xn, off}]) is Err ∧ encode_ldrsw(MemPre/MemPost with off) is Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, off=-257 — encode_ldrsw([Reg("x0"), Mem{x0,-257}]) = Ok(Word(0xb88ff000)) (imm9 wrapped to 255)
- Bug report: pbt-out/bug_reports/encode_ldrsw_offset_range.md

```property
function: encoder.load_store.encode_ldrsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, off]
  domain: { rt: "0..=31", rn: "0..=31", off: "out of unsigned-pimm and simm9" }
  relation:
    op: holds
    expr: encode_ldrsw(ops).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -1000, max: 20000, type: i64 }
expected_error: String
evidence: ARM ARM LDRSW pimm in [0,16380] multiple of 4 else simm9 in [-256,255]; llvm-mc "index must be an integer in range [-256, 255]"
```

## encode_ldrsw_neg_extra
- Tier: 4e
- Rationale: ARM ARM / llvm-mc LDRSW takes exactly two operands. A third operand is rejected. encode_ldrsw only checks len < 2, so extras are ignored. Documented error: Err.
- Seed: load_store.rs encode_ldur_stur_pbt extra_operand
- Formal: ∀ rt, rn ∈ {0..31}, extra ∈ Operand. encode_ldrsw([Reg(Xt), Mem{Xn,0}, extra]) is Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, extra=Reg("x2") — encode_ldrsw three operands = Ok(Word(0xb9800000))
- Bug report: pbt-out/bug_reports/encode_ldrsw_extra_operand.md

```property
function: encoder.load_store.encode_ldrsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, extra]
  domain: { rt: "0..=31", rn: "0..=31", extra: Operand }
  relation:
    op: holds
    expr: encode_ldrsw([Reg(Xt), Mem{Xn,0}, extra]).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: string }
expected_error: String
evidence: ARM ARM LDRSW two-operand syntax; llvm-mc extra operand invalid
```

## encode_ldrsw_diff_alt_spellings
- Tier: 2
- Rationale: Coverage-gaps sweep. llvm-mc accepts x31 as XZR, uppercase Xn/SP/XZR, and lr as X30. Same differential contract as encode_ldrsw_diff_unsigned_llvm_mc.
- Seed: load_store.rs encode_ldaxr_stlxr_pbt encode_ldaxr_stlxr_diff_alt_spellings
- Formal: ∀ rt, rn ∈ {0..31}, spelling ∈ {x31, uppercase, lr}. encode_ldrsw([Reg(spelling(rt)), Mem{spelling_base(rn), 0}]) = llvm-mc("ldrsw spelling(rt), [spelling_base(rn)]").
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, spelling]
  domain: { rt: "0..=31", rn: "0..=31", spelling: "x31|uppercase|lr" }
  relation:
    op: eq
    lhs: encode_ldrsw(ops_alt(rt, rn, spelling))
    rhs: llvm_mc_word(asm_alt)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  spelling: { gen: int, min: 0, max: 2, type: u32 }
evidence: src/backend/arm/assembler/README.md:11; llvm-mc accepts x31/XZR/LR/uppercase
```

## encode_ldrsw_neg_bad_extend_base
- Tier: 4e
- Rationale: Coverage-gaps sweep of the unsupported-extend Err arm and invalid base names. ARM ARM register-offset amount is only 0 or 2; uxtx is not valid on Xm for LDRSW; "foo" is not Xn|SP.
- Seed: load_store.rs encode_ldrsw unsupported extend/shift Err
- Formal: ∀ rt ∈ {0..31}, rn, rm ∈ {0..30}. encode_ldrsw(MemRegOffset lsl #1 or #3 or uxtx) is Err ∧ encode_ldrsw(Mem{base:"foo"}) is Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldrsw
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, rm, kind]
  domain: { rt: "0..=31", rn: "0..=30", rm: "0..=30", kind: "lsl1|lsl3|uxtx|foo" }
  relation:
    op: holds
    expr: encode_ldrsw(ops).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  rm: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: ARM ARM LDRSW (register) amount in {0,2}; llvm-mc rejects lsl #1/#3 and uxtx; parse_reg_num None => invalid base reg
```

## encode_ldrsw_literal_reloc
- Tier: 4
- Rationale: Coverage-gaps sweep. ARM ARM LDRSW (literal) 10 011 000 imm19 Rt; llvm-mc accepts `ldrsw Xt, label` with fixup_aarch64_ldr_pcrel_imm19; sibling encode_ldr_str maps Operand::Symbol to RelocType::Ldr19 for ldr. Not a same-job differential (ldr vs ldrsw opc). Stronger llvm-mc word compare rejected (encoding bytes contain fixup placeholders).
- Seed: encode_ldr_str Symbol arm in load_store.rs
- Formal: ∀ rt ∈ {0..31}. encode_ldrsw([Reg(Xt), Symbol("foo")]) = WordWithReloc { word[4:0]=rt ∧ word[31:24]=0x98 ∧ reloc_type=Ldr19 ∧ symbol="foo" }.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0 — Err("unsupported ldrsw operands: [Reg(\"x0\"), Symbol(\"foo\")]")
- Bug report: pbt-out/bug_reports/encode_ldrsw_literal.md

```property
function: encoder.load_store.encode_ldrsw
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt]
  domain: { rt: "0..=31" }
  relation:
    op: holds
    expr: encode_ldrsw([Reg(Xt), Symbol(foo)]) is WordWithReloc Ldr19 with opc 0x98
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM LDRSW (literal); llvm-mc ldrsw x0, label; encode_ldr_str Symbol => RelocType::Ldr19
```
