# Properties: encode_neon_float_cmp_zero

## encode_neon_float_cmp_zero_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (independent AArch64 assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree FCMEQ-zero decoder. encode_neon_cmp_zero / encode_neon_float_two_misc fail the same-job sibling gate (integer compare-zero vs FP two-misc). SUT-boundary: internal-helper of the GNU-style assembler (README gas-compatible). Mapping: [RegArrangement(Vd,T), RegArrangement(Vn,T)] + ARM-correct (U, size_hi=1, opcode) <-> `{fcmeq|fcmge|fcmgt|fcmle|fcmlt} Vd.T, Vn.T, #0.0`. Doc evidence: README.md:5-14, encoder/mod.rs:1-7, neon.rs:1573-1574, ARM ARM SIMD two-misc FP compare-zero (size=1sz).
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {2s,4s,2d}, (U,opcode) ∈ {(0,0b01101),(1,0b01100),(0,0b01100),(1,0b01101),(0,0b01110)}. encode_neon_float_cmp_zero([Vd.T, Vn.T], U, 1, opcode) = llvm-mc("{mnem} Vd.T, Vn.T, #0.0")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, opcode]
  domain: { rd: v0..v31, rn: v0..v31, t: {2s,4s,2d}, (u,opcode): ARM FCM* zero table }
  relation:
    op: eq
    lhs: encode_neon_float_cmp_zero([Vd.t, Vn.t], u, 1, opcode)
    rhs: llvm_mc("{mnem} Vd.t, Vn.t, #0.0")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:1-7; neon.rs:1573-1588
```

## encode_neon_float_cmp_zero_roundtrip_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM two-register-misc layout claimed at neon.rs:1574. Differential is stronger and used above; this unpacks Q/U/size/opcode/Rn/Rd so a packing slip still fails even if llvm-mc were unavailable. Stronger round-trip rejected: no decoder.
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_roundtrip_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {2s,4s,2d}, U ∈ {0,1}, size_hi ∈ {0,1}, opcode ∈ {0b01100,0b01101,0b01110}. let w = encode_neon_float_cmp_zero([Vd.T,Vn.T],U,size_hi,opcode). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=U ∧ w[28:24]=01110 ∧ w[23:22]=(size_hi<<1)|sz(T) ∧ w[21:17]=10000 ∧ w[16:12]=opcode ∧ w[11:10]=10 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, size_hi, opcode]
  domain: { t: "2s|4s|2d" }
  relation:
    op: holds
    expr: unpack(encode_neon_float_cmp_zero([Vd.t, Vn.t], u, size_hi, opcode)) matches ARM two-misc FP-cmp-zero layout
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1574 Format 0 Q U 01110 size 10000 opcode 10 Rn Rd
```

## encode_neon_float_cmp_zero_metamorphic_u_bit
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM U is bit 29 and is the only bit that distinguishes FCMEQ vs FCMLE (opcode 01101) and FCMGT vs FCMGE (opcode 01100). Stronger differential covers absolute encoding; this isolates the U toggle. State machine / round-trip rejected as above.
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_metamorphic_u_bit
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {2s,4s,2d}, size_hi ∈ {0,1}, opcode ∈ {0b01100,0b01101,0b01110}. encode_neon_float_cmp_zero(ops,0,size_hi,opcode) XOR encode_neon_float_cmp_zero(ops,1,size_hi,opcode) = 1<<29
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, size_hi, opcode]
  relation:
    op: eq
    lhs: encode(ops, 0, size_hi, opcode) XOR encode(ops, 1, size_hi, opcode)
    rhs: 1 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1574 U at bit 29; encoder/mod.rs:478-494 U distinguishes FCMEQ/FCMGT vs FCMGE/FCMLE
```

## encode_neon_float_cmp_zero_metamorphic_size_hi
- Tier: 4
- Rationale: Algebraic metamorphic: size = (size_hi << 1) | sz so toggling size_hi toggles only bit 23 (ARM size[1]). Documented at neon.rs:1574 "(float, size = 0sz)" and the size packing at neon.rs:1584. Bounds size_hi ∈ {0,1} sampled exactly.
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_metamorphic_u_bit
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {2s,4s,2d}, U ∈ {0,1}, opcode ∈ {0b01100,0b01101,0b01110}. encode_neon_float_cmp_zero(ops,U,0,opcode) XOR encode_neon_float_cmp_zero(ops,U,1,opcode) = 1<<23
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, opcode]
  relation:
    op: eq
    lhs: encode(ops, u, 0, opcode) XOR encode(ops, u, 1, opcode)
    rhs: 1 << 23
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1584 size = (size_hi << 1) | sz; ARM ARM size[1]=1 for FP compare-zero
```

## encode_neon_float_cmp_zero_neg_unsupported_arrangement
- Tier: 4
- Rationale: Negative/error contract. neon.rs:1579-1581 returns Err for arrangement other than 2s/4s/2d. llvm-mc rejects 8b/16b/4h/8h/1d/1s/3s/empty (fullfp16 4h/8h is a different encoding, not this helper). Documented bound T ∈ {2s,4s,2d} sampled with bound±1 neighbours (8b,4h,1d,8h,16b).
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_neg_invalid_arrangement
- Formal: ∀ rd,rn ∈ {0..31}, T ∉ {2s,4s,2d}, U ∈ {0,1}, size_hi ∈ {0,1}, opcode ∈ {0b01100,0b01101,0b01110}. encode_neon_float_cmp_zero([Vd.T, Vn.T], U, size_hi, opcode) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, size_hi, opcode]
  domain: { t: "not 2s|4s|2d" }
  relation:
    op: throws
    expr: encode_neon_float_cmp_zero([Vd.t, Vn.t], u, size_hi, opcode)
