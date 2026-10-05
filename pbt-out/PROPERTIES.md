# Properties: encode_neon_three_same

## encode_neon_three_same_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is Differential vs llvm-mc (independent LLVM assembler of the same GNU-style AArch64 text). State machine rejected — pure function, no lifecycle. Algebraic round-trip rejected — no in-tree three-same decoder. Sibling encode_neon_add_sub / encode_neon_mul rejected as primary (independence: shared get_neon_reg / neon_arr_to_q_size; same-job gate for MUL-only). README.md:12 claims gas-compatible encodings. Domain restricted to integer three-same mnemonics whose ARM T includes 8B/16B/4H/8H/2S/4S/2D (cmeq, cmhi, cmhs, cmge, cmgt, cmtst, sqadd, uqadd, sqsub, uqsub, sshl, ushl, sqshl, uqshl, srshl, urshl, sqrshl, uqrshl, addp).
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:58 "Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc." — asserted fingerprint 10f55808
- Seed: encode_neon_add_sub_pbt.rs:206 encode_neon_add_sub_diff_llvm_mc
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, (mnemonic,U,opcode) ∈ InsnTable. encode_neon_three_same([Vd.T,Vn.T,Vm.T], U, opcode) = llvm-mc("mnemonic Vd.T, Vn.T, Vm.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_three_same
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, t: three_same_t, insn: insn_table }
  relation:
    op: eq
    lhs: encode_neon_three_same([Vd.T, Vn.T, Vm.T], insn.u, insn.opcode)
    rhs: llvm_mc(insn.mnemonic + " Vd.T, Vn.T, Vm.T")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  insn: { gen: oneof, values: "InsnTable" }
evidence: src/backend/arm/assembler/encoder/neon.rs:58
```

## encode_neon_three_same_invariant_arm_fields
- Tier: 4
- Rationale: Doc layout at neon.rs:60 is an asserted bit-field contract independent of llvm-mc. Weaker than differential; kept as a structural check that Rd/Rn/Rm/Q/U/size/opcode land in the documented positions. u_bit domain {0,1} and opcode bits 15-11 from the function doc.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:60 "Layout: 0 Q U 01110 size 1 Rm opcode 1 Rn Rd" — asserted fingerprint 10f55808
- Seed: encode_neon_add_sub_pbt.rs:269 encode_neon_add_sub_invariant_arm_fields
- Formal: ∀ rd,rn,rm ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, u ∈ {0,1}, opcode ∈ {0..31}. let w = encode_neon_three_same([Vd.T,Vn.T,Vm.T], u, opcode). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=u ∧ w[28:24]=01110 ∧ w[23:22]=size(T) ∧ w[21]=1 ∧ w[20:16]=rm ∧ w[15:11]=opcode ∧ w[10]=1 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_three_same
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, rm, t, u, opcode]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, t: three_same_t, u: u_bit, opcode: opcode5 }
  body: word_fields_match_arm_layout(encode_neon_three_same([Vd.T,Vn.T,Vm.T], u, opcode), rd, rn, rm, t, u, opcode)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  u: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:60
```

## encode_neon_three_same_metamorphic_u_opcode_q
- Tier: 4
- Rationale: ARM three-same isolates U at bit 29, opcode at [15:11], Q at bit 30. Metamorphic XOR of paired encodings. Same-esize Q pair is 8b vs 16b (size=00). Weaker than differential; independently checks field isolation the doc asserts.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:60 "Layout: 0 Q U 01110 size 1 Rm opcode 1 Rn Rd" — asserted fingerprint 10f55808
- Seed: neon.rs:4381 encode_neon_float_three_same_metamorphic_u_bit
- Formal: ∀ rd,rn,rm ∈ {0..31}, opcode1,opcode2 ∈ {0..31}. encode(..., u=0, opcode1) XOR encode(..., u=1, opcode1) = 1<<29. encode(..., u=0, opcode1) XOR encode(..., u=0, opcode2) differs only in bits[15:11]. encode([Vd.8b,...], 0, opcode1) XOR encode([Vd.16b,...], 0, opcode1) = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_three_same
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, rm, opcode1, opcode2]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, opcode1: opcode5, opcode2: opcode5 }
  body: xor_u_is_bit29 AND xor_opcode_only_bits_15_11 AND xor_8b_16b_is_bit30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  opcode1: { gen: int, min: 0, max: 31, type: u32 }
  opcode2: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:60
