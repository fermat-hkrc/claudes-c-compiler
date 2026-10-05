# Properties: encode_neon_two_misc

## encode_neon_two_misc_diff_llvm_mc
- Tier: 5
- Rationale: Strongest applicable oracle is differential vs llvm-mc, the independent AArch64 assembler this GNU-style encoder claims to match (README.md:12). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree two-misc decoder). Sibling encode_neon_float_two_misc / encode_cnt / encode_neon_not / encode_neon_rev64 / encode_neon_two_misc_narrow / encode_neon_scalar_two_misc rejected by the same-job gate (float / dedicated / narrow / scalar encodings). Matching-T ABS/NEG/CLS/CLZ/REV16/REV32/SQABS/SQNEG with opcode-legal T is the documented public dispatch domain (encoder/mod.rs:324-326, 616-628, 637-645).
- Doc contract: neon.rs:1405 "Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc." — asserted fingerprint 512058a0
- Seed: encode_cnt_pbt.rs:170 encode_cnt_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (mnem,u,opc,T) ∈ matching_T_domain. encode_neon_two_misc([Vd.T, Vn.T], u, opc) = llvm-mc("{mnem} Vd.T, Vn.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_two_misc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, mnem, u_bit, opcode, t]
  domain: { rd: v0_31, rn: v0_31, (mnem,u_bit,opcode,t): matching_T_legal }
  relation:
    op: eq
    lhs: encode_neon_two_misc([RegArrangement(v{rd},t), RegArrangement(v{rn},t)], u_bit, opcode)
    rhs: llvm_mc("{mnem} v{rd}.{t}, v{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }
evidence: neon.rs:1405 README.md:12 encoder/mod.rs:616
```

## encode_neon_two_misc_diff_llvm_mc_pairwise
- Tier: 5
- Rationale: SADDLP/UADDLP/SADALP/UADALP are dispatched to this same symbol (encoder/mod.rs:630-633) with dest Ta mandated by source Tb; size in the ARM encoding is the SOURCE element size, not dest. Differential vs llvm-mc on that pairing is required — a matching-T-only differential never reaches this branch. Same stronger-oracle rejections as the matching-T differential.
- Doc contract: neon.rs:1405 "Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc." — asserted fingerprint 512058a0
- Seed: encode_cnt_pbt.rs:170 (generalized to pairwise dest/src)
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (mnem,u,opc,Tb,Ta) ∈ pairwise_domain. encode_neon_two_misc([Vd.Ta, Vn.Tb], u, opc) = llvm-mc("{mnem} Vd.Ta, Vn.Tb")
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: failing
- Counterexample: encode_neon_two_misc([v0.4h, v0.8b], u_bit=0, opcode=0b00010) → 0x0e602800, llvm-mc saddlp v0.4h, v0.8b = 0x0e202800
- Bug report: bug_reports/encode_neon_two_misc_pairwise_size_from_dest.md

```property
function: encoder.encode_neon_two_misc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, mnem, u_bit, opcode, tb, ta]
  domain: { rd: v0_31, rn: v0_31, (mnem,u_bit,opcode,tb,ta): pairwise_legal }
  relation:
    op: eq
    lhs: encode_neon_two_misc([RegArrangement(v{rd},ta), RegArrangement(v{rn},tb)], u_bit, opcode)
    rhs: llvm_mc("{mnem} v{rd}.{ta}, v{rn}.{tb}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  tb: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s"] }
evidence: neon.rs:1405 encoder/mod.rs:630-633 ARM two-misc SADDLP
```

## encode_neon_two_misc_meta_rd_rn_u_q
- Tier: 4
- Rationale: Documented format neon.rs:1406 places Rd in bits[4:0], Rn in bits[9:5], U in bit 29, Q in bit 30. Changing only one of those inputs must differ only in that field. State machine / round-trip rejected as above. Weaker than the llvm-mc differential but isolates field packing independently of the reference tool.
- Doc contract: neon.rs:1406 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd" — asserted fingerprint cbb51bf8
- Seed: encode_cnt_pbt.rs:183 encode_cnt_meta_rd_rn
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}, ∀ T ∈ {8b,16b}. let w(rd,rn,u,qT)=encode_neon_two_misc([Vd.T,Vn.T],u,ABS_opc). (w(rd1,rn1,0,T) xor w(rd2,rn1,0,T)) & ~0x1F = 0 ∧ (w(rd1,rn1,0,T) xor w(rd1,rn2,0,T)) & ~(0x1F<<5) = 0 ∧ (w(rd1,rn1,0,8b) xor w(rd1,rn1,1,8b)) = 1<<29 ∧ (w(rd1,rn1,0,8b) xor w(rd1,rn1,0,16b)) = 1<<30
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_two_misc
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2]
  domain: { rd1: v0_31, rd2: v0_31, rn1: v0_31, rn2: v0_31 }
  relation:
    op: holds
    expr: rd_rn_u_q_isolation
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1406
```

