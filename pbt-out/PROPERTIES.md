# Properties: encode_ldxp_stxp

## encode_ldxp_stxp_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (independent AArch64 assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree exclusive-pair decoder. encode_ldxr_stxr / encode_ldaxr_stlxr / encode_ldp_stp fail the same-job sibling gate (exclusive-single vs exclusive-pair; non-exclusive pair vs exclusive pair). SUT-boundary: internal-helper of the GNU-style assembler (README gas-compatible). Mapping: load [Reg(Rt), Reg(Rt2), Mem{Rn, 0}] <-> `{ldxp|ldaxp} Rt, Rt2, [Rn]`; store [Reg(Ws), Reg(Rt), Reg(Rt2), Mem{Rn, 0}] <-> `{stxp|stlxp} Ws, Rt, Rt2, [Rn]`. Doc evidence: README.md:5-14, encoder/mod.rs:1-7, encoder/mod.rs:366-369, load_store.rs:596-634, ARM ARM Load/Store Exclusive Pair.
- Seed: encode_ldar_stlr_pbt::encode_ldar_stlr_diff_llvm_mc
- Formal: ∀ rt,rt2,rn,ws ∈ {0..31}, is_load ∈ {T,F}, acqrel ∈ {T,F}, is_64 ∈ {T,F}. If ¬is_load, require ws ≠ rt ∧ ws ≠ rt2 ∧ (rn=31 ∨ ws ≠ rn) (ARM/llvm-mc: STXP status must not also be a source; WZR vs SP is allowed). Let Rt/Rt2 = Xn (n=31 → xzr) if is_64 else Wn (n=31 → wzr); Rn = SP if rn=31 else Xrn; Ws = Wws (ws=31 → wzr). encode_ldxp_stxp(ops, is_load, acqrel) = llvm-mc(mnemonic ops)
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, ws, is_load, acqrel, is_64]
  domain: { rt: 0..31, rt2: 0..31, rn: 0..31, ws: 0..31 }
  relation:
    op: eq
    lhs: encode_ldxp_stxp(ops, is_load, acqrel)
    rhs: llvm_mc("{ldxp|ldaxp|stxp|stlxp} ...")
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  acqrel: { gen: bool }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:366-369; load_store.rs:596-634
```

## encode_ldxp_stxp_invariant_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM exclusive-pair layout claimed at load_store.rs:598-603. Differential is stronger and used above; this unpacks size/L/o1/Rs/o0/Rt2/Rn/Rt so a packing slip still fails even if llvm-mc were unavailable. Stronger round-trip rejected: no decoder.
- Seed: encode_ldar_stlr_pbt::encode_ldar_stlr_roundtrip_arm_fields
- Formal: ∀ valid inputs as above. let w = encode_ldxp_stxp(...). w[31]=1 ∧ w[30]=sz ∧ w[29:24]=001000 ∧ w[23]=0 ∧ w[22]=is_load ∧ w[21]=1 ∧ (is_load ⇒ w[20:16]=31 else w[20:16]=ws) ∧ w[15]=acqrel ∧ w[14:10]=rt2 ∧ w[9:5]=rn ∧ w[4:0]=rt
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, ws, is_load, acqrel, is_64]
  relation:
    op: holds
    expr: arm_exclusive_pair_fields(encode_ldxp_stxp(ops, is_load, acqrel))
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  acqrel: { gen: bool }
  is_64: { gen: bool }
evidence: load_store.rs:598-603; ARM ARM Load/Store Exclusive Pair
```

