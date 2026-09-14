# Properties: encode_ldar_stlr

## encode_ldar_stlr_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc. State machine rejected — encode_ldar_stlr is a pure function with no lifecycle. Algebraic round-trip via an in-tree decoder rejected — no LDAR/STLR decoder exists. Same-job sibling gate: encode_ldaxr_stlxr / encode_ldxr_stxr are exclusive forms (Rs/Rt2/o0 differ), not LDAR/STLR. README claims gas-compatible AArch64 text; llvm-mc is an independent assembler of that contract. Documented bounds sampled exactly: size in {00,01,10,11}, Rt/Rn in {0,1,30,31}, offset #0.
- Seed: encode_adr_pbt::encode_adr_diff_imm_llvm_mc at load_store.rs encode_adr_pbt; codegen atomics.rs:93-128
- Formal: ∀ rt,rn ∈ {0..31}, is_load ∈ Bool, variant ∈ {word, byte, half}, is_64 ∈ Bool (word only). let rt_name = gpr_data(is_64, rt) using xzr/wzr for 31 (byte/half force 32-bit Wt). let rn_name = sp if rn=31 else xN (never xzr). encode_ldar_stlr([Reg(rt_name), Mem(rn_name, 0)], is_load, forced_size(variant)) = Word(w) ∧ w = llvm-mc("{ldar|stlr|ldarb|stlrb|ldarh|stlrh} rt_name, [rn_name]")
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldar_stlr
oracle: differential
predicate:
  quantifier: forall
  vars: [rt, rn, is_load, variant, is_64]
  domain: { rt: "0..=31", rn: "0..=31 meaning Xn|SP", variant: "word|byte|half", is_64: "word only" }
  body: encode_ldar_stlr(ops, is_load, forced_size) == llvm_mc_word(mnemonic, rt_name, rn_name)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:360-365 ldar/stlr dispatch; ARM ARM LDAR/STLR; llvm-mc ldar x0,[x1]=0xc8dffc20
```

## encode_ldar_stlr_roundtrip_arm_fields
- Tier: 4
- Rationale: ARM ARM Load/Store Exclusive/Acquire/Release layout is claimed by the body comment (size 001000 1 L 0 11111 1 11111 Rn Rt). Unpacking those fields is an independent inverse of packing, not a copy of the SUT. Stronger differential already covers llvm-mc agreement; this pins the architectural field map. No in-tree decoder for a true encode/decode round-trip.
- Seed: encode_adr_pbt::encode_adr_roundtrip_arm_fields
- Formal: ∀ rt,rn ∈ {0..31}, is_load ∈ Bool, variant ∈ {word, byte, half}, is_64 ∈ Bool (word only). let w = encode_ldar_stlr(...). unpack(w).rt = rt ∧ unpack(w).rn = rn ∧ unpack(w).L = is_load ∧ unpack(w).size = expected_size ∧ unpack(w).Rs = 31 ∧ unpack(w).o0 = 1 ∧ unpack(w).Rt2 = 31 ∧ bits[29:24]=001000 ∧ bit23=1 ∧ bit21=0
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldar_stlr
oracle: algebraic.round_trip
predicate:
  quantifier: forall
  vars: [rt, rn, is_load, variant, is_64]
  domain: { rt: "0..=31", rn: "0..=31", variant: "word|byte|half" }
  body: unpack_ldar_stlr(word) == (rt, rn, is_load, expected_size, 31, 1, 31)
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM LDAR/STLR encoding size 001000 1 L 0 11111 1 11111 Rn Rt; load_store.rs:645 comment
```

