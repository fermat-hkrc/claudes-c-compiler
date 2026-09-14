# Properties: encode_ldaxr_stlxr

## encode_ldaxr_stlxr_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree exclusive-acquire decoder). Same-job siblings encode_ldxr_stxr / encode_ldxp_stxp / encode_ldar_stlr rejected (o0=0 exclusive / exclusive-pair / ordered non-exclusive; different jobs). Doc evidence: assembler README.md:11 "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:354-359 dispatch of ldaxr/stlxr/ldaxrb/stlxrb/ldaxrh/stlxrh; ARM ARM Load/Store Exclusive o0=1; README.md:221 Loads/Stores table.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_diff_llvm_mc
- Formal: ∀ rt, rn, ws ∈ {0..31}, is_load ∈ {0,1}, variant ∈ {0,1,2}, is_64 ∈ {0,1}. Let data_64 = (variant=0 ∧ is_64). If ¬is_load ⇒ ¬stlxr_ws_aliases_source(ws, rt, rn). Then encode_ldaxr_stlxr(ops, is_load, forced(variant)) = llvm-mc("{ldaxr|ldaxrb|ldaxrh|stlxr|stlxrb|stlxrh} ...") as LE u32, where X31/W31 data is xzr/wzr, Rn=31 is sp, Ws=31 is wzr, byte/half take Wt, and ops is [Reg(Rt), Mem{Rn,0}] on load else [Reg(Ws), Reg(Rt), Mem{Rn,0}].
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, ws, is_load, variant, is_64]
  domain: { rt: "0..=31", rn: "0..=31", ws: "0..=31", variant: "0..=2" }
  relation:
    op: eq
    lhs: encode_ldaxr_stlxr(ops, is_load, forced(variant))
    rhs: llvm_mc_word(asm)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:11; encoder/mod.rs:354-359; ARM ARM Load/Store Exclusive o0=1
```

## encode_ldaxr_stlxr_arm_fields
- Tier: 4
- Rationale: ARM ARM Load/Store Exclusive encoding size 001000 0 L 0 Rs o0 Rt2 Rn Rt with o0=1, Rt2=11111, o1=0, bits[28:23]=001000, bit23=0. Weaker than differential; kept as an exact structural invariant that pins each field independently of llvm-mc parsing. Documented bounds size in {00,01,10,11}, L in {0,1}, Rs=11111 on load else Ws, Rn/Rt in 0..31 sampled at 0/1/30/31.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_arm_fields
- Formal: ∀ rt, rn, ws ∈ {0..31}, is_load ∈ {0,1}, variant ∈ {0,1,2}, is_64 ∈ {0,1}. Let w = encode_ldaxr_stlxr(ops, is_load, forced(variant)). Then w[31:30]=expected_size(variant, data_64) ∧ w[28:23]=001000 ∧ w[23]=0 ∧ w[22]=is_load ∧ w[21]=0 ∧ w[20:16]=(is_load ? 31 : ws) ∧ w[15]=1 ∧ w[14:10]=31 ∧ w[9:5]=rn ∧ w[4:0]=rt.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, rn, ws, is_load, variant, is_64]
  domain: { rt: "0..=31", rn: "0..=31", ws: "0..=31", variant: "0..=2" }
  relation:
    op: holds
    expr: unpack_matches_arm(encode_ldaxr_stlxr(ops, is_load, forced(variant)))
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM Load/Store Exclusive size 001000 0 L 0 Rs o0=1 Rt2=11111 Rn Rt
```

## encode_ldaxr_stlxr_metamorphic_l_size_o0
- Tier: 4
- Rationale: ARM ARM documents L as the load/store bit, size[31:30] as the access width, and o0 as the acquire/release bit distinguishing LDAXR/STLXR from LDXR/STXR. Metamorphic relations: ldaxr XOR stlxr(wzr) = 1<<22; X XOR W = 1<<30; byte XOR half = 1<<30; encode_ldaxr_stlxr XOR encode_ldxr_stxr at equal operands = 1<<15. Stronger differential already claimed by encode_ldaxr_stlxr_diff_llvm_mc. Sibling encode_ldxr_stxr is a different job so XOR-o0 is metamorphic, not differential.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_metamorphic_l_size
- Formal: ∀ rt ∈ {0..30}, rn, ws ∈ {0..31}. encode_ldaxr([Xt], [Xn]) XOR encode_stlxr(wzr, Xt, [Xn]) = 1<<22 ∧ encode_ldaxr(Xt) XOR encode_ldaxr(Wt) = 1<<30 ∧ (if ¬stlxr_ws_aliases_source(ws,rt,rn) then encode_stlxrb XOR encode_stlxrh = 1<<30) ∧ encode_ldaxr_stlxr(ops, is_load, forced) XOR encode_ldxr_stxr(ops, is_load, forced) = 1<<15.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, ws]
  domain: { rt: "0..=30", rn: "0..=31", ws: "0..=31" }
  relation:
    op: eq
    lhs: encode_ldaxr_stlxr(ops, is_load, forced) XOR encode_ldxr_stxr(ops, is_load, forced)
    rhs: "1u32 << 15"
