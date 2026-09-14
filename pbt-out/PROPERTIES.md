# Properties: encode_mvn

## encode_mvn_diff_gpr_llvm_mc
- Tier: 2
- Rationale: Strongest independent same-job reference is llvm-mc AArch64 assembler. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree MVN decoder). SUT-boundary: internal-helper of the GNU-style assembler whose public contract is gas-compatible AArch64 text (README.md:5-14). Mapping: operands <-> `mvn Rd, Rm{, shift #amt}`.
- Seed: encode_eon_pbt::encode_eon_diff_reg_llvm_mc
- Formal: ∀ rd,rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ [0, 31+32·is_64]. encode_mvn([Rd, Rm, Shift(kind,amt)]) = llvm-mc(`mvn Rd, Rm, kind #amt`) as a little-endian u32, where Rd/Rm are xN/xzr or wN/wzr of matching width (register 31 is ZR, never SP).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mvn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, amt]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, kind: {lsl,lsr,asr,ror}, amt: 0..(31+32*is_64) }
  relation:
    op: eq
    lhs: encode_mvn([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rm)), Shift(kind,amt)])
    rhs: llvm_mc("mvn " + gpr(is_64,rd) + ", " + gpr(is_64,rm) + ", " + kind + " #" + amt)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, values: [lsl, lsr, asr, ror] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: src/backend/arm/assembler/README.md:5-14; data_processing.rs:753; ARM ARM Logical (shifted register) MVN
```

## encode_mvn_diff_orn_alias
- Tier: 2
- Rationale: Documented alias MVN Rd, Rm ≡ ORN Rd, ZR, Rm (data_processing.rs:753). Independent reference is llvm-mc of both mnemonics, not encode_orn (shared construction). Metamorphic required at STANDARD. State machine / round-trip rejected as above.
- Seed: encode_mul_pbt::encode_mul_diff (MADD/XZR alias pattern)
- Formal: ∀ rd,rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ [0, 31+32·is_64]. encode_mvn([Rd, Rm, Shift(kind,amt)]) = llvm-mc(`orn Rd, ZR, Rm, kind #amt`) = llvm-mc(`mvn Rd, Rm, kind #amt`).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mvn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, amt]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, kind: {lsl,lsr,asr,ror}, amt: 0..(31+32*is_64) }
  relation:
    op: eq
    lhs: encode_mvn([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rm)), Shift(kind,amt)])
    rhs: llvm_mc("orn " + gpr(is_64,rd) + ", " + zr(is_64) + ", " + gpr(is_64,rm) + ", " + kind + " #" + amt)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, values: [lsl, lsr, asr, ror] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: data_processing.rs:753; ARM ARM MVN alias of ORN; llvm-mc aliases orn Rd, ZR, Rm to mvn
```

## encode_mvn_diff_neon_llvm_mc
- Tier: 2
- Rationale: README.md:225 lists NEON `not`/`mvn`. ARM ARM Advanced SIMD NOT T ∈ {8B,16B}. Independent reference llvm-mc. encode_neon_not is a callee, not a same-job sibling with a distinct public contract.
- Seed: encode_mul_pbt NEON differential
- Formal: ∀ vd,vn ∈ {0..31}, T ∈ {8b,16b}. encode_mvn([Vd.T, Vn.T]) = llvm-mc(`mvn Vd.T, Vn.T`) = llvm-mc(`not Vd.T, Vn.T`).
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mvn
oracle: differential
predicate:
  quantifier: forall
  vars: [vd, vn, t]
  domain: { vd: 0..31, vn: 0..31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_mvn([RegArrangement(v{vd}, t), RegArrangement(v{vn}, t)])
    rhs: llvm_mc("mvn v" + vd + "." + t + ", v" + vn + "." + t)
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: [8b, 16b] }
evidence: src/backend/arm/assembler/README.md:225; data_processing.rs:749; neon.rs:608-621; ARM ARM Advanced SIMD NOT
```

## encode_mvn_metamorphic_sf_xor
- Tier: 4
- Rationale: ARM ARM sf bit is the sole 32/64 discriminator for Logical (shifted register). Same register numbers at W vs X must differ by exactly bit 31. Weaker than differential; kept as a field-level metamorphic check independent of llvm-mc byte parsing.
- Seed: encode_mul_pbt sf XOR
- Formal: ∀ rd,rm ∈ {0..31}, kind ∈ {lsl,lsr,asr,ror}, amt ∈ [0,31]. encode_mvn(X-ops) XOR encode_mvn(W-ops) = 1<<31.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mvn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rm, kind, amt]
  domain: { rd: 0..31, rm: 0..31, kind: {lsl,lsr,asr,ror}, amt: 0..31 }
  relation:
    op: eq
    lhs: encode_mvn(x_ops) XOR encode_mvn(w_ops)
    rhs: 1u32 << 31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: oneof, values: [lsl, lsr, asr, ror] }
  amt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Logical (shifted register) sf at bit 31; data_processing.rs:755
