# Properties: encode_neon_addv

## encode_neon_addv_diff_llvm_mc
- Tier: 3
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. README.md:12 claims gas-compatible textual assembly; ADDV is listed in README.md:232 NEON reduce. llvm-mc is an independent AArch64 assembler. State machine rejected (pure function). Round-trip rejected (no in-tree ADDV decoder). Sibling encode_neon_across / encode_neon_across_long rejected (same-job gate: different opcodes/mnemonics; across docstring lists UMAXV/UMINV/SMAXV/SMINV only).
- Doc contract: neon.rs:423 "Encode NEON ADDV: add across vector lanes" — asserted fingerprint 5211ce8d
- Seed: encode_cnt_pbt.rs:encode_cnt_diff_llvm_mc; neon.rs:190-192 `addv b0, v0.8b`
- Formal: ∀ rd,rn ∈ {0..31}, (v,t) ∈ {(b,8b),(b,16b),(h,4h),(h,8h),(s,4s)}. encode_neon_addv([Reg(v{rd}), Vn.t]) = llvm-mc("addv v{rd}, v{rn}.t")
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, v=b, t=8b; SUT 0x0e30dc00 vs llvm-mc 0x0e31b800
- Bug report: pbt-out/bug_reports/encode_neon_addv_wrong_encoding.md

```property
function: encoder.encode_neon_addv
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, v, t]
  domain:
    rd: vreg 0..31
    rn: vreg 0..31
    v: {b, h, s}
    t: {8b, 16b, 4h, 8h, 4s}
  relation:
    op: eq
    lhs: encode_neon_addv([Reg(v{rd}), arr(rn,t)])
    rhs: llvm_mc("addv {v}{rd}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  vt: { gen: oneof, items: [["b","8b"],["b","16b"],["h","4h"],["h","8h"],["s","4s"]] }
evidence: neon.rs:423
```

## encode_neon_addv_meta_rd_rn
- Tier: 4
- Rationale: Metamorphic field isolation — changing only Rd/Rn must differ only in bits[4:0]/[9:5]. Independent of llvm-mc; grounded in ARM across-lanes layout cited at neon.rs:433.
- Doc contract: neon.rs:433 "ADDV: 0 Q 0 01110 size 11000 11011 10 Rn Rd" — asserted fingerprint 1abd18ff
- Seed: encode_cnt_pbt.rs:encode_cnt_meta_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, (v,t) ∈ ADDV_T. (w(rd1,rn1,t) ⊕ w(rd2,rn1,t)) ∧ ¬0x1F = 0 ∧ w.Rd = rd; (w(rd1,rn1,t) ⊕ w(rd1,rn2,t)) ∧ ¬(0x1F≪5) = 0 ∧ w.Rn = rn
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_addv
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, v, t]
  domain:
    rd1: vreg 0..31
    rd2: vreg 0..31
    rn1: vreg 0..31
    rn2: vreg 0..31
    v: "{b,h,s}"
    t: ADDV_T
  relation:
    op: holds
    expr: "(w11^w21)&!0x1F==0 && (w11^w12)&!(0x1F<<5)==0"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  vt: { gen: oneof, items: [["b","8b"],["b","16b"],["h","4h"],["h","8h"],["s","4s"]] }
