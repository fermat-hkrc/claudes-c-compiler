# Properties: encode_ldp_stp

## encode_ldp_stp_diff_signed_offset_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, which the assembler README claims gas-compatibility with. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree LDP/STP decoder). Sibling encode_ldnp_stnp rejected (same-job gate: non-temporal, bits[25:23]=000, shared get_reg).
- Doc contract: load_store.rs:503 "LDP/STP rt1, rt2, [base, #offset] (signed offset)" — asserted fingerprint ff9a7af8
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_diff_unsigned_llvm_mc
- Formal: ∀ is_load ∈ Bool, is_64 ∈ Bool, rt1,rt2,rn ∈ 0..31, imm7 ∈ [-64,63]. (is_load ⇒ rt1 ≠ rt2) ⇒ encode_ldp_stp([Reg(Rt1), Reg(Rt2), Mem{Xn|SP, imm7·scale}], is_load) = llvm-mc("ldp/stp Rt1, Rt2, [Xn|SP, #imm7·scale]") where scale=8 if is_64 else 4, Rt is Xt/Wt (31=XZR/WZR), Rn is Xn|SP (31=SP)
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldp_stp
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, imm7]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, imm7: i32_-64_63 }
  body: (is_load => rt1 != rt2) => encode_ldp_stp([Reg(gp(is_64,rt1)), Reg(gp(is_64,rt2)), Mem{rn_name(rn), imm7*scale(is_64)}], is_load) == llvm_mc(asm)
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm7: { gen: int, min: -64, max: 63, type: i32 }
evidence: README.md:12 gas-compatible textual assembly; README.md:221 ldp/stp listed; encoder/mod.rs:492-493 dispatch; ARM ARM C6 LDP/STP signed offset
```

## encode_ldp_stp_diff_pre_post_llvm_mc
- Tier: 5
- Rationale: Same differential vs llvm-mc for pre-index and post-index forms. Writeback with Rn in {Rt1,Rt2} and Rn!=SP is ARM-unpredictable and llvm-mc-rejected, so the valid-domain generator excludes it (negative_error covers it).
- Doc contract: load_store.rs:487 "STP rt1, rt2, [base, #offset]! (pre-index)" — asserted fingerprint eed85232
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_diff_unscaled_pre_post_llvm_mc
- Formal: ∀ is_load ∈ Bool, is_64 ∈ Bool, rt1,rt2,rn ∈ 0..31, imm7 ∈ [-64,63], pre ∈ Bool. (is_load ⇒ rt1 ≠ rt2) ∧ (rn=31 ∨ (rn≠rt1 ∧ rn≠rt2)) ⇒ encode_ldp_stp([Reg(Rt1), Reg(Rt2), Pre/Post{Xn|SP, imm7·scale}], is_load) = llvm-mc(corresponding asm)
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldp_stp
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, imm7, pre]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, imm7: i32_-64_63, pre: bool }
  body: valid_wb => encode_ldp_stp(pre_or_post) == llvm_mc(asm)
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm7: { gen: int, min: -64, max: 63, type: i32 }
  pre: { gen: bool }
evidence: README.md:12; load_store.rs:487-501 pre/post forms; ARM ARM writeback LDP/STP
```

## encode_ldp_stp_inv_arm_layout
- Tier: 4
- Rationale: Algebraic invariant from ARM ARM field layout. Stronger differential already used on the same domain; this pins opc/101/V/mode/L/imm7/Rt2/Rn/Rt independently of llvm-mc.
- Doc contract: load_store.rs:478 "Shift depends on register size" — other fingerprint 4a0ee4d8
- Seed: encode_ldr_str_pbt.rs ARM unsigned layout unpack
- Formal: ∀ is_load, is_64, rt1, rt2, rn ∈ 0..31, imm7 ∈ [-64,63], mode ∈ {signed=010, post=001, pre=011}. unpack(encode_ldp_stp(...)) = (opc=10 if is_64 else 00, bits[29:27]=101, V=0, bits[25:23]=mode, L=is_load, imm7[6:0], Rt2=rt2, Rn=rn, Rt=rt1)
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldp_stp
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, imm7, mode]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, imm7: i32_-64_63, mode: {0,1,2} }
  body: unpack(word).opc == (is_64?2:0) && bits29_27==0b101 && V==0 && mode_bits match && L==is_load && imm7_field==(imm7 as u7) && Rt2==rt2 && Rn==rn && Rt==rt1
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm7: { gen: int, min: -64, max: 63, type: i32 }
  mode: { gen: int, min: 0, max: 2, type: u32 }
