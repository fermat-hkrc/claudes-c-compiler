# Properties: encode_neon_sqshrun

## encode_neon_sqshrun_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. README.md:1-14 claims the assembler "accepts the same textual assembly that GCC's gas would consume"; encoder/mod.rs:1-7 encodes AArch64 into 32-bit words. State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree SQSHRUN decoder. Differential vs encode_neon_shrn / encode_neon_qshrn / encode_neon_scalar_qshrn rejected: same-job gate (different opcode / saturation / scalar vs vector). SUT-boundary: internal-helper of the GNU-style assembler. Mapping: encode_neon_sqshrun([Vd.Tb, Vn.Ta, #shift], is_rounding, is_high) <-> `{sq,sqr}shrun{2?} Vd.Tb, Vn.Ta, #shift`.
- Seed: README.md:229; encoder/mod.rs:649-650,841-842
- Formal: ∀ rd,rn ∈ {0..31}, Ta ∈ {8h,4s,2d}, is_high ∈ 𝔹, is_rounding ∈ 𝔹, shift ∈ {1..dest_esize(Ta)}. encode_neon_sqshrun([Vd.Tb, Vn.Ta, Imm(shift)], is_rounding, is_high) = llvm-mc("{sq,sqr}shrun{2?} Vd.Tb, Vn.Ta, #shift") where Tb = mandated pair of (Ta, is_high).
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sqshrun
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, shift, is_high, is_rounding]
  domain: {rd: vreg_0_31, rn: vreg_0_31, ta: ta_arr, shift: dest_shift, is_high: bool, is_rounding: bool}
  relation:
    op: eq
    lhs: sut_word(ops, is_rounding, is_high)
    rhs: llvm_mc_word(asm)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: src/backend/arm/assembler/README.md:1-14; encoder/mod.rs:649-650,841-842; ARM ARM Advanced SIMD shift by immediate SQSHRUN
```

## encode_neon_sqshrun_metamorphic_q
- Tier: 4
- Rationale: ARM ARM Q bit is bit 30 and is the sole difference between SQSHRUN and SQSHRUN2 at equal operands. Stronger differential is property 1; this pins the Q isolation independently of llvm-mc. State machine / round-trip rejected as above.
- Seed: neon.rs:147 let q = if is_high { 1u32 } else { 0 };
- Formal: ∀ valid (rd,rn,Ta,shift,is_rounding). encode_neon_sqshrun(..., is_rounding, false) XOR encode_neon_sqshrun(..., is_rounding, true) = 1<<30.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sqshrun
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, ta, shift, is_rounding]
  domain: {rd: vreg_0_31, rn: vreg_0_31, ta: ta_arr, shift: dest_shift, is_rounding: bool}
  relation:
    op: eq
    lhs: sut_word(ops, is_rounding, false) ^ sut_word(ops, is_rounding, true)
    rhs: 1u32 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_rounding: { gen: bool }
evidence: ARM ARM Advanced SIMD shift by immediate Q bit; neon.rs:147
```

