# Properties: encode_neon_float_two_misc

## encode_neon_float_two_misc_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc, an independent AArch64 assembler. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree float two-misc decoder). Sibling encode_neon_two_misc rejected as differential (same-job gate: integer two-misc, different size map). Sibling encode_neon_float_cmp_zero rejected (compare-with-zero job). Sibling encode_neon_float_three_same rejected (three-register). Doc evidence: README.md:11 GNU-style assembly; README.md:226/236 vector fneg/fabs/fsqrt/frint*/fcvtzs/fcvtzu/ucvtf/scvtf/frecpe/frsqrte; encoder/mod.rs:406-458 dispatch; ARM ARM Advanced SIMD two-register miscellaneous (FP).
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_diff_llvm_mc
- Formal: ∀ rd,rn ∈ 0..31, T ∈ {2s,4s,2d}, (U,size_hi,opcode,mnem) ∈ ARM-correct two-misc FP table. encode_neon_float_two_misc([RegArrangement(v{rd},T), RegArrangement(v{rn},T)], U, size_hi, opcode) = llvm-mc("{mnem} v{rd}.{T}, v{rn}.{T}")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, size_hi, opcode, mnem]
  domain: { rd: 0..31, rn: 0..31, t: {2s,4s,2d}, (u,size_hi,opcode,mnem): arm_fp_two_misc_table }
  relation:
    op: eq
    lhs: "encode_neon_float_two_misc([RegArrangement(v(rd), t), RegArrangement(v(rn), t)], u, size_hi, opcode)"
    rhs: "llvm_mc_word(format!(\"{} v{}.{ }, v{}.{}\", mnem, rd, t, rn, t))"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/README.md:11,226,236; encoder/mod.rs:406-458; ARM ARM Advanced SIMD two-register miscellaneous FP
```

## encode_neon_float_two_misc_arm_fields
- Tier: 4d
- Rationale: Algebraic invariant of the ARM word layout documented by neon.rs:1417-1418 and ARM ARM Advanced SIMD two-register miscellaneous. Stronger differential already used above. Field unpack is not a semantic inverse (no decoder).
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_roundtrip_arm_fields
- Formal: ∀ rd,rn ∈ 0..31, T ∈ {2s,4s,2d}, U,size_hi ∈ {0,1}, opcode ∈ 0..31. let w = encode_neon_float_two_misc([Vd.T,Vn.T],U,size_hi,opcode). Then w[31]=0, w[30]=Q(T), w[29]=U, w[28:24]=01110, w[23:22]=(size_hi<<1)|sz(T), w[21:17]=10000, w[16:12]=opcode, w[11:10]=10, w[9:5]=rn, w[4:0]=rd. Q(2s)=0,Q(4s)=1,Q(2d)=1; sz(2s)=0,sz(4s)=0,sz(2d)=1.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, size_hi, opcode]
  domain: { rd: 0..31, rn: 0..31, t: {2s,4s,2d}, u: 0..1, size_hi: 0..1, opcode: 0..31 }
  relation:
    op: holds
    expr: "word_fields_match_arm_two_misc_fp(encode_neon_float_two_misc([Vd.T,Vn.T],u,size_hi,opcode), rd, rn, t, u, size_hi, opcode)"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  size_hi: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1417-1418 Format 0 Q U 01110 size 10000 opcode 10 Rn Rd; ARM ARM Advanced SIMD two-register miscellaneous
```

## encode_neon_float_two_misc_metamorphic_u_bit
- Tier: 4c
- Rationale: Algebraic metamorphic. ARM U occupies bit 29 only; flipping U with other fields fixed must XOR exactly 1<<29. Stronger differential already used.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_metamorphic_u_bit
- Formal: ∀ rd,rn ∈ 0..31, T ∈ {2s,4s,2d}, size_hi ∈ {0,1}, opcode ∈ 0..31. encode(...,U=0,...) XOR encode(...,U=1,...) = 1<<29
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, size_hi, opcode]
  domain: { rd: 0..31, rn: 0..31, t: {2s,4s,2d}, size_hi: 0..1, opcode: 0..31 }
  relation:
    op: eq
    lhs: "encode_neon_float_two_misc(ops, 0, size_hi, opcode) ^ encode_neon_float_two_misc(ops, 1, size_hi, opcode)"
    rhs: "1u32 << 29"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  size_hi: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1417 Format 0 Q U 01110 ...; ARM ARM U at bit 29
