# Properties: encode_neon_sli

## encode_neon_sli_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (independent AArch64 assembler). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree SLI decoder. encode_neon_shl / encode_neon_shift_left_imm fail the same-job sibling gate (SHL vs SLI; parameterized SQSHL/UQSHL helper vs dedicated SLI). SUT-boundary: internal-helper of the GNU-style assembler (README gas-compatible). Mapping: [RegArrangement(Vd,T), RegArrangement(Vn,T), Imm(shift)] <-> `sli Vd.T, Vn.T, #shift`. Doc evidence: README.md:5-14, encoder/mod.rs:1-7, neon.rs:1257-1267, ARM ARM Advanced SIMD shift-by-immediate SLI (U=1).
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1]. encode_neon_sli([Vd.T, Vn.T, #shift]) = llvm-mc("sli Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sli
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 0..esize(t)-1 }
  relation:
    op: eq
    lhs: encode_neon_sli([Vd.t, Vn.t, Imm(shift)])
    rhs: llvm_mc("sli Vd.t, Vn.t, #shift")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 0, max: 63, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14; encoder/mod.rs:1-7; neon.rs:1257-1282
```

## encode_neon_sli_roundtrip_arm_fields
- Tier: 4
- Rationale: Algebraic invariant of the ARM ARM shift-by-immediate layout claimed at neon.rs:1266-1267. Differential is stronger and used above; this unpacks Q/U/immh:immb/opcode/Rn/Rd so a packing slip still fails even if llvm-mc were unavailable. Stronger round-trip rejected: no decoder.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_roundtrip_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1]. let w = encode_neon_sli([Vd.T,Vn.T,#shift]). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=1 ∧ w[28:23]=011110 ∧ w[22:16]=esize(T)+shift ∧ w[15:10]=010101 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sli
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { t: "8b|16b|4h|8h|2s|4s|2d", shift: 0..esize(t)-1 }
  relation:
    op: holds
    expr: unpack(encode_neon_sli([Vd.t, Vn.t, Imm(shift)])) matches ARM shift-by-immediate SLI layout
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1266-1267 Format 0 Q 1 0 11110 immh:immb 010101 Rn Rd (U=1)
```

## encode_neon_sli_metamorphic_q_bit
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM Q is bit 30 and is the only bit that distinguishes 8B vs 16B, 4H vs 8H, 2S vs 4S at equal shift. Stronger differential covers absolute encoding; this isolates the Q toggle. 2D has no valid Q=0 pair (1D reserved). State machine / round-trip rejected as above.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_metamorphic_u_bit
- Formal: ∀ rd,rn ∈ {0..31}, (Tlo,Thi) ∈ {(8b,16b),(4h,8h),(2s,4s)}, shift ∈ [0, esize(Tlo)-1]. encode_neon_sli([Vd.Tlo,Vn.Tlo,#shift]) XOR encode_neon_sli([Vd.Thi,Vn.Thi,#shift]) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sli
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, tlo, thi, shift]
  relation:
    op: eq
    lhs: encode([Vd.tlo, Vn.tlo, Imm(shift)]) XOR encode([Vd.thi, Vn.thi, Imm(shift)])
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1266 Q at bit 30; ARM ARM Q=0 for 8B/4H/2S, Q=1 for 16B/8H/4S
```

## encode_neon_sli_metamorphic_shift_inc
- Tier: 4
- Rationale: Algebraic metamorphic: ARM ARM immh:immb = esize + shift sits at bits [22:16]; incrementing a valid shift by 1 (still in range) must add exactly 1 to that field and touch no other bit. Stronger differential covers absolute encoding; this isolates the shift encoding. Documented bounds 0 and esize-1 are co-generated; the increment domain is [0, esize-2] so both endpoints of a +1 step are valid.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_metamorphic_size_hi
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-2]. encode_neon_sli([Vd.T,Vn.T,#(shift+1)]) − encode_neon_sli([Vd.T,Vn.T,#shift]) = 1<<16
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sli
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  relation:
    op: eq
    lhs: encode([Vd.t, Vn.t, Imm(shift+1)]) XOR encode([Vd.t, Vn.t, Imm(shift)])
    rhs: 1 << 16
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1266-1276 immh:immb = esize + shift at bits [22:16]; ARM ARM SLI shift encoding
```

## encode_neon_sli_neg_extra_operands
- Tier: 4e
- Rationale: Negative/error contract: llvm-mc rejects a fourth operand (`invalid operand for instruction`); README gas-compatible assembler must reject the same. Arity check is `len < 3` (not `!= 3`). Stronger differential does not apply on the invalid domain.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_extra_operands
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1], extra_op ∈ {RegArrangement, Imm, Reg, Mem}. encode_neon_sli([Vd.T, Vn.T, #shift, extra_op]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="8b", shift=0, extra_kind=0 — sli v0.8b, v0.8b, #0, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_sli_extra_operand.md

```property
function: encoder.neon.encode_neon_sli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, extra_op]
  relation:
    op: throws
    lhs: encode_neon_sli([Vd.t, Vn.t, Imm(shift), extra_op])
    error: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  extra_op: { gen: oneof, items: ["RegArrangement", "Imm", "Reg", "Mem"] }
expected_error: String
evidence: llvm-mc rejects fourth operand; README.md:5-14 gas-compatible; neon.rs:1259-1261 arity < 3
```

## encode_neon_sli_neg_shift_out_of_range
- Tier: 4e
- Rationale: Negative/error contract: llvm-mc rejects shift outside [0, esize-1] (`immediate must be an integer in range [0, esize-1]`). Documented bounds sampled at esize, esize+1, and -1 (min-1). SUT currently masks immh:immb rather than rejecting. Stronger differential does not apply on the invalid domain.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_unsupported_arrangement
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ {-1, esize(T), esize(T)+1} ∪ (ℤ \ [0, esize(T)-1]). encode_neon_sli([Vd.T, Vn.T, #shift]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", shift=-1 (debug overflow panic); also t="8b", shift=8 (masked Ok)
- Bug report: pbt-out/bug_reports/encode_neon_sli_shift_out_of_range.md

```property
function: encoder.neon.encode_neon_sli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { shift: not in 0..esize(t)-1, including -1, esize, esize+1 }
  relation:
    op: throws
    lhs: encode_neon_sli([Vd.t, Vn.t, Imm(shift)])
    error: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  shift: { gen: int, min: -2, max: 65, type: i64 }
expected_error: String
evidence: llvm-mc "immediate must be an integer in range [0, esize-1]"; ARM ARM SLI shift; neon.rs:1268-1276
```

## encode_neon_sli_neg_arity_and_shape
- Tier: 4e
- Rationale: Negative/error contract: llvm-mc rejects too few operands, non-register dest/src, invalid names (v32, foo, empty, v, v-1, v99), unsupported T (1d, 8s, 1s, empty), dest Operand::Reg (no arrangement — scalar SLI is a different encoding). neon.rs:1259-1261 arity < 3; get_neon_reg / neon_arr_to_q_size / arrangement match reject the rest.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_arity_and_shape
- Formal: ∀ n ∈ {0,1,2}, bad_name ∈ {v32,foo,"",v,v-1,v99}, Tbad ∈ {1d,8s,1s,"",b,h}, nonreg ∈ {Imm,Mem,Symbol,Shift,Cond,Label}. encode_neon_sli(ops[:n]) = Err ∧ encode_neon_sli([bad_name.8b, Vn.8b, #0]) = Err ∧ encode_neon_sli([Vd.Tbad, Vn.Tbad, #0]) = Err ∧ encode_neon_sli([nonreg, Vn.8b, #0]) = Err ∧ encode_neon_sli([Reg(Vd), Vn.8b, #0]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, bad_name, tbad, nonreg]
  relation:
    op: throws
    lhs: encode_neon_sli(short_or_malformed)
    error: Err
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  bad_name: { gen: oneof, items: ["v32", "foo", "", "v", "v-1", "v99"] }
expected_error: String
evidence: llvm-mc rejects too few operands / invalid names / T=1d; neon.rs:1259-1276; get_neon_reg neon.rs:7-21
```

## encode_neon_sli_neg_arrangement_mismatch_and_non_v
- Tier: 4e
- Rationale: Negative/error contract: llvm-mc rejects dest/src T mismatch (`invalid operand`) and non-V prefixes (x/w/d/s/q/h/b). README gas-compatible. SUT reads only dest T and parse_reg_num accepts those prefixes. Stronger differential does not apply on the invalid domain.
- Seed: encode_neon_float_cmp_zero_pbt::encode_neon_float_cmp_zero_neg_non_v_prefix / encode_neon_float_cmp_zero_neg_arrangement_mismatch
- Formal: ∀ rd,rn ∈ {0..31}, Td ≠ Tn ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, min(esize(Td),esize(Tn))-1], prefix ∈ {x,w,d,s,q,h,b}. encode_neon_sli([Vd.Td, Vn.Tn, #shift]) = Err ∧ encode_neon_sli([prefix{rd}.Td, Vn.Td, #shift]) = Err ∧ encode_neon_sli([Vd.Td, prefix{rn}.Td, #shift]) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: mismatch rd=0,rn=0,td="8b",tn="16b",shift=0 — sli v0.8b, v0.16b, #0; non-v rd=0,rn=0,t="8b",shift=0,prefix="x",which=0 — sli x0.8b, v0.8b, #0
- Bug report: pbt-out/bug_reports/encode_neon_sli_arrangement_mismatch.md; pbt-out/bug_reports/encode_neon_sli_non_v_prefix.md

```property
function: encoder.neon.encode_neon_sli
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, shift, prefix]
  relation:
    op: throws
    lhs: encode_neon_sli(mismatched_or_non_v)
    error: Err
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  prefix: { gen: oneof, items: ["x", "w", "d", "s", "q", "h", "b"] }
expected_error: String
evidence: llvm-mc rejects T mismatch and non-v prefixes; README.md:5-14; parse_reg_num encoder/mod.rs:131-148
```