evidence: neon.rs:433
```

## encode_neon_addv_inv_layout
- Tier: 4
- Rationale: ARM Advanced SIMD across-lanes ADDV encoding is an independent reference: 0 Q 0 01110 size 11000 11011 10 Rn Rd = 0x0e31b800 | (Q<<30) | (size<<22) | (Rn<<5) | Rd. Q/size from T per ARM (8B=Q0/sz00, 16B=Q1/sz00, 4H=Q0/sz01, 8H=Q1/sz01, 4S=Q1/sz10). Encoding comment neon.rs:433 states the same layout. Not copied from the producing `0b110111 << 10` statement.
- Doc contract: neon.rs:433 "ADDV: 0 Q 0 01110 size 11000 11011 10 Rn Rd" — asserted fingerprint 1abd18ff
- Seed: encode_cnt_pbt.rs:encode_cnt_inv_layout
- Formal: ∀ rd,rn ∈ {0..31}, (v,t) ∈ ADDV_T. encode_neon_addv([Reg(v{rd}), Vn.t]) = (Q(t)<<30) | (0b001110<<24) | (size(t)<<22) | (0b11000<<17) | (0b11011<<12) | (0b10<<10) | (rn≪5) | rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: retired
- Counterexample: rd=0, rn=0, v=b, t=8b; SUT 0x0e30dc00 vs ARM 0x0e31b800 (same witness as encode_neon_addv_diff_llvm_mc / B1)
- Bug report: pbt-out/bug_reports/encode_neon_addv_wrong_encoding.md
- Retired reason: same opcode-bit defect as encode_neon_addv_diff_llvm_mc (B1); ARM-layout oracle independently confirmed the same counterexample. The test remains in encode_neon_addv_pbt.rs as a failing witness.

```property
function: encoder.encode_neon_addv
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, v, t]
  domain:
    rd: vreg 0..31
    rn: vreg 0..31
    v: "{b,h,s}"
    t: ADDV_T
  relation:
    op: eq
    lhs: encode_neon_addv([Reg(v{rd}), arr(rn,t)])
    rhs: arm_addv_word(rd, rn, t)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  vt: { gen: oneof, items: [["b","8b"],["b","16b"],["h","4h"],["h","8h"],["s","4s"]] }
evidence: neon.rs:433
```

## encode_neon_addv_neg_arity
- Tier: 4
- Rationale: Documented arity contract neon.rs:426 "addv requires 2 operands" — fewer than 2 operands must Err. llvm-mc also rejects a missing operand.
- Doc contract: neon.rs:426 "addv requires 2 operands" — domain-restriction fingerprint cf2c7ab8
- Seed: encode_cnt_pbt.rs:encode_cnt_neg_arity
- Formal: ∀ n ∈ {0,1}, ops with |ops|=n. encode_neon_addv(ops) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_addv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, v, t]
  domain:
    n: "{0,1}"
    rd: vreg 0..31
    v: "{b,h,s}"
    t: ADDV_T
  relation:
    op: throws
    expr: encode_neon_addv(ops_of_len(n))
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  vt: { gen: oneof, items: [["b","8b"],["b","16b"],["h","4h"],["h","8h"],["s","4s"]] }
expected_error: String
evidence: neon.rs:426
```

