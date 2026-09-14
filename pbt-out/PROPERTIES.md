# Properties: encode_neon_qshrn

## encode_neon_qshrn_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected (pure function, no lifecycle). Round-trip rejected (no in-tree QSHRN decoder). encode_neon_shrn / encode_neon_sqshrun / encode_neon_scalar_qshrn rejected by same-job sibling gate (SHRN opcode 100001 vs 100101; SQSHRUN signed-to-unsigned; scalar vs vector). Doc evidence: README.md:5-14 gas-compatible assembly; README.md:229 NEON narrow list; encoder/mod.rs:1-7 32-bit words; encoder/mod.rs:637-648 dispatch; ARM ARM Advanced SIMD shift by immediate SQSHRN/UQSHRN/SQRSHRN/UQRSHRN; llvm-mc `-triple=aarch64`.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_diff_llvm_mc; encode_neon_sli_pbt::encode_neon_sli_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high,u_bit ∈ {0,1}, is_rounding ∈ {false,true}, shift ∈ {1..dest_esize(ta)}. encode_neon_qshrn([Vd.Tb, Vn.Ta, Imm(shift)], u_bit, is_rounding, is_high) = llvm-mc(mnem Vd.Tb, Vn.Ta, #shift) where Tb = mandated_tb(ta, is_high), dest_esize(8h)=8, dest_esize(4s)=16, dest_esize(2d)=32, mnem ∈ {sqshrn,sqshrn2,uqshrn,uqshrn2,sqrshrn,sqrshrn2,uqrshrn,uqrshrn2}.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_qshrn
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, u_bit, is_rounding, shift]
  domain:
    rd: 0..31
    rn: 0..31
    ta: {8h, 4s, 2d}
    is_high: bool
    u_bit: 0..1
    is_rounding: bool
    shift: 1..dest_esize(ta)
  relation:
    op: eq
    lhs: encode_neon_qshrn(vd_tb_vn_ta_imm(rd, rn, ta, is_high, shift), u_bit, is_rounding, is_high)
    rhs: llvm_mc(qshrn_asm(rd, rn, ta, is_high, u_bit, is_rounding, shift))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: src/backend/arm/assembler/README.md:5-14 README.md:229 encoder/mod.rs:637-648 ARM ARM Advanced SIMD shift by immediate SQSHRN
```

## encode_neon_qshrn_metamorphic_q
- Tier: 4
- Rationale: ARM ARM Q bit is the sole SQSHRN vs SQSHRN2 distinguisher of otherwise-identical encodings. Stronger differential covers full-word agreement; this metamorphic isolates Q. Round-trip rejected (no decoder). Doc evidence: ARM ARM `0 Q U 011110 immh immb opcode Rn Rd`; dispatch is_high => Q.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_q_bit_is_high
- Formal: ∀ rd,rn ∈ {0..31}, ta ∈ {8h,4s,2d}, u_bit ∈ {0,1}, is_rounding ∈ {false,true}, shift ∈ {1..dest_esize(ta)}. encode_neon_qshrn(..., is_high=false) XOR encode_neon_qshrn(..., is_high=true) = 1<<30 at equal other fields.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_qshrn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, ta, u_bit, is_rounding, shift]
  domain:
    rd: 0..31
    rn: 0..31
    ta: {8h, 4s, 2d}
    u_bit: 0..1
    is_rounding: bool
    shift: 1..dest_esize(ta)
  relation:
    op: eq
    lhs: encode_neon_qshrn(ops, u_bit, is_rounding, false) XOR encode_neon_qshrn(ops, u_bit, is_rounding, true)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: ARM ARM Advanced SIMD shift by immediate Q at bit 30 encoder/mod.rs:637-648
```

