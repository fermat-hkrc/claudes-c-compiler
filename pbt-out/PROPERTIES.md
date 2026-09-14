# Properties: encode_neon_across_long

## encode_neon_across_long_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is differential vs llvm-mc. State machine rejected — encode_neon_across_long is a pure function with no lifecycle. Algebraic round-trip via an in-tree decoder rejected — no SADDLV/UADDLV decoder exists. Same-job sibling gate: encode_neon_across (UMAXV/UMINV/SMAXV/SMINV) and encode_neon_addv are same-width reductions with different opcodes and dest width, not SADDLV/UADDLV. README claims gas-compatible AArch64 text; llvm-mc is an independent assembler of that contract. Documented bounds sampled exactly: T in {8B,16B,4H,8H,4S}, V matching T, Rd/Rn in {0,1,30,31} plus 0..=31, U in {0,1}.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_diff_llvm_mc at neon.rs encode_neon_three_diff_narrow_pbt; codegen alu.rs:65
- Formal: ∀ rd,rn ∈ {0..31}, u ∈ {0,1}, T ∈ {8b,16b,4h,8h,4s}. let V = H if T∈{8b,16b} else S if T∈{4h,8h} else D. encode_neon_across_long([Reg(Vrd), RegArrangement(vrn, T)], u, 0b00011) = Word(w) ∧ w = llvm-mc("{saddlv|uaddlv} Vrd, vrn.T")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across_long
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, u, t]
  domain: { rd: "0..=31", rn: "0..=31", u: "0|1", t: "8b|16b|4h|8h|4s" }
  body: encode_neon_across_long(ops, u, 0b00011) == llvm_mc_word(mnemonic, Vrd, vn.T)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "4s"] }
evidence: src/backend/arm/assembler/README.md:5-14 gas-compatible AArch64; encoder/mod.rs:679-680 saddlv/uaddlv dispatch; ARM ARM SADDLV/UADDLV; llvm-mc saddlv h0,v1.8b=0x0e303820
```

## encode_neon_across_long_roundtrip_arm_fields
- Tier: 4
- Rationale: ARM ARM Advanced SIMD across-lanes layout is claimed by the body comment (0 Q U 01110 size 11000 00011 10 Rn Rd). Unpacking those fields is an independent inverse of packing, not a copy of the SUT. Stronger differential already covers llvm-mc agreement; this pins the architectural field map. No in-tree decoder for a true encode/decode round-trip.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_word_layout
- Formal: ∀ rd,rn ∈ {0..31}, u ∈ {0,1}, T ∈ {8b,16b,4h,8h,4s}. let w = encode_neon_across_long(...). unpack(w).rd = rd ∧ unpack(w).rn = rn ∧ unpack(w).U = u ∧ unpack(w).Q = q(T) ∧ unpack(w).size = size(T) ∧ bits[28:24]=01110 ∧ bits[21:17]=11000 ∧ bits[16:12]=00011 ∧ bits[11:10]=10 ∧ bit31=0
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across_long
oracle: algebraic.round_trip
predicate:
  quantifier: forall
  vars: [rd, rn, u, t]
  domain: { rd: "0..=31", rn: "0..=31", t: "8b|16b|4h|8h|4s" }
  body: unpack_across_long(word) == (rd, rn, u, q(t), size(t), opcode=0b00011)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "4s"] }
evidence: ARM ARM SADDLV/UADDLV encoding 0 Q U 01110 size 11000 00011 10 Rn Rd; neon.rs:1722 comment
```

## encode_neon_across_long_metamorphic_u_bit
- Tier: 4
- Rationale: ARM ARM documents U=0 for SADDLV and U=1 for UADDLV at bit 29, with all other fields identical for the same Rd/Rn/T. Stronger differential already covers each mnemonic independently; this metamorphic checks the documented U-bit transform without copying the SUT body.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_u_bit
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,4s}. encode_neon_across_long(ops, 0, 0b00011) XOR encode_neon_across_long(ops, 1, 0b00011) = 1<<29
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across_long
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: "0..=31", rn: "0..=31", t: "8b|16b|4h|8h|4s" }
  relation:
    op: eq
    lhs: encode_neon_across_long(ops, 0, 0b00011) ^ encode_neon_across_long(ops, 1, 0b00011)
    rhs: "1u32 << 29"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "4s"] }
evidence: ARM ARM SADDLV U=0 vs UADDLV U=1 at bit 29; encoder/mod.rs:679-680
```

## encode_neon_across_long_invariant_fixed_bits
- Tier: 4
- Rationale: ARM ARM Advanced SIMD across-lanes class fixes bit 31=0, bits [28:24]=01110, bits [21:17]=11000, bits [16:12]=00011 (SADDLV/UADDLV opcode), bits [11:10]=10 on every success-path word. Weaker than field round-trip; kept as a structural invariant over the valid domain.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_word_layout
- Formal: ∀ valid (rd,rn,u,T). let w = encode_neon_across_long(...). (w>>31)&1 = 0 ∧ (w>>24)&0x1F = 0b01110 ∧ (w>>17)&0x1F = 0b11000 ∧ (w>>12)&0x1F = 0b00011 ∧ (w>>10)&0x3 = 0b10
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across_long
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, u, t]
  domain: { rd: "0..=31", rn: "0..=31", t: "8b|16b|4h|8h|4s" }
  relation:
    op: holds
    expr: fixed_across_long_bits(encode_neon_across_long(ops, u, 0b00011))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "4s"] }
evidence: ARM ARM SADDLV/UADDLV fixed opcode bits; neon.rs:1722
```

