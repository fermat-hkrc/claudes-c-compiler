# Properties: encode_ldr_str

## encode_ldr_str_diff_unsigned_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent GNU-style AArch64 assembler). README.md:12 claims gas-compatible textual assembly; encoder/mod.rs:473-478 dispatch ldr/str/ldrb/strb/ldrh/strh onto this symbol. State machine rejected (pure function). Round-trip rejected (no in-tree LDR/STR decoder). Sibling encode_ldur_stur/encode_ldrsw/encode_ldrs rejected (same-job gate). Valid domain is unsigned-offset Rt, [Xn|SP, #pimm] with pimm = imm12*(1<<size).
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_diff_unsigned_llvm_mc
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt ∈ 0..31, rn ∈ 0..31, imm12 ∈ 0..4095. let scale=1<<size; pimm=imm12*scale; dest=W(rt) if size<3 else X(rt); base=SP if rn=31 else X(rn); mnemonic=ldrb/strb/ldrh/strh/ldr/str. encode_ldr_str([Reg(dest), Mem{base,pimm}], is_load, size, false, false) = llvm_mc(mnemonic dest, [base{, #pimm}]) as Word
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, size, rt, rn, imm12]
  domain: { is_load: bool, size: 0..3, rt: 0..31, rn: 0..31, imm12: 0..4095 }
  relation:
    op: eq
    lhs: encode_ldr_str(unsigned_ops)
    rhs: llvm_mc(unsigned_asm)
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_diff_unscaled_pre_post_llvm_mc
- Tier: 2
- Rationale: Same llvm-mc differential on unscaled/pre/post forms. llvm-mc canonicalizes unaligned/negative LDR/STR to LDUR/STUR. Writeback Rt==Rn (Rn!=SP) is excluded from this valid-domain generator (negative_error covers it).
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_diff_unscaled_pre_post_llvm_mc
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt ∈ 0..31, rn ∈ 0..31, simm ∈ [-256,255], form ∈ {mem, pre, post}. (form ∈ {pre,post} ∧ rt=rn ∧ rn≠31) excluded. encode_ldr_str([Reg(gp_rt), form(base,simm)], is_load, size, false, false) = llvm_mc(corresponding asm) as Word
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, size, rt, rn, simm, form]
  domain: { is_load: bool, size: 0..3, rt: 0..31, rn: 0..31, simm: -256..255, form: 0..2 }
  relation:
    op: eq
    lhs: encode_ldr_str(form_ops)
    rhs: llvm_mc(form_asm)
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
  form: { gen: int, min: 0, max: 2, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_diff_regoff_llvm_mc
- Tier: 2
- Rationale: Same llvm-mc differential on register-offset form. ARM option in {UXTW,LSL,SXTW,SXTX}; S amount 0 or size.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_diff_regoff_llvm_mc
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn,rm ∈ 0..31, option ∈ valid(size), s ∈ {0,1}. encode_ldr_str([Reg(gp_rt), MemRegOffset{Xn|SP, Rm, option, shift}], is_load, size, false, false) = llvm_mc(mnemonic Rt, [Xn|SP, Rm{, option #shift}]) as Word
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: is_load=false, size=0, rt=0, rn=0, rm=0, w_index=false, ext_sel=0, s_bit=1 — strb w0, [x0, x0, lsl #0]; SUT=0x38206800 llvm-mc=0x38207800
- Bug report: bug_reports/encode_ldr_str_byte_lsl0.md

```property
function: encoder.encode_ldr_str
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, size, rt, rn, rm, w_index, ext_sel, s_bit]
  domain: { is_load: bool, size: 0..3, rt: 0..31, rn: 0..31, rm: 0..31, w_index: bool, ext_sel: 0..1, s_bit: 0..1 }
  relation:
    op: eq
    lhs: encode_ldr_str(regoff_ops)
    rhs: llvm_mc(regoff_asm)
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  w_index: { gen: bool }
  ext_sel: { gen: int, min: 0, max: 1, type: u32 }
  s_bit: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_arm_fields
- Tier: 4
- Rationale: ARM ARM bitfield layout of the unsigned-offset GPR form is an exact structural invariant.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_arm_fields
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt ∈ 0..31, rn ∈ 0..31, imm12 ∈ 0..4095. let w = encode_ldr_str([Reg(gp_rt), Mem{Xn|SP, imm12*(1<<size)}], is_load, size, false, false). w[31:30]=size ∧ w[29:27]=111 ∧ w[26]=0 ∧ w[25:24]=01 ∧ w[23:22]=(01 if is_load else 00) ∧ w[21:10]=imm12 ∧ w[9:5]=rn ∧ w[4:0]=rt
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_load, size, rt, rn, imm12]
  domain: { is_load: bool, size: 0..3, rt: 0..31, rn: 0..31, imm12: 0..4095 }
  relation:
    op: eq
    lhs: encode_ldr_str(unsigned_ops)
    rhs: arm_unsigned_pack(size, is_load, imm12, rn, rt)
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
evidence: ARM ARM LDR/STR (immediate unsigned) size 111 V 01 opc imm12 Rn Rt
```

## encode_ldr_str_meta_rt_rn_imm
- Tier: 4
- Rationale: Field isolation is a metamorphic consequence of the ARM layout: Rt+1 / Rn+1 / imm12+1 / load-vs-store opc / pre XOR post.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_metamorphic_rt_rn_imm
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..30, imm12 ∈ 0..4094, simm ∈ [-256,255]. enc(rt+1)-enc(rt)=1 ∧ enc(rn+1)-enc(rn)=32 ∧ enc(imm12+1)-enc(imm12)=1<<10 ∧ load XOR store = 1<<22 ∧ pre XOR post = 0b10<<10
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_load, size, rt, rn, imm12, simm]
  domain: { is_load: bool, size: 0..3, rt: 0..30, rn: 0..30, imm12: 0..4094, simm: -256..255 }
  relation:
    op: eq
    lhs: enc(rt + 1, rn, imm12) - enc(rt, rn, imm12)
    rhs: 1
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm12: { gen: int, min: 0, max: 4094, type: u32 }
  simm: { gen: int, min: -256, max: 255, type: i64 }
evidence: ARM ARM LDR/STR field layout Rt[4:0] Rn[9:5] imm12[21:10] opc[23:22]
```

## encode_ldr_str_neg_arity_kinds
- Tier: 5
- Rationale: llvm-mc rejects LDR/STR with fewer than two operands or a non-memory second operand. README.md:12 gas-compat is the error contract. Extra operands are a separate property (encode_ldr_str_neg_offset_extra).
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_neg_arity_kinds
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, ops with len<2 or ops[1] not in {Mem, MemPreIndex, MemPostIndex, MemRegOffset, Symbol-if-load}. encode_ldr_str(ops, is_load, size, false, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, size, rt, kind]
  domain: { is_load: bool, size: 0..3, rt: 0..31, kind: non_mem_operand }
  relation:
    op: holds
    expr: encode_ldr_str(short_or_bad).is_err()
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_neg_invalid_regs
- Tier: 5
- Rationale: llvm-mc rejects SP as Rt, XZR/x31 as base, W-prefixed base, W-index without uxtw/sxtw, and pre/post writeback with Rt==Rn. parse_reg_num maps SP and XZR both to 31 and accepts W bases; those inputs stay in the generator.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_neg_invalid_regs
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31. encode_ldr_str with Rt=SP or base in {XZR,x31,Wn,WSP} or W-index with extend=None or (pre/post ∧ rt=rn ∧ rn≠31) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: is_load=false, size=0, rt=0, rn=0 — STRB SP, [X0] returns Ok(Word(0x3D00001F)) instead of Err
- Bug report: bug_reports/encode_ldr_str_sp_dest.md

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, size, rt, rn]
  domain: { is_load: bool, size: 0..3, rt: 0..31, rn: 0..30 }
  relation:
    op: holds
    expr: encode_ldr_str(invalid_reg_ops).is_err()
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_neg_offset_extra
- Tier: 5
- Rationale: llvm-mc rejects a 3rd operand and rejects offsets outside unsigned pimm and simm9 [-256,255]. The body only checks len<2 and masks imm9 with 0x1FF.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_neg_offset_range_extra
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31, off ∉ valid_pimm ∪ [-256,255], extra ∈ Operand. encode_ldr_str([Rt, Mem{base,off}], ...) is Err ∧ encode_ldr_str([Rt, Mem{base,0}, extra], ...) is Err ∧ encode_ldr_str([Rt, Pre/Post{base,off}], ...) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: is_load=false, size=0, rt=0, rn=0, off=-257 — STRB W0, [X0, #-257] returns Ok(Word(0x380FF000)) instead of Err
- Bug report: bug_reports/encode_ldr_str_offset_range.md

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, size, rt, rn, off, extra]
  domain: { is_load: bool, size: 0..3, rt: 0..31, rn: 0..31, off: out_of_range_i64, extra: Operand }
  relation:
    op: holds
    expr: encode_ldr_str(out_of_range_or_extra).is_err()
generators:
  is_load: { gen: bool }
  size: { gen: int, min: 0, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  off: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_diff_simd_llvm_mc
- Tier: 2
- Rationale: Coverage-gaps sweep — unsigned SIMD S/D/Q path (V=1, is_128bit for Q) vs llvm-mc. Same differential oracle as the GPR unsigned property.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: encode_ldr_str_diff_unsigned_llvm_mc
- Formal: ∀ is_load ∈ {0,1}, fp ∈ {S,D,Q}, rt,rn ∈ 0..31, imm12 ∈ 0..4095. encode_ldr_str([Reg(fp_rt), Mem{Xn|SP, imm12*scale}], is_load, size(fp), false, is_128(fp)) = llvm_mc(ldr/str fp_rt, [base, #pimm])
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, fp_kind, rt, rn, imm12]
  domain: { is_load: bool, fp_kind: 0..2, rt: 0..31, rn: 0..31, imm12: 0..4095 }
  relation:
    op: eq
    lhs: encode_ldr_str(simd_ops)
    rhs: llvm_mc(simd_asm)
generators:
  is_load: { gen: bool }
  fp_kind: { gen: int, min: 0, max: 2, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm12: { gen: int, min: 0, max: 4095, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_diff_alt_spellings
- Tier: 2
- Rationale: Coverage-gaps sweep — x31 / uppercase Xn,XZR,SP / lr alias vs llvm-mc.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_diff_alt_spellings
- Formal: ∀ is_load ∈ {0,1}, rt,rn ∈ 0..31, spelling ∈ {x31, UPPER, lr}. encode_ldr_str([Reg(spell(rt)), Mem{spell(rn), 0}], is_load, 0b11, false, false) = llvm_mc(ldr/str spell(rt), [spell(rn)])
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, rt, rn, spelling]
  domain: { is_load: bool, rt: 0..31, rn: 0..31, spelling: 0..2 }
  relation:
    op: eq
    lhs: encode_ldr_str(alt_ops)
    rhs: llvm_mc(alt_asm)
generators:
  is_load: { gen: bool }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  spelling: { gen: int, min: 0, max: 2, type: u32 }
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_literal_reloc
- Tier: 4
- Rationale: Coverage-gaps sweep — LDR (literal) Symbol operand is a documented form (load-only). ARM opc 011 00 imm19 Rt plus RelocType::Ldr19. STR + Symbol must Err (llvm-mc rejects STR literal).
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: src/backend/arm/assembler/encoder/load_store.rs:encode_ldrsw_literal_reloc
- Formal: ∀ size ∈ {10,11}, rt ∈ 0..31. encode_ldr_str([Reg(gp_rt), Symbol("foo")], true, size, false, false) = WordWithReloc { word: (opc<<30)|(0b011<<27)|rt, Ldr19, "foo", 0 } ∧ encode_ldr_str(..., false, ...) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_ldr_str
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [size, rt]
  domain: { size: {2, 3}, rt: 0..31 }
  relation:
    op: eq
    lhs: encode_ldr_str([Reg(gp_rt), Symbol(foo)], true, size, false, false)
    rhs: WordWithReloc(Ldr19)
generators:
  size: { gen: int, min: 2, max: 3, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM LDR (literal); load_store.rs:213-238
```

## encode_ldr_str_neg_xzr_base
- Tier: 5
- Rationale: Split from encode_ldr_str_neg_invalid_regs. llvm-mc rejects XZR/x31 as base.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: test_encode_ldr_str_regression_xzr_base
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt ∈ 0..31. encode_ldr_str([Reg(gp_rt), Mem{base:XZR, offset:0}], is_load, size, false, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: LDR X0, [XZR] returns Ok(Word(0xF94003E0)) instead of Err
- Bug report: bug_reports/encode_ldr_str_xzr_base.md

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt]
  domain: { rt: 0..31 }
  relation:
    op: holds
    expr: encode_ldr_str([Reg(x0), Mem{base:xzr, offset:0}], true, 0b11, false, false).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_neg_w_base
- Tier: 5
- Rationale: Split from encode_ldr_str_neg_invalid_regs. llvm-mc rejects W-prefixed base.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: test_encode_ldr_str_regression_w_base
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, n ∈ 0..30. encode_ldr_str([Reg(gp_rt), Mem{base:W(n), offset:0}], is_load, size, false, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: LDR X0, [W0] returns Ok(Word(0xF9400000)) instead of Err
- Bug report: bug_reports/encode_ldr_str_w_base.md

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain: { n: 0..30 }
  relation:
    op: holds
    expr: encode_ldr_str([Reg(x0), Mem{base:w0, offset:0}], true, 0b11, false, false).is_err()
generators:
  n: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_neg_w_index
- Tier: 5
- Rationale: Split from encode_ldr_str_neg_invalid_regs. llvm-mc rejects W index without uxtw/sxtw.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: test_encode_ldr_str_regression_w_index
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn,rm ∈ 0..31. encode_ldr_str([Reg(gp_rt), MemRegOffset{Xn, Wm, extend:None}], is_load, size, false, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: LDR X0, [X1, W2] returns Ok(Word(0xF8624820)) instead of Err
- Bug report: bug_reports/encode_ldr_str_w_index.md

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt]
  domain: { rt: 0..31 }
  relation:
    op: holds
    expr: encode_ldr_str(w_index_no_extend).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_neg_writeback_overlap
- Tier: 5
- Rationale: Split from encode_ldr_str_neg_invalid_regs. llvm-mc rejects writeback Rt==Rn.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: test_encode_ldr_str_regression_writeback_overlap
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt ∈ 0..30. encode_ldr_str([Reg(gp_rt), MemPreIndex{base:X(rt), offset:scale}], is_load, size, false, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: LDR X0, [X0, #8]! returns Ok(Word(0xF8408C00)) instead of Err
- Bug report: bug_reports/encode_ldr_str_writeback_overlap.md

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt]
  domain: { rt: 0..30 }
  relation:
    op: holds
    expr: encode_ldr_str(preindex_rt_eq_rn).is_err()
generators:
  rt: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```

## encode_ldr_str_neg_extra_operand
- Tier: 5
- Rationale: Split from encode_ldr_str_neg_offset_extra. llvm-mc rejects a third operand.
- Doc contract: (none) — encode_ldr_str has no rustdoc
- Seed: test_encode_ldr_str_regression_extra_operand
- Formal: ∀ is_load ∈ {0,1}, size ∈ {0,1,2,3}, rt,rn ∈ 0..31, extra ∈ Operand. encode_ldr_str([Rt, Mem{base,0}, extra], is_load, size, false, false) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldr_str_pbt.rs
- Status: failing
- Counterexample: STRB W0, [X0], X0 returns Ok(Word(0x39000000)) instead of Err
- Bug report: bug_reports/encode_ldr_str_extra_operand.md

```property
function: encoder.encode_ldr_str
oracle: negative_error
predicate:
  quantifier: forall
  vars: [extra]
  domain: { extra: Operand }
  relation:
    op: holds
    expr: encode_ldr_str([Reg(w0), Mem{x0,0}, extra], false, 0, false, false).is_err()
generators:
  extra: { gen: const, value: "x0" }
expected_error: String
evidence: src/backend/arm/assembler/README.md:12
```