## encode_neon_qshrn_metamorphic_u_round
- Tier: 4
- Rationale: ARM ARM U bit selects signed vs unsigned saturating; opcode bit 11 selects rounding (100101 vs 100111). Stronger differential covers full-word agreement; this metamorphic isolates U and rounding. Round-trip rejected (no decoder). Doc evidence: ARM ARM U at bit 29, opcode 100101/100111; dispatch u_bit / is_rounding.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_u_bit
- Formal: ∀ rd,rn ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high ∈ {false,true}, shift ∈ {1..dest_esize(ta)}. encode(..., u=0) XOR encode(..., u=1) = 1<<29 at equal other fields; encode(..., is_rounding=false) XOR encode(..., is_rounding=true) = 1<<11 at equal other fields.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_qshrn
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, shift]
  domain:
    rd: 0..31
    rn: 0..31
    ta: {8h, 4s, 2d}
    is_high: bool
    shift: 1..dest_esize(ta)
  relation:
    op: holds
    expr: (encode(u=0) XOR encode(u=1) == 1<<29) AND (encode(round=false) XOR encode(round=true) == 1<<11)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: ARM ARM Advanced SIMD shift by immediate U at bit 29 opcode 100101 vs 100111 encoder/mod.rs:637-648
```

## encode_neon_qshrn_invariant_arm_fields
- Tier: 4
- Rationale: ARM ARM field layout is an exact structural predicate on every success-path word. Differential is stronger for the whole word; this invariant pins each field so a single-bit drift is localizable. Doc evidence: ARM ARM `0 Q U 011110 immh immb opcode Rn Rd`; shift = 2*esize - UInt(immh:immb).
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_word_layout; encode_neon_sli_pbt::encode_neon_sli_roundtrip_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high,u_bit,is_rounding, shift ∈ {1..dest_esize(ta)}. let w = encode_neon_qshrn(...). (w>>31)&1 = 0 ∧ (w>>30)&1 = Q(is_high) ∧ (w>>29)&1 = u_bit ∧ (w>>23)&0x3F = 0b011110 ∧ UInt((w>>16)&0x7F) = src_esize(ta) - shift ∧ (w>>10)&0x3F = opcode(is_rounding) ∧ (w>>5)&0x1F = rn ∧ w&0x1F = rd.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_qshrn
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, u_bit, is_rounding, shift]
  domain:
    rd: 0..31
    rn: 0..31
    ta: {8h, 4s, 2d}
    is_high: bool
    u_bit: 0..1
    is_rounding: bool
    shift: 1..dest_esize(ta)
  relation:
    op: holds
    expr: arm_fields_match(encode_neon_qshrn(...), rd, rn, ta, is_high, u_bit, is_rounding, shift)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: ARM ARM Advanced SIMD shift by immediate 0 Q U 011110 immh immb opcode Rn Rd
```

