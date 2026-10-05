# Properties: encode_neon_ushr

## encode_neon_ushr_diff_llvm_mc
- Tier: 2
- Rationale: Strongest applicable oracle is Differential vs llvm-mc (independent GNU-style assembler of the same textual assembly the SUT claims to accept). State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree USHR decoder). Sibling encode_neon_shift_imm rejected (independence gate: near-copy of this body). encode_neon_sshr rejected (same-job gate: SSHR / U=0). encode_neon_shift_right rejected (generic shift-right opcode table).
- Doc contract: neon.rs:1179 "Encode NEON USHR (unsigned shift right immediate)" — asserted fingerprint 42c4ff29
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_diff_llvm_mc
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_ushr([Vd.T, Vn.T, #shift]) = llvm-mc("ushr Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ushr
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: u32_0_31, rn: u32_0_31, t: neon_ushr_t, shift: 1_esize_t }
  relation:
    op: eq
    lhs: encode_neon_ushr([Vd.T, Vn.T, Imm(shift)])
    rhs: llvm_mc(ushr Vd.T, Vn.T, #shift)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint 10d52c4c
```

## encode_neon_ushr_metamorphic_rd_rn_q
- Tier: 3
- Rationale: ARM shift-by-immediate layout isolates Rd[4:0], Rn[9:5], Q[30]. Stronger differential is p1; this metamorphic check does not need llvm-mc and catches field-packing bugs. 8b vs 16b at the same shift must differ only in Q.
- Doc contract: neon.rs:1190 "0 Q 1 0 11110 immh:immb 000001 Rn Rd" — asserted fingerprint 726e0020
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_metamorphic_rd_rn_rm_u
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}. changing only Rd (resp. Rn) of encode_neon_ushr on T=8b shift=1 differs only in bits[4:0] (resp. [9:5]); 8b vs 16b at the same rd,rn,shift=1 differs only in bit 30
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ushr
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2]
  domain: { rd1: u32_0_31, rd2: u32_0_31, rn1: u32_0_31, rn2: u32_0_31 }
  body: changing only Rd differs only in bits[4:0]; only Rn in bits[9:5]; 8b vs 16b differs only in Q bit 30
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1190 "0 Q 1 0 11110 immh:immb 000001 Rn Rd"
```

## encode_neon_ushr_invariant_arm_fields
- Tier: 3
- Rationale: ARM Advanced SIMD shift-by-immediate USHR encoding is an exact structural predicate on the success-path word. Weaker than differential; pins bit layout even if llvm-mc is unavailable.
- Doc contract: neon.rs:1190 "0 Q 1 0 11110 immh:immb 000001 Rn Rd" — asserted fingerprint 726e0020
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_invariant_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. let w = encode_neon_ushr([Vd.T,Vn.T,#shift]). bit31(w)=0 ∧ Q(w)=Q(T) ∧ U(w)=1 ∧ bits[28:23](w)=011110 ∧ immh:immb(w)=2*esize(T)-shift ∧ bits[15:10](w)=000001 ∧ Rn(w)=rn ∧ Rd(w)=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ushr
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: u32_0_31, rn: u32_0_31, t: neon_ushr_t, shift: 1_esize_t }
  body: ARM USHR fields of encode_neon_ushr word match Q(T), U=1, bits[28:23]=011110, immh:immb=2*esize-shift, opcode=000001, Rn, Rd
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: neon.rs:1190 "0 Q 1 0 11110 immh:immb 000001 Rn Rd"
```

## encode_neon_ushr_neg_arity
- Tier: 4
- Rationale: USHR is a three-operand instruction. llvm-mc / gas reject arity 0–2. The SUT documents "ushr requires 3 operands" at neon.rs:1181.
- Doc contract: neon.rs:1181 "ushr requires 3 operands" — asserted fingerprint 33a152b7
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn ∈ {0..31}. encode_neon_ushr(ops[:n]) = Err ∧ llvm-mc rejects the corresponding arity-n ushr
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ushr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn]
  domain: { n: 0_2, rd: u32_0_31, rn: u32_0_31 }
  relation:
    op: holds
    lhs: encode_neon_ushr(ops.take(n)).is_err()
expected_error: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1181 "ushr requires 3 operands"
```

## encode_neon_ushr_neg_extra_operand
- Tier: 4
- Rationale: gas/llvm-mc reject a fourth operand on USHR. README claims the assembler accepts the same textual assembly gas would consume, so extra operands must Err. The SUT only checks len < 3.
- Doc contract: neon.rs:1189 "USHR Vd.T, Vn.T, #shift" — asserted fingerprint b69265b4
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. llvm-mc rejects "ushr Vd.T, Vn.T, #shift, v{extra}.T" ⇒ encode_neon_ushr([Vd.T,Vn.T,#shift,Vextra.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: failing
- Counterexample: encode_neon_ushr([v0.8b, v0.8b, #1, v0.8b])
- Bug report: pbt-out/bug_reports/encode_neon_ushr_extra_operand.md

```property
function: encoder.encode_neon_ushr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, shift]
  domain: { rd: u32_0_31, rn: u32_0_31, extra: u32_0_31, t: neon_ushr_t, shift: 1_esize_t }
  relation:
    op: holds
    lhs: encode_neon_ushr([Vd.T, Vn.T, Imm(shift), Vextra.T]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint 10d52c4c
```

## encode_neon_ushr_neg_invalid_t
- Tier: 4
- Rationale: ARM USHR vector form requires matching T in {8B,16B,4H,8H,2S,4S,2D}; 1D is reserved (immh encoding would collide with smaller esize); mismatched dest/src arrangements are invalid. llvm-mc rejects these. Source arrangement is discarded by the SUT.
- Doc contract: neon.rs:1189 "USHR Vd.T, Vn.T, #shift" — asserted fingerprint b69265b4
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, Td,Tn ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}, shift ∈ [1, esize(Td) if valid else 1..64]. ¬(valid_ushr_t(Td) ∧ Td=Tn) ⇒ encode_neon_ushr([Vd.Td,Vn.Tn,#shift]) = Err ∧ llvm-mc rejects
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: failing
- Counterexample: encode_neon_ushr([v0.8b, v0.16b, #1])
- Bug report: pbt-out/bug_reports/encode_neon_ushr_mismatched_t.md

```property
function: encoder.encode_neon_ushr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, shift]
  domain: { rd: u32_0_31, rn: u32_0_31, td: any_neon_t, tn: any_neon_t, shift: 1_64 }
  relation:
    op: holds
    lhs: encode_neon_ushr([Vd.Td, Vn.Tn, Imm(shift)]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: element, of: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tn: { gen: element, of: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: neon.rs:1189 "USHR Vd.T, Vn.T, #shift"
```

## encode_neon_ushr_neg_shift_oob
- Tier: 4
- Rationale: ARM/llvm-mc require shift in [1, esize(T)]. Bounds must be sampled at 0, 1, esize, esize+1. The SUT masks (2*esize - shift) which wraps OOB shifts into a valid-looking immh:immb. i64 values that truncate to a valid u32 shift are also out of the documented domain.
- Doc contract: neon.rs:1189 "USHR Vd.T, Vn.T, #shift" — asserted fingerprint b69265b4
- Seed: neon.rs encode_neon_shift_imm_neg_shift_oob
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ ℤ \ [1, esize(T)]. llvm-mc rejects "ushr Vd.T, Vn.T, #shift" ⇒ encode_neon_ushr([Vd.T,Vn.T,#shift]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: failing
- Counterexample: encode_neon_ushr([v0.8b, v0.8b, #0])
- Bug report: pbt-out/bug_reports/encode_neon_ushr_shift_oob.md

```property
function: encoder.encode_neon_ushr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: u32_0_31, rn: u32_0_31, t: neon_ushr_t, shift: not_in_1_esize }
  relation:
    op: holds
    lhs: encode_neon_ushr([Vd.T, Vn.T, Imm(shift)]).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  shift: { gen: int, min: -8, max: 128, type: i64 }
evidence: llvm-mc "immediate must be an integer in range [1, esize]"
```

## encode_neon_ushr_neg_gpr_or_bare
- Tier: 4
- Rationale: gas/llvm-mc require Vd.T / Vn.T. GPR dest, bare V without arrangement, and xN.T are invalid. get_neon_reg accepts Operand::Reg and parse_reg_num accepts x/w prefixes, so these inputs are caller-reachable.
- Doc contract: neon.rs:1189 "USHR Vd.T, Vn.T, #shift" — asserted fingerprint b69265b4
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_gpr_or_bare
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {gpr_dest, bare_src, x_arr_dest, bare_dest, fp_dest}. llvm-mc rejects the corresponding ushr ⇒ encode_neon_ushr(ops(kind)) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: failing
- Counterexample: encode_neon_ushr([x0.8b, v0.8b, #1])
- Bug report: pbt-out/bug_reports/encode_neon_ushr_gpr_dest.md

```property
function: encoder.encode_neon_ushr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind]
  domain: { rd: u32_0_31, rn: u32_0_31, kind: gpr_bare_kind }
  relation:
    op: holds
    lhs: encode_neon_ushr(ops(kind)).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
evidence: neon.rs:1189 "USHR Vd.T, Vn.T, #shift"
```

## encode_neon_ushr_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — README gas-compatibility plus parse_reg_num lowercasing V prefixes. Differential vs llvm-mc on uppercase mnemonic/V with lowercase T.
- Doc contract: neon.rs:1179 "Encode NEON USHR (unsigned shift right immediate)" — asserted fingerprint 42c4ff29
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [1, esize(T)]. encode_neon_ushr([Vd.T,Vn.T,#shift] with V prefix) = llvm-mc("USHR Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ushr
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: u32_0_31, rn: u32_0_31, t: neon_ushr_t, shift: 1_esize_t }
  relation:
    op: eq
    lhs: encode_neon_ushr([V{rd}.T, V{rn}.T, Imm(shift)])
    rhs: llvm_mc(USHR Vd.T, Vn.T, #shift)
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t: { gen: element, of: ["8b","16b","4h","8h","2s","4s","2d"] }
  shift: { gen: int, min: 1, max: 64, type: i64 }
evidence: src/backend/arm/assembler/README.md:12 "It accepts the same textual assembly that GCC's gas would consume" fingerprint 10d52c4c
```

## encode_neon_ushr_neg_nonreg
- Tier: 4
- Rationale: Sweep — Imm/Mem/Label at a register slot must Err (get_neon_reg error contract). llvm-mc rejects non-register dest.
- Doc contract: neon.rs:1189 "USHR Vd.T, Vn.T, #shift" — asserted fingerprint b69265b4
- Seed: encode_neon_add_sub_pbt.rs encode_neon_add_sub_neg_nonreg
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {Imm, Mem, Label}, slot ∈ {0,1}. encode_neon_ushr(ops with slot replaced by kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_ushr_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.encode_neon_ushr
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd: u32_0_31, rn: u32_0_31, kind: 0_2, slot: 0_1 }
  relation:
    op: holds
    lhs: encode_neon_ushr(ops_with_nonreg).is_err()
expected_error: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
evidence: neon.rs:1189 "USHR Vd.T, Vn.T, #shift"
```