```

## encode_neon_three_same_neg_arity
- Tier: 4
- Rationale: neon.rs:66-67 if operands.len() < 3 return Err. llvm-mc rejects too-few-operand three-same. Documented error contract.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:67 "NEON three-same requires 3 operands" — asserted fingerprint 10f55808
- Seed: encode_neon_add_sub_pbt.rs:296 encode_neon_add_sub_neg_arity
- Formal: ∀ n ∈ {0,1,2}, ops prefix of length n of a valid three-same triple. encode_neon_three_same(ops, u, opcode) = Err ∧ llvm-mc(truncated asm) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, rm, insn]
  domain: { n: 0..2, rd: v0_v31, rn: v0_v31, rm: v0_v31, insn: insn_table }
  relation:
    op: holds
    expr: encode_neon_three_same(ops.take(n), insn.u, insn.opcode).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:67
```

## encode_neon_three_same_neg_extra_operand
- Tier: 4
- Rationale: GNU/llvm-mc three-same is exactly 3 operands; a 4th is "invalid operand". README.md:12 gas-compatible. The SUT only checks len < 3, so extras are ignored — that is the candidate bug. Domain includes a 4th matching V register; do not shrink the generator to the passing 3-operand case.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:58 "Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc." — asserted fingerprint 10f55808
- Seed: encode_neon_add_sub_pbt.rs:325 encode_neon_add_sub_neg_extra_operand
- Formal: ∀ rd,rn,rm,extra ∈ {0..31}, T ∈ three_same_t, insn ∈ InsnTable. llvm-mc(mnemonic Vd.T,Vn.T,Vm.T,Vextra.T) = Err ⇒ encode_neon_three_same([Vd.T,Vn.T,Vm.T,Vextra.T], U, opcode) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_same([v0.8b, v0.8b, v0.8b, v0.8b], u=1, opcode=0b10001) — cmeq v0.8b, v0.8b, v0.8b, v0.8b
- Bug report: pbt-out/bug_reports/encode_neon_three_same_extra_operand.md

```property
function: encoder.encode_neon_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, extra, t, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, extra: v0_v31, t: three_same_t, insn: insn_table }
  relation:
    op: holds
    expr: encode_neon_three_same([Vd.T,Vn.T,Vm.T,Vextra.T], insn.u, insn.opcode).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:58
```

## encode_neon_three_same_neg_invalid_t
- Tier: 4
- Rationale: ARM three-same requires matching T; 1D (size:Q=11:0) is Reserved. llvm-mc rejects mismatched T, 1d, and unknown arrangements. neon_arr_to_q_size accepts 1d and the encoder discards Rn/Rm arrangements — candidate bugs. Domain is any (Td,Tn,Tm) that is not a matching valid T.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:58 "Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc." — asserted fingerprint 10f55808
- Seed: encode_neon_add_sub_pbt.rs:346 encode_neon_add_sub_neg_invalid_t
- Formal: ∀ rd,rn,rm ∈ {0..31}, Td,Tn,Tm ∈ Arr ∪ {1d,1q}, insn ∈ InsnTable. ¬(valid_T(Td) ∧ Td=Tn=Tm) ⇒ llvm-mc rejects ∧ encode_neon_three_same([Vd.Td,Vn.Tn,Vm.Tm], U, opcode) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_same([v0.8b, v0.16b, v0.8b], u=1, opcode=0b10001)
- Bug report: pbt-out/bug_reports/encode_neon_three_same_mismatched_t.md

```property
function: encoder.encode_neon_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, td, tn, tm, insn]
  domain: { rd: v0_v31, tn: any_arr, tm: any_arr, td: any_arr, insn: insn_table }
  body: not(valid_matching_t(td,tn,tm)) => encode_neon_three_same(ops, insn.u, insn.opcode).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  tn: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
  tm: { gen: oneof, values: ["8b", "16b", "4h", "8h", "2s", "4s", "1d", "2d", "1q"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:58
```

## encode_neon_three_same_neg_gpr_or_bare
- Tier: 4
- Rationale: llvm-mc rejects GPR dest (x0/w0), bare V without arrangement, and FP scalar names as three-same operands. parse_reg_num accepts x/w/d/s/q/h/b prefixes; get_neon_reg accepts Operand::Reg. Dest as Operand::Reg yields empty arrangement which neon_arr_to_q_size rejects; source as Operand::Reg is discarded — candidate bug for Rn/Rm.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:58 "Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc." — asserted fingerprint 10f55808
- Seed: encode_neon_add_sub_pbt.rs:375 encode_neon_add_sub_neg_gpr_or_bare
- Formal: ∀ rd,rn,rm ∈ {0..31}, kind ∈ {gpr_dest, bare_vn, bare_vm, fp_dest}. llvm-mc rejects the corresponding asm ⇒ encode_neon_three_same(ops, U, opcode) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_same([v0.8b, Reg("v0"), v0.8b], u=1, opcode=0b10001)
- Bug report: pbt-out/bug_reports/encode_neon_three_same_bare_src.md