## encode_neon_two_misc_inv_layout
- Tier: 4
- Rationale: The documented ARM two-misc layout (neon.rs:1406; ARM Advanced SIMD two-register miscellaneous) is an exact bit predicate on every success-path word: bit31=0, Q at 30, U at 29, bits[28:24]=01110, size at [23:22] from dest T, bits[21:17]=10000, opcode at [16:12], bits[11:10]=10, Rn at [9:5], Rd at [4:0]. Stronger oracles rejected as above.
- Doc contract: neon.rs:1406 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd" — asserted fingerprint cbb51bf8
- Seed: encode_cnt_pbt.rs:210 encode_cnt_inv_layout
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (u,opc,T) ∈ matching_T_legal. let w=encode_neon_two_misc([Vd.T,Vn.T],u,opc). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=u ∧ w[28:24]=01110 ∧ w[23:22]=size(T) ∧ w[21:17]=10000 ∧ w[16:12]=opc ∧ w[11:10]=10 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_two_misc
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, u_bit, opcode, t]
  domain: { rd: v0_31, rn: v0_31, (u_bit,opcode,t): matching_T_legal }
  relation:
    op: eq
    lhs: encode_neon_two_misc([Vd.T,Vn.T], u_bit, opcode)
    rhs: arm_two_misc_word(rd, rn, u_bit, opcode, t)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1406 ARM two-register miscellaneous
```

## encode_neon_two_misc_neg_arity
- Tier: 3
- Rationale: get_neon_reg on a missing operand returns Err ("expected NEON register at operand N, got None"). llvm-mc rejects abs/neg with fewer than two operands. Documented two-operand form Vd.T, Vn.T (README.md:225). Negative/error contract for arity < 2.
- Doc contract: neon.rs:1405 "Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc." — asserted fingerprint 512058a0
- Seed: encode_cnt_pbt.rs:228 encode_cnt_neg_arity
- Formal: ∀ n ∈ {0,1}, ∀ rd ∈ {0..31}, ∀ T ∈ {8b,16b}. encode_neon_two_misc(ops[0..n], 0, ABS_opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, t]
  domain: { n: 0..1, rd: v0_31, t: 8b_or_16b }
  relation:
    op: throws
    expr: encode_neon_two_misc(ops[0..n], 0, 0b01011)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 1, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1408-1409 get_neon_reg README.md:225
```