```

## encode_neon_float_two_misc_metamorphic_size_hi
- Tier: 4c
- Rationale: Algebraic metamorphic. size[1]=size_hi occupies bit 23 only; flipping size_hi must XOR exactly 1<<23. Stronger differential already used.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_metamorphic_size_hi
- Formal: ∀ rd,rn ∈ 0..31, T ∈ {2s,4s,2d}, U ∈ {0,1}, opcode ∈ 0..31. encode(...,size_hi=0,...) XOR encode(...,size_hi=1,...) = 1<<23
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, u, opcode]
  domain: { rd: 0..31, rn: 0..31, t: {2s,4s,2d}, u: 0..1, opcode: 0..31 }
  relation:
    op: eq
    lhs: "encode_neon_float_two_misc(ops, u, 0, opcode) ^ encode_neon_float_two_misc(ops, u, 1, opcode)"
    rhs: "1u32 << 23"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1418 size[1]=size_hi; ARM ARM size at bits[23:22]
```

## encode_neon_float_two_misc_metamorphic_q
- Tier: 4c
- Rationale: Algebraic metamorphic. 2S vs 4S differ only in Q (bit 30); sz is 0 for both. Stronger differential already used.
- Seed: neon.rs encode_neon_float_three_same_pbt encode_neon_float_three_same_metamorphic_q
- Formal: ∀ rd,rn ∈ 0..31, U,size_hi ∈ {0,1}, opcode ∈ 0..31. encode(2s,...) XOR encode(4s,...) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, u, size_hi, opcode]
  domain: { rd: 0..31, rn: 0..31, u: 0..1, size_hi: 0..1, opcode: 0..31 }
  relation:
    op: eq
    lhs: "encode_neon_float_two_misc(ops_2s, u, size_hi, opcode) ^ encode_neon_float_two_misc(ops_4s, u, size_hi, opcode)"
    rhs: "1u32 << 30"
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  size_hi: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1423-1425 2s=>Q=0, 4s=>Q=1, sz=0 both; ARM ARM Q at bit 30
```

## encode_neon_float_two_misc_neg_unsupported_arrangement
- Tier: 5
- Rationale: Negative/error contract. ARM ARM T in {2S,4S,2D}; llvm-mc rejects 8b/16b/4h/8h/1d/1s and empty/malformed T (fullfp16 not enabled, matching default llvm-mc). Function comment match arm documents only 2s/4s/2d. Stronger oracles do not apply to the invalid-T domain.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_neg_unsupported_arrangement
- Formal: ∀ rd,rn ∈ 0..31, U,size_hi ∈ {0,1}, opcode ∈ 0..31, T ∉ {2s,4s,2d} in {8b,16b,4h,8h,1d,1s,3s,8s,"",b,h}. encode_neon_float_two_misc([Vd.T,Vn.T],U,size_hi,opcode) is Err. For nonempty T, llvm-mc also rejects.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, u, size_hi, opcode, t]
  domain: { rd: 0..31, rn: 0..31, u: 0..1, size_hi: 0..1, opcode: 0..31, t: {8b,16b,4h,8h,1d,1s,3s,8s,"",b,h} }
  relation:
    op: throws
    expr: "encode_neon_float_two_misc([RegArrangement(v(rd), t), RegArrangement(v(rn), t)], u, size_hi, opcode)"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  size_hi: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "1d", "1s", "3s", "8s", "", "b", "h"] }
evidence: neon.rs:1423-1425 match 2s/4s/2d else Err; ARM ARM T in {2S,4S,2D}; llvm-mc rejects other T
```

## encode_neon_float_two_misc_neg_extra_operands
- Tier: 5
- Rationale: Negative/error contract. llvm-mc / gas reject a 3rd operand on vector fneg/fabs/etc. README.md:11 gas compatibility. Stronger oracles do not apply to the extra-operand domain.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_neg_extra_operands
- Formal: ∀ rd,rn,extra ∈ 0..31, T ∈ {2s,4s,2d}, (U,size_hi,opcode,mnem) ∈ ARM table, extra_kind ∈ {RegArrangement, Imm, Reg, Mem}. encode_neon_float_two_misc([Vd.T, Vn.T, extra], U, size_hi, opcode) is Err. llvm-mc("{mnem} Vd.T, Vn.T, extra") is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t=2s, insn=(1,1,15,fneg), extra_kind=0 (fneg v0.2s, v0.2s, v0.2s)
- Bug report: pbt-out/bug_reports/encode_neon_float_two_misc_extra_operand.md

```property
function: encode_neon_float_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, insn, extra_kind]
  domain: { rd,rn,extra: 0..31, t: {2s,4s,2d}, insn: arm_fp_two_misc_table, extra_kind: 0..3 }
  relation:
    op: throws
    expr: "encode_neon_float_two_misc([Vd.T, Vn.T, extra_op], u, size_hi, opcode)"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
evidence: README.md:11 gas compatibility; llvm-mc rejects extra operand on vector fneg/fabs
```