```property
function: encoder.encode_neon_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, kind, prefix, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, kind: gpr_or_bare, prefix: xwdqshb, insn: insn_table }
  body: encode_neon_three_same(malformed_ops(kind), insn.u, insn.opcode).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
evidence: src/backend/arm/assembler/encoder/neon.rs:58
```

## encode_neon_three_same_neg_invalid_name_nonreg
- Tier: 4
- Rationale: Invalid NEON names (v32, v99, foo, empty, v, v-1) and non-register operand kinds (Imm/Mem/Symbol/Shift/Cond) must Err. get_neon_reg returns Err for other kinds and parse_reg_num returns None for out-of-range / non-prefix names. llvm-mc also rejects these.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:58 "Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc." — asserted fingerprint 10f55808
- Seed: encode_neon_add_sub_pbt.rs (invalid name / nonreg follow-on)
- Formal: ∀ idx ∈ {0,1,2}, name ∈ InvalidNames ∪ NonRegKinds, rest valid. encode_neon_three_same(ops_with_slot(idx)=bad, U, opcode) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [slot, bad, insn]
  domain: { slot: 0..2, bad: invalid_name_or_nonreg, insn: insn_table }
  body: encode_neon_three_same(ops_with_bad_slot, insn.u, insn.opcode).is_err()
expected_error: String
generators:
  slot: { gen: int, min: 0, max: 2, type: usize }
  bad: { gen: oneof, values: ["v32", "v99", "foo", "", "v", "v-1", "Imm", "Mem", "Symbol"] }
evidence: src/backend/arm/assembler/encoder/neon.rs:58
```

## encode_neon_three_same_neg_reserved_1d
- Tier: 4
- Rationale: ARM ARM Advanced SIMD three-same reserves size:Q=11:0 (.1d). llvm-mc rejects matching .1d. neon_arr_to_q_size accepts 1d. Split from invalid_t so the reserved-encoding witness has its own property/bug pair.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:58 "Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc." — asserted fingerprint 10f55808
- Seed: encode_neon_three_same_pbt.rs test_encode_neon_three_same_regression_reserved_1d
- Formal: ∀ rd,rn,rm ∈ {0..31}, insn ∈ InsnTable. encode_neon_three_same([Vd.1d,Vn.1d,Vm.1d], U, opcode) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_same([v0.1d, v0.1d, v0.1d], u=1, opcode=0b10001)
- Bug report: pbt-out/bug_reports/encode_neon_three_same_reserved_1d.md

```property
function: encoder.encode_neon_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, rm, insn]
  domain: { rd: v0_v31, rn: v0_v31, rm: v0_v31, t: const_1d, insn: insn_table }
  relation:
    op: holds
    expr: encode_neon_three_same([Vd.1d,Vn.1d,Vm.1d], insn.u, insn.opcode).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  rm: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:58
```

## encode_neon_three_same_neg_gpr_dest
- Tier: 4
- Rationale: llvm-mc rejects GPR dest x0.8b. parse_reg_num accepts x prefix. Split from gpr_or_bare so the GPR-dest witness has its own property/bug pair.
- Doc contract: src/backend/arm/assembler/encoder/neon.rs:58 "Encode NEON three-same-register instructions: CMEQ, UQSUB, SQSUB, CMHI, etc." — asserted fingerprint 10f55808
- Seed: encode_neon_three_same_pbt.rs test_encode_neon_three_same_regression_gpr_dest
- Formal: ∀ rd ∈ {0..31}. encode_neon_three_same([RegArrangement{x{rd},8b}, v0.8b, v0.8b], U, opcode) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_three_same_pbt.rs
- Status: failing
- Counterexample: encode_neon_three_same([RegArrangement{reg:"x0", arrangement:"8b"}, v0.8b, v0.8b], u=1, opcode=0b10001)
- Bug report: pbt-out/bug_reports/encode_neon_three_same_gpr_dest.md

```property
function: encoder.encode_neon_three_same
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd]
  domain: { rd: v0_v31 }
  relation:
    op: holds
    expr: encode_neon_three_same([RegArrangement{x{rd},8b}, v0.8b, v0.8b], 1, 0b10001).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: src/backend/arm/assembler/encoder/neon.rs:58
```
