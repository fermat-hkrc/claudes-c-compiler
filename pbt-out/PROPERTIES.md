# Properties: encode_neon_shl

## encode_neon_shl_diff_llvm_mc
- Tier: 2
- Rationale: Strongest evidenced oracle is differential vs llvm-mc AArch64 assembler. State machine rejected (pure function, no lifecycle). Algebraic round-trip rejected (no in-tree SHL decoder). Sibling encode_neon_sli rejected (same-job gate: SLI / U=1). encode_neon_shift_left_imm rejected (independence gate: generic left-shift helper). encode_neon_sshr / encode_neon_ushr rejected (right-shift encodings). SUT-boundary: internal-helper of GNU-style assembler; encode_instruction dispatches `"shl"` through to encode_neon_shl with operands unchanged (encoder/mod.rs:702).
- Doc contract: neon.rs:1230 "Encode NEON SHL (shift left immediate)" — asserted fingerprint e66f0655
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_diff_llvm_mc (same encoding class, SHL shift domain [0, esize-1])
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1]. encode_neon_shl([Vd.T, Vn.T, #shift]) = llvm-mc("shl Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 0..esize(t)-1 }
  relation:
    op: eq
    lhs: encode_neon_shl([RegArrangement(v{rd},t), RegArrangement(v{rn},t), Imm(shift)])
    rhs: llvm_mc("shl v{rd}.{t}, v{rn}.{t}, #{shift}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 0, max: 63, type: i64 }] }
evidence: encoder/mod.rs:702 "shl" => encode_neon_shl(operands); README.md:12 gas-compatible; README.md:228 shl in NEON shifts; ARM SHL T in {8B,16B,4H,8H,2S,4S,2D} shift in [0, esize-1]
```

## encode_neon_shl_metamorphic_rd_rn_q
- Tier: 4
- Rationale: Weaker than differential. Changing only Rd / only Rn / only Q (8b vs 16b, same esize and shift) must affect only bits[4:0] / bits[9:5] / bit 30. Evidence: neon.rs:1242 layout "0 Q 0 0 11110 immh:immb 010101 Rn Rd".
- Doc contract: neon.rs:1242 "0 Q 0 0 11110 immh:immb 010101 Rn Rd" — asserted fingerprint 8fa3e60d
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_metamorphic_rd_rn_q
- Formal: ∀ rd1,rd2,rn1,rn2 ∈ {0..31}. let w11=SHL(rd1,rn1,8b,#0), w21=SHL(rd2,rn1,8b,#0), w12=SHL(rd1,rn2,8b,#0), w16=SHL(rd1,rn1,16b,#0). (w11⊕w21)∧¬0x1F = 0 ∧ (w11⊕w12)∧¬(0x1F≪5) = 0 ∧ (w11⊕w16)∧¬(1≪30) = 0 ∧ Q(w11)=0 ∧ Q(w16)=1
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shl
oracle: algebraic.metamorphic
predicate:
  quantifier: forall
  vars: [rd1, rd2, rn1, rn2]
  domain: { rd1: v0..v31, rd2: v0..v31, rn1: v0..v31, rn2: v0..v31 }
  body: ((w11 xor w21) and not 0x1F) == 0 and ((w11 xor w12) and not (0x1F << 5)) == 0 and ((w11 xor w16) and not (1 << 30)) == 0
generators:
  rd1: { gen: int, min: 0, max: 31, type: u32 }
  rd2: { gen: int, min: 0, max: 31, type: u32 }
  rn1: { gen: int, min: 0, max: 31, type: u32 }
  rn2: { gen: int, min: 0, max: 31, type: u32 }
evidence: neon.rs:1242 0 Q 0 0 11110 immh:immb 010101 Rn Rd
```

## encode_neon_shl_invariant_arm_fields
- Tier: 4
- Rationale: ARM Advanced SIMD shift-left-immediate field layout. Weaker than differential. U=0, opcode 010101, immh:immb = esize + shift, Q from T. Evidence: neon.rs:1242-1243 and ARM SHL encoding.
- Doc contract: neon.rs:1243 "immh:immb = element_size + shift" — asserted fingerprint 23939bc7
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_invariant_arm_fields
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1]. let w = encode_neon_shl(...). w[31]=0 ∧ w[30]=Q(T) ∧ w[29]=0 ∧ w[28:23]=011110 ∧ w[22:16]=esize(T)+shift ∧ w[15:10]=010101 ∧ w[9:5]=rn ∧ w[4:0]=rd
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shl
oracle: algebraic.invariant
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: {8b,16b,4h,8h,2s,4s,2d}, shift: 0..esize(t)-1 }
  body: fields(w) match ARM SHL encoding with U=0 opcode=010101 immh:immb=esize+shift
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 0, max: 63, type: i64 }] }
evidence: neon.rs:1242-1243 ARM SHL 0 Q 0 0 11110 immh:immb 010101 Rn Rd; immh:immb = element_size + shift
```

## encode_neon_shl_neg_arity
- Tier: 4
- Rationale: Negative/error contract. GNU/llvm-mc reject SHL with fewer than 3 operands. README.md:12 gas-compatible. Expected: Err.
- Doc contract: neon.rs:1241 "SHL Vd.T, Vn.T, #shift" — asserted fingerprint e46ca3fd
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_neg_arity
- Formal: ∀ n ∈ {0,1,2}, rd,rn ∈ {0..31}. encode_neon_shl(first n of [Vd.8b, Vn.8b, #0]) = Err ∧ llvm-mc rejects the matching arity-n asm
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [n, rd, rn]
  domain: { n: 0..2, rd: v0..v31, rn: v0..v31 }
  relation:
    op: throws
    lhs: encode_neon_shl(ops.take(n))
    rhs: String
generators:
  n: { gen: int, min: 0, max: 2, type: usize }
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
expected_error: String
evidence: neon.rs:1241 SHL Vd.T, Vn.T, #shift; README.md:12 gas-compatible; llvm-mc rejects arity < 3
```