```

## encode_mvn_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout of ORN/MVN: sf, opc=01, 01010, shift, N=1, Rm, imm6, Rn=31, Rd. Independent of llvm-mc. Weaker than differential.
- Seed: encode_eon_pbt::encode_eon_invariant_arm_fields
- Formal: ∀ rd,rm ∈ {0..31}, is_64 ∈ Bool, kind ∈ {lsl,lsr,asr,ror}, amt ∈ [0, 31+32·is_64]. let w = encode_mvn(...). (w>>31)&1 = sf(is_64) ∧ (w>>29)&3 = 0b01 ∧ (w>>24)&0x1F = 0b01010 ∧ (w>>22)&3 = st(kind) ∧ (w>>21)&1 = 1 ∧ (w>>16)&0x1F = rm ∧ (w>>10)&0x3F = amt ∧ (w>>5)&0x1F = 31 ∧ w&0x1F = rd.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mvn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, amt]
  domain: { rd: 0..31, rm: 0..31, is_64: bool, kind: {lsl,lsr,asr,ror}, amt: 0..(31+32*is_64) }
  body: fields(encode_mvn(...)) match ARM ARM ORN/MVN layout with Rn=31
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  is_64: { gen: bool }
  kind: { gen: oneof, values: [lsl, lsr, asr, ror] }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: ARM ARM Logical (shifted register) MVN/ORN; data_processing.rs:768-770
```

## encode_mvn_neg_arity_and_extra
- Tier: 4
- Rationale: ARM ARM / llvm-mc: MVN takes exactly two registers plus optional shift. Fewer than 2 operands is too few; a trailing non-shift extra operand is invalid. README gas-compatibility is the error contract. Negative/error is the strongest applicable for the invalid domain (no decoder to round-trip).
- Seed: encode_eon_pbt::encode_eon_neg_arity / encode_eon_neg_extra_operand
- Formal: ∀ ops. (|ops| < 2 ∨ (|ops| ≥ 3 ∧ ops[2] is not a valid Shift) ∨ |ops| ≥ 4) ∧ ops otherwise well-typed GPR ⇒ encode_mvn(ops) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: encode_mvn_neg_too_few passes; extra fails on [Reg(w0), Reg(w0), Reg(x0)] and [Reg(w0), Reg(w0), Shift{lsl,1}, Reg(x0)]
- Bug report: pbt-out/bug_reports/encode_mvn_extra_operand.md

```property
function: encoder.encode_mvn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain:
    ops: too-few GPR lists or two-GPR plus trailing non-shift extra
  relation:
    op: throws
    expr: encode_mvn(ops)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  extra: { gen: oneof, values: [Reg, Imm, Mem, Symbol] }
evidence: ARM ARM MVN operand list; llvm-mc rejects mvn x0 and mvn x0, x1, x2; README.md:5-14
```

## encode_mvn_neg_mixed_sp_fp
- Tier: 4
- Rationale: ARM ARM register 31 is XZR/WZR never SP/WSP; Rd and Rm must be the same width GPR; FP/SIMD names are not Logical (shifted register) operands. llvm-mc rejects all three. Gas-compatibility is the error contract.
- Seed: encode_eon_pbt::encode_eon_neg_mixed_width / encode_eon_neg_sp_fp
- Formal: ∀ mixed-width GPR pairs, or any slot in {Rd,Rm} being SP/WSP or {d,s,q,v,h,b}N. encode_mvn(ops) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: mixed [Reg(w0), Reg(x0)]; SP [Reg(wsp), Reg(w0)]; FP [Reg(d0), Reg(x1)]
- Bug report: pbt-out/bug_reports/encode_mvn_mixed_width.md; pbt-out/bug_reports/encode_mvn_sp.md; pbt-out/bug_reports/encode_mvn_fp_reg.md

```property
function: encoder.encode_mvn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain:
    ops: mixed X/W pair or SP/WSP or FP/SIMD name in Rd or Rm
  relation:
    op: throws
    expr: encode_mvn(ops)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: ARM ARM Logical (shifted register) register specifiers; llvm-mc rejects mvn x0 w1, mvn sp x0, mvn d0 x1
```

## encode_mvn_neg_shift_and_neon_t
- Tier: 4
- Rationale: Documented bounds: 32-bit imm6 in 0..31 (imm6<5>==1 UNALLOCATED); 64-bit 0..63. NEON T ∈ {8B,16B} only, matching source T. llvm-mc rejects out-of-range shift and T in {4h,8h,2s,4s,1d,2d} and mismatched T. Bounds sampled at 31/32/63/64 and each illegal T.
- Seed: encode_eon_pbt::encode_eon_neg_shift_range; encode_mul_pbt::encode_mul_neg_neon_d
- Formal: ∀ sf=0 ∧ amt ∈ {32,33,63,64} ∨ sf=1 ∧ amt ∈ {64,65,128} ∨ T ∉ {8b,16b} ∨ dest T ≠ src T. encode_mvn(ops) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: shift [Reg(w0), Reg(w0), Shift{lsl,32}]; neon T [v0.4h, v0.4h]; mismatch [v0.16b, v0.8b]
- Bug report: pbt-out/bug_reports/encode_mvn_shift_range.md; pbt-out/bug_reports/encode_mvn_neon_t.md; pbt-out/bug_reports/encode_mvn_neon_mismatch_t.md