## encode_neon_float_two_misc_neg_arity_and_shape
- Tier: 5
- Rationale: Negative/error contract. Fewer than 2 operands, non-register kinds, and invalid V names (v32/foo/empty/v/v-1/v99) must Err. llvm-mc rejects v32 and non-register shapes. get_neon_reg documents expected NEON register.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_neg_arity_and_shape
- Formal: ∀ n ∈ 0..1, invalid name ∈ {v32,foo,"",v,v-1,v99}, non-reg kind ∈ {Imm,Mem,Symbol,Shift,Cond,Label}. encode with len=n is Err; encode with invalid dest/src name is Err; encode with non-reg dest or src is Err; dest Operand::Reg (no arrangement) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, which, u, rd, bad]
  domain: { n: 0..1, which: 0..5, u: 0..1, rd: 0..31, bad: {v32,foo,"",v,v-1,v99} }
  relation:
    op: throws
    expr: "encode_neon_float_two_misc(short_or_ill_typed, u, 1, 0b01111)"
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  which: { gen: int, min: 0, max: 5, type: u32 }
  u: { gen: int, min: 0, max: 1, type: u32 }
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:7-20 get_neon_reg expected NEON register; llvm-mc rejects v32 and non-register operands
```

## encode_neon_float_two_misc_neg_non_v_prefix
- Tier: 5
- Rationale: Negative/error contract. ARM Vd/Vn are SIMD V registers; llvm-mc rejects x/w/d/s/q/h/b prefixes with arrangement. parse_reg_num accepts those prefixes, so this is a reachable invalid domain. README.md:11 gas compatibility.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_neg_non_v_prefix
- Formal: ∀ rd,rn ∈ 0..31, T ∈ {2s,4s,2d}, (U,size_hi,opcode,mnem) ∈ ARM table, prefix ∈ {x,w,d,s,q,h,b}, which ∈ {dest,src}. encode with prefix{rd}.T in slot which is Err. llvm-mc rejects.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t=2s, insn=(1,1,15,fneg), prefix=x, which=0 (fneg x0.2s, v0.2s)
- Bug report: pbt-out/bug_reports/encode_neon_float_two_misc_non_v_prefix.md

```property
function: encode_neon_float_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, insn, prefix, which]
  domain: { rd,rn: 0..31, t: {2s,4s,2d}, insn: arm_fp_two_misc_table, prefix: {x,w,d,s,q,h,b}, which: 0..1 }
  relation:
    op: throws
    expr: "encode_neon_float_two_misc(ops_with_non_v_prefix, u, size_hi, opcode)"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  prefix: { gen: oneof, values: ["x", "w", "d", "s", "q", "h", "b"] }
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:11 gas compatibility; ARM ARM Vd/Vn SIMD registers; llvm-mc rejects non-V prefix
```

## encode_neon_float_two_misc_neg_arrangement_mismatch
- Tier: 5
- Rationale: Negative/error contract. ARM requires dest T = src T; llvm-mc rejects mismatched arrangements. Function reads arrangement only from dest (neon.rs:1422 ignores src arr). README.md:11 gas compatibility.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt encode_neon_float_cmp_zero_neg_arrangement_mismatch
- Formal: ∀ rd,rn ∈ 0..31, Td,Tn ∈ {2s,4s,2d} with Td ≠ Tn, (U,size_hi,opcode,mnem) ∈ ARM table. encode_neon_float_two_misc([Vd.Td, Vn.Tn], U, size_hi, opcode) is Err. llvm-mc rejects.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, td=2s, tn=4s, insn=(1,1,15,fneg) (fneg v0.2s, v0.4s)
- Bug report: pbt-out/bug_reports/encode_neon_float_two_misc_arrangement_mismatch.md

```property
function: encode_neon_float_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, insn]
  domain: { rd,rn: 0..31, td,tn: {2s,4s,2d}, td != tn, insn: arm_fp_two_misc_table }
  relation:
    op: throws
    expr: "encode_neon_float_two_misc([RegArrangement(v(rd), td), RegArrangement(v(rn), tn)], u, size_hi, opcode)"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["2s", "4s", "2d"] }
  tn: { gen: oneof, values: ["2s", "4s", "2d"] }