generators:
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Load/Store Exclusive L=bit22, size=bits[31:30], o0=bit15 (1 for acquire/release)
```

## encode_ldaxr_stlxr_neg_extra_operand
- Tier: 4e
- Rationale: gas/llvm-mc reject a trailing operand on LDAXR/STLXR (llvm-mc: "invalid operand for instruction"). README.md:11 claims gas compatibility. Negative/error contract: extra operand ⇒ Err. Stronger oracles do not cover this invalid domain.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_neg_extra_operand
- Formal: ∀ valid ops of encode_ldaxr_stlxr, extra ∈ Operand. encode_ldaxr_stlxr(ops ++ [extra], is_load, forced) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, ws=0, is_load=false, variant=0, is_64=false, extra=Reg("x2") — stlxr w0, w0, [x0], x2 → Ok(Word(0x8800FC00))
- Bug report: pbt-out/bug_reports/encode_ldaxr_stlxr_extra_operand.md

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, ws, is_load, variant, is_64, extra]
  domain: { extra: "Reg|Imm|Symbol|Mem" }
  relation:
    op: throws
    expr: encode_ldaxr_stlxr(ops ++ [extra], is_load, forced(variant))
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: ["Reg(x2)", "Imm(0)", "Imm(1)", "Symbol(foo)", "Mem{x3,0}"] }
evidence: llvm-mc rejects trailing operand; README.md:11 gas compatibility; encoder/mod.rs:354-359
```

## encode_ldaxr_stlxr_neg_invalid_regs
- Tier: 4e
- Rationale: ARM ARM / llvm-mc: Rt is Wt/Xt (31=ZR never SP); Rn is Xn|SP never Wn/XZR/WZR; SIMD/FP not allowed as Rt; STLXR status is Ws never Xs; LDAXRB/LDAXRH take Wt not Xt. README.md:11 gas compatibility. Negative/error: each of those names ⇒ Err.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_neg_invalid_regs
- Formal: ∀ n, other ∈ {0..31}, kind ∈ {SP-as-Rt, W-base, XZR-base, FP-as-Rt, X-as-Ws, X-data-on-byte}. encode_ldaxr_stlxr(ops(kind), ...) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: kind=0, n=0 — ldaxr sp, [x0] → Ok(Word(0xC85FFC1F)). Additional witnesses (same property kinds 1–5, confirmed by regression tests): [w1] W-base, [xzr] XZR-base, d0 FP-as-Rt, stlxr x0 X-as-Ws, ldaxrb x0 X-data-on-byte.
- Bug report: pbt-out/bug_reports/encode_ldaxr_stlxr_sp_as_rt.md (also encode_ldaxr_stlxr_w_base.md, encode_ldaxr_stlxr_xzr_as_base.md, encode_ldaxr_stlxr_fp_as_rt.md, encode_ldaxr_stlxr_x_as_ws.md, encode_ldaxr_stlxr_x_data_byte.md)

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n, other]
  domain: { kind: "0..=5", n: "0..=31" }
  relation:
    op: throws
    expr: encode_ldaxr_stlxr(invalid_reg_ops(kind, n, other), is_load, forced)