## encode_neon_qshrn_neg_shift_oob
- Tier: 4
- Rationale: ARM ARM and llvm-mc pin shift to [1, dest element size]. Sibling encode_neon_shrn (neon.rs:1443-1444) checks `shift > half_bits` with half_bits = source/2. Documented bounds must be sampled at 0, dest_esize, dest_esize+1, source_esize, source_esize+1, -1. Stronger differential does not apply on the invalid domain. Doc evidence: llvm-mc "immediate must be an integer in range [1, 8/16/32]"; ARM ARM shift = 2*esize - UInt(immh:immb) with immh != 0000; neon.rs:1443-1444.
- Seed: encode_neon_sli_pbt oob_t_shift
- Formal: ∀ rd,rn ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high,u_bit,is_rounding, shift ∈ {-1, 0, dest_esize(ta)+1, src_esize(ta), src_esize(ta)+1, 256}. encode_neon_qshrn([Vd.Tb, Vn.Ta, Imm(shift)], ...) is Err. llvm-mc rejects the corresponding assembly.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, ta=8h, tb=8b, shift=9, u_bit=0, is_rounding=false, is_high=false (sqshrn v0.8b, v0.8h, #9)
- Bug report: pbt-out/bug_reports/encode_neon_qshrn_shift_oob.md

```property
function: encoder.neon.encode_neon_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, u_bit, is_rounding, shift]
  domain:
    rd: 0..31
    rn: 0..31
    ta: {8h, 4s, 2d}
    is_high: bool
    u_bit: 0..1
    is_rounding: bool
    shift: oob_shift(ta)
  relation:
    op: throws
    expr: encode_neon_qshrn(vd_tb_vn_ta_imm(rd, rn, ta, is_high, shift), u_bit, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: -16, max: 256, type: i64 }
evidence: ARM ARM SQSHRN shift in 1..dest_esize llvm-mc range diagnostic neon.rs:1443-1444 encode_neon_shrn half_bits
```

## encode_neon_qshrn_neg_dest_tb
- Tier: 4
- Rationale: GNU-style SQSHRN syntax requires Vd.Tb matching Ta and the `2` suffix (Q). llvm-mc rejects mismatched Tb and GPR/FP dest. The helper owns vector sqshrn encoding (dispatch mod.rs:637-648). Stronger differential does not apply on the invalid domain. Doc evidence: README.md:5-14 gas-compatible; llvm-mc "invalid operand"; ARM ARM Tb/Ta pairs.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_dest_tb_must_match / encode_neon_three_diff_narrow_gpr_dest_err
- Formal: ∀ rd,rn ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high,u_bit,is_rounding, shift ∈ {1..dest_esize(ta)}, tb ∉ {mandated_tb(ta,is_high)} ∪ GPR/FP dest. encode_neon_qshrn is Err. llvm-mc rejects the corresponding assembly.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, ta=8h, tb=4h, shift=1, u_bit=0, is_rounding=false, is_high=false (sqshrn v0.4h, v0.8h, #1); also GPR dest x0 (sqshrn x0, v0.8h, #1)
- Bug report: pbt-out/bug_reports/encode_neon_qshrn_mismatched_dest_tb.md; pbt-out/bug_reports/encode_neon_qshrn_gpr_dest.md

```property
function: encoder.neon.encode_neon_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, tb, is_high, u_bit, is_rounding, shift]
  domain:
    rd: 0..31
    rn: 0..31
    ta: {8h, 4s, 2d}
    tb: neon_arr \ {mandated_tb(ta, is_high)}
    is_high: bool
    u_bit: 0..1
    is_rounding: bool
    shift: 1..dest_esize(ta)
  relation:
    op: throws
    expr: encode_neon_qshrn([Vd.tb, Vn.ta, Imm(shift)], u_bit, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  tb: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d"] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: README.md:5-14 llvm-mc invalid operand ARM ARM SQSHRN Tb/Ta pairs encoder/mod.rs:637-648
```

## encode_neon_qshrn_neg_extra_operand
- Tier: 4
- Rationale: ARM ARM and llvm-mc require exactly three operands (Vd.Tb, Vn.Ta, #shift). llvm-mc rejects a fourth. Stronger differential does not apply on the invalid domain. Doc evidence: README.md:5-14 gas-compatible; llvm-mc "invalid operand"; ARM ARM SQSHRN syntax.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_extra_operand_err
- Formal: ∀ rd,rn,extra ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high,u_bit,is_rounding, shift ∈ {1..dest_esize(ta)}. encode_neon_qshrn([Vd.Tb, Vn.Ta, Imm(shift), extra], ...) is Err. llvm-mc rejects the 4-operand assembly.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, ta=8h, tb=8b, shift=1, u_bit=0, is_rounding=false, is_high=false (sqshrn v0.8b, v0.8h, #1, v0.8b)
- Bug report: pbt-out/bug_reports/encode_neon_qshrn_extra_operand.md

```property
function: encoder.neon.encode_neon_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, ta, is_high, u_bit, is_rounding, shift]
  domain:
    rd: 0..31
    rn: 0..31
    extra: 0..31
    ta: {8h, 4s, 2d}
    is_high: bool
    u_bit: 0..1
    is_rounding: bool
    shift: 1..dest_esize(ta)
  relation:
    op: throws
    expr: encode_neon_qshrn([Vd.Tb, Vn.Ta, Imm(shift), extra], u_bit, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: README.md:5-14 llvm-mc invalid operand ARM ARM SQSHRN three-operand syntax
```

## encode_neon_qshrn_neg_arity_src_nonreg
- Tier: 4
- Rationale: Body documents `qshrn requires 3 operands` (neon.rs:1497) and `unsupported source` unless Ta ∈ {8h,4s,2d} (neon.rs:1500-1502); get_imm / get_neon_reg reject non-matching operand kinds. llvm-mc rejects too-few, unsupported Ta, and non-register/non-imm slots. Documented error contract. Doc evidence: neon.rs:1497,1500-1502; get_imm encoder/mod.rs:968-972; get_neon_reg neon.rs:7-20.
- Seed: encode_neon_three_diff_narrow_pbt::encode_neon_three_diff_narrow_arity_err / unsupported_src / non_reg_err
- Formal: ∀ n < 3. encode_neon_qshrn(ops[0..n], ...) is Err. ∀ ta ∉ {8h,4s,2d}. encode_neon_qshrn([Vd.Tb, Vn.ta, Imm(shift)], ...) is Err. ∀ slot ∈ {0,1} with non-RegArrangement, or slot 2 with non-Imm. encode_neon_qshrn is Err. Invalid register names (v32, foo, empty) Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, ta, slot, kind]
  domain:
    n: 0..2
    ta: unsupported_ta
    slot: 0..2
    kind: {Imm, Mem, Symbol, Shift, Cond, Label, bad_name}
  relation:
    op: throws
    expr: encode_neon_qshrn(malformed(n, ta, slot, kind), u_bit, is_rounding, is_high)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  ta: { gen: oneof, values: ["8b", "16b", "4h", "2s", "1d", ""] }
  slot: { gen: int, min: 0, max: 2, type: usize }
evidence: neon.rs:1497 neon.rs:1500-1502 encoder/mod.rs:968-972 neon.rs:7-20
```

## encode_neon_qshrn_neg_shift_i64_trunc
- Tier: 4
- Rationale: Coverage sweep of the `get_imm as u32` cast (neon.rs:1499). ARM ARM / llvm-mc shift is the assembler immediate in [1, dest_esize], not the low 32 bits of an i64. An Imm whose i64 value is outside that range must Err even if `(imm as u32)` lands in 1..=src_esize. Doc evidence: llvm-mc range diagnostic; ARM ARM shift encoding.
- Seed: encode_neon_qshrn_neg_shift_oob
- Formal: ∀ rd,rn ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high,u_bit,is_rounding, shift ∈ {1..dest_esize(ta)}, k ∈ ℤ\{0}. let wide = shift + k*2^32. encode_neon_qshrn([Vd.Tb, Vn.Ta, Imm(wide)], ...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, ta=8h, tb=8b, Imm(4294967297), k=1, u_bit=0, is_rounding=false, is_high=false
- Bug report: pbt-out/bug_reports/encode_neon_qshrn_shift_i64_trunc.md

```property
function: encoder.neon.encode_neon_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, ta, is_high, u_bit, is_rounding, shift, k]
  domain:
    rd: 0..31
    rn: 0..31
    ta: {8h, 4s, 2d}
    is_high: bool
    u_bit: 0..1
    is_rounding: bool
    shift: 1..dest_esize(ta)
    k: int \\ {0}
  relation:
    op: throws
    expr: encode_neon_qshrn(vd_tb_vn_ta_imm(rd, rn, ta, is_high, shift + k*2^32), u_bit, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  k: { gen: int, min: -4, max: 4, type: i64 }
evidence: neon.rs:1499 get_imm as u32 ARM ARM / llvm-mc shift is the assembler immediate
```

## encode_neon_qshrn_neg_reg_source
- Tier: 4
- Rationale: Coverage sweep of get_neon_reg Operand::Reg on the source slot. Source must be Vn.Ta; a bare GPR/FP/V register yields empty arrangement and must Err (neon.rs:1500-1502 unsupported source). Doc evidence: ARM ARM Vn.Ta; llvm-mc invalid operand; neon.rs:1500-1502.
- Seed: encode_neon_qshrn_neg_arity_src_nonreg
- Formal: ∀ rd ∈ {0..31}, prefix ∈ {x,w,d,s,q,h,b,v}, n ∈ {0..31}, ta ∈ {8h,4s,2d}, is_high,u_bit,is_rounding, shift ∈ {1..dest_esize(ta)}. encode_neon_qshrn([Vd.Tb, Reg(prefix n), Imm(shift)], ...) is Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_qshrn
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, prefix, n, ta, is_high, u_bit, is_rounding, shift]
  domain:
    rd: 0..31
    prefix: {x, w, d, s, q, h, b, v}
    n: 0..31
    ta: {8h, 4s, 2d}
    is_high: bool
    u_bit: 0..1
    is_rounding: bool
    shift: 1..dest_esize(ta)
  relation:
    op: throws
    expr: encode_neon_qshrn([Vd.Tb, Reg(prefix n), Imm(shift)], u_bit, is_rounding, is_high)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  ta: { gen: oneof, values: ["8h", "4s", "2d"] }
  is_high: { gen: bool }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  is_rounding: { gen: bool }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: neon.rs:1500-1502 ARM ARM Vn.Ta llvm-mc invalid operand
```
