# Properties: encode_neon_rev64

## encode_neon_rev64_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc on the valid NEON REV64 domain. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree REV64 decoder). Sibling encode_cnt / encode_neon_not / encode_neon_rbit rejected (same-job gate: different two-misc opcodes). encode_rev rejected (scalar REV; dispatch does not route `rev64` there). llvm-mc `-triple=aarch64 -show-encoding` is an independent assembler. KAT gate pins known vectors before PBT.
- Doc contract: neon.rs:751 "Encode NEON REV64: reverse elements within 64-bit doublewords" — asserted fingerprint 4adb50d1
- Seed: encode_cnt_pbt.rs:171 encode_cnt_diff_llvm_mc (same two-misc shape; domain widened to T in {8b,16b,4h,8h,2s,4s})
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_rev64([Vd.T, Vn.T]) = Word(w) ∧ w = llvm-mc("rev64 Vd.T, Vn.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_rev64
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: vreg, rn: vreg, t: valid_rev64_arr }
  relation:
    op: eq
    lhs: encode_neon_rev64([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t)])
    rhs: llvm_mc("rev64 v{rd}.{t}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
evidence: neon.rs:751 purpose comment; encoder/mod.rs:750 dispatch; ARM ARM Advanced SIMD two-register miscellaneous REV64
```

## encode_neon_rev64_meta_rd_rn
- Tier: 4
- Rationale: ARM two-misc encoding places Rd in bits[4:0] and Rn in bits[9:5] independently of Q/size/opcode. Metamorphic isolation of those fields is weaker than llvm-mc differential but independently evidenced by the ARM field map (encoding comment neon.rs:761). Stronger differential already used on the same domain.
- Doc contract: neon.rs:761 "REV64 Vd.T, Vn.T: 0 Q 0 01110 size 10 0000 0000 10 Rn Rd" — asserted fingerprint eea97e18
- Seed: encode_cnt_pbt.rs:183 encode_cnt_meta_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. let w(rd,rn)=encode_neon_rev64([Vd.T,Vn.T]). (w(rd1,rn1) ⊕ w(rd2,rn1)) & ~0x1F = 0 ∧ w(rd,rn)[4:0]=rd ∧ (w(rd1,rn1) ⊕ w(rd1,rn2)) & ~(0x1F<<5) = 0 ∧ w(rd,rn)[9:5]=rn
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_rev64
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2, t]
  domain: { rd1: vreg, rd2: vreg, rn1: vreg, rn2: vreg, t: valid_rev64_arr }
  body: "((w11 ^ w21) & !0x1F == 0) && ((w11 ^ w12) & !(0x1F<<5) == 0) && (w11 & 0x1F == rd1) && ((w11>>5)&0x1F == rn1)"
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
evidence: neon.rs:761 ARM field map Rd bits[4:0] Rn bits[9:5]
```

## encode_neon_rev64_inv_layout
- Tier: 4
- Rationale: ARM Advanced SIMD two-register miscellaneous REV64 field layout is an exact structural invariant of every success-path word. Weaker than llvm-mc differential (does not catch a globally-wrong opcode that llvm-mc would). Q and size derived from T by ARM (Q=1 iff T in {16B,8H,4S}; size=00/01/10 for B/H/S), not from the SUT body.
- Doc contract: neon.rs:761 "REV64 Vd.T, Vn.T: 0 Q 0 01110 size 10 0000 0000 10 Rn Rd" — asserted fingerprint eea97e18
- Seed: encode_cnt_pbt.rs:209 encode_cnt_inv_layout
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. let w=encode_neon_rev64([Vd.T,Vn.T]). w[31]=0 ∧ w[30]=Q(T) ∧ w[29:24]=001110 ∧ w[23:22]=size(T) ∧ w[21:16]=100000 ∧ w[15:10]=000010 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_rev64
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: vreg, rn: vreg, t: valid_rev64_arr }
  relation:
    op: eq
    lhs: encode_neon_rev64([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t)])
    rhs: arm_rev64_word(rd, rn, t)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
