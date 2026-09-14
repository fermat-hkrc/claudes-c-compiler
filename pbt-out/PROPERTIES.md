# Properties: encode_ldxr_stxr

## encode_ldxr_stxr_diff_llvm_mc
- Tier: 7
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, which implements the same GNU-style AArch64 assembler contract (README.md:5-14). State machine rejected: pure function, no lifecycle. In-tree LDXR decoder rejected: none exists. encode_ldaxr_stlxr / encode_ldxp_stxp / encode_ldar_stlr fail the same-job sibling gate (o0=1 acquire-release exclusive; exclusive pair o1=1; ordered non-exclusive bit23=1).
- Seed: encode_ldxp_stxp_pbt::encode_ldxp_stxp_diff_llvm_mc; codegen/atomics.rs:24-25
- Formal: ∀ rt,rn,ws ∈ {0..31}, is_load ∈ Bool, variant ∈ {word,byte,half}, is_64 ∈ Bool. valid_ldxr_stxr(rt,rn,ws,is_load,variant,is_64) ⇒ encode_ldxr_stxr(ops, is_load, forced(variant)) = llvm-mc("{ldxr|stxr|ldxrb|stxrb|ldxrh|stxrh} ...")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxr_stxr
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, ws, is_load, variant, is_64]
  domain: { rt: 0..31, rn: 0..31, ws: 0..31, is_load: bool, variant: {word,byte,half}, is_64: bool }
  relation:
    op: eq
    lhs: encode_ldxr_stxr(ops(rt,rn,ws,is_load,variant,is_64), is_load, forced(variant))
    rhs: llvm_mc(asm(rt,rn,ws,is_load,variant,is_64))
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64 text; encoder/mod.rs:348-353 dispatch; ARM ARM Load/Store Exclusive
```

## encode_ldxr_stxr_arm_fields
- Tier: 4d
- Rationale: Algebraic invariant from ARM ARM exclusive encoding `size 001000 0 L 0 Rs o0 Rt2 Rn Rt`. Stronger differential is the sibling property above; this pins field layout independently of llvm-mc. Round-trip decoder rejected: none in-tree.
- Seed: encode_ldxp_stxp_pbt field unpack
- Formal: ∀ valid inputs. let w = encode_ldxr_stxr(...). w[29:24]=001000 ∧ w[23]=0 ∧ w[21]=0 ∧ w[15]=0 ∧ w[14:10]=11111 ∧ w[31:30]=size ∧ w[22]=L ∧ w[20:16]=(is_load ? 31 : ws) ∧ w[9:5]=rn ∧ w[4:0]=rt
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxr_stxr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, rn, ws, is_load, variant, is_64]
  domain: { rt: 0..31, rn: 0..31, ws: 0..31 }
  relation:
    op: holds
    expr: arm_exclusive_single_fields(encode_ldxr_stxr(ops, is_load, forced(variant)))
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM Load/Store Exclusive size 001000 0 L 0 Rs o0 Rt2 Rn Rt; load_store.rs:547-576 comment
```

## encode_ldxr_stxr_metamorphic_l_size
- Tier: 4c
- Rationale: ARM ARM places L at bit 22 and size at [31:30]. Independently of llvm-mc, flipping load/store with Rs=31 (WZR, no overlap) must XOR 1<<22; flipping X/W word size must XOR 1<<30; flipping byte/half must XOR 1<<30. Stronger differential is the sibling property.
- Seed: encode_ldxp_stxp_pbt o0/sz metamorphic
- Formal: ∀ rt,rn with rt≠31. encode(load,Rt,Rn) XOR encode(store,Ws=31,Rt,Rn) = 1<<22. ∀ rt,rn. encode(X) XOR encode(W) = 1<<30. ∀ rt,rn,ws valid. encode(byte) XOR encode(half) = 1<<30
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxr_stxr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, ws]
  domain: { rt: 0..31, rn: 0..31, ws: 0..31 }
  relation:
    op: eq
    lhs: encode_ldxr_stxr(load_ops, true, None) XOR encode_ldxr_stxr(store_ops_ws31, false, None)
    rhs: 1 << 22
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM exclusive L bit 22, size [31:30]
```

## encode_ldxr_stxr_neg_extra_operand
- Tier: 4e
- Rationale: llvm-mc and GNU as reject a trailing extra operand ("invalid operand" / too many). The assembler README claims gas-compatible text, so extra operands must Err. Documented invalid domain: arity above 2 (load) / 3 (store).
- Seed: encode_ldxp_stxp_pbt extra operand; llvm-mc `ldxr x0, [x1], x2` error
- Formal: ∀ valid ops, extra ∈ Operand. encode_ldxr_stxr(ops ++ [extra], ...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, ws=0, is_load=false, variant=0, is_64=false, extra=Reg("x2") — stxr w0, w0, [x0], x2 → Ok(Word(0x88007c00))
- Bug report: pbt-out/bug_reports/encode_ldxr_stxr_extra_operand.md

```property
function: encoder.load_store.encode_ldxr_stxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, ws, is_load, variant, extra]
  domain: { extra: Operand }
  relation:
    op: throws
    expr: encode_ldxr_stxr(ops ++ [extra], is_load, forced(variant))
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, options: [Reg, Imm, Symbol, Mem] }
evidence: llvm-mc rejects extra operand; README.md:5-14 gas-compatible
```

## encode_ldxr_stxr_neg_invalid_regs
- Tier: 4e
- Rationale: ARM ARM Rt is Wt/Xt (31=ZR not SP); Rn is Xn|SP not W/XZR/WZR/WSP; STXR status is Ws (31=WZR) not Xs/SP; byte/half data is Wt not Xt; no SIMD/FP. llvm-mc rejects each class. Gas-compatible contract requires Err.
- Seed: encode_ldxp_stxp_pbt SP/W-base/XZR/FP/X-Ws negatives
- Formal: ∀ invalid register-class inputs in {SP-as-Rt, W-base, XZR-base, WSP-base, FP-as-Rt, X-as-Ws, X-data-on-byte/half}. encode_ldxr_stxr(...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: kind=0, n=0 — ldxr sp, [x0] → Ok(Word) (also kind 1–5: W-base, XZR-base, FP Rt, X as Ws, X data on byte)
- Bug report: pbt-out/bug_reports/encode_ldxr_stxr_sp_as_rt.md

```property
function: encoder.load_store.encode_ldxr_stxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n]
  domain: { kind: 0..5, n: 0..31 }
  relation:
    op: throws
    expr: encode_ldxr_stxr(invalid_ops(kind, n), is_load, forced_size)