## encode_neon_across_long_neg_arity_and_shape
- Tier: 5
- Rationale: ARM ARM / GNU as syntax is exactly two operands: scalar dest then Vn.T. llvm-mc rejects missing operands and a non-register second operand (Imm, Symbol, Mem). Documented error is rejection (Err), not a wrong word. Invalid dest/src names (h32, v32, foo, empty) also Err via parse_reg_num.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_arity_err
- Formal: ∀ ops with len<2 ∨ ops[0] not Reg/RegArrangement ∨ ops[1] ∈ {Imm, Symbol, Mem, Shift, Cond, Label} ∨ invalid dest/src names. encode_neon_across_long(ops, u, 0b00011) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_across_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [shape, u]
  domain: { shape: "empty|dest-only|non-reg second operand|invalid name" }
  relation:
    op: holds
    expr: encode_neon_across_long(ops, u, 0b00011).is_err()
generators:
  u: { gen: int, min: 0, max: 1, type: u32 }
expected_error: String
evidence: ARM ARM SADDLV syntax Vd, Vn.T; llvm-mc rejects missing operands and saddlv h0, x1
```

## encode_neon_across_long_neg_extra_operands
- Tier: 5
- Rationale: llvm-mc / GNU as reject a third operand (saddlv h0, v1.8b, v2.8b → invalid operand). README gas-compatible contract. SUT uses only operands[0] and operands[1] with `len < 2` (no exact-arity check).
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_extra_operand_err
- Formal: ∀ valid 2-operand SADDLV/UADDLV ops, extra ∈ Operand. encode_neon_across_long(ops ++ [extra], u, 0b00011) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, u=0, t="8b", extra_kind=0 — saddlv h0, v0.8b, h0 encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_across_long_extra_operand.md

```property
function: encoder.neon.encode_neon_across_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, u, t, extra]
  domain: { extra: "Reg|Imm|RegArrangement|Mem" }
  relation:
    op: holds
    expr: encode_neon_across_long(ops_plus_extra, u, 0b00011).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "4s"] }
  extra_kind: { gen: int, min: 0, max: 3, type: u32 }
expected_error: String
evidence: llvm-mc saddlv h0, v1.8b, v2.8b error invalid operand; ARM ARM two-operand syntax
```

## encode_neon_across_long_neg_invalid_arrangement
- Tier: 5
- Rationale: ARM ARM SADDLV/UADDLV T is only 8B/16B/4H/8H/4S. size=11 (1D/2D) and size=10 Q=0 (2S) are reserved/UNDEFINED. llvm-mc rejects saddlv h0, v1.2s / v1.2d / v1.1d and unknown qualifiers (8s). neon_arr_to_q_size accepts 2s/1d/2d, so the encoder may silently encode reserved forms. Documented bounds sampled exactly: 2s, 1d, 2d, and unsupported strings.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_unsupported_src
- Formal: ∀ rd,rn ∈ {0..31}, u ∈ {0,1}, T ∉ {8b,16b,4h,8h,4s}. encode_neon_across_long([Reg(h/s/d rd), RegArrangement(vrn, T)], u, 0b00011) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, u=0, t="2s" — saddlv h0, v0.2s encodes instead of Err
- Bug report: pbt-out/bug_reports/encode_neon_across_long_reserved_arrangement.md

```property
function: encoder.neon.encode_neon_across_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, u, t]
  domain: { t: "2s|1d|2d|8s|16h|4d|8B-empty|b|h" }
  relation:
    op: holds
    expr: encode_neon_across_long(ops, u, 0b00011).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  t: { gen: oneof, options: ["2s", "1d", "2d", "8s", "16h", "4d", "", "b", "h"] }
expected_error: String
evidence: ARM ARM SADDLV T in {8B,16B,4H,8H,4S} size=11 and 2S reserved; llvm-mc rejects saddlv h0, v1.2s
```

## encode_neon_across_long_neg_dest_type
- Tier: 5
- Rationale: ARM ARM dest V is H for 8B/16B, S for 4H/8H, D for 4S — a scalar SIMD register, never GPR, never Q/B, never a vector arrangement. llvm-mc rejects saddlv s0, v1.8b / h0, v1.4h / x0 / b0 / q0 / v0.8h. SUT extracts only the register number and ignores the dest prefix and arrangement, so it may silently accept them. Documented dest/T pairs sampled exactly.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_gpr_dest_err
- Formal: ∀ rn ∈ {0..31}, u ∈ {0,1}, T ∈ {8b,16b,4h,8h,4s}, dest such that dest is not the mandated V for T (wrong scalar width, x/w/q/b/vN, or RegArrangement). encode_neon_across_long([dest, RegArrangement(vrn, T)], u, 0b00011) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, u=0, t="8b", prefix="b", as_arr=false — saddlv b0, v0.8b encodes as saddlv h0, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_across_long_dest_type.md

```property
function: encoder.neon.encode_neon_across_long
oracle: negative_error
predicate:
  quantifier: forall
  vars: [dest, rn, u, t]
  domain: { dest: "wrong-V|xN|wN|qN|bN|vN|RegArrangement", t: "8b|16b|4h|8h|4s" }
  relation:
    op: holds
    expr: encode_neon_across_long(ops, u, 0b00011).is_err()
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "4s"] }
  dest_kind: { gen: int, min: 0, max: 8, type: u32 }
expected_error: String
evidence: ARM ARM SADDLV dest V is H/S/D matching T; llvm-mc rejects saddlv s0, v1.8b, saddlv x0, v1.8b, saddlv v0.8h, v1.8b
```