## encode_ldar_stlr_metamorphic_l_bit
- Tier: 4
- Rationale: ARM ARM documents L=1 for LDAR/LDARB/LDARH and L=0 for STLR/STLRB/STLRH at bit 22, with all other fields identical for the same Rt/Rn/size. Stronger differential already covers each mnemonic independently; this metamorphic checks the documented L-bit transform without copying the SUT body.
- Seed: encode_eon_pbt::encode_eon_metamorphic_n_bit_vs_eor
- Formal: ∀ rt,rn ∈ {0..31}, variant, is_64. encode_ldar_stlr(ops, true, sz) XOR encode_ldar_stlr(ops, false, sz) = 1<<22
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldar_stlr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rt, rn, variant, is_64]
  domain: { rt: "0..=31", rn: "0..=31" }
  relation:
    op: eq
    lhs: encode_ldar_stlr(ops, true, sz) ^ encode_ldar_stlr(ops, false, sz)
    rhs: "1u32 << 22"
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM LDAR L=1 vs STLR L=0 at bit 22; load_store.rs:644-647
```

## encode_ldar_stlr_invariant_fixed_bits
- Tier: 4
- Rationale: ARM ARM Load-Acquire/Store-Release Register class fixes bits [29:24]=001000, bit 23=1 (not exclusive), bit 21=0, Rs=11111, o0=1, Rt2=11111 on every success-path word. Weaker than field round-trip; kept as a structural invariant over the valid domain including offset #0.
- Seed: encode_adr_pbt fixed-opcode checks
- Formal: ∀ valid (rt,rn,is_load,variant,is_64). let w = encode_ldar_stlr(...). (w>>24)&0x3F = 0b001000 ∧ (w>>23)&1 = 1 ∧ (w>>21)&1 = 0 ∧ (w>>16)&0x1F = 31 ∧ (w>>15)&1 = 1 ∧ (w>>10)&0x1F = 31
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldar_stlr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rt, rn, is_load, variant, is_64]
  domain: { rt: "0..=31", rn: "0..=31" }
  relation:
    op: holds
    expr: fixed_ldar_stlr_bits(encode_ldar_stlr(ops, is_load, sz))
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  is_64: { gen: bool }
evidence: ARM ARM LDAR/STLR fixed opcode bits; load_store.rs:645
```

## encode_ldar_stlr_neg_arity_and_shape
- Tier: 5
- Rationale: ARM ARM / GNU as syntax is exactly two operands: Rt then [Xn|SP]. llvm-mc rejects missing operands and a non-memory second operand (Imm, Symbol, pre/post-index, register-offset). Documented error is rejection (Err), not a wrong word.
- Seed: encode_adr_pbt negative arity tests
- Formal: ∀ ops with len<2 ∨ ops[0] not Reg ∨ ops[1] ∈ {Imm, Symbol, MemPreIndex, MemPostIndex, MemRegOffset, Reg} ∨ invalid base name. encode_ldar_stlr(ops, is_load, sz) is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.load_store.encode_ldar_stlr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [shape, is_load, variant]
  domain: { shape: "empty|rt-only|non-mem second operand" }
  relation:
    op: holds
    expr: encode_ldar_stlr(ops, is_load, sz).is_err()
generators:
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: ARM ARM LDAR syntax Rt, [Xn|SP]; llvm-mc rejects pre/post-index and extra/missing operands
```

## encode_ldar_stlr_neg_extra_operands
- Tier: 5
- Rationale: llvm-mc / GNU as reject a third operand (ldar x0, [x1], x2 → invalid operand). README gas-compatible contract. SUT uses only operands[0] and operands[1] with no length check.
- Seed: encode_eon_pbt extra-operand negative
- Formal: ∀ valid 2-operand LDAR/STLR ops, extra ∈ Operand. encode_ldar_stlr(ops ++ [extra], is_load, sz) is Err
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, rn=0, is_load=false, variant=0, is_64=false, extra=Reg("x2") — stlr w0, [x0], x2 encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_ldar_stlr_extra_operand.md

```property
function: encoder.load_store.encode_ldar_stlr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, rn, is_load, variant, extra]
  domain: { extra: "Reg|Imm|Mem|Symbol" }
  relation:
    op: holds
    expr: encode_ldar_stlr(ops_plus_extra, is_load, sz).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  extra_kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc ldar x0, [x1], x2 error invalid operand; ARM ARM two-operand syntax