## encode_neon_shl_neg_extra_operand
- Tier: 4
- Rationale: Negative/error contract. gas/llvm-mc reject a fourth operand. README.md:12. The SUT uses `len() < 3` (extra ignored) — that is the candidate defect, not a domain restriction.
- Doc contract: neon.rs:1241 "SHL Vd.T, Vn.T, #shift" — asserted fingerprint e46ca3fd
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_neg_extra_operand
- Formal: ∀ rd,rn,extra ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1]. encode_neon_shl([Vd.T, Vn.T, #shift, Vextra.T]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: failing
- Counterexample: encode_neon_shl([v0.8b, v0.8b, #0, v0.8b]) → Ok(Word) (rd=0, rn=0, extra=0, t=8b, shift=0)
- Bug report: bug_reports/encode_neon_shl_extra_operand.md

```property
function: encoder.neon.encode_neon_shl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, extra, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, extra: v0..v31, t: valid SHL T, shift: 0..esize(t)-1 }
  relation:
    op: throws
    lhs: encode_neon_shl([Vd.T, Vn.T, Imm(shift), Vextra.T])
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  extra: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 0, max: 63, type: i64 }] }
expected_error: String
evidence: neon.rs:1241 SHL Vd.T, Vn.T, #shift (exactly three operands); README.md:12; llvm-mc rejects fourth operand
```

## encode_neon_shl_neg_invalid_t
- Tier: 4
- Rationale: Negative/error contract. ARM SHL T is {8B,16B,4H,8H,2S,4S,2D} with matching arrangements; 1D reserved; mismatched T illegal. llvm-mc rejects. Source arrangement discarded in SUT is a candidate defect, not a restriction.
- Doc contract: neon.rs:1241 "SHL Vd.T, Vn.T, #shift" — asserted fingerprint e46ca3fd
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_neg_invalid_t
- Formal: ∀ rd,rn ∈ {0..31}, Td,Tn ∈ {8b,16b,4h,8h,2s,4s,1d,2d,1q}, shift ∈ [0,64]. (Td ≠ Tn ∨ Td ∉ valid_SHL_T) ⇒ encode_neon_shl([Vd.Td, Vn.Tn, #shift]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: failing
- Counterexample: encode_neon_shl([v0.2s, v0.4s, #0]) → Ok(Word) (rd=0, rn=0, td=2s, tn=4s, shift=0)
- Bug report: bug_reports/encode_neon_shl_mismatched_t.md

```property
function: encoder.neon.encode_neon_shl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, td, tn, shift]
  domain: { rd: v0..v31, rn: v0..v31, td: arrangements, tn: arrangements, shift: 0..64 }
  relation:
    op: throws
    lhs: encode_neon_shl([Vd.td, Vn.tn, Imm(shift)])
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  td: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  tn: { gen: oneof, values: ["8b","16b","4h","8h","2s","4s","1d","2d","1q"] }
  shift: { gen: int, min: 0, max: 64, type: i64 }
expected_error: String
evidence: neon.rs:1241 SHL Vd.T, Vn.T, #shift (matching T); ARM SHL T in {8B,16B,4H,8H,2S,4S,2D}; llvm-mc rejects mismatch/1d/1q
```

## encode_neon_shl_neg_shift_oob
- Tier: 4
- Rationale: Negative/error contract. ARM SHL shift is [0, esize-1]; llvm-mc rejects 0-1 below and esize / negatives. SUT masks OOB shifts — candidate defect, not a domain restriction. Documented bound sampled at  -1, esize, esize+1.
- Doc contract: neon.rs:1241 "SHL Vd.T, Vn.T, #shift" — asserted fingerprint e46ca3fd
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_neg_shift_oob
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∉ [0, esize(T)-1]. encode_neon_shl([Vd.T, Vn.T, #shift]) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: failing
- Counterexample: encode_neon_shl([v0.8b, v0.8b, #-1]) panics "attempt to add with overflow" (rd=0, rn=0, t=8b, shift=-1); also encode_neon_shl([v0.8b, v0.8b, #8]) → Ok(Word)
- Bug report: bug_reports/encode_neon_shl_shift_oob.md

```property
function: encoder.neon.encode_neon_shl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: valid SHL T, shift: not in [0, esize(t)-1] }
  relation:
    op: throws
    lhs: encode_neon_shl([Vd.T, Vn.T, Imm(shift)])
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: oneof, values: [-1, esize, esize+1] }] }
expected_error: String
evidence: ARM SHL shift in [0, esize-1]; README.md:12; llvm-mc rejects #esize / negatives
```

## encode_neon_shl_neg_gpr_or_bare
- Tier: 4
- Rationale: Negative/error contract. gas/llvm-mc require Vd.T / Vn.T NEON arrangements. GPR dest (xN.8b), bare Vn, bare Vd, FP scalar prefixes must Err. get_neon_reg accepts Operand::Reg and x/w prefixes — candidate defect.
- Doc contract: neon.rs:1241 "SHL Vd.T, Vn.T, #shift" — asserted fingerprint e46ca3fd
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_neg_gpr_or_bare
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {GPR dest, bare Vn, bare Vd, xN.8b dest, GPR src}. encode_neon_shl(kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: failing
- Counterexample: encode_neon_shl([v0.8b, Operand::Reg("v0"), #0]) → Ok(Word) (rd=0, rn=0, kind=1); also encode_neon_shl([x0.8b, v0.8b, #0]) → Ok(Word)
- Bug report: bug_reports/encode_neon_shl_bare_src.md

```property
function: encoder.neon.encode_neon_shl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind]
  domain: { rd: v0..v31, rn: v0..v31, kind: {gpr_dest, bare_vn, bare_vd, x_arr_dest, gpr_src} }
  relation:
    op: throws
    lhs: encode_neon_shl(kind_ops)
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 4, type: u8 }
expected_error: String
evidence: neon.rs:1241 SHL Vd.T, Vn.T, #shift; README.md:12; llvm-mc requires NEON arrangements
```

## encode_neon_shl_diff_alt_spellings
- Tier: 2
- Rationale: Sweep — uppercase mnemonic/register/arrangement spelling still maps to the same llvm-mc encoding. parse_reg_num lowercases. Documented by README.md:12 gas-compatible textual assembly.
- Doc contract: neon.rs:1230 "Encode NEON SHL (shift left immediate)" — asserted fingerprint e66f0655
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_diff_alt_spellings
- Formal: ∀ rd,rn ∈ {0..31}, T ∈ {8b,16b,4h,8h,2s,4s,2d}, shift ∈ [0, esize(T)-1]. encode_neon_shl([V{rd}.T, V{rn}.T, #shift]) = llvm-mc("SHL Vd.T, Vn.T, #shift")
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shl
oracle: differential
predicate:
  quantifier: forall
  vars: [rd, rn, t, shift]
  domain: { rd: v0..v31, rn: v0..v31, t: valid SHL T, shift: 0..esize(t)-1 }
  relation:
    op: eq
    lhs: encode_neon_shl([RegArrangement(V{rd}, t), RegArrangement(V{rn}, t), Imm(shift)])
    rhs: llvm_mc("SHL V{rd}.{T}, V{rn}.{T}, #{shift}")
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  t_shift: { gen: tuple, items: [{ gen: oneof, values: ["8b","16b","4h","8h","2s","4s","2d"] }, { gen: int, min: 0, max: 63, type: i64 }] }
evidence: README.md:12 gas-compatible; parse_reg_num lowercases; llvm-mc accepts uppercase SHL
```

## encode_neon_shl_neg_nonreg
- Tier: 4
- Rationale: Sweep — Imm/Mem/Label in dest or src slots must Err. get_neon_reg only accepts RegArrangement and Reg.
- Doc contract: neon.rs:1241 "SHL Vd.T, Vn.T, #shift" — asserted fingerprint e46ca3fd
- Seed: encode_neon_sshr_pbt.rs encode_neon_sshr_neg_nonreg
- Formal: ∀ rd,rn ∈ {0..31}, kind ∈ {Imm, Mem, Label}, slot ∈ {0,1}. encode_neon_shl(ops with slot replaced by kind) = Err
- Test file: src/backend/arm/assembler/encoder/encode_neon_shl_pbt.rs
- Status: passing
- Counterexample: (none)
- Bug report: (none)

```property
function: encoder.neon.encode_neon_shl
oracle: negative_error
predicate:
  quantifier: forall
  vars: [rd, rn, kind, slot]
  domain: { rd: v0..v31, rn: v0..v31, kind: {Imm, Mem, Label}, slot: {0,1} }
  relation:
    op: throws
    lhs: encode_neon_shl(ops_with_nonreg)
    rhs: String
generators:
  rd: { gen: int, min: 0, max: 31, type: u32 }
  rn: { gen: int, min: 0, max: 31, type: u32 }
  kind: { gen: int, min: 0, max: 2, type: u8 }
  slot: { gen: int, min: 0, max: 1, type: usize }
expected_error: String
evidence: neon.rs:1241 SHL Vd.T, Vn.T, #shift; get_neon_reg rejects Imm/Mem/Label
```