evidence: ARM ARM C6 LDP/STP encoding; load_store.rs:490-506 word assembly
```

## encode_ldp_stp_meta_fields
- Tier: 4
- Rationale: Metamorphic isolation: incrementing Rt1/Rt2/Rn/imm7 flips only that field; load XOR store is bit 22; pre XOR post is bits[24:23] 0b10. Stronger round-trip rejected (no decoder).
- Doc contract: load_store.rs:478 "Shift depends on register size" — other fingerprint 4a0ee4d8
- Seed: encode_ldr_str_pbt.rs Rt/Rn/imm12 metamorphic
- Formal: ∀ rt1∈0..30, rt2∈0..30, rn∈0..30, imm7∈[-64,62], is_64, is_load. encode(rt1+1) − encode(rt1) = 1; encode(rt2+1) − encode(rt2) = 1<<10; encode(rn+1) − encode(rn) = 1<<5; encode(imm7+1) − encode(imm7) = 1<<15; encode(load) XOR encode(store) = 1<<22; encode(pre) XOR encode(post) = 0b10<<23
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldp_stp
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, imm7]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_30, rt2: u32_0_30, rn: u32_0_30, imm7: i32_-64_62 }
  body: word(rt1+1)-word(rt1)==1 && word(rt2+1)-word(rt2)==(1<<10) && word(rn+1)-word(rn)==(1<<5) && word(imm7+1)-word(imm7)==(1<<15) && word(load) XOR word(store)==(1<<22) && word(pre) XOR word(post)==(0b10<<23)
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 30, type: u32 }
  rt2: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  imm7: { gen: int, min: -64, max: 62, type: i32 }
evidence: ARM ARM field positions Rt[4:0] Rt2[14:10] Rn[9:5] imm7[21:15] L[22] mode[25:23]
```

## encode_ldp_stp_neg_arity
- Tier: 3
- Rationale: Negative/error contract: load_store.rs:454 returns Err when len<3; load_store.rs:511 returns Err when the third operand is not Mem/Pre/Post. llvm-mc likewise rejects those forms. This is the specified rejection, not a limitation.
- Doc contract: load_store.rs:454 "ldp/stp requires 3 operands" — asserted fingerprint 617d3353
- Seed: encode_ldr_str_pbt.rs arity / non-memory address
- Formal: ∀ is_load ∈ Bool, ops with len ∈ {0,1,2} or third operand not in {Mem, MemPreIndex, MemPostIndex}. encode_ldp_stp(ops, is_load) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldp_stp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, kind]
  domain: { is_load: bool, kind: non_mem_operand }
  relation:
    op: holds
    expr: encode_ldp_stp(too_few_or_bad_addr, is_load).is_err()
generators:
  is_load: { gen: bool }
  kind: { gen: oneof, options: [Imm, Reg, Shift, Label, Cond, Extend] }
expected_error: String
evidence: load_store.rs:453-455 arity; load_store.rs:511 unsupported operands; llvm-mc rejects non-memory addressing
```

## encode_ldp_stp_neg_extra_operand
- Tier: 3
- Rationale: llvm-mc / gas reject a fourth operand on LDP/STP. The function has no upper bound (only len<3). Contract inferred from README gas-compatibility and llvm-mc.
- Doc contract: load_store.rs:454 "ldp/stp requires 3 operands" — asserted fingerprint 617d3353
- Seed: encode_ldr_str_pbt.rs extra operand
- Formal: ∀ is_load, is_64, rt1, rt2, rn ∈ 0..31, extra ∈ Operand. encode_ldp_stp([Reg, Reg, Mem{offset:0}, extra], is_load) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: failing
- Counterexample: is_load=false, is_64=false, rt1=0, rt2=0, rn=0, extra=Reg("x0") → Ok(Word(0x29000000))
- Bug report: bug_reports/encode_ldp_stp_extra_operand.md

```property
function: encoder.load_store.encode_ldp_stp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, extra]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, extra: Operand }
  relation:
    op: holds
    expr: encode_ldp_stp([Reg, Reg, Mem, extra], is_load).is_err()
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: oneof, options: [Reg, Imm, Shift, Label, Symbol, Cond] }
expected_error: String
evidence: README.md:12 gas-compatible; llvm-mc "invalid operand" on fourth operand
```

## encode_ldp_stp_neg_invalid_regs
- Tier: 3
- Rationale: llvm-mc rejects SP as Rt, XZR/x31/W as base, mixed W/X pair, LDP with Rt1==Rt2, and writeback with Rn in {Rt1,Rt2} (Rn!=SP). No function comment declares these invalid, so they stay in the generator. Contract inferred from README gas-compatibility + ARM UNPREDICTABLE.
- Doc contract: load_store.rs:478 "Shift depends on register size" — other fingerprint 4a0ee4d8
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_neg_invalid_regs
- Formal: ∀ is_load, is_64, rt, rn∈0..30. encode(SP dest) is Err ∧ encode(XZR/x31/W base) is Err ∧ encode(mixed W/X) is Err ∧ (is_load ⇒ encode(Rt1==Rt2) is Err) ∧ encode(pre/post with Rn∈{Rt1,Rt2}, Rn≠31) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: failing
- Counterexample: is_load=false, is_64=false, rt=0, rt2=0, rn=0 → accepted SP dest, XZR/x31/W base, mixed X/W, writeback Rn==Rt1
- Bug report: bug_reports/encode_ldp_stp_invalid_regs.md

```property
function: encoder.load_store.encode_ldp_stp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt, rn]
  domain: { is_load: bool, is_64: bool, rt: u32_0_30, rn: u32_0_30 }
  body: encode(SP dest).is_err && encode(XZR base).is_err && encode(W base).is_err && encode(mixed width).is_err && (is_load => encode(rt1==rt2).is_err) && encode(writeback overlap).is_err
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
expected_error: String
evidence: llvm-mc errors (invalid operand / unpredictable LDP/STP); ARM ARM writeback and LDP Rt==Rt2 UNPREDICTABLE; README.md:12
```

## encode_ldp_stp_neg_offset_range
- Tier: 3
- Rationale: llvm-mc requires the index be a multiple of scale in [−256,252] (W) or [−512,504] (X). The SUT shifts and masks imm7 with no range or alignment check. Bound sampled at min−1, max+1, unaligned, i64::MIN/MAX.
- Doc contract: load_store.rs:478 "Shift depends on register size" — other fingerprint 4a0ee4d8
- Seed: encode_ldr_str_pbt.rs encode_ldr_str_neg_offset_extra
- Formal: ∀ is_load, is_64, rt1, rt2, rn ∈ 0..31, off ∈ {min−1, max+1, 1 (unaligned if scale>1), i64::MIN, i64::MAX} where min/max are the ARM signed-offset bounds. encode_ldp_stp([Reg, Reg, Mem/Pre/Post{off}], is_load) is Err
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: failing
- Counterexample: is_load=false, is_64=false, rt1=0, rt2=0, rn=0, which_off=0, form=0, offset=-257 → Ok(Word(689930240))
- Bug report: bug_reports/encode_ldp_stp_imm7_range.md

```property
function: encoder.load_store.encode_ldp_stp
oracle: negative_error
predicate:
  quantifier: forall
  vars: [is_load, is_64, rt1, rt2, rn, which_off, form]
  domain: { is_load: bool, is_64: bool, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, which_off: 0..4, form: 0..2 }
  relation:
    op: holds
    expr: encode_ldp_stp([Reg, Reg, mem_form(off_out_of_range)], is_load).is_err()
