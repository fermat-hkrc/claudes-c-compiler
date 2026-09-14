# Properties: encode_neon_shift_imm

## encode_neon_shift_imm_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc (independent AArch64 assembler of the same GNU-style USHR text). State machine rejected: pure function, no lifecycle. Round-trip rejected: no in-tree USHR decoder. encode_neon_ushr rejected (independence gate: near-copy of this body). encode_neon_sshr rejected (same-job gate: SSHR / U=0). SUT-boundary: internal-helper of the GNU-style AArch64 assembler; mapping operands <-> `ushr Vd.T, Vn.T, #shift`.
- Seed: (none)
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_shift_imm([Vd.T, Vn.T, Imm(shift)], true) = Word(v) ∧ llvm-mc(-triple=aarch64, "ushr Vd.T, Vn.T, #shift") = v
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shift_imm
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: 0..31, rn: 0..31, t: neon_t, shift: 1..esize(t) }
  relation:
    op: eq
    lhs: encode_neon_shift_imm(ops, true)
    rhs: llvm_mc_word("ushr Vd.T, Vn.T, #shift")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: src/backend/arm/assembler/README.md:14 same textual assembly as gas; README.md:228 ushr under NEON shifts; neon.rs:372 Encode NEON USHR; ARM ARM Advanced SIMD shift by immediate USHR
```

## encode_neon_shift_imm_metamorphic_q
- Tier: 4c
- Rationale: ARM ARM Q is bit 30 of Advanced SIMD shift by immediate; same-esize Q=0 vs Q=1 arrangements (8b/16b, 4h/8h, 2s/4s) must differ only in Q. Stronger differential already used on the valid domain; this is an independent field metamorphic (required metamorphic/differential companion).
- Seed: (none)
- Formal: ∀ rd, rn ∈ {0..31}, (Tlo, Thi) ∈ {(8b,16b),(4h,8h),(2s,4s)}, shift ∈ [1, esize(Tlo)]. encode(Tlo) XOR encode(Thi) = 1<<30
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shift_imm
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd, rn, t_lo, t_hi, shift]
  domain: { pair: q_pairs, shift: 1..esize(t_lo) }
  relation:
    op: eq
    lhs: encode_neon_shift_imm(ops_lo, true) XOR encode_neon_shift_imm(ops_hi, true)
    rhs: 1 << 30
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_lo: { gen: oneof, items: ["8b", "4h", "2s"] }
  shift: { gen: int, min: 1, max: 32, type: i64 }
evidence: ARM ARM Advanced SIMD shift by immediate Q at bit 30; neon.rs:381 neon_arr_to_q_size; neon.rs:400 q << 30
```

## encode_neon_shift_imm_invariant_arm_fields
- Tier: 4d
- Rationale: ARM ARM field layout of USHR is an exact structural predicate on every success-path word. Stronger differential already covers value equality vs llvm-mc; this pins each field independently.
- Seed: (none)
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. word bit31=0 ∧ Q=q(T) ∧ U=1 ∧ bits[28:23]=011110 ∧ immh:immb=2*esize-shift ∧ bits[15:10]=000001 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shift_imm
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { t: neon_t, shift: 1..esize(t) }
  relation:
    op: holds
    expr: arm_ushr_fields(word)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: ARM ARM Advanced SIMD shift by immediate USHR 0 Q 1 011110 immh immb 00000 1 Rn Rd; neon.rs:383-400
```

## encode_neon_shift_imm_neg_shift_oob
- Tier: 4e
- Rationale: ARM ARM and llvm-mc require shift in [1, esize]; llvm-mc rejects 0 / esize+1 / negative. Documented error contract is Err (GNU assembler rejects). Bounds 0, 1, esize, esize+1 are pinned by the generator.
- Seed: (none)
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∉ [1, esize(T)]. llvm-mc("ushr … #shift") errors ∧ encode_neon_shift_imm([Vd.T, Vn.T, Imm(shift)], true) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", shift=-1 (debug panic: attempt to subtract with overflow at neon.rs:390). Related: shift=0 and shift=9 encode Ok(Word) via wrap/mask.
- Bug report: pbt-out/bug_reports/encode_neon_shift_imm_shift_oob.md

```property
function: encode_neon_shift_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { t: neon_t, shift: not_in_1_esize }
  relation:
    op: throws
    expr: encode_neon_shift_imm(ops, true)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: -16, max: 256, type: i64 }
evidence: ARM ARM shift in [1, esize]; llvm-mc "immediate must be an integer in range [1, esize]"; README.md:14 gas-compatible
```

## encode_neon_shift_imm_neg_extra_operand
- Tier: 4e
- Rationale: USHR is a three-operand instruction. llvm-mc rejects a fourth operand. Documented GNU-style assembler contract requires Err, not silent ignore (`len < 3` only).
- Seed: (none)
- Formal: ∀ rd, rn, extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. llvm-mc four-operand ushr errors ∧ encode_neon_shift_imm([Vd.T, Vn.T, Imm(shift), Vextra.T], true) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, extra=0, t="8b", shift=1 (`ushr v0.8b, v0.8b, #1, v0.8b`)
- Bug report: pbt-out/bug_reports/encode_neon_shift_imm_extra_operand.md