## encode_neon_two_misc_neg_extra
- Tier: 3
- Rationale: llvm-mc/gas reject a third operand on ABS/NEG/CLS/SADDLP (README.md:12 same textual assembly as gas). The public assembler contract therefore requires Err. The helper is caller-reachable with operands passed through (encoder/mod.rs:616).
- Doc contract: neon.rs:1405 "Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc." — asserted fingerprint 512058a0
- Seed: encode_cnt_pbt.rs:241 encode_cnt_neg_extra
- Formal: ∀ rd,rn,extra ∈ {0..31}, ∀ T ∈ matching_T(ABS). llvm-mc("abs Vd.T, Vn.T, Vextra.T") = Err ∧ encode_neon_two_misc([Vd.T,Vn.T,Vextra.T], 0, ABS_opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: failing
- Counterexample: encode_neon_two_misc([v0.8b, v0.8b, v0.8b], 0, 0b01011) = Ok(Word) while llvm-mc("abs v0.8b, v0.8b, v0.8b") = Err
- Bug report: bug_reports/encode_neon_two_misc_extra_operand.md

```property
function: encoder.encode_neon_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t]
  domain: { rd: v0_31, rn: v0_31, extra: v0_31, t: abs_legal }
  relation:
    op: throws
    expr: encode_neon_two_misc([Vd.T,Vn.T,Vextra.T], 0, 0b01011)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1405 README.md:12 llvm-mc rejects third operand
```

## encode_neon_two_misc_neg_mismatch
- Tier: 3
- Rationale: Matching-T two-misc instructions require dest and source arrangements to be identical; llvm-mc rejects abs Vd.8b, Vn.16b. Source arrangement is discarded by the SUT (`let (rn, _)`), so this is the documented gas contract, not a guessed restriction.
- Doc contract: neon.rs:1405 "Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc." — asserted fingerprint 512058a0
- Seed: encode_cnt_pbt.rs:280 encode_cnt_neg_mismatch_t
- Formal: ∀ rd,rn ∈ {0..31}, ∀ Td ≠ Tn ∈ abs_legal. llvm-mc("abs Vd.Td, Vn.Tn") = Err ∧ encode_neon_two_misc([Vd.Td,Vn.Tn], 0, ABS_opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: failing
- Counterexample: encode_neon_two_misc([v0.8b, v0.16b], 0, 0b01011) = Ok(Word) while llvm-mc("abs v0.8b, v0.16b") = Err
- Bug report: bug_reports/encode_neon_two_misc_mismatch_t.md

```property
function: encoder.encode_neon_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn]
  domain: { rd: v0_31, rn: v0_31, td: abs_legal, tn: abs_legal, td != tn }
  relation:
    op: throws
    expr: encode_neon_two_misc([Vd.Td,Vn.Tn], 0, 0b01011)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1405 README.md:12 llvm-mc rejects mismatched T
```

## encode_neon_two_misc_neg_reserved_t
- Tier: 3
- Rationale: ARM two-misc reserves opcode-specific arrangements that neon_arr_to_q_size still accepts: ABS/NEG 1D (Q=0 size=11 reserved for vector ABS), CLS/CLZ 2D (size=11 reserved), REV16 not-byte, REV32 not-byte/half. llvm-mc rejects those; the GNU-style assembler contract requires Err. 1D is in neon_arr_to_q_size's documented domain so it is not an input-domain restriction of this function — it is a known ARM reserved encoding the encoder still emits.
- Doc contract: neon.rs:1406 "Format: 0 Q U 01110 size 10000 opcode 10 Rn Rd" — asserted fingerprint cbb51bf8
- Seed: encode_cnt_pbt.rs:263 encode_cnt_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (mnem,u,opc,T) ∈ reserved_T_domain. llvm-mc("{mnem} Vd.T, Vn.T") = Err ∧ encode_neon_two_misc([Vd.T,Vn.T], u, opc) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: failing
- Counterexample: encode_neon_two_misc([v0.1d, v0.1d], 0, 0b01011) = Ok(Word) while llvm-mc("abs v0.1d, v0.1d") = Err
- Bug report: bug_reports/encode_neon_two_misc_reserved_t.md

```property
function: encoder.encode_neon_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, mnem, u_bit, opcode, t]
  domain: { rd: v0_31, rn: v0_31, (mnem,u_bit,opcode,t): reserved_T }
  relation:
    op: throws
    expr: encode_neon_two_misc([Vd.T,Vn.T], u_bit, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1406 ARM two-misc reserved size/Q llvm-mc rejects
```

## encode_neon_two_misc_diff_alt_spellings
- Tier: 5
- Rationale: Strengthening / sweep: parse_reg_num lowercases V prefixes; uppercase Vd.T must still match llvm-mc. Same differential oracle as matching-T.
- Doc contract: neon.rs:1405 "Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc." — asserted fingerprint 512058a0
- Seed: encode_cnt_pbt.rs:353 encode_cnt_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, ∀ (mnem,u,opc,T) ∈ matching_T_domain. encode_neon_two_misc([V{rd}.T, V{rn}.T], u, opc) = llvm-mc("{mnem} V{rd}.T, V{rn}.T")
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_two_misc
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, mnem, u_bit, opcode, t]
  domain: { rd: v0_31, rn: v0_31, (mnem,u_bit,opcode,t): matching_T_legal }
  relation:
    op: eq
    lhs: encode_neon_two_misc([RegArrangement(V{rd},t), RegArrangement(V{rn},t)], u_bit, opcode)
    rhs: llvm_mc("{mnem} V{rd}.{t}, V{rn}.{t}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1405 README.md:12 parse_reg_num lowercases
```

## encode_neon_two_misc_neg_nonreg
- Tier: 3
- Rationale: Strengthening / sweep: Imm/Mem/Label/GPR/bare-V at dest or src must Err (get_neon_reg other / empty arrangement). llvm-mc rejects the same forms.
- Doc contract: neon.rs:1405 "Encode NEON two-reg misc: ABS, NEG, CLS, CLZ, etc." — asserted fingerprint 512058a0
- Seed: encode_cnt_pbt.rs:297 encode_cnt_neg_gpr_bare_sp
- Formal: ∀ kind ∈ {Imm dest, Imm src, Mem dest, Label src, GPR dest, bare V}. encode_neon_two_misc(ops(kind), 0, ABS_opc) = Err ∧ llvm-mc(asm(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_two_misc_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_two_misc
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, kind]
  domain: { rd: v0_31, rn: v0_31, t: abs_legal, kind: nonreg }
  relation:
    op: throws
    expr: encode_neon_two_misc(ops(kind), 0, 0b01011)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1408 get_neon_reg README.md:12
```