```property
function: encoder.encode_mvn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [ops]
  domain:
    ops: GPR plus Shift with amt outside ARM ARM imm6 range or NEON T not in 8b/16b or mismatched T
  relation:
    op: throws
    expr: encode_mvn(ops)
expected_error: String
generators:
  amt_w: { gen: oneof, values: [32, 33, 63, 64] }
  amt_x: { gen: oneof, values: [64, 65, 128] }
  t: { gen: oneof, values: [4h, 8h, 2s, 4s, 1d, 2d] }
evidence: ARM ARM Logical (shifted register) imm6; ARM ARM Advanced SIMD NOT T in 8B/16B; llvm-mc rejects mvn w0 w1 lsl 32 and mvn v0.4s v1.4s
```

## encode_mvn_diff_lr
- Tier: 2
- Rationale: `lr` is a documented 64-bit alias of X30 (parse_reg_num / llvm-mc). Differential vs llvm-mc.
- Seed: encode_mul_pbt::encode_mul_diff_lr
- Formal: ∀ which ∈ {0,1}, other ∈ {0..30}, kind ∈ {lsl,lsr,asr,ror}, amt ∈ [0,63]. encode_mvn with `lr` in slot `which` equals llvm-mc of the same text.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mvn
oracle: differential
predicate:
  quantifier: forall
  vars: [which, other, kind, amt]
  domain:
    which: 0..1
    other: 0..30
    kind: lsl lsr asr ror
    amt: 0..63
  relation:
    op: eq
    lhs: encode_mvn(ops_with_lr)
    rhs: llvm_mc(asm_with_lr)
generators:
  which: { gen: int, min: 0, max: 1, type: u32 }
  other: { gen: int, min: 0, max: 30, type: u32 }
  amt: { gen: int, min: 0, max: 63, type: u32 }
evidence: encoder/mod.rs parse_reg_num lr => 30; llvm-mc accepts lr as x30
```

## encode_mvn_neg_too_few
- Tier: 4
- Rationale: llvm-mc rejects `mvn x0` (too few operands). Split out of arity property because extra-operand fails independently.
- Seed: encode_eon_pbt::encode_eon_neg_arity
- Formal: ∀ n ∈ {0,1}, is_64 ∈ Bool, r ∈ {0..31}. encode_mvn(ops[0..n]) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_mvn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n]
  domain:
    n: 0..1
  relation:
    op: throws
    expr: encode_mvn(ops[0..n])
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
evidence: llvm-mc rejects mvn x0 as too few operands
```

## encode_mvn_neg_bad_shift_kind
- Tier: 4
- Rationale: ARM ARM shift is LSL/LSR/ASR/ROR only. Unknown kind must Err. Strengthening round after first batch.
- Seed: encode_eon_pbt shift kind match
- Formal: ∀ rd,rm ∈ {0..30}, is_64 ∈ Bool, kind ∉ {lsl,lsr,asr,ror}, amt ∈ [0,31]. encode_mvn([Rd, Rm, Shift(kind,amt)]) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: rd=0, rm=0, is_64=false, kind="foo", amt=0
- Bug report: pbt-out/bug_reports/encode_mvn_bad_shift_kind.md

```property
function: encoder.encode_mvn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rm, is_64, kind, amt]
  domain:
    kind: not in lsl lsr asr ror
  relation:
    op: throws
    expr: encode_mvn([Reg(gpr(is_64,rd)), Reg(gpr(is_64,rm)), Shift(kind,amt)])
expected_error: String
generators:
  kind: { gen: oneof, values: [foo, lslv, rrx, empty, uxtw] }
  amt: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Logical (shifted register) shift; llvm-mc rejects unknown shift mnemonics
```

## encode_mvn_neg_neon_extra
- Tier: 4
- Rationale: NEON MVN is exactly two arrangement operands. Strengthening round after first batch.
- Seed: encode_mul extra operand
- Formal: ∀ vd,vn ∈ {0..31}, extra. encode_mvn([Vd.16b, Vn.16b, extra]) = Err.
- Test file: src/backend/arm/assembler/encoder/data_processing.rs
- Status: failing
- Counterexample: vd=0, vn=0, extra=Reg("x0")
- Bug report: pbt-out/bug_reports/encode_mvn_neon_extra.md

```property
function: encoder.encode_mvn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [vd, vn, extra]
  domain:
    vd: 0..31
    vn: 0..31
  relation:
    op: throws
    expr: encode_mvn([RegArrangement(v{vd}, 16b), RegArrangement(v{vn}, 16b), extra])
expected_error: String
generators:
  vd: { gen: int, min: 0, max: 31, type: u32 }
  vn: { gen: int, min: 0, max: 31, type: u32 }
evidence: ARM ARM Advanced SIMD NOT two operands; llvm-mc rejects mvn v0.16b v1.16b v2.16b
```
