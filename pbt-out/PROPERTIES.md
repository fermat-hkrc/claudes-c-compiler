# Properties: encode_ldur_stur

## encode_ldur_stur_diff_gpr_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (independent AArch64 assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree LDUR decoder. encode_ldr_str / encode_ldtr_sized fail the same-job sibling gate (LDR/STR vs unscaled; ldtrb/ldtrh vs this helper). SUT-boundary: internal-helper of the GNU-style assembler (README gas-compatible). Mapping: [Reg(Rt), Mem{Rn, offset}, is_load, op2] <-> `{ldur|stur|ldtr|sttr} Rt, [Rn{, #imm}]` over GPR. Doc evidence: README.md:5-14, encoder/mod.rs:1-7, load_store.rs:246-248, ARM ARM unscaled/unprivileged load/store.
- Seed: encode_ldar_stlr_pbt (same file, llvm-mc differential)
- Formal: ∀ rt,rn ∈ {0..31}, offset ∈ [-256, 255], is_load ∈ {T,F}, is_64 ∈ {T,F}, op2 ∈ {0b00, 0b10}. Let Rt = Xrt (rt=31 → xzr) if is_64 else Wrt (rt=31 → wzr); Rn = SP if rn=31 else Xrn. encode_ldur_stur([Rt, Mem(Rn, offset)], is_load, op2) = llvm-mc(mnemonic Rt, [Rn, #offset])
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldur_stur
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, offset, is_load, is_64, op2]
  domain: { rt: 0..31, rn: 0..31, offset: -256..255, op2: {0, 2} }
  relation:
    op: eq
    lhs: encode_ldur_stur([Reg(Rt), Mem(Rn, offset)], is_load, op2)
    rhs: llvm_mc("{ldur|stur|ldtr|sttr} Rt, [Rn, #offset]")
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  offset: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  is_64: { gen: bool }
  op2: { gen: int, min: 0, max: 1, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:336-339; load_store.rs:246-287
```

## encode_ldur_stur_diff_simd_llvm_mc
- Tier: 2
- Rationale: Differential vs llvm-mc for SIMD/FP Rt (B/H/S/D/Q) on LDUR/STUR only (op2=00). LDTR/STTR SIMD is invalid (llvm-mc rejects) and belongs in the negative-error property. Same stronger-oracle rejection as the GPR differential. Mapping: [Reg(Rt), Mem{Rn, offset}, is_load, 0b00] <-> `{ldur|stur} {b|h|s|d|q}t, [Rn{, #imm}]`.
- Seed: encode_ldar_stlr_pbt (same file, llvm-mc differential)
- Formal: ∀ rt,rn ∈ {0..31}, offset ∈ [-256, 255], is_load ∈ {T,F}, kind ∈ {b,h,s,d,q}. encode_ldur_stur([kind+rt, Mem(Rn, offset)], is_load, 0b00) = llvm-mc("{ldur|stur} kind+rt, [Rn, #offset]")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldur_stur
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, offset, is_load, kind]
  domain: { kind: {b,h,s,d,q}, offset: -256..255, op2: 0b00 }
  relation:
    op: eq
    lhs: encode_ldur_stur([Reg(kind+rt), Mem(Rn, offset)], is_load, 0b00)
    rhs: llvm_mc("{ldur|stur} kind+rt, [Rn, #offset]")
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  offset: { gen: int, min: -256, max: 255, type: i64 }
  is_load: { gen: bool }
  kind: { gen: oneof, items: ["b", "h", "s", "d", "q"] }
evidence: load_store.rs:257-273 SIMD size/opc; ARM ARM SIMD unscaled load/store
```

## encode_ldur_stur_roundtrip_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM unscaled layout claimed at load_store.rs:247. Differential is stronger and used above; this unpacks size/V/opc/imm9/op2/Rn/Rt so a packing slip still fails even if llvm-mc were unavailable. Stronger round-trip rejected: no decoder.
- Seed: encode_ldar_stlr_pbt::encode_ldar_stlr_invariant_fixed_bits
- Formal: ∀ valid GPR/SIMD inputs as above. let w = encode_ldur_stur(...). w[29:27]=111 ∧ w[25:24]=00 ∧ w[21]=0 ∧ w[20:12]=imm9_as_u9 ∧ w[11:10]=op2 ∧ w[9:5]=rn ∧ w[4:0]=rt ∧ V/size/opc match Rt class
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldur_stur
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, rn, offset, is_load, is_64, op2]
  relation:
    op: holds
    expr: unpack(encode_ldur_stur(...)) matches ARM unscaled/unprivileged layout
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  offset: { gen: int, min: -256, max: 255, type: i64 }
evidence: load_store.rs:247 Format size 111 V 00 opc 0 imm9 00 Rn Rt
```

## encode_ldur_stur_metamorphic_load_xor_store
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM opc for unsigned unscaled is 01 (load) vs 00 (store) for GPR/B/H/S/D, and 11 vs 10 for Q; both pairs differ only at bit 22. Stronger differential covers absolute encoding; this isolates the load/store toggle. Q is included so bit 23 stays set on both sides.
- Seed: encode_ldar_stlr_pbt::encode_ldar_stlr (L-bit XOR)
- Formal: ∀ rt,rn ∈ {0..31}, offset ∈ [-256, 255], kind ∈ {x,w,b,h,s,d,q}, op2 ∈ {0b00, 0b10} (op2=0b10 only for x/w). encode_ldur_stur(..., true, op2) XOR encode_ldur_stur(..., false, op2) = 1<<22
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldur_stur
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, offset, kind, op2]
  relation:
    op: eq
    lhs: encode(..., is_load=true) XOR encode(..., is_load=false)
    rhs: 1 << 22
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  offset: { gen: int, min: -256, max: 255, type: i64 }
evidence: load_store.rs:258-275 opc load vs store; ARM ARM opc at bits [23:22]
```

## encode_ldur_stur_metamorphic_unscaled_xor_unpriv
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM bits [11:10] are 00 (LDUR/STUR) vs 10 (LDTR/STTR) and are the only bits that distinguish the two classes at equal Rt/Rn/imm/opc. Restricted to GPR because SIMD LDTR is not a valid encoding. Stronger differential covers absolute encoding; this isolates op2.
- Seed: encode_ldar_stlr_pbt L-bit XOR
- Formal: ∀ rt,rn ∈ {0..31}, offset ∈ [-256, 255], is_load ∈ {T,F}, is_64 ∈ {T,F}. encode_ldur_stur(GPR, is_load, 0b00) XOR encode_ldur_stur(GPR, is_load, 0b10) = 1<<11
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldur_stur
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, offset, is_load, is_64]
  relation:
    op: eq
    lhs: encode(..., op2=0b00) XOR encode(..., op2=0b10)
    rhs: 1 << 11
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  offset: { gen: int, min: -256, max: 255, type: i64 }
evidence: encoder/mod.rs:336-339 op2 00 vs 10; ARM ARM bits [11:10]
```

## encode_ldur_stur_neg_extra_operands
- Tier: 4e
- Rationale: Negative/error contract: llvm-mc rejects a third operand (`invalid operand for instruction`); README gas-compatible assembler must reject the same. Arity check is `len < 2` (not `!= 2`). Stronger differential does not apply on the invalid domain.
- Seed: encode_ldar_stlr_pbt::encode_ldar_stlr_neg_extra_operands
- Formal: ∀ valid 2-operand LDUR/STUR/LDTR/STTR ops, extra ∈ {Reg, Imm, Mem, Symbol}. encode_ldur_stur(ops ++ [extra], is_load, op2) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, offset=-256, is_load=false, is_64=false, unpriv=false, extra=Reg("x2") — stur w0, [x0, #-256], x2
- Bug report: pbt-out/bug_reports/encode_ldur_stur_extra_operand.md

```property
function: encoder.load_store.encode_ldur_stur
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, offset, is_load, is_64, op2, extra]
  relation:
    op: throws
    expr: encode_ldur_stur(ops ++ [extra], is_load, op2)
expected_error: String
generators:
  extra: { gen: oneof, items: ["Reg", "Imm", "Mem", "Symbol"] }
evidence: llvm-mc rejects a 3rd operand; README.md:5-14 gas-compatible
```

## encode_ldur_stur_neg_imm9_range
- Tier: 4e
- Rationale: Negative/error contract: ARM ARM and llvm-mc require signed 9-bit offset in [-256, 255] (`index must be an integer in range [-256, 255]`). Bounds -257 and 256 (bound±1) plus extremes must be rejected. The body masks with 0x1FF with no range check. Stronger differential does not apply on the invalid domain.
- Seed: encode_adr_pbt imm-out-of-range (same file)
- Formal: ∀ rt,rn ∈ {0..31}, offset ∈ {i64 \ [-256,255]}, is_load, is_64, op2 ∈ {0b00,0b10}. encode_ldur_stur([Rt, Mem(Rn, offset)], is_load, op2) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, offset=-257, is_load=false, is_64=false, unpriv=false — stur w0, [x0, #-257]
- Bug report: pbt-out/bug_reports/encode_ldur_stur_imm9_range.md

```property
function: encoder.load_store.encode_ldur_stur
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, offset, is_load, is_64, op2]
  domain: { offset: i64 minus closed interval -256 to 255 }
  relation:
    op: throws
    expr: encode_ldur_stur([Reg(Rt), Mem(Rn, offset)], is_load, op2)
expected_error: String
generators:
  offset: { gen: int, min: -9223372036854775808, max: 9223372036854775807, type: i64 }
evidence: ARM ARM simm9; llvm-mc index must be an integer in range [-256, 255]
```

## encode_ldur_stur_neg_invalid_rt_rn
- Tier: 4e
- Rationale: Negative/error contract from llvm-mc / ARM ARM: SP/WSP as Rt (Rt=31 is ZR); W/WSP/XZR/WZR as base (Rn is Xn|SP); SIMD Rt on LDTR/STTR; V-register Rt (no arrangement). `ldur lr` is a valid 64-bit alias of x30 (covered by a dedicated alias property below — this entry is the reject set). Stronger differential does not apply on the invalid domain.
- Seed: encode_ldar_stlr_pbt::encode_ldar_stlr_neg_invalid_rt / neg_invalid_base_offset
- Formal: ∀ kind ∈ {sp-as-Rt, wsp-as-Rt, W-base, wsp-base, xzr-base, wzr-base, SIMD-Rt on LDTR/STTR, V-Rt}. encode_ldur_stur(kind) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: is_load=false, kind=0, n=0, offset=-256 — stur sp, [x0, #-256] (also W-base, XZR-base, SIMD-LDTR, V-Rt)
- Bug report: pbt-out/bug_reports/encode_ldur_stur_sp_as_rt.md

```property
function: encoder.load_store.encode_ldur_stur
oracle: negative_error
predicate:
  quantifier: forall
  vars: [kind, n, is_load]
  relation:
    op: throws
    expr: encode_ldur_stur(invalid_ops(kind))
expected_error: String
generators:
  kind: { gen: int, min: 0, max: 7, type: u32 }
evidence: llvm-mc rejects SP as Rt, [xzr]/[wN] as base, SIMD Rt on ldtr/sttr, vN as Rt
```

## encode_ldur_stur_neg_arity_and_shape
- Tier: 4e
- Rationale: Negative/error contract from the body (`len < 2` → Err; non-Mem second operand → Err; get_reg non-Reg → Err; parse_reg_num None → Err) and llvm-mc (pre/post-index, register offset, unknown base). Coverage-sweep: these arms were not reached by the first 8 properties. Stronger differential does not apply on the invalid domain.
- Seed: encode_ldar_stlr_pbt::encode_ldar_stlr_neg_arity_and_shape
- Formal: ∀ shape ∈ {empty, 1-op, Reg+Imm, Reg+Symbol, pre-index, post-index, Imm+Mem, base=foo, base=x32}, is_load, op2. encode_ldur_stur(shape) = Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldur_stur
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, unpriv, shape, rt, off]
  relation:
    op: throws
    expr: encode_ldur_stur(arity_or_shape_ops(shape), is_load, op2)
expected_error: String
generators:
  shape: { gen: int, min: 0, max: 8, type: u32 }
evidence: load_store.rs:250-251 arity; load_store.rs:282 non-Mem; encoder/mod.rs:956 get_reg; encoder/mod.rs:131 parse_reg_num
```

## encode_ldur_stur_diff_lr_alias
- Tier: 2
- Rationale: Differential vs llvm-mc for the architectural LR = X30 alias. Coverage-sweep: encode_ldur_stur sizes GPR via `starts_with('x')` and misses `lr`, while sibling `is_64bit_reg` (mod.rs:151-154) and `encode_ldr_str_auto` (load_store.rs:17) treat `lr` as 64-bit. llvm-mc accepts `ldur lr, [x0]` as `ldur x30, [x0]`.
- Seed: encode_ldur_stur_diff_gpr_llvm_mc
- Formal: ∀ rn ∈ {0..31}, offset ∈ [-256, 255], is_load ∈ {T,F}, op2 ∈ {0b00, 0b10}. encode_ldur_stur([Reg("lr"), Mem(Rn, offset)], is_load, op2) = llvm-mc("{ldur|stur|ldtr|sttr} lr, [Rn, #offset]")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rn=0, offset=-256, is_load=false, unpriv=false — stur lr, [x0, #-256] encodes as stur w30 (0xb810001e) vs llvm-mc stur x30 (0xf810001e)
- Bug report: pbt-out/bug_reports/encode_ldur_stur_lr_as_w30.md

```property
function: encoder.load_store.encode_ldur_stur
oracle: differential
predicate:
  quantifier: forall
  vars: [rn, offset, is_load, unpriv]
  relation:
    op: eq
    lhs: encode_ldur_stur([Reg("lr"), Mem(Rn, offset)], is_load, op2)
    rhs: llvm_mc("{ldur|stur|ldtr|sttr} lr, [Rn, #offset]")
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  offset: { gen: int, min: -256, max: 255, type: i64 }
evidence: encoder/mod.rs:151-154 is_64bit_reg; load_store.rs:17 encode_ldr_str_auto treats lr as 64-bit; llvm-mc accepts lr as X30
```