expected_error: String
generators:
  kind: { gen: int, min: 0, max: 5, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  other: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc "invalid operand"; ARM ARM LDAXR Rt=Wt/Xt Rn=Xn|SP; STLXR Ws; LDAXRB Wt
```

## encode_ldaxr_stlxr_neg_arity_shape
- Tier: 4e
- Rationale: Exclusive load/store addressing is [Xn|SP]{,#0} only. llvm-mc: "index must be absent or #0"; too few operands, Imm/Symbol/pre/post-index, invalid base names, and nonzero offset are rejected. README.md:11 gas compatibility. Documented bound offset=0 sampled at 0 (valid, other properties) and ±1 / ±8 / 256 / i64 min/max (invalid here).
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_neg_arity_shape
- Formal: ∀ shape ∈ {empty, dest-only, Imm, Symbol, MemPreIndex, MemPostIndex, base=foo, base=x32, offset≠0}. encode_ldaxr_stlxr(ops(shape), is_load, None) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: is_load=false, shape=8, rt=0, offset=-1 — stlxr w1, x0, [x2, #-1] → Ok(Word(0xC801FC40))
- Bug report: pbt-out/bug_reports/encode_ldaxr_stlxr_nonzero_offset.md

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, shape, rt, offset]
  domain: { shape: "0..=8", offset: "nonzero i64" }
  relation:
    op: throws
    expr: encode_ldaxr_stlxr(ops(shape), is_load, None)
expected_error: String
generators:
  is_load: { gen: bool }
  shape: { gen: int, min: 0, max: 8, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  offset: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: llvm-mc "index must be absent or #0"; ARM ARM LDAXR/STLXR addressing [Xn|SP]{,#0}
```

## encode_ldaxr_stlxr_neg_ws_overlap
- Tier: 4e
- Rationale: llvm-mc rejects STLXR when the status register is also a source ("unpredictable STXR instruction, status is also a source"): Ws==Rt, or Ws==Rn with Rn≠SP (WZR vs SP both encode 31 is allowed). ARM ARM CONSTRAINED UNPREDICTABLE. README.md:11 gas compatibility. Negative/error: overlap ⇒ Err.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_neg_ws_overlap
- Formal: ∀ rt, rn ∈ {0..31}, variant ∈ {0,1,2}. If ws aliases rt or (rn≠31 ∧ ws=rn), then encode_ldaxr_stlxr(store_ops(ws,rt,rn), false, forced(variant)) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, variant=0, is_64=false, overlap_rt=false — stlxr w0, w0, [x0] → Ok(Word(0x8800FC00))
- Bug report: pbt-out/bug_reports/encode_ldaxr_stlxr_ws_overlap.md

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, variant, overlap_rt]
  domain: { rt: "0..=31", rn: "0..=31", variant: "0..=2" }
  relation:
    op: throws
    expr: encode_ldaxr_stlxr(store_ops(ws, rt, rn), false, forced(variant))
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  overlap_rt: { gen: bool }
evidence: llvm-mc "unpredictable STXR instruction, status is also a source"; ARM ARM CONSTRAINED UNPREDICTABLE
```

## encode_ldaxr_stlxr_diff_alt_spellings
- Tier: 2
- Rationale: Coverage-gaps sweep (no LLVM profraw). Documented gas/llvm-mc aliases: x31=XZR, uppercase Xn/SP/XZR, lr=X30, W-form ldaxr. Differential vs llvm-mc. Seeded from encode_ldxr_stxr_pbt alt-spelling KATs (lr) plus ARM register-name aliases.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_kat_llvm_mc_ldxr_lr
- Formal: ∀ rt, rn ∈ {0..31}, spelling ∈ {x31, uppercase, lr, W-form}. encode_ldaxr_stlxr([Reg(alias(rt)), Mem{alias(rn),0}], true, None) = llvm-mc("ldaxr alias(rt), [alias(rn)]").
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, spelling]
  domain: { rt: "0..=31", rn: "0..=31", spelling: "0..=3" }
  relation:
    op: eq
    lhs: encode_ldaxr_stlxr([Reg(alias(rt, spelling)), Mem{alias(rn, spelling), 0}], true, None)
    rhs: llvm_mc_word("ldaxr {alias(rt)}, [{alias(rn)}]")
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  spelling: { gen: int, min: 0, max: 3, type: u32 }
evidence: llvm-mc accepts x31/XZR/X0/lr/SP; README.md:11 gas compatibility; parse_reg_num lowercases and maps lr=30 x31=31
```

## encode_ldaxr_stlxr_neg_mem_index
- Tier: 4e
- Rationale: Coverage-gaps sweep. ARM ARM / llvm-mc: exclusive addressing is [Xn|SP]{,#0} only; [Xn, Xm] and symbolic MemExpr must be rejected ("index must be absent or #0"). arity_shape covered Imm/pre/post/nonzero-imm but not MemRegOffset or MemExpr.
- Seed: load_store.rs encode_ldxr_stxr_pbt encode_ldxr_stxr_neg_arity_shape
- Formal: ∀ rt, rn ∈ {0..31}, idx ∈ {0..30}, kind ∈ {MemRegOffset, MemExpr, MemRegOffset+lsl}. encode_ldaxr_stlxr([Reg(Xt), mem(kind)], true, None) = Err.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldaxr_stlxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, idx, kind]
  domain: { rt: "0..=31", rn: "0..=31", idx: "0..=30", kind: "0..=2" }
  relation:
    op: throws
    expr: encode_ldaxr_stlxr([Reg(Xt), mem(kind)], true, None)
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  idx: { gen: int, min: 0, max: 30, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u32 }
evidence: llvm-mc "index must be absent or #0"; ARM ARM LDAXR addressing [Xn|SP]{,#0}
```