## encode_neon_sqshrun_metamorphic_round
- Tier: 4
- Rationale: ARM ARM opcode bit 11 distinguishes SQSHRUN (100001) from SQRSHRUN (100011). Stronger differential is property 1; this pins the rounding isolation. State machine / round-trip rejected as above.
- Seed: neon.rs:151 opcode_bits = if is_rounding { 0b100011 } else { 0b100001 }
- Formal: ∀ valid (rd,rn,Ta,shift,is_high). encode_neon_sqshrun(..., false, is_high) XOR encode_neon_sqshrun(..., true, is_high) = 1<<11.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sqshrun
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, ta, shift, is_high]
  domain: {rd: vreg_0_31, rn: vreg_0_31, ta: ta_arr, shift: dest_shift, is_high: bool}
  relation:
    op: eq
    lhs: sut_word(ops, false, is_high) ^ sut_word(ops, true, is_high)
    rhs: 1u32 << 11
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_high: { gen: bool }
evidence: ARM ARM SQSHRUN opcode 100001 vs SQRSHRUN 100011; neon.rs:151
```

## encode_neon_sqshrun_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an exact structural predicate on every success-path word. Stronger differential is property 1; this asserts each field independently. Dest esize bound is pinned via generator min/max straddling 1 and dest_esize.
- Seed: neon.rs:117-118 Format comment; ARM ARM Advanced SIMD shift by immediate
- Formal: ∀ valid (rd,rn,Ta,shift,is_high,is_rounding). let w = encode_neon_sqshrun(...). w[31]=0 ∧ w[30]=is_high ∧ w[29]=1 ∧ w[28:23]=011110 ∧ w[22:19]=(src_esize-shift)>>3 ∧ w[18:16]=(src_esize-shift)&7 ∧ w[15:10]=100001 or 100011 ∧ w[9:5]=rn ∧ w[4:0]=rd.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sqshrun
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, ta, shift, is_high, is_rounding]
  domain: {rd: vreg_0_31, rn: vreg_0_31, ta: ta_arr, shift: dest_shift, is_high: bool, is_rounding: bool}
  relation:
    op: holds
    expr: arm_fields_match(sut_word(ops, is_rounding, is_high), rd, rn, ta, shift, is_high, is_rounding)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: ARM ARM Advanced SIMD shift by immediate SQSHRUN; neon.rs:117-155
```