## encode_ldxp_stxp_metamorphic_o0
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM o0 bit (acquire/release) is the sole difference between LDXP/STXP and LDAXP/STLXP. Stronger differential used above; this relation is independent of llvm-mc. State machine / round-trip rejected as above.
- Seed: encode_ldar_stlr_pbt L-bit XOR
- Formal: ∀ valid ops, is_load. encode_ldxp_stxp(ops, is_load, true) XOR encode_ldxp_stxp(ops, is_load, false) = 1<<15
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, ws, is_load, is_64]
  relation:
    op: eq
    lhs: encode_ldxp_stxp(ops, is_load, true) XOR encode_ldxp_stxp(ops, is_load, false)
    rhs: 1 << 15
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  is_64: { gen: bool }
evidence: load_store.rs:598-603 o0; encoder/mod.rs:366-369 ldaxp/stlxp vs ldxp/stxp
```

## encode_ldxp_stxp_metamorphic_sz
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM size bit 30 is the sole difference between W-pair (size=10) and X-pair (size=11) at equal register numbers. Stronger differential used above.
- Seed: encode_ldar_stlr_pbt size field
- Formal: ∀ rt,rt2,rn,ws ∈ {0..31}, is_load, acqrel. encode_ldxp_stxp(X-ops, is_load, acqrel) XOR encode_ldxp_stxp(W-ops, is_load, acqrel) = 1<<30
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, ws, is_load, acqrel]
  relation:
    op: eq
    lhs: encode_ldxp_stxp(Xops, is_load, acqrel) XOR encode_ldxp_stxp(Wops, is_load, acqrel)
    rhs: 1 << 30
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ws: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  acqrel: { gen: bool }
evidence: load_store.rs:616 sz from is_64; ARM ARM size=10/11
```

## encode_ldxp_stxp_neg_ws_overlap
- Tier: 4
- Rationale: Negative/error contract from llvm-mc ("unpredictable STXP instruction, status is also a source") and ARM CONSTRAINED UNPREDICTABLE when STXP Ws aliases Rt, Rt2, or Xn. WZR vs SP (both encode 31) is allowed and excluded. Stronger oracles rejected: these inputs are outside the valid domain. Differential filters this case; this property pins the rejection.
- Seed: llvm-mc error on `stxp wzr, wzr, w0, [x0]`
- Formal: ∀ rt,rt2,rn ∈ {0..31}, acqrel, is_64, kind ∈ {ws=rt, ws=rt2, ws=rn ∧ rn≠31}. encode_ldxp_stxp(store_ops(ws, rt, rt2, rn), false, acqrel) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rt2=0, rn=0, acqrel=false, is_64=false, kind=0 — stxp w0, w0, w0, [x0] (Ws aliases Rt)
- Bug report: pbt-out/bug_reports/encode_ldxp_stxp_ws_overlap.md

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, acqrel, is_64, kind]
  relation:
    op: throws
    expr: encode_ldxp_stxp(store_ops_with_ws_aliasing_source, false, acqrel)
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  acqrel: { gen: bool }
  is_64: { gen: bool }
  kind: { gen: int, min: 0, max: 2, type: u32 }
evidence: llvm-mc "unpredictable STXP instruction, status is also a source"; ARM ARM STXP CONSTRAINED UNPREDICTABLE
```

## encode_ldxp_stxp_neg_arity_extra
- Tier: 4
- Rationale: Negative/error contract from llvm-mc and gas-compatible README: load needs exactly 3 operands, store exactly 4; extra operands are invalid. Documented bound sampled at arity-1 and arity+1. Stronger oracles rejected: extra operands are outside the valid domain.
- Seed: encode_ldar_stlr_pbt::test_encode_ldar_stlr_regression_extra_operand
- Formal: ∀ valid prefix ops, extra ∈ Operand\{empty}. encode_ldxp_stxp(ops++[extra], is_load, acqrel) = Err. ∀ too-short prefixes (load len<3, store len<4). encode_ldxp_stxp(prefix, ...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rt2=0, rn=0, ws=0, is_load=false, acqrel=false, is_64=false, extra=Reg("x2"), short_len=0 — stxp w0, w0, w0, [x0], x2
- Bug report: pbt-out/bug_reports/encode_ldxp_stxp_extra_operand.md

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, ws, is_load, acqrel, is_64, extra]
  relation:
    op: throws
    expr: encode_ldxp_stxp(ops ++ [extra], is_load, acqrel)
expected_error: String
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, items: ["Reg(x2)", "Imm(0)", "Symbol(foo)"] }
evidence: llvm-mc rejects extra operand; README.md:5-14 gas-compatible
```