expected_error: String
generators:
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "1d", "1s", "3s", "8s", "", "b", "h"] }
evidence: neon.rs:1579-1581 match 2s/4s/2d else Err; llvm-mc rejects those T
```

## encode_neon_float_cmp_zero_neg_extra_operands
- Tier: 4
- Rationale: Negative/error contract from the gas-compatible assembler contract (README.md:5-14). llvm-mc rejects a third operand on FCM* #0.0. Integer sibling encode_neon_cmp_zero checks len < 2. This helper has no arity check; extra operands must still Err under the public assembler contract. Stronger differential does not apply to invalid assembly.
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_neg_extra_operands
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ {2s,4s,2d}, U ∈ {0,1}. llvm-mc("{mnem} Vd.T, Vn.T, #0.0, extra") is Err ∧ encode_neon_float_cmp_zero([Vd.T, Vn.T, Imm(0), extra], U, 1, opcode) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="2s", insn=(0, 0b01101, "fcmeq"), extra_kind=0 — fcmeq v0.2s, v0.2s, #0.0, v0.2s
- Bug report: pbt-out/bug_reports/encode_neon_float_cmp_zero_extra_operand.md

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, u]
  relation:
    op: throws
    expr: encode_neon_float_cmp_zero([Vd.t, Vn.t, extra], u, 1, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: README.md:5-14 gas-compatible; llvm-mc rejects extra operand on fcmeq v0.4s, v1.4s, #0.0, v2.4s
```

## encode_neon_float_cmp_zero_neg_arity_and_shape
- Tier: 4
- Rationale: Negative/error contract. get_neon_reg Err on missing operand / non-register / parse_reg_num None (v32, foo, empty). llvm-mc rejects v32 and too-few operands. Documented register bound 0..31 sampled at 32 and malformed names.
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_neg_arity_and_shape
- Formal: ∀ n < 2, ∀ non-Reg/RegArrangement operand, ∀ invalid name ∈ {v32, v, foo, "", v-1, v99}. encode_neon_float_cmp_zero(ops, U, 1, 0b01101) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, which, bad]
  relation:
    op: throws
    expr: encode_neon_float_cmp_zero(malformed, u, 1, 0b01101)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  bad: { gen: oneof, items: ["v32", "foo", "", "v", "v-1", "v99"] }
evidence: neon.rs:7-21 get_neon_reg; parse_reg_num None for num>31 / unknown prefix; llvm-mc rejects v32 and too-few operands
```

## encode_neon_float_cmp_zero_neg_arrangement_mismatch
- Tier: 4
- Rationale: Negative/error contract. ARM ARM and llvm-mc require dest T == src T (`fcmeq v0.4s, v1.2s, #0.0` is invalid). The helper reads only dest arrangement (neon.rs:1578-1580 ignores src arr). Under the gas-compatible contract, mismatched T must Err, not encode as dest T. Stronger differential does not apply to invalid assembly.
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_neg_dest_type
- Formal: ∀ rd,rn ∈ {0..31}, Td ≠ Tn, Td,Tn ∈ {2s,4s,2d}, U ∈ {0,1}. llvm-mc("{mnem} Vd.Td, Vn.Tn, #0.0") is Err ∧ encode_neon_float_cmp_zero([Vd.Td, Vn.Tn], U, 1, opcode) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, td="4s", tn="2s", insn=(0, 0b01101, "fcmeq") — fcmeq v0.4s, v0.2s, #0.0
- Bug report: pbt-out/bug_reports/encode_neon_float_cmp_zero_arrangement_mismatch.md

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, u]
  domain: { td: "2s|4s|2d", tn: "2s|4s|2d", distinct: true }
  relation:
    op: throws
    expr: encode_neon_float_cmp_zero([Vd.td, Vn.tn], u, 1, opcode)
expected_error: String
generators:
  td: { gen: oneof, items: ["2s", "4s", "2d"] }
  tn: { gen: oneof, items: ["2s", "4s", "2d"] }
evidence: ARM ARM FCMEQ <Vd>.<T>, <Vn>.<T>, #0.0 same T; llvm-mc rejects fcmeq v0.4s, v1.2s, #0.0
```

## encode_neon_float_cmp_zero_neg_non_v_prefix
- Tier: 4
- Rationale: Coverage-sweep negative/error contract. llvm-mc requires a V register with arrangement (`v0.4s`); parse_reg_num accepts x/w/d/s/q/h/b prefixes with num<=31, so a non-v name would encode as the matching-number V register unless rejected. Gas-compatible assembler contract (README.md:5-14). Stronger differential does not apply to invalid assembly.
- Seed: encode_neon_across_long_pbt::encode_neon_across_long_neg_dest_type
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {2s,4s,2d}, prefix ∈ {x,w,d,s,q,h,b}. llvm-mc("{mnem} {prefix}{rd}.T, Vn.T, #0.0") is Err ∧ encode_neon_float_cmp_zero([RegArrangement({prefix}{rd}, T), Vn.T], U, 1, opcode) is Err (and symmetrically for src)
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="2s", insn=(0, 0b01101, "fcmeq"), prefix="x", which=0 — fcmeq x0.2s, v0.2s, #0.0
- Bug report: pbt-out/bug_reports/encode_neon_float_cmp_zero_non_v_prefix.md

```property
function: encoder.neon.encode_neon_float_cmp_zero
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, prefix, which]
  relation:
    op: throws
    expr: encode_neon_float_cmp_zero(ops_with_non_v_prefix, u, 1, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  prefix: { gen: oneof, items: ["x", "w", "d", "s", "q", "h", "b"] }
evidence: README.md:5-14 gas-compatible; llvm-mc rejects fcmeq x0.4s, v1.4s, #0.0; parse_reg_num encoder/mod.rs:131-148
```
