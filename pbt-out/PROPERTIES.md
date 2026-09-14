# Properties: encode_neon_rbit

## encode_neon_rbit_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. README claims gas-compatible textual assembly; encoder emits 32-bit AArch64 words. State machine rejected (pure function). Round-trip rejected (no in-tree RBIT decoder). Sibling encode_rbit (bitfield.rs) rejected (copied NEON formula; independence/same-job gate).
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b}. encode_neon_rbit([Vd.T, Vn.T]) = llvm-mc("rbit Vd.T, Vn.T") as little-endian u32
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: 0..31, rn: 0..31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_rbit([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t)])
    rhs: llvm_mc("rbit v{rd}.{t}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
evidence: src/backend/arm/assembler/README.md:11-13; encoder/mod.rs:1-7; encoder/mod.rs:902-909; neon.rs:1311-1328
```

## encode_neon_rbit_meta_q_bit
- Tier: 4c
- Rationale: ARM Advanced SIMD two-misc RBIT uses Q (bit 30) for .8b vs .16b and no other field. Metamorphic over a behavior-preserving T change. Stronger differential is the primary property; this pins the Q bit in isolation.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}. encode_neon_rbit([Vd.8b, Vn.8b]) XOR encode_neon_rbit([Vd.16b, Vn.16b]) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn]
  domain: { rd: 0..31, rn: 0..31 }
  relation:
    op: eq
    lhs: encode_neon_rbit([Vd.8b, Vn.8b]) XOR encode_neon_rbit([Vd.16b, Vn.16b])
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1319-1326; ARM ARM Advanced SIMD two-register miscellaneous RBIT Q bit
```

## encode_neon_rbit_word_layout
- Tier: 4d
- Rationale: ARM two-misc RBIT layout 0 Q 1 01110 01 10000 00101 10 Rn Rd is cited on the function. Invariant over the success path; weaker than differential but localizes field-packing bugs.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b}. let Q = [T=16b]. encode_neon_rbit([Vd.T, Vn.T]) = (Q<<30) | 0x2E605800 | (rn<<5) | rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: 0..31, rn: 0..31, t: {8b,16b} }
  relation:
    op: eq
    lhs: encode_neon_rbit([Vd.T, Vn.T])
    rhs: ((t==16b) << 30) | 0x2E605800 | (rn << 5) | rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
evidence: neon.rs:1324-1327; ARM ARM Advanced SIMD two-register miscellaneous RBIT
```

## encode_neon_rbit_meta_rd_rn
- Tier: 4c
- Rationale: Rd occupies bits [4:0] and Rn bits [9:5]; incrementing one register must add 1 or 32 and leave all other bits unchanged. Metamorphic field-isolation check.
- Seed: (none)
- Formal: ∀ rd ∈ {0..30}, rn ∈ {0..30}, T ∈ {8b,16b}. encode(rd+1,rn,T) - encode(rd,rn,T) = 1 AND encode(rd,rn+1,T) - encode(rd,rn,T) = 1<<5
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: 0..30, rn: 0..30, t: {8b,16b} }
  relation:
    op: holds
    expr: encode(rd+1,rn,t) - encode(rd,rn,t) == 1 && encode(rd,rn+1,t) - encode(rd,rn,t) == 32
generators:
  rd: { gen: int, min: 0, max: 30, type: u32 }
  rn: { gen: int, min: 0, max: 30, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
evidence: neon.rs:1324-1327; ARM ARM Rd[4:0] Rn[9:5]
```

## encode_neon_rbit_neg_arity
- Tier: 4e
- Rationale: Function documents "neon rbit requires 2 operands". llvm-mc / gas RBIT vector form is binary. Fewer than 2 operands must Err. Non-register dest kinds must Err.
- Seed: (none)
- Formal: ∀ ops. |ops| < 2 ⇒ encode_neon_rbit(ops) is Err. ∀ rd,rn,T, dest ∈ {Imm, Mem, Shift, RegList, Label}. encode_neon_rbit([dest, Vn.T]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, t]
  domain: { n: 0..1, rd: 0..31, rn: 0..31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_rbit(ops_of_len_n)
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
expected_error: String
evidence: neon.rs:1313-1315; ARM ARM RBIT Vd.T, Vn.T (exactly two operands)
```

## encode_neon_rbit_neg_extra_operands
- Tier: 4e
- Rationale: llvm-mc rejects a third operand on vector RBIT. GNU gas-compatible assembler must not silently ignore extra operands. Documented bound is exactly 2.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b}, extra. llvm-mc("rbit Vd.T, Vn.T, extra") is Err ⇒ encode_neon_rbit([Vd.T, Vn.T, extra]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="8b", extra_kind=0 — rbit v0.8b, v0.8b, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_rbit_extra_operand.md

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, extra]
  domain: { rd: 0..31, rn: 0..31, t: {8b,16b}, extra: Operand }
  relation:
    op: throws
    expr: encode_neon_rbit([Vd.T, Vn.T, extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
  extra: { gen: oneof, options: ["RegArrangement", "Imm", "Reg", "Mem"] }
expected_error: String
evidence: llvm-mc rejects rbit v0.8b, v1.8b, v2.8b; README.md:11-13 gas-compatible; ARM ARM binary RBIT
```

## encode_neon_rbit_neg_bad_arrangement
- Tier: 4e
- Rationale: neon.rs and ARM ARM restrict T to .8b/.16b. llvm-mc rejects .4h/.8h/.2s/.4s/.2d/.1d. Dest T outside {8b,16b} must Err. Bounds 8b/16b sampled exactly; neighbours (4h, 8h, empty, 8s) generated.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, T ∉ {8b,16b}. encode_neon_rbit([Vd.T, Vn.T]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: 0..31, rn: 0..31, t: NEON arrangements except 8b and 16b }
  relation:
    op: throws
    expr: encode_neon_rbit([Vd.T, Vn.T])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["4h", "8h", "2s", "4s", "2d", "1d", "8s", "4b", "", "b"] }