```property
function: encode_neon_shift_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, shift]
  domain: { t: neon_t, shift: 1..esize(t) }
  relation:
    op: throws
    expr: encode_neon_shift_imm([Vd.T, Vn.T, Imm(shift), Vextra.T], true)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: llvm-mc rejects extra operand; README.md:14 gas-compatible; ARM ARM USHR three operands
```

## encode_neon_shift_imm_neg_mismatched_t
- Tier: 4e
- Rationale: ARM ARM USHR requires Vd and Vn the same arrangement T. llvm-mc rejects mismatched T. Source arrangement must not be discarded.
- Seed: (none)
- Formal: ∀ rd, rn ∈ {0..31}, Td ≠ Ts ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(Td)]. llvm-mc("ushr Vd.Td, Vn.Ts, #shift") errors ∧ encode_neon_shift_imm([Vd.Td, Vn.Ts, Imm(shift)], true) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, td="8b", ts="16b", shift=1 (`ushr v0.8b, v0.16b, #1`)
- Bug report: pbt-out/bug_reports/encode_neon_shift_imm_mismatched_t.md

```property
function: encode_neon_shift_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, ts, shift]
  domain: { td != ts, shift: 1..esize(td) }
  relation:
    op: throws
    expr: encode_neon_shift_imm([Vd.Td, Vn.Ts, Imm(shift)], true)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  ts: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: ARM ARM USHR Vd.T, Vn.T same T; llvm-mc "invalid operand" on mismatched T; README.md:14
```

## encode_neon_shift_imm_neg_arity_kinds
- Tier: 4e
- Rationale: Documented three-operand Vd.T, Vn.T, #imm form. llvm-mc rejects fewer than 3 operands, T=1d (Reserved Q=0 && esize==64), GPR/FP dest, invalid register names, and non-RegArrangement kinds at dest. Exact failure is Err.
- Seed: (none)
- Formal: ∀ ops with len<3 ∨ T=1d ∨ dest not Vd.T ∨ dest name invalid. encode_neon_shift_imm(ops, true) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encode_neon_shift_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn, dest_kind]
  domain: { n: 0..2, T: 1d or dest not Vd.T }
  relation:
    op: throws
    expr: encode_neon_shift_imm(ops, true)
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: llvm-mc too few operands / invalid operand / 1d reserved; ARM ARM Q=0 && esize==64 Reserved; neon.rs:374 ushr requires 3 operands
```

## encode_neon_shift_imm_neg_shift_i64_trunc
- Tier: 4e
- Rationale: Coverage sweep. get_imm returns i64; the body uses `shift as u32` (neon.rs:389-394). llvm-mc / ARM ARM require the full i64 immediate in [1, esize], not the truncated low 32 bits. Documented error contract is Err.
- Seed: (none)
- Formal: ∀ rd, rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, s ∈ [1, esize(T)], k ≠ 0. encode_neon_shift_imm([Vd.T, Vn.T, Imm(s + k·2^32)], true) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, rn=0, t="8b", shift=1, k=1 (Imm(4294967297) encodes as #1)
- Bug report: pbt-out/bug_reports/encode_neon_shift_imm_shift_i64_trunc.md

```property
function: encode_neon_shift_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift, k]
  domain: { t: neon_t, shift: 1..esize(t), k: nonzero }
  relation:
    op: throws
    expr: encode_neon_shift_imm([Vd.T, Vn.T, Imm(shift + k * 2^32)], true)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
  k: { gen: int, min: -4, max: 4, type: i64 }
evidence: ARM ARM shift in [1, esize]; llvm-mc range check on the textual immediate; README.md:14 gas-compatible
```

## encode_neon_shift_imm_neg_reg_source
- Tier: 4e
- Rationale: Coverage sweep. llvm-mc rejects a bare GPR/FP/V source (`ushr v0.8b, x0, #1`). USHR source must be Vn.T. get_neon_reg accepts Operand::Reg and discards the empty arrangement.
- Seed: (none)
- Formal: ∀ rd, n ∈ {0..31}, prefix ∈ {x,w,d,s,q,h,b,v}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_shift_imm([Vd.T, Reg(prefix n), Imm(shift)], true) = Err
- Test file: src/backend/arm/assembler/encoder/neon.rs
- Status: failing
- Counterexample: rd=0, prefix="x", n=0, t="8b", shift=1 (`ushr v0.8b, x0, #1`)
- Bug report: pbt-out/bug_reports/encode_neon_shift_imm_reg_source.md

```property
function: encode_neon_shift_imm
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, prefix, n, t, shift]
  domain: { prefix: xwdsqhbv, n: 0..31, t: neon_t, shift: 1..esize(t) }
  relation:
    op: throws
    expr: encode_neon_shift_imm([Vd.T, Reg(prefix n), Imm(shift)], true)
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  n: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: oneof, items: ["8b", "16b", "4h", "8h", "2s", "4s", "2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: llvm-mc invalid operand on bare source; ARM ARM USHR Vn.T; README.md:14 gas-compatible
```