expected_error: String
generators:
  kind: { gen: int, min: 0, max: 5, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM exclusive Rt=Wt/Xt Rn=Xn|SP Ws=Wt; llvm-mc rejects SP/W-base/XZR/FP/X-Ws/X-on-byte
```

## encode_ldxr_stxr_neg_arity_shape
- Tier: 4e
- Rationale: LDXR requires exactly 2 operands (Rt, [Xn]); STXR requires exactly 3 (Ws, Rt, [Xn]). Memory form is unscaled [Xn] or [Xn,#0] only — pre-index, post-index, Imm, Symbol, and nonzero offset are invalid (llvm-mc: "index must be absent or #0" / "invalid operand" / "too few operands").
- Seed: encode_ldxp_stxp_pbt arity/shape; llvm-mc `ldxr x0, [x1, #8]` and `ldxr x0`
- Formal: ∀ too-few / non-Mem / MemPreIndex / MemPostIndex / Imm / Symbol / nonzero-offset shapes. encode_ldxr_stxr(...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: is_load=false, shape=8, rt=0, offset=-1 — stxr w1, x0, [x2, #-1] → Ok(Word) (shapes 0–7 correctly Err)
- Bug report: pbt-out/bug_reports/encode_ldxr_stxr_nonzero_offset.md

```property
function: encoder.load_store.encode_ldxr_stxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [shape, is_load, offset]
  domain: { shape: 0..8, offset: i64 }
  relation:
    op: throws
    expr: encode_ldxr_stxr(shape_ops, is_load, None)
expected_error: String
generators:
  shape: { gen: int, min: 0, max: 8, type: u32 }
  is_load: { gen: bool }
  offset: { gen: int, min: -4096, max: 4096, type: i64 }
evidence: llvm-mc too few operands / index must be absent or #0; ARM ARM exclusive offset {,#0}
```

## encode_ldxr_stxr_neg_ws_overlap
- Tier: 4e
- Rationale: llvm-mc rejects STXR when the status register is also a source (Ws number equals Rt or equals Rn unless Rn is SP). ARM CONSTRAINED UNPREDICTABLE. Gas-compatible contract requires Err. WZR vs SP (both encode 31) is allowed.
- Seed: encode_ldxp_stxp_pbt ws_overlap; llvm-mc `stxr w0, x0, [x2]` "unpredictable STXR instruction, status is also a source"
- Formal: ∀ ws,rt,rn with (ws=rt ∨ (rn≠31 ∧ ws=rn)). encode_ldxr_stxr(store_ops(ws,rt,rn), false, ...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, variant=0, is_64=false, overlap_rt=false — stxr w0, w0, [x0] → Ok(Word(0x88007c00))
- Bug report: pbt-out/bug_reports/encode_ldxr_stxr_ws_overlap.md

```property
function: encoder.load_store.encode_ldxr_stxr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ws, rt, rn, variant, is_64]
  domain: { ws: 0..31, rt: 0..31, rn: 0..31 }
  relation:
    op: throws
    expr: encode_ldxr_stxr(store_ops(ws, rt, rn), false, forced(variant))
expected_error: String
generators:
  ws: { gen: int, min: 0, max: 31, type: u32 }
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc unpredictable STXR, status is also a source; ARM CONSTRAINED UNPREDICTABLE
```