```

## encode_ldar_stlr_neg_invalid_rt
- Tier: 5
- Rationale: ARM ARM Rt is Wt/Xt (31=WZR/XZR), never SP/WSP, never FP/SIMD. Byte/halfword forms take Wt only (llvm-mc rejects ldarb x0, [x1]). README gas-compatible contract. parse_reg_num maps sp/wsp and d/s/q/v/h/b prefixes onto GPR numbers, so the encoder may silently accept them.
- Seed: encode_eon_pbt SP/FP negatives
- Formal: ∀ is_load, variant. encode_ldar_stlr([Reg(bad_rt), Mem(x0,0)], is_load, sz) is Err where bad_rt ∈ {sp, wsp} ∪ FP {d,s,q,v,h,b}0 ∪ (Xt when variant ∈ {byte,half})
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: is_load=false, kind=0, n=0 — stlr sp, [x0] encodes as stlr xzr, [x0]
- Bug report: pbt-out/bug_reports/encode_ldar_stlr_sp_as_rt.md

```property
function: encoder.load_store.encode_ldar_stlr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [bad_rt, is_load, variant]
  domain: { bad_rt: "sp|wsp|d0|s0|q0|v0|h0|b0|xN-when-byte-half" }
  relation:
    op: holds
    expr: encode_ldar_stlr(ops, is_load, sz).is_err()
generators:
  bad_kind: { gen: int, min: 0, max: 8, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
expected_error: String
evidence: ARM ARM LDAR Rt is Wt/Xt not SP; llvm-mc rejects ldar sp [x0], ldar d0 [x1], ldarb x0 [x1]
```

## encode_ldar_stlr_neg_invalid_base_offset
- Tier: 5
- Rationale: ARM ARM Rn is Xn|SP; offset absent or #0. llvm-mc rejects W/WSP/XZR/WZR as base (ldar x0, [w1] / [xzr]) and nonzero offset (index must be absent or #0). SUT takes Mem { base, .. } and ignores offset, and parse_reg_num does not check base width or SP-vs-ZR.
- Seed: llvm-mc ldar x0, [x1, #8] error; ARM ARM {,#0}
- Formal: ∀ is_load, variant, rt valid. encode_ldar_stlr([Reg(rt), Mem(bad_base, off)], ...) is Err when bad_base ∈ {wN, wsp, wzr, xzr} ∨ off ≠ 0. Bounds off ∈ {0±1, 8, i64::MIN, i64::MAX} sampled exactly.
- Test file: src/backend/arm/assembler/encoder/load_store.rs
- Status: failing
- Counterexample: rt=0, is_load=false, variant=0, is_64=false, kind=0, wn=0 — stlr w0, [w0] encodes as stlr w0, [x0]
- Bug report: pbt-out/bug_reports/encode_ldar_stlr_w_base.md

```property
function: encoder.load_store.encode_ldar_stlr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rt, is_load, variant, bad_base, offset]
  domain: { bad_base: "wN|wsp|wzr|xzr", offset: "nonzero or combined with valid base" }
  relation:
    op: holds
    expr: encode_ldar_stlr(ops, is_load, sz).is_err()
generators:
  rt: { gen: int, min: 0, max: 31, type: u32 }
  is_load: { gen: bool }
  variant: { gen: int, min: 0, max: 2, type: u32 }
  base_kind: { gen: int, min: 0, max: 5, type: u32 }
  offset: { gen: int, min: -8, max: 8, type: i64 }
expected_error: String
evidence: ARM ARM Rn is Xn|SP offset {,#0}; llvm-mc ldar x0 [x1, #8] index must be absent or #0; llvm-mc rejects [w1]/[xzr]/[wzr]/[wsp]
```