evidence: ARM ARM Advanced SIMD two-register miscellaneous REV64; neon.rs:761 encoding comment
```

## encode_neon_rev64_neg_arity
- Tier: 3
- Rationale: Documented arity contract: "rev64 requires 2 operands" (neon.rs:754). Inputs with 0 or 1 operand are out of domain and must return Err. Stronger oracles do not apply on the empty/short domain (no encoding to compare).
- Doc contract: neon.rs:754 "rev64 requires 2 operands" — domain-restriction fingerprint cf9fb806
- Seed: encode_cnt_pbt.rs:228 encode_cnt_neg_arity
- Formal: ∀ n ∈ {0,1}, ops with |ops|=n. encode_neon_rev64(ops) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_rev64
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t]
  domain: { n: {0,1}, rd: vreg, t: valid_rev64_arr }
  body: encode_neon_rev64(ops_of_len_n).is_err()
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
expected_error: String
evidence: neon.rs:754 "rev64 requires 2 operands"
```

## encode_neon_rev64_neg_extra
- Tier: 3
- Rationale: llvm-mc and gas both reject a third operand on REV64. The documented arity is 2 (neon.rs:754). Extra operands are invalid and must Err. Domain is the valid two-operand form plus one extra arranged register.
- Doc contract: neon.rs:754 "rev64 requires 2 operands" — domain-restriction fingerprint cf9fb806
- Seed: encode_cnt_pbt.rs:241 encode_cnt_neg_extra
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. llvm-mc("rev64 Vd.T, Vn.T, Vextra.T") fails ∧ encode_neon_rev64([Vd.T, Vn.T, Vextra.T]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="8b" (rev64 v0.8b, v0.8b, v0.8b encodes as Word(0x0e200800))
- Bug report: pbt-out/bug_reports/encode_neon_rev64_extra_operand.md

```property
function: encoder.encode_neon_rev64
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t]
  domain: { rd: vreg, rn: vreg, extra: vreg, t: valid_rev64_arr }
  body: encode_neon_rev64([Vd.T, Vn.T, Vextra.T]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
expected_error: String
evidence: neon.rs:754 arity 2; llvm-mc/gas reject third operand
```

## encode_neon_rev64_neg_invalid_t
- Tier: 3
- Rationale: ARM ARM reserves size=11 for REV64; gas lists only {8b,16b,4h,8h,2s,4s} as valid variants; llvm-mc rejects .1d/.2d and other arrangements. The function's own comment does not declare those inputs invalid, so they stay in the generator. neon_arr_to_q_size accepts 1d/2d.
- Doc contract: neon.rs:751 "Encode NEON REV64: reverse elements within 64-bit doublewords" — asserted fingerprint 4adb50d1
- Seed: encode_cnt_pbt.rs:262 encode_cnt_neg_invalid_t (domain includes size=11 1d/2d which CNT never encoded)
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {1d,2d,4b,8d,1s,2h,8s,32b}. llvm-mc("rev64 Vd.T, Vn.T") fails ∧ encode_neon_rev64([Vd.T, Vn.T]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="1d" (rev64 v0.1d, v0.1d encodes as Word(0x0ee00800); .2d also accepted)
- Bug report: pbt-out/bug_reports/encode_neon_rev64_invalid_arrangement.md

```property
function: encoder.encode_neon_rev64
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: vreg, rn: vreg, t: invalid_rev64_arr }
  body: encode_neon_rev64([Vd.T, Vn.T]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["1d", "2d", "4b", "8d", "1s", "2h", "8s", "32b"] }
expected_error: String
evidence: ARM ARM REV64 size=11 reserved; gas valid variants {8b,16b,4h,8h,2s,4s}; llvm-mc rejects .2d/.1d
```

## encode_neon_rev64_neg_mismatch_t
- Tier: 3
- Rationale: llvm-mc and gas reject mismatched arrangements (`rev64 v0.8b, v1.16b` → operand mismatch). ARM REV64 requires Vd.T and Vn.T to share T. The SUT discards source arrangement (`let (rn, _)`).
- Doc contract: neon.rs:761 "REV64 Vd.T, Vn.T: 0 Q 0 01110 size 10 0000 0000 10 Rn Rd" — asserted fingerprint eea97e18
- Seed: encode_cnt_pbt.rs:283 encode_cnt_neg_mismatch_t
- Formal: ∀ rd,rn ∈ {0..31}, Td,Tn ∈ {8b,16b,4h,8h,2s,4s}, Td ≠ Tn. llvm-mc("rev64 Vd.Td, Vn.Tn") fails ∧ encode_neon_rev64([Vd.Td, Vn.Tn]) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, td="8b", tn="16b" (rev64 v0.8b, v0.16b encodes as Word(0x0e200800) using dest T only)
- Bug report: pbt-out/bug_reports/encode_neon_rev64_mismatch_t.md

```property
function: encoder.encode_neon_rev64
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn]
  domain: { rd: vreg, rn: vreg, td: valid_rev64_arr, tn: valid_rev64_arr, td != tn }
  body: encode_neon_rev64([Vd.Td, Vn.Tn]).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
  tn: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
expected_error: String
evidence: llvm-mc/gas operand mismatch on unequal T; neon.rs:761 Vd.T, Vn.T same T
```

## encode_neon_rev64_neg_non_neon
- Tier: 3
- Rationale: NEON REV64 requires arranged SIMD registers. gas rejects GPR/SP/bare-V/FP. llvm-mc aliases `rev64 x0,x1` to scalar `rev` — different job, not this function's contract. Error contract is gas/ARM NEON form. Dest GPR and source GPR both stay in the domain.
- Doc contract: neon.rs:751 "Encode NEON REV64: reverse elements within 64-bit doublewords" — asserted fingerprint 4adb50d1
- Seed: encode_cnt_pbt.rs:304 encode_cnt_neg_gpr_bare_sp
- Formal: ∀ kind ∈ {x-dest, w-dest, sp-dest, bare-v, d-dest, s-dest, q-dest, x-src}. gas rejects the corresponding `rev64` form ∧ encode_neon_rev64(ops(kind)) = Err(_)
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", kind=7 (rev64 v0.8b, x0 encodes as Word(0x0e200800); dest GPR/SP/bare-V/FP correctly Err)
- Bug report: pbt-out/bug_reports/encode_neon_rev64_gpr_src.md

```property
function: encoder.encode_neon_rev64
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, kind]
  domain: { rd: vreg, rn: vreg, t: valid_rev64_arr, kind: non_neon_kind }
  body: encode_neon_rev64(ops(kind)).is_err()
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
  kind: { gen: int, min: 0, max: 7, type: u8 }
expected_error: String
evidence: neon.rs:751 NEON REV64; gas rejects GPR/SP/bare-V/FP for NEON REV64
```

## encode_neon_rev64_diff_alt_spellings
- Tier: 5
- Rationale: Sweep: parse_reg_num lowercases V/X/W prefixes, so uppercase Vd/Vn must match llvm-mc on the valid domain. Differential vs llvm-mc. parse_reg_num (encoder/mod.rs:153) lowercases the name. Not a new contract — same encoding as lowercase, different spelling.
- Doc contract: neon.rs:751 "Encode NEON REV64: reverse elements within 64-bit doublewords" — asserted fingerprint 4adb50d1
- Seed: encode_cnt_pbt.rs encode_cnt_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s}. encode_neon_rev64([V{rd}.T, V{rn}.T]) = llvm-mc("rev64 V{rd}.T, V{rn}.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_rev64_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_rev64
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t]
  domain: { rd: vreg, rn: vreg, t: valid_rev64_arr }
  relation:
    op: eq
    lhs: encode_neon_rev64([RegArrangement(V{rd}, t), RegArrangement(V{rn}, t)])
    rhs: llvm_mc("rev64 V{rd}.{t}, V{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s"] }
evidence: encoder/mod.rs:153 parse_reg_num lowercases V prefix; llvm-mc accepts uppercase V
```