generators:
  is_load: { gen: bool }
  is_64: { gen: bool }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  which_off: { gen: int, min: 0, max: 4, type: u32 }
  form: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: llvm-mc "index must be a multiple of {4,8} in range [min, max]"; ARM ARM imm7 signed scaled
```

## encode_ldp_stp_diff_simd_llvm_mc
- Tier: 5
- Rationale: Sweep — documented SIMD S/D/Q pair encoding (V=1, opc 00/01/10, scale 4/8/16) vs llvm-mc. Same differential as GPR signed-offset.
- Doc contract: load_store.rs:478 "Shift depends on register size" — other fingerprint 4a0ee4d8
- Seed: encode_ldp_stp_kat_llvm_mc_simd
- Formal: ∀ is_load ∈ Bool, kind ∈ {S,D,Q}, rt1,rt2,rn ∈ 0..31, imm7 ∈ [-64,63]. (is_load ⇒ rt1 ≠ rt2) ⇒ encode_ldp_stp([Reg(kind rt1), Reg(kind rt2), Mem{Xn|SP, imm7·scale(kind)}], is_load) = llvm-mc(asm)
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldp_stp
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, kind, rt1, rt2, rn, imm7]
  domain: { is_load: bool, kind: {0,1,2}, rt1: u32_0_31, rt2: u32_0_31, rn: u32_0_31, imm7: i32_-64_63 }
  relation:
    op: eq
    lhs: encode_ldp_stp(simd_ops, is_load)
    rhs: llvm_mc(asm)
generators:
  is_load: { gen: bool }
  kind: { gen: int, min: 0, max: 2, type: u32 }
  rt1: { gen: int, min: 0, max: 31, type: u32 }
  rt2: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  imm7: { gen: int, min: -64, max: 63, type: i32 }
evidence: ARM ARM LDP/STP SIMD V=1; load_store.rs:459-474 fp opc/shift; README.md:12
```

## encode_ldp_stp_diff_alt_spellings
- Tier: 5
- Rationale: Sweep — uppercase Xn, WZR/W30/SP, lr, w31 aliases must match llvm-mc, as parse_reg_num lowercases and maps lr=30, w31=31=WZR.
- Doc contract: load_store.rs:478 "Shift depends on register size" — other fingerprint 4a0ee4d8
- Seed: encode_ldr_str_pbt alt-spellings
- Formal: ∀ is_load ∈ Bool, which ∈ {X0/X1/X2, wzr/W30/SP, x0/x1/lr, w31/w0/sp}. encode_ldp_stp(aliased ops, is_load) = llvm-mc(canonical asm)
- Test file: src/backend/arm/assembler/encoder/encode_ldp_stp_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldp_stp
oracle: differential
predicate:
  quantifier: forall
  vars: [is_load, which]
  domain: { is_load: bool, which: 0..3 }
  relation:
    op: eq
    lhs: encode_ldp_stp(alias_ops, is_load)
    rhs: llvm_mc(canonical_asm)
generators:
  is_load: { gen: bool }
  which: { gen: int, min: 0, max: 3, type: u32 }
evidence: parse_reg_num lowercase / lr=30 / wzr=31; README.md:12
```