expected_error: String
evidence: neon.rs:1319-1321; ARM ARM RBIT T is 8B or 16B; llvm-mc rejects rbit v0.4h, v1.4h
```

## encode_neon_rbit_neg_mismatch_nonreg_invalid
- Tier: 4e
- Rationale: llvm-mc rejects mismatched T. ARM ARM requires the same T on Vd and Vn. Dest T is checked; source T is discarded — this property requires both to match.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, Td,Tn ∈ arrangements. Td ≠ Tn ⇒ encode_neon_rbit([Vd.Td, Vn.Tn]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, td="8b", tn="16b" — rbit v0.8b, v0.16b
- Bug report: pbt-out/bug_reports/encode_neon_rbit_mismatch_arrangement.md

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn]
  domain: { rd: 0..31, rn: 0..31, td: {8b,16b}, tn: {8b,16b,4h,8h,2s,4s} }
  relation:
    op: throws
    expr: encode_neon_rbit([Vd.td, Vn.tn])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, options: ["8b", "16b"] }
  tn: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
expected_error: String
evidence: llvm-mc rejects rbit v0.8b, v1.16b; ARM ARM Vd.T, Vn.T same T
```

## encode_neon_rbit_neg_bare_src
- Tier: 4e
- Rationale: Strengthening after the first failing batch. Vector RBIT requires Vn.T; a bare register source is rejected by llvm-mc.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b}. encode_neon_rbit([Vd.T, Reg(Vn)]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b" — rbit v0.8b, v0
- Bug report: pbt-out/bug_reports/encode_neon_rbit_bare_src.md

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: 0..31, rn: 0..31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_rbit([Vd.T, Reg(Vn)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
expected_error: String
evidence: llvm-mc rejects rbit v0.8b, v0; ARM ARM RBIT Vd.T, Vn.T
```

## encode_neon_rbit_neg_imm_src
- Tier: 4e
- Rationale: Strengthening. Immediate in the Vn slot is not a NEON register; get_neon_reg must Err.
- Seed: (none)
- Formal: ∀ rd ∈ {0..31}, T ∈ {8b,16b}, imm ∈ ℤ. encode_neon_rbit([Vd.T, Imm(imm)]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t, imm]
  domain: { rd: 0..31, t: {8b,16b}, imm: -2..2 }
  relation:
    op: throws
    expr: encode_neon_rbit([Vd.T, Imm(imm)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
  imm: { gen: int, min: -2, max: 2, type: i64 }
expected_error: String
evidence: neon.rs:7-21 get_neon_reg expected NEON register
```

## encode_neon_rbit_neg_invalid_name
- Tier: 4e
- Rationale: Strengthening. parse_reg_num rejects v32/foo/empty/v/v99/v-1. Invalid names must Err.
- Seed: (none)
- Formal: ∀ bad ∈ {v32, foo, "", v, v99, v-1}, rn ∈ {0..31}, T ∈ {8b,16b}. encode_neon_rbit([bad.T, Vn.T]) is Err AND encode_neon_rbit([Vn.T, bad.T]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rn, t, bad]
  domain: { rn: 0..31, t: {8b,16b}, bad: {v32,foo,"",v,v99,v-1} }
  relation:
    op: throws
    expr: encode_neon_rbit([bad.T, Vn.T])
generators:
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
  bad: { gen: oneof, options: ["v32", "foo", "", "v", "v99", "v-1"] }
expected_error: String
evidence: encoder/mod.rs:131-147 parse_reg_num; neon.rs get_neon_reg invalid NEON register
```

## encode_neon_rbit_neg_bad_prefix
- Tier: 4e
- Rationale: Strengthening. README documents V registers v0-v31 for NEON arrangements. llvm-mc rejects x0.8b / d0.8b as vector RBIT operands.
- Seed: (none)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b}, p ∈ {x,w,d,s,q,h,b}. encode_neon_rbit([p{rd}.T, p{rn}.T]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", prefix="x" — rbit x0.8b, x0.8b
- Bug report: pbt-out/bug_reports/encode_neon_rbit_non_v_prefix.md

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, prefix]
  domain: { rd: 0..31, rn: 0..31, t: {8b,16b}, prefix: {x,w,d,s,q,h,b} }
  relation:
    op: throws
    expr: encode_neon_rbit([prefix{rd}.T, prefix{rn}.T])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
  prefix: { gen: oneof, options: ["x", "w", "d", "s", "q", "h", "b"] }
expected_error: String
evidence: README.md:275-277 v0-v31; llvm-mc rejects rbit x0.8b, x1.8b
```

## encode_neon_rbit_neg_sp
- Tier: 4e
- Rationale: Strengthening. SP is not a NEON register. llvm-mc rejects sp.8b. parse_reg_num maps sp to 31 (V31).
- Seed: (none)
- Formal: ∀ rd ∈ {0..31}, T ∈ {8b,16b}. encode_neon_rbit([sp.T, Vd.T]) is Err AND encode_neon_rbit([Vd.T, sp.T]) is Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, t="8b", which=0 — rbit sp.8b, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_rbit_sp_as_neon.md

```property
function: encoder.neon.encode_neon_rbit
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, t]
  domain: { rd: 0..31, t: {8b,16b} }
  relation:
    op: throws
    expr: encode_neon_rbit([sp.T, Vd.T])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b"] }
expected_error: String
evidence: llvm-mc rejects rbit sp.8b, v0.8b; README.md:275-277 v0-v31
```