## encode_ldxp_stxp_neg_invalid_rt_base
- Tier: 4
- Rationale: Negative/error: llvm-mc rejects SP as Rt/Rt2/Ws (register 31 is ZR, never SP), XZR/WZR/W/WSP as base (Rn is Xn|SP), SIMD/FP as data or status, mixed X/W pair, and X as STXP status. Bound: register 31 as Rt vs as Rn is the documented SP/ZR split. Stronger oracles rejected: these inputs are outside the valid domain.
- Seed: encode_ldar_stlr_pbt::test_encode_ldar_stlr_regression_sp_as_rt / _w_base
- Formal: ∀ is_load, acqrel. encode_ldxp_stxp with Rt/Rt2/Ws ∈ {sp,wsp} = Err; with base ∈ {xzr,wzr,wN,wsp,foo,x32} = Err; with SIMD prefix {d,s,q,v,h,b} as Rt = Err; mixed X/W Rt/Rt2 = Err; STXP Ws starting with x = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: n=0, rn=0, is_load=false, acqrel=false, is_64=false, kind=0 — stxp w0, sp, w0, [x0] (Ok(Word(0xc81f003f)))
- Bug report: pbt-out/bug_reports/encode_ldxp_stxp_sp_as_rt.md

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, acqrel, bad_kind]
  relation:
    op: throws
    expr: encode_ldxp_stxp(bad_ops, is_load, acqrel)
expected_error: String
generators:
  is_load: { gen: bool }
  acqrel: { gen: bool }
  bad_kind: { gen: oneof, items: ["sp_rt", "w_base", "xzr_base", "fp_rt", "mixed_width", "x_ws"] }
evidence: llvm-mc rejects SP as Rt, W/XZR base, SIMD Rt, mixed width, X as STXP Ws; README.md:5-14
```

## encode_ldxp_stxp_neg_offset_nonmem
- Tier: 4
- Rationale: Negative/error: llvm-mc "index must be absent or #0"; non-Mem addressing (Imm/Symbol/pre/post/reg-offset) is invalid. Documented offset bound 0 sampled at 0 (valid, covered by differential) and at ±1/±8/nonzero (invalid). Stronger oracles rejected: outside valid domain.
- Seed: encode_ldar_stlr_pbt nonzero offset / non-Mem
- Formal: ∀ valid register triple, offset ∈ ℤ\{0}. encode_ldxp_stxp(..., Mem{Rn, offset}, ...) = Err. ∀ non-Mem addressing mode at the memory slot. encode_ldxp_stxp(...) = Err. Invalid register names (empty, foo, x32, x99) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rt2=0, rn=0, ws=0, is_load=false, acqrel=false, is_64=false, offset=-1, shape=0 — stxp w0, w0, w0, [x0, #-1]
- Bug report: pbt-out/bug_reports/encode_ldxp_stxp_nonzero_offset.md

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, offset, is_load, acqrel, is_64]
  domain: { offset: int excluding 0 }
  relation:
    op: throws
    expr: encode_ldxp_stxp(ops_with_nonzero_offset, is_load, acqrel)
expected_error: String
generators:
  offset: { gen: int, min: -4096, max: 4096, type: i64 }
  is_load: { gen: bool }
  acqrel: { gen: bool }
evidence: llvm-mc "index must be absent or #0"; load_store.rs:611-612 / 625-626 Mem-only
```

## encode_ldxp_stxp_neg_too_short_nonmem_badname
- Tier: 4
- Rationale: Coverage-sweep negative/error for documented Err arms the extra/offset properties never execute (they fail on the success-path bugs first): too few operands (get_reg None), non-Reg first operand, non-Mem at the memory slot, parse_reg_num None on the base. llvm-mc / get_reg / match `_` evidence.
- Seed: encode_ldar_stlr_pbt get_reg non-Reg / parse_reg_num None sweep
- Formal: ∀ too-short prefixes, non-Reg Rt, non-Mem addressing, base ∈ {foo, x32}. encode_ldxp_stxp(...) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldxp_stxp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rt2, rn, ws, is_load, acqrel, is_64, shape]
  relation:
    op: throws
    expr: encode_ldxp_stxp(too_short_or_nonmem_or_badname, is_load, acqrel)
expected_error: String
generators:
  shape: { gen: int, min: 0, max: 6, type: u32 }
  is_load: { gen: bool }
  acqrel: { gen: bool }
evidence: load_store.rs:608-612 / 622-626 get_reg and Mem match; parse_reg_num None
```