## encode_neon_sqshrun_neg_shift_oob
- Tier: 5
- Rationale: ARM ARM shift range is 1..=dest_esize (esize = dest element size). llvm-mc rejects 0, dest_esize+1, src_esize. Sibling encode_neon_shrn (neon.rs:1443-1444) uses half_bits = source/2 as the same bound. Negative/error contract: out-of-range shift must Err. Documented bounds dest_esize and dest_esize+1 are pinned in the generator. Stronger oracles do not apply on the invalid domain.
- Seed: neon.rs:141-143; encode_neon_shrn neon.rs:1443-1444; llvm-mc rejection of #0/#9/#16 for 8h
- Formal: ∀ rd,rn ∈ {0..31}, Ta ∈ {8h,4s,2d}, is_high, is_rounding, shift ∈ ℤ. (shift < 1 ∨ shift > dest_esize(Ta)) ⇒ encode_neon_sqshrun([Vd.Tb, Vn.Ta, Imm(shift)], is_rounding, is_high) is Err ∧ llvm-mc rejects the corresponding asm.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, ta=8h, shift=9, is_high=false, is_rounding=false (sqshrun v0.8b, v0.8h, #9)
- Bug report: pbt-out/bug_reports/encode_neon_sqshrun_shift_oob.md

```property
function: encoder.neon.encode_neon_sqshrun
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, shift, is_high, is_rounding]
  domain: {rd: vreg_0_31, rn: vreg_0_31, ta: ta_arr, shift: oob_shift, is_high: bool, is_rounding: bool}
  relation:
    op: throws
    expr: encode_neon_sqshrun(ops, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: -16, max: 80, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: ARM ARM SQSHRUN shift 1..=esize; encode_neon_shrn neon.rs:1443-1444; llvm-mc
```

## encode_neon_sqshrun_neg_dest_tb
- Tier: 5
- Rationale: ARM ARM mandates Tb/Ta pairs (Q=0: 8B<-8H, 4H<-4S, 2S<-2D; Q=1: 16B<-8H, 8H<-4S, 4S<-2D). llvm-mc rejects mismatched dest. README claims gas-compatible text. Dest arrangement is a documented contract of the public GNU-style mnemonic, not an implementation detail. Stronger oracles do not apply on the invalid domain.
- Seed: README.md:1-14; llvm-mc rejection of sqshrun v0.16b, v1.8h, #1
- Formal: ∀ rd,rn, Ta ∈ {8h,4s,2d}, shift ∈ {1..dest_esize(Ta)}, is_high, is_rounding, Tb ∈ arrangements. Tb ≠ mandated(Ta, is_high) ⇒ encode_neon_sqshrun([Vd.Tb, Vn.Ta, Imm(shift)], is_rounding, is_high) is Err ∧ llvm-mc rejects.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, ta=8h, shift=1, tb=4h, is_high=false, is_rounding=false (sqshrun v0.4h, v0.8h, #1)
- Bug report: pbt-out/bug_reports/encode_neon_sqshrun_mismatched_dest_tb.md

```property
function: encoder.neon.encode_neon_sqshrun
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, shift, tb, is_high, is_rounding]
  domain: {rd: vreg_0_31, rn: vreg_0_31, ta: ta_arr, tb: mismatched_tb, shift: dest_shift, is_high: bool, is_rounding: bool}
  relation:
    op: throws
    expr: encode_neon_sqshrun(ops, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  tb: { gen: oneof, options: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: ARM ARM SQSHRUN Tb/Ta pairs; README.md:1-14; llvm-mc
```

## encode_neon_sqshrun_neg_gpr_dest
- Tier: 5
- Rationale: GNU-style SQSHRUN dest is Vd.Tb, never a GPR or scalar FP register. llvm-mc rejects sqshrun x0, v1.8h, #1. parse_reg_num accepts x/w/d/s/q/h/b prefixes, so a bare Operand::Reg dest is a documented invalid domain. Stronger oracles do not apply on the invalid domain.
- Seed: README.md:229 NEON narrow (vector Vd.Tb); llvm-mc rejection of GPR dest
- Formal: ∀ prefix ∈ {x,w,d,s,q,h,b}, n ∈ {0..31}, rn, Ta, shift ∈ {1..dest_esize}, is_high, is_rounding. encode_neon_sqshrun([Reg(prefix n), Vn.Ta, Imm(shift)], is_rounding, is_high) is Err ∧ llvm-mc rejects.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: prefix=x, n=0, rn=0, ta=8h, shift=1, is_high=false, is_rounding=false (sqshrun x0, v0.8h, #1)
- Bug report: pbt-out/bug_reports/encode_neon_sqshrun_gpr_dest.md

```property
function: encoder.neon.encode_neon_sqshrun
oracle: negative_error
predicate:
  quantifier: forall
  vars: [prefix, n, rn, ta, shift, is_high, is_rounding]
  domain: {prefix: gpr_fp_prefix, n: 0..31, rn: vreg_0_31, ta: ta_arr, shift: dest_shift, is_high: bool, is_rounding: bool}
  relation:
    op: throws
    expr: encode_neon_sqshrun(ops, is_rounding, is_high)
expected_error: String
generators:
  prefix: { gen: oneof, options: ["x", "w", "d", "s", "q", "h", "b"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: README.md:229; llvm-mc; parse_reg_num encoder/mod.rs:131-147
```

## encode_neon_sqshrun_neg_extra_arity_kinds
- Tier: 5
- Rationale: GNU-style SQSHRUN is a 3-operand instruction (Vd.Tb, Vn.Ta, #shift). llvm-mc rejects a 4th operand. Arity less than 3 is documented by the SUT error string "sqshrun requires 3 operands". Unsupported source Ta (not 8h/4s/2d) and non-register/non-imm kinds are invalid GNU-style forms. Stronger oracles do not apply on the invalid domain.
- Seed: neon.rs:121-122; llvm-mc rejection of 4-operand form
- Formal: ∀ (len < 3) or Ta not in {8h,4s,2d} or non-matching operand kind at a slot or invalid register name. encode_neon_sqshrun(...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sqshrun
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ta, is_high, is_rounding, n, slot, which, bad]
  domain: {arity: 0..2, extra: vreg, ta: bad_ta, kinds: non_reg_imm, names: invalid_vreg}
  relation:
    op: throws
    expr: encode_neon_sqshrun(malformed, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 2, type: usize }
  ta: { gen: oneof, options: ["8b", "16b", "4h", "2s", "1d", "16h", "8s", "4d", "", "b", "h"] }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: neon.rs:121-122; README.md:1-14; llvm-mc
```

## encode_neon_sqshrun_neg_extra_operand
- Tier: 5
- Rationale: GNU-style SQSHRUN is a 3-operand instruction (Vd.Tb, Vn.Ta, #shift). llvm-mc rejects a 4th operand. README.md:1-14 claims gas-compatible text. Stronger oracles do not apply on the invalid domain.
- Seed: neon.rs:121-122; llvm-mc rejection of 4-operand form
- Formal: ∀ rd,rn,extra ∈ {0..31}, Ta ∈ {8h,4s,2d}, shift ∈ {1..dest_esize(Ta)}, is_high, is_rounding. encode_neon_sqshrun([Vd.Tb, Vn.Ta, Imm(shift), extra], is_rounding, is_high) is Err ∧ llvm-mc rejects.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, ta=8h, shift=1, is_high=false, is_rounding=false (sqshrun v0.8b, v0.8h, #1, v0)
- Bug report: pbt-out/bug_reports/encode_neon_sqshrun_extra_operand.md

```property
function: encoder.neon.encode_neon_sqshrun
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ta, shift, is_high, is_rounding]
  domain: {rd: vreg_0_31, rn: vreg_0_31, extra: vreg_0_31, ta: ta_arr, shift: dest_shift, is_high: bool, is_rounding: bool}
  relation:
    op: throws
    expr: encode_neon_sqshrun(ops4, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: neon.rs:121-122; README.md:1-14; llvm-mc
```

## encode_neon_sqshrun_neg_shift_i64_trunc
- Tier: 5
- Rationale: Operand::Imm is i64. The SUT does `*v as u32`, so an i64 whose low 32 bits look like a valid shift is encoded as that shift. GNU-style #imm must be rejected when the i64 is not in 1..=dest_esize. Coverage-sweep of the as-u32 path. Stronger oracles do not apply on the invalid domain.
- Seed: neon.rs:125 `*v as u32`; parser.rs:24 Operand::Imm(i64)
- Formal: ∀ rd,rn, Ta, shift ∈ {1..dest_esize}, k ≠ 0, is_high, is_rounding. let wide = shift + k*2^32. encode_neon_sqshrun([Vd.Tb, Vn.Ta, Imm(wide)], is_rounding, is_high) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, ta=8h, shift=1, k=1, is_high=false, is_rounding=false (Imm(4294967297))
- Bug report: pbt-out/bug_reports/encode_neon_sqshrun_shift_i64_trunc.md

```property
function: encoder.neon.encode_neon_sqshrun
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, shift, is_high, is_rounding, k]
  domain: {rd: vreg_0_31, rn: vreg_0_31, ta: ta_arr, shift: dest_shift, k: nonzero_i64, is_high: bool, is_rounding: bool}
  relation:
    op: throws
    expr: encode_neon_sqshrun(ops_wide, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  k: { gen: int, min: -4, max: 4, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: neon.rs:125; parser.rs:24 Operand::Imm(i64)
```

## encode_neon_sqshrun_neg_reg_source
- Tier: 5
- Rationale: GNU-style SQSHRUN source is Vn.Ta. A bare Operand::Reg (GPR/FP/V without arrangement) is not Vn.Ta. Coverage-sweep of the Operand::Reg source arm. Stronger oracles do not apply on the invalid domain.
- Seed: neon.rs:14-17 Operand::Reg path; README.md:229 Vn.Ta
- Formal: ∀ prefix ∈ {x,w,d,s,q,h,b,v}, n ∈ {0..31}, rd, Ta, shift ∈ {1..dest_esize}, is_high, is_rounding. encode_neon_sqshrun([Vd.Tb, Reg(prefix n), Imm(shift)], is_rounding, is_high) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_sqshrun
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, prefix, n, ta, shift, is_high, is_rounding]
  domain: {rd: vreg_0_31, prefix: gpr_fp_v, n: 0..31, ta: ta_arr, shift: dest_shift, is_high: bool, is_rounding: bool}
  relation:
    op: throws
    expr: encode_neon_sqshrun(ops, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  prefix: { gen: oneof, options: ["x", "w", "d", "s", "q", "h", "b", "v"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, options: ["8h", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  is_high: { gen: bool }
  is_rounding: { gen: bool }
evidence: neon.rs:14-17; README.md:229
```
