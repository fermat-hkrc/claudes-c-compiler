# Properties: encode_neon_shift_right

## encode_neon_shift_right_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree SRSHR/SSRA decoder. Sibling encode_neon_ushr/sshr rejected as differential (same-job gate: different opcode 000001 vs 001001/000101/001101). Doc evidence: README.md:5-14 gas-compatible GNU assembly; README.md:228 lists srshr/urshr/ssra/usra/srsra/ursra; ARM ARM AdvSIMD shift by immediate; dispatch mod.rs:607-612.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)], (u,opc) ∈ {(0,0b001001),(1,0b001001),(0,0b000101),(1,0b000101),(0,0b001101),(1,0b001101)}. encode_neon_shift_right([Vd.T, Vn.T, #shift], u, opc) = llvm-mc("{mnem} Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_right
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u_bit, opcode]
  domain: { t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..=esize(t) }
  relation:
    op: eq
    lhs: encode_neon_shift_right([Vd.T, Vn.T, Imm(shift)], u_bit, opcode)
    rhs: llvm_mc("{mnem} Vd.T, Vn.T, #shift")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: src/backend/arm/assembler/README.md:5-14; README.md:228; encoder/mod.rs:607-612; neon.rs:1450-1468
```

## encode_neon_shift_right_metamorphic_u
- Tier: 4c
- Rationale: ARM U bit is bit 29 and is the only field that differs between signed/unsigned twins (srshr/urshr, ssra/usra, srsra/ursra). Stronger differential already used as a separate property. Doc evidence: neon.rs:1451 Format `0 Q U 01111 0 ...`; ARM ARM U=0 signed / U=1 unsigned.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_metamorphic_u_round
- Formal: ∀ valid (rd,rn,T,shift,opc). encode(..., u=0, opc) XOR encode(..., u=1, opc) = 1<<29
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_right
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, opcode]
  domain: { t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..=esize(t) }
  relation:
    op: eq
    lhs: encode_neon_shift_right(ops, 0, opcode) XOR encode_neon_shift_right(ops, 1, opcode)
    rhs: 1 << 29
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: neon.rs:1451; ARM ARM AdvSIMD shift by immediate U bit
```

