# Properties: encode_neon_sshr

## encode_neon_sshr_diff_llvm_mc
- Tier: 5
- Rationale: Strongest evidenced oracle is differential vs llvm-mc. README.md:12 claims gas-compatible textual assembly; README.md:228 lists sshr; encoder/mod.rs:699 dispatches sshr to this function with operands passed through. ARM Advanced SIMD SSHR encoding is independently realized by llvm-mc. State machine rejected (pure function). Algebraic round-trip rejected (no in-tree SSHR decoder). Differential vs encode_neon_ushr rejected (same-job gate: USHR U=1). Differential vs encode_neon_shift_imm rejected (independence gate: near-copy USHR body).
- Doc contract: neon.rs:1204 "Encode NEON SSHR (signed shift right immediate)" — asserted fingerprint 4256ae10
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:247 encode_neon_ushr_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_sshr([Vd.T, Vn.T, #shift]) = Word(w) ∧ w = llvm-mc("sshr Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_sshr
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 1..=esize(t) }
  relation:
    op: eq
    lhs: encode_neon_sshr([RegArrangement(v{rd}, t), RegArrangement(v{rn}, t), Imm(shift)])
    rhs: llvm_mc("sshr v{rd}.{t}, v{rn}.{t}, #{shift}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, elems: [{ gen: string }, { gen: int, min: 1, max: 64, type: i64 }] }
evidence: src/backend/arm/assembler/README.md:12; README.md:228; encoder/mod.rs:699; neon.rs:1204
```

## encode_neon_sshr_metamorphic_rd_rn_q
- Tier: 4
- Rationale: ARM SSHR layout isolates Rd at [4:0], Rn at [9:5], Q at bit 30. Changing only Rd/Rn/Q must differ only in that field. Stronger differential covers the full word vs llvm-mc; this metamorphic check isolates field packing independently of the assembler. State machine / round-trip rejected as above.
- Doc contract: neon.rs:1216 "0 Q 0 0 11110 immh:immb 000001 Rn Rd  (U=0)" — asserted fingerprint 70f4105c
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:264 encode_neon_ushr_metamorphic_rd_rn_q
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}. let w11=sshr(rd1,rn1,8b,#1), w21=sshr(rd2,rn1,8b,#1), w12=sshr(rd1,rn2,8b,#1), w16=sshr(rd1,rn1,16b,#1). (w11 ⊕ w21) ∧ ¬0x1F = 0 ∧ (w21 ∧ 0x1F) = rd2 ∧ (w11 ⊕ w12) ∧ ¬(0x1F≪5) = 0 ∧ ((w12≫5) ∧ 0x1F) = rn2 ∧ (w11 ⊕ w16) ∧ ¬(1≪30) = 0 ∧ Q(w11)=0 ∧ Q(w16)=1
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_sshr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2]
  domain: { rd1: 0..31, rd2: 0..31, rn1: 0..31, rn2: 0..31 }
  body: changing only Rd/Rn/Q differs only in bits[4:0]/[9:5]/bit30
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1216
```

## encode_neon_sshr_invariant_arm_fields
- Tier: 4
- Rationale: ARM Advanced SIMD SSHR word: bit31=0, Q from T, U=0, bits[28:23]=011110, immh:immb=2*esize-shift, opcode=000001, Rn, Rd. Independent of llvm-mc. Documented by neon.rs:1216.
- Doc contract: neon.rs:1216 "0 Q 0 0 11110 immh:immb 000001 Rn Rd  (U=0)" — asserted fingerprint 70f4105c
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:305 encode_neon_ushr_invariant_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. let w = encode_neon_sshr(...). bit31(w)=0 ∧ Q(w)=Q(T) ∧ U(w)=0 ∧ bits[28:23]=0b011110 ∧ bits[22:16]=(2*esize(T)-shift) ∧ bits[15:10]=0b000001 ∧ Rn=rn ∧ Rd=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_sshr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: 0..31, rn: 0..31, t: valid SSHR T, shift: 1..=esize(t) }
  body: ARM SSHR fields of encode_neon_sshr word match bit31=0 Q U=0 011110 immh:immb opcode=000001 Rn Rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, elems: [{ gen: string }, { gen: int, min: 1, max: 64, type: i64 }] }