## encode_neon_addv_neg_extra
- Tier: 4
- Rationale: README.md:12 gas-compatible assembly; llvm-mc rejects a third operand on addv. "addv requires 2 operands" (neon.rs:426) is the arity contract. Extra operands must Err, not be silently ignored (`len < 2`).
- Doc contract: neon.rs:426 "addv requires 2 operands" — domain-restriction fingerprint cf2c7ab8
- Seed: encode_cnt_pbt.rs:encode_cnt_neg_extra
- Formal: ∀ rd,rn,extra ∈ {0..31}, (v,t) ∈ ADDV_T. llvm-mc("addv v{rd}, v{rn}.t, v{extra}.t") is Err ∧ encode_neon_addv([dest, src, extra]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, v=b, t=8b; encode_neon_addv([b0, v0.8b, v0.8b]) = Ok(Word(0x0e30dc00))
- Bug report: pbt-out/bug_reports/encode_neon_addv_extra_operand.md

```property
function: encoder.encode_neon_addv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, v, t]
  domain:
    rd: vreg 0..31
    rn: vreg 0..31
    extra: vreg 0..31
    v: "{b,h,s}"
    t: ADDV_T
  relation:
    op: throws
    expr: encode_neon_addv([dest, src, extra])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  vt: { gen: oneof, items: [["b","8b"],["b","16b"],["h","4h"],["h","8h"],["s","4s"]] }
expected_error: String
evidence: neon.rs:426
```

## encode_neon_addv_neg_invalid_t
- Tier: 4
- Rationale: ARM ADDV T is {8B,16B,4H,8H,4S}; size:Q = 10:0 (2S) and size=11 (1D/2D) are reserved. llvm-mc rejects those arrangements. Documented bound sampled at 2s (the reserved encoding) and at 1d/2d.
- Doc contract: neon.rs:423 "Encode NEON ADDV: add across vector lanes" — asserted fingerprint 5211ce8d
- Seed: encode_cnt_pbt.rs:encode_cnt_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, t ∈ {2s,1d,2d,4b,8d,2h,1s}. llvm-mc rejects addv with Vn.t ∧ encode_neon_addv([scalar, Vn.t]) is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, t=2s; encode_neon_addv([s0, v0.2s]) = Ok(Word(0x0eb0dc00))
- Bug report: pbt-out/bug_reports/encode_neon_addv_reserved_t.md

```property
function: encoder.encode_neon_addv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain:
    rd: vreg 0..31
    rn: vreg 0..31
    t: "{2s,1d,2d,4b,8d,2h,1s}"
  relation:
    op: throws
    expr: encode_neon_addv([Reg(b{rd}), arr(rn,t)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["2s", "1d", "2d", "4b", "8d", "2h", "1s"] }
expected_error: String
evidence: neon.rs:423
```

## encode_neon_addv_neg_dest
- Tier: 4
- Rationale: ARM ADDV dest width must match T (B for 8B/16B, H for 4H/8H, S for 4S). llvm-mc rejects GPR dest, SP, arrangement dest, and mismatched scalar width. Codegen/intrinsics.rs:190 emits scalar SIMD dest. SUT discards dest arrangement/type (`let (rd, _)`).
- Doc contract: neon.rs:423 "Encode NEON ADDV: add across vector lanes" — asserted fingerprint 5211ce8d
- Seed: neon.rs encode_neon_across_long_neg_dest_type; encode_cnt_pbt.rs:encode_cnt_neg_gpr_bare_sp
- Formal: ∀ rd,rn ∈ {0..31}, (v,t) ∈ ADDV_T, dest ∈ {GPR, SP, Vn.t arrangement, mismatched scalar}. llvm-mc rejects ∧ encode_neon_addv is Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, v=b, t=8b, kind=0; encode_neon_addv([x0, v0.8b]) = Ok(Word(0x0e30dc00))
- Bug report: pbt-out/bug_reports/encode_neon_addv_invalid_dest.md

```property
function: encoder.encode_neon_addv
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, v, t, kind]
  domain:
    rd: vreg 0..31
    rn: vreg 0..31
    v: "{b,h,s}"
    t: ADDV_T
    kind: "{gpr_x, gpr_w, sp, arr_dest, mismatch_scalar}"
  relation:
    op: throws
    expr: encode_neon_addv([bad_dest, arr(rn,t)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  vt: { gen: oneof, items: [["b","8b"],["b","16b"],["h","4h"],["h","8h"],["s","4s"]] }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:423
```

## encode_neon_addv_diff_alt_spellings
- Tier: 3
- Rationale: Differential vs llvm-mc on uppercase register prefixes (B0 / V1.8B). parse_reg_num lowercases; README gas-compatibility includes case-insensitive assembly that llvm-mc accepts.
- Doc contract: neon.rs:423 "Encode NEON ADDV: add across vector lanes" — asserted fingerprint 5211ce8d
- Seed: encode_cnt_pbt.rs:encode_cnt_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, (v,t) ∈ ADDV_T. encode_neon_addv([Reg(V{rd}), Vn.T uppercase]) = llvm-mc("addv V{rd}, V{rn}.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_addv_pbt.rs
- Status: retired
- Counterexample: rd=0, rn=0, v=b, t=8b; SUT 0x0e30dc00 vs llvm-mc 0x0e31b800 (same witness as encode_neon_addv_diff_llvm_mc / B1; uppercase prefixes parse)
- Bug report: pbt-out/bug_reports/encode_neon_addv_wrong_encoding.md
- Retired reason: uppercase V/B prefixes parse via parse_reg_num; the llvm-mc disagreement is the same B1 encoding defect. The test remains in encode_neon_addv_pbt.rs as a failing witness.

```property
function: encoder.encode_neon_addv
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, v, t]
  domain:
    rd: vreg 0..31
    rn: vreg 0..31
    v: "{b,h,s}"
    t: ADDV_T
  relation:
    op: eq
    lhs: encode_neon_addv([Reg(upper v{rd}), arr_upper(rn,t)])
    rhs: llvm_mc("addv {V}{rd}, V{rn}.{T}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  vt: { gen: oneof, items: [["b","8b"],["b","16b"],["h","4h"],["h","8h"],["s","4s"]] }
evidence: neon.rs:423
```