## encode_neon_shift_right_metamorphic_opcode_q
- Tier: 4c
- Rationale: Q is determined only by dest arrangement (8b/4h/2s => 0; 16b/8h/4s/2d => 1). Opcode occupies bits [15:10] and is the only field that differs across srshr/ssra/srsra at fixed U. Doc evidence: neon.rs:1451,1460,1465-1466; ARM ARM Q and opcode fields.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_metamorphic_q
- Formal: ∀ valid (rd,rn,esize-pair,shift,u). encode(T_Q0) XOR encode(T_Q1) = 1<<30 when esize matches. ∀ valid ops. encode(..., opc_a) XOR encode(..., opc_b) differs only in bits [15:10].
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_right
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, pair, shift, u_bit, opc_a, opc_b]
  domain: { pair: {(8b,16b),(4h,8h),(2s,4s)}, shift: 1..=esize }
  relation:
    op: eq
    lhs: encode(T_lo) XOR encode(T_hi)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  pair: { gen: oneof, args: [["8b","16b"], ["4h","8h"], ["2s","4s"]] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opc_a: { gen: oneof, args: [9, 5, 13] }
  opc_b: { gen: oneof, args: [9, 5, 13] }
evidence: neon.rs:1460 neon_arr_to_q_size; neon.rs:1465-1466 opcode<<10; ARM ARM Q bit 30
```

## encode_neon_shift_right_invariant_arm_fields
- Tier: 4d
- Rationale: ARM field layout of AdvSIMD shift by immediate is an exact structural predicate on every success-path word. Stronger differential already a separate property. Doc evidence: neon.rs:1451 Format `0 Q U 01111 0 immh immb opcode 1 Rn Rd`; ARM ARM `0 Q U 011110 immh immb opcode Rn Rd`.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_invariant_arm_fields
- Formal: ∀ valid inputs. word bit31=0 ∧ Q=q(T) ∧ U=u_bit ∧ bits[28:23]=011110 ∧ immh:immb = 2*esize(T)-shift ∧ bits[15:10]=opcode ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_right
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u_bit, opcode]
  domain: { t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..=esize(t) }
  body: bit31==0 && Q==q(t) && U==u_bit && bits[28:23]==0b011110 && immhb==2*esize(t)-shift && bits[15:10]==opcode && Rn==rn && Rd==rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: neon.rs:1451; ARM ARM AdvSIMD shift by immediate
```

## encode_neon_shift_right_neg_shift_oob
- Tier: 4e
- Rationale: ARM and llvm-mc require shift in [1, esize]; 0 and esize+1 are UNALLOCATED / assembler errors. Bounds 1 and esize are pinned in the valid generator; this property pins 0, esize+1, -1, esize, and large values. Doc evidence: llvm-mc "immediate must be an integer in range [1, esize]"; ARM shift = (2*esize)-UInt(immh:immb) in [1, esize]; neon.rs:1463.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_neg_shift_oob
- Formal: ∀ rd,rn,T,(u,opc), shift ∉ [1, esize(T)]. encode_neon_shift_right([Vd.T, Vn.T, #shift], u, opc) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_right
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u_bit, opcode]
  domain: { t: arrangement, shift: oob_imm }
  relation:
    op: throws
    expr: encode_neon_shift_right([Vd.T, Vn.T, Imm(shift)], u_bit, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: -16, max: 256, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: llvm-mc range [1, esize]; ARM ARM shift in [1, esize]; neon.rs:1463
```

## encode_neon_shift_right_neg_mismatched_t
- Tier: 4e
- Rationale: ARM and llvm-mc require matching dest/source T (`Vd.<T>, Vn.<T>`). SUT discards source arrangement (neon.rs:1458 `(rn, _)`). Documented gas-compatible contract requires rejection. Doc evidence: ARM ARM SSHR etc. `Vd.<T>, Vn.<T>`; llvm-mc "invalid operand" on `srshr v0.8b, v1.4h, #1`; README.md:5-14.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_neg_dest_tb
- Formal: ∀ rd,rn,Td ≠ Ts, Td,Ts ∈ valid T ∪ {1d}, shift ∈ [1, esize(Td)] if Td valid. encode([Vd.Td, Vn.Ts, #shift], u, opc) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, td="8b", ts="16b", shift=1, u_bit=0, opcode=9 (srshr v0.8b, v0.16b, #1)
- Bug report: pbt-out/bug_reports/encode_neon_shift_right_mismatched_t.md

```property
function: encoder.neon.encode_neon_shift_right
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, ts, shift, u_bit, opcode]
  domain: { td: arrangement, ts: arrangement }
  relation:
    op: throws
    expr: encode_neon_shift_right([Vd.Td, Vn.Ts, Imm(shift)], u_bit, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d", "1d"] }
  ts: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d", "1d"] }
  shift: { gen: int, min: 1, max: 8, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: ARM ARM Vd.<T>, Vn.<T>; llvm-mc invalid operand on mismatched T; README.md:5-14
```

## encode_neon_shift_right_neg_extra_operand
- Tier: 4e
- Rationale: ARM and llvm-mc require exactly three operands. SUT only checks `len < 3` (neon.rs:1456). Gas-compatible contract requires rejection of a fourth operand. Doc evidence: ARM ARM three-operand form; llvm-mc rejects 4-operand srshr; README.md:5-14.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_neg_extra_operand
- Formal: ∀ valid 3-op vector plus extra RegArrangement. encode(ops++[extra], u, opc) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="8b", shift=1, u_bit=0, opcode=9 (srshr v0.8b, v0.8b, #1, v0.8b)
- Bug report: pbt-out/bug_reports/encode_neon_shift_right_extra_operand.md

```property
function: encoder.neon.encode_neon_shift_right
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, shift, u_bit, opcode]
  domain: { t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..=esize(t) }
  relation:
    op: throws
    expr: encode_neon_shift_right([Vd.T, Vn.T, Imm(shift), Vextra.T], u_bit, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: ARM ARM three-operand form; llvm-mc rejects extra operand; neon.rs:1456 len<3 only
```

## encode_neon_shift_right_neg_arity_kinds
- Tier: 4e
- Rationale: Fewer than 3 operands, 1d (Q=0 && esize==64 Reserved), GPR/FP dest (not Vd.T), invalid register names, and non-RegArrangement/non-Imm kinds must Err. llvm-mc rejects 1d and GPR dest for the vector form. Doc evidence: ARM Reserved when Q==0 && esize==64; neon.rs:1456 arity; neon.rs:1461-1462 arrangement match omits 1d; get_neon_reg; llvm-mc `srshr v0.1d, v1.1d, #1` error.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_neg_arity_src_nonreg / encode_neon_qshrn_neg_gpr_dest
- Formal: ∀ n<3. encode(ops[..n]) = Err. ∀ T=1d. encode = Err. ∀ dest ∈ GPR/FP names. encode = Err. ∀ invalid names {v32,foo,"",v,v-1}. encode = Err. ∀ non matching Operand kind at a slot. encode = Err.
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shift_right
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, t1d, dest_prefix, bad_name, slot, kind]
  domain: { n: 0..2, dest_prefix: prefix, bad_name: invalid_reg }
  relation:
    op: throws
    expr: encode_neon_shift_right(malformed, u_bit, opcode)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  dest_prefix: { gen: oneof, args: ["x", "w", "d", "s", "q", "h", "b"] }
  bad_name: { gen: oneof, args: ["v32", "v99", "foo", "", "v", "v-1"] }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: ARM Reserved Q=0 esize=64; neon.rs:1456; llvm-mc rejects 1d and GPR dest
```

## encode_neon_shift_right_neg_shift_i64_trunc
- Tier: 4e
- Rationale: Coverage sweep of get_imm as u32 (neon.rs:1459). ARM/llvm-mc shift is the full immediate, not the low 32 bits. An i64 whose low 32 bits land in [1, esize] but whose value is outside that range must Err. Doc evidence: llvm-mc range [1, esize]; Operand::Imm is i64 (parser.rs:24); neon.rs:1459 `get_imm as u32`.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_neg_shift_i64_trunc
- Formal: ∀ valid (rd,rn,T,shift,u,opc), k≠0. encode([Vd.T, Vn.T, Imm(shift + k*2^32)], u, opc) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", shift=1, Imm(4294967297), u_bit=0, opcode=9
- Bug report: pbt-out/bug_reports/encode_neon_shift_right_shift_i64_trunc.md

```property
function: encoder.neon.encode_neon_shift_right
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, u_bit, opcode, k]
  domain: { t: arrangement, shift: 1..=esize, k: nonzero }
  relation:
    op: throws
    expr: encode_neon_shift_right([Vd.T, Vn.T, Imm(shift + k * 2^32)], u_bit, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
  k: { gen: int, min: -4, max: 4, type: i64 }
evidence: neon.rs:1459 get_imm as u32; parser.rs:24 Imm is i64; llvm-mc range [1, esize]
```

## encode_neon_shift_right_neg_reg_source
- Tier: 4e
- Rationale: Coverage sweep of get_neon_reg accepting Operand::Reg (neon.rs:14-18). Vector form requires Vn.T (RegArrangement). A bare GPR/FP/V source must Err. Doc evidence: ARM Vn.<T>; llvm-mc rejects `srshr v0.8b, x0, #1`; get_neon_reg Reg arm.
- Seed: neon.rs encode_neon_qshrn_pbt::encode_neon_qshrn_neg_reg_source
- Formal: ∀ prefix ∈ {x,w,d,s,q,h,b,v}, n ∈ 0..31, valid dest T and shift. encode([Vd.T, Reg(prefix+n), #shift], u, opc) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, prefix="x", n=0, t="8b", shift=1, u_bit=0, opcode=9 (source x0)
- Bug report: pbt-out/bug_reports/encode_neon_shift_right_reg_source.md

```property
function: encoder.neon.encode_neon_shift_right
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, prefix, n, t, shift, u_bit, opcode]
  domain: { prefix: gpr_fp_v, n: 0..31, t: arrangement }
  relation:
    op: throws
    expr: encode_neon_shift_right([Vd.T, Reg(prefix+n), Imm(shift)], u_bit, opcode)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  prefix: { gen: oneof, args: ["x", "w", "d", "s", "q", "h", "b", "v"] }
  n: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, args: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  u_bit: { gen: int, min: 0, max: 1, type: u32 }
  opcode: { gen: oneof, args: [9, 5, 13] }
evidence: neon.rs:14-18 get_neon_reg Operand::Reg; ARM Vn.<T>; llvm-mc rejects bare source
```