evidence: neon.rs:1216
```

## encode_neon_sshr_neg_arity
- Tier: 3
- Rationale: neon.rs:1206-1208 documents "sshr requires 3 operands". llvm-mc rejects arity 0/1/2. Negative/error contract: fewer than 3 operands must Err.
- Doc contract: neon.rs:1207 "sshr requires 3 operands" — asserted fingerprint 51823b02
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:331 encode_neon_ushr_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn ∈ {0..31}. encode_neon_sshr(ops[0..n]) = Err ∧ llvm-mc(arity-n asm) rejects
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_sshr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn]
  domain: { n: 0..=2, rd: 0..31, rn: 0..31 }
  relation:
    op: throws
    expr: encode_neon_sshr(take(ops_t(rd,rn,8b,1), n))
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1206-1208; llvm-mc rejects arity 0/1/2
```

## encode_neon_sshr_neg_extra_operand
- Tier: 3
- Rationale: GNU/llvm-mc reject a fourth operand (`sshr v0.8b, v1.8b, #1, v2.8b`). README.md:12 gas-compatible contract. The SUT only checks `operands.len() < 3`, so extra operands are a documented-valid rejection the code may miss.
- Doc contract: neon.rs:1215 "SSHR Vd.T, Vn.T, #shift" — asserted fingerprint 0167e5c9
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:357 encode_neon_ushr_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ valid, shift ∈ [1, esize(T)]. llvm-mc rejects 4-operand sshr ∧ encode_neon_sshr([Vd.T, Vn.T, #shift, Vextra.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: failing
- Counterexample: encode_neon_sshr([v0.8b, v0.8b, #1, v0.8b])
- Bug report: pbt-out/bug_reports/encode_neon_sshr_extra_operand.md

```property
function: encoder.encode_neon_sshr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, shift]
  domain: { rd: 0..31, rn: 0..31, extra: 0..31, t: valid SSHR T, shift: 1..=esize(t) }
  relation:
    op: throws
    expr: encode_neon_sshr([Vd.T, Vn.T, Imm(shift), Vextra.T])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, elems: [{ gen: string }, { gen: int, min: 1, max: 64, type: i64 }] }
expected_error: String
evidence: README.md:12; llvm-mc rejects a fourth operand; neon.rs:1215 three-operand form
```

## encode_neon_sshr_neg_invalid_t
- Tier: 3
- Rationale: ARM SSHR T is {8B,16B,4H,8H,2S,4S,2D} with matching Vd/Vn arrangements; 1D is reserved. llvm-mc rejects mismatched/reserved T. Source arrangement discarded in SUT (`let (rn, _)`) so mismatch may encode.
- Doc contract: neon.rs:1215 "SSHR Vd.T, Vn.T, #shift" — asserted fingerprint 0167e5c9
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:382 encode_neon_ushr_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, Td,Tn arrangements, shift. (Td ≠ Tn ∨ Td ∉ valid SSHR T) ⇒ encode_neon_sshr = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: failing
- Counterexample: encode_neon_sshr([v0.2s, v0.8b, #1])
- Bug report: pbt-out/bug_reports/encode_neon_sshr_mismatched_t.md

```property
function: encoder.encode_neon_sshr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, shift]
  domain: { rd: 0..31, rn: 0..31, td: arrangements, tn: arrangements, shift: 1..=64 }
  relation:
    op: throws
    expr: encode_neon_sshr([Vd.Td, Vn.Tn, Imm(shift)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: string }
  tn: { gen: string }
  shift: { gen: int, min: 1, max: 64, type: i64 }
expected_error: String
evidence: neon.rs:1215 matching T; ARM reserved 1D; llvm-mc rejects
```

## encode_neon_sshr_neg_shift_oob
- Tier: 3
- Rationale: ARM/llvm-mc require shift in [1, esize(T)] (llvm-mc: "immediate must be an integer in range [1, 8]" for 8b). Documented bounds 1 and esize and bound±1 must be sampled. SUT masks `(2*esize - shift)` so OOB may wrap into a valid encoding.
- Doc contract: neon.rs:1215 "SSHR Vd.T, Vn.T, #shift" — asserted fingerprint 0167e5c9
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:413 encode_neon_ushr_neg_shift_oob
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ valid, shift ∉ [1, esize(T)]. encode_neon_sshr = Err ∧ (for |shift|<10000, llvm-mc rejects)
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: failing
- Counterexample: encode_neon_sshr([v0.8b, v0.8b, #0])
- Bug report: pbt-out/bug_reports/encode_neon_sshr_shift_oob.md

```property
function: encoder.encode_neon_sshr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: 0..31, rn: 0..31, t: valid SSHR T, shift: {0, esize+1, -1, 2*esize, 1<<32} }
  relation:
    op: throws
    expr: encode_neon_sshr([Vd.T, Vn.T, Imm(shift)])
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, elems: [{ gen: string }, { gen: int, type: i64 }] }
expected_error: String
evidence: ARM SSHR shift in [1, esize]; llvm-mc range error
```

## encode_neon_sshr_neg_gpr_or_bare
- Tier: 3
- Rationale: llvm-mc requires Vd.T / Vn.T. GPR names (x0.8b), bare v0, and Operand::Reg dest/src are invalid. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w/d/s/q/h/b prefixes, so non-V names may encode as Vd.
- Doc contract: neon.rs:1215 "SSHR Vd.T, Vn.T, #shift" — asserted fingerprint 0167e5c9
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs:440 encode_neon_ushr_neg_gpr_or_bare
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ GPR-dest | bare-Vn | bare-Vd | xN.8b dest | xN src. encode_neon_sshr = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: failing
- Counterexample: encode_neon_sshr([v0.8b, Operand::Reg("v0"), #1])
- Bug report: pbt-out/bug_reports/encode_neon_sshr_bare_src.md

```property
function: encoder.encode_neon_sshr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, fp_prefix]
  domain: { rd: 0..31, rn: 0..31, kind: 0..=4, fp_prefix: {x,w,d,s,q,h,b} }
  relation:
    op: throws
    expr: encode_neon_sshr(gpr_or_bare_ops(kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
  fp_prefix: { gen: string }
expected_error: String
evidence: neon.rs:1215 Vd.T Vn.T; llvm-mc rejects GPR/bare
```

## encode_neon_sshr_diff_alt_spellings
- Tier: 5
- Rationale: Sweep: parse_reg_num lowercases names, so V0 must encode as v0. llvm-mc accepts SSHR Vd.T uppercase. Differential vs llvm-mc on that spelling.
- Doc contract: neon.rs:1204 "Encode NEON SSHR (signed shift right immediate)" — asserted fingerprint 4256ae10
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs encode_neon_ushr_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ valid, shift ∈ [1, esize(T)]. encode_neon_sshr([V{rd}.T, V{rn}.T, #shift]) = llvm-mc("SSHR V{rd}.{T}, V{rn}.{T}, #{shift}")
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_sshr
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: 0..31, rn: 0..31, t: valid SSHR T, shift: 1..=esize(t) }
  relation:
    op: eq
    lhs: encode_neon_sshr([RegArrangement(V{rd}, t), RegArrangement(V{rn}, t), Imm(shift)])
    rhs: llvm_mc("SSHR V{rd}.{T}, V{rn}.{T}, #{shift}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: string }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: README.md:12; parse_reg_num lowercases
```

## encode_neon_sshr_neg_nonreg
- Tier: 3
- Rationale: Sweep: get_neon_reg other-arm returns Err for Imm/Mem/Label. llvm-mc rejects those as dest.
- Doc contract: neon.rs:1215 "SSHR Vd.T, Vn.T, #shift" — asserted fingerprint 0167e5c9
- Seed: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs encode_neon_ushr_neg_nonreg
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {Imm, Mem, Label}, slot ∈ {0,1}. encode_neon_sshr with that slot replaced = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_sshr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_sshr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd: 0..31, rn: 0..31, kind: 0..=2, slot: 0..=1 }
  relation:
    op: throws
    expr: encode_neon_sshr(ops_with_nonreg(slot, kind))
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: neon.rs:17 get_neon_reg other => Err
```