evidence: README.md:11 gas compatibility; ARM ARM Vd.<T>, Vn.<T> same T; llvm-mc rejects mismatched T
```

## encode_neon_float_two_misc_neg_bare_src
- Tier: 5
- Rationale: Negative/error contract. llvm-mc rejects a source without arrangement (fneg v0.4s, v1). get_neon_reg accepts Operand::Reg and the function ignores src arrangement. README.md:11 gas compatibility.
- Seed: neon.rs encode_neon_aes_pbt test_encode_neon_aes_regression_bare_src
- Formal: ∀ rd,rn ∈ 0..31, T ∈ {2s,4s,2d}, (U,size_hi,opcode,mnem) ∈ ARM table. encode_neon_float_two_misc([RegArrangement(v{rd},T), Reg(v{rn})], U, size_hi, opcode) is Err. llvm-mc("{mnem} v{rd}.{T}, v{rn}") is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t=2s, insn=(1,1,15,fneg) (fneg v0.2s, v0)
- Bug report: pbt-out/bug_reports/encode_neon_float_two_misc_bare_src.md

```property
function: encode_neon_float_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, insn]
  domain: { rd,rn: 0..31, t: {2s,4s,2d}, insn: arm_fp_two_misc_table }
  relation:
    op: throws
    expr: "encode_neon_float_two_misc([RegArrangement(v(rd), t), Reg(v(rn))], u, size_hi, opcode)"
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
evidence: README.md:11 gas compatibility; llvm-mc rejects bare source without arrangement
```

## encode_neon_float_two_misc_neg_sp
- Tier: 5
- Rationale: Negative/error contract (coverage sweep). Parser is_register accepts sp/wsp/xzr/wzr/lr; parse_reg_num maps them to 31/30. llvm-mc rejects those as SIMD operands. README.md:11 gas compatibility.
- Seed: neon.rs encode_neon_aes_pbt test_encode_neon_aes_regression_sp
- Formal: ∀ T ∈ {2s,4s,2d}, (U,size_hi,opcode,mnem) ∈ ARM table, alias ∈ {sp,wsp,xzr,wzr,lr}, which ∈ {dest,src}. encode with alias.T in slot which is Err. llvm-mc rejects.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: t=2s, insn=(1,1,15,fneg), alias=sp, which=0 (fneg sp.2s, v0.2s)
- Bug report: pbt-out/bug_reports/encode_neon_float_two_misc_sp.md

```property
function: encode_neon_float_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [t, insn, alias, which]
  domain: { t: {2s,4s,2d}, insn: arm_fp_two_misc_table, alias: {sp,wsp,xzr,wzr,lr}, which: 0..1 }
  relation:
    op: throws
    expr: "encode_neon_float_two_misc(ops_with_gp_alias, u, size_hi, opcode)"
expected_error: String
generators:
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  alias: { gen: oneof, values: ["sp", "wsp", "xzr", "wzr", "lr"] }
  which: { gen: int, min: 0, max: 1, type: u32 }
evidence: README.md:11 gas compatibility; parser.rs:2280 sp|wsp|xzr|wzr|lr; llvm-mc rejects GP aliases as SIMD regs
```

## encode_neon_float_two_misc_diff_alt_spellings
- Tier: 2
- Rationale: Differential vs llvm-mc for alternate V-register spellings (v0/V0/v31/V31). parse_reg_num lowercases; llvm-mc accepts uppercase. Coverage sweep of register-name aliases that ARE valid.
- Seed: neon.rs encode_neon_float_cmp_zero_pbt (vreg helper); bitfield encode_ubfx_diff_alt_spellings
- Formal: ∀ T ∈ {2s,4s,2d}, (U,size_hi,opcode,mnem) ∈ ARM table, dest,src ∈ {v0,V0,v31,V31,v1,V1}. encode_neon_float_two_misc([RegArrangement(dest,T), RegArrangement(src,T)], U, size_hi, opcode) = llvm-mc("{mnem} dest.T, src.T")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_float_two_misc
oracle: differential
predicate:
  quantifier: forall
  vars: [t, insn, dest, src]
  domain: { t: {2s,4s,2d}, insn: arm_fp_two_misc_table, dest: {v0,V0,v31,V31}, src: {v1,V1,v31,V31} }
  relation:
    op: eq
    lhs: "encode_neon_float_two_misc([RegArrangement(dest, t), RegArrangement(src, t)], u, size_hi, opcode)"
    rhs: "llvm_mc_word(format!(\"{} {}.{}, {}.{}\", mnem, dest, t, src, t))"
generators:
  t: { gen: oneof, values: ["2s", "4s", "2d"] }
  dest: { gen: oneof, values: ["v0", "V0", "v31", "V31"] }
  src: { gen: oneof, values: ["v1", "V1", "v31", "V31"] }
evidence: README.md:11 gas compatibility; parse_reg_num lowercases; llvm-mc accepts V0/V31
```
